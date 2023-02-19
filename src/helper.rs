use hyper_openssl::HttpsConnector;
use hyper::{Client, client::HttpConnector};

// new client with httpsconnector
pub fn new_client() -> Client<HttpsConnector<HttpConnector>> {
    let https = HttpsConnector::new().unwrap();
    Client::builder().build::<_, hyper::Body>(https)
}