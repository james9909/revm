#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate vmtests;

use serde_json::Value;
use std::collections::HashMap;
use vmtests::{load_tests, run_test};

lazy_static! {
    static ref TESTS: HashMap<String, Value> = load_tests("tests/vmBlockInfoTest/");
}

#[test]
fn test_coinbase() {
    assert!(run_test(&TESTS["coinbase"]));
}
#[test]
fn test_difficulty() {
    assert!(run_test(&TESTS["difficulty"]));
}
#[test]
fn test_gaslimit() {
    assert!(run_test(&TESTS["gaslimit"]));
}
#[test]
fn test_number() {
    assert!(run_test(&TESTS["number"]));
}
#[test]
fn test_timestamp() {
    assert!(run_test(&TESTS["timestamp"]));
}
