use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;
use crate::state::write_to_file;
use super::super::enums::TaskStatus;

pub trait Edit {
    fn set_to_done(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(TaskStatus::DONE.to_string()));
        write_to_file("state.json", state);
    }

    fn set_to_pending(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(TaskStatus::PENDING.to_string()));
        write_to_file("state.json", state);
    }
}
