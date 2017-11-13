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
    c_int,
};
use std::ptr;

use time::Timespec;
use time::now;

use ffi_utils::{
    string_to_c_char,
    c_char_to_string,
};

#[derive(Debug, Clone)]
pub struct Item {
    pub id: Option<isize>,
    pub description: String,
    pub created_at: Timespec,
    pub due_date: Option<Timespec>,
    pub is_complete: bool
}

impl Drop for Item {
    fn drop(&mut self) {
        println!("{:?} is being deallocated", self);
    }
}

#[no_mangle]
pub extern "C" fn item_new() -> *mut Item {
    let item = Item{
        id: None,
        description: "".to_string(),
        created_at: now().to_timespec(),
        due_date: None,
        is_complete: false
    };
    let boxed_item = Box::new(item);
    Box::into_raw(boxed_item)
}

#[no_mangle]
pub unsafe extern "C" fn item_destroy(item: *mut Item) {
    let _ = Box::from_raw(item);
}

#[no_mangle]
pub unsafe extern "C" fn item_get_id(item: *const Item) -> *mut c_int {
    let item = &*item;
    match item.id {
        Some(id) => Box::into_raw(Box::new(id as c_int)),
        None => ptr::null_mut()
    }

}

#[no_mangle]
pub unsafe extern "C" fn item_get_description(item: *const Item) -> *mut c_char {
    let item = &*item;
    string_to_c_char(item.description.clone())
}

#[no_mangle]
pub unsafe extern "C" fn item_set_description(item: *mut Item, description: *const c_char) {
    let item = &mut*item;
    item.description = c_char_to_string(description);
}

#[no_mangle]
pub unsafe extern "C" fn item_get_created_at(item: *const Item) -> c_int {
    let item = &*item;
    item.created_at.sec as c_int
}

#[no_mangle]
pub unsafe extern "C" fn item_get_due_date(item: *const Item) -> *mut i64 {
    let item = &*item;
    match item.due_date {
        Some(date) => {
            println!("item_get_due_date: returning {:?} for {:?}", date.sec, item.description);
            Box::into_raw(Box::new(date.sec))
        },
        None => {
            println!("item_get_due_date: returning null_mut for {:?}", item.description);
            ptr::null_mut()
        }
    }

}

#[no_mangle]
pub unsafe extern "C" fn item_set_due_date(item: *mut Item, due_date: *const size_t) {
    let item = &mut*item;
    if !due_date.is_null() {
        item.due_date = Some(Timespec::new(due_date as i64, 0));
    } else {
        item.due_date = None;
    }
}

#[no_mangle]
pub unsafe extern "C" fn item_get_is_complete(item: *const Item) -> c_int {
    let item = &*item;
    item.is_complete as c_int
}

#[no_mangle]
pub unsafe extern "C" fn item_set_is_complete(item: *mut Item, is_complete: size_t) {
    let item = &mut*item;
    let is_complete = is_complete as usize;
    match is_complete {
        0 => item.is_complete = false,
        _ => item.is_complete = true
    }
}
