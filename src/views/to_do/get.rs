use actix_web::Responder;

use crate::json_serialization::to_do_items::ToDoItems;

pub async fn get_all() -> impl Responder {
    ToDoItems::get_state()
}
