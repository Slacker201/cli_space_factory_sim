#[cfg(test)]
mod tests {
    use crate::{
        entities::entity_components::{
            assembler::{assembler::Assembler, processing_state::ProcessingState},
            inventory::Inventory,
        },
        item_utils::{item::item_builder::ItemBuilder, recipe::recipe::Recipe},
    };

    #[test]
    fn default_values() {
        let assembler = Assembler::new();

        assert_eq!(assembler.input_inventory(), &Inventory::new());
        assert_eq!(assembler.output_inventory(), &Inventory::new());
        assert_eq!(assembler.processing_inventory(), &Inventory::new());
        assert_eq!(assembler.recipe(), &Recipe::new());
        assert_eq!(assembler.processing_state(), &ProcessingState::Idle)
    }
    #[test]
    fn input_inventory_setter() {
        // Arrange
        let mut inv = Inventory::new();
        let mut assembler = Assembler::new();

        // Act
        let _ = inv.add(ItemBuilder::new().set_count(1).set_id(1).build());
        assembler.set_input_inventory(inv.clone());

        // Assert
        assert_eq!(assembler.input_inventory(), &inv)
    }
    #[test]
    fn output_inventory_setter() {
        // Arrange
        let mut inv = Inventory::new();
        let mut assembler = Assembler::new();

        // Act
        let _ = inv.add(ItemBuilder::new().set_count(1).set_id(1).build());
        assembler.set_output_inventory(inv.clone());

        // Assert
        assert_eq!(assembler.output_inventory(), &inv)
    }

    #[test]
    fn processing_inventory_setter() {
        // Arrange
        let mut inv = Inventory::new();
        let mut assembler = Assembler::new();

        // Act
        let _ = inv.add(ItemBuilder::new().set_count(1).set_id(1).build());
        assembler.set_processing_inventory(inv.clone());

        // Assert
        assert_eq!(assembler.processing_inventory(), &inv)
    }

    #[test]
    fn recipe_setter() {
        // Arrange
        let mut rec = Recipe::new();
        let mut assembler = Assembler::new();

        // Act
        rec.set_input_items(Vec::from([ItemBuilder::new()
            .set_count(1)
            .set_id(1)
            .build()]));
        assembler.set_recipe(rec.clone());

        // Assert
        assert_eq!(assembler.recipe(), &rec);
    }

    #[test]
    fn processing_state_setter() {
        // Arrange
        let mut assembler = Assembler::new();
        // Act
        assembler.set_processing_state(ProcessingState::Processing(69));
        // Assert
        assert_eq!(
            assembler.processing_state(),
            &ProcessingState::Processing(69)
        );
    }

    #[test]
    fn tick_no_items_generates_no_items() {
        // Arrange
        let mut assembler = Assembler::new();
        let mut rec = Recipe::new();
        // Act
        rec.set_input_items(Vec::from([ItemBuilder::new()
            .set_count(5)
            .set_id(1)
            .build()]));
        rec.set_processing_time(1);
        assembler.set_recipe(rec);
        for _ in 0..10 {
            // 10 is an arbitrary number that should ensure that any processing cycles that will happen do happen
            assembler.tick();
        }
        // Assert
        assert!(assembler.output_inventory().is_empty())
    }

    #[test]
    fn tick_rec_processing_time_greater_than_one_processing_state_is_processing_after_one_tick() {
        // Arrange
        let mut assembler = Assembler::new();
        let mut rec = Recipe::new();

        // Act
        rec.set_input_items(Vec::from([ItemBuilder::new()
            .set_count(5)
            .set_id(1)
            .build()]));
        rec.set_processing_time(20);
        assembler.set_recipe(rec);
        let _ = assembler
            .input_inventory_mut()
            .add(ItemBuilder::new().set_count(200).set_id(1).build());
        assembler.tick();

        // Assert
        assert_eq!(
            assembler.processing_state(),
            &ProcessingState::Processing(1),
            "After one tick from initlialization and sufficient items, the processing state should be 1"
        );
        assembler.tick();
        assert_eq!(
            assembler.processing_state(),
            &ProcessingState::Processing(2),
            "After two tick from initlialization and sufficient items, the processing state should be 2"
        );
        assembler.tick();
        assert_eq!(
            assembler.processing_state(),
            &ProcessingState::Processing(3),
            "After three tick from initlialization and sufficient items, the processing state should be 3"
        );
    }

    #[test]
    fn tick_item_is_produced_when_sufficient_items_are_present() {
        // Arrange
        let mut assembler = Assembler::new();
        let mut rec = Recipe::new();

        // Act
        rec.set_input_items(Vec::from([ItemBuilder::new()
            .set_count(5)
            .set_id(1)
            .build()]));
        rec.set_output_items(Vec::from([ItemBuilder::new()
            .set_count(1)
            .set_id(2)
            .build()]));
        rec.set_processing_time(1);
        assembler.set_recipe(rec);
        let _ = assembler
            .input_inventory_mut()
            .add(ItemBuilder::new().set_count(200).set_id(1).build());
        assembler.tick();

        // Assert
        assert_eq!(
            assembler.output_inventory().get(2),
            Some(&ItemBuilder::new().set_count(1).set_id(2).build())
        )
    }

    #[test]
    fn tick_multiple_items_are_produced_with_enough_input() {
        // Arrange
        let mut assembler = Assembler::new();
        let mut rec = Recipe::new();

        // Act
        rec.set_input_items(Vec::from([ItemBuilder::new()
            .set_count(5)
            .set_id(1)
            .build()]));
        rec.set_output_items(Vec::from([ItemBuilder::new()
            .set_count(1)
            .set_id(2)
            .build()]));
        rec.set_processing_time(1);
        assembler.set_recipe(rec);
        let _ = assembler
            .input_inventory_mut()
            .add(ItemBuilder::new().set_count(200).set_id(1).build());
        for _ in 0..5 {
            assembler.tick();
        }

        // Assert
        assert_eq!(
            assembler.output_inventory().get(2),
            Some(&ItemBuilder::new().set_count(5).set_id(2).build())
        )
    }
    #[test]
    fn tick_inputs_are_removed_on_item_construction() {
        // Arrange
        let mut assembler = Assembler::new();
        let mut rec = Recipe::new();

        // Act
        rec.set_input_items(Vec::from([ItemBuilder::new()
            .set_count(5)
            .set_id(1)
            .build()]));
        rec.set_output_items(Vec::from([ItemBuilder::new()
            .set_count(1)
            .set_id(2)
            .build()]));
        rec.set_processing_time(1);
        assembler.set_recipe(rec);
        let _ = assembler
            .input_inventory_mut()
            .add(ItemBuilder::new().set_count(200).set_id(1).build());
        for _ in 0..5 {
            assembler.tick();
        }

        // Assert
        assert_eq!(
            assembler.input_inventory().get(1),
            Some(&ItemBuilder::new().set_count(175).set_id(1).build())
        )
    }

    #[test]
    fn tick_on_item_completion_processing_state_is_idle() {
        // Arrange
        let mut assembler = Assembler::new();
        let mut rec = Recipe::new();

        // Act
        rec.set_input_items(Vec::from([ItemBuilder::new()
            .set_count(5)
            .set_id(1)
            .build()]));
        rec.set_output_items(Vec::from([ItemBuilder::new()
            .set_count(1)
            .set_id(2)
            .build()]));
        rec.set_processing_time(1);
        assembler.set_recipe(rec);
        let _ = assembler
            .input_inventory_mut()
            .add(ItemBuilder::new().set_count(5).set_id(1).build());
        for _ in 0..5 {
            assembler.tick();
        }

        // Assert
        assert_eq!(assembler.processing_state(), &ProcessingState::Idle)
    }

    #[test]
    fn tick_stays_idle_on_insufficient_items() {
        // Arrange
        let mut assembler = Assembler::new();
        let mut rec = Recipe::new();

        // Act
        rec.set_input_items(Vec::from([ItemBuilder::new()
            .set_count(5)
            .set_id(1)
            .build()]));
        rec.set_output_items(Vec::from([ItemBuilder::new()
            .set_count(1)
            .set_id(2)
            .build()]));
        rec.set_processing_time(1);
        assembler.set_recipe(rec);
        let _ = assembler
            .input_inventory_mut()
            .add(ItemBuilder::new().set_count(4).set_id(1).build());
        assembler.tick();

        // Assert
        assert!(assembler.output_inventory().is_empty());
    }

    #[test]
    fn tick_produces_one_item_recipe_when_processing_time_is_zero() {
        // Arrange
        let mut assembler = Assembler::new();
        let mut rec = Recipe::new();

        // Act
        rec.set_input_items(Vec::from([ItemBuilder::new()
            .set_count(5)
            .set_id(1)
            .build()]));
        rec.set_output_items(Vec::from([ItemBuilder::new()
            .set_count(1)
            .set_id(2)
            .build()]));
        rec.set_processing_time(0);
        assembler.set_recipe(rec);
        let _ = assembler
            .input_inventory_mut()
            .add(ItemBuilder::new().set_count(20).set_id(1).build());
        assembler.tick();

        // Assert
        assert_eq!(
            assembler.output_inventory().get(2),
            Some(&ItemBuilder::new().set_count(1).set_id(2).build()),
            "More or less than one item was produced"
        );
        assert_eq!(assembler.processing_state(), &ProcessingState::Idle)
    }

    #[test]
    fn tick_produces_items_on_rec_input_items_being_empty() {
        // Arrange
        let mut assembler = Assembler::new();
        let mut rec = Recipe::new();

        // Act
        rec.set_input_items(Vec::new());
        rec.set_output_items(Vec::from([ItemBuilder::new()
            .set_count(1)
            .set_id(2)
            .build()]));
        rec.set_processing_time(1);
        assembler.set_recipe(rec);
        let _ = assembler
            .input_inventory_mut()
            .add(ItemBuilder::new().set_count(4).set_id(1).build());
        assembler.tick();

        // Assert
        assert_eq!(
            assembler.output_inventory().get(2),
            Some(&ItemBuilder::new().set_count(1).set_id(2).build())
        );
    }

    #[test]
    fn tick_produces_items_when_input_items_are_equal_to_requirements() {
        // Arrange
        let mut assembler = Assembler::new();
        let mut rec = Recipe::new();

        // Act
        rec.set_input_items(Vec::from([ItemBuilder::new()
            .set_count(5)
            .set_id(1)
            .build()]));
        rec.set_output_items(Vec::from([ItemBuilder::new()
            .set_count(1)
            .set_id(2)
            .build()]));
        rec.set_processing_time(1);
        assembler.set_recipe(rec);
        let _ = assembler
            .input_inventory_mut()
            .add(ItemBuilder::new().set_count(5).set_id(1).build());
        assembler.tick();

        // Assert
        assert_eq!(
            assembler.output_inventory().get(2),
            Some(&ItemBuilder::new().set_count(1).set_id(2).build())
        )
    }
}
