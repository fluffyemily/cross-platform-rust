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

use ffi_utils::strings::string_to_c_char;
use items::Item;

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
    let category_list = &mut*category_list;
    let category = &*category;
    category_list.push((*category).clone())
}

#[no_mangle]
pub unsafe extern "C" fn category_add_item(category: *mut Category, item: *const Item) {
    let category = &mut*category;
    let item = &*item;
    category.items.push((*item).clone());
}
