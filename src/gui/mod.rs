use egui::Ui;

use crate::{
    entities::{factories::factory::Factory, node::Node, world::World},
    error,
    gui::screen_selector::{ScreenSelector, SelectedInventory, SelectedRecipeField},
    info,
    item_utils::item::item::Item,
};

pub mod screen_selector;
pub struct SFSGui {
    screen_selected: ScreenSelector,
    world: Option<World>,
    selected_node: Option<u64>,
    selected_factory: Option<u64>,
    selected_inv: Option<SelectedInventory>,
}

impl Default for SFSGui {
    fn default() -> Self {
        Self {
            screen_selected: ScreenSelector::World,
            world: None,
            selected_factory: None,
            selected_node: None,
            selected_inv: None,
        }
    }
}

impl eframe::App for SFSGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match &self.screen_selected {
            ScreenSelector::World => {
                self.draw_world_view(ctx);
                println!("Drawing World View");
            }
            ScreenSelector::DeveloperTools => {
                self.draw_developer_tools(ctx);
                println!("Drawing Dev Tools")
            }
            ScreenSelector::Node(node) => {
                self.selected_node = Some(*node);
                self.draw_node_view(ctx, *node);
                println!("Drawing All facs in selected node");
            }
            ScreenSelector::Factory(id) => {
                self.selected_factory = Some(*id);
                println!("Drawing specific fac");
                let node_id = match self.selected_node {
                    Some(id) => id,
                    None => {
                        error!("Node id not found");
                        return;
                    }
                };
                self.draw_factory_view(ctx, node_id, *id);
            }
            ScreenSelector::Inventory(inventory) => {
                println!("Drawing inv");
                self.selected_inv = Some(inventory.clone());
                let node_id = match self.selected_node {
                    Some(id) => id,
                    None => {
                        error!("Missing Node Id");
                        return;
                    }
                };
                let fac_id = match self.selected_factory {
                    Some(id) => id,
                    None => {
                        error!("Missing Factory Id");
                        return;
                    }
                };
                self.draw_inventory_view(ctx, node_id, fac_id, inventory.clone());
            }
            ScreenSelector::Recipe(field) => {
                println!("Drawing recipe");
                let node_id = match self.selected_node {
                    Some(id) => id,
                    None => {
                        error!("Missing Node Id");
                        return;
                    }
                };
                let factory_id = match self.selected_factory {
                    Some(id) => id,
                    None => {
                        error!("Missing Factory Id");
                        return;
                    }
                };

                self.draw_recipe_view(ctx, node_id, factory_id, field.clone());
            }
        }
    }
}

impl SFSGui {
    fn draw_task_bar(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("task_bar").show(ctx, |ui| {
            ui.vertical(|ui| {
                if ui.button("World").clicked() {
                    self.screen_selected = ScreenSelector::World;
                }

                if ui.button("Dev Tools").clicked() {
                    self.screen_selected = ScreenSelector::DeveloperTools
                }
            });
        });
    }

    fn draw_world_view(&mut self, ctx: &egui::Context) {
        let node_ids: Vec<u64> = match &self.world {
            Some(world) => world.nodes().iter().map(|node| node.1.id()).collect(),
            None => {
                error!("World not found");
                return;
            }
        };
        self.draw_task_bar(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            for id in node_ids {
                self.draw_node(ui, id);
            }
        });
    }
    fn draw_node_view(&mut self, ctx: &egui::Context, selected_node: u64) {
        self.draw_task_bar(ctx);
        let node = match self.try_get_node(selected_node) {
            Ok(node) => node,
            Err(e) => {
                error!("{}", e);
                return; //TODO add error
            }
        };
        let id = node.id();
        let mut facs: Vec<u64> = node.factories().values().map(|v| v.id()).collect();
        facs.sort();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("Id: {}", id));
            for fac in facs {
                ui.group(|ui| {
                    self.draw_factory(ui, id, fac);
                });
            }
        });
    }
    fn draw_factory_view(&mut self, ctx: &egui::Context, node_id: u64, factory_id: u64) {
        self.draw_task_bar(ctx);
        let mut selected_screen = None;
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.try_get_factory(node_id, factory_id) {
                Ok(factory) => {
                    match factory.name() {
                        Some(name) => {
                            ui.heading(format!("Name: {}. Id: {}", name, factory_id));
                        }
                        None => {
                            ui.heading(format!("Id: {}", factory_id));
                        }
                    }
                    if ui.button("Input Inventory").clicked() {
                        selected_screen =
                            Some(ScreenSelector::Inventory(SelectedInventory::InputInventory));
                    }
                    if ui.button("Output Inventory").clicked() {
                        selected_screen = Some(ScreenSelector::Inventory(
                            SelectedInventory::OutputInventory,
                        ));
                    }
                    if ui.button("Processing Inventory").clicked() {
                        selected_screen = Some(ScreenSelector::Inventory(
                            SelectedInventory::ProcessingInventory,
                        ));
                    }
                    if ui.button("Recipe").clicked() {
                        selected_screen = Some(ScreenSelector::Recipe(
                            screen_selector::SelectedRecipeField::None,
                        ));
                    }
                }
                Err(e) => {
                    error!("{}", e);
                }
            }
        });
        if let Some(unwrapped_screen_selector) = selected_screen {
            self.screen_selected = unwrapped_screen_selector;
        }
    }
    fn draw_inventory_view(
        &mut self,
        ctx: &egui::Context,
        node_id: u64,
        factory_id: u64,
        selected_inventory: SelectedInventory,
    ) {
        self.draw_task_bar(ctx);
        let inv = match self.try_get_factory(node_id, factory_id) {
            Ok(factory) => match selected_inventory {
                SelectedInventory::InputInventory => factory.get_assembler().input_inventory(),
                SelectedInventory::OutputInventory => factory.get_assembler().output_inventory(),
                SelectedInventory::ProcessingInventory => {
                    factory.get_assembler().processing_inventory()
                }
            },
            Err(e) => {
                error!("{}", e);
                return;
            }
        };
        let items: Vec<Item> = inv.items().values().cloned().collect();
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_item_array_in_horizontal_scroll(ui, &items);
        });
    }
    fn draw_recipe_view(
        &mut self,
        ctx: &egui::Context,
        node_id: u64,
        factory_id: u64,
        field: SelectedRecipeField,
    ) {
        self.draw_task_bar(ctx);
        let data = match self.try_get_factory(node_id, factory_id) {
            Ok(factory) => {
                let recipe = factory.get_assembler().recipe();
                recipe.clone()
            }
            Err(e) => {
                error!("{}", e);
                return;
            }
        };
        let mut new_screen_selector = None;
        egui::CentralPanel::default().show(ctx, |ui| match field {
            SelectedRecipeField::None => {
                ui.vertical(|ui| {
                    ui.heading(data.name());
                    ui.separator();
                    if ui.button("Input Items").clicked() {
                        new_screen_selector =
                            Some(ScreenSelector::Recipe(SelectedRecipeField::InputItems));
                    }
                    if ui.button("Output Items").clicked() {
                        new_screen_selector =
                            Some(ScreenSelector::Recipe(SelectedRecipeField::OutputItems));
                    }
                });
            }
            SelectedRecipeField::InputItems => {
                ui.vertical(|ui| {
                    ui.heading(format!("{}'s input items", data.name()));
                    self.draw_item_array_in_horizontal_scroll(ui, data.input_items());
                    if ui.button("Back").clicked() {
                        new_screen_selector =
                            Some(ScreenSelector::Recipe(SelectedRecipeField::None));
                    }
                });
            }
            SelectedRecipeField::OutputItems => {
                ui.vertical(|ui| {
                    ui.heading(format!("{}'s output items", data.name()));
                    self.draw_item_array_in_horizontal_scroll(ui, data.output_items());
                    if ui.button("Back").clicked() {
                        new_screen_selector =
                            Some(ScreenSelector::Recipe(SelectedRecipeField::None));
                    }
                });
            }
        });

        if let Some(unwrapped_screen_selector) = new_screen_selector {
            self.screen_selected = unwrapped_screen_selector;
        }
    }
    fn draw_developer_tools(&mut self, ctx: &egui::Context) {
        self.draw_task_bar(ctx);
        egui::CentralPanel::default().show(ctx, |ui| ui.heading("TODO ADD DEV TOOLS"));
    }

    fn draw_factory(&mut self, ui: &mut Ui, node_id: u64, factory_id: u64) {
        let mut new_screen = None;
        match self.try_get_factory(node_id, factory_id) {
            Ok(fac) => {
                ui.horizontal(|ui| {
                    match fac.name() {
                        Some(name) => {
                            ui.label(format!("Name: {}", name));
                        }
                        None => {
                            info!("Name not found");
                        }
                    }
                    let factory_id = fac.id();
                    ui.label(format!("Id: {}", factory_id));
                    if ui.button("View Factory").clicked() {
                        new_screen = Some(ScreenSelector::Factory(factory_id));
                    }
                });
            }
            Err(e) => {
                error!("{}", e)
            }
        }
        if let Some(unwrapped_screen_selector) = new_screen {
            self.screen_selected = unwrapped_screen_selector;
            self.selected_node = Some(node_id);
            self.selected_factory = Some(factory_id);
        }
    }

    fn draw_item_array_in_horizontal_scroll(&mut self, ui: &mut egui::Ui, items: &[Item]) {
        let row_height = ui.text_style_height(&egui::TextStyle::Body);
        let max_height = row_height * 30.0;

        egui::ScrollArea::vertical()
            .max_height(max_height)
            .show(ui, |ui| {
                for item in items {
                    match item.name() {
                        Some(name) => {
                            ui.group(|ui| ui.label(format!("{} {}", item.count(), name)));
                        }
                        None => {
                            ui.group(|ui| {
                                ui.label(format!("Id: {}, Count: {}", item.id(), item.count()))
                            });
                        }
                    }
                }
            });
    }

    fn draw_node(&mut self, ui: &mut Ui, node_id: u64) {
        let mut selected_screen = None;
        match self.try_get_node(node_id) {
            Ok(node) => {
                ui.label(format!("Id: {}", node.id()));
                if ui.button("Factories").clicked() {
                    selected_screen = Some(ScreenSelector::Node(node_id));
                }
            }
            Err(e) => {
                ui.label(format!("Failed to get factory {}", node_id));
                error!("{}", e);
            }
        }
        if let Some(unwrapped_selected_screen) = selected_screen {
            self.screen_selected = unwrapped_selected_screen;
        }
    }

    fn try_get_node(&self, node_id: u64) -> Result<&Node, String> {
        match &self.world {
            Some(world) => match world.nodes().get(&node_id) {
                Some(node) => Ok(node),
                None => Err("Node Not Found".to_owned()),
            },
            None => Err("World Not Found".to_string()),
        }
    }
    fn try_get_factory(&self, node_id: u64, fac_id: u64) -> Result<&Factory, String> {
        match self.try_get_node(node_id) {
            Ok(node) => match node.get_factory(fac_id) {
                Some(fac) => Ok(fac),
                None => Err("Factory Not Found".to_owned()),
            },
            Err(e) => Err(e),
        }
    }

    pub fn set_world(&mut self, new_world: World) {
        self.world = Some(new_world)
    }
}
