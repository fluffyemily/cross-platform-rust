// Copyright 2016 Mozilla
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

extern crate rusqlite;
extern crate ffi_utils;

use std::os::raw::{
    c_char
};
use std::rc::Rc;
use std::sync::{Mutex, MutexGuard};

use rusqlite::{
    Connection
};

use ffi_utils::strings::c_char_to_string;

#[derive(Debug, Clone)]
#[repr(C)]
/// Store containing a SQLite connection
pub struct Store {
    pub conn: Rc<Mutex<Connection>>,
    uri: Option<String>,
}

impl Drop for Store {
    fn drop(&mut self) {
        println!("{:?} is being deallocated", self);
    }
}

impl Store {
    pub fn new<T>(uri: T) -> Self
    where T: Into<Option<String>> {
        let uri_string = uri.into();
        let c = match &uri_string {
            &Some(ref u) => Connection::open(u.clone()).expect("Expected a connection for URI"),
            &None => Connection::open_in_memory().expect("Expected an in memory connection"),
        };
        Store {
            conn: Rc::new(Mutex::new(c)),
            uri: uri_string,
        }
    }
}

#[no_mangle]
pub extern "C" fn new_store(uri: *const c_char) -> *mut Store {
    let uri = c_char_to_string(uri);
    let store = Store::new(Some(uri));
    Box::into_raw(Box::new(store))
}

#[no_mangle]
pub unsafe extern "C" fn store_destroy(data: *mut Store) {
    let _ = Box::from_raw(data);
}
