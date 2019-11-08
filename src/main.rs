extern crate hyper;
extern crate jsonwebtoken as jwt;
#[macro_use] extern crate serde_derive;

use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

use jwt::{encode, decode, Header, Algorithm, Validation};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    sub: String,
    exp: usize,
}

fn main() {
    let addr = ([0, 0, 0, 0], 3000).into();

    let new_svc = || {
        service_fn_ok(hello_world)
    };

    let server = Server::bind(&addr).serve(new_svc).map_err(|e| eprintln!("Server error: {}", e));
    hyper::rt::run(server);
}

fn hello_world(req: Request<Body>) -> Response<Body> {
    let mut response = Response::new(Body::from(""));

    let encoded_token = match req.headers().get("token") {
        Some(t) =>
            match t.to_str() {
                Ok(s) => s,
                Err(_) => {
                    *response.status_mut() = StatusCode::UNAUTHORIZED;
                    return response;
                }
            }
        None => "",
    };

    let token = match decode::<Claims>(encoded_token, "secret".as_ref(), &Validation::default()) {
        Ok(t) => t,
        Err(_) => {
            *response.status_mut() = StatusCode::UNAUTHORIZED;
            return response;
        },
    };

    return response;
}