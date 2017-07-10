// Copyright (c) 2016-2017 Nikita Pekin and the wolfram_alpha_rs contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]

//! Struct and enum definitions of values in the Wolfram|Alpha model.
//!
//! For more information, see [Wolfram|Alpha's API
//! documentation](http://products.wolframalpha.com/api/documentation.html).

use url::Url;
use url_serde;

/// `QueryResult` is the outer wrapper for all results from the query function.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct QueryResult {
    pub success: bool,
    //pub error: bool,
    pub numpods: u32,
    pub version: Option<String>, // TODO: replace this with a better type.
    pub datatypes: String, // TODO: possibly replace this with an enum?
    pub timing: f64,
    pub timedout: String,
    pub timedoutpods: Option<String>,
    pub parsetiming: f64,
    pub parsetimedout: Option<bool>,
    pub recalculate: Option<String>,
    pub id: Option<String>,
    pub host: Option<String>,
    pub server: Option<u32>,
    pub related: Option<String>,
    #[serde(rename = "pod")]
    pub pods: Option<Vec<Pod>>,
    pub assumptions: Option<Assumptions>,
    pub sources: Option<Sources>,
    //pub error: Option<Error>,
    pub tips: Option<Tips>,
    pub didyoumeans: Option<DidYouMeans>,
    pub languagemsg: Option<LanguageMsg>,
    pub futuretopic: Option<FutureTopic>,
    pub relatedexamples: Option<RelatedExamples>,
    pub examplepage: Option<ExamplePage>,
    pub generalization: Option<Generalization>,
    pub warnings: Option<Warnings>,
}

/// A series of `Assumption` elements.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Assumptions {
    pub count: u32,
    pub assumption: Vec<Assumption>,
}

/// A single assumption, typically about the meaning of a word or phrase.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Assumption {
    #[serde(rename = "type")]
    pub assumption_type: String,
    pub desc: Option<String>,
    pub word: Option<String>,
    pub template: Option<String>,
    pub current: Option<u32>,
    pub count: u32,
    pub value: Vec<Value>,
}

/// A series of `Tip` elements.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Tips {
    pub count: u32,
    pub tip: Vec<Tip>,
}

/// A single tip, typically regarding suggestions for fixing the query.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Tip {
    pub text: String,
}

/// A series of `DidYouMean` elements.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct DidYouMeans {
    pub count: u32,
    pub didyoumean: Vec<DidYouMean>,
}

/// Provides a suggestion for a different query than the one provided.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct DidYouMean {
    pub score: String,
    pub level: String,
    #[serde(rename = "$value")]
    pub value: String,
}

/// Generated when a foreign language is detected in a query.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct LanguageMsg {
    pub english: String,
    pub other: Option<String>,
}

/// Provides information for queries regarding a topic under development.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct FutureTopic {
    pub topic: String,
    pub msg: String,
}

/// A series of `RelatedExample` elements.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RelatedExamples {
    pub count: u32,
    #[serde(rename = "relatedexample")]
    pub related_example: Vec<RelatedExample>,
}

/// Provides a link to an HTML page of related examples on the requested topic.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RelatedExample {
    pub input: String,
    pub desc: String,
    pub category: String,
    #[serde(with = "url_serde")]
    pub categorythumb: Url,
    #[serde(with = "url_serde")]
    pub categorypage: Url,
}

/// Provides a link to an HTML page of sample queries on the requested topic.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ExamplePage {
    pub category: String,
    #[serde(with = "url_serde")]
    pub url: Url,
}

/// A series of `Warning` elements.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Warnings {
    pub count: u32,
    // TODO: find a way to merge these into an enum?
    pub spellcheck: Option<Vec<Spellcheck>>,
    pub delimeters: Option<Vec<Delimeters>>,
    pub translation: Option<Vec<Translation>>,
    pub reinterpret: Option<Vec<Reinterpret>>,
}

/// Provides word and suggestion attributes as alternative spellings for a word
/// in the query.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Spellcheck {
    word: String,
    suggestion: String,
    text: String,
}

/// Represents a warning regarding mismatched delimiters in a query.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Delimeters {
    text: String,
}

/// Represents a word or a phrase which was translated in the query.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Translation {
    phrase: String,
    trans: String,
    lang: String,
    text: String,
}

/// Represents a warning that the query was reinterpred.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Reinterpret {
    text: String,
    alternative: Vec<Alternative>,
}

/// An alternative interpretation of an element in a query.
pub type Alternative = String;

/// Provides a link to a generalization of the requested query.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Generalization {
    pub topic: String,
    pub desc: String,
    #[serde(with = "url_serde")]
    pub url: Url,
}

/// A series of `Source` elements.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Sources {
    pub count: u32,
    pub source: Vec<Source>,
}

/// A link to a web page of source information.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Source {
    pub url: String,
    pub text: String,
}

/// The value of an assumption.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Value {
    pub name: String,
    pub desc: String,
    pub input: String,
    pub word: Option<String>,
}

/// `Pod` elements are subelements of `QueryResult`. Each contains the results
/// for a single pod.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Pod {
    pub title: String,
    pub error: bool,
    pub position: u32,
    pub scanner: String,
    pub id: String,
    pub numsubpods: u32,
    pub primary: Option<bool>,
    #[serde(rename="subpod")]
    pub subpods: Vec<SubPod>,
    //pub states: Option<States>,
    pub infos: Option<Infos>,
    // TODO: find a way to parse errors.
    // The naive method (uncommenting the following line) doesn't work as `Pod`
    // also contains an `error` attribute.
    // pub error: Option<Error>,
    pub sounds: Option<Sounds>,
    pub definitions: Option<Definitions>,
    pub notes: Option<Notes>,
}


/// A series of `State` attributes.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct States {
    pub count: u32,
    //pub state: Vec<State>,
    //pub statelist: Option<Vec<Statelist>>,
}

/// A series of `State` attributes, combined into a list.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Statelist {
    pub count: u32,
    pub value: String,
    //pub state: Vec<State>,
}

/// An alternative state which is available to a `Pod` or `Subpod`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct State {
    pub name: String,
    pub input: String,
}

/// A series of `Info` elements.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Infos {
    pub count: u32,
    pub info: Vec<Info>,
}

/// A container for extra information regarding the contents of a pod.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Info {
    pub text: Option<String>,
    pub img: Option<Vec<Img>>,
    pub link: Option<Vec<Link>>,
    pub units: Option<Vec<Units>>,
}

/// A series of `Sound` elements.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Sounds {
    pub count: u32,
    pub sound: Vec<Sound>,
}

/// Provides a link to a playable sound file.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Sound {
    #[serde(with = "url_serde")]
    pub url: Url,
    #[serde(rename = "type")]
    pub mimetype: String, // TODO: replace this with a Mimetype type.
}

/// A series of `Definition` attributes.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Definitions {
    pub count: u32,
    pub definition: Vec<Definition>,
}

/// An element containing a definition of a concept used in the query result.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Definition {
    pub word: String,
    pub desc: String,
}

/// A series of `Note` attributes.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Notes {
    pub count: u32,
    pub note: Vec<String>,
}

/// A subelement of `Pod`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct SubPod {
    pub title: String,
    pub img: Option<Img>,
    pub plaintext: Option<String>,
    //pub states: Option<States>,
}

/// An HTML img element suitable for direct inclusion in a web page.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Img {
    #[serde(with = "url_serde")]
    pub src: Url,
    pub alt: Option<String>,
    pub title: Option<String>,
    pub width: u32,
    pub height: u32,
}

/// An HTML image map.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ImageMap {
    rect: Vec<Rect>,
}

/// A rectangular section of an HTML image map.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Rect {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub query: String,
    pub assumptions: String,
    pub title: String,
}

/// A series of `Unit` elements.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Units {
    pub count: u32,
    pub unit: Vec<Unit>,
    pub img: Img,
}

/// A table of unit abbreviations used inside a `Pod`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Unit {
    pub short: String,
    pub long: String,
}

/// A standard link of some text pointing to a URL.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Link {
    pub url: String,
    pub text: String,
    pub title: Option<String>,
}

/// `Error` elements provide a code and a description of an error which has
/// occured while performing a query to the API.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Error {
    pub code: u32,
    pub msg: String,
}

#[cfg(test)]
mod tests {
    use super::{Definitions, DidYouMean, DidYouMeans, Error, Img, Infos, LanguageMsg, Notes,
                Plaintext, Pod, QueryResult, Spellcheck, State, Statelist, States, Subpod, Tips,
                Warnings};
    use serde_xml::deserialize;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    fn read_sample_data_from_path<P>(path: P) -> String
    where
        P: AsRef<Path>,
    {
        let mut file = File::open(path).unwrap();
        let mut body = String::new();
        file.read_to_string(&mut body).unwrap();
        body
    }

    #[test]
    fn test_query_result_deserializer() {
        let _: QueryResult = deserialize(
            read_sample_data_from_path("tests/sample-data/query_result_1.xml").as_bytes(),
        ).unwrap();
        let _: QueryResult = deserialize(
            read_sample_data_from_path("tests/sample-data/query_result_2.xml").as_bytes(),
        ).unwrap();
        let _: QueryResult = deserialize(
            read_sample_data_from_path("tests/sample-data/query_result_3.xml").as_bytes(),
        ).unwrap();
        let _: QueryResult = deserialize(
            read_sample_data_from_path("tests/sample-data/query_result_4.xml").as_bytes(),
        ).unwrap();
        let _: QueryResult = deserialize(
            read_sample_data_from_path("tests/sample-data/query_result_5.xml").as_bytes(),
        ).unwrap();
        let _: QueryResult = deserialize(
            read_sample_data_from_path("tests/sample-data/query_result_6.xml").as_bytes(),
        ).unwrap();
    }

    #[test]
    fn test_query_result_with_error_deserializer() {
        let _: QueryResult = deserialize(
            read_sample_data_from_path("tests/sample-data/query_result-with-error.xml").as_bytes(),
        ).unwrap();
    }

    #[test]
    fn test_notes_deserializer() {
        let _: Notes = deserialize(
            read_sample_data_from_path("tests/sample-data/notes.xml").as_bytes(),
        ).unwrap();
    }

    #[test]
    fn test_languagemsg_deserializer() {
        let _: LanguageMsg = deserialize(
            read_sample_data_from_path("tests/sample-data/languagemsg.xml").as_bytes(),
        ).unwrap();
    }

    #[test]
    fn test_definitions_deserializer() {
        let _: Definitions = deserialize(
            read_sample_data_from_path("tests/sample-data/definitions.xml").as_bytes(),
        ).unwrap();
    }

    #[test]
    fn test_didyoumean_deserializer() {
        let _: DidYouMean = deserialize(
            read_sample_data_from_path("tests/sample-data/didyoumean.xml").as_bytes(),
        ).unwrap();
    }

    #[test]
    fn test_didyoumeans_deserializer() {
        let _: DidYouMeans = deserialize(
            read_sample_data_from_path("tests/sample-data/didyoumeans.xml").as_bytes(),
        ).unwrap();
    }

    #[test]
    fn test_warning_deserializer() {
        let _: Spellcheck = deserialize(
            read_sample_data_from_path("tests/sample-data/spellcheck.xml").as_bytes(),
        ).unwrap();
    }

    #[test]
    fn test_warnings_deserializer() {
        let _: Warnings = deserialize(
            read_sample_data_from_path("tests/sample-data/warnings.xml").as_bytes(),
        ).unwrap();
    }

    #[test]
    fn test_tips_deserializer() {
        let _: Tips = deserialize(
            read_sample_data_from_path("tests/sample-data/tips.xml").as_bytes(),
        ).unwrap();
    }

    #[test]
    fn test_state_deserializer() {
        let _: State = deserialize(
            read_sample_data_from_path("tests/sample-data/state/state.xml").as_bytes(),
        ).unwrap();
    }

    #[test]
    fn test_statelist_deserializer() {
        let _: Statelist = deserialize(
            read_sample_data_from_path("tests/sample-data/state/statelist.xml").as_bytes(),
        ).unwrap();
    }

    #[test]
    fn test_states_deserializer() {
        let _: States = deserialize(
            read_sample_data_from_path("tests/sample-data/state/states.xml").as_bytes(),
        ).unwrap();
        let _: States = deserialize(
            read_sample_data_from_path("tests/sample-data/state/states-multiple-states.xml")
                .as_bytes(),
        ).unwrap();
        let _: States = deserialize(
            read_sample_data_from_path("tests/sample-data/state/states-multiple-statelists.xml")
                .as_bytes(),
        ).unwrap();
        let _: States = deserialize(
            read_sample_data_from_path("tests/sample-data/state/states-out-of-order.xml")
                .as_bytes(),
        ).unwrap();
        let sts: States = deserialize(
            read_sample_data_from_path("tests/sample-data/state/states-out-of-order-complex.xml")
                .as_bytes(),
        ).unwrap();
        assert_eq!(
            sts,
            States {
                count: 5, /*
                state: vec![
                    State {
                        name: 'a'.to_string(),
                        input: 'a'.to_string(),
                    },
                    State {
                        name: 'c'.to_string(),
                        input: 'c'.to_string(),
                    },
                    State {
                        name: 'f'.to_string(),
                        input: 'f'.to_string(),
                    },
                ],
                
                statelist: Some(vec![
                    Statelist {
                        count: 2,
                        value: 'b'.to_string(),
                        state: vec![
                            State {
                                name: "b1".to_owned(),
                                input: "b1".to_owned(),
                            },
                            State {
                                name: "b2".to_owned(),
                                input: "b2".to_owned(),
                            },
                        ],
                    },
                    Statelist {
                        count: 2,
                        value: 'd'.to_string(),
                        state: vec![
                            State {
                                name: "d1".to_owned(),
                                input: "d1".to_owned(),
                            },
                            State {
                                name: "d2".to_owned(),
                                input: "d2".to_owned(),
                            },
                        ],
                    },
                    Statelist {
                        count: 2,
                        value: 'e'.to_string(),
                        state: vec![
                            State {
                                name: "e1".to_owned(),
                                input: "e1".to_owned(),
                            },
                            State {
                                name: "e2".to_owned(),
                                input: "e2".to_owned(),
                            },
                        ],
                    },
                ]),*/
            }
        );
    }

    #[test]
    fn test_pod_deserializer() {
        let _: Pod = deserialize(
            read_sample_data_from_path("tests/sample-data/pod.xml").as_bytes(),
        ).unwrap();
    }

    #[test]
    fn test_subpod_deserializer() {
        let _: Subpod = deserialize(
            read_sample_data_from_path("tests/sample-data/subpod.xml").as_bytes(),
        ).unwrap();
    }

    #[test]
    fn test_img_deserializer() {
        let _: Img = deserialize(
            read_sample_data_from_path("tests/sample-data/img.xml").as_bytes(),
        ).unwrap();
    }

    #[test]
    fn test_plaintext_deserializer() {
        let _: Plaintext = deserialize("<plaintext>pi</plaintext>".as_bytes()).unwrap();
        let _: Option<Plaintext> = deserialize("<plaintext/>".as_bytes()).unwrap();
    }

    #[test]
    fn test_infos_deserializer() {
        let _: Infos = deserialize(
            read_sample_data_from_path("tests/sample-data/infos.xml").as_bytes(),
        ).unwrap();
    }

    #[test]
    fn test_error_deserializer() {
        let _: Error = deserialize(
            read_sample_data_from_path("tests/sample-data/error.xml").as_bytes(),
        ).unwrap();
    }
}
