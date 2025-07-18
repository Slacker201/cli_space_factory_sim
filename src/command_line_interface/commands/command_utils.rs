use crate::{command_line_interface::command_struct::Command, error};




/// This returns the last argument for a given argument name, or nothing if its not found
pub fn get_single_arg(argument_name: &str, cmd: &Command) -> Option<String> {
    match cmd.args().get(argument_name) {
        Some(names) => Some(names.last()?.to_string()),
        None => {
            error!("Argument {} not found", argument_name);
            None
        }
    }
}