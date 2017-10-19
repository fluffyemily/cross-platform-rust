
use time::Timespec;
use time::now;

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
