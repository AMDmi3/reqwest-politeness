# reqwest-politeness

[![Build status](https://github.com/AMDmi3/reqwest-politeness/actions/workflows/ci.yml/badge.svg)](https://github.com/AMDmi3/reqwest-politeness/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/reqwest-politeness)](https://crates.io/crates/reqwest-politeness)
[![Documentation](https://docs.rs/reqwest-politeness/badge.svg)](https://docs.rs/reqwest-politeness)

This crate provides `reqwest` middleware which enforces delay between
requests to a single host.

```rust
use reqwest_politeness::ReqwestPoliteness;

let middleware = ReqwestPoliteness::default()
    .with_delay(Duration::from_secs(1));

// Base clent
let client = reqwest::Client::builder()
    .build()?;

// With added middleware
let client = reqwest_middleware::ClientBuilder::new(client)
    .with(middleware)
    .build();

client.get("https://foo.com/").send().await?;
client.get("https://foo.com/").send().await?; // waits for 1 second
client.get("https://bar.com/").send().await?; // does not wait, other host
```

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

## Supported Rust versions

This crate supports current stable Rust version and 2 most recent minor releases before it.
Increasing MSRV is not considered a semver breaking change as long as it follows this policy.
The current MSRV is 1.88.

## Documentation

See https://docs.rs/reqwest-politeness/latest/reqwest-politeness/ for complete documentation.

## Author

* [Dmitry Marakasov](https://github.com/AMDmi3) <amdmi3@amdmi3.ru>

## License

* [MIT](COPYING-MIT) OR [Apache-2.0](COPYING-APACHE)
