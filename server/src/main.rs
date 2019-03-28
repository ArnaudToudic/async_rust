#[macro_use]
extern crate diesel;
#[macro_use]
extern crate juniper;

use futures::{future, Future};

use hyper::{Body, Method, Request, Response, Server};
use hyper::service::service_fn;

use std::sync::Arc;

mod db;
mod graphql;
mod model;
mod schema;

macro_rules! respond_with_json {
    (status: $status:tt) => {
        Response::builder()
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .status(hyper::StatusCode::$status)
            .body(Body::empty())
            .unwrap()
    };
}

fn call(req: Request<Body>) -> Box<Future<Item=Response<Body>, Error=hyper::Error> + Send> {
    let pool = db::init_pool();
    let db = match db::establish_connection(&pool) {
        Some(connection) => connection,
        None => {
            return Box::new(future::ok(
                respond_with_json!(status: INTERNAL_SERVER_ERROR)
            ))
        }
    };

    let ctx = Arc::new(graphql::Context {
        db_conn: db
    });
    let root_node = Arc::new(graphql::create_schema());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Box::new(juniper_hyper::graphiql("/graphql")),
        //(&Method::GET, "/graphql") => Box::new(juniper_hyper::graphql(root_node, ctx, req)),
        //(&Method::POST, "/graphql") => Box::new(juniper_hyper::graphql(root_node, ctx, req)),
        _ => {
            Box::new(future::ok(
                respond_with_json!(status: NOT_FOUND)
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
