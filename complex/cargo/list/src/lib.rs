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
use std::sync::Arc;

use mentat::query::QueryResults;
use mentat_core::Uuid;
use time::Timespec;

pub mod labels;
pub mod items;
pub mod errors;

use errors as list_errors;
use ffi_utils::strings::c_char_to_string;
use items::Item;
use labels::Label;
use store::{
    Store,
    ToInner
};

#[derive(Debug)]
#[repr(C)]
pub struct ListManager {
    store: Arc<Store>,
}

impl ListManager {
    pub fn new(store: Arc<Store>) -> Result<ListManager, list_errors::Error> {
        let mut manager = ListManager {
            store: store,
        };
        manager.create_labels_table()?;
        manager.create_items_table()?;
        Ok(manager)
    }

    fn write_connection(&mut self) -> &mut Store {
        Arc::get_mut(&mut self.store).unwrap()
    }

    pub fn create_labels_table(&mut self) -> Result<(), list_errors::Error> {
        let schema = r#"[{  :db/ident     :label/name
    :db/valueType :db.type/string
    :db/cardinality :db.cardinality/one
    :db/unique :db.unique/identity
    :db/fulltext true },
 {  :db/ident     :label/color
    :db/valueType :db.type/string
    :db/cardinality :db.cardinality/one }]"#;
        let _ = self.write_connection().transact(schema)?;
        Ok(())
    }

    pub fn create_label(&mut self, name: String, color: String) -> Result<Option<Label>, list_errors::Error> {
        let query = format!("[{{ :label/name \"{0}\" :label/color \"{1}\" }}]", &name, &color);
        self.write_connection().transact(&query)?;
        self.fetch_label(&name)
    }

    pub fn fetch_label(&self, name: &String) -> Result<Option<Label>, list_errors::Error> {
        let query = r#"[:find ?eid, ?name, ?color
            :in ?name
            :where
            [?eid :label/name ?name]
            [?eid :label/color ?color]
        ]"#;
        let result = Arc::clone(&self.store).query_args(query, &[&(&"?name".to_string(), &name)])?;
        if let QueryResults::Rel(rows) = result {
            if !rows.is_empty() {
                Ok(Label::from_row(&rows[0]))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub fn fetch_labels(&self) -> Result<Vec<Label>, list_errors::Error> {
        let query = r#"[:find ?eid, ?name, ?color
            :where
            [?eid :label/name ?name]
            [?eid :label/color ?color]
        ]"#;
        let result = Arc::clone(&self.store).query(query)?;
        if let QueryResults::Rel(rows) = result {
            Ok(rows.iter().map(|row| Label::from_row(&row).unwrap()).collect())
        } else {
            Ok(vec![])
        }
    }

    pub fn fetch_labels_for_item(&self, item_uuid: &Uuid) -> Result<Vec<Label>, list_errors::Error> {
        let query = r#"[:find ?l, ?name, ?color
            :in ?item_uuid
            :where
            [?l :label/name ?name]
            [?l :label/color ?color]
            [?i :item/label ?l]
            [?i :item/uuid ?item_uuid]
        ]"#;
        let result = Arc::clone(&self.store).query_args(query, &[&(&"?item_uuid".to_string(), &item_uuid)])?;
        if let QueryResults::Rel(rows) = result {
            Ok(rows.iter().filter_map(|row| Label::from_row(&row)).collect())
        } else {
            println!("no labels for item {:?}", item_uuid);
            Ok(vec![])
        }
    }

    pub fn create_items_table(&mut self) -> Result<(), list_errors::Error> {
        let schema = r#"[
        {   :db/ident       :item/uuid
            :db/valueType   :db.type/uuid
            :db/cardinality :db.cardinality/one
            :db/unique      :db.unique/value
            :db/index true },
        {   :db/ident       :item/name
            :db/valueType   :db.type/string
            :db/cardinality :db.cardinality/one
            :db/fulltext    true  },
        {   :db/ident       :item/due_date
            :db/valueType   :db.type/instant
            :db/cardinality :db.cardinality/one  },
        {   :db/ident       :item/completion_date
            :db/valueType   :db.type/instant
            :db/cardinality :db.cardinality/one  },
        {   :db/ident       :item/label
            :db/valueType   :db.type/ref
            :db/cardinality :db.cardinality/many }]"#;
        let _ = self.write_connection().transact(schema)?;
        Ok(())
    }

    pub fn fetch_items_with_label(&self, label: &Label) -> Result<Vec<Item>, list_errors::Error> {
        let query = r#"[:find ?uuid
            :in ?label
            :where
            [?eid :item/uuid ?uuid]
            [?eid :item/label ?l]
            [?l :label/name ?label]
        ]"#;
        let result = Arc::clone(&self.store).query_args(query, &[&(&"?label".to_string(), &label.name)])?;
        if let QueryResults::Rel(rows) = result {
            Ok(rows.iter().filter_map(|row| {
                let uuid: Uuid = row[0].to_owned().to_inner();
                self.fetch_item(&uuid).unwrap_or(None)
            }).collect())
        } else {
            println!("no items for label {:?}", label.name);
            Ok(vec![])
        }
    }

    pub fn fetch_item(&self, uuid: &Uuid) -> Result<Option<Item> , list_errors::Error>{
        let query = r#"[:find ?eid, ?uuid, ?name
            :in ?uuid
            :where
            [?eid :item/uuid ?uuid]
            [?eid :item/name ?name]
        ]"#;
        let result = Arc::clone(&self.store).query_args(query, &[&(&"?uuid".to_string(), &uuid)])?;
        if let QueryResults::Rel(rows) = result {
            if !rows.is_empty() {
                let row = &rows[0];
                Ok(Some(Item{
                    id: row[0].clone().to_inner(),
                    uuid: row[1].clone().to_inner(),
                    name: row[2].clone().to_inner(),
                    due_date: self.fetch_date_for_item("due_date", &uuid).unwrap_or(None),
                    completion_date: self.fetch_date_for_item("completion_date", &uuid).unwrap_or(None),
                    labels: self.fetch_labels_for_item(&uuid).unwrap_or(vec![]),
                }))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    fn fetch_date_for_item(&self, attr: &str, item_id: &Uuid) -> Result<Option<Timespec>, list_errors::Error> {
        let query = format!(r#"[:find ?{0}
            :in ?uuid
            :where
            [?eid :item/{0} ?{0}]
            [?eid :item/uuid ?uuid]
        ]"#, attr);
        let result = Arc::clone(&self.store).query_args(&query, &[&(&"?uuid".to_string(), &item_id)])?;
        if let QueryResults::Rel(rows) = result {
            if !rows.is_empty() {
                let row = &rows[0];
                let date: Option<Timespec> = row[0].clone().to_inner();
                Ok(date)
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }

    }

    pub fn create_item(&mut self, item: &Item) -> Result<Uuid, list_errors::Error> {
        let label_str = item.labels.iter().filter_map(|label| {
            if label.id.is_some() {
                Some(format!("{}",label.id.to_owned().unwrap().id))
            } else {
                None
            }
        }).collect::<Vec<String>>().join(", ");
        let tmp_uuid = uuid::Uuid::new_v4().hyphenated().to_string();
        let item_uuid = Uuid::parse_str(&tmp_uuid).unwrap();
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
            query = format!(r#"{0}:item/label [{1}]
                "#, &query, &label_str);
        }
        query = format!("{0}}}]", &query);
        let _ = self.write_connection().transact(&query)?;
        Ok(item_uuid)
    }

    pub fn create_and_fetch_item(&mut self, item: &Item) -> Result<Option<Item>, list_errors::Error> {
        let item_uuid = self.create_item(&item)?;
        self.fetch_item(&item_uuid)
    }

    pub fn update_item(&mut self, item: &Item, name: Option<String>, due_date: Option<Timespec>, completion_date: Option<Timespec>, labels: Option<&Vec<Label>>) -> Result<(), list_errors::Error> {
        let item_id = item.id.to_owned().expect("item must have ID to be updated");
        let mut transaction = vec![];

        if let Some(name) = name {
            if item.name != name {
                transaction.push(format!("[:db/add {0} :item/name \"{1}\"]", &item_id.id, name));
            }
        }
        if item.due_date != due_date {
            if let Some(date) = due_date {
                let micro_seconds = date.sec * 1000000;
                transaction.push(format!("[:db/add {:?} :item/due_date #instmicros {}]", &item_id.id, &micro_seconds));
            } else {
                let micro_seconds = item.due_date.unwrap().sec * 1000000;
                transaction.push(format!("[:db/retract {:?} :item/due_date #instmicros {}]", &item_id.id, &micro_seconds));
            }
        }

        if item.completion_date != completion_date {
            if let Some(date) = completion_date {
                let micro_seconds = date.sec * 1000000;
                transaction.push(format!("[:db/add {:?} :item/completion_date #instmicros {}]", &item_id.id, &micro_seconds));
            } else {
                let micro_seconds = item.completion_date.unwrap().sec * 1000000;
                transaction.push(format!("[:db/retract {:?} :item/completion_date #instmicros {}]", &item_id.id, &micro_seconds));
            }
        }

        if let Some(new_labels) = labels {
            let existing_labels = self.fetch_labels_for_item(&(item.uuid)).unwrap_or(vec![]);

            let labels_to_add = new_labels.iter().filter(|label| !existing_labels.contains(label) ).map(|label| {
                let label_id = label.id.to_owned().unwrap().id;
                format!("{}", label_id)
            }).collect::<Vec<String>>().join(", ");
            if !labels_to_add.is_empty() {
                transaction.push(format!("[:db/add {0} :item/label [{1}]]", &item_id.id, labels_to_add));
            }
            let labels_to_remove = existing_labels.iter().filter(|label| !new_labels.contains(label) ).map(|label| {
                let label_id = label.id.to_owned().unwrap().id;
                format!("{}", label_id)
            }).collect::<Vec<String>>().join(", ");
            if !labels_to_remove.is_empty() {
                transaction.push(format!("[:db/retract {0} :item/label [{1}]]", &item_id.id, labels_to_remove));
            }
        }
        let query = format!("[{0}]", transaction.join(""));
        let _ = self.write_connection().transact(&query)?;
        Ok(())
    }
}

#[no_mangle]
pub unsafe extern "C" fn list_manager_get_all_labels(manager: *const ListManager) -> *mut Vec<Label> {
    let manager = &*manager;
    let label_list = Box::new(manager.fetch_labels().unwrap_or(vec![]));
    Box::into_raw(label_list)
}

#[no_mangle]
pub unsafe extern "C" fn list_manager_create_item(manager: *mut ListManager, item: *const Item) {
    let manager = &mut*manager;
    let item = &*item;
    let _ = manager.create_item(&item);
}

#[no_mangle]
pub unsafe extern "C" fn list_manager_update_item(manager: *mut ListManager, item: *const Item, name: *const c_char, due_date: *const size_t, completion_date: *const size_t, labels: *const Vec<Label>) {
    let manager = &mut*manager;
    let item = &*item;
    let labels = &*labels;
    let name = Some(c_char_to_string(name));
    let due: Option<Timespec>;
    if !due_date.is_null() {
        due = Some(Timespec::new(due_date as i64, 0));
    } else {
        due = None;
    }
    let completion: Option<Timespec>;
    if !completion_date.is_null() {
        completion = Some(Timespec::new(completion_date as i64, 0));
    } else {
        completion = None;
    }
    let _ = manager.update_item(item, name, due, completion, Some(labels));
}

#[no_mangle]
pub unsafe extern "C" fn list_manager_create_label(manager: *mut ListManager, name: *const c_char, color: *const c_char) -> *mut Option<Label> {
    let manager = &mut*manager;
    let name = c_char_to_string(name);
    let color = c_char_to_string(color);
    let label = Box::new(manager.create_label(name, color).unwrap_or(None));
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
    use uuid;


    fn list_manager() -> ListManager {
        let store = Arc::new(Store::new(None).expect("Expected a store"));
        ListManager::new(store).expect("Expected a list manager")
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
        let name = "test".to_string();
        let color = "#000000".to_string();

        let label = manager.create_label(name.clone(), color.clone()).expect("expected a label option");
        assert!(label.is_some());
        let label = label.unwrap();
        assert!(label.id.is_some());
        assert_eq!(label.name, name);
        assert_eq!(label.color, color);
    }

    #[test]
    fn test_fetch_label() {
        let mut manager = list_manager();
        let created_label = manager.create_label("test".to_string(), "#000000".to_string()).expect("expected a label option").expect("Expected a label");
        let fetched_label = manager.fetch_label(&created_label.name).expect("expected a label option").expect("expected a label");
        assert_eq!(fetched_label, created_label);

        let fetched_label = manager.fetch_label(&"doesn't exist".to_string()).expect("expected a label option");
        assert_eq!(fetched_label, None);
    }

    #[test]
    fn test_fetch_labels() {
        let mut manager = list_manager();

        let labels = ["label1".to_string(), "label2".to_string(), "label3".to_string()];
        for label in labels.iter() {
            let _  = manager.create_label(label.clone(), "#000000".to_string()).expect("expected a label option");
        }
        let fetched_labels = manager.fetch_labels().expect("expected a vector of labels");
        assert_eq!(fetched_labels.len(), labels.len());
        for label in fetched_labels.iter() {
            assert!(labels.contains(&label.name));
        }
    }

    #[test]
    fn test_create_item() {
        let mut manager = list_manager();
        let label = manager.create_label("label1".to_string(), "#000000".to_string()).expect("expected a label option").unwrap();
        let label2 = manager.create_label("label2".to_string(), "#000000".to_string()).expect("expected a label option").unwrap();

        let date = now_utc().to_timespec();
        let i = Item {
            id: None,
            uuid: Uuid::nil(),
            name: "test item".to_string(),
            due_date: Some(date.clone()),
            completion_date: Some(date.clone()),
            labels: vec![label, label2]
        };

        let item = manager.create_and_fetch_item(&i).expect("expected an item option").expect("expected an item");
        assert!(!item.uuid.is_nil());
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
            id: None,
            name: "label1".to_string(),
            color: "#000000".to_string()
        };
        let label = manager.create_label(l.name.clone(), l.color.clone()).expect("expected a label option").unwrap();

        let l2 = Label {
            id: None,
            name: "label2".to_string(),
            color: "#000000".to_string()
        };
        let label2 = manager.create_label(l2.name.clone(), l2.color.clone()).expect("expected an item option").unwrap();

        let date = now_utc().to_timespec();
        let i = Item {
            id: None,
            uuid: Uuid::nil(),
            name: "test item".to_string(),
            due_date: None,
            completion_date: Some(date.clone()),
            labels: vec![label, label2]
        };

        let item = manager.create_and_fetch_item(&i).expect("expected an item option").expect("expected an item");
        assert!(!item.uuid.is_nil());
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
            id: None,
            name: "label1".to_string(),
            color: "#000000".to_string()
        };
        let label = manager.create_label(l.name.clone(), l.color.clone()).expect("expected a label option").unwrap();

        let l2 = Label {
            id: None,
            name: "label2".to_string(),
            color: "#000000".to_string()
        };
        let label2 = manager.create_label(l2.name.clone(), l2.color.clone()).expect("expected a label option").unwrap();

        let date = now_utc().to_timespec();
        let i = Item {
            id: None,
            uuid: Uuid::nil(),
            name: "test item".to_string(),
            due_date: Some(date.clone()),
            completion_date: None,
            labels: vec![label, label2]
        };

        let item = manager.create_and_fetch_item(&i).expect("expected an item option").expect("expected an item");
        assert!(!item.uuid.is_nil());
        assert_eq!(item.name, i.name);
        let due_date = item.due_date.expect("expecting a due date");
        assert_eq!(due_date.sec, date.sec);
        assert_eq!(item.completion_date, i.completion_date);
        assert_eq!(item.labels, i.labels);
    }

    #[test]
    fn test_fetch_item() {
        let mut manager = list_manager();
        let label = manager.create_label("label1".to_string(), "#000000".to_string()).expect("expected a label option").unwrap();
        let mut created_item = Item {
            id: None,
            uuid: Uuid::nil(),
            name: "test item".to_string(),
            due_date: None,
            completion_date: None,
            labels: vec![label]
        };

        created_item.uuid = manager.create_item(&created_item).expect("expected a uuid");
        let fetched_item = manager.fetch_item(&created_item.uuid).expect("expected an item option").expect("expected an item");
        assert_eq!(fetched_item.uuid, created_item.uuid);
        assert_eq!(fetched_item.name, created_item.name);
        assert_eq!(fetched_item.due_date, created_item.due_date);
        assert_eq!(fetched_item.completion_date, created_item.completion_date);
        assert_eq!(fetched_item.labels, created_item.labels);

        let tmp_uuid = uuid::Uuid::new_v4().hyphenated().to_string();
        let item_uuid = Uuid::parse_str(&tmp_uuid).unwrap();
        let fetched_item = manager.fetch_item(&item_uuid).expect("expected an item option");
        assert_eq!(fetched_item, None);
    }

    #[test]
    fn test_fetch_labels_for_item() {
        let mut manager = list_manager();
        let label = manager.create_label("label1".to_string(), "#000000".to_string()).expect("expected a label option").unwrap();
        let label2 = manager.create_label("label2".to_string(), "#000000".to_string()).expect("expected a label option").unwrap();
        let label3 = manager.create_label("label3".to_string(), "#000000".to_string()).expect("expected a label option").unwrap();

        let mut item1 = Item {
            id: None,
            uuid: Uuid::nil(),
            name: "test item 1".to_string(),
            due_date: None,
            completion_date: None,
            labels: vec![label, label2, label3]
        };

        item1.uuid = manager.create_item(&item1).expect("expected a uuid");

        let fetched_labels = manager.fetch_labels_for_item(&item1.uuid).expect("expected a vector of labels");
        assert_eq!(fetched_labels, item1.labels);
    }

    #[test]
    fn test_fetch_items_with_label() {
        let mut manager = list_manager();
        let label = manager.create_label("label1".to_string(), "#000000".to_string()).expect("expected a label option").unwrap();
        let label2 = manager.create_label("label2".to_string(), "#000000".to_string()).expect("expected a label option").unwrap();

        let item1 = Item {
            id: None,
            uuid: Uuid::nil(),
            name: "test item 1".to_string(),
            due_date: None,
            completion_date: None,
            labels: vec![label.clone()]
        };
        let item2 = Item {
            id: None,
            uuid: Uuid::nil(),
            name: "test item 2".to_string(),
            due_date: None,
            completion_date: None,
            labels: vec![label.clone()]
        };
        let item3 = Item {
            id: None,
            uuid: Uuid::nil(),
            name: "test item 3".to_string(),
            due_date: None,
            completion_date: None,
            labels: vec![label.clone(), label2.clone()]
        };

        let item4 = Item {
            id: None,
            uuid: Uuid::nil(),
            name: "test item 4".to_string(),
            due_date: None,
            completion_date: None,
            labels: vec![label2.clone()]
        };

        let item1 = manager.create_and_fetch_item(&item1).expect("expected an item option").expect("expected item1");
        let item2 = manager.create_and_fetch_item(&item2).expect("expected an item option").expect("expected item2");
        let item3 = manager.create_and_fetch_item(&item3).expect("expected an item option").expect("expected item3");
        let item4 = manager.create_and_fetch_item(&item4).expect("expected an item option").expect("expected item4");

        let fetched_label1_items = manager.fetch_items_with_label(&label).expect("expected a vector of items");
        assert_eq!(fetched_label1_items, vec![item1, item2, item3.clone()]);
        let fetched_label2_items = manager.fetch_items_with_label(&label2).expect("expected a vector of items");
        assert_eq!(fetched_label2_items, vec![item3, item4]);
    }

    #[test]
    fn test_update_item_add_label() {
        let mut manager = list_manager();
        let label = manager.create_label("label1".to_string(), "#000000".to_string()).expect("expected a label option").unwrap();
        let label2 = manager.create_label("label2".to_string(), "#000000".to_string()).expect("expected a labeloption").unwrap();
        let label3 = manager.create_label("label3".to_string(), "#000000".to_string()).expect("expected a label option").unwrap();

        let item1 = Item {
            id: None,
            uuid: Uuid::nil(),
            name: "test item 1".to_string(),
            due_date: None,
            completion_date: None,
            labels: vec![label, label2]
        };

        let mut created_item = manager.create_and_fetch_item(&item1).expect("expected an item option").expect("expected an item");
        let mut new_labels = item1.labels.clone();
        new_labels.push(label3);

        match manager.update_item(&created_item, None, None, None, Some(&new_labels)) {
            Ok(()) => (),
            Err(e) => {
                println!("e {:?}", e);
                assert!(false)
            }
        }

        created_item.labels = new_labels;

        let fetched_item = manager.fetch_item(&created_item.uuid).expect("expected an item option").expect("expected an item");
        assert_eq!(fetched_item, created_item);
    }

    #[test]
    fn test_update_item_remove_label() {
        let mut manager = list_manager();
        let label = manager.create_label("label1".to_string(), "#000000".to_string()).expect("expected a label option").unwrap();
        let label2 = manager.create_label("label2".to_string(), "#000000".to_string()).expect("expected a label option").unwrap();
        let label3 = manager.create_label("label3".to_string(), "#000000".to_string()).expect("expected a label option").unwrap();

        let item1 = Item {
            id: None,
            uuid: Uuid::nil(),
            name: "test item 1".to_string(),
            due_date: None,
            completion_date: None,
            labels: vec![label, label2, label3]
        };

        let mut created_item = manager.create_and_fetch_item(&item1).expect("expected an item option").expect("expected an item");
        let mut new_labels = created_item.labels.clone();
        new_labels.remove(2);

        match manager.update_item(&created_item, None, None, None, Some(&new_labels)) {
            Ok(()) => (),
            Err(e) => {
                println!("e {:?}", e);
                assert!(false)
            }
        }

        created_item.labels = new_labels;

        let fetched_item = manager.fetch_item(&created_item.uuid).expect("expected an item option").expect("expected an item");
        assert_eq!(fetched_item, created_item);
    }

    #[test]
    fn test_update_item_add_due_date() {
        let mut manager = list_manager();
        let label = manager.create_label("label1".to_string(), "#000000".to_string()).expect("expected a label option").unwrap();
        let label2 = manager.create_label("label2".to_string(), "#000000".to_string()).expect("expected alabel option").unwrap();
        let label3 = manager.create_label("label3".to_string(), "#000000".to_string()).expect("expected a label option").unwrap();

        let date = now_utc().to_timespec();
        let item1 = Item {
            id: None,
            uuid: Uuid::nil(),
            name: "test item 1".to_string(),
            due_date: None,
            completion_date: None,
            labels: vec![label, label2, label3]
        };

        let created_item = manager.create_and_fetch_item(&item1).expect("expected an item option").expect("expected an item");

        match manager.update_item(&created_item, None, Some(date), None, None) {
            Ok(()) => (),
            Err(e) => {
                println!("e {:?}", e);
                assert!(false)
            }
        }

        let fetched_item = manager.fetch_item(&created_item.uuid).expect("expected an item option").expect("expected an item");
        let due_date = fetched_item.due_date.expect("expected a due date");
        assert_eq!(due_date.sec, date.sec);
    }

    #[test]
    fn test_update_item_change_name() {
        let mut manager = list_manager();
        let label = manager.create_label("label1".to_string(), "#000000".to_string()).expect("expected a label option").unwrap();
        let label2 = manager.create_label("label2".to_string(), "#000000".to_string()).expect("expected a label option").unwrap();
        let label3 = manager.create_label("label3".to_string(), "#000000".to_string()).expect("expected a label option").unwrap();

        let date = now_utc().to_timespec();
        let item1 = Item {
            id: None,
            uuid: Uuid::nil(),
            name: "test item 1".to_string(),
            due_date: Some(date),
            completion_date: None,
            labels: vec![label, label2, label3]
        };

        let mut created_item = manager.create_and_fetch_item(&item1).expect("expected an item option").expect("expected an item");
        match manager.update_item(&created_item, Some("new name".to_string()), None, None, None) {
            Ok(()) => (),
            Err(e) => {
                println!("e {:?}", e);
                assert!(false)
            }
        }

        created_item.name = "new name".to_string();

        let fetched_item = manager.fetch_item(&created_item.uuid).expect("expected an item option").expect("expected an item");
        assert_eq!(fetched_item.name, created_item.name);
    }

    #[test]
    fn test_update_item_complete_item() {
        let mut manager = list_manager();
        let label = manager.create_label("label1".to_string(), "#000000".to_string()).expect("expected a label option").unwrap();
        let label2 = manager.create_label("label2".to_string(), "#000000".to_string()).expect("expected a label option").unwrap();
        let label3 = manager.create_label("label3".to_string(), "#000000".to_string()).expect("expected a label option").unwrap();

        let date = now_utc().to_timespec();
        let item1 = Item {
            id: None,
            uuid: Uuid::nil(),
            name: "test item 1".to_string(),
            due_date: None,
            completion_date: None,
            labels: vec![label, label2, label3]
        };

        let created_item = manager.create_and_fetch_item(&item1).expect("expected an item option").expect("expected an item");
        match manager.update_item(&created_item, None, None, Some(date), None) {
            Ok(()) => (),
            Err(e) => {
                println!("e {:?}", e);
                assert!(false)
            }
        }

        let fetched_item = manager.fetch_item(&created_item.uuid).expect("expected an item option").expect("expected an item");
        let completion_date = fetched_item.completion_date.expect("expected a completion_date");
        assert_eq!(completion_date.sec, date.sec);
    }
}
