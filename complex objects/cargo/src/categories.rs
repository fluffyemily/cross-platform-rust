use std::os::raw::{c_char, c_int};

use items::Item;
use utils::{c_char_to_string, string_to_c_char};

#[derive(Debug)]
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
pub extern "C" fn category_new() -> *mut Category {
    let category = Category{
        id: 0,
        name: "Open items".to_string(),
        items: Vec::new(),
    };
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
pub extern "C" fn get_all_categories() -> *mut Vec<Category> {
    let mut category_list = Vec::new();
    let category = Category{
        id: 0,
        name: "Open items".to_string(),
        items: Vec::new(),
    };
    category_list.push(category);
    let boxed_list = Box::new(category_list);
    Box::into_raw(boxed_list)
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
