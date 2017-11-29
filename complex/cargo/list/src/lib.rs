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
extern crate edn;
extern crate mentat;
extern crate mentat_core;
extern crate rusqlite;
extern crate store;
extern crate time;
extern crate uuid;

extern crate ffi_utils;

use libc::size_t;
use std::os::raw::c_char;
use std::sync::{
    Arc,
};

use mentat::query::QueryResults;
use mentat_core::{
    TypedValue,
    Uuid,
};
use time::{
    at,
    Timespec,
    Tm,
    TmFmt,
};
use uuid::UuidVersion;

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
        manager
    }

    fn write_connection(&mut self) -> &mut Store {
        Arc::get_mut(&mut self.store).unwrap()
    }

    pub fn create_labels_table(&mut self) {
        let schema = r#"[{  :db/ident     :label/name
    :db/valueType :db.type/string
    :db/cardinality :db.cardinality/one
    :db/unique :db.unique/value
    :db/fulltext true },
 {  :db/ident     :label/color
    :db/valueType :db.type/string
    :db/cardinality :db.cardinality/one },
 {  :db/ident     :label/items
    :db/valueType :db.type/ref
    :db/cardinality :db.cardinality/many }]"#;
        let _ = self.write_connection().transact(schema);
    }

    pub fn create_label(&mut self, name: String, color: String) -> Option<Label> {
        let query = format!(r#"[{{
            :label/name "{0}"
            :label/color "{1}"
            }}]"#, &name, &color);
        println!("{:?}", query);
        let res = self.write_connection().transact(&query);
        println!("res: {:?}", res);
        self.fetch_label(&name)
    }

    pub fn fetch_label(&self, name: &String) -> Option<Label> {
        let query = r#"[:find ?name, ?color
            :in ?name
            :where
            [?eid :label/name ?name]
            [?eid :label/color ?color]
        ]"#;
        let result = Arc::clone(&self.store).query_args(query, &[&(&"?name".to_string(), &name)]);
        match result {
            Ok(rel) => {
                if let QueryResults::Rel(rows) = rel {
                    if !rows.is_empty() {
                        Label::from_row(&rows[0])
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            Err(e) => {
                println!("Failed to fetch {}, {}", name, e);
                None
            },
        }
    }

    pub fn fetch_labels(&self) -> Vec<Label> {
        let query = r#"[:find ?name, ?color
            :where
            [?eid :label/name ?name]
            [?eid :label/color ?color]
        ]"#;
        println!("{:?}", query);
        let result = Arc::clone(&self.store).query(query);
        match result {
            Ok(rel) => {
                if let QueryResults::Rel(rows) = rel {
                    rows.iter().map(|row| Label::from_row(&row).unwrap()).collect()
                } else {
                    vec![]
                }
            },
            Err(e) => {
                println!("Failed to fetch labels");
                vec![]
            },
        }
    }

    pub fn fetch_labels_for_item(&self, item_uuid: &Uuid) -> Vec<Label> {
        println!("fetch_labels_for_item");
        let query = r#"[:find ?name, ?color
            :in ?item_uuid
            :where
            [?l :label/name ?name]
            [?l :label/color ?color]
            [?l :label/items ?i]
            [?i :item/uuid ?item_uuid]
        ]"#;
        println!("query {:?}, {:?}", query, item_uuid);
        let result = Arc::clone(&self.store).query_args(query, &[&(&"?item_uuid".to_string(), &item_uuid)]);
        match result {
            Ok(rel) => {
                if let QueryResults::Rel(rows) = rel {
                    println!("rows {:?}", rows);
                    rows.iter().map(|row| Label::from_row(&row).unwrap()).collect()
                } else {
                    println!("no labels for item {:?}", item_uuid);
                    vec![]
                }
            },
            Err(e) => {
                println!("Failed to fetch labels for {:?}: {}", item_uuid, e);
                vec![]
            },
        }
    }

    pub fn create_items_table(&mut self) {
        let schema = r#"[{  :db/ident     :item/uuid
            :db/valueType :db.type/uuid
            :db/cardinality :db.cardinality/one
            :db/unique :db.unique/identity
            :db/index true },
        {  :db/ident     :item/name
            :db/valueType :db.type/string
            :db/cardinality :db.cardinality/one
            :db/fulltext true  },
        {  :db/ident     :item/due_date
            :db/valueType :db.type/instant
            :db/cardinality :db.cardinality/one  },
        {  :db/ident     :item/completion_date
            :db/valueType :db.type/instant
            :db/cardinality :db.cardinality/one  },
        {  :db/ident     :item/labels
            :db/valueType :db.type/ref
            :db/cardinality :db.cardinality/many }]"#;
        let _ = self.write_connection().transact(schema);
    }

    pub fn fetch_items_with_label(&self, label: &Label) -> Vec<Item> {
        // let sql = r#"SELECT uuid, name, created_at, due_date, completion_date
        //              FROM items LEFT JOIN item_labels on items.uuid=item_label.item_uuid
        //              WHERE item_labels.label_name=?"#;
        // let conn = self.store.read_connection();
        // let mut stmt = conn.prepare(sql).unwrap();
        // let mut item_iter = stmt.query_map(&[&label.name], |row| {
        //     let uuid: String = row.get(0);
        //     Item {
        //         uuid: uuid.clone(),
        //         name: row.get(1),
        //         due_date: row.get(2),
        //         completion_date: row.get(3),
        //         labels: self.fetch_labels_for_item(&uuid)
        //     }
        // }).unwrap();

        let mut item_list: Vec<Item> = Vec::new();
        // while let Some(result) = item_iter.next() {
        //     if let Some(i) = result.ok() {
        //         item_list.push(i);
        //     }
        // }
        item_list
    }

    pub fn fetch_item(&self, uuid: &Uuid) -> Option<Item> {
        println!("fetching item {}", uuid);
        let query = r#"[:find ?eid, ?uuid, ?name, ?due_date, ?completion_date
            :in ?uuid
            :where
            [?eid :item/uuid ?uuid]
            [?eid :item/name ?name]
            [?eid :item/due_date ?due_date]
            [?eid :item/completion_date ?completion_date]
        ]"#;
        println!("{:?} {:?}", query, uuid);
        let result = Arc::clone(&self.store).query_args(query, &[&(&"?uuid".to_string(), &uuid)]);
        match result {
            Ok(rel) => {
                if let QueryResults::Rel(rows) = rel {
                    println!("rows {:?}", rows);
                    if !rows.is_empty() {
                        let mut item = Item::from_row(&rows[0]).unwrap();
                        println!("fetching labels for item: {:?}", item);
                        item.labels = self.fetch_labels_for_item(&uuid);
                        println!("returning item: {:?}", item);
                        Some(item)
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            Err(e) => {
                println!("Failed to fetch {}, {}", uuid, e);
                None
            },
        }
    }

    pub fn create_item(&mut self, item: &Item) -> Uuid {
        let label_str = item.labels.iter().map(|label| label.name.to_owned()).collect::<Vec<String>>().join("\", \"");
        let tmp_uuid = uuid::Uuid::new_v4().hyphenated().to_string();
        let item_uuid = Uuid::parse_str(&tmp_uuid).unwrap();
        // println!("item uuid: {:?}", item_uuid);
        let mut query = format!(r#"[{{
            :item/uuid #uuid "{0}"
            :item/name "{1}"
            "#, &item_uuid.hyphenated().to_string(), &(item.name));
        if let Some(due_date) = item.due_date {
            let micro_seconds = due_date.sec * 1000000;
            query = format!(r#"{}:item/due_date #instmicros {}
                "#, &query, &micro_seconds);
        }
        if let Some(completion_date) = item.completion_date {
            let micro_seconds = completion_date.sec * 1000000;
            query = format!(r#"{}:item/completion_date #instmicros {}
                "#, &query, &micro_seconds);
        }
        if !label_str.is_empty() {
            query = format!(r#"{0}:item/labels ["{1}"]
                "#, &query, &label_str);
        }
        query = format!("{0}}}]", &query);
        println!("query: {}", query);
        let res = self.write_connection().transact(&query);
        // println!("res: {:?}", res);
        item_uuid
    }

    pub fn create_and_fetch_item(&mut self, item: &Item) -> Option<Item> {
        println!("Creating item {:?}", item);
        let item_uuid = self.create_item(&item);
        println!("Fetching item {:?}", item_uuid);
        self.fetch_item(&item_uuid)
    }

    pub fn update_item(&mut self, item: &Item, name: Option<String>, due_date: Option<Timespec>, completion_date: Option<Timespec>, labels: Option<&Vec<Label>>) {
        let item_id = item.id.expect("item must have ID to be updated");
        let mut transaction = vec![];

        if let Some(name) = name {
            if item.name != name {
                transaction.push(format!("[:db/add {0} :item/name {1}]", &item_id, name));
            }
        }
        if item.due_date != due_date {
            if let Some(date) = due_date {
                transaction.push(format!("[:db/add {:?} :item/due_date {:?}]", &item_id, date));
            } else {
                transaction.push(format!("[:db/retract {:?} :item/due_date {:?}]", &item_id, item.due_date.unwrap()));
            }
        }

        if item.completion_date != completion_date {
            if let Some(date) = due_date {
                transaction.push(format!("[:db/add {:?} :item/completion_date {:?}]", &item_id, date));
            } else {
                transaction.push(format!("[:db/retract {:?} :item/completion_date {:?}]", &item_id, item.completion_date.unwrap()));
            }
        }

        if let Some(new_labels) = labels {
            let existing_labels = self.fetch_labels_for_item(&(item.uuid));

            let labels_to_add = new_labels.iter().filter(|label| !existing_labels.contains(label) ).map(|label| label.name.to_owned()).collect::<Vec<String>>().join("\", \"");
            if !labels_to_add.is_empty() {
                transaction.push(format!("[:db/add {0} :item/labels [\"{1}\"]]", &item_id, labels_to_add));
            }
            let labels_to_remove = existing_labels.iter().filter(|label| !new_labels.contains(label) ).map(|label| label.name.to_owned()).collect::<Vec<String>>().join("\", \"");
            if !labels_to_remove.is_empty() {
                transaction.push(format!("[:db/retract {0} :item/labels [\"{1}\"]]", &item_id, labels_to_remove));
            }
        }
        let query = format!("[{0}]", transaction.join(""));
        let _ = self.write_connection().transact(&query);
    }
}

#[no_mangle]
pub unsafe extern "C" fn list_manager_get_all_labels(manager: *const ListManager) -> *mut Vec<Label> {
    let manager = &*manager;
    let label_list = Box::new(manager.fetch_labels());
    Box::into_raw(label_list)
}

#[no_mangle]
pub unsafe extern "C" fn list_manager_create_item(manager: *mut ListManager, item: *const Item) {
    let manager = &mut*manager;
    let item = &*item;
    manager.create_item(&item);
}

#[no_mangle]
pub unsafe extern "C" fn list_manager_update_item(manager: *mut ListManager, item: *const Item, name: *const c_char, due_date: *const size_t, completion_date: *const size_t, labels: *const Vec<Label>) {
    let manager = &mut*manager;
    let item = &*item;
    let labels = &*labels;
    let name = Some(c_char_to_string(name));
    let mut due: Option<Timespec>;
    if !due_date.is_null() {
        due = Some(Timespec::new(due_date as i64, 0));
    } else {
        due = None;
    }
    let mut completion: Option<Timespec>;
    if !completion_date.is_null() {
        completion = Some(Timespec::new(completion_date as i64, 0));
    } else {
        completion = None;
    }
    manager.update_item(item, name, due, completion, Some(labels));
}

#[no_mangle]
pub unsafe extern "C" fn list_manager_create_label(manager: *mut ListManager, name: *const c_char, color: *const c_char) -> *mut Label {
    let manager = &mut*manager;
    let name = c_char_to_string(name);
    let color = c_char_to_string(color);
    let label = Box::new(manager.create_label(name, color).unwrap());
    Box::into_raw(label)
}


#[cfg(test)]
mod test {
    extern crate edn;

    use super::{
        Store,
        ListManager,
        Label,
        Item,
    };

    use std::sync::Arc;

    use mentat_core::Uuid;
    use time::now_utc;


    fn list_manager() -> ListManager {
        let store = Arc::new(Store::new(None).expect("Expected a store"));
        ListManager::new(store)
    }

    fn assert_ident_present(edn: edn::Value, namespace: &str, name: &str) -> bool {
        match edn {
            edn::Value::Vector(v) => {
                let mut found = false;
                for val in v.iter() {
                    found = assert_ident_present(val.clone(), namespace, name);
                    if found {
                        break;
                    }
                }
                found
            },
            edn::Value::Map(m) => {
                let mut found = false;
                for (key, val) in &m {
                    if let edn::Value::NamespacedKeyword(ref kw) = *key {
                        if kw.namespace == "db" && kw.name == "ident" {
                            found = assert_ident_present(val.clone(), namespace, name);
                            if found { break; }
                        } else {
                            continue
                        }
                    }
                }
                found
            },
            edn::Value::NamespacedKeyword(kw) => kw.namespace == namespace && kw.name == name,
            _ => false
        }
    }

    #[test]
    fn test_new_list_manager() {
        let manager = list_manager();
        let schema = Arc::clone(&manager.store).fetch_schema();
        assert_ident_present(schema.clone(), "label", "name");
        assert_ident_present(schema, "list", "name");
    }

    #[test]
    fn test_create_label() {
        let mut manager = list_manager();
        let l = Label {
            name: "test".to_string(),
            color: "#000000".to_string()
        };
        let label = manager.create_label(l.name.clone(), l.color.clone());
        assert_eq!(label, Some(l));
    }

    #[test]
    fn test_fetch_label() {
        let mut manager = list_manager();
        let created_label = manager.create_label("test".to_string(), "#000000".to_string()).unwrap();
        let fetched_label = manager.fetch_label(&created_label.name).unwrap();
        assert_eq!(fetched_label, created_label);

        let fetched_label = manager.fetch_label(&"doesn't exist".to_string());
        assert_eq!(fetched_label, None);
    }

    #[test]
    fn test_fetch_labels() {
        let mut manager = list_manager();

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
            id: None,
            uuid: Uuid::nil(),
            name: "test item".to_string(),
            due_date: Some(date.clone()),
            completion_date: Some(date.clone()),
            labels: vec![label, label2]
        };

        println!("creating and fetching item {:?}", i);

        let item = manager.create_and_fetch_item(&i).expect("expected an item");
        assert!(!item.uuid.is_nil());
        assert_eq!(item.name, i.name);
        let due_date = item.due_date.expect("expecting a due date");
        assert_eq!(due_date.sec, date.sec);
        let completion_date = item.completion_date.expect("expecting a completion date");
        assert_eq!(completion_date.sec, date.sec);
        assert_eq!(item.labels, i.labels);
    }

    // #[test]
    // fn test_create_item_no_due_date() {
    //     let mut manager = list_manager();
    //     let l = Label {
    //         name: "label1".to_string(),
    //         color: "#000000".to_string()
    //     };
    //     let label = manager.create_label(l.name.clone(), l.color.clone()).unwrap();

    //     let l2 = Label {
    //         name: "label2".to_string(),
    //         color: "#000000".to_string()
    //     };
    //     let label2 = manager.create_label(l2.name.clone(), l2.color.clone()).unwrap();

    //     let date = now_utc().to_timespec();
    //     let i = Item {
    //         id: None,
    //         uuid: "".to_string(),
    //         name: "test item".to_string(),
    //         due_date: None,
    //         completion_date: Some(date.clone()),
    //         labels: vec![label, label2]
    //     };

    //     let item = manager.create_and_fetch_item(&i).expect("expected an item");
    //     assert!(item.uuid.len() > 0);
    //     assert_eq!(item.name, i.name);
    //     assert_eq!(item.due_date, i.due_date);
    //     let completion_date = item.completion_date.expect("expecting a completion date");
    //     assert_eq!(completion_date.sec, date.sec);
    //     assert_eq!(item.labels, i.labels);
    // }

    // #[test]
    // fn test_create_item_no_completion_date() {
    //     let mut manager = list_manager();
    //     let l = Label {
    //         name: "label1".to_string(),
    //         color: "#000000".to_string()
    //     };
    //     let label = manager.create_label(l.name.clone(), l.color.clone()).unwrap();

    //     let l2 = Label {
    //         name: "label2".to_string(),
    //         color: "#000000".to_string()
    //     };
    //     let label2 = manager.create_label(l2.name.clone(), l2.color.clone()).unwrap();

    //     let date = now_utc().to_timespec();
    //     let i = Item {
    //         id: None,
    //         uuid: "".to_string(),
    //         name: "test item".to_string(),
    //         due_date: Some(date.clone()),
    //         completion_date: None,
    //         labels: vec![label, label2]
    //     };

    //     let item = manager.create_and_fetch_item(&i).expect("expected an item");
    //     assert!(item.uuid.len() > 0);
    //     assert_eq!(item.name, i.name);
    //     let due_date = item.due_date.expect("expecting a due date");
    //     assert_eq!(due_date.sec, date.sec);
    //     assert_eq!(item.completion_date, i.completion_date);
    //     assert_eq!(item.labels, i.labels);
    // }

    // #[test]
    // fn test_fetch_item() {
    //     let mut manager = list_manager();
    //     let label = manager.create_label("label1".to_string(), "#000000".to_string()).unwrap();
    //     let mut created_item = Item {
    //         id: None,
    //         uuid: "".to_string(),
    //         name: "test item".to_string(),
    //         due_date: None,
    //         completion_date: None,
    //         labels: vec![label]
    //     };

    //     created_item.uuid = manager.create_item(&created_item);
    //     let fetched_item = manager.fetch_item(&created_item.uuid).expect("expected an item");
    //     assert_eq!(fetched_item, created_item);

    //     let fetched_item = manager.fetch_item(&"doesn't exist".to_string());
    //     assert_eq!(fetched_item, None);
    // }

    // #[test]
    // fn test_fetch_labels_for_item() {
    //     let mut manager = list_manager();
    //     let label = manager.create_label("label1".to_string(), "#000000".to_string()).unwrap();
    //     let label2 = manager.create_label("label2".to_string(), "#000000".to_string()).unwrap();
    //     let label3 = manager.create_label("label3".to_string(), "#000000".to_string()).unwrap();

    //     let mut item1 = Item {
    //         id: None,
    //         uuid: "".to_string(),
    //         name: "test item 1".to_string(),
    //         due_date: None,
    //         completion_date: None,
    //         labels: vec![label, label2, label3]
    //     };

    //     item1.uuid = manager.create_item(&item1);

    //     let fetched_labels = manager.fetch_labels_for_item(&item1.uuid);
    //     assert_eq!(fetched_labels, item1.labels);
    // }

    // #[test]
    // fn test_fetch_items_with_label() {
    //     let mut manager = list_manager();
    //     let label = manager.create_label("label1".to_string(), "#000000".to_string()).unwrap();
    //     let label2 = manager.create_label("label2".to_string(), "#000000".to_string()).unwrap();

    //     let mut item1 = Item {
    //         id: None,
    //         uuid: "".to_string(),
    //         name: "test item 1".to_string(),
    //         due_date: None,
    //         completion_date: None,
    //         labels: vec![label.clone()]
    //     };
    //     let mut item2 = Item {
    //         id: None,
    //         uuid: "".to_string(),
    //         name: "test item 2".to_string(),
    //         due_date: None,
    //         completion_date: None,
    //         labels: vec![label.clone()]
    //     };
    //     let mut item3 = Item {
    //         id: None,
    //         uuid: "".to_string(),
    //         name: "test item 3".to_string(),
    //         due_date: None,
    //         completion_date: None,
    //         labels: vec![label.clone(), label2.clone()]
    //     };

    //     let mut item4 = Item {
    //         id: None,
    //         uuid: "".to_string(),
    //         name: "test item 4".to_string(),
    //         due_date: None,
    //         completion_date: None,
    //         labels: vec![label2.clone()]
    //     };

    //     item1.uuid = manager.create_item(&item1);
    //     item2.uuid = manager.create_item(&item2);
    //     item3.uuid = manager.create_item(&item3);
    //     item4.uuid = manager.create_item(&item4);

    //     let fetched_label1_items = manager.fetch_items_with_label(&label);
    //     assert_eq!(fetched_label1_items, vec![item1, item2, item3.clone()]);
    //     let fetched_label2_items = manager.fetch_items_with_label(&label2);
    //     assert_eq!(fetched_label2_items, vec![item3, item4]);
    // }

    // #[test]
    // fn test_update_item_add_label() {
    //     let mut manager = list_manager();
    //     let label = manager.create_label("label1".to_string(), "#000000".to_string()).unwrap();
    //     let label2 = manager.create_label("label2".to_string(), "#000000".to_string()).unwrap();
    //     let label3 = manager.create_label("label3".to_string(), "#000000".to_string()).unwrap();

    //     let mut item1 = Item {
    //         id: None,
    //         uuid: "".to_string(),
    //         name: "test item 1".to_string(),
    //         due_date: None,
    //         completion_date: None,
    //         labels: vec![label, label2]
    //     };

    //     item1.uuid = manager.create_item(&item1);
    //     let mut new_labels = item1.labels.clone();
    //     new_labels.push(label3);

    //     manager.update_item(&item1, None, None, None, Some(&new_labels));

    //     let fetched_item = manager.fetch_item(&item1.uuid).expect("expected an item");
    //     assert_eq!(fetched_item, item1);
    // }

    // #[test]
    // fn test_update_item_remove_label() {
    //     let mut manager = list_manager();
    //     let label = manager.create_label("label1".to_string(), "#000000".to_string()).unwrap();
    //     let label2 = manager.create_label("label2".to_string(), "#000000".to_string()).unwrap();
    //     let label3 = manager.create_label("label3".to_string(), "#000000".to_string()).unwrap();

    //     let mut item1 = Item {
    //         id: None,
    //         uuid: "".to_string(),
    //         name: "test item 1".to_string(),
    //         due_date: None,
    //         completion_date: None,
    //         labels: vec![label, label2, label3]
    //     };

    //     item1.uuid = manager.create_item(&item1);
    //     let mut new_labels = item1.labels.clone();
    //     new_labels.remove(2);

    //     manager.update_item(&item1, None, None, None, Some(&new_labels));

    //     let fetched_item = manager.fetch_item(&item1.uuid).expect("expected an item");
    //     assert_eq!(fetched_item, item1);
    // }

    // #[test]
    // fn test_update_item_add_due_date() {
    //     let mut manager = list_manager();
    //     let label = manager.create_label("label1".to_string(), "#000000".to_string()).unwrap();
    //     let label2 = manager.create_label("label2".to_string(), "#000000".to_string()).unwrap();
    //     let label3 = manager.create_label("label3".to_string(), "#000000".to_string()).unwrap();

    //     let date = now_utc().to_timespec();
    //     let mut item1 = Item {
    //         id: None,
    //         uuid: "".to_string(),
    //         name: "test item 1".to_string(),
    //         due_date: None,
    //         completion_date: None,
    //         labels: vec![label, label2, label3]
    //     };

    //     item1.uuid = manager.create_item(&item1);

    //     manager.update_item(&item1, None, Some(date), None, None);

    //     let fetched_item = manager.fetch_item(&item1.uuid).expect("expected an item");
    //     let due_date = fetched_item.due_date.expect("expected a due date");
    //     assert_eq!(due_date.sec, item1.due_date.unwrap().sec);
    // }

    // #[test]
    // fn test_update_item_change_name() {
    //     let mut manager = list_manager();
    //     let label = manager.create_label("label1".to_string(), "#000000".to_string()).unwrap();
    //     let label2 = manager.create_label("label2".to_string(), "#000000".to_string()).unwrap();
    //     let label3 = manager.create_label("label3".to_string(), "#000000".to_string()).unwrap();

    //     let date = now_utc().to_timespec();
    //     let mut item1 = Item {
    //         id: None,
    //         uuid: "".to_string(),
    //         name: "test item 1".to_string(),
    //         due_date: Some(date),
    //         completion_date: None,
    //         labels: vec![label, label2, label3]
    //     };

    //     item1.uuid = manager.create_item(&item1);
    //     manager.update_item(&item1, Some("new name".to_string()), None, None, None);

    //     let fetched_item = manager.fetch_item(&item1.uuid).expect("expected an item");
    //     assert_eq!(fetched_item.name, item1.name);
    // }

    // #[test]
    // fn test_update_item_complete_item() {
    //     let mut manager = list_manager();
    //     let label = manager.create_label("label1".to_string(), "#000000".to_string()).unwrap();
    //     let label2 = manager.create_label("label2".to_string(), "#000000".to_string()).unwrap();
    //     let label3 = manager.create_label("label3".to_string(), "#000000".to_string()).unwrap();

    //     let date = now_utc().to_timespec();
    //     let mut item1 = Item {
    //         id: None,
    //         uuid: "".to_string(),
    //         name: "test item 1".to_string(),
    //         due_date: None,
    //         completion_date: None,
    //         labels: vec![label, label2, label3]
    //     };

    //     item1.uuid = manager.create_item(&item1);
    //     manager.update_item(&item1, None, None, Some(date), None);

    //     let fetched_item = manager.fetch_item(&item1.uuid).expect("expected an item");
    //     let completion_date = fetched_item.completion_date.expect("expected a completion_date");
    //     assert_eq!(completion_date.sec, date.sec);
    // }
}
