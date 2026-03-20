//! Keychain integration for securely storing API keys on macOS.

use security_framework::passwords::{delete_generic_password, get_generic_password, set_generic_password};

const SERVICE_NAME: &str = "com.rustcast.app";
const ACCOUNT_NAME: &str = "ai_api_key";

/// Retrieve the AI API key from the macOS Keychain.
/// Returns `None` if no key is stored.
pub fn get_api_key() -> Option<String> {
    get_generic_password(SERVICE_NAME, ACCOUNT_NAME)
        .ok()
        .and_then(|bytes| String::from_utf8(bytes.to_vec()).ok())
}

/// Store the AI API key in the macOS Keychain.
pub fn set_api_key(key: &str) -> Result<(), String> {
    // Delete existing entry first (update requires delete + set)
    let _ = delete_generic_password(SERVICE_NAME, ACCOUNT_NAME);
    set_generic_password(SERVICE_NAME, ACCOUNT_NAME, key.as_bytes())
        .map_err(|e| format!("Failed to save API key to Keychain: {e}"))
}

/// Delete the AI API key from the macOS Keychain.
pub fn delete_api_key() -> Result<(), String> {
    delete_generic_password(SERVICE_NAME, ACCOUNT_NAME)
        .map_err(|e| format!("Failed to delete API key from Keychain: {e}"))
}
