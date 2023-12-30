use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Sets the parameters for TDLib initialization. Works only when the current authorization state is authorizationStateWaitTdlibParameters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetTdlibParameters {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type"))]
    td_type: String,
    use_test_dc: bool,
    /// The path to the directory for the persistent database; if empty, the current working directory will be used
    database_directory: String,
    /// The path to the directory for storing files; if empty, database_directory will be used
    files_directory: String,
    /// If set to true, information about downloaded and uploaded files will be saved between application restarts
    use_file_database: bool,
    /// If set to true, the library will maintain a cache of users, basic groups, supergroups, channels and secret chats. Implies use_file_database
    use_chat_info_database: bool,
    /// If set to true, the library will maintain a cache of chats and messages. Implies use_chat_info_database
    use_message_database: bool,
    /// If set to true, support for secret chats will be enabled
    use_secret_chats: bool,
    /// Application identifier for Telegram API access, which can be obtained at https://my.telegram.org
    api_id: i32,
    /// Application identifier hash for Telegram API access, which can be obtained at https://my.telegram.org
    api_hash: String,
    /// IETF language tag of the user's operating system language; must be non-empty
    system_language_code: String,
    /// Model of the device the application is being run on; must be non-empty
    device_model: String,
    /// Version of the operating system the application is being run on. If empty, the version is automatically detected by TDLib
    system_version: String,
    /// Application version; must be non-empty
    application_version: String,
    /// If set to true, old files will automatically be deleted
    enable_storage_optimizer: bool,
    /// If set to true, original file names will be ignored. Otherwise, downloaded files will be saved under names as close as possible to the original name
    ignore_file_names: bool,

    #[serde(rename(serialize = "@client_id"))]
    client_id: Option<i32>,
}

impl SetTdlibParameters {
    pub fn new(api_id: i32, api_hash: String, client_id: i32) -> Self {
        let extra = Some(Uuid::new_v4().to_string());
        Self {
            client_id: Some(client_id),
            use_file_database: false,
            use_secret_chats: false,
            system_version: "".to_string(),
            files_directory: "".to_string(),
            use_chat_info_database: false,
            use_message_database: false,
            database_directory: "tddb_auth".to_string(),
            use_test_dc: false,
            api_id,
            api_hash,
            system_language_code: "en".to_string(),
            device_model: "Desktop".to_string(),
            application_version: env!("CARGO_PKG_VERSION").to_string(),
            enable_storage_optimizer: true,
            ignore_file_names: true,
            td_type: "setTdlibParameters".to_string(),
        }
    }
}
