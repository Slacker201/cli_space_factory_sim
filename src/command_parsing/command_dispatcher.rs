use std::{collections::HashMap, fmt::Display};

use thiserror::Error;

use crate::{
    command_parsing::{
        command_struct::Command,
        command_token::CommandToken,
        commands::{
            add_factory::add_factory_cmd, add_recipe::add_recipe_cmd,
            load_recipes_from_file::load_recipes_cmd, remove_factory::remove_factory_cmd,
            save_recipes_to_file::save_recipes_cmd, view_factories::view_factories_cmd,
            view_factory::view_factory_cmd, view_recipe::view_recipes_cmd,
        },
    },
    entities::world::World,
    error, info,
};

/// This splits the command into tokens and runs them through a parser before dispatching the command
pub fn parse_and_dispatch_command(comd: &str, world: &mut World) -> Result<(), CommandParseError> {
    let cmd = comd.to_lowercase();

    let parts: Vec<&str> = cmd.split_whitespace().collect();

    let arg_map = parse(parts)?;
    info!("{arg_map:?}");
    dispatch_command(arg_map, world);

    Ok(())
}

/// This uses a switch statement on the command name to run a command
fn dispatch_command(cmd: Command, world: &mut World) {
    info!("Dispatching command");
    match cmd.name().to_lowercase().as_str() {
        "add_recipe" => {
            info!("{:?}", add_recipe_cmd(cmd, world.all_recipes_mut()));
        }
        "view_recipes" => {
            view_recipes_cmd(cmd, world.all_recipes_mut());
        }
        "load_recipes" => {
            load_recipes_cmd(cmd, world.all_recipes_mut());
        }
        "save_recipes" => {
            save_recipes_cmd(cmd, world.all_recipes_mut());
        }
        "add_factory" => {
            info!("{:?}", add_factory_cmd(cmd, world));
        }
        "view_factories" => {
            view_factories_cmd(cmd, world);
        }
        "remove_factory" => {
            info!("{:?}", remove_factory_cmd(cmd, world));
        }
        "view_factory" => {
            view_factory_cmd(cmd, world);
        }
        _ => {
            error!("Unknown Command on command dispatch")
        }
    }
}
pub fn parse(mut cmd_tokens: Vec<&str>) -> Result<Command, CommandParseError> {
    // Get the command name
    // Parse out arguments
    let mut cmd = Command::new();
    match cmd_tokens.first() {
        Some(name) => {
            cmd.set_name(name.to_string());
        }
        None => return Err(CommandParseError::EmptyTokenArray),
    }
    let mut tracked_flag: Option<String> = None;
    let mut hash_map: HashMap<String, Vec<CommandToken>> = HashMap::new();
    for token in &mut cmd_tokens[1..] {
        if token.starts_with("--") {
            tracked_flag = token.strip_prefix("--").map(|s| s.to_string());
            match tracked_flag.as_ref() {
                Some(flag) => {
                    hash_map.entry(flag.to_string()).or_insert_with(Vec::new);
                }
                None => {
                    println!("flag was gone");
                }
            }
        } else if token.contains("=") {
            println!("Found = arg");
            let parts: Vec<&str> = token.split("=").collect();
            if parts.len() != 2 {
                println!("Invalid equal count idfk");
                continue;
            }
            let p1 = parts[0].to_string();
            hash_map
                .entry(p1)
                .or_insert_with(Vec::new)
                .push(CommandToken::Value(parts[1].to_string()));
            println!("Added {} to {}", parts[0], parts[1])
        } else {
            match tracked_flag.as_ref() {
                Some(arg) => {
                    println!("Got arg {} for {}", token, arg);
                    hash_map
                        .entry(arg.clone())
                        .or_insert_with(Vec::new)
                        .push(CommandToken::Value(token.to_string()));
                }
                None => {
                    println!("no flag to tie arg {} to", token)
                }
            }
        }
    }
    for item in &mut hash_map {
        if item.1.is_empty() {
            item.1.push(CommandToken::BooleanTrue);
        }
    }

    cmd.set_args(hash_map);
    Ok(cmd)
}

#[derive(Error, Debug, PartialEq)]
pub enum CommandParseError {
    EmptyTokenArray,
}

impl Display for CommandParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandParseError::EmptyTokenArray => {
                write!(f, "Empty Token Array")
            }
        }
    }
}
