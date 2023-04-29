use serde::Serialize;
use crate::to_do::ItemTypes;
use crate::to_do::structs::base::Base;

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

impl ToDoItems {
    pub fn new(items: Vec<ItemTypes>) -> ToDoItems {
        let mut pending_items = Vec::new();
        let mut done_items = Vec::new();
        for item in items {
            match item {
                ItemTypes::Pending(packed) => pending_items.push(packed.to_do),
                ItemTypes::Done(packed) => done_items.push(packed.to_do),
            }
        }
        let pending_item_count = pending_items.len() as i8;
        let done_item_count = done_items.len() as i8;

        ToDoItems{
            pending_items,
            done_items,
            pending_item_count,
            done_item_count,
        }
    }   
} 
