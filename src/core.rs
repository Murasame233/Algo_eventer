// core
// this conatins the core logic

use tokio::time::{sleep, Duration};

use crate::api::*;
use crate::blocker::*;
use crate::hooker::run_hook;
use lazy_static::lazy_static;
use rocket::{self, routes};
use std::collections::HashMap;
use tokio::sync::RwLock;

// hooks and targets struct
// this conatins the hooks and targets struct
pub struct Hooks {
    // URL, hooktarget
    pub hooks: HashMap<String, HookTarget>,
}
impl Hooks {
    fn new() -> Self {
        Self {
            hooks: HashMap::new(),
        }
    }
    fn add(&mut self, url: String, hook: HookTarget) {
        self.hooks.insert(url, hook);
    }
}

pub struct HookTarget {
    // Account or Application
    pub type_: String,
    // Account or Application index
    pub app_index: Option<u64>,
    pub account: Option<String>,
}
impl HookTarget {
    pub fn new(type_: String, app_index: Option<u64>, account: Option<String>) -> Self {
        Self {
            type_,
            app_index,
            account,
        }
    }
}

lazy_static! {
    static ref HOOKS: RwLock<Hooks> = RwLock::new(Hooks::new());
}

pub async fn add_hooks(target: String, hook: HookTarget) {
    let mut hooks = HOOKS.write().await;
    hooks.add(target, hook);
}

pub async fn delete_hooks(uri: String) {
    let mut hooks = HOOKS.write().await;
    hooks.hooks.remove(&uri);
}

//  fn start.
pub async fn start() {
    //  mount api to rocket
    let r = rocket::build().mount("/", routes![new_api, delete_api]);
    let _ = r.launch().await.unwrap();
}

pub async fn block_checker() {
    let mut newest_block = get_newest_block().await;
    loop {
        sleep(Duration::from_secs(1)).await;
        let txns = new_block(newest_block + 1).await;
        newest_block += 1;
        tokio::task::spawn(run_hook(HOOKS.read().await, txns)).await.unwrap();
    }
}
