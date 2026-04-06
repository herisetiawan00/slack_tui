use crate::common::Injectable;

mod configuration_local_datasource;

pub use configuration_local_datasource::ConfigurationLocalDatasource;

trait LocalDatasource: Injectable {}
