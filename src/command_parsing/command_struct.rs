use std::collections::HashMap;

use crate::command_parsing::command_token::CommandToken;

/// Represents a command, holds a command name and a hashmap of arguments
#[derive(Debug)]
pub struct Command {
    name: String,
    args: HashMap<String, Vec<CommandToken>>,
}

/// Impl block for Command. Contains setters, getters and new()
impl Command {
    pub fn new() -> Command {
        Command {
            name: String::from(""),
            args: HashMap::new(),
        }
    }
    /// Setter for name
    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }
    /// Returns the name
    pub fn name(&self) -> String {
        self.name.to_string()
    }
    /// Setter for args
    pub fn set_args(&mut self, args: HashMap<String, Vec<CommandToken>>) {
        self.args = args;
    }
    /// Returns the args
    pub fn args(&self) -> &HashMap<String, Vec<CommandToken>> {
        &self.args
    }
}
