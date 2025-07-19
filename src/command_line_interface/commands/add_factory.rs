use rand::random;

use crate::{command_line_interface::{command_struct::Command, commands::command_utils::get_single_arg}, entities::{factories::factory::Factory, world::World}, error, info, warn};



/// Adds a new factory to the node
/// 
/// # Arguments
/// * cmd - The command object constructed by the parser
/// * world - A reference to the global world object
/// 
pub fn add_factory_cmd(cmd: Command, world: &mut World) {
    // Get name argument value. Make new factory with random id
    let name = match get_single_arg("name", &cmd) {
        Some(name) => name.trim().to_lowercase().to_string(),
        None => {
            error!("Name not provided. Using random id");
            "default".to_string()
        },
    };
    if world.node().contains_factory_with_name(&name) {
        warn!("Name already used");
        return;
    }
    let mut factory = Factory::new();
    factory.set_name(name);
    let rand_num: u64 = random();
    factory.set_id(rand_num);
    let failed_factory_add_factory = world.node_mut().add_factory(factory);
    match failed_factory_add_factory {
        Ok(_) => {
            info!("Successfully added the factory")
        },
        Err(err) => {
            match err {
                crate::entities::node::NodeFactoryAddError::FactoryAddDuplicateId(_) => {
                    error!("The factory's id was taken")
                },
                crate::entities::node::NodeFactoryAddError::FactoryAddDuplicateName(_) => {
                    error!("The factory's name was taken")
                },
                crate::entities::node::NodeFactoryAddError::LimitReached(_) => {
                    error!("The node contained too many factories")
                },
            }
        },
    }
}