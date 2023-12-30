# Useful links to keep this going

## Get API ID and Hash
- https://my.telegram.org/apps

## Running the app 
- It needs to have the linked 'C' library in the rustc compiler call (run by cargo). Set the env variable as: 

`export RUSTFLAGS="-C link-args=-Wl,-rpath,./td/build,-L./td/build"`

## TDLib
- Build Script Generator - https://tdlib.github.io/td/build.html?language=Rust
- Rust client lib example (not great, too convoluted and out of date) - https://github.com/antonio-antuan/rust-tdlib/blob/714e60d55fdb551fa007d8f6dfcde349d219433a/src/types/set_tdlib_parameters.rs#L7
- Docs - https://core.telegram.org/tdlib/getting-started#tdlib-glossary
- More Docs, with functions - https://core.telegram.org/tdlib/docs/td__json__client_8h.html

and most importantly...

- *Schema* - https://github.com/tdlib/td/blob/master/td/generate/scheme/td_api.tl

# TODO
- Cancel session
- Check Auth code and sign in
- Retrieve all chats
- Retrieve single chat
- Retrieve all messages in a chat
- Send messages
- Wait for message
- Exit chat
- Start new chat from contacts

## Decide on whether to use Termion or Crossterm or other tool...
