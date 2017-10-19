/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
#[macro_use]
extern crate lazy_static;

extern crate rusqlite;
extern crate time;
extern crate uuid;
extern crate r2d2;
extern crate r2d2_sqlite;

pub mod logins;
pub mod categories;
pub mod items;
pub mod utils;
pub mod db;

use r2d2_sqlite::SqliteConnectionManager;
use r2d2::{Pool, Config, PooledConnection};

use categories::Category;

lazy_static! {
    pub static ref DB_POOL: Pool<SqliteConnectionManager> = setup_db();
}

fn setup_db() -> Pool<SqliteConnectionManager> {
    let config = Config::default();
    let manager = SqliteConnectionManager::file("test.db");
    let pool = Pool::new(config, manager).unwrap();
    init_db();
    pool
}

pub fn connection() -> PooledConnection<SqliteConnectionManager> {
    let pool = DB_POOL.clone();
    pool.get().unwrap()
}

fn init_db() {
    create_logins_table();
    create_categories_table();
    create_items_table();
}

fn create_logins_table() {
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
    connection().execute(sql, &[]).unwrap();
}

fn create_categories_table() {
    let sql = r#"CREATE TABLE IF NOT EXISTS categories (
            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        )"#;
    match connection().execute(sql, &[]).unwrap() {
        1 => {
            let _ = Category::new("To Do".to_string());
        },
        _ => {}
    };
}

fn create_items_table() {
    let sql = r#"CREATE TABLE IF NOT EXISTS items (
            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            description TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            due_date DATETIME,
            is_complete TINYINT DEFAULT 0,
            category REFERENCES categories(id)
        )"#;
    connection().execute(sql, &[]).unwrap();
}
