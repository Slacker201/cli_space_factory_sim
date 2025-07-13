use std::{fs::File, io::Write};

use bincode::config;

use crate::{command_line_interface::command_struct::Command, item_utils::recipe::recipe::Recipe};




static CFG: config::Configuration = bincode::config::standard();

pub fn save_recipes_cmd(cmd: Command, recipes: &mut Vec<Recipe>) {
    match cmd.args().get("location") {
        Some(loc) => {
            let location = match loc.get(0) {
                Some(location) => match location {
                    crate::command_line_interface::command_dispatcher::ArgumentFlag::BooleanTrue => {
                        println!("Value was a boolean flag");
                        return;
                    },
                    crate::command_line_interface::command_dispatcher::ArgumentFlag::Value(var) => {
                        var
                    },
                },
                None => {
                    println!("Location has no value");
                    return;
                },
            };
            write_to_location(location, recipes);
        }
        None => {
            write_to_location("assets/recipe.sgs", recipes);
        },
    }
}


fn write_to_location(loc: &str, recipes: &Vec<Recipe>) {
    let mut target_file = match File::create(loc) {
        Ok(file) => {
            println!("Successfully created file");
            file
        },
        Err(e) => {
            println!("Failed to open file: {}", e);
            return;
        },
    };
    match bincode::encode_to_vec(recipes, CFG) {
        Ok(encoded_data) => {
            let write_to_file_error = target_file.write_all(&encoded_data);

            if write_to_file_error.is_err() {
                println!("Error writing to file: {:?}", write_to_file_error)
            }
        },
        Err(e) => {
            println!("Error encoding data: {:?}", e);
            return;
        },
    };
    

}
