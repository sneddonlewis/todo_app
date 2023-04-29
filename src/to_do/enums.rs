use std::fmt::{Display, Formatter, Result};
use serde::ser::{Serialize, Serializer};

pub enum TaskStatus {
    DONE,
    PENDING,
}

impl TaskStatus {
    pub fn from_string(input: &str) -> Self {
        match input {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => panic!("input {} not supported", input),
        }
    }
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TaskStatus::DONE => write!(f, "{}", "DONE".to_string()),
            TaskStatus::PENDING => write!(f, "{}", "PENDING".to_string()),
        }
    }
}

impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer {
        Ok(serializer.serialize_str(&self.to_string().as_str())?)
    }
}
