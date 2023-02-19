#![feature(proc_macro_hygiene, decl_macro)]
use lazy_static::lazy_static;
use tokio::sync::RwLock;
use tokio;

pub mod api;
pub mod blocker;
mod config_struct;
mod core;
pub mod helper;
pub mod hooker;

use config_struct::GlobalConf;

lazy_static! {
    static ref CONF: RwLock<GlobalConf> = RwLock::new(GlobalConf::new());
}

#[tokio::main]
async fn main() {
    CONF.write().await.base_url = "node.testnet.algoexplorerapi.io".to_string();
    // start core
    tokio::task::spawn(core::block_checker());
    core::start().await;
    
}
