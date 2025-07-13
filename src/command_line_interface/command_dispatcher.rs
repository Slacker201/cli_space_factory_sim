use std::collections::HashMap;

use crate::{
    command_line_interface::{
        command_struct::Command,
        commands::{
            add_recipe::add_recipe_cmd,
            load_recipes_from_file::load_recipes_cmd,
            save_recipes_to_file::save_recipes_cmd,
            view_recipe::view_recipes_cmd,
        },
    },
    item_utils::recipe::recipe::Recipe,
};

/// This splits the command into tokens and runs them through a parser before dispatching the command
pub fn parse_and_dispatch_command(comd: &str, recipes: &mut Vec<Recipe>) {
    let mut command = Command::new();
    let cmd = comd.to_lowercase();
    if cmd.contains("\x01true") {
        println!("Invalid Characters");
        return;
    }
    let parts: Vec<&str> = cmd.split_whitespace().collect();

    match parts.get(0) {
        Some(name) => {
            command.set_name(name.to_string());

            let args = &parts[1..];

            let arg_map = parse_multiparam(args);
            println!("{:?}", arg_map);
            command.set_args(arg_map);
            dispatch_command(command, recipes);
        }
        None => {
            println!("Unknown Command2");
        }
    }
}

/// This uses a switch statement on the command name to run a command
fn dispatch_command(cmd: Command, recipes: &mut Vec<Recipe>) {
    println!("Dispatching command");
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
        _ => { println!("Unknown command3") }
    }
}
/// This takes a &str array and returns a hashmap of (argument names, argument flag vectors)
fn parse_multiparam(args: &[&str]) -> HashMap<String, Vec<ArgumentFlag>> {
    println!("printing");
    println!("{:?}", args);
    let mut arg_map: HashMap<String, Vec<ArgumentFlag>> = HashMap::new();
    let mut i = 0;
    let args_len = args.len();
    while i < args_len {
        let current_arg = args[i];
        if current_arg.starts_with("--") {
            // now we take it and the next word, and skip the next word.
            if i + 1 >= args_len {
                // there is no room for a value for the argument
                println!(
                    "Invalid argument, no value for argument {} so we are treating it as a boolean flag",
                    args[i]
                );
                i += 1;
                arg_map.insert(current_arg.to_string(), vec![ArgumentFlag::BooleanTrue]);

                continue;
            }
            // There is room for the argument
            let arg_value = args[i + 1];
            match args[i].strip_prefix("--") {
                Some(argument_name) => {
                    if argument_name == "" {
                        println!("Argument name was empty");
                        i += 1;
                        continue;
                    }
                    if arg_value.starts_with("--") {
                        println!("{}'s value started with \"--\", so we are treating it as a boolean flag", argument_name);
                        i += 1;
                        arg_map.insert(argument_name.to_string(), vec![ArgumentFlag::BooleanTrue]);
                        continue;
                    }
                    println!("The argument name contained \"--\"");
                    println!("Adding {} to {argument_name}", arg_value);
                    match arg_map.get_mut(argument_name) {
                        Some(mutt) => {
                            println!(
                                "The arg map contained the argument name so we add the new value to the end"
                            );
                            mutt.push(ArgumentFlag::Value(arg_value.to_string()));
                        }
                        None => {
                            println!("Arg map does not contain {}", argument_name);
                            arg_map.insert(
                                argument_name.to_string(),
                                vec![ArgumentFlag::Value(arg_value.to_string())]
                            );
                        }
                    }
                }
                None => {
                    println!("The argument name did not contain \"--\"");
                }
            }
            i += 2;
        } else {
            // otherwise we call invalid argument and skip it
            println!("Invalid Argument");
            i += 1;
        }
    }
    println!("Finished parsing");
    arg_map
}

#[derive(Debug)]
/// This is a simple type for command line arguments
pub enum ArgumentFlag {
    BooleanTrue,
    Value(String),
}
/// Impl block for ArgumentFlag
impl ArgumentFlag {
    /// Returns the string value of the argument. Returns True if the flag is BooleanTrue and the string value if it is Value
    pub fn to_string(&self) -> String {
        match self {
            ArgumentFlag::BooleanTrue => String::from("True"),
            ArgumentFlag::Value(val) => val.clone(),
        }
    }
    /// Returns the string value of the argument. Returns True if the flag is BooleanTrue and the string value if it is Value. Consumes the argument
    pub fn to_string_consume(self) -> String {
        match self {
            ArgumentFlag::BooleanTrue => String::from("True"),
            ArgumentFlag::Value(val) => val,
        }
    }
}
