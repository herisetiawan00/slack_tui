use crate::{
    common::{Config, State, injectable::Registry},
    data::datasources::local::ConfigurationLocalDatasource,
};
pub struct Context {
    state: Option<Box<dyn State>>,
    pub registry: Registry,
    pub config: Config,
}

impl Context {
    pub fn new(registry: Registry, config: Config) -> Self {
        Self {
            state: None,
            registry,
            config,
        }
    }

    pub fn refresh_config(&mut self) -> Option<()> {
        self.config = self
            .registry
            .resolve::<ConfigurationLocalDatasource>()?
            .get();
        return Some(());
    }

    pub fn set_state<T: State + 'static>(&mut self, state: T) {
        self.state = Some(Box::new(state));
    }

    pub fn get_state<T: State + 'static>(&self) -> Option<&T> {
        self.state.as_ref()?.as_any().downcast_ref::<T>()
    }

    pub fn clear_state(&mut self) {
        self.state = None;
    }
}
