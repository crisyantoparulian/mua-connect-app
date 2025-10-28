use uuid::Uuid;

pub fn validate_uuid(uuid_str: &str) -> Result<Uuid, String> {
    Uuid::parse_str(uuid_str).map_err(|_| "Invalid UUID format".to_string())
}