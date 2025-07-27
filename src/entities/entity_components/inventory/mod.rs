pub mod tests;

use std::collections::HashMap;

use crate::{
    entities::entity_components::inventory::{
        errors::InventoryTransportError,
        strong_types::{AddItem, RemoveItem},
    },
    item_utils::{
        item::{item::Item, item_builder::ItemBuilder},
        transport_order::transport_order::TransportOrder,
    },
};

pub mod errors;
pub mod strong_types;

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
    capacity: u128,
}

impl Inventory {
    /// Generates a new empty `Inventory` with a maximum capacity of 100.
    pub fn new() -> Inventory {
        Inventory {
            items: HashMap::new(),
            max_capacity: 100,
            items_changed: true,
            capacity: 0,
        }
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
    pub fn add(&mut self, item: Item) -> Result<(), InventoryTransportError> {
        self.items_changed = true;
        let item_add2 = match self.added_items_are_too_heavy(&item) {
            PartialItemAdd::No => {
                println!("The items fit");
                item
            }
            PartialItemAdd::Yes(item) => item.0.inner(),
        };
        let entry = self.items.entry(item_add2.id()).or_insert(
            ItemBuilder::new()
                .set_id(item_add2.id())
                .set_count(0)
                .build(),
        );
        entry.set_count(entry.count() + item_add2.count());
        println!("Entry: {:?}", entry);
        Ok(())
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
        println!("{:?}", self.items());
        if self.items_changed {
            self.cal_capacity();
        }
        self.capacity
    }
    pub fn cal_capacity(&mut self) {
        let mut mass_count = 0u128;
        for item in &self.items {
            mass_count += item.1.weight();
        }
        self.capacity = mass_count;
    }
    /// Removes an item from the inventory and returns it if found
    pub fn remove(&mut self, item: &Item) -> Option<Item> {
        self.items_changed = true;
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
        self.items_changed = true;
        self.items.remove(&id)
    }
    /// Removes a specific amount of item, according to the provided id and count
    pub fn remove_by_id_and_count(&mut self, id: u64, count: u128) -> Option<Item> {
        self.items_changed = true;
        match self.get_mut(id) {
            Some(var) if var.count() > count => {
                var.set_count(var.count() - count);
                Some(ItemBuilder::new().set_count(count).set_id(id).build())
            }
            Some(_) => {
                // Item count is less than or equal to requested removal count
                self.remove_by_id(id)
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
        self.items_changed = true;
        self.items.values_mut().collect()
    }
    /// Moves items from itself into another inventory according to a transport order
    pub fn move_items_to(&mut self, t_order: TransportOrder, tar_inv: &mut Inventory) {
        self.items_changed = true;
        for item in t_order.items() {
            // Check if the source inventory contains the item
            match self.get_mut(item.id()) {
                Some(_) => {
                    let item_id = item.id();
                    let item_count = item.count();

                    // Attempt to remove the specified count of the item from source inventory
                    if let Some(removed_item) = self.remove_by_id_and_count(item_id, item_count) {
                        let _ = tar_inv.add(removed_item);
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
            let _ = self.add(item);
        }
    }

    /// Returns true if the inventory is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    /// Clears the inventory of all items
    pub fn clear(&mut self) {
        self.items_changed = true;
        self.items.clear();
    }
    /// NEEDS DOCUMENTATION TODO
    fn added_items_are_too_heavy(&mut self, item: &Item) -> PartialItemAdd {
        if self.capacity() + item.weight() > self.max_capacity() {
            // Calculate how many items we need to return
            let extra_weight = (self.capacity() + item.weight()) - self.max_capacity();
            println!("How much extra weight we have: {}", extra_weight);
            let weight_to_add = self.max_capacity() - self.capacity();
            println!("Weight to add: {}", weight_to_add);
            let items_to_add = weight_to_add / item.weight_per_item();
            let items_to_return = extra_weight / item.weight_per_item();
            println!("Items_to_add: {}", items_to_add);
            let item_to_add = ItemBuilder::new()
                .set_count(items_to_add)
                .set_id(item.id())
                .build();
            let item_to_return = ItemBuilder::new()
                .set_count(items_to_return)
                .set_id(item.id())
                .build();
            return PartialItemAdd::Yes((
                AddItem::Item(item_to_add),
                RemoveItem::Item(item_to_return),
            ));
        }
        PartialItemAdd::No
    }
}

enum PartialItemAdd {
    No,
    Yes((AddItem, RemoveItem)),
}
