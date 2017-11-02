// Copyright 2016 Mozilla
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

use libc::size_t;
use std::os::raw::{
    c_char,
    c_int
};
use std::sync::{
    Arc,
    Mutex
};

use rusqlite::Connection;
use time::Timespec;

use items::Item;
use utils::{
    c_char_to_string,
    read_connection,
    string_to_c_char
};

#[derive(Debug)]
pub struct CategoryManager {
    conn: Arc<Mutex<Connection>>,
    uri: String
}

impl CategoryManager {
    pub fn new(uri: String, conn: Arc<Mutex<Connection>>) -> CategoryManager {
        CategoryManager {
            conn: conn,
            uri: uri
        }
    }

    pub fn create_categories_table(&self) {
        let sql = r#"CREATE TABLE IF NOT EXISTS categories (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL
            )"#;
        let db = self.conn.lock().unwrap();
        match db.execute(sql, &[]).unwrap() {
            1 => {
                self.create_category("To Do".to_string());
            },
            _ => {}
        };
    }

    pub fn create_category(&self, name: String) -> Option<Category> {
        let sql = r#"INSERT INTO categories (name) VALUES (?)"#;
        let db = self.conn.lock().unwrap();
        db.execute(sql, &[&name]).unwrap();
        self.fetch_category(&name)
    }

    pub fn fetch_category(&self, name: &String) -> Option<Category> {
        let sql = r#"SELECT id, name FROM categories WHERE name=?"#;

        let db = read_connection(&self.uri);
        let mut stmt = db.prepare(sql).unwrap();
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
        let db = read_connection(&self.uri);
        let mut stmt = db.prepare(sql).unwrap();
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
        let db = self.conn.lock().unwrap();
        db.execute(sql, &[]).unwrap();
    }

    pub fn fetch_items_for_category(&self, category: &Category) -> Vec<Item> {
        let sql = r#"SELECT id, description, created_at, due_date, is_complete
                     FROM items
                     WHERE category=?"#;
        let db = read_connection(&self.uri);
        let mut stmt = db.prepare(sql).unwrap();
        let mut item_iter = stmt.query_map(&[&category.id], |row| {
            let complete: i64 = row.get(4);
            Item {
                id: row.get(0),
                description: row.get(1),
                created_at: row.get(2),
                due_date: row.get(3),
                is_complete: complete != 0
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

    pub fn create_item(&self, item: &Item, category_id: i64) -> Option<Item> {
        let sql = r#"INSERT INTO items (description, created_at, due_date, is_complete, category) VALUES (?, ?, ?, ?, ?)"#;
        let db = self.conn.lock().unwrap();
        let mut stmt = db.prepare(sql).unwrap();
        match stmt.insert(&[&item.description, &item.created_at, &item.due_date, &item.is_complete, &category_id]) {
            Ok(row_id) => {
                let fetch_sql = r#"SELECT id, description, created_at, due_date, is_complete FROM items WHERE rowid=?"#;
                stmt = db.prepare(fetch_sql).unwrap();
                let mut item_iter = stmt.query_map(&[&row_id], |row| {
                    let complete: i64 = row.get(4);
                    Item {
                        id: row.get(0),
                        description: row.get(1),
                        created_at: row.get(2),
                        due_date: row.get(3),
                        is_complete: complete != 0
                    }
                }).unwrap();
                match item_iter.next() {
                    Some(result) => {
                        match result {
                            Ok(item) => Some(item),
                            Err(e) => {
                                println!("Failed to fetch item {:?}", e);
                                None
                            }
                        }
                    },
                    None => {
                        None
                    }
                }
            },
            Err(e) => {
                println!("Failed to create item {:?}", e);
                None
            }
        }
    }

    pub fn update_item(&self, item: &Item) {
        let sql = r#"UPDATE items SET description=?, due_date=?, is_complete=? WHERE id=?"#;
        let db = self.conn.lock().unwrap();
        let _ = db.execute(sql, &[&item.description, &item.due_date, &item.is_complete, &item.id]);
    }
}

#[derive(Debug, Clone)]
pub struct Category {
    pub id: isize,
    pub name: String,
    pub items: Vec<Item>
}

impl Drop for Category {
    fn drop(&mut self) {
        println!("{:?} is being deallocated", self);
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_all_categories(manager: *const Arc<CategoryManager>) -> *mut Vec<Category> {
    let manager = &*manager;
    let category_list = Box::new(manager.fetch_categories());
    Box::into_raw(category_list)
}

#[no_mangle]
pub unsafe extern "C" fn category_manager_create_item(manager: *const Arc<CategoryManager>, item: *const Item, category_id: c_int) {
    let manager = &*manager;
    let item = &*item;
    let cat_id = category_id as i64;
    manager.create_item(item, cat_id);
}

#[no_mangle]
pub unsafe extern "C" fn category_manager_update_item(manager: *const Arc<CategoryManager>, item: *const Item) {
    let manager = &*manager;
    let item = &*item;
    manager.update_item(item)
}

#[no_mangle]
pub unsafe extern "C" fn category_new(manager: *const Arc<CategoryManager>, name: *const c_char) -> *mut Category {
    let manager = &*manager;
    let name = c_char_to_string(name);
    let category = Box::new(manager.create_category(name).unwrap());
    Box::into_raw(category)
}

#[no_mangle]
pub unsafe extern "C" fn category_destroy(category: *mut Category) {
    let _ = Box::from_raw(category);
}

#[no_mangle]
pub unsafe extern "C" fn category_get_id(category: *const Category) -> c_int {
    let category = &*category;
    category.id as c_int
}

#[no_mangle]
pub unsafe extern "C" fn category_get_name(category: *const Category) -> *mut c_char {
    let category = &*category;
    string_to_c_char(category.name.clone())
}

#[no_mangle]
pub unsafe extern "C" fn category_get_items(category: *const Category) -> *mut Vec<Item> {
    let category = &*category;
    let boxed_items = Box::new(category.items.clone());
    Box::into_raw(boxed_items)
}

#[no_mangle]
pub unsafe extern "C" fn category_items_count(category: *const Category) -> c_int {
    let category = &*category;
    category.items.len() as c_int
}

#[no_mangle]
pub unsafe extern "C" fn category_item_at(item_list: *const Vec<Item>, index: size_t) -> *const Item {
    let item_list = &*item_list;
    let index = index as usize;
    let item = Box::new(item_list[index].clone());
    Box::into_raw(item)
}

#[no_mangle]
pub unsafe extern "C" fn category_list_destroy(category_list: *mut Vec<Category>) {
    let _ = Box::from_raw(category_list);
}

#[no_mangle]
pub unsafe extern "C" fn category_list_count(category_list: *const Vec<Category>) -> c_int {
    let category_list = &*category_list;
    category_list.len() as c_int
}

#[no_mangle]
pub unsafe extern "C" fn category_list_item_at(category_list: *const Vec<Category>, index: size_t) -> *const Category {
    let category_list = &*category_list;
    let index = index as usize;
    let category = Box::new(category_list[index].clone());
    Box::into_raw(category)
}

#[no_mangle]
pub unsafe extern "C" fn add_category(category_list: *mut Vec<Category>, category: *const Category) {
    let mut category_list = &mut*category_list;
    let category = &*category;
    category_list.push((*category).clone())
}

#[no_mangle]
pub unsafe extern "C" fn category_add_item(category: *mut Category, item: *const Item) {
    let mut category = &mut*category;
    let item = &*item;
    category.items.push((*item).clone());
}
