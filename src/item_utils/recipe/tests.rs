


#[cfg(test)]
mod recipe_tests {
    use crate::{entities::entity_components::inventory::inventory::Inventory, item_utils::{item::{item::Item, item_builder::ItemBuilder}, recipe::recipe::Recipe}};


    #[test]
    fn default_values() {
        let recipe = Recipe::new();
        
        assert_eq!(recipe.input_items(), Vec::<&Item>::new());
        assert_eq!(recipe.output_items(), Vec::<&Item>::new());
        assert_eq!(recipe.power_draw(), 1);
        assert_eq!(recipe.heat_produced(), 1);
    }

    #[test]
    fn set_get_input_items() {
        let mut recipe = Recipe::new();
        recipe.set_input_items([ItemBuilder::new().set_count(5).set_id(1).build(), ItemBuilder::new().set_count(4).set_id(2).build()].to_vec());
        assert_eq!(recipe.input_items(), [&ItemBuilder::new().set_count(5).set_id(1).build(), &ItemBuilder::new().set_count(4).set_id(2).build()].to_vec());
    }
    #[test]
    fn set_get_output_items() {
        let mut recipe = Recipe::new();
        recipe.set_output_items([ItemBuilder::new().set_count(5).set_id(1).build(), ItemBuilder::new().set_count(4).set_id(2).build()].to_vec());
        assert_eq!(recipe.output_items(), [&ItemBuilder::new().set_count(5).set_id(1).build(), &ItemBuilder::new().set_count(4).set_id(2).build()].to_vec());
    }

    #[test]
    fn set_get_processing_time() {
        let mut recipe = Recipe::new();
        recipe.set_processing_time(4);
        assert_eq!(recipe.processing_time(), 4);
    }
    #[test]
    fn set_get_heat_production() {
        let mut recipe = Recipe::new();
        recipe.set_heat_produced(4);
        assert_eq!(recipe.heat_produced(), 4);
    }
    #[test]
    fn set_get_power_draw() {
        let mut recipe = Recipe::new();
        recipe.set_power_draw(4);
        assert_eq!(recipe.power_draw(), 4);
    }

    #[test]
    fn can_be_produced_returns_true() {
        let mut inv = Inventory::new();
        inv.add_multiple([ItemBuilder::new().set_count(50).set_id(1).build(), ItemBuilder::new().set_count(25).set_id(2).build()].to_vec());
        let mut recipe = Recipe::new();
        recipe.set_input_items([ItemBuilder::new().set_count(10).set_id(1).build(), ItemBuilder::new().set_count(5).set_id(2).build()].to_vec());

        assert!(recipe.can_be_produced(&inv))
    }
    #[test]
    fn can_be_produced_returns_false_on_inv_empty() {
        let inv = Inventory::new();
        let mut recipe = Recipe::new();
        recipe.set_input_items([ItemBuilder::new().set_count(10).set_id(1).build(), ItemBuilder::new().set_count(5).set_id(2).build()].to_vec());

        assert!(!recipe.can_be_produced(&inv))
    }
    #[test]
    fn can_be_produced_returns_false_on_not_enough_items() {
        let mut inv = Inventory::new();
        inv.add_multiple([ItemBuilder::new().set_count(5).set_id(1).build(), ItemBuilder::new().set_count(4).set_id(2).build()].to_vec());
        let mut recipe = Recipe::new();
        recipe.set_input_items([ItemBuilder::new().set_count(10).set_id(1).build(), ItemBuilder::new().set_count(5).set_id(2).build()].to_vec());

        assert!(!recipe.can_be_produced(&inv))
    }

    #[test]
    fn can_be_produced_returns_false_on_enough_items_except_for_one_stack() {
        let mut inv = Inventory::new();
        inv.add_multiple([ItemBuilder::new().set_count(20).set_id(1).build(), ItemBuilder::new().set_count(4).set_id(2).build()].to_vec());
        let mut recipe = Recipe::new();
        recipe.set_input_items([ItemBuilder::new().set_count(10).set_id(1).build(), ItemBuilder::new().set_count(5).set_id(2).build()].to_vec());

        assert!(!recipe.can_be_produced(&inv))
    }
    #[test]
    fn can_be_produced_ignores_zero_count_requirements() {
        let mut inv = Inventory::new();
        inv.add_multiple([ItemBuilder::new().set_count(20).set_id(2).build()].to_vec());
        let mut recipe = Recipe::new();
        recipe.set_input_items([ItemBuilder::new().set_count(0).set_id(1).build(), ItemBuilder::new().set_count(5).set_id(2).build()].to_vec());

        assert!(recipe.can_be_produced(&inv))
    }

    #[test]
    fn can_be_produced_ignores_extra_items() {
        let mut inv = Inventory::new();
        inv.add_multiple([ItemBuilder::new().set_count(20).set_id(1).build(), ItemBuilder::new().set_count(5).set_id(2).build()].to_vec());
        let mut recipe = Recipe::new();
        recipe.set_input_items([ItemBuilder::new().set_count(5).set_id(1).build()].to_vec());

        assert!(recipe.can_be_produced(&inv))
    }

    #[test] 
    fn can_be_produced_empty_recipe_returns_true_on_filled_inventory() {
        let mut inv = Inventory::new();
        inv.add_multiple([ItemBuilder::new().set_count(20).set_id(1).build(), ItemBuilder::new().set_count(5).set_id(2).build()].to_vec());
        let recipe = Recipe::new();

        assert!(recipe.can_be_produced(&inv))
    }

    #[test] 
    fn can_be_produced_empty_recipe_returns_true_on_empty_inventory() {
        let inv = Inventory::new();
        let recipe = Recipe::new();

        assert!(recipe.can_be_produced(&inv))
    }

    #[test]
    fn as_transport_order_basic_values() {
        let mut recipe = Recipe::new();
        recipe.set_input_items([ItemBuilder::new().set_count(20).set_id(1).build()].to_vec());
        let t_order = recipe.output_items_as_transport_order();

        assert_eq!(t_order.items(), &[ItemBuilder::new().set_count(20).set_id(1).build()].to_vec())
    }
    #[test]
    fn as_transport_order_zero_count() {
        let mut recipe = Recipe::new();
        recipe.set_input_items([ItemBuilder::new().set_count(0).set_id(1).build()].to_vec());
        let t_order = recipe.output_items_as_transport_order();

        assert_eq!(t_order.items(), &[].to_vec())
    }
    #[test]
    fn as_transport_order_multiple_nonzero_values() {
        let mut recipe = Recipe::new();
        recipe.set_input_items([ItemBuilder::new().set_count(20).set_id(1).build(), ItemBuilder::new().set_count(20).set_id(2).build()].to_vec());
        let t_order = recipe.output_items_as_transport_order();

        assert_eq!(t_order.items(), &[ItemBuilder::new().set_count(20).set_id(1).build(), ItemBuilder::new().set_count(20).set_id(2).build()].to_vec())
    }
    #[test]
    fn as_transport_order_multiple_values_mixed_zero_and_non_zero () {
        let mut recipe = Recipe::new();
        recipe.set_input_items([ItemBuilder::new().set_count(20).set_id(1).build(), ItemBuilder::new().set_count(0).set_id(2).build()].to_vec());
        let t_order = recipe.output_items_as_transport_order();

        assert_eq!(t_order.items(), &[ItemBuilder::new().set_count(20).set_id(1).build()].to_vec())
    }
}