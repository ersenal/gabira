extern crate actix_web;
extern crate gabira;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use actix_web::http::Method;
use actix_web::test::TestServer;
use actix_web::{App, Json, Responder};
use gabira::*;

#[derive(Serialize, Deserialize)]
struct CreateCommentDto {
  contents: String,
}

#[derive(Serialize, Deserialize)]
struct CommentDto {
  id: i64,
  contents: String,
}

fn create_comment(dto: Json<CreateCommentDto>) -> impl Responder {
  Json(CommentDto {
    id: 101,
    contents: dto.contents.to_string(),
  })
}

fn main() {
  let srv = TestServer::with_factory(|| {
    App::new().resource("/comment", |r| r.method(Method::POST).with(create_comment))
  });

  post(&srv.url("/comment"))
    .set_header("Authorization", "Bearer mytoken")
    .send_json(CreateCommentDto {
      contents: "hello world".to_string(),
    })
    .expect_status(200)
    .end_json_with(|r: &CommentDto| {
      assert!(r.id > 100);
    });
}
