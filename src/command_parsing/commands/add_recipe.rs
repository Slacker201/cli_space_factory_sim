use std::collections::HashMap;

use crate::{
    command_parsing::{
        command_struct::Command,
        commands::{command_error::CommandError, command_utils::get_single_arg},
    },
    error, info,
    item_utils::{
        item::{item::Item, item_builder::ItemBuilder},
        recipe::recipe::Recipe,
    },
    warn,
};

/// This adds a recipe to the recipe vector if it has the correct arguments. Otherwise it fails
pub fn add_recipe_cmd(
    cmd: Command,
    recipes: &mut HashMap<String, Recipe>,
) -> Result<(), CommandError> {
    info!(": [add_recipe] Adding recipe");
    let input_items = match get_item_args("input_item", &cmd) {
        Some(items) => items,
        None => {
            error!("No input item arguments found");
            return Err(CommandError::MissingArgument("input_item".to_string()));
        }
    };
    let output_items = match get_item_args("output_item", &cmd) {
        Some(items) => items,
        None => {
            error!("No output item arguments found");
            return Err(CommandError::MissingArgument("output_item".to_string()));
        }
    };

    let processing_time = match get_single_arg("processing_time", &cmd) {
        Some(str) => match str.parse::<u32>() {
            Ok(val) => val,
            Err(e) => {
                return Err(CommandError::CommandParseIntError(e));
            }
        },
        None => {
            return Err(CommandError::MissingArgument("processing_time".to_string()));
        }
    };

    let heat_produced_per_tick = match get_single_arg("heat_produced", &cmd) {
        Some(str) => match str.parse::<u32>() {
            Ok(val) => val,
            Err(e) => {
                return Err(CommandError::CommandParseIntError(e));
            }
        },
        None => {
            return Err(CommandError::MissingArgument("heat_produced".to_string()));
        }
    };
    let power_draw = match get_single_arg("power_draw", &cmd) {
        Some(str) => match str.parse::<u32>() {
            Ok(val) => val,
            Err(e) => {
                return Err(CommandError::CommandParseIntError(e));
            }
        },
        None => {
            return Err(CommandError::MissingArgument("power_draw".to_string()));
        }
    };
    let name = match get_single_arg("name", &cmd) {
        Some(str) => str.replace("^", " "),
        None => {
            return Err(CommandError::MissingArgument("name".to_string()));
        }
    };
    let mut recipe = Recipe::new();
    recipe.set_input_items(input_items);
    recipe.set_output_items(output_items);
    recipe.set_processing_time(processing_time);
    recipe.set_heat_produced(heat_produced_per_tick);
    recipe.set_name(name.clone());
    recipe.set_power_draw(power_draw);
    info!("{:?}", recipe);
    recipes.insert(name, recipe);
    Ok(())
}

/// This returns an option for vector holding items. The items are found by parsing a given argument
fn get_item_args(argument_name: &str, cmd: &Command) -> Option<Vec<Item>> {
    let mut input_items = Vec::new();
    match cmd.args().get(argument_name) {
        Some(argument_list) => {
            for arg in argument_list {
                match arg {
                    crate::command_parsing::command_token::CommandToken::BooleanTrue => {
                        warn!(
                            "The argument value given is not an item. The correct format is id:count"
                        );
                    }
                    crate::command_parsing::command_token::CommandToken::Value(
                        id_count_amalgamation,
                    ) => {
                        let arg_parts: Vec<&str> = id_count_amalgamation.split(':').collect();
                        if arg_parts.len() != 2 {
                            warn!("Invalid item format. The correct format is id:count");
                            continue;
                        }
                        let id = match arg_parts[0].parse::<u64>() {
                            Ok(parsed_id) => parsed_id,
                            Err(e) => {
                                warn!(
                                    "Error parsing the count of {}. Error is {}",
                                    id_count_amalgamation, e
                                );
                                continue;
                            }
                        };
                        let count = match arg_parts[1].parse::<u128>() {
                            Ok(parsed_count) => parsed_count,
                            Err(e) => {
                                warn!(
                                    "Error parsing the count of {}. Error is {}",
                                    id_count_amalgamation, e
                                );
                                continue;
                            }
                        };

                        input_items.push(ItemBuilder::new().set_count(count).set_id(id).build());
                    }
                }
            }
        }
        None => {
            warn!("No items given");
            return None;
        }
    }
    Some(input_items)
}
