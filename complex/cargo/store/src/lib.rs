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

extern crate mentat;
extern crate edn;
extern crate mentat_query;
extern crate mentat_core;
extern crate mentat_db;
extern crate ordered_float;
extern crate rusqlite;
extern crate time;
extern crate uuid;

extern crate ffi_utils;

use std::fmt;
use std::os::raw::{
    c_char
};
use std::rc::Rc;

use edn::{
    DateTime,
    FromMicros,
    Utc,
    Uuid
};
use mentat::{
    new_connection,
};
use mentat::conn::Conn;
use mentat_core::{
    Entid,
    TypedValue,
};
use mentat_db::types::TxReport;
use mentat::query::{
    QueryInputs,
    QueryResults,
    Variable,
};
use ordered_float::OrderedFloat;
use rusqlite::{
    Connection
};
use time::Timespec;

pub mod errors;

use ffi_utils::strings::c_char_to_string;
use errors as store_errors;

pub trait ToTypedValue {
    fn to_typed_value(&self) -> Result<TypedValue, ()>;
}

impl<'a> ToTypedValue for &'a String {
    fn to_typed_value(&self) -> Result<TypedValue, ()> {
        Ok(TypedValue::String(Rc::new((*self).to_owned())))
    }
}

impl ToTypedValue for String {
    fn to_typed_value(&self) -> Result<TypedValue, ()> {
        Ok(TypedValue::String(Rc::new((*self).to_owned())))
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Entity {
    pub id: Entid
}

impl Entity {
    fn new(id: Entid) -> Entity {
        Entity { id: id}
    }
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl ToTypedValue for Entity {
    fn to_typed_value(&self) -> Result<TypedValue, ()> {
        Ok(TypedValue::Ref(self.id.clone()))
    }
}

impl<'a> ToTypedValue for &'a bool {
    fn to_typed_value(&self) -> Result<TypedValue, ()> {
        Ok(TypedValue::Boolean((*self).to_owned()))
    }
}

impl<'a> ToTypedValue for &'a i64 {
    fn to_typed_value(&self) -> Result<TypedValue, ()> {
        Ok(TypedValue::Long((*self).to_owned()))
    }
}

impl<'a> ToTypedValue for &'a f64 {
    fn to_typed_value(&self) -> Result<TypedValue, ()> {
        Ok(TypedValue::Double(OrderedFloat((*self).to_owned())))
    }
}

impl<'a> ToTypedValue for &'a Timespec {
    fn to_typed_value(&self) -> Result<TypedValue, ()> {
        let micro_seconds = (self.sec * 1000000) + i64::from((self.nsec * 1000));
        Ok(TypedValue::Instant(DateTime::<Utc>::from_micros(micro_seconds)))
    }
}

impl<'a> ToTypedValue for &'a mentat_core::Uuid {
    fn to_typed_value(&self) -> Result<TypedValue, ()> {
        Ok(TypedValue::Uuid((*self).to_owned()))
    }
}

pub trait ToInner<T> {
    fn to_inner(self) -> T;
}

impl ToInner<Option<Entity>> for TypedValue {
    fn to_inner(self) -> Option<Entity> {
        match self {
            TypedValue::Ref(r) => Some(Entity::new(r.clone())),
            _ => None,
        }
    }
}

impl ToInner<Option<i64>> for TypedValue {
    fn to_inner(self) -> Option<i64> {
        match self {
            TypedValue::Long(v) => Some(v),
            _ => None,
        }
    }
}

impl ToInner<String> for TypedValue {
    fn to_inner(self) -> String {
        match self {
            TypedValue::String(s) => s.to_string(),
            _ => String::new(),
        }
    }
}

impl ToInner<Uuid> for TypedValue {
    fn to_inner(self) -> Uuid {
        match self {
            TypedValue::Uuid(u) => u,
            _ => Uuid::nil(),
        }
    }
}

impl ToInner<Option<Timespec>> for TypedValue {
    fn to_inner(self) -> Option<Timespec> {
        match self {
            TypedValue::Instant(v) => {
                let timestamp = v.timestamp();
                Some(Timespec::new(timestamp, 0))
            },
            _ => None,
        }
    }
}

#[repr(C)]
/// Store containing a SQLite connection
pub struct Store {
    handle: Connection,
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
    pub fn new<T>(uri: T) -> Result<Self, store_errors::Error>
    where T: Into<Option<String>> {
        let uri_string = uri.into().unwrap_or(String::new());
        let mut h = try!(new_connection(&uri_string));
        let c = try!(Conn::connect(&mut h));
        Ok(Store {
            handle: h,
            conn:c,
            uri: uri_string,
        })
    }

    pub fn open<T>(&mut self, uri: T) -> Result<(), store_errors::Error>
    where T: Into<Option<String>> {
        let uri_string = uri.into().unwrap_or(String::new());
        self.handle = try!(new_connection(&uri_string));
        self.conn = try!(Conn::connect(&mut self.handle));
        self.uri = uri_string;
        Ok(())
    }

    pub fn query(&self, query: &str) ->  Result<QueryResults, store_errors::Error> {
        Ok(self.conn.q_once(&self.handle, query, None)?)
    }

    pub fn query_args<T>(&self, query: &str, inputs: &[&(&String, &T)]) ->  Result<QueryResults, store_errors::Error>
        where T: ToTypedValue {
        let mut ee = vec![];
        for &&(ref arg, ref val) in inputs.iter() {
            ee.push((Variable::from_valid_name(&arg), val.to_typed_value().ok().unwrap()));
        }
        let i = QueryInputs::with_value_sequence(ee);
        Ok(self.conn.q_once(&self.handle, query, i)?)
    }

    pub fn transact(&mut self, transaction: &str) -> Result<TxReport, store_errors::Error> {
        Ok(self.conn.transact(&mut self.handle, transaction)?)
    }

    pub fn fetch_schema(&self) -> edn::Value {
        self.conn.current_schema().to_edn_value()
    }
}

#[no_mangle]
pub extern "C" fn new_store(uri: *const c_char) -> *mut Store {
    let uri = c_char_to_string(uri);
    let store = Store::new(uri).expect("expected Store");
    Box::into_raw(Box::new(store))
}

#[no_mangle]
pub unsafe extern "C" fn store_destroy(data: *mut Store) {
    let _ = Box::from_raw(data);
}
