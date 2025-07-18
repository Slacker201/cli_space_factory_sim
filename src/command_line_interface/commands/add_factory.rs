use rand::random;

use crate::{command_line_interface::command_struct::Command, entities::{factories::factory::Factory, world::World}, error, info, warn};




pub fn add_factory_cmd(cmd: Command, world: &mut World) {
    // Get name argument value. Make new factory with random id
    let name = match get_single_arg("name", &cmd) {
        Some(name) => name.trim().to_lowercase().to_string(),
        None => {
            error!("Name not provided. Using random id");
            "default".to_string()
        },
    };
    for fac in world.node().factories().values() {
        if fac.name() == &name {
            warn!("Name already used");
            return;
        }
    }
    let mut factory = Factory::new();
    factory.set_name(name);
    let rand_num: u64 = random();
    factory.set_id(rand_num);
    let failed_factory_add_factory = world.node_mut().add_factory(factory);
    match failed_factory_add_factory {
        Some(_fac) => {
            info!("Node was too full");
        }
        None => {
            info!("Successfully added factory")
        }
    }
}


/// This returns the last argument for a given argument name, or nothing if its not found
fn get_single_arg(argument_name: &str, cmd: &Command) -> Option<String> {
    match cmd.args().get(argument_name) {
        Some(names) => Some(names.last()?.to_string()),
        None => {
            error!("Argument {} not found", argument_name);
            None
        }
    }
}