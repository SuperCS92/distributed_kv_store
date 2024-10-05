use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use serde_json::value;
use std::convert::Infallible;
use tokio::sync::Mutex;
use std::sync::Arc;
use  kv_store::KVStore;

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

async fn handle_request(
    req: Request<Body>,
    store: Arc<Mutex<KVStore>>
) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        
        // Handle GET request ( e.g., GET /key/:key)
        (&hyper::Method::GET, path) => {
            let key = path.trim_start_matches("/key/");
            let mut store = store.lock().await;
            if let Some(value) = store.get(key) {
                Ok(Response::new(Body::from(value.clone())))
            } else {
                Ok(Response::builder().status(404).body(Body::from("Key not found")).unwrap())
            }
        }

        // Handle POST request (e.g., POST /key to set a value)
        (&hyper::Method::POST, "/key") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let body_str = String::from_utf8(whole_body.to_vec()).unwrap();
            let mut parts: Vec<&str> = body_str.split('=').collect();
            if parts.len() == 2 {
                let key = parts.remove(0).to_string();
                let value = parts.remove(0).to_string();
                let mut store = store.lock().await;
                store.set(key.clone(), value.clone());
                Ok(Response::new(Body::from(format!("Key {} set sucessfully", key))))
            } else {
                Ok(Response::builder().status(400).body(Body::from("Invalid request")).unwrap())
            }   
        }

        // Handle DELETE request (e.g., DELETE /key/:key)
        (&hyper::Method::DELETE, path) => {
            let key = path.trim_start_matches("/key/");
            let mut store = store.lock().await;
            if let Some(_) = store.delete(key) {
                Ok(Response::new(Body::from("Key deleted sucessfully")))
            } else {
                Ok(Response::builder().status(404).body(Body::from("Key not found")).unwrap())
            }
        }

        _ => Ok(Response::builder().status(404).body(Body::from("Not found")).unwrap()),
    }
}