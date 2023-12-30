use serde::{Deserialize, Serialize};

//@description Checks the authentication code. Works only when the current authorization state is authorizationStateWaitCode @code Authentication code to check
// checkAuthenticationCode code:string = Ok;
//
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckAuthenticationCode<'a> {
    #[serde(rename(serialize = "@type"))]
    td_type: String,

    code: &'a str,
}

impl<'a> CheckAuthenticationCode<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            td_type: "checkAuthenticationCode".to_string(),
            code,
        }
    }
}
