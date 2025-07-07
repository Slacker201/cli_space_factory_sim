#[cfg(test)]
mod tests {
    use crate::{entities::entity_components::{assembler::{assembler::Assembler, processing_state::ProcessingState}, inventory::inventory::Inventory}, item_utils::recipe::recipe::Recipe};

    #[test]
    fn default_values() {
        let assembler = Assembler::new();
    

        assert_eq!(assembler.input_inventory(), &Inventory::new());
        assert_eq!(assembler.output_inventory(), &Inventory::new());
        assert_eq!(assembler.processing_inventory(), &Inventory::new());
        assert_eq!(assembler.recipe(), &Recipe::new());
        assert_eq!(assembler.processing_state(), &ProcessingState::Idle)
    }
}