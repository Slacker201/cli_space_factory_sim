use crate::{command_line_interface::command_struct::Command, entities::world::World};


/// Prints each factory on a new line
/// 
/// # Arguments
/// * _cmd - The command object constructed by the parser. Unused
/// * world - A reference to the global world object
/// 
pub fn view_factories_cmd (_cmd: Command, world: &World) {
    for fac in world.node().factories().values() {
        println!("{:?}", fac);
    }
}