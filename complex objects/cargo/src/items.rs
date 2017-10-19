
use time::Timespec;
use time::now;

use categories::Category;
use connection;

#[derive(Debug, Clone)]
pub struct Item {
    pub id: isize,
    pub description: String,
    pub created_at: Timespec,
    pub due_date: Option<Timespec>,
    pub is_complete: bool
}

impl Drop for Item {
    fn drop(&mut self) {
        println!("{:?} is being deallocated", self);
    }
}

pub fn fetch_items_for_category(category: &Category) -> Vec<Item> {
    let sql = r#"SELECT id, description, created_at, due_date, is_complete
                    FROM items
                    WHERE category=?"#;
    let conn = connection();
    let mut stmt = conn.prepare(sql).unwrap();
    let mut item_iter = stmt.query_map(&[&category.id], |row| {
        Item {
            id: row.get(0),
            description: row.get(1),
            created_at: row.get(2),
            due_date: row.get(3),
            is_complete: row.get(4)
        }
    }).unwrap();

    let mut item_list: Vec<Item> = Vec::new();

    loop {
        match item_iter.next() {
            Some(result) => {
                match result {
                    Ok(i) => {
                        item_list.push(i);
                    },
                    Err(_) => {}
                }
            },
            None => break
        }
    }
    item_list
}

#[no_mangle]
pub extern "C" fn item_new() -> *mut Item {
    let item = Item{
        id: 1,
        description: "description".to_string(),
        created_at: now().to_timespec(),
        due_date: None,
        is_complete: false
    };
    let boxed_item = Box::new(item);
    Box::into_raw(boxed_item)
}

#[no_mangle]
pub unsafe extern "C" fn item_destroy(item: *mut Item) {
    let _ = Box::from_raw(item);
}
