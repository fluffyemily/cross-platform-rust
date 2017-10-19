use std::os::raw::{c_char, c_int};

use time::now;
use time::Timespec;
use uuid::Uuid;

use connection;
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
    fn new(username: String, password: String) -> Login {
        let sql = r#"INSERT INTO logins (username, password, guid, time_last_used, times_used) VALUES (?1, ?2, ?3, ?4, ?5)"#;
        let time_last_used:Option<isize> = None;
        connection().execute(sql, &[&username, &password, &Uuid::new_v4().simple().to_string(), &time_last_used, &0]).unwrap();
        fetch_login(username, password).unwrap()
    }

    pub fn update_as_used(&mut self) {
        let sql = r#"UPDATE logins SET time_last_used=?1, times_used=?2 WHERE id=?3"#;
        self.times_used = self.times_used+1;
        self.time_last_used = Some(now().to_timespec());
        connection().execute(sql, &[&self.time_last_used, &self.times_used, &self.id]).unwrap();
    }
}

fn fetch_login(username: String, password: String) -> Option<Login> {
    let sql = r#"SELECT id, username, password, guid, time_created, time_last_used, time_password_changed, times_used
                    FROM logins
                    WHERE username=?
                    LIMIT 1"#;

    let conn = connection();
    let mut stmt = conn.prepare(sql).unwrap();
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

#[no_mangle]
pub unsafe extern "C" fn login_new(username: *const c_char, password: *const c_char) -> *mut Login {
    let uname = c_char_to_string(username);
    let pword = c_char_to_string(password);
    let login = Login::new(uname, pword);
    Box::into_raw(Box::new(login))
}

#[no_mangle]
pub unsafe extern "C" fn login_destroy(data: *mut Login) {
    let _ = Box::from_raw(data);
}

#[no_mangle]
pub unsafe extern "C" fn validate_login(username: *const c_char, password: *const c_char) -> c_int {
    let uname = c_char_to_string(username);
    let pword = c_char_to_string(password);
    match fetch_login(uname, pword) {
        Some(mut login) => {
            login.update_as_used();
            login.is_valid as c_int
        },
        None => 2 as c_int
    }
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

