// Copyright 2016 Mozilla
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

extern crate libc;
extern crate rusqlite;
extern crate time;
extern crate uuid;
extern crate store;
extern crate ffi_utils;

use std::os::raw::c_char;
use std::sync::{
    Arc,
};
use uuid::Uuid;

pub mod labels;
pub mod items;

use labels::Label;
use ffi_utils::strings::c_char_to_string;
use items::Item;
use store::Store;


#[derive(Debug)]
#[repr(C)]
pub struct ListManager {
    store: Arc<Store>,
}

impl ListManager {
    pub fn new(store: Arc<Store>) -> ListManager {
        let manager = ListManager {
            store: store,
        };
        manager.create_labels_table();
        manager.create_items_table();
        manager.create_item_labels_table();
        manager
    }

    pub fn create_labels_table(&self) {
        let sql = r#"CREATE TABLE IF NOT EXISTS labels (
                name TEXT NOT NULL PRIMARY KEY,
                color TEXT NOT NULL
            )"#;
        let db = self.store.write_connection();
        db.execute(sql, &[]).unwrap();
    }

    pub fn create_label(&self, name: String, color: String) -> Option<Label> {
        let db = self.store.write_connection();
        let sql = r#"INSERT INTO labels (name, color) VALUES (?1, ?2)"#;
        db.execute(sql, &[&name, &color]).unwrap();
        self.fetch_label(&name)
    }

    pub fn fetch_label(&self, name: &String) -> Option<Label> {
        let sql = r#"SELECT name, color FROM labels WHERE name=?"#;

        let conn = self.store.read_connection();
        let mut stmt = conn.prepare(sql).unwrap();
        let mut label_iter = stmt.query_map(&[name], |row| {
            Label {
                name: row.get(0),
                color: row.get(1)
            }
        }).unwrap();

        if let Some(result) = label_iter.next() {
            result.ok()
        } else {
            println!("No label found for name {:?}", name);
            None
        }
    }

    pub fn fetch_labels(&self) -> Vec<Label> {
        let sql = r#"SELECT name, color
                     FROM labels"#;
        let conn = self.store.read_connection();
        let mut stmt = conn.prepare(sql).unwrap();
        let mut label_iter = stmt.query_map(&[], |row| {
            Label {
                name: row.get(0),
                color: row.get(1)
            }
        }).unwrap();

        let mut label_list: Vec<Label> = Vec::new();
        while let Some(result) = label_iter.next() {
            if let Some(mut label) = result.ok() {
                label_list.push(label);
            }
        }
        label_list
    }

    pub fn fetch_labels_for_item(&self, item_uuid: &String) -> Vec<Label> {
        let sql = r#"SELECT name, color
                     FROM labels JOIN item_labels on item_labels.item_uuid=?"#;
        let conn = self.store.read_connection();
        let mut stmt = conn.prepare(sql).unwrap();
        let mut label_iter = stmt.query_map(&[item_uuid], |row| {
            println!("found row!");
            Label {
                name: row.get(0),
                color: row.get(1)
            }
        }).unwrap();

        let mut label_list: Vec<Label> = Vec::new();
        while let Some(result) = label_iter.next() {
            if let Some(mut label) = result.ok() {
                label_list.push(label);
            }
        }
        label_list
    }

    pub fn create_items_table(&self) {
        let sql = r#"CREATE TABLE IF NOT EXISTS items (
                uuid TEXT NOT NULL PRIMARY KEY,
                name TEXT NOT NULL,
                due_date DATETIME,
                completion_date DATETIME
            )"#;
        let db = self.store.write_connection();
        db.execute(sql, &[]).unwrap();
    }

    pub fn create_item_labels_table(&self) {
        let sql = r#"CREATE TABLE IF NOT EXISTS item_labels (
                item_uuid TEXT NOT NULL,
                label_name TEXT NOT NULL,
                PRIMARY KEY(item_uuid, label_name)
            )"#;
        let db = self.store.write_connection();
        let r = db.execute(sql, &[]);
        if r.is_err() {
            println!("failed to create item_labels table {:?}", r.err());
        }
    }

    pub fn fetch_items_with_label(&self, label: &Label) -> Vec<Item> {
        let sql = r#"SELECT uuid, name, created_at, due_date, completion_date
                     FROM items JOIN item_labels on items.uuid=item_label.item_uuid
                     WHERE item_labels.label_name=?"#;
        let conn = self.store.read_connection();
        let mut stmt = conn.prepare(sql).unwrap();
        let mut item_iter = stmt.query_map(&[&label.name], |row| {
            let uuid: String = row.get(0);
            Item {
                uuid: uuid.clone(),
                name: row.get(1),
                due_date: row.get(2),
                completion_date: row.get(3),
                labels: self.fetch_labels_for_item(&uuid)
            }
        }).unwrap();

        let mut item_list: Vec<Item> = Vec::new();
        while let Some(result) = item_iter.next() {
            if let Some(i) = result.ok() {
                item_list.push(i);
            }
        }
        item_list
    }

    pub fn fetch_item(&self, uuid: &String) -> Option<Item> {
        let sql = r#"SELECT uuid, name, due_date, completion_date FROM items WHERE uuid=?"#;

        let conn = self.store.read_connection();
        let mut stmt = conn.prepare(sql).unwrap();
        let mut item_iter = stmt.query_map(&[uuid], |row| {
            Item {
                uuid: row.get(0),
                name: row.get(1),
                due_date: row.get(2),
                completion_date: row.get(3),
                labels: self.fetch_labels_for_item(uuid)
            }
        }).unwrap();

        if let Some(result) = item_iter.next() {
            result.ok()
        } else {
            println!("No item found for uuid {:?}", uuid);
            None
        }
    }

    pub fn create_item(&self, item: &Item) -> Option<Item> {
        let item_sql = r#"INSERT INTO items (uuid, name, due_date, completion_date) VALUES (?, ?, ?, ?)"#;
        let conn = self.store.write_connection();
        let item_uuid = Uuid::new_v4().simple().to_string();
        conn.execute(item_sql, &[&item_uuid, &item.name, &item.due_date, &item.completion_date]).unwrap();

        let item_label_sql = r#"INSERT INTO item_labels (item_uuid, label_name) VALUES (?, ?)"#;
        for label in item.labels.iter() {
            conn.execute(&item_label_sql, &[&item_uuid, &label.name]).unwrap();
        }
        self.fetch_item(&item_uuid)
    }

    pub fn update_item(&self, item: &Item) {
        let sql = r#"UPDATE items SET name=?, due_date=?, completion_date=? WHERE uuid=?"#;
        let conn = self.store.write_connection();
        let _ = conn.execute(sql, &[&item.name, &item.due_date, &item.completion_date, &item.uuid]);

        let existing_labels = self.fetch_labels_for_item(&(item.uuid));
        let item_label_insert_sql = r#"INSERT INTO item_labels (item_uuid, label_name) VALUES (?, ?)"#;
        let _ = item.labels.iter().map(|label| {
            if !existing_labels.contains(label) {
                // add label to item
                conn.execute(&item_label_insert_sql, &[&item.uuid, &label.name]).unwrap();
            }
        });
        let item_label_delete_sql = r#"DELETE FROM item_labels WHERE item_uuid=? AND label_name=?"#;
        let _ = existing_labels.iter().map(|label| {
            if !item.labels.contains(label) {
                // delete label from item
                conn.execute(&item_label_delete_sql, &[&item.uuid, &label.name]).unwrap();
            }
        });
    }
}


#[no_mangle]
pub unsafe extern "C" fn list_manager_get_all_labels(manager: *const Arc<ListManager>) -> *mut Vec<Label> {
    let manager = &*manager;
    let label_list = Box::new(manager.fetch_labels());
    Box::into_raw(label_list)
}

#[no_mangle]
pub unsafe extern "C" fn list_manager_create_item(manager: *const Arc<ListManager>, item: *const Item) {
    let manager = &*manager;
    let item = &*item;
    manager.create_item(item);
}

#[no_mangle]
pub unsafe extern "C" fn list_manager_update_item(manager: *const Arc<ListManager>, item: *const Item) {
    let manager = &*manager;
    let item = &*item;
    manager.update_item(item)
}

#[no_mangle]
pub unsafe extern "C" fn list_manager_create_label(manager: *const Arc<ListManager>, name: *const c_char, color: *const c_char) -> *mut Label {
    let manager = &*manager;
    let name = c_char_to_string(name);
    let color = c_char_to_string(color);
    let label = Box::new(manager.create_label(name, color).unwrap());
    Box::into_raw(label)
}





#[cfg(test)]
mod test {
    use super::{
        Store,
        ListManager,
        Label,
        Item
    };

    use std::sync::Arc;

    fn list_manager() -> ListManager {
        let store = Arc::new(Store::new(Some("sql_test".to_string())));
        ListManager::new(store)
    }

    fn clean_up(store: &Store) {
        let conn = store.write_connection();
        let tables = [&"items", &"labels", &"item_labels"];
        for &table in tables.iter() {
            let sql = format!("DROP TABLE {}", table);
            let _ = conn.execute(&sql, &[]);
        }
    }

    #[test]
    fn test_new_list_manager() {
        let manager = list_manager();
        let sql = r#"SELECT count(name) FROM sqlite_master WHERE type='table' AND name=?"#;
        let conn = manager.store.write_connection();
        // test that items table has been created
        let mut stmt = conn.prepare(sql).unwrap();
        let tables = [&"items", &"labels", &"item_labels"];
        for &table in tables.iter() {
            let mut r = stmt.query(&[table]).unwrap();
            match r.next() {
                Some(Ok(row)) => {
                    let count: i64 = row.get(0);
                    assert_eq!(count, 1);
                },
                _ => assert!(false),
            }
        }

        clean_up(&manager.store);
    }

    #[test]
    fn test_create_label() {
        let manager = list_manager();
        let l = Label {
            name: "test".to_string(),
            color: "#000000".to_string()
        };
        let label = manager.create_label(l.name.clone(), l.color.clone());
        assert_eq!(label, Some(l));
        clean_up(&manager.store);
    }

    #[test]
    fn test_fetch_label() {
        let manager = list_manager();
        let created_label = manager.create_label("test".to_string(), "#000000".to_string()).unwrap();
        let fetched_label = manager.fetch_label(&created_label.name).unwrap();
        assert_eq!(fetched_label, created_label);

        let fetched_label = manager.fetch_label(&"doesn't exist".to_string());
        assert_eq!(fetched_label, None);

        clean_up(&manager.store);
    }

    #[test]
    fn test_fetch_labels() {
        let manager = list_manager();

        let labels = ["label1".to_string(), "label2".to_string(), "label3".to_string()];
        for label in labels.iter() {
            manager.create_label(label.clone(), "#000000".to_string());
        }
        let fetched_labels = manager.fetch_labels();
        assert_eq!(fetched_labels.len(), labels.len());
        for label in fetched_labels.iter() {
            assert!(labels.contains(&label.name));
        }

        clean_up(&manager.store);
    }

    #[test]
    fn test_create_item() {
        let manager = list_manager();
        let l = Label {
            name: "test".to_string(),
            color: "#000000".to_string()
        };
        let label = manager.create_label(l.name.clone(), l.color.clone()).unwrap();

        let i = Item {
            uuid: "".to_string(),
            name: "test item".to_string(),
            due_date: None,
            completion_date: None,
            labels: vec![label]
        };

        let item = manager.create_item(&i).expect("expected an item");
        assert!(item.uuid.len() > 0);
        assert_eq!(item.name, i.name);
        assert_eq!(item.due_date, i.due_date);
        assert_eq!(item.completion_date, i.completion_date);
        assert_eq!(item.labels, i.labels);
        clean_up(&manager.store);
    }

    #[test]
    fn test_fetch_item() {
    }

    #[test]
    fn test_fetch_labels_for_item() {
    }

    #[test]
    fn test_fetch_items_with_label() {
    }

    #[test]
    fn test_update_item() {
    }
}
