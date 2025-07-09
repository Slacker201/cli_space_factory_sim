use crate::item_utils::item::item::Item;


#[derive(Debug)]
/// A transport order representing a list of items and a boolean
pub struct TransportOrder {
    /// The items to move
    items: Vec<Item>,
    /// Whether to fill up the inventory completely or cancel the trade if there is not enough room
    saturate_inv: bool
}


impl TransportOrder {
    /// Generates a transport order with no items and saturate inventory set to true
    pub fn new() -> TransportOrder {
        TransportOrder { items: Vec::new(), saturate_inv: true }
    }
    /// Returns a reference to the item vector
    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }
    /// Returns a mutable reference to the item vector
    pub fn items_mut(&mut self) -> &mut Vec<Item> {
        &mut self.items
    }
    /// Returns whether the order should saturate the inventory on overflow
    pub fn saturate_inv(&self) -> bool {
        self.saturate_inv
    }
    /// Setter for the item vector. Requires passing ownership to the transport order
    pub fn set_items(&mut self, new_items: Vec<Item>) {
        self.items = new_items;
    }
    /// Setter for the item vector. This does not require passing ownership, but requires a higher computation cost
    pub fn set_items_from_refs(&mut self, new_items: Vec<&Item>) {
        let mut arr = Vec::new();
        for item in new_items {
            arr.push(item.clone());
        }
        self.items = arr;
    }
    pub fn set_saturate_inv(&mut self, new_saturate_inv: bool) {
        self.saturate_inv = new_saturate_inv;
    }
}

/// Implimentation block for default
impl Default for TransportOrder {
    /// Generates a transport order with no items and saturate inventory set to true
    fn default() -> Self {
        TransportOrder { items: Vec::default(), saturate_inv: true }
    }
}