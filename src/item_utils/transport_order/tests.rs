



#[cfg(test)]
mod transport_order_tests {
    
    use crate::item_utils::{item::item_builder::ItemBuilder, transport_order::transport_order::TransportOrder};


    #[test]
    fn id_persistance() {
        let mut t_order = TransportOrder::new();
        t_order.set_items([ItemBuilder::new().set_count(5).set_id(1).build()].to_vec());

        assert_eq!(t_order.items()[0].id(), 1)
    }
    #[test]
    fn count_persistance() {
        let mut t_order = TransportOrder::new();
        t_order.set_items([ItemBuilder::new().set_count(5).set_id(1).build()].to_vec());

        assert_eq!(t_order.items()[0].count(), 5)
    }
}