extern crate actix_web;
extern crate gabira;
#[macro_use(Serialize, Deserialize)]
extern crate serde_derive;

use actix_web::http::Method;
use actix_web::test::TestServer;
use actix_web::{App, Json, Responder};
use gabira::*;

#[derive(Serialize, Deserialize)]
struct LoginDto {
  username: String,
  password: String,
}

#[derive(Serialize)]
struct TokenDto {
  token: String,
}

fn login(dto: Json<LoginDto>) -> impl Responder {
  Json(TokenDto {
    token: format!("{}{}", dto.username, dto.password),
  })
}

fn main() {
  let srv = TestServer::with_factory(|| {
    App::new().resource("/login", |r| r.method(Method::POST).with(login))
  });

  let expect = TokenDto {
    token: "helloworld".to_string(),
  };

  post(&srv.url("/login"))
    .send_json(LoginDto {
      username: "hello".to_string(),
      password: "world".to_string(),
    }).expect_status(200)
    .expect_json(&expect)
    .end();
}
