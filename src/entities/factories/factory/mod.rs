

use crate::{
    entities::{
        entity_components::{assembler::assembler::Assembler, inventory::Inventory},
        factories::entity_base::entity_base::EntityBase,
    },
    item_utils::transport_order::transport_order::TransportOrder,
};
#[derive(Clone)]
pub struct Factory {
    assembler: Assembler,
    id: u64,
    name: String
}

impl Factory {
    pub fn new() -> Factory {
        Factory {
            assembler: Assembler::new(),
            id: 0,
            name: "default".to_string()
        }
    }
    pub fn move_items_from_output_to(&mut self, tar_inv: &mut Inventory, t_order: TransportOrder) {
        self.assembler
            .output_inventory_mut()
            .move_items_to(t_order, tar_inv);
    }
    pub fn move_items_from_input_to(&mut self, tar_inv: &mut Inventory, t_order: TransportOrder) {
        self.assembler
            .input_inventory_mut()
            .move_items_to(t_order, tar_inv);
    }
    pub fn get_assembler(&self) -> &Assembler {
        &self.assembler
    }
    pub fn get_assembler_mut(&mut self) -> &mut Assembler {
        &mut self.assembler
    }
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }
    pub fn set_id(&mut self, new_id: u64) {
        self.id = new_id;
    }
}

impl EntityBase for Factory {
    fn tick(&mut self) {
        self.assembler.tick();
    }
}
