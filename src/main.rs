mod state;
use std::env;
use state::{write_to_file, read_file};
use serde_json::value::Value;
use serde_json::{Map, json};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let status = &args[1];
    let title = &args[2];
    // let mut state = Map::new(); first time so there's valid json
    let mut state: Map<String, Value> = read_file("state.json");
    state.insert(title.to_string(), json!(status));
    write_to_file("state.json", &mut state);
}
