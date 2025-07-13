
use crate::{command_line_interface::command_struct::Command, item_utils::{item::{item::Item, item_builder::ItemBuilder}, recipe::recipe::Recipe}};





pub fn add_recipe_cmd(cmd: Command, recipes: &mut Vec<Recipe>) {
    println!("Adding Recipe");
    let input_items = match get_item_args("input_item", &cmd) {
        Some(items) => {
            items
        },
        None => {
            return;
        },
    };
    let output_items = match get_item_args("output_item", &cmd) {
        Some(items) => {
            items
        },
        None => {
            return;
        },
    };

    let mut recipe = Recipe::new();
    recipe.set_input_items(input_items);
    recipe.set_output_items(output_items);
    println!("{:?}", recipe);
    recipes.push(recipe);
}

fn get_item_args(argument_name: &str, cmd: &Command) -> Option<Vec<Item>>{
    let mut input_items = Vec::new();
    match cmd.args().get(argument_name) {
        Some(argument_list) => {
            for arg in argument_list {
                match arg {
                    crate::command_line_interface::command_dispatcher::ArgumentFlag::BooleanTrue => {
                        println!("The argument value given is not an item. The correct format is id:count")
                    },
                    crate::command_line_interface::command_dispatcher::ArgumentFlag::Value(id_count_amalgamation) => {
                        let arg_parts: Vec<&str> = id_count_amalgamation.split(':').collect();
                        if arg_parts.len() != 2 {
                            println!("Invalid item format. The correct format is id:count");
                            return None;
                        }
                        let id;
                        let count;
                        match arg_parts[0].parse::<u64>() {
                            Ok(parsed_id) => {
                                id = parsed_id
                            },
                            Err(e) => {
                                println!("Error parsing the count of {}. Error is {}", id_count_amalgamation, e);
                                return None;
                            },
                        }
                        match arg_parts[1].parse::<u128>() {
                            Ok(parsed_count) => {
                                count = parsed_count
                            },
                            Err(e) => {
                                println!("Error parsing the count of {}. Error is {}", id_count_amalgamation, e);
                                return None;
                            },
                        }

                        input_items.push(ItemBuilder::new().set_count(count).set_id(id).build());
                    },
                }
            }
        },
        None => {
            println!("No items given");
            return None;
        },
    }
    Some(input_items)
}