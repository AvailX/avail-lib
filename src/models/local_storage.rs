use crate::errors::{AvailError, AvailErrorType, AvailResult};

pub fn try_label_to_account_str(label: &str) -> AvailResult<String> {
    let account_str = match label {
        "avl-p" => "avail-user-private",
        "avl-v" => "avail-user-view",
        _ => {
            return Err(AvailError::new(
                AvailErrorType::LocalStorage,
                "Error retrieving key from keychain".to_string(),
                "Error retrieving key from keychain".to_string(),
            ))
        }
    };

    Ok(account_str.to_string())
}

pub fn try_get_auth_type(auth_type: &str) -> AvailResult<bool> {
    let auth_type = match auth_type {
        "true" => true,
        "false" => false,
        _ => {
            return Err(AvailError::new(
                AvailErrorType::InvalidData,
                "AuthType is not true or false".to_string(),
                "AuthType is not true or false".to_string(),
            ))
        }
    };

    Ok(auth_type)
}
