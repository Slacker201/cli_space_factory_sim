use std::fmt::Display;

use thiserror::Error;

use crate::{entities::entity_components::inventory::Inventory, item_utils::item::item::Item};

#[derive(Error, PartialEq, Debug)]
#[allow(dead_code)]
pub enum InventoryTransportError {
    /// Value is the remaining items
    InventoryFull(Inventory),
    /// Value is leftover items
    ItemAddCapacityOverflow(Item),
}

impl Display for InventoryTransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Factory was too full to add the items.")
    }
}
