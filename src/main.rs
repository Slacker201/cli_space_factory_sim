use std::collections::HashMap;

use crate::{
    entities::{
        entity_components::inventory::Inventory,
        factories::{entity_base::entity_base::EntityBase, factory::Factory},
        node::Node,
        world::World,
    },
    gui::SFSGui,
    item_utils::{
        item::item_builder::ItemBuilder, recipe::recipe::Recipe,
        transport_order::transport_order::TransportOrder,
    },
    logging::logger::{self, LoggingLevels::*},
};
mod data_handling;
mod entities;
mod gui;
mod item_utils;
mod logging;
pub fn main() {
    run();
}
fn run() {
    use std::sync::{Arc, RwLock};
    use std::thread;
    use std::time::Duration;

    logger::set_params(vec![Info(true), Warn(true), Error(true)]);
    info!("Testing");
    warn!("HELP ME");
    error!("Program died");

    println!("Enter your command. Type exit to exit program");

    compiler_tickles();

    let options = eframe::NativeOptions::default();
    let mut world = World::new();

    let mut fac = Factory::new();
    fac.set_id(16);
    fac.set_name("Le Bobbert".to_owned());

    let mut recipe = Recipe::new();
    let mut vec = Vec::new();
    for i in 0..200 {
        vec.push(ItemBuilder::new().set_count(5).set_id(i).build());
    }
    recipe.set_input_items(vec);

    let _ = fac
        .get_assembler_mut()
        .input_inventory_mut()
        .add(ItemBuilder::new().set_count(5).set_id(1).build());
    let _ = fac
        .get_assembler_mut()
        .input_inventory_mut()
        .add(ItemBuilder::new().set_count(5).set_id(2).build());
    let _ = fac
        .get_assembler_mut()
        .output_inventory_mut()
        .add(ItemBuilder::new().set_count(5).set_id(1).build());
    let _ = fac
        .get_assembler_mut()
        .output_inventory_mut()
        .add(ItemBuilder::new().set_count(5).set_id(2).build());
    fac.get_assembler_mut().set_recipe(recipe);

    let mut map = HashMap::new();
    let mut node1 = Node::new();
    let _ = node1.add_factory(Factory::new());
    let _ = node1.add_factory(fac.clone());
    map.insert(node1.id(), node1);

    let mut node2 = Node::new();
    node2.set_id(1);
    let _ = node2.add_factory(Factory::new());
    let _ = node2.add_factory(fac);
    map.insert(node2.id(), node2);

    world.set_nodes(map);

    let shared_world = Arc::new(RwLock::new(world));
    

    {
        let world = Arc::clone(&shared_world);
        thread::spawn(move || {
            loop {
                {
                    let mut world = world.write().unwrap();
                    // Game simulation logic here
                    println!("Ticking");
                    world.tick();
                }
                thread::sleep(Duration::from_millis(16));
            }
        });
    }

    let app = SFSGui::new_with_world(shared_world);
    let _ = eframe::run_native(
        "Space Logistical Simulator",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    );
}

fn compiler_tickles() {
    let world = World::new();
    let mut fac = Factory::new();
    let mut inv = Inventory::new();
    let mut t_order = TransportOrder::new();
    let mut rec = Recipe::new();
    let mut node = Node::new();
    let t_order2 = TransportOrder::new();
    let _ = node.add_factory(fac.clone());
    let i_b = ItemBuilder::new().set_count(1).set_id(1);
    let i_b2 = ItemBuilder::new().set_count(1).set_id(1);
    let i_b3 = ItemBuilder::new().set_count(1).set_id(1);
    let i_b4 = ItemBuilder::new().set_count(1).set_id(1);
    let i_b5 = ItemBuilder::new().set_count(1).set_id(1);
    let assembler = fac.get_assembler_mut();
    world.nodes();
    node.factories();
    node.factories_mut();
    node.clear_factories();
    node.contains_factory(1);
    inv.max_capacity();
    inv.items();
    inv.remove(&i_b.clone().build());
    inv.capacity();
    inv.set_max_capacity(20);
    inv.get_all();
    inv.get_all_mut();
    inv.is_empty();
    i_b5.build().name();
    t_order.saturate_inv();
    t_order.set_saturate_inv(true);
    t_order.set_items(Vec::from([i_b.build()]));
    t_order.set_items_from_refs(Vec::from([&i_b2.build()]));
    rec.can_be_produced(&inv);
    rec.heat_produced();
    rec.input_items();
    rec.input_items_as_transport_order();
    rec.output_items();
    rec.output_items_as_transport_order();
    rec.power_draw();
    rec.power_draw();
    rec.processing_time();
    rec.set_heat_produced(1);
    rec.set_input_items(Vec::from([i_b3.build()]));
    rec.set_output_items(Vec::from([i_b4.build()]));
    rec.set_power_draw(1);
    rec.set_processing_time(1);
    rec.name();
    rec.set_name(String::from("Hey_Bob"));
    assembler.input_inventory();
    assembler.input_inventory_mut();
    assembler.output_inventory();
    assembler.output_inventory_mut();
    assembler.processing_inventory();
    assembler.processing_inventory_mut();
    assembler.processing_state();
    assembler.set_processing_inventory(inv.clone());
    assembler.recipe();
    assembler.recipe_mut();
    assembler.set_input_inventory(inv.clone());
    assembler.set_output_inventory(inv.clone());
    assembler.set_recipe(rec);
    fac.get_assembler();
    fac.move_items_from_input_to(&mut inv, t_order);
    fac.move_items_from_output_to(&mut inv, t_order2);
    fac.tick();
}
