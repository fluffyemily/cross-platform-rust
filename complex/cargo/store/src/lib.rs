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
use std::sync::{
    Arc,
    Mutex
};

use rusqlite::{
    Connection
};

use ffi_utils::strings::c_char_to_string;

#[derive(Debug)]
#[repr(C)]
/// Store containing a SQLite connection
pub struct Store {
    conn: Mutex<Arc<Connection>>,
    uri: String,
}

impl Drop for Store {
    fn drop(&mut self) {
        println!("{:?} is being deallocated", self);
    }
}

impl Store {
    pub fn new(uri: String) -> Self {
        Store {
            conn: Mutex::new(Arc::new(Connection::open(uri.clone()).unwrap())),
            uri: uri.clone(),
        }
    }

    pub fn write_connection(&self) -> Arc<Connection> {
        self.conn.lock().unwrap().clone()
    }

    pub fn read_connection(&self) -> Arc<Connection> {
        Arc::new(Connection::open_with_flags(self.uri.clone(), rusqlite::SQLITE_OPEN_READ_ONLY).unwrap())
    }
}

#[no_mangle]
pub extern "C" fn new_store(uri: *const c_char) -> *mut Arc<Store> {
    let uri = c_char_to_string(uri);
    let store = Arc::new(Store::new(uri));
    Box::into_raw(Box::new(store))
}

#[no_mangle]
pub unsafe extern "C" fn store_destroy(data: *mut Store) {
    let _ = Box::from_raw(data);
}
