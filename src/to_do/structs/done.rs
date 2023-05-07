use super::base::Base;
use super::super::enums::TaskStatus;

pub struct Done {
    pub to_do: Base,
}

impl Done {
    pub fn new(title: &str) -> Self {
        let to_do = Base{
            title: title.to_string(),
            status: TaskStatus::DONE,
        };
        Done{
            to_do
        }
    }
}

