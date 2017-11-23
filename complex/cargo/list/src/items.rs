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

use ffi_utils::strings::{
    string_to_c_char,
    c_char_to_string,
};
use labels::Label;

#[derive(Debug, Clone)]
pub struct Item {
    pub uuid: String,
    pub name: String,
    pub due_date: Option<Timespec>,
    pub completion_date: Option<Timespec>,
    pub labels: Vec<Label>,
}

impl Drop for Item {
    fn drop(&mut self) {
        println!("{:?} is being deallocated", self);
    }
}

#[no_mangle]
pub extern "C" fn item_new() -> *mut Item {
    let item = Item{
        uuid: "".to_string(),
        name: "".to_string(),
        due_date: None,
        completion_date: None,
        labels: vec![]
    };
    let boxed_item = Box::new(item);
    Box::into_raw(boxed_item)
}

#[no_mangle]
pub unsafe extern "C" fn item_destroy(item: *mut Item) {
    let _ = Box::from_raw(item);
}

#[no_mangle]
pub unsafe extern "C" fn item_get_name(item: *const Item) -> *mut c_char {
    let item = &*item;
    string_to_c_char(item.name.clone())
}

#[no_mangle]
pub unsafe extern "C" fn item_set_name(item: *mut Item, name: *const c_char) {
    let item = &mut*item;
    item.name = c_char_to_string(name);
}

#[no_mangle]
pub unsafe extern "C" fn item_get_due_date(item: *const Item) -> *mut i64 {
    let item = &*item;
    match item.due_date {
        Some(date) => {
            println!("item_get_due_date: returning {:?} for {:?}", date.sec, item.name);
            Box::into_raw(Box::new(date.sec))
        },
        None => {
            println!("item_get_due_date: returning null_mut for {:?}", item.name);
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
pub unsafe extern "C" fn item_get_completion_date(item: *const Item) -> *mut i64 {
    let item = &*item;
    match item.completion_date {
        Some(date) => {
            println!("item_get_due_date: returning {:?} for {:?}", date.sec, item.name);
            Box::into_raw(Box::new(date.sec))
        },
        None => {
            println!("item_get_due_date: returning null_mut for {:?}", item.name);
            ptr::null_mut()
        }
    }

}

#[no_mangle]
pub unsafe extern "C" fn item_set_completion_date(item: *mut Item, completion_date: *const size_t) {
    let item = &mut*item;
    if !completion_date.is_null() {
        item.completion_date = Some(Timespec::new(completion_date as i64, 0));
    } else {
        item.completion_date = None;
    }
}

#[no_mangle]
pub unsafe extern "C" fn item_get_labels(item: *const Item) -> *mut Vec<Label> {
    let item = &*item;
    let boxed_labels = Box::new(item.labels.clone());
    Box::into_raw(boxed_labels)
}

#[no_mangle]
pub unsafe extern "C" fn item_labels_count(item: *const Item) -> c_int {
    let item = &*item;
    item.labels.len() as c_int
}

#[no_mangle]
pub unsafe extern "C" fn item_label_at(label_list: *const Vec<Label>, index: size_t) -> *const Label {
    let label_list = &*label_list;
    let index = index as usize;
    let label = Box::new(label_list[index].clone());
    Box::into_raw(label)
}
