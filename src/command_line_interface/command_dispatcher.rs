use std::collections::HashMap;

use crate::{command_line_interface::{command_struct::Command, commands::{add_recipe::add_recipe_cmd, load_recipes_from_file::load_recipes_cmd, save_recipes_to_file::save_recipes_cmd, view_recipe::view_recipes_cmd}}, item_utils::recipe::recipe::Recipe};







pub fn parse_and_dispatch_command(cmd: &str, recipes: &mut Vec<Recipe>) {
    let mut command = Command::new();
    let parts: Vec<&str> = cmd.split_whitespace().collect();

    match parts.get(0) {
        Some(name) => {
            command.set_name(name.to_string());

            let mut arg_map: HashMap<String, String> = HashMap::new();
            let args = &parts[1..];
            let mut i = 0;

            while i < args.len() {
                if args[i].starts_with("--") {
                    let key = args[i].strip_prefix("--").unwrap();

                    if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                        let value = args[i + 1];
                        arg_map.insert(key.to_string(), value.to_string());
                        i += 2;
                    } else {
                        arg_map.insert(key.to_string(), "".to_string());
                        println!("Added flag {} with no value", key);
                        i += 1;
                    }
                } else {
                    println!("Word did not have \"--\" in front of it: {}", args[i]);
                    i += 1;
                }
            }

            // Here you can use arg_map for the command logic
            command.set_args(arg_map);

            dispatch_command(command, recipes);
        }
        None => {
            println!("Unknown Command2");
        }
    }
}


fn dispatch_command(cmd: Command, recipes: &mut Vec<Recipe>) {
    match cmd.name().to_lowercase().as_str() {
        "add_recipe" => {
            add_recipe_cmd(cmd, recipes);
        }
        "view_recipes" => {
            view_recipes_cmd(cmd, recipes);
        }
        "load_recipes" => {
            load_recipes_cmd(cmd, recipes);
        }
        "save_recipes" => {
            save_recipes_cmd(cmd, recipes);
        }
        _ => {
            println!("Unknown command3")
        }
    }
}