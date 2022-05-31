#[macro_use]
extern crate pest;
extern crate pest_grammars;

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

#[test]
fn custom_fn_indirect_with_alias() {
    parses_to! {
        parser: FnParser,
        input: "[D]",
        rule: Rule::c,
        tokens: [
            c(0, 3, [
                d(1,2)
            ])
        ]
    };
}
