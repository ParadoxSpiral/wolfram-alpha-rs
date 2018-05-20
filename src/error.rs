// Copyright (c) 2016-2017 Nikita Pekin and the wolfram_alpha_rs contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]

pub type Result<T> = ::std::result::Result<T, Error>;

pub enum Error {
    Deserialization(::serde_xml::Error),
    Reqwest(::reqwest::Error),
    Io(::std::io::Error),
}

impl From<::serde_xml::Error> for Error {
    fn from(other: ::serde_xml::Error) -> Error {
        Error::Deserialization(other)
    }
}

impl From<::reqwest::Error> for Error {
    fn from(other: ::reqwest::Error) -> Error {
        Error::Reqwest(other)
    }
}

impl From<::std::io::Error> for Error {
    fn from(other: ::std::io::Error) -> Error {
        Error::Io(other)
    }
}
