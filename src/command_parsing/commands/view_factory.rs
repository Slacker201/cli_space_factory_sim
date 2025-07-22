use crate::{
    command_parsing::{command_struct::Command, commands::command_utils::get_single_arg},
    entities::world::World,
    error, warn,
};

/// Prints a specified factory or nothing if the factory doesn't exist.
///
/// # Arguments
/// * cmd - The command object constructed by the parser
/// * world - A reference to the global world object
///
pub fn view_factory_cmd(cmd: Command, world: &World) {
    let id: u64 = match get_single_arg("name", &cmd) {
        Some(name) => match world.node().name_to_id(&name) {
            Some(le_new_id) => le_new_id,
            None => {
                error!("Factory not found");
                return;
            }
        },
        None => {
            warn!("Could not find name argument, trying id argument");
            match try_get_id(&cmd) {
                Some(id) => id,
                None => {
                    error!("Could not find id argument.");
                    return;
                }
            }
        }
    };

    match world.node().get_factory(id) {
        Some(fac) => {
            println!("{:?}", fac)
        }
        None => {
            error!("Factory not found")
        }
    }
}

/// Tries to the the id field. A simple helper function to reduce bloat
fn try_get_id(cmd: &Command) -> Option<u64> {
    match get_single_arg("id", &cmd) {
        Some(val) => match val.parse() {
            Ok(id) => Some(id),
            Err(err) => {
                error!("{err}");
                warn!("Could not parse {} into id", val);
                None
            }
        },
        None => None,
    }
}
