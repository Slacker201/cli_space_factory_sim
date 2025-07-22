use crate::{
    command_parsing::{
        command_struct::Command,
        commands::{command_error::CommandError, command_utils::get_single_arg},
    },
    entities::world::World,
    error, info,
};

/// Removes a factory from the world
///
/// # Arguments
/// * cmd - The command object constructed by the parser
/// * world - A reference to the global world object
///
pub fn remove_factory_cmd(cmd: Command, world: &mut World) -> Result<(), CommandError> {
    let id;
    match get_single_arg("name", &cmd) {
        Some(name) => {
            id = match world.node().name_to_id(&name) {
                Some(id) => id,
                None => {
                    error!("Factory {name} not found");
                    return Err(CommandError::MissingArgument("name".to_string()));
                }
            }
        }
        None => {
            info!("Name not found. Checking id...");
            id = match get_single_arg("id", &cmd) {
                Some(id) => match id.parse() {
                    Ok(val) => val,
                    Err(err) => return Err(CommandError::CommandParseIntError(err)),
                },
                None => return Err(CommandError::MissingArgument("id".to_string())),
            };
        }
    }
    match world.node_mut().remove_factory(id) {
        Ok(_) => Ok(()),
        Err(e) => Err(CommandError::CommandNodeFactoryRemoveError(e)),
    }
}
