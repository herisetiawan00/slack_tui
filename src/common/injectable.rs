use std::{
    any::{Any, TypeId},
    collections::HashSet,
};

use crate::common::Context;

pub trait Injectable {
    fn as_any(&self) -> &dyn Any;
}

pub struct Registry {
    registered: Vec<Box<dyn Injectable>>,
    types: HashSet<TypeId>,
}

impl Registry {
    pub fn new() -> Self {
        return Self {
            registered: Vec::new(),
            types: HashSet::new(),
        };
    }

    pub fn register<T: Injectable + 'static>(&mut self, injectable: T) {
        let type_id = TypeId::of::<T>();

        if self.types.contains(&type_id) {
            return;
        }

        self.types.insert(type_id);
        self.registered.push(Box::new(injectable));
    }

    pub fn unregister<T: Injectable + 'static>(mut self) {
        let type_id = TypeId::of::<T>();

        if !self.types.contains(&type_id) {
            return;
        }

        self.types.remove(&type_id);
        self.registered.retain(|i| Any::type_id(&**i) != type_id);
    }

    pub fn resolve<T: Injectable + 'static>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();

        if !self.types.contains(&type_id) {
            return None;
        }

        self.registered
            .iter()
            .find_map(|i| (**i).as_any().downcast_ref::<T>())
    }

    pub fn of(context: &Context) -> &Self {
        &context.registry
    }
}
