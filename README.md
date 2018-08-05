# Gabira

[![Build Status](https://travis-ci.org/ersenal/gabira.svg?branch=master)](https://travis-ci.org/ersenal/gabira)
[![Latest version](https://img.shields.io/crates/v/gabira.svg)](https://crates.io/crates/gabira)
[![License](https://img.shields.io/crates/l/gabira.svg)](https://github.com/ersenal/gabira#license)

A rust library for testing HTTP servers. It focuses on ergonomics and brevity.

Documentation:

- [API reference for the latest release](https://docs.rs/gabira/0.1.2)
- [Changelog](CHANGELOG.md)

## Usage

Add the following dependency to your Cargo.toml:

```toml
[dev-dependencies]
gabira = "0.1"
```

and import it in your tests:

```rust
extern crate gabira;

use gabira::*;
```

## [Example](examples)

```rust
get("http://localhost:3000/ping")
  .expect_status(200) // <- Assert http status code
  .expect_body("pong") // <- Assert response body
  .end(); // <- Consume the test. Compile-time warnings are issued if forgotten.
```

Here's an example with actix-web's TestServer:

```rust
let srv = TestServer::with_factory(|| {
  App::new().resource("/login", |r| r.method(Method::POST).with(login))
});

// POST request with json body
post(&srv.url("/login"))
  .send_json(LoginDto {
    username: "...",
    password: "...",
  }).expect_status(200)
  .expect_json(TokenDto {
    token: "...",
  })
  .end();
```

Expectations can be chained:

```rust
let invoice: InvoiceDto = post(&url)
  .set_header("Authorization", &token)
  .set_header("Accept", "application/json")
  .send_json(CreateInvoiceDto { ... })
  .expect_status(200)
  .expect_header("Access-Control-Allow-Origin", "*")
  .expect_header("Access-Control-Allow-Credentials", "true")
  .expect(|r| println!("{}", r.headers().len()))
  .expect(|r| assert!(r.headers().len() > 2))
  .end_json_with(|r: &InvoiceDto| {
    assert!(r.total > 1000);
  });

process_invoice(&invoice);
```

## API

```Rust
get(path: &str)
post(path: &str)
put(path: &str)
delete(path: &str)
  .set_cookie(name: &str, value: &str)
  .set_header(field: &str, value: &str)
  .send(body: Into<Body>)
  .send_json(json: Serialize)
  .send_form(json: Serialize)
  .expect_status(status: u16)
  .expect_cookie(name: &str, value: &str)
  .expect_header(field: &str, value: &str)
  .expect_json(json: Serialize)
  .expect_form(form: Serialize)
  .expect_body(body: Into<Body>)
  .expect(f: FnMut(&ClientResponse))
  .end() -> ClientResponse
  .end_with(f: FnMut(&ClientResponse)) -> ClientResponse
  .end_json<T: DeserializeOwned>() -> T
  .end_json_with<T: DeserializeOwned>(f: FnMut(&T)) -> T
```

## Functionality

- Requests are synchronous
- Expectations (e.g. `expect_status`, `expect_json`) are run in the order of definition

## Limitations

Range of assertions are limited for the moment. See the [documentation](https://docs.rs/gabira/0.1.2/gabira/trait.Expect.html).

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
