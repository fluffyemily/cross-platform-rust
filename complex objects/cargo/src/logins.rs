use std::os::raw::{c_char, c_int};

use time::Timespec;

use utils::{c_char_to_string, string_to_c_char};

#[derive(Debug)]
pub struct Login {
    pub id: isize,
    pub username: String,
    pub password: String,
    pub guid: String,
    pub time_created: Timespec,
    pub time_last_used: Option<Timespec>,
    pub time_password_changed: Timespec,
    pub times_used: isize,
    pub is_valid: isize
}

impl Drop for Login {
    fn drop(&mut self) {
        println!("{:?} is being deallocated", self);
    }
}

impl Login {

}

#[no_mangle]
pub unsafe extern "C" fn login_destroy(data: *mut Login) {
    // Convert a *mut Login back into a Box<Login>.
    // This function is unsafe because the Rust compiler can't know
    // whether data is actually pointing to a boxed Login.
    //
    // Note that we don't actually have to do anything else or even
    // give the new Box a name - when we convert it back to a Box
    // and then don't use it, the Rust compiler will insert the
    // necessary code to drop it (deallocating the memory).
    let _ = Box::from_raw(data);
}

#[no_mangle]
pub unsafe extern "C" fn login_get_id(login: *const Login) -> c_int {
    let login = &*login;
    login.id as c_int
}


#[no_mangle]
pub unsafe extern "C" fn login_get_username(login: *const Login) -> *mut c_char {
    let login = &*login;
    string_to_c_char(login.username.clone())
}

#[no_mangle]
pub unsafe extern "C" fn login_get_password(login: *const Login) -> *mut c_char {
    let login = &*login;
    string_to_c_char(login.password.clone())
}

#[no_mangle]
pub unsafe extern "C" fn login_get_guid(login: *const Login) -> *mut c_char {
    let login = &*login;
    string_to_c_char(login.guid.clone())
}

#[no_mangle]
pub unsafe extern "C" fn login_set_guid(login: *mut Login, guid: *const c_char) {
    let mut login = &mut *login;
    login.guid = c_char_to_string(guid);
}

#[no_mangle]
pub unsafe extern "C" fn login_get_time_created(login: *const Login) -> c_int {
    let login = &*login;
    login.time_created.sec as c_int
}

#[no_mangle]
pub unsafe extern "C" fn login_get_time_last_used(login: *const Login) -> c_int {
    let login = &*login;
    match login.time_last_used {
        Some(tm) => tm.sec as c_int,
        None => 0 as c_int
    }
}

#[no_mangle]
pub unsafe extern "C" fn login_get_time_password_changed(login: *const Login) -> c_int {
    let login = &*login;
    login.time_password_changed.sec as c_int
}

#[no_mangle]
pub unsafe extern "C" fn login_get_times_used(login: *const Login) -> c_int {
    let login = &*login;
    login.times_used as c_int
}

#[no_mangle]
pub unsafe extern "C" fn login_is_valid(login: *const Login) -> c_int {
    1 as c_int
}

