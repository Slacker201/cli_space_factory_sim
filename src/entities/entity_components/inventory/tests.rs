#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        entities::entity_components::inventory::Inventory,
        item_utils::item::{item::Item, item_builder::ItemBuilder},
    };

    #[test]
    fn default_values() {
        let inv = Inventory::new();
        assert_eq!(inv.max_capacity(), 100);
        assert_eq!(inv.items(), &HashMap::<u64, Item>::new())
    }

    #[test]
    fn set_max_capacity() {
        let mut inv = Inventory::new();
        inv.set_max_capacity(5);
        assert_eq!(inv.max_capacity(), 5)
    }

    #[test]
    fn add_item_add() {
        let mut inv = Inventory::new();
        assert_eq!(
            false,
            inv.add(ItemBuilder::new().set_count(1).set_id(1).build()).0
        );
    }

    #[test]
    fn add_item_merge() {
        let mut inv = Inventory::new();
        inv.add(ItemBuilder::new().set_count(1).set_id(1).build());
        assert_eq!(
            true,
            inv.add(ItemBuilder::new().set_count(1).set_id(1).build()).0
        );
    }

    #[test]
    fn add_item_correct_values() {
        let mut inv = Inventory::new();
        inv.add(ItemBuilder::new().set_count(1).set_id(1).build());

        let item = inv.get(1);
        assert!(item.is_some(), "Item not found");

        let item = item.unwrap();
        assert_eq!(item.count(), 1);
        assert_eq!(item.id(), 1);
    }

    #[test]
    fn get_mut_mutability() {
        let mut inv = Inventory::new();
        inv.add(ItemBuilder::new().set_count(1).set_id(1).build());

        let item = inv.get_mut(1).unwrap();

        item.set_count(5);

        let item2 = inv.get(1).unwrap();

        assert_eq!(item2.count(), 5)
    }

    #[test]
    fn add_item_capacity() {
        let mut inv = Inventory::new();
        inv.add(ItemBuilder::new().set_count(1).set_id(1).build());

        assert_eq!(inv.capacity(), 1);

        inv.add(ItemBuilder::new().set_count(5).set_id(2).build());

        assert_eq!(inv.capacity(), 6);
    }

    #[test]
    fn clear_inventory() {
        let mut inv = Inventory::new();
        inv.add(ItemBuilder::new().set_count(5).set_id(1).build());
        inv.clear();
        assert!(inv.items().is_empty())
    }
}
