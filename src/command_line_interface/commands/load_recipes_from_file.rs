
use std::{fs::File, io::Read};

use bincode::config::{self};

use crate::{command_line_interface::command_struct::Command, item_utils::recipe::recipe::Recipe};



static CFG: config::Configuration = bincode::config::standard();

pub fn load_recipes_cmd(cmd: Command, recipes: &mut Vec<Recipe>) {
    match cmd.args().get("location") {
        Some(loc) => {
            load_from_location(loc, recipes);
        }
        None => {
            load_from_location("assets/recipe.sgs", recipes);
        },
    }
}


fn load_from_location(loc: &str, recipes: &mut Vec<Recipe>) {
    let mut a = match File::open(loc) {
        Ok(file1) => file1,
        Err(e) => {
            println!("Error opening file: {}", e);
            return;
        },
    };
    let mut buffer: Vec<u8> = Vec::new();
    let _ = a.read_to_end(&mut buffer);
    let decoded_data: Result<(Vec<Recipe>, usize), bincode::error::DecodeError> = bincode::decode_from_slice(&buffer, CFG);
    match decoded_data {
        Ok(decoded) => {
            println!("Decoded Recipes");
            *recipes = decoded.0;
        },
        Err(e) => {
            println!("Error decoding recipes: {}", e)
        },
    }
}
