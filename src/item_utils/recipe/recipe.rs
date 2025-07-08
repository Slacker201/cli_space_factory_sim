use crate::{entities::entity_components::inventory::inventory::Inventory, item_utils::{item::item::Item, transport_order::transport_order::TransportOrder}};


#[derive(Debug, PartialEq)]
/// Represents a recipe that holds input items, output items, power cost, processing time, and heat produced
pub struct Recipe {
    /// A vector containing the items required to make the recipe
    input_items: Vec<Item>,
    /// A vector containing the items outputed by the recipe
    output_items: Vec<Item>,
    /// How much power the recipe consumes per tick
    power_draw: u32,
    /// How much heat the recipe produces per tick
    heat_produced: u32,
    /// How many ticks the recipe takes to complete
    processing_time: u32
}

impl Recipe {
    /// Creates an empty recipe with a power draw, processing time, and heat production of 1
    pub fn new() -> Recipe {
        Recipe { input_items: Vec::new(), output_items: Vec::new(), power_draw: 1, heat_produced: 1, processing_time: 1}
    }
    /// Returns a vector containing references to all input items
    pub fn input_items(&self) -> Vec<&Item> {
        self.input_items.iter().collect()
    }
    /// Returns a vector containing references to all output items
    pub fn output_items(&self) -> Vec<&Item> {
        self.output_items.iter().collect()
    }
    /// Returns the power draw
    pub fn power_draw(&self) -> u32 {
        self.power_draw
    }
    /// Returns heat produced per tick
    pub fn heat_produced(&self) -> u32 {
        self.heat_produced
    }
    /// Returns processing time
    pub fn processing_time(&self) -> u32 {
        self.processing_time
    }
    /// Sets input items
    pub fn set_input_items(&mut self, new_items: Vec<Item>) {
        self.input_items = new_items
    }
    /// Sets output items
    pub fn set_output_items(&mut self, new_items: Vec<Item>) {
        self.output_items = new_items
    }
    /// Sets power draw
    pub fn set_power_draw(&mut self, new_draw: u32) {
        self.power_draw = new_draw
    }
    /// Sets heat produced
    pub fn set_heat_produced(&mut self, new_heat_prod: u32) {
        self.heat_produced = new_heat_prod
    }
    /// Sets processing time
    pub fn set_processing_time(&mut self, new_processing_time: u32) {
        self.processing_time = new_processing_time
    }
    /// Checks whether this recipe can be produced given an inventory
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
    /// Returns the output items as a transport order with saturate inventory set to true
    pub fn output_items_as_transport_order(&self) -> TransportOrder {
        let mut order = TransportOrder::new();
        for item in &self.input_items {
            if item.count() > 0 {
                order.items_mut().push(item.clone());
            }
        }
        order
    }
}