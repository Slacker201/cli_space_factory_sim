#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        entities::entity_components::inventory::{Inventory, errors::InventoryTransportError},
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
            Ok(()),
            inv.add(ItemBuilder::new().set_count(1).set_id(1).build())
        );
    }

    #[test]
    fn add_item_correct_values() {
        let mut inv = Inventory::new();
        let _ = inv.add(ItemBuilder::new().set_count(1).set_id(1).build());

        let item = inv.get(1);
        assert!(item.is_some(), "Item not found");

        let item = item.unwrap();
        assert_eq!(item.count(), 1);
        assert_eq!(item.id(), 1);
    }

    #[test]
    fn get_mut_mutability() {
        let mut inv = Inventory::new();
        let _ = inv.add(ItemBuilder::new().set_count(1).set_id(1).build());

        let item = inv.get_mut(1).unwrap();

        item.set_count(5);

        let item2 = inv.get(1).unwrap();

        assert_eq!(item2.count(), 5)
    }

    #[test]
    fn add_item_capacity() {
        let mut inv = Inventory::new();
        let _ = inv.add(ItemBuilder::new().set_count(1).set_id(1).build());

        assert_eq!(inv.capacity(), 1);

        let _ = inv.add(ItemBuilder::new().set_count(5).set_id(2).build());

        assert_eq!(inv.capacity(), 6);
    }

    #[test]
    fn clear_inventory() {
        // Arrange
        let mut inv = Inventory::new();

        // Act
        let _ = inv.add(ItemBuilder::new().set_count(5).set_id(1).build());
        inv.clear();

        // Assert
        assert!(inv.items().is_empty())
    }
    #[test]
    fn add_returns_ok_when_added_item_is_below_max_capacity() {
        // Arrange
        let mut inv = Inventory::new();

        // Act
        inv.set_max_capacity(20);
        let item = ItemBuilder::new().set_count(19).set_id(1).build();

        // Assert
        assert_eq!(Ok(()), inv.add(item));
    }

    #[test]
    fn add_returns_ok_when_added_item_is_equal_to_max_capacity() {
        // Arrange
        let mut inv = Inventory::new();

        // Act
        inv.set_max_capacity(20);
        let item = ItemBuilder::new().set_count(20).set_id(1).build();

        // Assert
        assert_eq!(Ok(()), inv.add(item));
        assert_eq!(20, inv.capacity());
    }

    #[test]
    fn add_test() {
        // Arrange
        let mut inv = Inventory::new();

        // Act
        inv.set_max_capacity(20);
        let item = ItemBuilder::new().set_count(20).set_id(1).build();
        let item_2 = ItemBuilder::new().set_count(1).set_id(1).build();
        println!("adding item");
        println!("Added Item: {:?}", inv.add(item));
        println!("added item");
        // Assert
        assert_eq!(
            Err(InventoryTransportError::ItemAddCapacityOverflow(
                ItemBuilder::new().set_count(1).set_id(1).build()
            )),
            inv.add(item_2)
        )
    }
}
