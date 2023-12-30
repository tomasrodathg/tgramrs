/// This module defines the JSON model
/// to interact with the authentication features of TDlib
use serde::{Deserialize, Serialize};

use super::types::json_value::{JsonValue, JsonValueBoolean, JsonValueNumber, JsonValueString};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthSendCode {
    phone_number: JsonValue,
    api_id: JsonValue,
    api_hash: JsonValue,
    settings: CodeSettings,
}

impl AuthSendCode {
    pub fn new(phone_number: &str, api_id: i32, api_hash: &str, settings: CodeSettings) -> Self {
        Self {
            phone_number: JsonValue::String(JsonValueString::new(phone_number.to_string())),
            api_id: JsonValue::Number(JsonValueNumber::new(api_id)),
            api_hash: JsonValue::String(JsonValueString::new(api_hash.to_string())),
            settings,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "@type")]
pub struct CodeSettings {
    allow_flashcall: JsonValue,
}

impl CodeSettings {
    pub fn new() -> Self {
        Self {
            allow_flashcall: JsonValue::Boolean(JsonValueBoolean::new(false)),
        }
    }
}
