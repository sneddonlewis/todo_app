use serde_json::Map;
use serde_json::value::Value;
use super::to_do::structs::done::Done;
use super::to_do::structs::pending::Pending;
use super::to_do::traits::get::Get;
use super::to_do::traits::create::Create;
use super::to_do::traits::edit::Edit;
use super::to_do::traits::delete::Delete;
use super::to_do::ItemTypes;

pub fn process_input(item: ItemTypes, command: String, state: &Map<String, Value>) {
    match item {
        ItemTypes::Pending(item) => process_pending(item, command, state),
        ItemTypes::Done(item) => process_done(item, command, state),
    }
}

fn process_pending(item: Pending, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.to_do.title, &state),
        "create" => item.create(&item.to_do.title, &item.to_do.status.to_string(), &mut state),
        "edit" => item.set_to_done(&item.to_do.title, &mut state),
        _ => println!("command {} not supported", command),
    }
}

fn process_done(item: Done, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.to_do.title, &state),
        "delete" => item.delete(&item.to_do.title, &mut state),
        "edit" => item.set_to_pending(&item.to_do.title, &mut state),
        _ => println!("command {} not supported", command),
    }
}
