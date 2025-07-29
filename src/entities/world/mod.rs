#![allow(dead_code)]

use std::collections::HashMap;

use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::{
    entities::node::Node, item_utils::
        recipe::recipe::Recipe
    
};

pub struct World {
    player_recipes: HashMap<String, Recipe>,
    all_recipes: HashMap<String, Recipe>,
    nodes: HashMap<u64, Node>,
}

impl World {
    pub fn new() -> World {
        World {
            player_recipes: HashMap::new(),
            all_recipes: HashMap::new(),
            nodes: HashMap::new(),
        }
    }
    pub fn all_recipes_mut(&mut self) -> &mut HashMap<String, Recipe> {
        &mut self.all_recipes
    }
    pub fn nodes(&self) -> &HashMap<u64, Node> {
        &self.nodes
    }
    pub fn nodes_mut(&mut self) -> &mut HashMap<u64, Node> {
        &mut self.nodes
    }
    pub fn set_nodes(&mut self, nodes: HashMap<u64, Node>) {
        self.nodes = nodes;
    }
    pub fn tick(&mut self) {
        self.nodes_mut().par_iter_mut().for_each(|(_node_id, node)| {
            node.tick();
        });
    }
}
