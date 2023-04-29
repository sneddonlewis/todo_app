use std::fs::File;
use std::fs;
use std::io::Read;
use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let mut file = File::open(file_name.to_string()).expect("could not open file");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("could not read from file");
    let json_data = serde_json::from_str(&data).expect("could not deserialize from json");
    json_data
        .as_object::<Map<String, Value>>()
        .expect("could not map json to hash table")
        .clone()
}

pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
    let json_data = json!(state);
    fs::write(file_name.to_string(), json_data.to_string())
        .expect("could not write to file");
}
