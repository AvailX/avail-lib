use crate::errors::{AvailError, AvailResult};
use actix_web::{web, HttpRequest, HttpResponse};
use reqwest::header;

use super::totp_auth::verify_totp;

pub async fn verify_admin(req: HttpRequest) -> AvailResult<bool> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");

    // Typically, the Authorization header is "Bearer {token}", so we split and take the token part
    let secret = if auth_header.starts_with("Bearer ") {
        auth_header.strip_prefix("Bearer ")
    } else {
        return Err(AvailError::new(
            crate::errors::AvailErrorType::Internal,
            "Invalid Authorization header".to_string(),
            "Invalid Authorization header".to_string(),
        ));
    };

    match verify_totp(secret.unwrap()) {
        Ok(res) => Ok(res),
        Err(_) => Err(AvailError::new(
            crate::errors::AvailErrorType::Internal,
            "Error verifying TOTP".to_string(),
            "Error verifying TOTP".to_string(),
        )),
    }
}
