use std::os::raw::{c_char, c_int};

use connection;
use items::Item;
use items::fetch_items_for_category;
use utils::{c_char_to_string, string_to_c_char};

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

impl Category {
    pub fn new(name: String) -> Category {
        let sql = r#"INSERT INTO categories (name) VALUES (?)"#;
        connection().execute(sql, &[&name]).unwrap();
        fetch_category(&name).unwrap()
    }
}

fn fetch_category(name: &String) -> Option<Category> {
    let sql = r#"SELECT id, name FROM categories WHERE name=?"#;

    let conn = connection();
    let mut stmt = conn.prepare(sql).unwrap();
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
                    category.items = fetch_items_for_category(&category);
                    Some(category)
                },
                Err(_) => None
            }
        },
        None => None
    }
}

fn fetch_categories() -> Vec<Category> {
    let sql = r#"SELECT id, name
                    FROM categories"#;
    let conn = connection();
    let mut stmt = conn.prepare(sql).unwrap();
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
                        category.items = fetch_items_for_category(&category);
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

#[no_mangle]
pub unsafe extern "C" fn get_all_categories() -> *mut Vec<Category> {
    let category_list = Box::new(fetch_categories());
    Box::into_raw(category_list)
}

#[no_mangle]
pub extern "C" fn category_new(name: *const c_char) -> *mut Category {
    let category = Category::new(c_char_to_string(name));
    let boxed_category = Box::new(category);
    Box::into_raw(boxed_category)
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
pub unsafe extern "C" fn category_list_destroy(category_list: *mut Vec<Category>) {
    let _ = Box::from_raw(category_list);
}

#[no_mangle]
pub unsafe extern "C" fn category_list_count(category_list: *const Vec<Category>) -> c_int {
    let category_list = &*category_list;
    category_list.len() as c_int
}
