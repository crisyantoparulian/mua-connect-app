use aws_config::SdkConfig;
use aws_sdk_s3::{Client, primitives::ByteStream, config::Region, types::ObjectCannedAcl, presigning::PresigningConfig};
use aws_credential_types::Credentials;
use bytes::Bytes;
use base64::{Engine as _, engine::general_purpose};
use anyhow::Result;
use std::env;
use std::time::Duration;
use uuid::Uuid;

pub struct S3Service {
    client: Client,
    bucket_name: String,
}

impl S3Service {
    pub async fn new() -> Result<Self> {
        let s3_api_url = env::var("S3_API_URL")
            .expect("S3_API_URL must be set");
        let s3_region = env::var("S3_REGION")
            .expect("S3_REGION must be set");
        let s3_access_key = env::var("S3_ACCESS_KEY_ID")
            .expect("S3_ACCESS_KEY_ID must be set");
        let s3_secret_key = env::var("S3_SECRET_ACCESS_KEY")
            .expect("S3_SECRET_ACCESS_KEY must be set");

        // Get bucket name from environment or use default for Supabase
        let bucket_name = env::var("S3_BUCKET_NAME")
            .unwrap_or_else(|_| "storage".to_string());

        let credentials = Credentials::new(
            s3_access_key,
            s3_secret_key,
            None,
            None,
            "custom",
        );

        // For Supabase, we need to use the full endpoint including /storage/v1/s3
        // and ensure it ends with a slash so bucket concatenation works correctly
        let base_endpoint = if s3_api_url.ends_with('/') {
            s3_api_url.clone()
        } else {
            format!("{}/", s3_api_url)
        };

        let config = aws_config::from_env()
            .region(Region::new(s3_region))
            .endpoint_url(&base_endpoint)
            .credentials_provider(credentials)
            .load()
            .await;

        // Configure for path-style addressing for Supabase compatibility
        let config_builder = aws_sdk_s3::config::Builder::from(&config)
            .force_path_style(true);

        let s3_config = config_builder.build();

        // Test connection by constructing a simple client
        println!("DEBUG: Testing connection to S3 base endpoint: {}", base_endpoint);

        let client = Client::from_conf(s3_config);

        Ok(Self {
            client,
            bucket_name,
        })
    }

    pub async fn upload_image(
        &self,
        image_data: Bytes,
        content_type: &str,
        folder: &str,
    ) -> Result<String> {
        let file_extension = self.get_file_extension(content_type)?;
        let file_name = format!("{}.{}", Uuid::new_v4(), file_extension);
        let key = format!("{}/{}", folder, file_name);

        println!("DEBUG: S3 upload - bucket: {}, key: {}, content_type: {}", self.bucket_name, key, content_type);

        let body = ByteStream::from(image_data);

        let result = self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(&key)
            .body(body)
            .content_type(content_type)
            .send()
            .await;

        match result {
            Ok(_) => {
                println!("DEBUG: S3 upload successful");
            }
            Err(e) => {
                println!("DEBUG: S3 upload failed with AWS error: {:?}", e);
                return Err(anyhow::anyhow!("S3 upload error: {}", e));
            }
        }

        // Construct the public URL
        let s3_api_url = env::var("S3_API_URL")?;
        let public_url = format!("{}/{}", s3_api_url, key);
        println!("DEBUG: Constructed public URL: {}", public_url);

        Ok(public_url)
    }

    pub async fn upload_base64_image(
        &self,
        base64_data: &str,
        folder: &str,
    ) -> Result<String> {
        // Remove data URL prefix if present
        let (mime_type, base64_content) = if base64_data.starts_with("data:") {
            let parts: Vec<&str> = base64_data.split(',').collect();
            if parts.len() != 2 {
                return Err(anyhow::anyhow!("Invalid base64 data URL format"));
            }

            let mime_part = parts[0];
            let mime_type = mime_part
                .split(':')
                .nth(1)
                .and_then(|s| s.split(';').next())
                .unwrap_or("image/jpeg");

            (mime_type.to_string(), parts[1])
        } else {
            ("image/jpeg".to_string(), base64_data)
        };

        let image_data = general_purpose::STANDARD
            .decode(base64_content)
            .map_err(|e| anyhow::anyhow!("Failed to decode base64: {}", e))?;

        self.upload_image(Bytes::from(image_data), &mime_type, folder).await
    }

    pub async fn delete_image(&self, image_url: &str) -> Result<()> {
        // Extract key from URL
        let key = image_url
            .split('/')
            .last()
            .ok_or_else(|| anyhow::anyhow!("Invalid image URL"))?;

        self.client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await?;

        Ok(())
    }

    fn get_file_extension(&self, content_type: &str) -> Result<&str> {
        match content_type {
            "image/jpeg" => Ok("jpg"),
            "image/jpg" => Ok("jpg"),
            "image/png" => Ok("png"),
            "image/gif" => Ok("gif"),
            "image/webp" => Ok("webp"),
            _ => Err(anyhow::anyhow!("Unsupported image format: {}", content_type)),
        }
    }

    pub fn validate_image_base64(&self, base64_data: &str) -> Result<(String, Vec<u8>)> {
        let (mime_type, base64_content) = if base64_data.starts_with("data:") {
            let parts: Vec<&str> = base64_data.split(',').collect();
            if parts.len() != 2 {
                return Err(anyhow::anyhow!("Invalid base64 data URL format"));
            }

            let mime_part = parts[0];
            let mime_type = mime_part
                .split(':')
                .nth(1)
                .and_then(|s| s.split(';').next())
                .unwrap_or("image/jpeg");

            (mime_type.to_string(), parts[1])
        } else {
            ("image/jpeg".to_string(), base64_data)
        };

        // Validate MIME type
        if !mime_type.starts_with("image/") {
            return Err(anyhow::anyhow!("Invalid file type: {}. Only images are allowed.", mime_type));
        }

        // Decode and validate file size (max 10MB)
        let image_data = general_purpose::STANDARD
            .decode(base64_content)
            .map_err(|e| anyhow::anyhow!("Failed to decode base64: {}", e))?;

        if image_data.len() > 10 * 1024 * 1024 {
            return Err(anyhow::anyhow!("File too large. Maximum size is 10MB."));
        }

        Ok((mime_type, image_data))
    }

    /// Generate a presigned URL for direct upload to S3
    /// Valid for 1 hour (3600 seconds)
    pub async fn get_presigned_upload_url(
        &self,
        file_name: &str,
        content_type: &str,
        folder: &str,
    ) -> Result<(String, String)> {
        let file_extension = self.get_file_extension(content_type)?;
        let unique_file_name = format!("{}.{}", Uuid::new_v4(), file_extension);
        let key = format!("{}/{}", folder, unique_file_name);

        println!("DEBUG: Generating presigned URL - bucket: {}, key: {}, content_type: {}",
                self.bucket_name, key, content_type);

        let presigned_config = PresigningConfig::expires_in(Duration::from_secs(3600)) // 1 hour
            .map_err(|e| anyhow::anyhow!("Failed to create presigning config: {}", e))?;

        let presigned_request = self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(&key)
            .content_type(content_type)
            .presigned(presigned_config)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to generate presigned URL: {}", e))?;

        let presigned_url = presigned_request.uri().to_string();

        // Construct the final public URL that will be accessible after upload
        let s3_api_url = env::var("S3_API_URL")?;
        let public_url = format!("{}/{}", s3_api_url, key);

        println!("DEBUG: Generated presigned URL and public URL");
        println!("DEBUG: Public URL will be: {}", public_url);

        Ok((presigned_url, public_url))
    }

    /// Generate a presigned URL for direct upload to S3 with custom filename
    /// Valid for 1 hour (3600 seconds)
    pub async fn get_presigned_upload_url_with_filename(
        &self,
        file_name: &str,
        content_type: &str,
        folder: &str,
    ) -> Result<(String, String)> {
        let file_extension = self.get_file_extension(content_type)?;
        let sanitized_filename = format!("{}.{}",
            file_name.trim_end_matches(&format!(".{}", file_extension)),
            file_extension
        );
        let key = format!("{}/{}", folder, sanitized_filename);

        println!("DEBUG: Generating presigned URL with custom filename - bucket: {}, key: {}, content_type: {}",
                self.bucket_name, key, content_type);

        let presigned_config = PresigningConfig::expires_in(Duration::from_secs(3600)) // 1 hour
            .map_err(|e| anyhow::anyhow!("Failed to create presigning config: {}", e))?;

        let presigned_request = self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(&key)
            .content_type(content_type)
            .presigned(presigned_config)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to generate presigned URL: {}", e))?;

        let presigned_url = presigned_request.uri().to_string();

        // Construct the final public URL that will be accessible after upload
        let s3_api_url = env::var("S3_API_URL")?;
        let public_url = format!("{}/{}", s3_api_url, key);

        println!("DEBUG: Generated presigned URL and public URL");
        println!("DEBUG: Public URL will be: {}", public_url);

        Ok((presigned_url, public_url))
    }
}