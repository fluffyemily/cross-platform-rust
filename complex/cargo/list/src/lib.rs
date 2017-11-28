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
        let mut manager = ListManager {
            store: store,
        };
        manager.create_labels_table();
        manager.create_items_table();
        manager.create_item_labels_table();
        manager
    }

    fn get_store(&self) -> Arc<Store> {
        Arc::clone(&self.store)
    }

    fn get_store_mut(&mut self) -> &mut Store {
        Arc::get_mut(&mut self.store).unwrap()
    }

    pub fn create_labels_table(&mut self) {
        let sql = r#"CREATE TABLE IF NOT EXISTS labels (
                name TEXT NOT NULL PRIMARY KEY,
                color TEXT NOT NULL
            )"#;
        let s = self.get_store_mut();
        let db = s.get_conn_mut();
        db.execute(sql, &[]).unwrap();
    }

    pub fn create_label(&self, name: String, color: String) -> Option<Label> {
        let db = self.get_store().get_conn();
        let sql = r#"INSERT INTO labels (name, color) VALUES (?1, ?2)"#;
        db.execute(sql, &[&name, &color]).unwrap();
        self.fetch_label(&name)
    }

    pub fn fetch_label(&self, name: &String) -> Option<Label> {
        let sql = r#"SELECT name, color FROM labels WHERE name=?"#;

        let conn = self.get_store().get_conn();
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
        let conn = self.get_store().get_conn();
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

    pub fn fetch_labels_for_item(&mut self, item_uuid: &String) -> Vec<Label> {
        let sql = r#"SELECT name, color
                     FROM labels JOIN item_labels on item_labels.label_name=labels.name
                     WHERE item_labels.item_uuid=?"#;
        let conn = self.get_store().get_conn();
        let mut stmt = conn.prepare(sql).unwrap();
        let mut label_iter = stmt.query_map(&[item_uuid], |row| {
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

    pub fn create_items_table(&mut self) {
        let sql = r#"CREATE TABLE IF NOT EXISTS items (
                uuid TEXT NOT NULL PRIMARY KEY,
                name TEXT NOT NULL,
                due_date DATETIME,
                completion_date DATETIME
            )"#;
        let db = self.get_store_mut().get_conn_mut();
        db.execute(sql, &[]).unwrap();
    }

    pub fn create_item_labels_table(&mut self) {
        let sql = r#"CREATE TABLE IF NOT EXISTS item_labels (
                item_uuid TEXT NOT NULL,
                label_name TEXT NOT NULL,
                PRIMARY KEY(item_uuid, label_name)
            )"#;
        let db = self.get_store_mut().get_conn_mut();
        let r = db.execute(sql, &[]);
        if r.is_err() {
            println!("failed to create item_labels table {:?}", r.err());
        }
    }

    pub fn fetch_items_with_label(&mut self, label: &Label) -> Vec<Item> {
        let sql = r#"SELECT uuid, name, due_date, completion_date
                     FROM items JOIN item_labels on items.uuid=item_labels.item_uuid
                     WHERE item_labels.label_name=?"#;
        let conn = self.get_store().get_conn();
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

    pub fn fetch_item(&mut self, uuid: &String) -> Option<Item> {
        let sql = r#"SELECT uuid, name, due_date, completion_date FROM items WHERE uuid=?"#;

        let conn = self.get_store().get_conn();
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

    pub fn create_item(&mut self, item: &Item) -> String {
        let item_sql = r#"INSERT INTO items (uuid, name, due_date, completion_date) VALUES (?, ?, ?, ?)"#;
        let conn = self.get_store_mut().get_conn_mut();
        let tx = conn.transaction().expect("expected a transaction");
        let item_uuid = Uuid::new_v4().simple().to_string();
        let _ = tx.execute(item_sql, &[&item_uuid, &item.name, &item.due_date, &item.completion_date]);

        let item_label_sql = r#"INSERT INTO item_labels (item_uuid, label_name) VALUES (?, ?)"#;
        for label in item.labels.iter() {
            tx.execute(&item_label_sql, &[&item_uuid, &label.name]).unwrap();
        }
        let _ = tx.commit();
        item_uuid
    }

    pub fn update_item(&mut self, item: &Item, existing_labels: Vec<Label>) {
        let sql = r#"UPDATE items SET name=?, due_date=?, completion_date=? WHERE uuid=?"#;
        let conn = self.get_store_mut().get_conn_mut();
        let tx = conn.transaction().expect("expected a transaction");
        let _ = tx.execute(sql, &[&item.name, &item.due_date, &item.completion_date, &item.uuid]);

        let item_label_insert_sql = r#"INSERT INTO item_labels (item_uuid, label_name) VALUES (?, ?)"#;
        for label in item.labels.iter() {
            if !existing_labels.contains(label) {
                // add label to item
                tx.execute(&item_label_insert_sql, &[&item.uuid, &label.name]).unwrap();
            }
        }
        let item_label_delete_sql = r#"DELETE FROM item_labels WHERE item_uuid=? AND label_name=?"#;
        for label in existing_labels.iter() {
            if !item.labels.contains(label) {
                // delete label from item
                tx.execute(&item_label_delete_sql, &[&item.uuid, &label.name]).unwrap();
            }
        }
        let _ = tx.commit();
    }
}

fn create_and_fetch_item(manager: &mut ListManager, item: &Item) -> Option<Item> {
    let item_uuid = manager.create_item(item);
    manager.fetch_item(&item_uuid)
}


#[no_mangle]
pub unsafe extern "C" fn list_manager_get_all_labels(manager: *mut Arc<ListManager>) -> *mut Vec<Label> {
    let manager = Arc::get_mut(&mut *manager).unwrap();
    let label_list = Box::new(manager.fetch_labels());
    Box::into_raw(label_list)
}

#[no_mangle]
pub unsafe extern "C" fn list_manager_create_item(manager: *mut Arc<ListManager>, item: *const Item) {
    let manager = Arc::get_mut(&mut *manager).unwrap();
    let item = &*item;
    create_and_fetch_item(manager, &item);
}

#[no_mangle]
pub unsafe extern "C" fn list_manager_update_item(manager: *mut Arc<ListManager>, item: *const Item) {
    let manager = Arc::get_mut(&mut *manager).unwrap();
    let item = &*item;
    let existing_labels = manager.fetch_labels_for_item(&(item.uuid));
    manager.update_item(item, existing_labels)
}

#[no_mangle]
pub unsafe extern "C" fn list_manager_create_label(manager: *mut Arc<ListManager>, name: *const c_char, color: *const c_char) -> *mut Label {
    let manager = Arc::get_mut(&mut *manager).unwrap();
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
        Item,
        create_and_fetch_item,
    };

    use std::sync::Arc;

    use time::now_utc;

    fn list_manager() -> ListManager {
        let store = Arc::new(Store::new(None));
        ListManager::new(store)
    }

    #[test]
    fn test_new_list_manager() {
        let manager = list_manager();
        let sql = r#"SELECT count(name) FROM sqlite_master WHERE type='table' AND name=?"#;
        let conn = manager.get_store().get_conn();
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
    }

    #[test]
    fn test_fetch_label() {
        let manager = list_manager();
        let created_label = manager.create_label("test".to_string(), "#000000".to_string()).unwrap();
        let fetched_label = manager.fetch_label(&created_label.name).unwrap();
        assert_eq!(fetched_label, created_label);

        let fetched_label = manager.fetch_label(&"doesn't exist".to_string());
        assert_eq!(fetched_label, None);
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
    }

    #[test]
    fn test_create_item() {
        let mut manager = list_manager();
        let l = Label {
            name: "label1".to_string(),
            color: "#000000".to_string()
        };
        let label = manager.create_label(l.name.clone(), l.color.clone()).unwrap();

        let l2 = Label {
            name: "label2".to_string(),
            color: "#000000".to_string()
        };
        let label2 = manager.create_label(l2.name.clone(), l2.color.clone()).unwrap();

        let date = now_utc().to_timespec();
        let i = Item {
            uuid: "".to_string(),
            name: "test item".to_string(),
            due_date: Some(date.clone()),
            completion_date: Some(date.clone()),
            labels: vec![label, label2]
        };

        let item = create_and_fetch_item(&mut manager, &i).expect("expected an item");
        assert!(item.uuid.len() > 0);
        assert_eq!(item.name, i.name);
        let due_date = item.due_date.expect("expecting a due date");
        assert_eq!(due_date.sec, date.sec);
        let completion_date = item.completion_date.expect("expecting a completion date");
        assert_eq!(completion_date.sec, date.sec);
        assert_eq!(item.labels, i.labels);
    }

    #[test]
    fn test_create_item_no_due_date() {
        let mut manager = list_manager();
        let l = Label {
            name: "label1".to_string(),
            color: "#000000".to_string()
        };
        let label = manager.create_label(l.name.clone(), l.color.clone()).unwrap();

        let l2 = Label {
            name: "label2".to_string(),
            color: "#000000".to_string()
        };
        let label2 = manager.create_label(l2.name.clone(), l2.color.clone()).unwrap();

        let date = now_utc().to_timespec();
        let i = Item {
            uuid: "".to_string(),
            name: "test item".to_string(),
            due_date: None,
            completion_date: Some(date.clone()),
            labels: vec![label, label2]
        };

        let item = create_and_fetch_item(&mut manager, &i).expect("expected an item");
        assert!(item.uuid.len() > 0);
        assert_eq!(item.name, i.name);
        assert_eq!(item.due_date, i.due_date);
        let completion_date = item.completion_date.expect("expecting a completion date");
        assert_eq!(completion_date.sec, date.sec);
        assert_eq!(item.labels, i.labels);
    }

    #[test]
    fn test_create_item_no_completion_date() {
        let mut manager = list_manager();
        let l = Label {
            name: "label1".to_string(),
            color: "#000000".to_string()
        };
        let label = manager.create_label(l.name.clone(), l.color.clone()).unwrap();

        let l2 = Label {
            name: "label2".to_string(),
            color: "#000000".to_string()
        };
        let label2 = manager.create_label(l2.name.clone(), l2.color.clone()).unwrap();

        let date = now_utc().to_timespec();
        let i = Item {
            uuid: "".to_string(),
            name: "test item".to_string(),
            due_date: Some(date.clone()),
            completion_date: None,
            labels: vec![label, label2]
        };

        let item = create_and_fetch_item(&mut manager, &i).expect("expected an item");
        assert!(item.uuid.len() > 0);
        assert_eq!(item.name, i.name);
        let due_date = item.due_date.expect("expecting a due date");
        assert_eq!(due_date.sec, date.sec);
        assert_eq!(item.completion_date, i.completion_date);
        assert_eq!(item.labels, i.labels);
    }

    #[test]
    fn test_fetch_item() {
        let mut manager = list_manager();
        let label = manager.create_label("label1".to_string(), "#000000".to_string()).unwrap();
        let mut created_item = Item {
            uuid: "".to_string(),
            name: "test item".to_string(),
            due_date: None,
            completion_date: None,
            labels: vec![label]
        };

        created_item.uuid = manager.create_item(&created_item);
        let fetched_item = manager.fetch_item(&created_item.uuid).expect("expected an item");
        assert_eq!(fetched_item, created_item);

        let fetched_item = manager.fetch_item(&"doesn't exist".to_string());
        assert_eq!(fetched_item, None);
    }

    #[test]
    fn test_fetch_labels_for_item() {
        let mut manager = list_manager();
        let label = manager.create_label("label1".to_string(), "#000000".to_string()).unwrap();
        let label2 = manager.create_label("label2".to_string(), "#000000".to_string()).unwrap();
        let label3 = manager.create_label("label3".to_string(), "#000000".to_string()).unwrap();

        let mut item1 = Item {
            uuid: "".to_string(),
            name: "test item 1".to_string(),
            due_date: None,
            completion_date: None,
            labels: vec![label, label2, label3]
        };

        item1.uuid = manager.create_item(&item1);

        let fetched_labels = manager.fetch_labels_for_item(&item1.uuid);
        assert_eq!(fetched_labels, item1.labels);
    }

    #[test]
    fn test_fetch_items_with_label() {
        let mut manager = list_manager();
        let label = manager.create_label("label1".to_string(), "#000000".to_string()).unwrap();
        let label2 = manager.create_label("label2".to_string(), "#000000".to_string()).unwrap();

        let mut item1 = Item {
            uuid: "".to_string(),
            name: "test item 1".to_string(),
            due_date: None,
            completion_date: None,
            labels: vec![label.clone()]
        };
        let mut item2 = Item {
            uuid: "".to_string(),
            name: "test item 2".to_string(),
            due_date: None,
            completion_date: None,
            labels: vec![label.clone()]
        };
        let mut item3 = Item {
            uuid: "".to_string(),
            name: "test item 3".to_string(),
            due_date: None,
            completion_date: None,
            labels: vec![label.clone(), label2.clone()]
        };

        let mut item4 = Item {
            uuid: "".to_string(),
            name: "test item 4".to_string(),
            due_date: None,
            completion_date: None,
            labels: vec![label2.clone()]
        };

        item1.uuid = manager.create_item(&item1);
        item2.uuid = manager.create_item(&item2);
        item3.uuid = manager.create_item(&item3);
        item4.uuid = manager.create_item(&item4);

        let fetched_label1_items = manager.fetch_items_with_label(&label);
        assert_eq!(fetched_label1_items, vec![item1, item2, item3.clone()]);
        let fetched_label2_items = manager.fetch_items_with_label(&label2);
        assert_eq!(fetched_label2_items, vec![item3, item4]);
    }

    #[test]
    fn test_update_item_add_label() {
        let mut manager = list_manager();
        let label = manager.create_label("label1".to_string(), "#000000".to_string()).unwrap();
        let label2 = manager.create_label("label2".to_string(), "#000000".to_string()).unwrap();
        let label3 = manager.create_label("label3".to_string(), "#000000".to_string()).unwrap();

        let mut item1 = Item {
            uuid: "".to_string(),
            name: "test item 1".to_string(),
            due_date: None,
            completion_date: None,
            labels: vec![label, label2]
        };

        item1.uuid = manager.create_item(&item1);
        item1.labels.push(label3);

        let existing_labels = manager.fetch_labels_for_item(&item1.uuid);
        manager.update_item(&item1, existing_labels);

        let fetched_item = manager.fetch_item(&item1.uuid).expect("expected an item");
        assert_eq!(fetched_item, item1);
    }

    #[test]
    fn test_update_item_remove_label() {
        let mut manager = list_manager();
        let label = manager.create_label("label1".to_string(), "#000000".to_string()).unwrap();
        let label2 = manager.create_label("label2".to_string(), "#000000".to_string()).unwrap();
        let label3 = manager.create_label("label3".to_string(), "#000000".to_string()).unwrap();

        let mut item1 = Item {
            uuid: "".to_string(),
            name: "test item 1".to_string(),
            due_date: None,
            completion_date: None,
            labels: vec![label, label2, label3]
        };

        item1.uuid = manager.create_item(&item1);
        item1.labels.remove(2);

        let existing_labels = manager.fetch_labels_for_item(&item1.uuid);
        manager.update_item(&item1, existing_labels);

        let fetched_item = manager.fetch_item(&item1.uuid).expect("expected an item");
        assert_eq!(fetched_item, item1);
    }

    #[test]
    fn test_update_item_add_due_date() {
        let mut manager = list_manager();
        let label = manager.create_label("label1".to_string(), "#000000".to_string()).unwrap();
        let label2 = manager.create_label("label2".to_string(), "#000000".to_string()).unwrap();
        let label3 = manager.create_label("label3".to_string(), "#000000".to_string()).unwrap();

        let mut item1 = Item {
            uuid: "".to_string(),
            name: "test item 1".to_string(),
            due_date: None,
            completion_date: None,
            labels: vec![label, label2, label3]
        };

        item1.uuid = manager.create_item(&item1);
        item1.due_date = Some(now_utc().to_timespec());

        let existing_labels = manager.fetch_labels_for_item(&item1.uuid);
        manager.update_item(&item1, existing_labels);

        let fetched_item = manager.fetch_item(&item1.uuid).expect("expected an item");
        let due_date = fetched_item.due_date.expect("expected a due date");
        assert_eq!(due_date.sec, item1.due_date.unwrap().sec);
    }

    #[test]
    fn test_update_item_change_name() {
        let mut manager = list_manager();
        let label = manager.create_label("label1".to_string(), "#000000".to_string()).unwrap();
        let label2 = manager.create_label("label2".to_string(), "#000000".to_string()).unwrap();
        let label3 = manager.create_label("label3".to_string(), "#000000".to_string()).unwrap();

        let date = now_utc().to_timespec();
        let mut item1 = Item {
            uuid: "".to_string(),
            name: "test item 1".to_string(),
            due_date: Some(date),
            completion_date: None,
            labels: vec![label, label2, label3]
        };

        item1.uuid = manager.create_item(&item1);
        item1.name = "new name".to_string();

        let existing_labels = manager.fetch_labels_for_item(&item1.uuid);
        manager.update_item(&item1, existing_labels);

        let fetched_item = manager.fetch_item(&item1.uuid).expect("expected an item");
        assert_eq!(fetched_item.name, item1.name);
    }

    #[test]
    fn test_update_item_complete_item() {
        let mut manager = list_manager();
        let label = manager.create_label("label1".to_string(), "#000000".to_string()).unwrap();
        let label2 = manager.create_label("label2".to_string(), "#000000".to_string()).unwrap();
        let label3 = manager.create_label("label3".to_string(), "#000000".to_string()).unwrap();

        let date = now_utc().to_timespec();
        let mut item1 = Item {
            uuid: "".to_string(),
            name: "test item 1".to_string(),
            due_date: None,
            completion_date: None,
            labels: vec![label, label2, label3]
        };

        item1.uuid = manager.create_item(&item1);
        item1.completion_date = Some(date);

        let existing_labels = manager.fetch_labels_for_item(&item1.uuid);
        manager.update_item(&item1, existing_labels);

        let fetched_item = manager.fetch_item(&item1.uuid).expect("expected an item");
        let completion_date = fetched_item.completion_date.expect("expected a completion_date");
        assert_eq!(completion_date.sec, date.sec);
    }
}
