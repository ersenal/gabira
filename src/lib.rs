#[macro_use]
extern crate pretty_assertions;
extern crate actix_web;

mod assert;
mod expect;
pub mod expect_builder;
pub mod request_builder;

pub use self::expect::Expect;
use actix_web::client::ClientRequest;
use actix_web::http;
use request_builder::GabiraRequestBuilder;

fn req(path: &str, method: http::Method) -> GabiraRequestBuilder {
  GabiraRequestBuilder {
    req_builder: {
      let mut builder = ClientRequest::build();
      builder.uri(path).method(method);
      builder
    },
  }
}

#[must_use = "request builder does nothing unless consumed by an expectation builder"]
pub fn get(path: &str) -> GabiraRequestBuilder {
  req(path, http::Method::GET)
}

#[must_use = "request builder does nothing unless consumed by an expectation builder"]
pub fn head(path: &str) -> GabiraRequestBuilder {
  req(path, http::Method::HEAD)
}

#[must_use = "request builder does nothing unless consumed by an expectation builder"]
pub fn post(path: &str) -> GabiraRequestBuilder {
  req(path, http::Method::POST)
}

#[must_use = "request builder does nothing unless consumed by an expectation builder"]
pub fn put(path: &str) -> GabiraRequestBuilder {
  req(path, http::Method::PUT)
}

#[must_use = "request builder does nothing unless consumed by an expectation builder"]
pub fn delete(path: &str) -> GabiraRequestBuilder {
  req(path, http::Method::DELETE)
}

#[must_use = "request builder does nothing unless consumed by an expectation builder"]
pub fn trace(path: &str) -> GabiraRequestBuilder {
  req(path, http::Method::TRACE)
}

#[must_use = "request builder does nothing unless consumed by an expectation builder"]
pub fn options(path: &str) -> GabiraRequestBuilder {
  req(path, http::Method::OPTIONS)
}

#[must_use = "request builder does nothing unless consumed by an expectation builder"]
pub fn connect(path: &str) -> GabiraRequestBuilder {
  req(path, http::Method::CONNECT)
}

#[must_use = "request builder does nothing unless consumed by an expectation builder"]
pub fn patch(path: &str) -> GabiraRequestBuilder {
  req(path, http::Method::PATCH)
}
