use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

use bevy_ecs::prelude::*;

#[derive(Default, Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct TypeId(usize);

#[derive(Resource, Default)]
struct TypeHierarchy<'a> {
    numbers: HashMap<Cow<'a, str>, TypeId>,
    nodes: Vec<Cow<'a, str>>,
    subclass: Vec<HashSet<TypeId>>,
    superclass: Vec<HashSet<TypeId>>,
}

impl<'a> TypeHierarchy<'a> {
    fn get_id(&mut self, class: &str) -> TypeId {
        if let Some(id) = self.numbers.get(class) {
            *id
        } else {
            let new_id = TypeId(self.nodes.len());
            let class_cow: Cow<'a, str> = Cow::Owned(class.to_string());
            self.nodes.push(class_cow.clone());
            self.numbers.insert(class_cow, new_id);
            self.subclass.push(HashSet::new());
            self.superclass.push(HashSet::new());
            new_id
        }
    }

    fn set_subclass_of(&mut self, class: TypeId, to: &str) {
        let to = self.get_id(to);
        self.subclass[class.0].insert(to);
        self.superclass[to.0].insert(class);
    }
}

pub fn extract_type_hierarchy(query: Query<()>) {}
