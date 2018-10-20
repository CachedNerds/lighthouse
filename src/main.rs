#[macro_use]
extern crate serde_derive;

extern crate actix_web;
mod routes;

use actix_web::{server, App};

fn main() {
    server::new(|| {
        App::new().resource("/_matrix/client/versions", |r| {
            r.f(routes::version::versions)
        })
    }).bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}
