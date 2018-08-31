#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;
extern crate serde_json;

mod common;

use common::{load_tests, run_test};
use serde_json::Value;
use std::collections::HashMap;

lazy_static! {
    static ref TESTS: HashMap<String, Value> = load_tests("tests/vmBitwiseLogicOperation/");
}

#[test]
fn test_and0() {
    run_test(&TESTS["and0"]);
}
#[test]
fn test_and1() {
    run_test(&TESTS["and1"]);
}
#[test]
fn test_and2() {
    run_test(&TESTS["and2"]);
}
#[test]
fn test_and3() {
    run_test(&TESTS["and3"]);
}
#[test]
fn test_and4() {
    run_test(&TESTS["and4"]);
}
#[test]
fn test_and5() {
    run_test(&TESTS["and5"]);
}
#[test]
fn test_byte0() {
    run_test(&TESTS["byte0"]);
}
#[test]
fn test_byte10() {
    run_test(&TESTS["byte10"]);
}
#[test]
fn test_byte11() {
    run_test(&TESTS["byte11"]);
}
#[test]
fn test_byte1() {
    run_test(&TESTS["byte1"]);
}
#[test]
fn test_byte2() {
    run_test(&TESTS["byte2"]);
}
#[test]
fn test_byte3() {
    run_test(&TESTS["byte3"]);
}
#[test]
fn test_byte4() {
    run_test(&TESTS["byte4"]);
}
#[test]
fn test_byte5() {
    run_test(&TESTS["byte5"]);
}
#[test]
fn test_byte6() {
    run_test(&TESTS["byte6"]);
}
#[test]
fn test_byte7() {
    run_test(&TESTS["byte7"]);
}
#[test]
fn test_byte8() {
    run_test(&TESTS["byte8"]);
}
#[test]
fn test_byte9() {
    run_test(&TESTS["byte9"]);
}
#[test]
fn test_byteBN() {
    run_test(&TESTS["byteBN"]);
}
#[test]
fn test_eq0() {
    run_test(&TESTS["eq0"]);
}
#[test]
fn test_eq1() {
    run_test(&TESTS["eq1"]);
}
#[test]
fn test_eq2() {
    run_test(&TESTS["eq2"]);
}
#[test]
fn test_gt0() {
    run_test(&TESTS["gt0"]);
}
#[test]
fn test_gt1() {
    run_test(&TESTS["gt1"]);
}
#[test]
fn test_gt2() {
    run_test(&TESTS["gt2"]);
}
#[test]
fn test_gt3() {
    run_test(&TESTS["gt3"]);
}
#[test]
fn test_iszeo2() {
    run_test(&TESTS["iszeo2"]);
}
#[test]
fn test_iszero0() {
    run_test(&TESTS["iszero0"]);
}
#[test]
fn test_iszero1() {
    run_test(&TESTS["iszero1"]);
}
#[test]
fn test_lt0() {
    run_test(&TESTS["lt0"]);
}
#[test]
fn test_lt1() {
    run_test(&TESTS["lt1"]);
}
#[test]
fn test_lt2() {
    run_test(&TESTS["lt2"]);
}
#[test]
fn test_lt3() {
    run_test(&TESTS["lt3"]);
}
#[test]
fn test_not0() {
    run_test(&TESTS["not0"]);
}
#[test]
fn test_not1() {
    run_test(&TESTS["not1"]);
}
#[test]
fn test_not2() {
    run_test(&TESTS["not2"]);
}
#[test]
fn test_not3() {
    run_test(&TESTS["not3"]);
}
#[test]
fn test_not4() {
    run_test(&TESTS["not4"]);
}
#[test]
fn test_not5() {
    run_test(&TESTS["not5"]);
}
#[test]
fn test_or0() {
    run_test(&TESTS["or0"]);
}
#[test]
fn test_or1() {
    run_test(&TESTS["or1"]);
}
#[test]
fn test_or2() {
    run_test(&TESTS["or2"]);
}
#[test]
fn test_or3() {
    run_test(&TESTS["or3"]);
}
#[test]
fn test_or4() {
    run_test(&TESTS["or4"]);
}
#[test]
fn test_or5() {
    run_test(&TESTS["or5"]);
}
#[test]
fn test_sgt0() {
    run_test(&TESTS["sgt0"]);
}
#[test]
fn test_sgt1() {
    run_test(&TESTS["sgt1"]);
}
#[test]
fn test_sgt2() {
    run_test(&TESTS["sgt2"]);
}
#[test]
fn test_sgt3() {
    run_test(&TESTS["sgt3"]);
}
#[test]
fn test_sgt4() {
    run_test(&TESTS["sgt4"]);
}
#[test]
fn test_slt0() {
    run_test(&TESTS["slt0"]);
}
#[test]
fn test_slt1() {
    run_test(&TESTS["slt1"]);
}
#[test]
fn test_slt2() {
    run_test(&TESTS["slt2"]);
}
#[test]
fn test_slt3() {
    run_test(&TESTS["slt3"]);
}
#[test]
fn test_slt4() {
    run_test(&TESTS["slt4"]);
}
#[test]
fn test_xor0() {
    run_test(&TESTS["xor0"]);
}
#[test]
fn test_xor1() {
    run_test(&TESTS["xor1"]);
}
#[test]
fn test_xor2() {
    run_test(&TESTS["xor2"]);
}
#[test]
fn test_xor3() {
    run_test(&TESTS["xor3"]);
}
#[test]
fn test_xor4() {
    run_test(&TESTS["xor4"]);
}
#[test]
fn test_xor5() {
    run_test(&TESTS["xor5"]);
}
