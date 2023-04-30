use actix_web::{web::Json, HttpResponse};
use serde_json::{Map, Value};
use crate::{json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems}, state::read_file, to_do::{enums::TaskStatus, to_do_factory}, processes::process_input, jwt::JwToken};


pub async fn delete(to_do_item: Json<ToDoItem>, token: JwToken) -> HttpResponse {
    println!("JWT message: {}", token.message);

    let state: Map<String, Value> = read_file("state.json");
    let status: TaskStatus;

    match &state.get(&to_do_item.title) {
        Some(result) => {
            status = TaskStatus::from_string(result.as_str().unwrap());
        },
        None => return HttpResponse::NotFound().json(format!("{} not in state", &to_do_item.title)),
    }

    let existing_item = to_do_factory(to_do_item.title.as_str(), status.clone());

    process_input(existing_item, "delete".to_string(), &state);
    HttpResponse::Ok().json(ToDoItems::get_state())
}
