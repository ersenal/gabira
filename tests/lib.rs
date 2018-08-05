extern crate actix_web;
extern crate futures;
extern crate gabira;
#[macro_use(Serialize, Deserialize)]
extern crate serde_derive;

use actix_web::http::Method;
use actix_web::test::TestServer;
use actix_web::{App, HttpMessage, HttpRequest, HttpResponse, Json};
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

fn setup() -> TestServer {
  TestServer::with_factory(|| {
    App::new()
      .resource("/login", |r| {
        r.method(Method::POST).with(|dto: Json<LoginDto>| {
          Json(TokenDto {
            token: format!("{}{}", dto.username, dto.password),
          })
        })
      }).resource("/ping", |r| r.method(Method::GET).f(|_| "pong"))
      .resource("/auth", |r| {
        r.method(Method::POST).f(|req: &HttpRequest| {
          if req.headers().contains_key("Authorization") {
            return HttpResponse::Ok();
          }
          HttpResponse::Unauthorized()
        })
      }).resource("/echo-header", |r| {
        r.method(Method::GET).f(|req: &HttpRequest| {
          let mut resp = HttpResponse::Ok();
          for (key, val) in req.headers() {
            resp.header(key.clone(), val.clone());
          }
          resp
        })
      }).resource("/echo-cookie", |r| {
        r.method(Method::GET).f(|req: &HttpRequest| {
          let mut resp = HttpResponse::Ok();
          for cookie in req.cookies().unwrap().iter() {
            resp.cookie(cookie.clone());
          }
          resp
        })
      })
  })
}

#[test]
fn expect_body() {
  let srv = setup();
  get(&srv.url("/ping")).expect_body("pong").end();
}

#[test]
#[should_panic]
fn expect_body_panic() {
  let srv = setup();
  get(&srv.url("/ping")).expect_body("ping").end();
}

#[test]
fn expect_status() {
  let srv = setup();
  get(&srv.url("/login")).expect_status(404).end();
}

#[test]
#[should_panic]
fn expect_status_panic() {
  let srv = setup();
  get(&srv.url("/login")).expect_status(200).end();
}

#[test]
fn expect_status_2() {
  let srv = setup();
  post(&srv.url("/auth"))
    .set_header("Authorization", "token")
    .expect_status(200)
    .end();
}

#[test]
#[should_panic]
fn expect_status_2_panic() {
  let srv = setup();
  post(&srv.url("/auth")).expect_status(200).end();
}

#[test]
fn expect_header() {
  let srv = setup();
  get(&srv.url("/echo-header"))
    .set_header("head", "1")
    .set_header("head2", "2")
    .expect_header("head", "1")
    .expect_header("head2", "2")
    .end();
}

#[test]
#[should_panic]
fn expect_header_panic() {
  let srv = setup();
  get(&srv.url("/echo-header"))
    .set_header("head", "2")
    .expect_header("head", "1")
    .end();
}

#[test]
fn expect_cookie() {
  let srv = setup();
  get(&srv.url("/echo-cookie"))
    .set_cookie("session", "my-session")
    .expect_cookie("session", "my-session")
    .end();
}

#[test]
#[should_panic]
fn expect_cookie_panic() {
  let srv = setup();
  get(&srv.url("/echo-cookie"))
    .set_cookie("session", "my-session")
    .expect_cookie("session", "my-sessio")
    .end();
}

#[test]
fn expect_json() {
  let srv = setup();

  post(&srv.url("/login"))
    .send_json(LoginDto {
      username: "hello".to_string(),
      password: "world".to_string(),
    }).expect_status(200)
    .expect_json(TokenDto {
      token: "helloworld".to_string(),
    }).end();

  post(&srv.url("/login"))
    .send_json(LoginDto {
      username: "user".to_string(),
      password: "pass".to_string(),
    }).expect_status(200)
    .expect_json(TokenDto {
      token: "userpass".to_string(),
    }).end();
}

#[test]
#[should_panic]
fn expect_json_panic() {
  let srv = setup();

  post(&srv.url("/login"))
    .send_json(LoginDto {
      username: "hello".to_string(),
      password: "world".to_string(),
    }).expect_status(200)
    .expect_json(TokenDto {
      token: "hello world".to_string(),
    }).end();
}
