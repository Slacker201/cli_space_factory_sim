



#[cfg(test)]
mod test {
    use crate::entities::{factories::factory::Factory, node::Node};

    #[test]
    fn add_factory_adds_a_factory() {
        let mut node = Node::new();
        let mut fac = Factory::new();
        fac.set_id(5);
        fac.set_name("foo".to_string());
        node.add_factory(fac);

        assert!(node.contains_factory_with_name(&"foo".to_string()));
        assert!(node.contains_factory(5))
    }

    #[test]
    fn add_factory_adds_multiple_factories() {
        let mut node = Node::new();
        let mut fac = Factory::new();
        let mut fac2 = Factory::new();
        fac.set_id(5);
        fac.set_name("foo".to_string());

        fac2.set_id(4);
        fac2.set_name("bar".to_string());
        
        node.add_factory(fac);
        node.add_factory(fac2);

        assert!(node.contains_factory_with_name(&"foo".to_string()));
        assert!(node.contains_factory_with_name(&"bar".to_string()));
        assert!(node.contains_factory(5));
        assert!(node.contains_factory(4))
    }

    #[test]
    fn remove_factory_removes_factory() {
        let mut node = Node::new();
        let mut fac = Factory::new();
        fac.set_id(5);
        fac.set_name("foo".to_string());
        
        node.add_factory(fac);

        node.remove_factory(5);
        assert!(node.factories().is_empty())
    }
    #[test]
    fn remove_factories_with_multiple_factories_removes_one() {
                let mut node = Node::new();
        let mut fac = Factory::new();
        let mut fac2 = Factory::new();
        fac.set_id(5);
        fac.set_name("foo".to_string());

        fac2.set_id(4);
        fac2.set_name("bar".to_string());
        
        node.add_factory(fac);
        node.add_factory(fac2);

        node.remove_factory(5);
        
        
        assert_eq!(node.factories().len(), 1);
    }
}