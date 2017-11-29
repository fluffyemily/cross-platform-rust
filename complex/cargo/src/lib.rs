// Copyright 2016 Mozilla
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

extern crate ffi_utils;
extern crate store;
extern crate list;

use std::os::raw::{
    c_char
};
use std::sync::{
    Arc,
};

use ffi_utils::strings::c_char_to_string;
use ffi_utils::android::log;
use list::ListManager;
use list::items::Item;
use store::Store;

#[repr(C)]
#[derive(Debug)]
pub struct Toodle {
    store: Arc<Store>,
    list: Arc<ListManager>
}

impl Toodle {
    fn new(uri: String) -> Toodle {
        let store = Arc::new(Store::new(uri));
        Toodle {
            store: store.clone(),
            list: Arc::new(ListManager::new(store.clone()))
        }
    }
}

#[no_mangle]
pub extern "C" fn new_toodle(uri: *const c_char) -> *mut Toodle {
    let uri = c_char_to_string(uri);
    log(&format!("Got db uri: {}", uri)[..]);
    Box::into_raw(Box::new(Toodle::new(uri)))
}

#[no_mangle]
pub unsafe extern "C" fn toodle_destroy(toodle: *mut Toodle) {
    let _ = Box::from_raw(toodle);
    log("Letting Toodle ref go out of scope");
}

#[no_mangle]
pub unsafe extern "C" fn toodle_list(toodle: *mut Toodle) -> *mut Arc<ListManager> {
    let toodle = &*toodle;
    Box::into_raw(Box::new(toodle.list.clone()))
}

#[no_mangle]
pub unsafe extern "C" fn toodle_list_destroy(list_manager: *mut Arc<ListManager>) {
    let _ = Arc::from_raw(list_manager);
    log("Letting ListManager Arc ref go out of scope");
}

#[no_mangle]
pub unsafe extern "C" fn a_toodle_list(toodle: *const Toodle) -> *const ListManager {
    let toodle = &*toodle;
    Arc::into_raw(toodle.list.clone())
}

// TODO these interfaces probably belong in separate platform-specific "interface" crates.
#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use std::str;
    use std::mem;

    use std::ffi::{CStr,CString};

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jlong};

    // #[no_mangle]
    // pub unsafe extern fn Java_com_mozilla_toodle_RustToodle_newToodle(env: JNIEnv, _: JClass, db_path: JString) -> jlong {
    //     let db_path_uri: String = env.get_string(db_path).expect("Couldn't get db path").into();
    //     log(&db_path_uri);
    //     let toodle: Toodle = Toodle::new(db_path_uri);
    //     Box::into_raw(Box::new(toodle)) as jlong
    // }

    // #[no_mangle]
    // pub unsafe extern fn Java_com_mozilla_toodle_RustToodle_toodleDestroy(_: JNIEnv, _: JClass, toodle: *mut Toodle) {
    //     let _ = &*toodle;
    // }

    // #[no_mangle]
    // pub unsafe extern fn Java_com_mozilla_toodle_ListManager_listManager(_: JNIEnv, _: JClass, toodle: *mut Toodle) -> jlong {
    //     let toodle = &*toodle;
    //     Box::into_raw(Box::new(toodle.list.clone())) as jlong
    // }

    // #[no_mangle]
    // pub unsafe extern fn Java_com_mozilla_toodle_Item_itemCreate(env: JNIEnv, _: JClass, item: *mut Item) {
    //     // let toodle = &mut*toodle;
    //     // log(&format!("Got Toodle: {:?}", toodle)[..]);

    //     // debug notes:
    //     // this crashes with "Cause: null pointer dereference", even though the passed-in item pointer was
    //     // just used in itemSetDueDate...
    //     let item = &mut*item;
    //     log(&format!("Got item: {:?}", item)[..]);

    //     // list_manager.create_item(item);
    // }

    // #[no_mangle]
    // #[allow(non_snake_case)]
    // pub extern fn renderGreeting(name: *const c_char) -> *const c_char {
    //     let name = to_string(name);

    //     // Convert the Rust string back to a C string so that we can return it
    //     to_ptr(format!("Hello, {}!", name))
    // }

    // /// Convert a native string to a Rust string
    // fn to_string(pointer: *const c_char) -> String {
    //     let slice = unsafe { CStr::from_ptr(pointer).to_bytes() };
    //     str::from_utf8(slice).unwrap().to_string()
    // }

    // /// Convert a Rust string to a native string
    // fn to_ptr(string: String) -> *const c_char {
    //     let cs = CString::new(string.as_bytes()).unwrap();
    //     let ptr = cs.as_ptr();
    //     // Tell Rust not to clean up the string while we still have a pointer to it.
    //     // Otherwise, we'll get a segfault.
    //     mem::forget(cs);
    //     ptr
    // }
}
