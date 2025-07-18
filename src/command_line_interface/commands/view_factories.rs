use crate::{command_line_interface::command_struct::Command, entities::world::World};



pub fn view_factories_cmd (_cmd: Command, world: &World) {
    for fac in world.node().factories().values() {
        println!("{:?}", fac);
    }
}