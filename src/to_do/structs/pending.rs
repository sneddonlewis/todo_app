use crate::to_do::traits::create::Create;
use crate::to_do::traits::delete::Delete;

use super::base::Base;
use super::super::enums::TaskStatus;
use super::super::traits::get::Get;
use super::super::traits::edit::Edit;

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

impl Get for Pending {
}

impl Edit for Pending {
}

impl Delete for Pending {}

impl Create for Pending {
    
}
