extern crate serde;

use self::serde::Serialize;
use actix_web::client::ClientResponse;
use actix_web::Body;
use expect_builder::GabiraExpectBuilder;

pub trait Expect<'a> {
  #[must_use]
  fn expect_status(self, status: u16) -> GabiraExpectBuilder<'a>;

  #[must_use]
  fn expect_body<B: Into<Body> + 'a>(self, body: B) -> GabiraExpectBuilder<'a>;

  #[must_use]
  fn expect_json<T: Serialize>(self, json: T) -> GabiraExpectBuilder<'a>;

  #[must_use]
  fn expect_form<T: Serialize>(self, form: T) -> GabiraExpectBuilder<'a>;

  #[must_use]
  fn expect_cookie(self, name: &'a str, value: &'a str) -> GabiraExpectBuilder<'a>;

  #[must_use]
  fn expect_header(self, field: &'a str, value: &'a str) -> GabiraExpectBuilder<'a>;

  #[must_use]
  fn expect<F>(self, f: F) -> GabiraExpectBuilder<'a>
  where
    F: FnMut(&ClientResponse) + 'a;
}
