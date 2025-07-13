use bincode::{Decode, Encode};

#[derive(PartialEq, Debug, Clone, Decode, Encode)]
/// Represents an item with an id and a count
pub struct Item {
    /// The id of the item
    id: u64,
    /// The quantity of the item
    count: u128,
}

/// The lookup table converting ids to names
static ITEM_NAMES: &[&str] = &["null", "Iron Ore", "Iron Ingot"];

impl Item {
    /// Generates a new item with an id of 0 and a count of 1
    pub fn new() -> Item {
        Item { id: 0, count: 1 }
    }
    /// Returns the item id
    pub fn id(&self) -> u64 {
        self.id
    }
    /// Returns the item count
    pub fn count(&self) -> u128 {
        self.count
    }
    /// Setter for the item id
    ///
    /// #parameters
    /// - 'id': The new id to set
    ///
    /// #note
    /// No checks are done on the id to ensure it is valid
    pub fn set_id(&mut self, id: u64) {
        self.id = id;
    }
    /// Setter for the item count
    ///
    /// #parameters
    /// - 'count': The new count for the item
    ///
    /// #note
    /// No checks are done on the count to ensure it is valid
    pub fn set_count(&mut self, count: u128) {
        self.count = count;
    }
    /// Getter for the item name
    pub fn name(&self) -> Option<&str> {
        ITEM_NAMES.get(self.id as usize).map(|&name| name)
    }
}
