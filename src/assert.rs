extern crate futures;
extern crate serde;
extern crate serde_json;
extern crate serde_urlencoded;

use self::serde::Serialize;
use actix_web::client::ClientResponse;
use actix_web::{Binary, Body, HttpMessage};
use std::boxed::Box;

pub fn create_status_assert(status: u16) -> Box<FnMut(&ClientResponse)> {
  Box::new(move |r| {
    assert_eq!(r.status().as_u16(), status);
  })
}

pub fn create_body_assert<'a>(body: &'a Body) -> Box<FnMut(&ClientResponse) + 'a> {
  Box::new(move |r| assert_body(r, body))
}

pub fn create_cookie_assert<'a>(name: &'a str, value: &'a str) -> Box<FnMut(&ClientResponse) + 'a> {
  Box::new(move |r| {
    assert!(r.cookie(name).is_some(), "cookie {} does not exist", name);
    assert_eq!(r.cookie(name).unwrap().value(), value);
  })
}

pub fn create_json_assert<'a, T: Serialize>(json: &'a T) -> Box<FnMut(&ClientResponse) + 'a> {
  Box::new(move |r| assert_body(r, &Body::from(serde_json::to_string(json).unwrap())))
}

pub fn create_form_assert<'a, T: Serialize>(form: &'a T) -> Box<FnMut(&ClientResponse) + 'a> {
  Box::new(move |r| assert_body(r, &Body::from(serde_urlencoded::to_string(form).unwrap())))
}

pub fn create_header_assert<'a>(
  field: &'a str,
  value: &'a str,
) -> Box<FnMut(&ClientResponse) + 'a> {
  Box::new(move |r| {
    assert!(
      r.headers().contains_key(field),
      "header {} does not exist",
      field
    );
    assert_eq!(r.headers()[field], value);
  })
}

fn assert_body<'a>(r: &ClientResponse, body: &'a Body) {
  use self::futures::future::Future;

  let left = r.body().wait().unwrap();
  match body {
    Body::Binary(body) => match body {
      Binary::Bytes(bytes) => assert_eq!(left, bytes),
      Binary::Slice(slice) => assert_eq!(&left, slice),
      _ => unimplemented!("unsupported binary type"),
    },
    _ => unimplemented!("unsupported body type"),
  }
}
