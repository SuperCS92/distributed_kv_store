use hyper::{Body, Request, Response};
use std::convert::Infallible;
use tokio::sync::Mutex;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json::json;

use kv_store::KVStore;

#[derive(Serialize, Deserialize)]
struct KeyValue {
    key: String,
    value: String,
}

pub async fn handle_request(
    req: Request<Body>,
    store: Arc<Mutex<KVStore>>
) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        
        // Handle GET request ( e.g., GET /key/:key)
        (&hyper::Method::GET, path) => {
            let key = path.trim_start_matches("/key/");
            let mut store = store.lock().await;
            if let Some(value) = store.get(key) {
                let response_json = json!({"key": key, "value": value});
                Ok(Response::new(Body::from(response_json.to_string())))
            } else {
                Ok(Response::builder().status(404).body(Body::from("Key not found")).unwrap())
            }
        }

        // Handle POST request (e.g., POST /key to set a value)
        (&hyper::Method::POST, "/key") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let key_value: KeyValue = serde_json::from_slice(&whole_body).unwrap();
            let mut store = store.lock().await;
            store.set(key_value.key.clone(), key_value.value.clone());
            let response_json = json!({"message": "Key set susccessfully"});
            Ok(Response::new(Body::from(response_json.to_string())))
        }


        // Handle DELETE request (e.g., DELETE /key/:key)
        (&hyper::Method::DELETE, path) => {
            let key = path.trim_start_matches("/key/");
            let mut store = store.lock().await;
            if let Some(_) = store.delete(key) {
                let response_json = json!({"message": "Key deleted successfully"});
                Ok(Response::new(Body::from(response_json.to_string())))
            } else {
                Ok(Response::builder().status(404).body(Body::from("Key not found")).unwrap())
            }
        }

        _ => Ok(Response::builder().status(404).body(Body::from("Not found")).unwrap()),
    }
}