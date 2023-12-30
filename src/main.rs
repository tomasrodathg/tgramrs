#![deny(clippy::perf, clippy::all, clippy::pedantic)]

use tgramrs::{
    get_code_from_user_and_send, get_current_state, new_client, receive,
    set_authentication_phone_number, set_tdlib_parameters,
    tdlib::{
        set_authentication_phone_number::{
            self, PhoneNumberAuthenticationSettings, SetAuthenticationPhoneNumber,
        },
        td_lib_parameters::SetTdlibParameters,
    },
};

fn main() {
    let client = new_client();

    let set_td_lib_params = SetTdlibParameters::new(
        28213303,
        "f893d4dbee2a8f5b085ada6c5cb6329d".to_string(),
        client,
    );

    // temporary way to start the auth process
    get_current_state(client);

    let set_auth_phone_number_req = SetAuthenticationPhoneNumber::new(
        "+447393343884",
        PhoneNumberAuthenticationSettings::new(true, false),
    );

    loop {
        let res = receive(30.0).unwrap();

        match res {
            res if res.contains("authorizationStateWaitTdlibParameters") => {
                set_tdlib_parameters(client, &set_td_lib_params);
            }
            res if res.contains("authorizationStateWaitPhoneNumber") => {
                set_authentication_phone_number(client, &set_auth_phone_number_req);
            }
            res if res.contains("authorizationStateWaitCode") => {
                get_code_from_user_and_send(client);
            }
            _ => println!("Received response: {res}"),
        }
    }
}
