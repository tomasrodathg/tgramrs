use serde::{Deserialize, Serialize};

/// Sets the parameters for TDLib initialization. Works only when the current authorization state is authorizationStateWaitTdlibParameters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetAuthenticationPhoneNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type"))]
    td_type: String,

    phone_number: String,

    settings: PhoneNumberAuthenticationSettings,
}

impl SetAuthenticationPhoneNumber {
    pub fn new(phone_number: &str, settings: PhoneNumberAuthenticationSettings) -> Self {
        Self {
            phone_number: phone_number.to_string(),
            settings,
            td_type: "setAuthenticationPhoneNumber".to_string(),
        }
    }
}

// @description Contains settings for the authentication of the user's phone number
// @allow_flash_call Pass true if the authentication code may be sent via a flash call to the specified phone number
// @allow_missed_call Pass true if the authentication code may be sent via a missed call to the specified phone number
// @is_current_phone_number Pass true if the authenticated phone number is used on the current device
// @allow_sms_retriever_api For official applications only. True, if the application can use Android SMS Retriever API (requires Google Play Services >= 10.2) to automatically receive the authentication code from the SMS. See https://developers.google.com/identity/sms-retriever/ for more details
// @firebase_authentication_settings For official Android and iOS applications only; pass null otherwise. Settings for Firebase Authentication
// @authentication_tokens List of up to 20 authentication tokens, recently received in updateOption("authentication_token") in previously logged out sessions
// phoneNumberAuthenticationSettings allow_flash_call:Bool allow_missed_call:Bool is_current_phone_number:Bool allow_sms_retriever_api:Bool firebase_authentication_settings:FirebaseAuthenticationSettings authentication_tokens:vector<string> = PhoneNumberAuthenticationSettings;
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PhoneNumberAuthenticationSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type"))]
    td_type: String,

    allow_flash_call: bool,
    allow_missed_call: bool,
    is_current_phone_number: bool,
    allow_sms_retriever_api: bool,
    firebase_authentication_settings: Option<FirebaseAuthenticationSettingsIos>,
    authentication_tokens: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FirebaseAuthenticationSettingsIos {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type"))]
    td_type: String,

    device_token: String,

    is_app_sandbox: bool,
}

impl PhoneNumberAuthenticationSettings {
    pub fn new(allow_flash_call: bool, allow_missed_call: bool) -> Self {
        Self {
            allow_flash_call,
            allow_missed_call,
            allow_sms_retriever_api: false,
            authentication_tokens: Vec::new(),
            firebase_authentication_settings: None,
            is_current_phone_number: false,
            td_type: "phoneNumberAuthenticationSettings".to_string(),
        }
    }
}
