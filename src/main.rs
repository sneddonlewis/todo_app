mod to_do;

use to_do::to_do_factory;
use to_do::ItemTypes;
use to_do::enums::TaskStatus;
use to_do::traits::get::Get;
use to_do::traits::edit::Edit;
use to_do::traits::delete::Delete;

fn main() {
    let washing = to_do_factory("washing", TaskStatus::DONE);
    match washing {
        ItemTypes::Done(item) => {
            item.get(&item.to_do.title);
            item.delete(&item.to_do.title);
        },
        ItemTypes::Pending(item) => {
            item.get(&item.to_do.title);
            item.set_to_done(&item.to_do.title);
        },
    }
}
