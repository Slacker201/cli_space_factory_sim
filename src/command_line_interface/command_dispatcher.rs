use std::collections::HashMap;

use crate::command_line_interface::{command_struct::Command, commands::add_recipe::{self, add_recipe_cmd}};
use phf::phf_map;


static COMMAND_MAP: phf::Map<&'static str, &'static [&'static str]> = phf_map! {
    "add_recipe" => &["cmd_found"],
};





pub fn parse_and_dispatch_command(cmd: &str) {
    let mut command = Command::new();
    let parts: Vec<&str> = cmd.split_whitespace().collect();

    match parts.get(0) {
        Some(name) => {
            println!("Name Found");
            command.set_name(name.to_string());

            if COMMAND_MAP.get(&command.name()).is_none() {
                println!("MASSIVE ERRORS AHEAD");
                return;
            }

            println!("Command Found: {}", command.name());
            println!("Gathering arguments...");

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

            dispatch_command(command);
        }
        None => {
            println!("Invalid Command");
        }
    }
}


fn dispatch_command(cmd: Command) {
    match cmd.name().to_lowercase().as_str() {
        "add_recipe" => {
            println!("Running Add Recipe command");
            add_recipe_cmd(cmd);
        }

        _ => {
            println!("Unknown command")
        }
    }
}