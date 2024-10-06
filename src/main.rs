use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use serde_json::json;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use tokio::sync::Mutex;
use std::sync::Arc;
use  kv_store::KVStore;
use distributed_kv_store::handle_request;



#[tokio::main]
async fn main() {
    //Initialize an in-memory KVStore and wrap it in an Arc<Mutex<>>
    let store = Arc::new(Mutex::new(KVStore::new()));

    // Define the address and port to listen on
    let addr = ([127, 0, 0, 1], 3000).into();

    let make_svc = make_service_fn(move |_conn| {
        let store = store.clone();

        // Create a service that handles incoming requests
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let store = store.clone();
                handle_request(req, store)
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);
    println!("Listening on http://{}", addr);

    //Run the server and catch any errors
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

