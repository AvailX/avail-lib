use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::errors::AvailResult;

#[derive(Deserialize, Serialize)]
pub struct CreateSessionRequest {
    pub public_key: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VerifySessionRequest {
    pub signature: String,
    pub session_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateSessionResponse {
    pub hash: String,
    pub session_id: Uuid,
    pub expires_on: DateTime<Utc>,
}
/* User Verification */
#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyUserRequest {
    pub address: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyUserResponse {
    pub exists: bool,
    pub backup: bool,
}
/* Session */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub address: String,
    pub session_id: Uuid,
}

impl Session {
    pub fn new(address: String, session_id: Uuid) -> Self {
        Self {
            address,
            session_id,
        }
    }

    pub fn from_str(session_string: &str) -> AvailResult<Self> {
        let session: Session = match serde_json::from_str(session_string) {
            Ok(session) => session,
            Err(e) => {
                tracing::error!("Error deserializing session: {}", e);
                return Err(e.into());
            }
        };

        Ok(session)
    }
}

#[test]
fn test_encrypt_view_key() {
    use snarkvm::prelude::*;

    use crate::aleo_tools::encryptor::Encryptor;

    let mut rng = TestRng::default();
    let private_key = PrivateKey::<Testnet3>::new(&mut rng).unwrap();
    let view_key = ViewKey::<Testnet3>::try_from(&private_key).unwrap();
    let enc = Encryptor::<Testnet3>::encrypt_view_key_with_secret(&view_key, "mypassword").unwrap();
    let recovered_view_key =
        Encryptor::<Testnet3>::decrypt_view_key_with_secret(&enc, "mypassword").unwrap();
    assert_eq!(view_key, recovered_view_key);
}
