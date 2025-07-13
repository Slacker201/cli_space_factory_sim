use std::collections::HashMap;

use crate::command_line_interface::command_dispatcher::ArgumentFlag;




pub struct Command {
    name: String,
    args: HashMap<String, Vec<ArgumentFlag>>
}



impl Command {
    pub fn new() -> Command {
        Command { name: String::from("error"), args: HashMap::new() }
    }
    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name
    }
    pub fn name(&self) -> String {
        self.name.to_string()
    }
    pub fn set_args(&mut self, args: HashMap<String, Vec<ArgumentFlag>>) {
        self.args = args;
    }
    pub fn args(&self) -> &HashMap<String, Vec<ArgumentFlag>> {
        &self.args
    }
}