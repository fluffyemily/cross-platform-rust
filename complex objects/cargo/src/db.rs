use std::os::raw::{c_char, c_int};
use std::sync::{Arc};

use rusqlite::Connection;
use time::now;
use uuid::Uuid;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::{Pool, Config, PooledConnection};

use logins::Login;
use categories::Category;
use items::Item;
use utils::{c_char_to_string};

lazy_static! {
    pub static ref DB_POOL: Pool<SqliteConnectionManager> = setup_db();
}

fn setup_db() -> Pool<SqliteConnectionManager> {
    let config = Config::default();
    let manager = SqliteConnectionManager::file("test.db");
    Pool::new(config, manager).unwrap()
}

fn connection() -> PooledConnection<SqliteConnectionManager> {
    let pool = DB_POOL.clone();
    pool.get().unwrap()
}

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
        self.conn.execute(sql, &[&username, &password, &Uuid::new_v4().simple().to_string(), &time_last_used, &0]).unwrap();
        self.fetch_login(username, password)
    }

    pub fn update_login_as_used(&self, login: &mut Login) {
        let sql = r#"UPDATE logins SET time_last_used=?1, times_used=?2 WHERE id=?3"#;
        login.times_used = login.times_used+1;
        login.time_last_used = Some(now().to_timespec());
        self.conn.execute(sql, &[&login.time_last_used, &login.times_used, &login.id]).unwrap();
    }

    pub fn create_categories_table(&self) {
        let sql = r#"CREATE TABLE IF NOT EXISTS categories (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL
            )"#;
        match self.conn.execute(sql, &[]).unwrap() {
            1 => {
                self.create_category("To Do".to_string());
            },
            _ => {}
        };
    }

    pub fn create_category(&self, name: String) -> Option<Category> {
        let sql = r#"INSERT INTO categories (name) VALUES (?)"#;
        self.conn.execute(sql, &[&name]).unwrap();
        self.fetch_category(&name)
    }

    pub fn fetch_category(&self, name: &String) -> Option<Category> {
        let sql = r#"SELECT id, name FROM categories WHERE name=?"#;

        let mut stmt = self.conn.prepare(sql).unwrap();
        let mut category_iter = stmt.query_map(&[name], |row| {
            Category {
                id: row.get(0),
                name: row.get(1),
                items: Vec::new()
            }
        }).unwrap();

        match category_iter.next() {
            Some(result) => {
                match result {
                    Ok(mut category) => {
                        category.items = self.fetch_items_for_category(&category);
                        Some(category)
                    },
                    Err(_) => None
                }
            },
            None => None
        }
    }

    pub fn fetch_categories(&self) -> Vec<Category> {
        let sql = r#"SELECT id, name
                     FROM categories"#;
        let mut stmt = self.conn.prepare(sql).unwrap();
        let mut category_iter = stmt.query_map(&[], |row| {
            Category {
                id: row.get(0),
                name: row.get(1),
                items: Vec::new()
            }
        }).unwrap();

        let mut category_list: Vec<Category> = Vec::new();
        loop {
            match category_iter.next() {
                Some(result) => {
                    match result {
                        Ok(mut category) => {
                            category.items = self.fetch_items_for_category(&category);
                            category_list.push(category);
                        },
                        Err(_) => {}
                    }
                },
                None => break
            }
        }
        category_list
    }

    pub fn create_items_table(&self) {
        let sql = r#"CREATE TABLE IF NOT EXISTS items (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                description TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                due_date DATETIME,
                is_complete TINYINT DEFAULT 0,
                category REFERENCES categories(id)
            )"#;
        self.conn.execute(sql, &[]).unwrap();
    }

    pub fn fetch_items_for_category(&self, category: &Category) -> Vec<Item> {
        let sql = r#"SELECT id, description, created_at, due_date, is_complete
                     FROM items
                     WHERE category=?"#;
        let mut stmt = self.conn.prepare(sql).unwrap();
        let mut item_iter = stmt.query_map(&[&category.id], |row| {
            Item {
                id: row.get(0),
                description: row.get(1),
                created_at: row.get(2),
                due_date: row.get(3),
                is_complete: row.get(4)
            }
        }).unwrap();

        let mut item_list: Vec<Item> = Vec::new();

        loop {
            match item_iter.next() {
                Some(result) => {
                    match result {
                        Ok(i) => {
                            item_list.push(i);
                        },
                        Err(_) => {}
                    }
                },
                None => break
            }
        }
        item_list
    }
}

#[no_mangle]
pub extern "C" fn new_store(uri: *const c_char) -> *mut Store {
    let uri = c_char_to_string(uri);
    let store = Store::new(uri);
    // create tables
    store.create_logins_table();
    store.create_categories_table();
    store.create_items_table();
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
    let login = store.create_login(uname, pword).unwrap();
    Box::into_raw(Box::new(login))
}

#[no_mangle]
pub unsafe extern "C" fn validate_login(store: *const Store, username: *const c_char, password: *const c_char) -> c_int {
    let uname = c_char_to_string(username);
    let pword = c_char_to_string(password);
    let store = &*store;
    match store.fetch_login(uname, pword) {
        Some(mut login) => {
            store.update_login_as_used(&mut login);
            login.is_valid as c_int
        },
        None => 2 as c_int
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_all_categories(store: *const Store) -> *mut Vec<Category> {
    let store = &*store;
    let category_list = Box::new(store.fetch_categories());
    Box::into_raw(category_list)
}

#[no_mangle]
pub unsafe extern "C" fn create_category(store: *const Store, name: *const c_char) -> *mut Category {
    let store = &*store;
    let name = c_char_to_string(name);
    let category = Box::new(store.create_category(name).unwrap());
    Box::into_raw(category)
}



