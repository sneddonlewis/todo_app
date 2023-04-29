use super::base::Base;
use super::super::enums::TaskStatus;
use super::super::traits::get::Get;
use super::super::traits::edit::Edit;
use super::super::traits::delete::Delete;

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

impl Get for Done {
}

impl Edit for Done {
}

impl Delete for Done {
}
