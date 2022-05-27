#[macro_use]
extern crate pest;
extern crate pest_grammars;

use std::fs::File;
use std::io::Read;

use pest::Parser;

use pest_grammars::with_fn::*;

#[test]
fn custom_fn_indirect() {
    parses_to! {
        parser: FnParser,
        input: "(!)",
        rule: Rule::a,
        tokens: [
            a(0, 3, [
                b(1, 2)
            ])
        ]
    };
}

#[test]
fn custom_fn_direct() {
    parses_to! {
        parser: FnParser,
        input: "!",
        rule: Rule::b,
        tokens: [
            b(0, 1)
        ]
    };
}
