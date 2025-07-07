#![allow(dead_code)]

use std::collections::HashMap;

use crate::item_utils::{item::item::Item, transport_order::transport_order::TransportOrder};



#[derive(Debug, PartialEq)]
pub struct Inventory {
    items: HashMap<u64, Item>,
    max_capacity: u64,
    items_changed: bool,
    capacity: u128
}


impl Inventory {
    pub fn new() -> Inventory {
        Inventory { items: HashMap::new(), max_capacity: 100, items_changed: true, capacity: 0}
    }
    pub fn items(&self) -> &HashMap<u64, Item> {
        &self.items
    }
    pub fn max_capacity(&self) -> u64 {
        self.max_capacity
    }
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

    pub fn get(&self, id: u64) -> Option<&Item> {
        self.items.get(&id)
    }
    pub fn get_mut(&mut self, id: u64) -> Option<&mut Item> {
        self.items_changed = true;
        self.items.get_mut(&id)
    }

    pub fn capacity(&mut self) -> u128 {
        if self.items_changed {
            let mut mass_count = 0u128;
            for item in &self.items {
                mass_count += item.1.count();
            }
            self.capacity = mass_count;
        }
        self.items_changed = false;
        self.capacity
    }

    pub fn remove(&mut self, item: &Item) -> Option<Item> {
        self.items.remove(&item.id())
    }
    pub fn remove_by_id(&mut self, id: u64) -> Option<Item> {
        self.items.remove(&id)
    }
    pub fn set_max_capacity(&mut self, new_capacity: u64) {
        self.max_capacity = new_capacity;
    }

    pub fn get_all(&self) -> Vec<&Item> {
        self.items.values().collect()
    }

    pub fn move_items_to(&mut self, t_order: TransportOrder, tar_inv: &mut Inventory) {
        println!("Currently adding these items: {:?}", t_order.items());
        for item in t_order.items() {
            println!("Adding item: {:?}", item);
            let item_id;
            if let Some(src_item) = self.get_mut(item.id()) {
                item_id = src_item.id();
                println!("Adding item1")
            } else {
                println!("Source inventory does not contain {}", item.name().unwrap_or_else(|| "Invalid Item"));
                continue
            }
            if let Some(removed) = self.remove_by_id(item_id) {
                println!("5 Adding item: {:?}", removed);
                tar_inv.add(removed);
                println!("Tar inv: {:?}", tar_inv)
            } else {
                println!("Failed to add item")
            }
        }
    }

    pub fn add_multiple(&mut self, items: Vec<Item>) {
        for item in items {
            self.add(item);
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

}

