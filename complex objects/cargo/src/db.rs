use std::os::raw::{c_char, c_int};
use std::sync::{Arc};

use rusqlite::Connection;
use rusqlite::Error;
use time::now_utc;
use uuid::Uuid;

use logins::Login;
use utils::{c_char_to_string};

#[derive(Debug)]
pub struct Store {
    pub conn: Arc<Connection>
}

impl Drop for Store {
    fn drop(&mut self) {
        println!("{:?} is being deallocated", self);
    }
}

impl Store {
    pub fn new(uri: String) -> Self {
        Store { conn: Arc::new(Connection::open(uri).unwrap()) }
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
        self.conn.execute(sql, &[]).unwrap();
    }

    pub fn fetch_login(&self, username: String, password: String) -> Option<Login> {
        let sql = r#"SELECT id, username, password, guid, time_created, time_last_used, time_password_changed, times_used
                     FROM logins
                     WHERE username=?
                     LIMIT 1"#;
        let mut stmt = self.conn.prepare(sql).unwrap();
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
                    Ok(login) => {
                        let mut l = login;
                        if l.password != password {
                            l.is_valid = 2;
                        }
                        Some(l)
                    },
                    Err(err) => {
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
        let result = self.conn.execute(sql, &[&username, &password, &Uuid::new_v4().simple().to_string(), &time_last_used, &0]).unwrap();
        self.fetch_login(username, password)
    }
}

#[no_mangle]
pub extern "C" fn new_store(uri: *const c_char) -> *mut Store {
    let uri = c_char_to_string(uri);
    let store = Store::new(uri);
    // create logins table
    store.create_logins_table();
    Box::into_raw(Box::new(store))
}

#[no_mangle]
pub unsafe extern "C" fn store_destroy(data: *mut Store) {
    let _ = Box::from_raw(data);
}

#[no_mangle]
pub unsafe extern "C" fn create_login(store: *const Store, username: *const c_char, password: *const c_char) -> *mut Login {
    let uname = c_char_to_string(username);
    let pword = c_char_to_string(password);
    let store = &*store;
    // let now = now_utc().to_timespec().sec as isize;
    // let login = Box::new(Login{
    //         id: 0,
    //         username: uname,
    //         password: pword,
    //         guid: Uuid::new_v4().simple().to_string(),
    //         time_created: now.clone(),
    //         time_last_used: None,
    //         time_password_changed: now,
    //         times_used: 0,
    //         is_valid: 1
    //     });
    let login = store.create_login(uname, pword).unwrap();
    Box::into_raw(Box::new(login))
}

#[no_mangle]
pub unsafe extern "C" fn validate_login(store: *const Store, username: *const c_char, password: *const c_char) -> c_int {
    let uname = c_char_to_string(username);
    let pword = c_char_to_string(password);
    let store = &*store;
    match store.fetch_login(uname, pword) {
        Some(login) => login.is_valid as c_int,
        None => 2 as c_int
    }
}



