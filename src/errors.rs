use serde::ser::SerializeStruct;
use serde::Serialize;
use std::convert::Infallible;
use std::fmt;
extern crate alloc;

//TODO: Utilize other error types more
#[derive(Debug, PartialEq, Eq)]
pub enum AvailErrorType {
    Internal,
    External,
    Database,
    LocalStorage,
    NotFound,
    InvalidData,
    Validation,
    Network,
    File,
    Node,
    #[cfg(feature = "snarkvm")]
    SnarkVm,
    Unauthorized,
}

impl fmt::Display for AvailErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str_value = match &self {
            AvailErrorType::Internal => "Internal",
            AvailErrorType::External => "External",
            AvailErrorType::Database => "Database",
            AvailErrorType::NotFound => "Not Found",
            AvailErrorType::InvalidData => "Invalid Data",
            AvailErrorType::Validation => "Validation",
            AvailErrorType::LocalStorage => "Local Storage",
            AvailErrorType::Network => "Network",
            AvailErrorType::File => "File",
            AvailErrorType::Node => "Node",
            #[cfg(feature = "snarkvm")]
            AvailErrorType::SnarkVm => "SnarkVm",
            AvailErrorType::Unauthorized => "Unauthorized",
        };

        write!(f, "{}", str_value)
    }
}

#[derive(Debug)]
pub struct AvailError {
    pub error_type: AvailErrorType,
    pub internal_msg: String,
    pub external_msg: String,
}

impl AvailError {
    pub fn new(
        error_type: AvailErrorType,
        internal_msg: String,
        external_msg: String,
    ) -> AvailError {
        AvailError {
            error_type,
            internal_msg,
            external_msg,
        }
    }
}

#[cfg(feature = "diesel_postgres")]
impl From<diesel::result::Error> for AvailError {
    fn from(value: diesel::result::Error) -> Self {
        Self {
            error_type: AvailErrorType::Database,
            internal_msg: format!("DieselError: {}", value),
            external_msg: "Database error".to_string(),
        }
    }
}

impl From<uuid::Error> for AvailError {
    fn from(value: uuid::Error) -> Self {
        Self {
            error_type: AvailErrorType::InvalidData,
            internal_msg: format!("UuidError: {}", value),
            external_msg: "Invalid UUID".to_string(),
        }
    }
}

impl From<reqwest::Error> for AvailError {
    fn from(value: reqwest::Error) -> Self {
        Self {
            error_type: AvailErrorType::Network,
            internal_msg: format!("ReqwestError: {}", value),
            external_msg: "Network error".to_string(),
        }
    }
}

impl From<bs58::decode::Error> for AvailError {
    fn from(value: bs58::decode::Error) -> Self {
        Self {
            error_type: AvailErrorType::InvalidData,
            internal_msg: format!("Bs58Error: {}", value),
            external_msg: "Invalid Base58".to_string(),
        }
    }
}

impl From<serde_json::Error> for AvailError {
    fn from(value: serde_json::Error) -> Self {
        Self {
            error_type: AvailErrorType::InvalidData,
            internal_msg: format!("SerdeJsonError: {}", value),
            external_msg: "Invalid JSON".to_string(),
        }
    }
}

impl From<bincode::ErrorKind> for AvailError {
    fn from(value: bincode::ErrorKind) -> Self {
        Self {
            error_type: AvailErrorType::InvalidData,
            internal_msg: format!("BincodeError: {}", value),
            external_msg: "Invalid Bincode".to_string(),
        }
    }
}

impl From<Box<bincode::ErrorKind>> for AvailError {
    fn from(value: Box<bincode::ErrorKind>) -> Self {
        Self {
            error_type: AvailErrorType::InvalidData,
            internal_msg: format!("BincodeError: {}", value),
            external_msg: "Invalid Bincode".to_string(),
        }
    }
}

impl From<std::num::TryFromIntError> for AvailError {
    fn from(value: std::num::TryFromIntError) -> Self {
        Self {
            error_type: AvailErrorType::InvalidData,
            internal_msg: format!("TryFromIntError: {}", value),
            external_msg: "Invalid TryFromIntError".to_string(),
        }
    }
}

#[cfg(feature = "snarkvm")]
impl From<snarkvm::prelude::bech32::Error> for AvailError {
    fn from(value: snarkvm::prelude::bech32::Error) -> Self {
        Self {
            error_type: AvailErrorType::InvalidData,
            internal_msg: format!("Bech32Error: {}", value),
            external_msg: "Invalid Bech32".to_string(),
        }
    }
}

impl From<app_dirs::AppDirsError> for AvailError {
    fn from(value: app_dirs::AppDirsError) -> Self {
        Self {
            error_type: AvailErrorType::LocalStorage,
            internal_msg: format!("AppDirsError: {}", value),
            external_msg: "File error".to_string(),
        }
    }
}

impl From<rusqlite::Error> for AvailError {
    fn from(value: rusqlite::Error) -> Self {
        if value.to_string().contains("no such table") {
            Self {
                error_type: AvailErrorType::NotFound,
                internal_msg: format!("RusqliteError: {}", value),
                external_msg: "Not found".to_string(),
            }
        } else {
            Self {
                error_type: AvailErrorType::Database,
                internal_msg: format!("RusqliteError: {}", value),
                external_msg: "Database error".to_string(),
            }
        }
    }
}

#[cfg(feature = "diesel_postgres")]
impl From<deadpool::managed::PoolError<diesel_async::pooled_connection::PoolError>> for AvailError {
    fn from(
        value: deadpool::managed::PoolError<diesel_async::pooled_connection::PoolError>,
    ) -> Self {
        Self {
            error_type: AvailErrorType::Database,
            internal_msg: format!("PoolError: {}", value),
            external_msg: "Database error".to_string(),
        }
    }
}

#[cfg(feature = "diesel_postgres")]
impl From<deadpool::managed::BuildError<diesel_async::pooled_connection::PoolError>>
    for AvailError
{
    fn from(
        value: deadpool::managed::BuildError<diesel_async::pooled_connection::PoolError>,
    ) -> Self {
        Self {
            error_type: AvailErrorType::Database,
            internal_msg: format!("BuildError: {}", value),
            external_msg: "Database error".to_string(),
        }
    }
}

impl From<aes_gcm::aead::Error> for AvailError {
    fn from(value: aes_gcm::aead::Error) -> Self {
        Self {
            error_type: AvailErrorType::InvalidData,
            internal_msg: format!("AesGcmError: {}", value),
            external_msg: "Invalid AES GCM".to_string(),
        }
    }
}

#[cfg(feature = "snarkvm")]
impl From<snarkvm::prelude::Error> for AvailError {
    fn from(value: snarkvm::prelude::Error) -> Self {
        Self {
            error_type: AvailErrorType::InvalidData,
            internal_msg: format!("CircuitError: {}", value),
            external_msg: "Invalid Circuit".to_string(),
        }
    }
}

#[cfg(feature = "diesel_postgres")]
impl From<diesel::ConnectionError> for AvailError {
    fn from(value: diesel::ConnectionError) -> Self {
        Self {
            error_type: AvailErrorType::Database,
            internal_msg: format!("ConnectionError: {}", value),
            external_msg: "Database error".to_string(),
        }
    }
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
impl From<security_framework::base::Error> for AvailError {
    fn from(value: security_framework::base::Error) -> Self {
        Self {
            error_type: AvailErrorType::LocalStorage,
            internal_msg: format!("iOS SecurityFrameworkError: {}", value),
            external_msg: "Local storage error".to_string(),
        }
    }
}

impl From<jni::errors::Error> for AvailError {
    fn from(value: jni::errors::Error) -> Self {
        Self {
            error_type: AvailErrorType::LocalStorage,
            internal_msg: format!("Android JNIError: {}", value),
            external_msg: "Local storage error".to_string(),
        }
    }
}

impl From<tokio::task::JoinError> for AvailError {
    fn from(value: tokio::task::JoinError) -> Self {
        Self {
            error_type: AvailErrorType::Internal,
            internal_msg: format!("Tokio JoinError: {}", value),
            external_msg: "Internal error".to_string(),
        }
    }
}

impl From<argon2::Error> for AvailError {
    fn from(value: argon2::Error) -> Self {
        Self {
            error_type: AvailErrorType::Internal,
            internal_msg: format!("Argon2 Error: {}", value),
            external_msg: "Internal error".to_string(),
        }
    }
}

impl From<alloc::string::FromUtf8Error> for AvailError {
    fn from(value: alloc::string::FromUtf8Error) -> Self {
        Self {
            error_type: AvailErrorType::InvalidData,
            internal_msg: format!("FromUtf8Error: {}", value),
            external_msg: "Invalid UTF8".to_string(),
        }
    }
}

impl From<std::io::Error> for AvailError {
    fn from(value: std::io::Error) -> Self {
        Self {
            error_type: AvailErrorType::File,
            internal_msg: format!("IOError: {}", value),
            external_msg: "Microservice initialization Fail".to_string(),
        }
    }
}

impl From<std::convert::Infallible> for AvailError {
    fn from(value: Infallible) -> Self {
        Self {
            error_type: AvailErrorType::Internal,
            internal_msg: format!("Infallible: {}", value),
            external_msg: "Internal error".to_string(),
        }
    }
}

impl From<std::num::ParseIntError> for AvailError {
    fn from(value: std::num::ParseIntError) -> Self {
        Self {
            error_type: AvailErrorType::InvalidData,
            internal_msg: format!("ParseIntError: {}", value),
            external_msg: "Invalid Int".to_string(),
        }
    }
}

impl From<hex::FromHexError> for AvailError {
    fn from(value: hex::FromHexError) -> Self {
        Self {
            error_type: AvailErrorType::InvalidData,
            internal_msg: format!("FromHexError: {}", value),
            external_msg: "Invalid Hex".to_string(),
        }
    }
}

impl From<keyring::Error> for AvailError {
    fn from(value: keyring::Error) -> Self {
        Self {
            error_type: AvailErrorType::LocalStorage,
            internal_msg: format!("KeyringError: {}", value),
            external_msg: "Local storage error".to_string(),
        }
    }
}

#[cfg(feature = "tauri")]
impl From<tauri::Error> for AvailError {
    fn from(value: tauri::Error) -> Self {
        Self {
            error_type: AvailErrorType::Internal,
            internal_msg: format!("TauriError: {}", value),
            external_msg: "Internal error".to_string(),
        }
    }
}

pub type AvailResult<T> = Result<T, AvailError>;

impl fmt::Display for AvailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Type: '{}' | Internal Msg: '{}' | External Msg: '{}'",
            self.error_type, self.internal_msg, self.external_msg
        )
    }
}

impl Serialize for AvailErrorType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type AvError = AvailError;

impl Serialize for AvailError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("AvailError", 3)?;
        state.serialize_field("error_type", &self.error_type)?;
        state.serialize_field("internal_msg", &self.internal_msg)?;
        state.serialize_field("external_msg", &self.external_msg)?;
        state.end()
    }
}
