
use crate::{command_line_interface::command_struct::Command, item_utils::{item::item_builder::ItemBuilder, recipe::recipe::Recipe}};





pub fn add_recipe_cmd(cmd: Command, recipes: &mut Vec<Recipe>) {
    let item_id;
    let item_count;
    match cmd.args().get("item_id") {
        Some(id_str) => {
            match id_str.parse::<u64>() {
                Ok(arg) => {
                    item_id = arg
                },
                Err(e) => {
                    println!("Error in parsing item_id: {}", e);
                    return;
                },
            }
        },
        None => {
            println!("Missing item_id argument");
            return;
        },
    }
    match cmd.args().get("item_count") {
        Some(id_str) => {
            match id_str.parse::<u128>() {
                Ok(arg) => {
                    item_count = arg
                },
                Err(e) => {
                    println!("Error in parsing item_count: {}", e);
                    return;
                },
            }
        },
        None => {
            println!("Missing item_count argument");
            return;
        },
    }
    let mut rec = Recipe::new();
    rec.set_input_items(Vec::from([ItemBuilder::new().set_count(item_count).set_id(item_id).build()]));
    println!("{:?}", rec);
    recipes.push(rec);
}
