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

use logins::{
    LoginManager
};
use categories::{
    CategoryManager
};
use utils::{
    c_char_to_string,
    read_connection
};

#[derive(Debug)]
pub struct Store {
    pub conn: Arc<Mutex<Connection>>,
    pub uri: String,
    pub logins: Arc<LoginManager>,
    pub categories: Arc<CategoryManager>
}

impl Drop for Store {
    fn drop(&mut self) {
        println!("{:?} is being deallocated", self);
    }
}

impl Store {
    pub fn new(uri: String) -> Self {
        let conn = Arc::new(Mutex::new(Connection::open(uri.clone()).unwrap()));
        Store {
            conn: conn.clone(),
            uri: uri.clone(),
            logins: Arc::new(LoginManager::new(uri.clone(), conn.clone())),
            categories: Arc::new(CategoryManager::new(uri.clone(), conn))
        }
    }
}

#[no_mangle]
pub extern "C" fn new_store(uri: *const c_char) -> *mut Store {
    let uri = c_char_to_string(uri);
    let store = Store::new(uri);
    // create tables
    store.logins.create_logins_table();
    store.categories.create_categories_table();
    store.categories.create_items_table();
    Box::into_raw(Box::new(store))
}

#[no_mangle]
pub unsafe extern "C" fn store_destroy(data: *mut Store) {
    let _ = Box::from_raw(data);
}

#[no_mangle]
pub unsafe extern "C" fn store_logins(store: *mut Store) -> *mut Arc<LoginManager> {
    let store = &*store;
    let logins = Box::new(store.logins.clone());
    Box::into_raw(logins)
}

#[no_mangle]
pub unsafe extern "C" fn store_categories(store: *mut Store) -> *mut Arc<CategoryManager> {
    let store = &*store;
    let cats = Box::new(store.categories.clone());
    Box::into_raw(cats)
}

#[no_mangle]
pub unsafe extern "C" fn store_write_connection(store: *mut Store) -> *mut Arc<Mutex<Connection>> {
    let store = &*store;
    let conn = Box::new(store.conn.clone());
    Box::into_raw(conn)
}

#[no_mangle]
pub unsafe extern "C" fn store_read_connection(store: *mut Store) -> *mut Arc<Connection> {
    let store = &*store;
    let conn = Box::new(read_connection(&store.uri));
    Box::into_raw(conn)
}





