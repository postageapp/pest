#[macro_use]
extern crate pest;
extern crate pest_grammars;

use std::fs::File;
use std::io::Read;

use pest::Parser;

use pest_grammars::with_fn::*;
