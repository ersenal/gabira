extern crate actix_web;
extern crate gabira;

use actix_web::test::TestServer;
use actix_web::{Body, HttpRequest};
use gabira::*;

fn ping(_: &HttpRequest) -> &'static str {
  "pong"
}

fn main() {
  let srv = TestServer::new(|app| app.handler(ping));
  let expect = Body::from("pong");

  get(&srv.url("/"))
    .expect_status(200)
    .expect_body(&expect)
    .end();
}
