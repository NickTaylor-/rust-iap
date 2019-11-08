extern crate hyper;
extern crate jsonwebtoken as jwt;
extern crate pretty_env_logger;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
extern crate futures;

use hyper::{Body, Request, Response, Server, StatusCode, Method};
use hyper::rt::Future;
use hyper::service::service_fn;
use futures::future;

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
struct Claim {
    aud: String,
    sub: String,
    exp: usize,
}

fn main() {
    pretty_env_logger::init_timed();
    let addr = ([0, 0, 0, 0], 3000).into();
    let server = Server::bind(&addr).serve(|| service_fn(route)).map_err(|e| eprintln!("Server error: {}", e));

    hyper::rt::run(server);
}

type FutureResponse = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn route(req: Request<Body>) -> FutureResponse {
    let mut res = Response::new(Body::empty());

    error!("Received {} request for {}", req.method(), req.uri().path());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/authorize") => {
            authorize(req, &mut res);
        },
        (&Method::GET, "/token") => {
            token(req, &mut res);
        },

        _ => {
            *res.status_mut() = StatusCode::NOT_FOUND;
        }
    }

    Box::new(future::ok(res))
}

fn token(_req: Request<Body>, res: &mut Response<Body>) {
    let claims = Claim { aud: "example".to_string(), sub: "unknown".to_string(), exp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize + 600 };
    let token = jwt::encode(&jwt::Header::default(), &claims, "secret".as_ref()).unwrap();
    *res.body_mut() = Body::from(token);
}

fn authorize(req: Request<Body>, res: &mut Response<Body>) {
    let encoded_token = match req.headers().get("token") {
        Some(t) =>
            match t.to_str() {
                Ok(s) => s,
                Err(e) => {
                    error!("{:?}", e);
                    *res.status_mut() = StatusCode::UNAUTHORIZED;
                    return;
                }
            }
        None => "",
    };

    let token = match jwt::decode::<Claim>(encoded_token, "secret".as_ref(), &jwt::Validation::default()) {
        Ok(t) => t,
        Err(e) => {
            error!("{:?}", e);
            *res.status_mut() = StatusCode::UNAUTHORIZED;
            return;
        },
    };
}