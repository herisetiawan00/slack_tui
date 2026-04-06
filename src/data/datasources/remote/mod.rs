use std::any::Any;

use crate::common::Injectable;

mod slack_remote_datasource;

pub use slack_remote_datasource::SlackRemoteDatasource;

trait RemoteDatasource: Injectable {
    fn base_url() -> String;
}
