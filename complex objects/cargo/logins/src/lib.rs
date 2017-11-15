// Copyright 2016 Mozilla
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

extern crate rusqlite;
extern crate time;
extern crate uuid;
extern crate ffi_utils;
extern crate store;

use std::os::raw::{
    c_char,
    c_int}
;
use std::sync::{
    Arc,
};
use time::{
    now,
    Timespec
};
use uuid::Uuid;

use ffi_utils::strings::{
    c_char_to_string,
    string_to_c_char
};
use store::Store;

#[derive(Debug)]
#[repr(C)]
pub struct LoginManager {
    pub store: Arc<Store>,
}

impl LoginManager {
    pub fn new(store: Arc<Store>) -> LoginManager {
        let manager = LoginManager {
            store: store,
        };
        manager.create_logins_table();
        manager
    }

    pub fn create_logins_table(&self) {
        let sql = r#"CREATE TABLE IF NOT EXISTS logins (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL,
                guid TEXT NOT NULL,
                time_created DATETIME DEFAULT CURRENT_TIMESTAMP,
                time_last_used DATETIME,
                time_password_changed DATETIME DEFAULT CURRENT_TIMESTAMP,
                times_used INTEGER DEFAULT 0
            )"#;
        let conn = self.store.write_connection();
        conn.execute(sql, &[]).unwrap();
    }

    pub fn fetch_login(&self, username: String, password: String) -> Option<Login> {
        let sql = r#"SELECT id, username, password, guid, time_created, time_last_used, time_password_changed, times_used
                    FROM logins
                    WHERE username=?
                    LIMIT 1"#;
        let db = self.store.read_connection();
        let mut stmt = db.prepare(sql).unwrap();
        let mut login_iter = stmt.query_map(&[&username], |row| {
            Login {
                id: row.get(0),
                username: row.get(1),
                password: row.get(2),
                guid: row.get(3),
                time_created: row.get(4),
                time_last_used: row.get(5),
                time_password_changed: row.get(6),
                times_used: row.get(7),
                is_valid: LoginStatus::Valid
            }
        }).unwrap();

        if let Some(result) = login_iter.next() {
            match result.ok() {
                Some(mut login) => {
                    if login.password != password {
                        login.is_valid = LoginStatus::IncorrectPassword;
                    }
                    Some(login)
                },
                None => None
            }
        } else {
            println!("No item found");
            None
        }
    }

    pub fn create_login(&self, username: String, password: String) -> Option<Login> {
        let sql = r#"INSERT INTO logins (username, password, guid, time_last_used, times_used) VALUES (?1, ?2, ?3, ?4, ?5)"#;
        let time_last_used:Option<isize> = None;
        let db = self.store.write_connection();
        db.execute(sql, &[&username, &password, &Uuid::new_v4().simple().to_string(), &time_last_used, &0]).unwrap();
        self.fetch_login(username, password)
    }

    pub fn update_login_as_used(&self, login: &mut Login) {
        let sql = r#"UPDATE logins SET time_last_used=?1, times_used=?2 WHERE id=?3"#;
        login.times_used = login.times_used+1;
        login.time_last_used = Some(now().to_timespec());

        let db = self.store.write_connection();
        db.execute(sql, &[&login.time_last_used, &login.times_used, &login.id]).unwrap();
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub enum LoginStatus {
    Valid,
    UnknownUsername,
    IncorrectPassword,
    Invalid,
}

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
    pub is_valid: LoginStatus
}

impl Drop for Login {
    fn drop(&mut self) {
        println!("{:?} is being deallocated", self);
    }
}

impl Login {

}

#[no_mangle]
pub unsafe extern "C" fn create_login(manager: *const Arc<LoginManager>, username: *const c_char, password: *const c_char) -> *mut Login {
    let uname = c_char_to_string(username);
    let pword = c_char_to_string(password);
    let manager = &*manager;
    let login = manager.create_login(uname, pword).unwrap();
    Box::into_raw(Box::new(login))
}

#[no_mangle]
pub unsafe extern "C" fn validate_login(manager: *const Arc<LoginManager>, username: *const c_char, password: *const c_char) -> LoginStatus {
    let uname = c_char_to_string(username);
    let pword = c_char_to_string(password);
    let manager = &*manager;
    match manager.fetch_login(uname, pword) {
        Some(mut login) => {
            manager.update_login_as_used(&mut login);
            login.is_valid
        },
        None => LoginStatus::IncorrectPassword
    }
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
    let login = &mut *login;
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
pub unsafe extern "C" fn login_is_valid(login: *const Login) -> LoginStatus {
    let login = &*login;
    login.is_valid
}
