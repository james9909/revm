#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;
extern crate serde_json;

mod common;

use common::{load_tests, run_test};
use serde_json::Value;
use std::collections::HashMap;

lazy_static! {
    static ref TESTS: HashMap<String, Value> = load_tests("tests/vmSha3Test/");
}

#[test]
fn test_sha3_0() {
    assert!(run_test(&TESTS["sha3_0"]));
}
#[test]
fn test_sha3_1() {
    assert!(run_test(&TESTS["sha3_1"]));
}
#[test]
fn test_sha3_2() {
    assert!(run_test(&TESTS["sha3_2"]));
}
#[test]
fn test_sha3_3() {
    assert!(run_test(&TESTS["sha3_3"]));
}
#[test]
fn test_sha3_4() {
    assert!(run_test(&TESTS["sha3_4"]));
}
#[test]
fn test_sha3_5() {
    assert!(run_test(&TESTS["sha3_5"]));
}
#[test]
fn test_sha3_6() {
    assert!(run_test(&TESTS["sha3_6"]));
}
#[test]
fn test_sha3_bigOffset() {
    assert!(run_test(&TESTS["sha3_bigOffset"]));
}
#[test]
fn test_sha3_bigOffset2() {
    assert!(run_test(&TESTS["sha3_bigOffset2"]));
}
#[test]
fn test_sha3_bigSize() {
    assert!(run_test(&TESTS["sha3_bigSize"]));
}
#[test]
fn test_sha3_memSizeNoQuadraticCost31() {
    assert!(run_test(&TESTS["sha3_memSizeNoQuadraticCost31"]));
}
#[test]
fn test_sha3_memSizeQuadraticCost32() {
    assert!(run_test(&TESTS["sha3_memSizeQuadraticCost32"]));
}
#[test]
fn test_sha3_memSizeQuadraticCost32_zeroSize() {
    assert!(run_test(&TESTS["sha3_memSizeQuadraticCost32_zeroSize"]));
}
#[test]
fn test_sha3_memSizeQuadraticCost33() {
    assert!(run_test(&TESTS["sha3_memSizeQuadraticCost33"]));
}
#[test]
fn test_sha3_memSizeQuadraticCost63() {
    assert!(run_test(&TESTS["sha3_memSizeQuadraticCost63"]));
}
#[test]
fn test_sha3_memSizeQuadraticCost64() {
    assert!(run_test(&TESTS["sha3_memSizeQuadraticCost64"]));
}
#[test]
fn test_sha3_memSizeQuadraticCost64_2() {
    assert!(run_test(&TESTS["sha3_memSizeQuadraticCost64_2"]));
}
#[test]
fn test_sha3_memSizeQuadraticCost65() {
    assert!(run_test(&TESTS["sha3_memSizeQuadraticCost65"]));
}
