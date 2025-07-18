use std::collections::HashMap;

use crate::{entities::factories::factory::Factory, info, warn};




/// Represents a node with a factory hashmap, a name to id map, and a factory limit
pub struct Node {
    /// The factories the node is storing
    factories: HashMap<u64, Factory>,
    /// The name to id converter
    name_to_id_map: HashMap<String, u64>,
    /// The amount of factories the node can hold
    factory_limit: usize
}

impl Node {
    /// The new constructer. Returns a new node with a max factory limit of 5
    /// 
    /// # Returns
    /// 
    /// A new node object with factory_limit set to 5
    /// 
    pub fn new() -> Node {
        Node { factories: HashMap::new(), name_to_id_map: HashMap::new(), factory_limit: 5 }
    }
    /// Adds a factory, and returns the factory if it cannot add it.
    /// 
    /// # Arguments
    /// * fac - The factory that will be added
    ///
    /// # Returns
    /// 
    /// None if the factory is added, and the factory if it is not added because the id already exists, the name already exists, or the node contains too many factories
    /// 
    pub fn add_factory(&mut self, fac: Factory) -> Option<Factory> {
        info!("Factory current len and limit: {} {}", self.factories.len(), self.factory_limit);
        if self.factories.len() >= self.factory_limit {
            return Some(fac)
        }
        if self.factories.contains_key(&fac.id()) {
            warn!("Duplicate Ids: {}", fac.id());
            return Some(fac)
        }
        if self.name_to_id_map.contains_key(fac.name()) {
            warn!("Duplicate Names: {}", fac.name());
            return Some(fac)
        }
        self.name_to_id_map.insert(normalize_name_str(&fac.name()), fac.id());
        self.factories.insert(fac.id(), fac);
        None
    }
    /// Immutable getter for factories
    /// 
    /// # Returns
    /// 
    /// A reference to the hashmap containing the factories
    /// 
    pub fn factories(&self) -> &HashMap<u64, Factory> {
        &self.factories
    }
    /// Mutable getter for the factories
    /// 
    /// # Returns
    /// 
    /// A mutable reference to the hashmap containing the factories
    /// 
    pub fn factories_mut(&mut self) -> &mut HashMap<u64, Factory> {
        &mut self.factories
    }
    /// Clears the factories hashmap and the name to id map
    /// 
    pub fn clear_factories(&mut self) {
        self.factories.clear();
        self.name_to_id_map.clear();
    }
    /// Returns if there is a factory with the given name
    /// 
    /// # Arguments
    /// 
    /// - "name" - The name of the factory you are trying to find
    /// 
    /// # Returns
    /// 
    /// True or false, depending if the factory exists
    /// 
    pub fn contains_factory_with_name(&self, name: &String) -> bool {
        self.name_to_id_map.contains_key(&normalize_name_str(name))
    }
    /// Converts a name to an option for a factory id
    /// 
    /// # Arguments
    /// 
    /// * name - The name of the factory you are getting the id of. This is trimmed and set to lowercase before use
    /// 
    /// # Returns
    /// 
    /// Some(id) if the factory exists, or None if it does not
    /// 
    pub fn name_to_id(&self, name: &String) -> Option<u64> {
        self.name_to_id_map.get(&normalize_name_str(name)).copied()
    }
    /// Removes a factory from the factory hashmap
    /// 
    /// # Arguments
    /// * id - The id of the factory you want to remove
    pub fn remove_factory(&mut self, id: u64) {
        match self.factories.remove(&id) {
            Some(fac) => {
                self.name_to_id_map.remove(fac.name());
            },
            None => {
                info!("Factory not removed due to it not being present")
            },
        }
    }
    /// Returns an option for a factory reference
    /// 
    /// # Arguments
    /// 
    /// * id - The factory id you are trying to get
    /// 
    /// # Returns
    /// 
    /// None if the factory doesn't exist, and Some(&factory) if it does
    /// 
    pub fn get_factory(&self, id: u64) -> Option<&Factory> {
        self.factories.get(&id)
    }
}

fn normalize_name_str(name: &str) -> String {
    name.to_lowercase().trim().to_string()
}