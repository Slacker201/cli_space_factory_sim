use rand::random;

use crate::entities::node::node_error::NodeFactoryAddError::DuplicateName;
use crate::{
    command_parsing::{
        command_struct::Command,
        commands::{command_error::CommandError, command_utils::get_single_arg},
    },
    entities::{factories::factory::Factory, world::World},
    error, info, warn,
};
/// Adds a new factory to the node
///
/// # Arguments
/// * cmd - The command object constructed by the parser
/// * world - A reference to the global world object
///
pub fn add_factory_cmd(cmd: Command, world: &mut World) -> Result<(), CommandError> {
    // Get name argument value. Make new factory with random id
    let name = match get_single_arg("name", &cmd) {
        Some(name) => {
            if world.node().contains_factory_with_name(&name) {
                warn!("Name already used");
                return Err(CommandError::CommandNodeFactoryAddError(DuplicateName(
                    name,
                )));
            }
            name.trim().to_lowercase().to_string()
        }
        None => {
            error!("Name not provided. Using random id");
            "".to_string()
        }
    };

    let mut factory = Factory::new();
    factory.set_name(name);
    let rand_num: u64 = match get_single_arg("id", &cmd) {
        Some(id) => match id.parse() {
            Ok(parsed_id) => parsed_id,
            Err(e) => {
                error!("{}", e);
                random()
            }
        },
        None => random(),
    };
    factory.set_id(rand_num);
    let failed_factory_add_factory = world.node_mut().add_factory(factory);
    match failed_factory_add_factory {
        Ok(_) => {
            info!("Successfully added the factory");
            Ok(())
        }
        Err(err) => {
            return Err(CommandError::CommandNodeFactoryAddError(err));
        }
    }
}
