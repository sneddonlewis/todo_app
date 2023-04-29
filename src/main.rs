use actix_web::{App, HttpServer};
mod views;
mod state;
mod to_do;
mod processes;

use std::env;
use state::read_file;
use serde_json::value::Value;
use serde_json::Map;
use to_do::to_do_factory;
use to_do::enums::TaskStatus;
use processes::process_input;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(views::views_factory)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

fn _main() {
    let args = env::args().collect::<Vec<String>>();
    let command = &args[1];
    let title = &args[2];
    // let mut state = Map::new(); first time so there's valid json
    let state: Map<String, Value> = read_file("state.json");

    let status: String;
    match &state.get(*&title) {
        Some(result) => status = result.to_string().replace('\"', ""),
        None => status = "pending".to_owned(),
    }

    let item = to_do_factory(title, TaskStatus::from_string(status.to_uppercase()));

    process_input(item, command.to_string(), &state);
}
