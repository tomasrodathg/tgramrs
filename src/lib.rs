// comment
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int};

use tdlib::auth::AuthSendCode;
use tdlib::check_authentication_code::CheckAuthenticationCode;
use tdlib::set_authentication_phone_number::SetAuthenticationPhoneNumber;
use tdlib::td_lib_parameters::SetTdlibParameters;
pub mod tdlib;

pub type ClientId = i32;

#[link(name = "tdjson")]
extern "C" {
    fn td_create_client_id() -> c_int;
    fn td_send(client_id: c_int, request: *const c_char);
    fn td_receive(timeout: c_double) -> *const c_char;
}

pub fn new_client() -> ClientId {
    unsafe { td_create_client_id() }
}

fn send(client_id: i32, request: String) {
    let request = CString::new(request.as_str()).unwrap();
    let client_id = client_id as c_int;

    println!("c request: {:?}", request);
    unsafe { td_send(client_id, request.as_ptr()) }
}

pub fn send_auth_code(client_id: i32, auth_send_code: AuthSendCode) {
    let auth_send_code_req =
        serde_json::ser::to_string(&auth_send_code).expect("Failed to serialize request");

    println!("{auth_send_code_req}");
    send(client_id, auth_send_code_req);
}

pub fn get_current_state(client_id: i32) {
    let get_current_state_req = format!(
        r#"{{"@type":"getCurrentState","@client_id":{}}}"#,
        client_id
    );

    send(client_id, get_current_state_req);
}

pub fn set_tdlib_parameters(client_id: i32, tdlib_params: &SetTdlibParameters) {
    let set_tdlib_params_req = serde_json::ser::to_string(tdlib_params).unwrap();

    println!("{set_tdlib_params_req}");
    send(client_id, set_tdlib_params_req);
}

pub fn set_authentication_phone_number(
    client_id: i32,
    set_authentication_phone_number: &SetAuthenticationPhoneNumber,
) {
    let set_authentication_phone_number_req =
        serde_json::ser::to_string(set_authentication_phone_number).unwrap();

    println!("{set_authentication_phone_number_req}");
    send(client_id, set_authentication_phone_number_req);
}

pub fn check_authentication_code(
    client_id: i32,
    check_authentication_code_request: &CheckAuthenticationCode,
) {
    let check_authentication_code_request =
        serde_json::ser::to_string(check_authentication_code_request).unwrap();

    #[cfg(debug_assertions)]
    println!("{check_authentication_code_request}");
    send(client_id, check_authentication_code_request);
}

pub fn get_code_from_user_and_send(client_id: i32) {
    use std::io::{self, Write};

    // Print a prompt
    print!("Enter code: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed

    // Read input
    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();

    // Process the input
    println!("Submiting code: {}", code.trim());

    let check_authentication_code_request = CheckAuthenticationCode::new(&code);

    check_authentication_code(client_id, &check_authentication_code_request);
}

pub fn receive(timeout: f32) -> Option<String> {
    unsafe {
        td_receive(timeout as c_double)
            .as_ref()
            .map(|response| CStr::from_ptr(response).to_string_lossy().to_string())
    }
}
