extern crate hyper;
extern crate jsonwebtoken as jwt;
#[macro_use] extern crate serde_derive;
extern crate futures;

use hyper::{Body, Request, Response, Server, StatusCode, Method};
use hyper::rt::Future;
use hyper::service::service_fn;
use futures::future;

use jwt::{encode, decode, Header, Algorithm, Validation};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    sub: String,
    exp: usize,
}

fn main() {
    let addr = ([0, 0, 0, 0], 3000).into();
    let server = Server::bind(&addr).serve(|| service_fn(route)).map_err(|e| eprintln!("Server error: {}", e));

    hyper::rt::run(server);
}

type FutureResponse = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn route(req: Request<Body>) -> FutureResponse {
    let mut res = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/authorize") => {
            authorize(req, &mut res);
        }

        _ => {
            *res.status_mut() = StatusCode::NOT_FOUND;
        }
    }

    Box::new(future::ok(res))
}

fn authorize(req: Request<Body>, res: &mut Response<Body>) {
    let encoded_token = match req.headers().get("token") {
        Some(t) =>
            match t.to_str() {
                Ok(s) => s,
                Err(_) => {
                    *res.status_mut() = StatusCode::UNAUTHORIZED;
                    return;
                }
            }
        None => "",
    };

    let token = match decode::<Claims>(encoded_token, "secret".as_ref(), &Validation::default()) {
        Ok(t) => t,
        Err(_) => {
            *res.status_mut() = StatusCode::UNAUTHORIZED;
            return;
        },
    };
}