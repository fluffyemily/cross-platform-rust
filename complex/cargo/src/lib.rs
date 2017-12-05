// Copyright 2016 Mozilla
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

#[macro_use] extern crate error_chain;
extern crate ffi_utils;
extern crate store;
extern crate list;

use std::os::raw::{
    c_char
};
use std::sync::{
    Arc,
};

mod errors;

use ffi_utils::strings::c_char_to_string;
use list::ListManager;
use store::Store;

pub struct Toodle {
    store: Arc<Store>,
    list: Arc<ListManager>
}

impl Toodle {
    fn new(uri: String) -> Result<Toodle, errors::Error> {
        let store = Arc::new(Store::new(uri)?);
        Ok(Toodle {
            store: Arc::clone(&store),
            list: Arc::new(ListManager::new(Arc::clone(&store))?)
        })
    }
}

#[no_mangle]
pub extern "C" fn new_toodle(uri: *const c_char) -> *mut Toodle {
    let uri = c_char_to_string(uri);
    let toodle = Toodle::new(uri).expect("expected a toodle");
    Box::into_raw(Box::new(toodle))
}

#[no_mangle]
pub unsafe extern "C" fn toodle_destroy(toodle: *mut Toodle) {
    let _ = Box::from_raw(toodle);
}

#[no_mangle]
pub unsafe extern "C" fn toodle_list(toodle: *mut Toodle) -> *mut Arc<ListManager> {
    let toodle = &*toodle;
    Box::into_raw(Box::new(toodle.list.clone()))
}
