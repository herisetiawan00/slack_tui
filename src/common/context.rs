use crate::common::{State, injectable::Registry};
pub struct Context {
    state: Option<Box<dyn State>>,
    // Use a reference here if you want the Registry to persist between frames!
    pub registry: Registry,
}

impl Context {
    pub fn new() -> Self {
        Self {
            state: None,
            registry: Registry::new(),
        }
    }

    // Use &mut self so the context isn't destroyed
    pub fn set_state<T: State + 'static>(&mut self, state: T) {
        self.state = Some(Box::new(state));
    }

    // Use &self to peek at state, or keep the existing move if you want to extract it
    pub fn get_state<T: State + 'static>(&self) -> Option<&T> {
        self.state.as_ref()?.as_any().downcast_ref::<T>()
    }

    pub fn clear_state(&mut self) {
        self.state = None;
    }
}
