// Copyright 2016 Mozilla
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

use std::os::raw::c_char;

use mentat_core::TypedValue;

use ffi_utils::strings::{
    string_to_c_char,
    c_char_to_string,
};
use store::{
    Entity,
    ToInner
};

#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    pub id: Option<Entity>,    // id should not be leaked outside of the library
    pub name: String,
    pub color: String
}

impl Drop for Label {
    fn drop(&mut self) {
        println!("{:?} is being deallocated", self);
    }
}

impl Label {
    pub fn from_row(row: &Vec<TypedValue>) -> Option<Label> {
        Some(Label {
            id: row[0].clone().to_inner(),
            name: row[1].clone().to_inner(),
            color: row[2].clone().to_inner()
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn label_destroy(label: *mut Label) {
    let _ = Box::from_raw(label);
}

#[no_mangle]
pub unsafe extern "C" fn label_get_name(label: *const Label) -> *mut c_char {
    let label = &*label;
    string_to_c_char(label.name.clone())
}

#[no_mangle]
pub unsafe extern "C" fn label_get_color(label: *const Label) -> *mut c_char {
    let label = &*label;
    string_to_c_char(label.color.clone())
}

#[no_mangle]
pub unsafe extern "C" fn label_set_color(label: *mut Label, color: *const c_char) {
    let label = &mut*label;
    label.color = c_char_to_string(color);
}
