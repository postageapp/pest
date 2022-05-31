// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! # pest grammars
//!
//! Contains a series of default grammars.

#![doc(html_root_url = "https://docs.rs/pest_grammars")]

extern crate pest;
#[macro_use]
extern crate pest_derive;

pub use pest::Parser;

pub mod with_fn {
    /// Demonstrates = fn notation
    #[derive(Parser)]
    #[grammar = "grammars/with_fn.pest"]
    pub struct FnParser;

    use pest::ParserState;
    use pest::ParseResult;

    impl FnParser {
        pub fn b(state : Box <ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            // state.match_string("!")
            state.rule(
                Rule::b,
                |s| { s.match_string("!") }
            )
        }

        pub fn d_alias(state : Box <ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
            // state.match_string("!")
            state.rule(
                Rule::d,
                |s| { s.match_string("D") }
            )
        }
    }
}

pub mod json {
    /// JSON parser.
    #[derive(Parser)]
    #[grammar = "grammars/json.pest"]
    pub struct JsonParser;
}

pub mod toml {
    /// TOML parser.
    #[derive(Parser)]
    #[grammar = "grammars/toml.pest"]
    pub struct TomlParser;
}
