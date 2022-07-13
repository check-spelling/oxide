use crate::handler::{CommandExecutionError, Request};
use bson::Document;

mod build_info;
mod coll_stats;
mod connection_status;
mod db_stats;
mod drop;
mod find;
mod get_cmd_line_opts;
mod get_parameter;
mod hello;
mod insert;
mod is_master;
mod list_collections;
mod list_databases;
mod ping;
mod whats_my_uri;

pub use self::build_info::BuildInfo;
pub use self::coll_stats::CollStats;
pub use self::connection_status::ConnectionStatus;
pub use self::db_stats::DbStats;
pub use self::drop::Drop;
pub use self::find::Find;
pub use self::get_cmd_line_opts::GetCmdLineOpts;
pub use self::get_parameter::GetParameter;
pub use self::hello::Hello;
pub use self::insert::Insert;
pub use self::is_master::IsMaster;
pub use self::list_collections::ListCollections;
pub use self::list_databases::ListDatabases;
pub use self::ping::Ping;
pub use self::whats_my_uri::WhatsMyUri;

pub trait Handler {
    fn new() -> Self;
    fn handle(
        &self,
        request: &Request,
        msg: &Vec<Document>,
    ) -> Result<Document, CommandExecutionError>;
}
