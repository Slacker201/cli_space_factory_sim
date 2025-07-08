use crate::{entities::entity_components::inventory::inventory::Inventory, item_utils::{item::item::Item, transport_order::transport_order::TransportOrder}};


#[derive(Debug, PartialEq)]
pub struct Recipe {
    input_items: Vec<Item>,
    output_items: Vec<Item>,
    power_draw: u32,
    heat_produced: u32,
    processing_time: u32
}

impl Recipe {
    pub fn new() -> Recipe {
        Recipe { input_items: Vec::new(), output_items: Vec::new(), power_draw: 1, heat_produced: 1, processing_time: 1}
    }
    pub fn input_items(&self) -> &[Item] {
        &self.input_items
    }

    pub fn output_items(&self) -> &[Item] {
        &self.output_items
    }
    pub fn power_draw(&self) -> u32 {
        self.power_draw
    }
    pub fn heat_produced(&self) -> u32 {
        self.heat_produced
    }

    pub fn processing_time(&self) -> u32 {
        self.processing_time
    }

    pub fn set_input_items(&mut self, new_items: Vec<Item>) {
        self.input_items = new_items
    }

    pub fn set_output_items(&mut self, new_items: Vec<Item>) {
        self.output_items = new_items
    }

    pub fn set_power_draw(&mut self, new_draw: u32) {
        self.power_draw = new_draw
    }

    pub fn set_heat_produced(&mut self, new_heat_prod: u32) {
        self.heat_produced = new_heat_prod
    }

    pub fn set_processing_time(&mut self, new_processing_time: u32) {
        self.processing_time = new_processing_time
    }

    pub fn can_be_produced(&self, inv: &Inventory) -> bool {
        for recipe_item in &self.input_items {
            match inv.get(recipe_item.id()) {
                
                Some(inventory_item) => {
                    if inventory_item.count() < recipe_item.count() {
                        return false
                    }
                },
                _ => {
                    if recipe_item.count() != 0 {
                        return false
                    }
                },
            }
        }
        true
    }
    
    pub fn as_transport_order(&self) -> TransportOrder {
        let mut order = TransportOrder::new();
        for item in &self.input_items {
            if item.count() > 0 {
                order.items_mut().push(item.clone());
            }
        }
        order
    }
}