#[macro_use]
extern crate log;

use futures::{future, Future};

use hyper::{Body, Method, Request, Response, Server};
use hyper::service::service_fn;

mod db;

macro_rules! respond_with_json {
    (status: $status:tt, body: $body:expr) => {
        Response::builder()
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .status(hyper::StatusCode::$status)
            .body(Body::from($body))
            .unwrap()
    };
}

fn call(req: Request<Body>) -> Box<Future<Item=Response<Body>, Error=hyper::Error> + Send> {
    let db_connection = match db::establish_connection() {
        Some(connection) => connection,
        None => {
            return Box::new(future::ok(
                respond_with_json!(
                    status: INTERNAL_SERVER_ERROR,
                    body:r#"{"msg":"Internal server error"}"#
                )
            ))
        }
    };

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            Box::new(future::ok(
                respond_with_json!(status: OK, body:r#"{"hello": "world"}"#)
            ))
        }
        (&Method::POST, "/") => {
            Box::new(future::ok(
                respond_with_json!(status: OK, body:r#"{"hello": "world"}"#)
            ))
        }
        _ => {
            Box::new(future::ok(
                respond_with_json!(status: NOT_FOUND, body:r#"{"msg": "Not found"}"#)
            ))
        }
    }
}

fn main() {
    pretty_env_logger::init();

    let addr = "127.0.0.1:8080".parse().unwrap();

    tokio::run(future::lazy(move || {
        let service = move || {
            service_fn(move |req| {
                call(req)
            })
        };

        let server = Server::bind(&addr)
            .serve(service)
            .map_err(|e| eprintln!("server error: {}", e));

        println!("Server listening on http://{}", addr);

        server
    }));
}
