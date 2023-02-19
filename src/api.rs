use rocket::post;
use serde_json::Value;

use crate::core::{add_hooks, delete_hooks, HookTarget};

// api, using rocket.
// Path: /v1/new, args:target, type, app_index, account
#[post("/v1/new", data = "<new>")]
pub async fn new_api(new: String) -> String {
    // parse json
    let new: Value = serde_json::from_str(&new).unwrap();
    let target = new["target"].as_str().unwrap();
    let type_ = new["type"].as_str().unwrap();
    let app_index = new["app_index"].as_u64();
    let account = if new["account"].as_str().is_none() {
        None
    } else {
        Some(new["account"].as_str().unwrap().to_string())
    };
    add_hooks(
        target.to_string(),
        HookTarget::new(type_.to_string(), app_index, account),
    ).await;
    println!("Add {}", target);
    format!("ok")
}

// Path: /v1/delete, args:target
#[post("/v1/delete", data = "<delete>")]
pub async fn delete_api(delete: String) -> String {
    // parse json
    let delete: Value = serde_json::from_str(&delete).unwrap();
    let target = delete["target"].as_str().unwrap();
    delete_hooks(target.to_string()).await;
    println!("Del {}", target);
    format!("ok")
}
