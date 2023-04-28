mod to_do;

use to_do::to_do_factory;
use to_do::ItemTypes;
use crate::to_do::enums::TaskStatus;

fn main() {
    let washing = to_do_factory("washing", TaskStatus::DONE);
    let shopping = to_do_factory("shopping", TaskStatus::DONE);
    let laundry = to_do_factory("laundry", TaskStatus::PENDING);
    match washing {
        ItemTypes::Done(item) => println!("{} is done", item.to_do.title),
        ItemTypes::Pending(item) => println!("{} is not yet done", item.to_do.title),
    }
    match shopping {
        ItemTypes::Done(item) => println!("{} is done", item.to_do.title),
        ItemTypes::Pending(item) => println!("{} is not yet done", item.to_do.title),
    }
    match laundry {
        ItemTypes::Done(item) => println!("{} is done", item.to_do.title),
        ItemTypes::Pending(item) => println!("{} is not yet done", item.to_do.title),
    }
}
