use crate::{entities::entity_components::{assembler::processing_state::ProcessingState, inventory::inventory::Inventory}, item_utils::recipe::recipe::Recipe};


/// The assembler module for a factory. This module converts items to other items
pub struct Assembler {
    /// The inventory that input items are placed in
    input_inventory: Inventory,
    /// The inventory that items are stored in while processing
    processing_inventory: Inventory,
    /// The inventory that items are stored in while waiting to be removed
    output_inventory: Inventory,
    /// The recipe to craft
    recipe: Recipe,
    /// The counter that determines when to add items to the output inventory and move items from the input to the processing inventory
    processing_state: ProcessingState
}

impl Assembler {
    /// Generates a new assembler with an empty input inventory, an empty processing inventory, an empty output inventory, an empty recipe, and Idle for processing Timer
    pub fn new() -> Assembler {
        Assembler { input_inventory: Inventory::new(), processing_inventory: Inventory::new(), output_inventory: Inventory::new(), recipe: Recipe::new(), processing_state: ProcessingState::Idle }
    }
    /// Returns a reference to the input inventory
    pub fn input_inventory(&self) -> &Inventory {
        &self.input_inventory
    }
    /// Returns a reference to the processing inventory
    pub fn processing_inventory(&self) -> &Inventory {
        &self.processing_inventory
    }
    /// Returns a reference to the output inventory
    pub fn output_inventory(&self) -> &Inventory {
        &self.output_inventory
    }
    /// Returns a reference to the recipe
    pub fn recipe(&self) -> &Recipe {
        &self.recipe
    }
    /// Returns a reference to the processing state
    pub fn processing_state(&self) -> &ProcessingState {
        &self.processing_state
    }
    /// Returns a mutable reference to the input inventory
    pub fn input_inventory_mut(&mut self) -> &mut Inventory {
        &mut self.input_inventory
    }
    /// Returns a mutable reference to the processing inventory
    pub fn processing_inventory_mut(&mut self) -> &mut Inventory {
        &mut self.processing_inventory
    }
    /// Returns a mutable reference to the output inventory
    pub fn output_inventory_mut(&mut self) -> &mut Inventory {
        &mut self.output_inventory
    }
    /// Returns a mutable reference to the recipe.
    /// This may be removed in future versions
    pub fn recipe_mut(&mut self) -> &mut Recipe {
        &mut self.recipe
    }
    /// Setter for the input inventory.
    /// No checks are conducted
    pub fn set_input_inventory(&mut self, inv: Inventory) {
        self.input_inventory = inv;
    }
    /// Setter for the processing inventory.
    /// No checks are conducted
    pub fn set_processing_inventory(&mut self, inv: Inventory) {
        self.processing_inventory = inv;
    }
    /// Setter for the output inventory.
    /// No checks are conducted
    pub fn set_output_inventory(&mut self, inv: Inventory) {
        self.output_inventory = inv;
    }
    /// Setter for the recipe.
    /// No checks are conducted
    pub fn set_recipe(&mut self, recipe: Recipe) {
        self.recipe = recipe;
    }
    /// Setter for the processing timer.
    /// No checks are conducted.
    /// This may be replaced by a reset function for the processing count
    pub fn set_processing_state(&mut self, new_count: ProcessingState) {
        self.processing_state = new_count;
    }

    /// Processes a single tick for the assembler.
    ///
    /// If enough time has passed, completes the current recipe and moves output items to the output inventory.
    /// Otherwise, checks if the recipe can be produced and, if so, starts the processing cycle.
    /// This function starts producing a recipe the same tick the previous recipe finished, assuming it has enough items.
    pub fn tick(&mut self) {
        if let ProcessingState::Processing(count) = self.processing_state {
            self.processing_state += 1; // Increase the processing count
            if count+1 >= self.recipe.processing_time() {
                self.output_inventory.add_multiple(self.recipe.output_items().into_iter().cloned().collect()); // Add items to the output inventory
                self.processing_inventory.clear();
                self.set_processing_state(ProcessingState::Idle); // Reset to idle
            }
            else {
                return; // We are still processing, return to skip the relatively costly can be produced check
            }
        }

        if self.recipe.can_be_produced(&self.input_inventory) {
            self.input_inventory.move_items_to(self.recipe.input_items_as_transport_order(), &mut self.processing_inventory); // Move items from the input inventory to the processing inventory
            self.processing_state = ProcessingState::Processing(1) // Start the processing
        }
    }
}
