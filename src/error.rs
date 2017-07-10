// Copyright (c) 2016-2017 Nikita Pekin and the wolfram_alpha_rs contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]

error_chain! {
    foreign_links {
        Deserialization(::serde_xml::Error);
        Reqwest(::reqwest::Error);
        Io(::std::io::Error);
        Utf8(::std::string::FromUtf8Error);
    }
    errors {}
}
