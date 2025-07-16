use std::collections::HashMap;

use crate::{
    command_line_interface::{
        argument_flag::ArgumentFlag,
        command_struct::Command,
        commands::{
            add_recipe::add_recipe_cmd, load_recipes_from_file::load_recipes_cmd,
            save_recipes_to_file::save_recipes_cmd, view_recipe::view_recipes_cmd,
        },
    },
    item_utils::recipe::recipe::Recipe,
};

/// This splits the command into tokens and runs them through a parser before dispatching the command
pub fn parse_and_dispatch_command(comd: &str, recipes: &mut Vec<Recipe>) {
    let mut command = Command::new();
    let cmd = comd.to_lowercase();

    let parts: Vec<&str> = cmd.split_whitespace().collect();

    match parts.first() {
        Some(name) => {
            command.set_name(name.to_string());

            let args = &parts[1..];

            let arg_map = parse_multiparam(args);
            println!("{arg_map:?}");
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
        _ => {
            println!("Unknown command3")
        }
    }
}
/// This takes a &str array and returns a hashmap of (argument names, argument flag vectors)
pub(crate) fn parse_multiparam(args: &[&str]) -> HashMap<String, Vec<ArgumentFlag>> {
    println!("printing");
    println!("{args:?}");
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
                match current_arg.strip_prefix("--") {
                    Some(found) => {
                        println!("New arg is \"{}\"", found);
                        arg_map.insert(found.to_string(), vec![ArgumentFlag::BooleanTrue]);
                        continue;
                    }
                    None => {
                        println!("Did not add argument bcuz it was None when prefix was stripped");
                        continue;
                    }
                }
            }
            // There is room for the argument
            let arg_value = args[i + 1];
            match args[i].strip_prefix("--") {
                Some(argument_name) => {
                    if argument_name.is_empty() {
                        println!("Argument name was empty");
                        i += 1;
                        continue;
                    }
                    if arg_value.starts_with("--") {
                        println!(
                            "{argument_name}'s value started with \"--\", so we are treating it as a boolean flag"
                        );
                        i += 1;
                        println!("Stripping prefix");
                        println!("{}", argument_name);
                        if !argument_name.starts_with("--") {
                            println!("New arg is \"{}\"", argument_name);
                            arg_map
                                .insert(argument_name.to_string(), vec![ArgumentFlag::BooleanTrue]);
                            continue;
                        } else {
                            match argument_name.strip_prefix("--") {
                                Some(found) => {
                                    println!("New arg is \"{}\"", found);
                                    arg_map
                                        .insert(found.to_string(), vec![ArgumentFlag::BooleanTrue]);
                                    continue;
                                }
                                None => {
                                    println!(
                                        "Did not add argument bcuz it was None when prefix was stripped"
                                    );
                                    continue;
                                }
                            }
                        }
                    }
                    println!("The argument name contained \"--\"");
                    println!("Adding {arg_value} to {argument_name}");
                    match arg_map.get_mut(argument_name) {
                        Some(mutt) => {
                            println!(
                                "The arg map contained the argument name so we add the new value to the end"
                            );
                            mutt.push(ArgumentFlag::Value(arg_value.to_string()));
                        }
                        None => {
                            println!("Arg map does not contain {argument_name}");
                            arg_map.insert(
                                argument_name.to_string(),
                                vec![ArgumentFlag::Value(arg_value.to_string())],
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
