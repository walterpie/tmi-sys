use std::ffi::{c_void, CStr, CString};
use std::os::raw::{c_char, c_int};

use tmi_sys::*;

struct Userdata;

extern "C" fn catch(_client: *mut TmiClient, _e: *mut TmiObject) {
    eprintln!("encountered an error!");
}

extern "C" fn on_chat(
    client: *mut TmiClient,
    channel: *const c_char,
    userstate: *mut TmiObject,
    msg: *const c_char,
    self_: c_int,
) {
    let _userdata = unsafe { &mut *(tmi_userdata(client) as *mut Userdata) };
    let channel = unsafe { CStr::from_ptr(channel) };
    let msg = unsafe { CStr::from_ptr(msg) };
    let msg = msg.to_string_lossy();
    let self_ = self_ != 0;

    let username_obj = unsafe { tmi_object_get(userstate, b"display-name\0".as_ptr() as _) };
    let username = unsafe { tmi_object_to_string(username_obj) };
    let _username = unsafe { CStr::from_ptr(username) };

    if self_ {
        return;
    }

    if msg.starts_with("!echo ") {
        let msg = CString::new(&msg["!echo ".len()..]).expect("message contained a nul-byte");
        unsafe {
            let promise = tmi_client_say(client, channel.as_ptr(), msg.as_ptr());
            tmi_promise_or_else(promise, Some(catch));
        }
    }

    unsafe {
        tmi_del_object(username_obj);
    }
}

#[no_mangle]
pub extern "C" fn tmicxx_main(client: *mut TmiClient) {
    let mut userdata = Userdata;
    let userdata = &mut userdata as *mut _ as *mut c_void;

    unsafe {
        tmi_connect(client, userdata);
        tmi_on_chat(client, Some(on_chat));
    }
}
