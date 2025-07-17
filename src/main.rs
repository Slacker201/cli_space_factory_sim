use std::io;

use crate::{
    entities::{
        entity_components::inventory::Inventory,
        factories::{entity_base::entity_base::EntityBase, factory::Factory}, node::Node,
    },
    item_utils::{
        item::item_builder::ItemBuilder, recipe::recipe::Recipe,
        transport_order::transport_order::TransportOrder,
    },
    logging::logger::{self, LoggingLevels::*},
};

mod command_line_interface;
mod data_handling;
mod entities;
mod item_utils;
mod logging;
pub fn main() {
    logger::set_params(vec![Info(true), Warn(true), Error(true)]);
    info!("Testing");
    warn!("HELP ME");
    error!("Program died");
    println!("Enter your command. Type exit to exit program");
    let mut recipes = Vec::<Recipe>::new();
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "exit" {
            break;
        } else {
            command_line_interface::command_dispatcher::parse_and_dispatch_command(
                &input,
                &mut recipes,
            );
        }
    }

    compiler_tickles();
}

fn compiler_tickles() {

    let mut fac = Factory::new();
    let mut inv = Inventory::new();
    let mut t_order = TransportOrder::new();
    let mut rec = Recipe::new();
    let mut node = Node::new();
    let t_order2 = TransportOrder::new();
    node.add_factory(fac.clone());
    let i_b = ItemBuilder::new().set_count(1).set_id(1);
    let i_b2 = ItemBuilder::new().set_count(1).set_id(1);
    let i_b3 = ItemBuilder::new().set_count(1).set_id(1);
    let i_b4 = ItemBuilder::new().set_count(1).set_id(1);
    let i_b5 = ItemBuilder::new().set_count(1).set_id(1);
    let assembler = fac.get_assembler_mut();
    
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
