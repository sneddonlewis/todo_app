use actix_web::{HttpResponse, web::Json};
use serde_json::{Map, Value};

use crate::{json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems}, state::read_file, to_do::{enums::TaskStatus, to_do_factory}, processes::process_input};

pub async fn edit(to_do_item: Json<ToDoItem>) -> HttpResponse {
    let state: Map<String, Value> = read_file("state.json");
    let status: TaskStatus;

    match &state.get(&to_do_item.title) {
        Some(result) => {
            status = TaskStatus::from_string(result.as_str().unwrap());
        },
        None => return HttpResponse::NotFound().json(format!("{} not in state", &to_do_item.title)),
    }

    let existing_item = to_do_factory(to_do_item.title.as_str(), status.clone());

    if &status.to_string() == &TaskStatus::from_string(&to_do_item.status.as_str()).to_string() {
        return HttpResponse::Ok().json(ToDoItems::get_state());
    }

    process_input(existing_item, "edit".to_string(), &state);
    HttpResponse::Ok().json(ToDoItems::get_state())
}
