




#[cfg(test)]
mod item_builder_tests {
    use crate::item_utils::item::item_builder::ItemBuilder;

    #[test]
    fn default_values() {
        let builder = ItemBuilder::new();

        assert_eq!(builder.id(), 0);
        assert_eq!(builder.count(), 1)
    }
    #[test]
    fn id_setter() {
        let builder = ItemBuilder::new().set_id(2);

        assert_eq!(builder.id(), 2)
    }
    #[test]
    fn count_setter() {
        let builder= ItemBuilder::new().set_count(2);

        assert_eq!(builder.count(), 2)
    }
}

#[cfg(test)]
mod item_tests {
    use std::u64;

    use crate::item_utils::item::item::Item;


    #[test]
    fn default_values() {
        let item: Item = Item::new();
        assert_eq!(item.id(), 0);
        assert_eq!(item.count(), 1);
    }
    #[test]
    fn id_setter() {
        let mut item: Item = Item::new();
        item.set_id(2);
        assert_eq!(item.id(), 2)
    }
    #[test]
    fn count_setter() {
        let mut item: Item = Item::new();
        item.set_count(2);
        assert_eq!(item.count(), 2)
    }
    #[test]
    fn name_returns_null_for_default_item() {
        let item: Item = Item::new();
        assert_eq!(item.name(), Some("null"));
    }
    #[test]
    fn name_returns_correct_name_for_valid_id() {
        let mut item: Item = Item::new();
        item.set_id(1);
        assert_eq!(item.name(), Some("Iron Ore"));
    }
    #[test]
    fn name_returns_none_for_invalid_id() {
        let mut item: Item = Item::new();
        item.set_id(u64::MAX);
        assert_eq!(item.name(), None);
    }
}