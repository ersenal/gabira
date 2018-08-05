extern crate serde;
extern crate tokio;

use self::serde::Serialize;
use self::tokio::runtime::current_thread::Runtime;
use actix_web::client::{ClientRequestBuilder, ClientResponse};
use actix_web::http::Cookie;
use actix_web::Body;
use assert::*;
use expect::Expect;
use expect_builder::GabiraExpectBuilder;
use std::borrow::Cow;
use std::boxed::Box;

pub struct GabiraRequestBuilder {
  pub req_builder: ClientRequestBuilder,
}

impl<'a> GabiraRequestBuilder {
  #[must_use]
  pub fn set_header(mut self, field: &str, value: &str) -> GabiraRequestBuilder {
    self.req_builder.set_header(field, value);
    self
  }

  #[must_use]
  pub fn set_cookie(mut self, name: &str, value: &str) -> GabiraRequestBuilder {
    self.req_builder.cookie(Cookie::new(
      Cow::Borrowed(name).into_owned(),
      Cow::Borrowed(value).into_owned(),
    ));
    self
  }

  #[must_use]
  pub fn send<B: Into<Body>>(mut self, body: B) -> GabiraExpectBuilder<'a> {
    GabiraExpectBuilder {
      req: self.req_builder.body(body).unwrap(),
      expectations: vec![],
      runtime: Runtime::new().unwrap(),
    }
  }

  #[must_use]
  pub fn send_json<T: Serialize>(mut self, json: T) -> GabiraExpectBuilder<'a> {
    GabiraExpectBuilder {
      req: self.req_builder.json(json).unwrap(),
      expectations: vec![],
      runtime: Runtime::new().unwrap(),
    }
  }

  #[must_use]
  pub fn send_form<T: Serialize>(mut self, form: T) -> GabiraExpectBuilder<'a> {
    GabiraExpectBuilder {
      req: self.req_builder.form(form).unwrap(),
      expectations: vec![],
      runtime: Runtime::new().unwrap(),
    }
  }
}

impl<'a> Expect<'a> for GabiraRequestBuilder {
  fn expect<F>(mut self, f: F) -> GabiraExpectBuilder<'a>
  where
    F: FnMut(&ClientResponse) + 'a,
  {
    GabiraExpectBuilder {
      req: self.req_builder.finish().unwrap(),
      expectations: vec![Box::new(f)],
      runtime: Runtime::new().unwrap(),
    }
  }

  fn expect_header(mut self, field: &'a str, value: &'a str) -> GabiraExpectBuilder<'a> {
    GabiraExpectBuilder {
      req: self.req_builder.finish().unwrap(),
      expectations: vec![create_header_assert(field, value)],
      runtime: Runtime::new().unwrap(),
    }
  }

  fn expect_status(mut self, status: u16) -> GabiraExpectBuilder<'a> {
    GabiraExpectBuilder {
      req: self.req_builder.finish().unwrap(),
      expectations: vec![create_status_assert(status)],
      runtime: Runtime::new().unwrap(),
    }
  }

  fn expect_body<B: Into<Body> + 'a>(mut self, body: B) -> GabiraExpectBuilder<'a> {
    GabiraExpectBuilder {
      req: self.req_builder.finish().unwrap(),
      expectations: vec![create_body_assert(body.into())],
      runtime: Runtime::new().unwrap(),
    }
  }

  fn expect_json<T: Serialize>(mut self, json: T) -> GabiraExpectBuilder<'a> {
    GabiraExpectBuilder {
      req: self.req_builder.finish().unwrap(),
      expectations: vec![
        create_header_assert("Content-Type", "application/json"),
        create_json_assert(json),
      ],
      runtime: Runtime::new().unwrap(),
    }
  }

  fn expect_form<T: Serialize>(mut self, form: T) -> GabiraExpectBuilder<'a> {
    GabiraExpectBuilder {
      req: self.req_builder.finish().unwrap(),
      expectations: vec![
        create_header_assert("Content-Type", "application/x-www-form-urlencoded"),
        create_form_assert(form),
      ],
      runtime: Runtime::new().unwrap(),
    }
  }

  fn expect_cookie(mut self, name: &'a str, value: &'a str) -> GabiraExpectBuilder<'a> {
    GabiraExpectBuilder {
      req: self.req_builder.finish().unwrap(),
      expectations: vec![create_cookie_assert(name, value)],
      runtime: Runtime::new().unwrap(),
    }
  }
}
