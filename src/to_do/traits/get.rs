use serde_json::Map;
use serde_json::value::Value;

pub trait Get {
    fn get(&self, title: &str, state: &Map<String, Value>) {
        match state.get(title) {
            Some(result) => {
                println!("\n\nItem: {}", title);
                println!("Status: {}\n\n", result);
            },
            None => println!("Item: {} was not found", title),
        }
    }
}
