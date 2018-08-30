#[macro_use]
extern crate lazy_static;
extern crate serde_json;

mod common;

use common::{load_tests, run_test};
use serde_json::Value;
use std::collections::HashMap;

lazy_static! {
    static ref tests: HashMap<String, Value> = load_tests("tests/vmArithmeticTest/");
}

#[test]
fn test_add0() {
    assert!(run_test(&tests["add0"]));
}
