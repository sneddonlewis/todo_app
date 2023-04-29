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
    fn get(&self, title: &str) {
        println!("Getting pending {}", title);
    }
}

impl Edit for Pending {
    fn set_to_done(&self, title: &str) {
        println!("Set {} from pending to done", title);
    }
    fn set_to_pending(&self, title: &str) {
        println!("{} is already pending", title);
    }
}
