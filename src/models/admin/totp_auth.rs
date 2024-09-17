use image::Luma;
use qrcode::{render::unicode, QrCode};
use rand::rngs::OsRng;
use rand::RngCore;
use totp_rs::{Algorithm, Secret, TOTP};

pub fn generate_secret_key() -> String {
    let mut secret = [0u8; 20]; // 160 bits as recommended
    OsRng.fill_bytes(&mut secret);
    base32::encode(base32::Alphabet::Rfc4648 { padding: false }, &secret)
}

pub fn generate_totp_uri(secret_key: &str, username: &str, issuer_name: &str) -> String {
    format!(
        "otpauth://totp/{}:{}?secret={}&issuer={}",
        issuer_name, username, secret_key, issuer_name
    )
}

use crate::errors::AvailResult;

pub fn create_qr_code(uri: &str) {
    let code = QrCode::new(uri.as_bytes()).unwrap();
    let image = code.render::<Luma<u8>>().build();
    image.save("totp_qr.png").unwrap();
}

pub fn verify_totp(otp: &str) -> AvailResult<bool> {
    // let secret = Secret::Encoded(secret.to_string());
    let sec_env = env!("TOTP_SECRET");
    let secret = Secret::Encoded(sec_env.to_string());

    let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, secret.to_bytes().unwrap()).unwrap();
    let now = chrono::Utc::now()
        .timestamp()
        .to_string()
        .parse::<u64>()
        .unwrap();
    let otp_u32 = otp.parse::<u32>().unwrap();
    if totp.check(otp, now) {
        Ok(true)
    } else {
        Ok(false)
    }
}

// write test cases for above functions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_secret_key() {
        let secret_key = generate_secret_key();
        assert_eq!(secret_key.len(), 32);
        println!("Secret Key: {}", secret_key);
    }

    #[test]
    fn test_generate_totp_uri() {
        let secret_key = "...";
        let username = "learnaleo-admin";
        let issuer_name = "learnaleo";
        let uri = generate_totp_uri(secret_key, username, issuer_name);
        // assert_eq!(
        //     uri,
        //     "otpauth://totp/AvailX:test_user?secret=JBSWY3DPEHPK3PXP&issuer=AvailX"
        // );
        println!("TOTP URI: {}", uri);
    }
    #[test]
    fn test_create_qr_code() {
        let uri = "...";
        create_qr_code(uri);
    }
    #[test]
    fn test_totp() {
        let otp = "993864";
        let sec_env = env!("TOTP_SECRET");

        let result = super::verify_totp(otp, &sec_env).unwrap();
        println!("Result: {}", result);
    }
}
