#[derive(Debug, Clone)]
pub struct Item {
    pub id: isize,
    pub description: String,
    pub created_at: isize,
    pub due_date: Option<isize>
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
        created_at: 0,
        due_date: None,
    };
    let boxed_item = Box::new(item);
    Box::into_raw(boxed_item)
}

#[no_mangle]
pub unsafe extern "C" fn item_destroy(item: *mut Item) {
    let _ = Box::from_raw(item);
}
