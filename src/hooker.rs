use tokio::sync::RwLockReadGuard;
use std::str::FromStr;

use hyper::Uri;

use crate::{blocker::Txns, core::Hooks, helper::new_client};

// a function with two parameters hooks and txns
pub async fn run_hook(hooks: RwLockReadGuard<'_, Hooks>, txns: Txns) {
    // iterate over the hooks,check the type of the hook and check if its in txns,if it is then call the hook
    for (url, hook) in &hooks.hooks {
        let mut checker = false;
        if hook.type_ == "account" {
            for txn in &txns.txns {
                if Some(txn.send.clone()) == hook.account || txn.receiver == hook.account {
                    checker = true;
                }
            }
        } else if hook.type_ == "application" {
            for txn in &txns.txns {
                if txn.app_index == hook.app_index {
                    checker = true;
                }
            }
        }
        if checker {
            println!("Calling hook: {}", url);
            let _ = new_client()
                .get(Uri::from_str(&url).unwrap())
                .await
                .unwrap();
        }
    }
}
