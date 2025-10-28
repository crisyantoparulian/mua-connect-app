use aws_sdk_s3::{Client, primitives::ByteStream, config::Region};
use aws_credential_types::Credentials;
use bytes::Bytes;
use std::env;
use std::error::Error;
use sha2::Digest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();

    let s3_api_url = env::var("S3_API_URL").expect("S3_API_URL must be set");
    let s3_region = env::var("S3_REGION").expect("S3_REGION must be set");
    let s3_access_key = env::var("S3_ACCESS_KEY_ID").expect("S3_ACCESS_KEY_ID must be set");
    let s3_secret_key = env::var("S3_SECRET_ACCESS_KEY").expect("S3_SECRET_ACCESS_KEY must be set");
    let bucket_name = env::var("S3_BUCKET_NAME").unwrap_or_else(|_| "storage".to_string());

    println!("=== S3 Configuration Test ===");
    println!("API URL: {}", s3_api_url);
    println!("Region: {}", s3_region);
    println!("Bucket: {}", bucket_name);
    println!("Access Key: {}...", &s3_access_key[..20]);

    let credentials = Credentials::new(
        &s3_access_key,
        &s3_secret_key,
        None,
        None,
        "custom",
    );

    // Test 1: Basic AWS SDK configuration
    println!("\n=== Test 1: Basic AWS SDK Configuration ===");

    let config = aws_config::from_env()
        .region(Region::new(s3_region.clone()))
        .endpoint_url(&s3_api_url)
        .credentials_provider(credentials.clone())
        .load()
        .await;

    let client = Client::new(&config);

    // Test list buckets (should work with proper credentials)
    match client.list_buckets().send().await {
        Ok(result) => {
            println!("✅ List buckets successful!");
            if let Some(buckets) = result.buckets() {
                for bucket in buckets {
                    println!("  Bucket: {}", bucket.name().unwrap_or("unknown"));
                }
            }
        }
        Err(e) => {
            println!("❌ List buckets failed: {}", e);
            println!("  Error details: {:?}", e);
        }
    }

    // Test 2: Try a simple HEAD request to the bucket
    println!("\n=== Test 2: HEAD Bucket Test ===");
    match client.head_bucket().bucket(&bucket_name).send().await {
        Ok(_) => {
            println!("✅ HEAD bucket successful!");
        }
        Err(e) => {
            println!("❌ HEAD bucket failed: {}", e);
            println!("  Error details: {:?}", e);
        }
    }

    // Test 3: Try with different HTTP client configuration
    println!("\n=== Test 3: Alternative HTTP Client Test ===");

    // Try using a custom HTTP client builder
    let custom_config = aws_config::from_env()
        .region(Region::new(s3_region))
        .endpoint_url(&s3_api_url)
        .credentials_provider(credentials)
        .load()
        .await;

    let custom_client = Client::new(&custom_config);

    let test_data = Bytes::from("Hello, Supabase S3! Test from custom client.");
    let test_key = format!("test-upload-{}.txt", chrono::Utc::now().timestamp());

    match custom_client
        .put_object()
        .bucket(&bucket_name)
        .key(&test_key)
        .body(ByteStream::from(test_data))
        .content_type("text/plain")
        .send()
        .await {
        Ok(result) => {
            println!("✅ Upload successful with custom client!");
            println!("  ETag: {:?}", result.e_tag());
            println!("  Version: {:?}", result.version_id());
            println!("  Key: {}", test_key);

            // Test if we can retrieve it
            println!("\n=== Test 4: Get Object Test ===");
            match custom_client.get_object()
                .bucket(&bucket_name)
                .key(&test_key)
                .send()
                .await {
                Ok(result) => {
                    println!("✅ Get object successful!");
                    let data = result.body.collect().await?.into_bytes();
                    println!("  Content: {}", String::from_utf8_lossy(&data));
                }
                Err(e) => {
                    println!("❌ Get object failed: {}", e);
                    println!("  Error details: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Upload failed with custom client: {}", e);
            println!("  Error details: {:?}", e);

            // Print more detailed error information
            if let Some(source) = e.source() {
                println!("  Source: {}", source);
            }
        }
    }

    // Test 5: Test manual signature generation (like we would do with custom HTTP client)
    println!("\n=== Test 5: Manual HTTP Request Test ===");
    test_manual_http_request(&s3_api_url, &bucket_name, &s3_access_key, &s3_secret_key).await;

    println!("\n=== Test Complete ===");
    Ok(())
}

async fn test_manual_http_request(
    s3_api_url: &str,
    bucket_name: &str,
    access_key: &str,
    secret_key: &str,
) {
    use chrono::Utc;
    use hex;
    use hmac::{Hmac, Mac};
    use sha2::{Sha256, Digest};

    type HmacSha256 = Hmac<Sha256>;

    println!("Testing manual S3 HTTP request with custom signing...");

    let method = "PUT";
    let test_key = format!("manual-test-{}.txt", chrono::Utc::now().timestamp());
    let content = "Test file with manual signing";
    let host = s3_api_url.replace("https://", "").replace("http://", "");
    let region = "ap-southeast-1";

    // Current timestamp
    let now = Utc::now();
    let amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();
    let date_stamp = now.format("%Y%m%d").to_string();

    // Create canonical request
    let canonical_uri = format!("/{}/{}", bucket_name, test_key);
    let canonical_querystring = "";
    let canonical_headers = format!(
        "host:{}\nx-amz-content-sha256:{}\nx-amz-date:{}\n",
        host,
        hex::encode(sha2::Sha256::digest(content.as_bytes())),
        amz_date
    );
    let signed_headers = "host;x-amz-content-sha256;x-amz-date";
    let payload_hash = hex::encode(sha2::Sha256::digest(content.as_bytes()));

    let canonical_request = format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        method, canonical_uri, canonical_querystring, canonical_headers, signed_headers, payload_hash
    );

    println!("Canonical request: {}", canonical_request);

    // Create string to sign
    let algorithm = "AWS4-HMAC-SHA256";
    let credential_scope = format!("{}/{}/s3/aws4_request", date_stamp, region);
    let string_to_sign = format!(
        "{}\n{}\n{}\n{}",
        algorithm,
        amz_date,
        credential_scope,
        hex::encode(sha2::Sha256::digest(canonical_request.as_bytes()))
    );

    println!("String to sign: {}", string_to_sign);

    // Calculate signature
    let signing_key = get_signature_key(secret_key, &date_stamp, region, "s3");
    let mut mac = HmacSha256::new_from_slice(&signing_key).unwrap();
    mac.update(string_to_sign.as_bytes());
    let signature = hex::encode(mac.finalize().into_bytes());

    println!("Signature: {}", signature);

    // Create authorization header
    let authorization_header = format!(
        "{} Credential={}/{}, SignedHeaders={}, Signature={}",
        algorithm, access_key, credential_scope, signed_headers, signature
    );

    println!("Authorization: {}", authorization_header);

    // Make the HTTP request
    let url = format!("{}/{}/{}", s3_api_url, bucket_name, test_key);

    match reqwest::Client::new()
        .put(&url)
        .header("Host", host)
        .header("x-amz-content-sha256", &payload_hash)
        .header("x-amz-date", &amz_date)
        .header("Authorization", &authorization_header)
        .header("Content-Type", "text/plain")
        .body(content)
        .send()
        .await {
        Ok(response) => {
            println!("✅ Manual request successful! Status: {}", response.status());
            if response.status().is_success() {
                println!("  File uploaded successfully to: {}", test_key);
            } else {
                println!("  Response: {:?}", response.text().await);
            }
        }
        Err(e) => {
            println!("❌ Manual request failed: {}", e);
        }
    }
}

fn get_signature_key(key: &str, date_stamp: &str, region_name: &str, service_name: &str) -> Vec<u8> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    let k_date = sign(format!("AWS4{}", key).as_bytes(), date_stamp.as_bytes());
    let k_region = sign(&k_date, region_name.as_bytes());
    let k_service = sign(&k_region, service_name.as_bytes());
    let k_signing = sign(&k_service, b"aws4_request");

    k_signing
}

fn sign(key: &[u8], msg: &[u8]) -> Vec<u8> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(key).unwrap();
    mac.update(msg);
    mac.finalize().into_bytes().to_vec()
}