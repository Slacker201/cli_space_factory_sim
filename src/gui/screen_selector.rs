#[derive(Clone)]
pub enum ScreenSelector {
    World,
    Node(u64),
    Factory(u64),
    Inventory(SelectedInventory),
    DeveloperTools,
    Recipe(SelectedRecipeField),
}
#[derive(Clone)]
pub enum SelectedInventory {
    InputInventory,
    OutputInventory,
    ProcessingInventory,
}

#[derive(Clone)]
pub enum SelectedRecipeField {
    InputItems,
    OutputItems,
    None,
}
