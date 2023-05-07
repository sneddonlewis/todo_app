use crate::diesel;
use diesel::prelude::*;
use crate::database::establish_connection;
use crate::models::item::item::Item;
use crate::schema::to_do;
use actix_web::http::header::ContentType;
use actix_web::{Responder, HttpRequest, HttpResponse};
use actix_web::body::BoxBody;
use serde::Serialize;

use crate::to_do::{ItemTypes, to_do_factory};
use crate::to_do::enums::TaskStatus;
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

    pub fn get_state() -> ToDoItems {
        let connection = establish_connection();
        let mut buffer = Vec::new();

        let items = to_do::table
            .order(to_do::columns::id.asc())
            .load::<Item>(&connection)
            .unwrap();

        for item in items {
            let status = TaskStatus::from_string(&item.status.as_str().to_string());
            let res = to_do_factory(&item.title, status);
            buffer.push(res);
        }
        ToDoItems::new(buffer)
    }
} 

impl Responder for ToDoItems {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();    
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
