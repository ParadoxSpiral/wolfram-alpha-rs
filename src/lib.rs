// Copyright (c) 2016-2017 Nikita Pekin and the wolfram_alpha_rs contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(missing_docs)]

//! A library providing Rust bindings for the Wolfram|Alpha web API.
//!
//! The library provides a `WolframAlphaRequestSender` trait which can be
//! implemented by various request senders. These implementations may then be
//! used to execute requests to the API.
//!
//! If the `hyper` feature is enabled during compilation, then this library
//! provides an implementation of the `WolframAlphaRequestSender` trait for
//! the `hyper::Client` of the [`hyper`](https://github.com/hyperium/hyper)
//! library.
//!
//! Response bodies are deserialized from XML into structs via the
//! [`serde_xml`](https://github.com/serde-rs/xml) library.

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs as serde_xml;
extern crate url;
extern crate url_serde;

mod error;
pub use error::*;

pub mod model;
pub mod query;
