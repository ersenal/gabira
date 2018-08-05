extern crate actix_web;
extern crate gabira;

use actix_web::test::TestServer;
use actix_web::HttpRequest;
use gabira::*;

fn ping(_: &HttpRequest) -> &'static str {
  "pong"
}

fn main() {
  let srv = TestServer::new(|app| app.handler(ping));

  get(&srv.url("/"))
    .expect_status(200)
    .expect_body("pong")
    .end();
}
