extern crate serde;
extern crate serde_json;
extern crate tokio;

use self::serde::de::DeserializeOwned;
use self::serde::Serialize;
use self::tokio::runtime::current_thread::Runtime;
use actix_web::client::{ClientRequest, ClientResponse};
use actix_web::{Body, HttpMessage};
use assert::*;
use expect::Expect;

pub struct GabiraExpectBuilder<'a> {
  pub req: ClientRequest,
  pub runtime: Runtime,
  pub expectations: Vec<Box<FnMut(&ClientResponse) + 'a>>,
}

impl<'a> GabiraExpectBuilder<'a> {
  pub fn end(self) -> ClientResponse {
    self.end_with(|_| ())
  }

  pub fn end_json<T: DeserializeOwned>(mut self) -> T {
    let response = self.runtime.block_on(self.req.send()).unwrap();
    for mut i in self.expectations {
      i(&response);
    }
    let body = self.runtime.block_on(response.body()).unwrap();
    serde_json::from_slice(&body).unwrap()
  }

  pub fn end_with<F>(mut self, f: F) -> ClientResponse
  where
    F: FnOnce(&ClientResponse) + 'a,
  {
    let response = self.runtime.block_on(self.req.send()).unwrap();
    for mut i in self.expectations {
      i(&response);
    }
    f(&response);
    response
  }

  pub fn end_json_with<T: DeserializeOwned, F>(mut self, f: F) -> T
  where
    F: FnOnce(&T) + 'a,
  {
    let response = self.runtime.block_on(self.req.send()).unwrap();
    for mut i in self.expectations {
      i(&response);
    }
    let body = self.runtime.block_on(response.body()).unwrap();
    let json: T = serde_json::from_slice(&body).unwrap();
    f(&json);
    json
  }
}

impl<'a> Expect<'a> for GabiraExpectBuilder<'a> {
  fn expect<F>(mut self, f: F) -> GabiraExpectBuilder<'a>
  where
    F: FnMut(&ClientResponse) + 'a,
  {
    self.expectations.push(Box::new(f));
    self
  }

  fn expect_header(mut self, field: &'a str, value: &'a str) -> GabiraExpectBuilder<'a> {
    self.expectations.push(create_header_assert(field, value));
    self
  }

  fn expect_status(mut self, status: u16) -> GabiraExpectBuilder<'a> {
    self.expectations.push(create_status_assert(status));
    self
  }

  fn expect_body(mut self, body: &'a Body) -> GabiraExpectBuilder<'a> {
    self.expectations.push(create_body_assert(body));
    self
  }

  fn expect_json<T: Serialize>(mut self, json: &'a T) -> GabiraExpectBuilder<'a> {
    self
      .expectations
      .push(create_header_assert("Content-Type", "application/json"));
    self.expectations.push(create_json_assert(json));
    self
  }

  fn expect_form<T: Serialize>(mut self, form: &'a T) -> GabiraExpectBuilder<'a> {
    self.expectations.push(create_header_assert(
      "Content-Type",
      "application/x-www-form-urlencoded",
    ));
    self.expectations.push(create_form_assert(form));
    self
  }

  fn expect_cookie(mut self, name: &'a str, value: &'a str) -> GabiraExpectBuilder<'a> {
    self.expectations.push(create_cookie_assert(name, value));
    self
  }
}
