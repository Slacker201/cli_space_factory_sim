use std::{collections::HashMap, fs::File, io::Write};

use bincode::config;

use crate::{
    command_parsing::command_struct::Command, error, info, item_utils::recipe::recipe::Recipe, warn,
};

static CFG: config::Configuration = bincode::config::standard();
/// This saves the recipes vector to the given file, or "assets/recipe.sgs" if it is not given
pub fn save_recipes_cmd(cmd: Command, recipes: &mut HashMap<String, Recipe>) {
    match cmd.args().get("location") {
        Some(loc) => {
            let location = match loc.first() {
                Some(location) => match location {
                    crate::command_parsing::command_token::CommandToken::BooleanTrue => {
                        warn!("Value was a boolean flag");
                        return;
                    }
                    crate::command_parsing::command_token::CommandToken::Value(var) => var,
                },
                None => {
                    error!("Location has no value");
                    return;
                }
            };
            write_to_location(&location, recipes);
        }
        None => {
            write_to_location("assets/recipe.sgs", recipes);
        }
    }
}

/// This encodes the recipe vector and saves it to a file, printing an error and returning if it fails
fn write_to_location(loc: &str, recipes: &HashMap<String, Recipe>) {
    let mut target_file = match File::create(loc) {
        Ok(file) => {
            info!("Successfully created file");
            file
        }
        Err(e) => {
            error!("Failed to open file: {e}");
            return;
        }
    };
    match bincode::encode_to_vec(recipes, CFG) {
        Ok(encoded_data) => {
            let write_to_file_error = target_file.write_all(&encoded_data);

            if write_to_file_error.is_err() {
                error!("Error writing to file: {write_to_file_error:?}")
            }
        }
        Err(e) => {
            error!("Error encoding data: {e}");
        }
    };
}
