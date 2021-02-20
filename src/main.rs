use hyper::{
    service::{make_service_fn, service_fn},
    Request, Server
};
use router::Router;
use context::Context;
use std::env;
use std::sync::Arc;
use std::net::SocketAddr;

mod handler;
mod router;
mod context;

type Response = hyper::Response<hyper::Body>;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub state_thing: String,
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
    println!("CTRL-C received ...");
}

#[tokio::main]
async fn main() {
    let some_state = "running".to_string();

    let port = match env::var("PORT") {
        Ok(val) => val.parse::<u16>().unwrap(),
        Err(_e) => 3001,
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let mut router: Router = Router::new();
    router.get("/test", Box::new(handler::test_handler));
    router.post("/send", Box::new(handler::send_handler));
    router.get("/params/:some_param", Box::new(handler::param_handler));

    let shared_router = Arc::new(router);
    let new_service = make_service_fn(move |_| {
        let app_state = AppState {
            state_thing: some_state.clone(),
        };

        let router_capture = shared_router.clone();
        async {
            Ok::<_, Error>(service_fn(move |req| {
                route(router_capture.clone(), req, app_state.clone())
            }))
        }
    });

    /*
    let server = Server::bind(&addr)
        .serve(make_service_fn(|conn: &AddrStream| {
            let remote_addr = conn.remote_addr();
            service_fn_ok(move |_: Request<Body>| {
                Response::new(Body::from(format!("Hello, {}", remote_addr)))
            })
        }))
    */

    let server = Server::bind(&addr).serve(new_service);
    println!("Listening on http://{}", addr);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }

}

async fn route(
    router: Arc<Router>,
    req: Request<hyper::Body>,
    app_state: AppState,
) -> Result<Response, Error> {
    let found_handler = router.route(req.uri().path(), req.method());
    let resp = found_handler
        .handler
        .invoke(Context::new(app_state, req, found_handler.params))
        .await;
    Ok(resp)
}

