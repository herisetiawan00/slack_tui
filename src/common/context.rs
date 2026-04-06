use crate::{
    common::{Config, State, injectable::Registry},
    data::{
        datasources::local::ConfigurationLocalDatasource,
        entities::authorization::AuthorizationEntity,
    },
};
pub struct Context {
    state: Option<Box<dyn State>>,
    authorization: Option<AuthorizationEntity>,
    pub registry: Registry,
    pub config: Config,
}

impl Context {
    pub fn new(registry: Registry, config: Config) -> Self {
        Self {
            state: None,
            authorization: None,
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

    pub fn get_auth(&self) -> AuthorizationEntity {
        self.authorization
            .clone()
            .expect("User should be authorized")
    }

    pub fn set_auth(&mut self, authorization: AuthorizationEntity) {
        self.authorization = Some(authorization);
    }
}
