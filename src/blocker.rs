use crate::helper::new_client;
use crate::CONF;
use hyper::Uri;
use serde::Deserialize;
use serde_json::Value;

// Txns struct
#[derive(Debug, Deserialize)]
pub struct Txns {
    pub txns: Vec<Txn>,
}
// impl Txns new and add function
impl Txns {
    pub fn new() -> Self {
        Self { txns: Vec::new() }
    }
    pub fn add(&mut self, txn: Txn) {
        self.txns.push(txn);
    }
}
// Txn struct
#[derive(Debug, Deserialize)]
pub struct Txn {
    pub send: String,
    pub type_: String,
    // when type is payment, have receiver,
    // when type is ApplicationCall, have app-index
    pub receiver: Option<String>,
    pub app_index: Option<u64>,
    pub amount: u64,
}

pub async fn new_block(round: u128) -> Txns {
    // wait for new block
    println!("checking new block: {}", round);
    let _ = new_client()
        .get(
            Uri::builder()
                .authority(CONF.read().await.base_url.clone())
                .scheme("https")
                .path_and_query(format!("/v2/status/wait-for-block-after/{}", round))
                .build()
                .unwrap(),
        )
        .await
        .unwrap();
    // using Client to get block transactions.
    let r = new_client()
        .get(
            Uri::builder()
                .authority(CONF.read().await.base_url.clone())
                .scheme("https")
                .path_and_query(format!("/v2/blocks/{}", round))
                .build()
                .unwrap(),
        )
        .await
        .unwrap();
    // parse result to json
    let body = r.into_parts().1;
    let txns: Value = serde_json::from_slice(&hyper::body::to_bytes(body).await.unwrap()).unwrap();
    // parse json to Txns struct
    let origin: Value = serde_json::from_value(txns["block"]["txns"].clone()).unwrap();
    //  make a new txns
    let mut txns = Txns::new();
    // iterate origin txns
    if let Some(origin_txns) = origin.as_array() {
        for origin_txn in origin_txns {
            // parse json to Txn struct
            // make a new txn
            let mut txn = Txn {
                send: String::new(),
                type_: String::new(),
                receiver: None,
                app_index: None,
                amount: 0,
            };
            txn.send = origin_txn["txn"]["snd"].as_str().unwrap().to_string();
            txn.type_ = origin_txn["txn"]["type"].as_str().unwrap().to_string();
            if txn.type_ == "pay" {
                txn.receiver = Some(origin_txn["txn"]["rcv"].as_str().unwrap().to_string());
                txn.amount = origin_txn["txn"]["amt"].as_u64().unwrap();
            } else if txn.type_ == "appl" {
                txn.app_index = Some(origin_txn["txn"]["apid"].as_u64().unwrap());
            }
            txns.add(txn);
        }
    }
    return txns;
}

pub async fn get_newest_block() -> u128 {
    let v: Value = serde_json::from_slice(
        &hyper::body::to_bytes(
            new_client()
                .get(
                    Uri::builder()
                        .authority(CONF.read().await.base_url.clone())
                        .scheme("https")
                        .path_and_query("/v2/status")
                        .build()
                        .unwrap(),
                )
                .await
                .unwrap()
                .into_parts()
                .1,
        )
        .await
        .unwrap(),
    )
    .unwrap();
    v["last-round"].as_u64().unwrap() as u128
}

// test for new_block
#[tokio::test]
async fn test_new_block() {
    CONF.write().await.base_url = "node.testnet.algoexplorerapi.io".to_string();
    // print test conf
    println!("{:?}", CONF.read().await.base_url);
    let round = 27853135;
    let uri_test = Uri::builder()
        .authority(CONF.read().await.base_url.clone())
        .scheme("https")
        .path_and_query(format!("/v2/status/wait-for-block-after/{}", round))
        .build()
        .unwrap();
    // print test uri
    println!("{:?}", uri_test);

    let txns = new_block(round).await;
    println!("{:?}", txns);
}
