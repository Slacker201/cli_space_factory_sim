use std::{collections::HashMap, fs::File, io::Read};

use bincode::config::{self};

use crate::{
    command_parsing::{self, command_struct::Command},
    error, info,
    item_utils::recipe::recipe::Recipe,
};

static CFG: config::Configuration = bincode::config::standard();
/// This loads the recipe vector from the given location, if the location is not given it uses "assets/recipe.sgs"
pub fn load_recipes_cmd(cmd: Command, recipes: &mut HashMap<String, Recipe>) {
    match cmd.args().get("location") {
        Some(loc) => {
            let location = match loc.first() {
                Some(location) => match location {
                    command_parsing::command_token::CommandToken::BooleanTrue => {
                        error!("Value was a boolean flag");
                        return;
                    }
                    command_parsing::command_token::CommandToken::Value(var) => var,
                },
                None => {
                    error!("Location has no value");
                    return;
                }
            };
            load_from_location(&location, recipes);
        }
        None => {
            load_from_location("assets/recipe.sgs", recipes);
        }
    }
}

/// This loads a file from the given location and decodes the recipes and sets the recipes variable to the decoded recipes, printing and error and returning if it fails
fn load_from_location(loc: &str, recipes: &mut HashMap<String, Recipe>) {
    let mut a = match File::open(loc) {
        Ok(file1) => file1,
        Err(e) => {
            error!("Error opening file: {}", e);
            return;
        }
    };
    let mut buffer: Vec<u8> = Vec::new();
    let _ = a.read_to_end(&mut buffer);
    let decoded_data: Result<(HashMap<String, Recipe>, usize), bincode::error::DecodeError> =
        bincode::decode_from_slice(&buffer, CFG);
    match decoded_data {
        Ok(decoded) => {
            info!("Decoded Recipes");
            info!("{:?}", decoded.0);
            *recipes = decoded.0;
        }
        Err(e) => {
            error!("Error decoding recipes: {}", e)
        }
    }
}
