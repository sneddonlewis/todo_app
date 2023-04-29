use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;
use crate::state::write_to_file;

pub trait Create {
    fn create(&self, title: &str, status: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(status));
        write_to_file("state.json", state);
    }
}
