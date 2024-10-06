use hyper::{Body, Client, Method, Request};
use hyper::service::{make_service_fn, service_fn};
use tokio::sync::Mutex;
use kv_store::KVStore;
use std::sync::Arc;
use distributed_kv_store::handle_request; // Import handle_request


#[tokio::test]
async fn test_set_get_key() {
    let store = Arc::new(Mutex::new(KVStore::new()));

    //Start an in-memory server for testing
    let make_svc = make_service_fn(move |_conn| {
        let store = store.clone();
        async move {
            Ok::<_, hyper::Error> (service_fn(move |req| {
                let store = store.clone();
                crate::handle_request(req, store)
            }))
        }
    });

    let server = hyper::Server::bind(&([127, 0, 0, 1], 0).into()).serve(make_svc);

    let server_addr = server.local_addr();

    //Spawn the server in the background
    tokio::spawn(server);

    let client = Client::new();
    let req = Request::builder()
        .method(Method::POST)
        .uri(format!("http://{}/key", server_addr))
        .header("content-type", "application/json")
        .body(Body::from(r#"{"key": "key1", "value": "value1"}"#))
        .unwrap();

    let res = client.request(req).await.unwrap();

    assert_eq!(res.status(), 200);

    // Get the key
    let res = client
    .get(format!("http://{}/key/key1", server_addr).parse().unwrap())
    .await
    .unwrap();

    assert_eq!(res.status(), 200);
}