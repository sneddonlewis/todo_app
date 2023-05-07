use crate::{diesel, database::DB};
use diesel::prelude::*;
use actix_web::{HttpResponse, web::Json};

use crate::{json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems}, jwt::JwToken};
use crate::schema::to_do;

pub async fn edit(to_do_item: Json<ToDoItem>, token: JwToken, db: DB) -> HttpResponse {
    println!("JWT message: {}", token.message);

    let results = to_do::table
        .filter(to_do::columns::title.eq(&to_do_item.title));

    let _ = diesel::update(results)
        .set(to_do::columns::status.eq("DONE"))
        .execute(&db.connection);

    HttpResponse::Ok().json(ToDoItems::get_state())
}
