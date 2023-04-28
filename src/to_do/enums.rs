use std::fmt::{Display, Formatter, Result};

pub enum TaskStatus {
    DONE,
    PENDING,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TaskStatus::DONE => write!(f, "{}", "DONE".to_string()),
            TaskStatus::PENDING => write!(f, "{}", "PENDING".to_string()),
        }
    }
}
