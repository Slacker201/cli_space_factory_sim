#![allow(dead_code)]

use std::collections::HashMap;

use crate::item_utils::{item::{item::Item, item_builder::ItemBuilder}, transport_order::transport_order::TransportOrder};



#[derive(Debug, PartialEq, Clone)]
/// Represents an inventory that holds items. Used by objects to manage their item collections.
pub struct Inventory {
    /// The item hashmap which stores items indexed by their unique item id
    items: HashMap<u64, Item>,
    /// The max weight capacity of the inventory. Currently unimplemented
    max_capacity: u128,
    /// Indicates whether the items have changed, used to avoid unnecessary recalculations of capacity.
    items_changed: bool,
    /// Cached total item count
    capacity: u128
}


impl Inventory {
    /// Generates a new empty `Inventory` with a maximum capacity of 100.
    pub fn new() -> Inventory {
        Inventory { items: HashMap::new(), max_capacity: 100, items_changed: true, capacity: 0}
    }
    /// Returns a reference to the items hashmap
    pub fn items(&self) -> &HashMap<u64, Item> {
        &self.items
    }
    /// Returns the max capacity
    pub fn max_capacity(&self) -> u128 {
        self.max_capacity
    }
    /// Adds an item to the hashmap, adding to the item count if the item is already in the map
    /// Currently does not respect max capacity.
    pub fn add(&mut self, item_to_add: Item) -> (bool, Option<Item>) {
        let id = item_to_add.id();
        self.items_changed = true;
        match self.items.get_mut(&id) {
            Some(item_in_inventory) => {
                item_in_inventory.set_count(item_in_inventory.count().saturating_add(item_to_add.count()));
                (true, None)
            },
            None => {
                self.items.insert(id, item_to_add);
                (false, None)
            },
        }
    }
    /// returns an optional reference to the item corresponding to the id
    pub fn get(&self, id: u64) -> Option<&Item> {
        self.items.get(&id)
    }
    /// returns a mutable option to the item corresponding to the id
    pub fn get_mut(&mut self, id: u64) -> Option<&mut Item> {
        self.items_changed = true;
        self.items.get_mut(&id)
    }
    /// Returns the capacity
    pub fn capacity(&mut self) -> u128 {
        if self.items_changed {
            let mut mass_count = 0u128;
            for item in &self.items {
                mass_count += item.1.count()
            }
            self.capacity = mass_count;
        }
        self.items_changed = false;
        self.capacity
    }
    /// Removes an item from the inventory and returns it if found
    pub fn remove(&mut self, item: &Item) -> Option<Item> {
        let found = self.get_mut(item.id())?;
        
        if found.count() > item.count() {
            found.set_count(found.count() - item.count());
            Some(item.clone())
        } else {
            let returner = found.clone();
            self.items.remove(&item.id());
            Some(returner)
        }
    }

    /// Removes an item by id, and returns it if found
    pub fn remove_by_id(&mut self, id: u64) -> Option<Item> {
        self.items.remove(&id)
    }
    /// Removes a specific amount of item, according to the provided id and count
    pub fn remove_by_id_and_count(&mut self, id: u64, count: u128) -> Option<Item> {
        match self.get_mut(id) {
            Some(var) if var.count() > count => {
                var.set_count(var.count() - count);
                Some(ItemBuilder::new().set_count(count).set_id(id).build())
            }
            Some(_) => {
                // Item count is less than or equal to requested removal count
                let removed = self.remove_by_id(id);
                removed
            }
            None => None,
        }
    }

    /// Sets the maximum capacity.
    /// No checks are conducted
    pub fn set_max_capacity(&mut self, new_capacity: u128) {
        self.max_capacity = new_capacity;
    }
    /// Returns a vector of references to all items in the inventory
    pub fn get_all(&self) -> Vec<&Item> {
        self.items.values().collect()
    }
    /// Returns a vector of mutable references to all items in the inventory
    pub fn get_all_mut(&mut self) -> Vec<&mut Item> {
        self.items.values_mut().collect()
    }

    /// Moves items from itself into another inventory according to a transport order
    pub fn move_items_to(&mut self, t_order: TransportOrder, tar_inv: &mut Inventory) {

        for item in t_order.items() {

            // Check if the source inventory contains the item
            match self.get_mut(item.id()) {
                Some(_) => {
                    let item_id = item.id();
                    let item_count = item.count();

                    // Attempt to remove the specified count of the item from source inventory
                    match self.remove_by_id_and_count(item_id, item_count) {
                        Some(removed_item) => {
                            tar_inv.add(removed_item);
                        }
                        None => {
                        }
                    }
                }
                None => {
                    continue;
                }
            }
        }
    }

    /// Adds multiple items to the inventory
    pub fn add_multiple(&mut self, items: Vec<Item>) {
        for item in items {
            self.add(item);
        }
    }
    
    /// Returns true if the inventory is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    /// Clears the inventory of all items
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

