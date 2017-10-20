use std::os::raw::c_char;
use std::ffi::{
    CString,
    CStr
};
use std::sync::{
    Arc
};

use rusqlite;
use rusqlite::Connection;

pub fn c_char_to_string(cchar: *const c_char) -> String {
    let c_str = unsafe { CStr::from_ptr(cchar) };
    let r_str = match c_str.to_str() {
        Err(_) => "",
        Ok(string) => string,
    };
    r_str.to_string()
}

pub fn string_to_c_char(r_string: String) -> *mut c_char {
    CString::new(r_string).unwrap().into_raw()
}

pub fn read_connection(uri: &String) -> Arc<Connection> {
    Arc::new(Connection::open_with_flags(uri.clone(), rusqlite::SQLITE_OPEN_READ_ONLY).unwrap())
}
