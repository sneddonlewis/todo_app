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
    fn get(&self, title: &str) {
        println!("Getting done {}", title);
    }
}

impl Edit for Done {
    fn set_to_done(&self, title: &str) {
        println!("Wat!, {} is  done already", title);
    }
    fn set_to_pending(&self, title: &str) {
        println!("Set {} from done to pending", title);
    }
}

impl Delete for Done {
    fn delete(&self, title: &str) {
        println!("deleting {}", title);
    }
}
