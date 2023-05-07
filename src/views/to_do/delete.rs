use crate::{diesel, database::DB};
use diesel::prelude::*;
use actix_web::{web::Json, HttpResponse};
use crate::{json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems},  jwt::JwToken};
use crate::schema::to_do;

pub async fn delete(to_do_item: Json<ToDoItem>, token: JwToken, db: DB) -> HttpResponse {
    println!("JWT message: {}", token.message);

    let results = to_do::table
        .filter(to_do::columns::title.eq(&to_do_item.title));

    let _ = diesel::delete(results)
        .execute(&db.connection);

    HttpResponse::Ok().json(ToDoItems::get_state())
}
