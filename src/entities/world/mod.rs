use std::collections::HashMap;

use crate::{entities::node::Node, item_utils::recipe::recipe::Recipe};






pub struct World {
    _player_recipes: HashMap<String, Recipe>,
    all_recipes: HashMap<String, Recipe>,
    node: Node
}


impl World {
    pub fn new() -> World {
        World { _player_recipes: HashMap::new(), all_recipes: HashMap::new(), node: Node::new() }
    }
    pub fn all_recipes_mut(&mut self) -> &mut HashMap<String, Recipe> {
        &mut self.all_recipes
    }
    pub fn node(&self) -> &Node {
        &self.node
    }
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}