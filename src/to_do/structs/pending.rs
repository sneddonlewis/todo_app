use super::base::Base;
use super::super::enums::TaskStatus;

pub struct Pending {
    pub to_do: Base,
}

impl Pending {
    pub fn new(title: &str) -> Self {
        let to_do = Base{
            title: title.to_string(),
            status: TaskStatus::PENDING,
        };
        Pending{
            to_do,
        }
    }
}

