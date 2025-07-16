use crate::{
    entities::{
        entity_components::{assembler::assembler::Assembler, inventory::Inventory},
        factories::entity_base::entity_base::EntityBase,
    },
    item_utils::transport_order::transport_order::TransportOrder,
};

pub struct Factory {
    assembler: Assembler,
}

impl Factory {
    pub fn new() -> Factory {
        Factory {
            assembler: Assembler::new(),
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
}

impl EntityBase for Factory {
    fn tick(&mut self) {
        self.assembler.tick();
    }
}
