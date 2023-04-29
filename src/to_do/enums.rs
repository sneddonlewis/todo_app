use std::fmt::{Display, Formatter, Result};

pub enum TaskStatus {
    DONE,
    PENDING,
}

impl TaskStatus {
    pub fn from_string(input: String) -> Self {
        match input.as_str() {
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
