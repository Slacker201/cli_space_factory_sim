use crate::item_utils::item::item::Item;


/// Item builder, it builds items.
pub struct ItemBuilder {
    item: Item
}

impl ItemBuilder {
    /// Generates a new builder
    pub fn new() -> ItemBuilder {
        ItemBuilder { item: Item::new() }
    }

    /// Sets the item id
    pub fn set_id(mut self, id: u64) -> Self {
        self.item.set_id(id);
        self
    }
    /// Sets the item count
    pub fn set_count(mut self, count: u128) -> Self {
        self.item.set_count(count);
        self
    }
    /// Consumes the builder and returns an item
    pub fn build(self) -> Item {
        self.item
    }
    

    // Test only functions
    #[cfg(test)]
    /// Test-only getter for the item id
    pub fn id(&self) -> u64 {
        self.item.id()
    }
    #[cfg(test)]
    /// Test-only getter for the item count
    pub fn count(&self) -> u128 {
        self.item.count()
    }
}

