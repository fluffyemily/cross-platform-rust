use std::os::raw::{
    c_char,
    c_int}
;
use std::sync::{
    Arc,
    Mutex
};

use rusqlite::Connection;
use time::{
    now,
    Timespec
};
use uuid::Uuid;

use utils::{
    c_char_to_string,
    read_connection,
    string_to_c_char
};

#[derive(Debug)]
pub struct LoginManager {
    pub conn: Arc<Mutex<Connection>>,
    uri: String
}

impl LoginManager {
    pub fn new(uri: String, conn: Arc<Mutex<Connection>>) -> LoginManager {
        LoginManager {
            conn: conn,
            uri: uri
        }
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
        let db = self.conn.lock().unwrap();
        db.execute(sql, &[]).unwrap();
    }

    pub fn fetch_login(&self, username: String, password: String) -> Option<Login> {
        let sql = r#"SELECT id, username, password, guid, time_created, time_last_used, time_password_changed, times_used
                     FROM logins
                     WHERE username=?
                     LIMIT 1"#;
        let db = read_connection(&self.uri);
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
                is_valid: 0
            }
        }).unwrap();
        match login_iter.next() {
            Some(result) => {
                match result {
                    Ok(mut login) => {
                        if login.password != password {
                            login.is_valid = 2;
                        }
                        Some(login)
                    },
                    Err(_) => {
                        None
                    }
                }
            },
            None => None
        }
    }

    pub fn create_login(&self, username: String, password: String) -> Option<Login> {
        let sql = r#"INSERT INTO logins (username, password, guid, time_last_used, times_used) VALUES (?1, ?2, ?3, ?4, ?5)"#;
        let time_last_used:Option<isize> = None;
        let db = self.conn.lock().unwrap();
        db.execute(sql, &[&username, &password, &Uuid::new_v4().simple().to_string(), &time_last_used, &0]).unwrap();
        self.fetch_login(username, password)
    }

    pub fn update_login_as_used(&self, login: &mut Login) {
        let sql = r#"UPDATE logins SET time_last_used=?1, times_used=?2 WHERE id=?3"#;
        login.times_used = login.times_used+1;
        login.time_last_used = Some(now().to_timespec());

        let db = self.conn.lock().unwrap();
        db.execute(sql, &[&login.time_last_used, &login.times_used, &login.id]).unwrap();
    }
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
pub unsafe extern "C" fn create_login(manager: *const Arc<LoginManager>, username: *const c_char, password: *const c_char) -> *mut Login {
    let uname = c_char_to_string(username);
    let pword = c_char_to_string(password);
    let manager = &*manager;
    let login = manager.create_login(uname, pword).unwrap();
    Box::into_raw(Box::new(login))
}

#[no_mangle]
pub unsafe extern "C" fn validate_login(manager: *const Arc<LoginManager>, username: *const c_char, password: *const c_char) -> c_int {
    let uname = c_char_to_string(username);
    let pword = c_char_to_string(password);
    let manager = &*manager;
    match manager.fetch_login(uname, pword) {
        Some(mut login) => {
            manager.update_login_as_used(&mut login);
            login.is_valid as c_int
        },
        None => 2 as c_int
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
    let login = &*login;
    login.is_valid as c_int
}

