use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "@type")]
pub enum JsonValue {
    #[serde(rename = "jsonValueString")]
    String(JsonValueString),
    #[serde(rename = "jsonValueNumber")]
    Number(JsonValueNumber),
    #[serde(rename = "jsonValueBoolean")]
    Boolean(JsonValueBoolean),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JsonValueBoolean {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The value

    #[serde(default)]
    value: bool,
}

impl JsonValueBoolean {
    pub fn new(value: bool) -> Self {
        Self {
            value,
            client_id: None,
            extra: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonValueString {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The value

    #[serde(default)]
    value: String,
}

impl JsonValueString {
    pub fn new(value: String) -> Self {
        Self {
            extra: None,
            client_id: None,
            value,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JsonValueNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The value

    #[serde(default)]
    value: i32,
}

impl JsonValueNumber {
    pub fn new(value: i32) -> Self {
        Self {
            extra: None,
            client_id: None,
            value,
        }
    }
}
