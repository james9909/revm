#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate vmtests;

use serde_json::Value;
use std::collections::HashMap;
use vmtests::{load_tests, run_test};

lazy_static! {
    static ref TESTS: HashMap<String, Value> = load_tests("tests/vmEnvironmentalInfo/");
}

#[test]
fn test_address0() {
    assert!(run_test(&TESTS["address0"]));
}
#[test]
fn test_address1() {
    assert!(run_test(&TESTS["address1"]));
}
#[test]
fn test_calldatacopy0() {
    assert!(run_test(&TESTS["calldatacopy0"]));
}
// #[test]
// fn test_calldatacopy0_return() {
//     assert!(run_test(&TESTS["calldatacopy0_return"]));
// }
#[test]
fn test_calldatacopy1() {
    assert!(run_test(&TESTS["calldatacopy1"]));
}
// #[test]
// fn test_calldatacopy1_return() {
//     assert!(run_test(&TESTS["calldatacopy1_return"]));
// }
#[test]
fn test_calldatacopy2() {
    assert!(run_test(&TESTS["calldatacopy2"]));
}
// #[test]
// fn test_calldatacopy2_return() {
//     assert!(run_test(&TESTS["calldatacopy2_return"]));
// }
#[test]
fn test_calldatacopyUnderFlow() {
    assert!(run_test(&TESTS["calldatacopyUnderFlow"]));
}
#[test]
fn test_calldatacopyZeroMemExpansion() {
    assert!(run_test(&TESTS["calldatacopyZeroMemExpansion"]));
}
// #[test]
// fn test_calldatacopyZeroMemExpansion_return() {
//     assert!(run_test(&TESTS["calldatacopyZeroMemExpansion_return"]));
// }
#[test]
fn test_calldatacopy_DataIndexTooHigh() {
    assert!(run_test(&TESTS["calldatacopy_DataIndexTooHigh"]));
}
#[test]
fn test_calldatacopy_DataIndexTooHigh2() {
    assert!(run_test(&TESTS["calldatacopy_DataIndexTooHigh2"]));
}
#[test]
fn test_calldatacopy_DataIndexTooHigh2_return() {
    assert!(run_test(&TESTS["calldatacopy_DataIndexTooHigh2_return"]));
}
#[test]
fn test_calldatacopy_DataIndexTooHigh_return() {
    assert!(run_test(&TESTS["calldatacopy_DataIndexTooHigh_return"]));
}
#[test]
fn test_calldatacopy_sec() {
    assert!(run_test(&TESTS["calldatacopy_sec"]));
}
#[test]
fn test_calldataload0() {
    assert!(run_test(&TESTS["calldataload0"]));
}
#[test]
fn test_calldataload1() {
    assert!(run_test(&TESTS["calldataload1"]));
}
#[test]
fn test_calldataload2() {
    assert!(run_test(&TESTS["calldataload2"]));
}
#[test]
fn test_calldataloadSizeTooHigh() {
    assert!(run_test(&TESTS["calldataloadSizeTooHigh"]));
}
#[test]
fn test_calldataloadSizeTooHighPartial() {
    assert!(run_test(&TESTS["calldataloadSizeTooHighPartial"]));
}
#[test]
fn test_calldataload_BigOffset() {
    assert!(run_test(&TESTS["calldataload_BigOffset"]));
}
#[test]
fn test_calldatasize0() {
    assert!(run_test(&TESTS["calldatasize0"]));
}
#[test]
fn test_calldatasize1() {
    assert!(run_test(&TESTS["calldatasize1"]));
}
#[test]
fn test_calldatasize2() {
    assert!(run_test(&TESTS["calldatasize2"]));
}
#[test]
fn test_caller() {
    assert!(run_test(&TESTS["caller"]));
}
#[test]
fn test_callvalue() {
    assert!(run_test(&TESTS["callvalue"]));
}
#[test]
fn test_codecopy0() {
    assert!(run_test(&TESTS["codecopy0"]));
}
#[test]
fn test_codecopyZeroMemExpansion() {
    assert!(run_test(&TESTS["codecopyZeroMemExpansion"]));
}
#[test]
fn test_codecopy_DataIndexTooHigh() {
    assert!(run_test(&TESTS["codecopy_DataIndexTooHigh"]));
}
#[test]
fn test_codesize() {
    assert!(run_test(&TESTS["codesize"]));
}
#[test]
fn test_gasprice() {
    assert!(run_test(&TESTS["gasprice"]));
}
#[test]
fn test_origin() {
    assert!(run_test(&TESTS["origin"]));
}
