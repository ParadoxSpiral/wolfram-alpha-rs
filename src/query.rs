// Copyright (c) 2016-2017 Nikita Pekin and the wolfram_alpha_rs contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Performs a query to Wolfram|Alpha.
//!
//! For more information, see [Wolfram|Alpha's API
//! documentation](http://products.wolframalpha.com/api/documentation.html).


use error::Result;
use model::QueryResult;
use reqwest::Client;
use serde_xml;
use std::collections::HashMap;
use std::io::Read;
use url::Url;

/// A container struct for the parameters for a query to the Wolfram|Alpha API.
// TODO: replace these with concrete types.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryParameters<'a> {
    pub format: Option<&'a str>,
    pub includepodid: Option<&'a str>,
    pub excludepodid: Option<&'a str>,
    pub podtitle: Option<&'a str>,
    pub podindex: Option<&'a str>,
    pub scanner: Option<&'a str>,
    pub async: Option<&'a str>,
    pub ip: Option<&'a str>,
    pub latlong: Option<&'a str>,
    pub location: Option<&'a str>,
    pub assumption: Option<&'a str>,
    pub podstate: Option<&'a str>,
    pub units: Option<&'a str>,
    pub width: Option<&'a str>,
    pub maxwidth: Option<&'a str>,
    pub plotwidth: Option<&'a str>,
    pub mag: Option<&'a str>,
    pub scantimeout: Option<&'a str>,
    pub podtimeout: Option<&'a str>,
    pub formattimeout: Option<&'a str>,
    pub parsetimeout: Option<&'a str>,
    pub totaltimeout: Option<&'a str>,
    pub reinterpret: Option<&'a str>,
    pub translation: Option<&'a str>,
    pub ignorecase: Option<&'a str>,
    pub sig: Option<&'a str>,
}

impl<'a> Default for QueryParameters<'a> {
    fn default() -> QueryParameters<'a> {
        QueryParameters{
            format: Some("plaintext"),
            includepodid: None,
            excludepodid: None,
            podtitle: None,
            podindex: None,
            scanner: None,
            async: Some("false"),
            ip: None,
            latlong: None,
            location: None,
            assumption: None,
            podstate: None,
            units: Some("metric"),
            width: None,
            maxwidth: None,
            plotwidth: None,
            mag: None,
            scantimeout: None,
            podtimeout: None,
            formattimeout: None,
            parsetimeout: None,
            totaltimeout: None,
            reinterpret: None,
            translation: None,
            ignorecase: Some("true"),
            sig: None,
        }
    }
}

/// Performs a query to the Wolfram|Alpha API.
pub fn query(
    client: Option<&Client>,
    appid: &str,
    input: &str,
    optional_query_parameters: Option<QueryParameters>,
) -> Result<QueryResult> {
    let mut params = HashMap::new();
    params.insert("input", input);
    params.insert("appid", appid);

    // If present, we insert the optional parameters.
    if let Some(v) = optional_query_parameters {
        for &(name, value) in &[
            ("format", v.format),
            ("includepodid", v.includepodid),
            ("excludepodid", v.excludepodid),
            ("podtitle", v.podtitle),
            ("podindex", v.podindex),
            ("scanner", v.scanner),
            ("async", v.async),
            ("ip", v.ip),
            ("latlong", v.latlong),
            ("location", v.location),
            ("assumption", v.assumption),
            ("podstate", v.podstate),
            ("units", v.units),
            ("width", v.width),
            ("maxwidth", v.maxwidth),
            ("plotwidth", v.plotwidth),
            ("mag", v.mag),
            ("scantimeout", v.scantimeout),
            ("podtimeout", v.podtimeout),
            ("formattimeout", v.formattimeout),
            ("parsetimeout", v.parsetimeout),
            ("totaltimeout", v.totaltimeout),
            ("reinterpret", v.reinterpret),
            ("translation", v.translation),
            ("ignorecase", v.ignorecase),
            ("sig", v.sig),
        ] {
            if let Some(value) = value {
                params.insert(name, value);
            }
        }
    }

    parse_wolfram_alpha_response(&if let Some(client) = client {
        send_authed(client, "query", &mut params)?
    } else {
        send_authed(&Client::new(), "query", &mut params)?
    })
}

fn send_authed<'a>(
    client: &Client,
    method: &str,
    params: &mut HashMap<&str, &'a str>,
) -> Result<String> {
    let url_string = format!("https://api.wolframalpha.com/v2/{}", method);
    let mut url = url_string.parse::<Url>().expect("Unable to parse URL");
    url.query_pairs_mut().extend_pairs(params.into_iter());

    trace!("Sending query \"{:?}\" to url: {}", params, url);
    let mut response = client.get(url).send()?;
    let mut result = String::new();
    response.read_to_string(&mut result)?;
    trace!("Query result: {}", result);

    Ok(result)
}

fn parse_wolfram_alpha_response(ser: &str) -> Result<QueryResult> {
    let parsed_response = serde_xml::from_str(ser)?;
    trace!("Parsed response: {:?}", parsed_response);
    Ok(parsed_response)
}
