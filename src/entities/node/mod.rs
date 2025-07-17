use std::collections::HashMap;

use crate::entities::factories::factory::Factory;





pub struct Node {
    factories: HashMap<u64, Factory>,
    factory_limit: usize
}

impl Node {
    pub fn new() -> Node {
        Node { factories: HashMap::new(), factory_limit: 5 }
    }
    pub fn add_factory(&mut self, fac: Factory) -> Option<Factory> {
        if self.factories.len() >= self.factory_limit {
            return Some(fac)
        }
        self.factories.insert(fac.id(), fac);
        None
    }
    pub fn factories(&self) -> &HashMap<u64, Factory> {
        &self.factories
    }
    pub fn factories_mut(&mut self) -> &mut HashMap<u64, Factory> {
        &mut self.factories
    }
    pub fn clear_factories(&mut self) {
        self.factories.clear();
    }
}