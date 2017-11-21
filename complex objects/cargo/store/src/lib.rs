// Copyright 2016 Mozilla
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

extern crate mentat;
extern crate edn;
extern crate mentat_query;
extern crate mentat_core;
extern crate mentat_db;
extern crate rusqlite;
extern crate ffi_utils;

use std::fmt;
use std::os::raw::{
    c_char
};
use std::sync::{
    Arc,
    Mutex
};

use ffi_utils::strings::c_char_to_string;
use mentat::{
    new_connection,
};
use mentat::conn::Conn;
use mentat_db::types::TxReport;
use mentat::query::QueryResults;


#[repr(C)]
/// Store containing a SQLite connection
pub struct Store {
    handle: rusqlite::Connection,
    conn: Conn,
    uri: String,
}

impl Drop for Store {
    fn drop(&mut self) {
        println!("{:?} is being deallocated", self);
    }
}

impl fmt::Debug for Store {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Store at {:?}", self.uri)
    }
}

impl Store {
    pub fn new(uri: String) -> Result<Self, mentat::errors::Error> {
        let mut h = try!(new_connection(&uri));
        let c = try!(Conn::connect(&mut h));
        Ok(Store {
            handle: h,
            conn:c,
            uri: uri,
        })
    }

    pub fn open(&mut self, uri: String) -> Result<(), mentat::errors::Error> {
        self.handle = try!(new_connection(&uri));
        self.conn = try!(Conn::connect(&mut self.handle));
        self.uri = uri;
        Ok(())
    }

    pub fn query(&self, query: String) -> Result<QueryResults, mentat::errors::Error> {
        Ok(self.conn.q_once(&self.handle, &query, None)?)
    }

    pub fn transact(&mut self, transaction: String) -> Result<TxReport, mentat::errors::Error> {
        Ok(self.conn.transact(&mut self.handle, &transaction)?)
    }

    pub fn fetch_schema(&self) -> edn::Value {
        self.conn.current_schema().to_edn_value()
    }
}

#[no_mangle]
pub extern "C" fn new_store(uri: *const c_char) -> *mut Store {
    let uri = c_char_to_string(uri);
    let store = Store::new(uri).expect("Expected Store");
    Box::into_raw(Box::new(store))
}

#[no_mangle]
pub unsafe extern "C" fn store_destroy(data: *mut Store) {
    let _ = Box::from_raw(data);
}
