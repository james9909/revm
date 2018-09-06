#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate vmtests;

use serde_json::Value;
use std::collections::HashMap;
use vmtests::{load_tests, run_test};

lazy_static! {
    static ref TESTS: HashMap<String, Value> = load_tests("tests/vmPushDupSwapTest/");
}

#[test]
fn test_dup1() {
    assert!(run_test(&TESTS["dup1"]));
}
#[test]
fn test_dup10() {
    assert!(run_test(&TESTS["dup10"]));
}
#[test]
fn test_dup11() {
    assert!(run_test(&TESTS["dup11"]));
}
#[test]
fn test_dup12() {
    assert!(run_test(&TESTS["dup12"]));
}
#[test]
fn test_dup13() {
    assert!(run_test(&TESTS["dup13"]));
}
#[test]
fn test_dup14() {
    assert!(run_test(&TESTS["dup14"]));
}
#[test]
fn test_dup15() {
    assert!(run_test(&TESTS["dup15"]));
}
#[test]
fn test_dup16() {
    assert!(run_test(&TESTS["dup16"]));
}
#[test]
fn test_dup2() {
    assert!(run_test(&TESTS["dup2"]));
}
#[test]
fn test_dup2error() {
    assert!(run_test(&TESTS["dup2error"]));
}
#[test]
fn test_dup3() {
    assert!(run_test(&TESTS["dup3"]));
}
#[test]
fn test_dup4() {
    assert!(run_test(&TESTS["dup4"]));
}
#[test]
fn test_dup5() {
    assert!(run_test(&TESTS["dup5"]));
}
#[test]
fn test_dup6() {
    assert!(run_test(&TESTS["dup6"]));
}
#[test]
fn test_dup7() {
    assert!(run_test(&TESTS["dup7"]));
}
#[test]
fn test_dup8() {
    assert!(run_test(&TESTS["dup8"]));
}
#[test]
fn test_dup9() {
    assert!(run_test(&TESTS["dup9"]));
}
#[test]
fn test_push1() {
    assert!(run_test(&TESTS["push1"]));
}
#[test]
fn test_push10() {
    assert!(run_test(&TESTS["push10"]));
}
#[test]
fn test_push11() {
    assert!(run_test(&TESTS["push11"]));
}
#[test]
fn test_push12() {
    assert!(run_test(&TESTS["push12"]));
}
#[test]
fn test_push13() {
    assert!(run_test(&TESTS["push13"]));
}
#[test]
fn test_push14() {
    assert!(run_test(&TESTS["push14"]));
}
#[test]
fn test_push15() {
    assert!(run_test(&TESTS["push15"]));
}
#[test]
fn test_push16() {
    assert!(run_test(&TESTS["push16"]));
}
#[test]
fn test_push17() {
    assert!(run_test(&TESTS["push17"]));
}
#[test]
fn test_push18() {
    assert!(run_test(&TESTS["push18"]));
}
#[test]
fn test_push19() {
    assert!(run_test(&TESTS["push19"]));
}
#[test]
fn test_push1_missingStack() {
    assert!(run_test(&TESTS["push1_missingStack"]));
}
#[test]
fn test_push2() {
    assert!(run_test(&TESTS["push2"]));
}
#[test]
fn test_push20() {
    assert!(run_test(&TESTS["push20"]));
}
#[test]
fn test_push21() {
    assert!(run_test(&TESTS["push21"]));
}
#[test]
fn test_push22() {
    assert!(run_test(&TESTS["push22"]));
}
#[test]
fn test_push23() {
    assert!(run_test(&TESTS["push23"]));
}
#[test]
fn test_push24() {
    assert!(run_test(&TESTS["push24"]));
}
#[test]
fn test_push25() {
    assert!(run_test(&TESTS["push25"]));
}
#[test]
fn test_push26() {
    assert!(run_test(&TESTS["push26"]));
}
#[test]
fn test_push27() {
    assert!(run_test(&TESTS["push27"]));
}
#[test]
fn test_push28() {
    assert!(run_test(&TESTS["push28"]));
}
#[test]
fn test_push29() {
    assert!(run_test(&TESTS["push29"]));
}
#[test]
fn test_push3() {
    assert!(run_test(&TESTS["push3"]));
}
#[test]
fn test_push30() {
    assert!(run_test(&TESTS["push30"]));
}
#[test]
fn test_push31() {
    assert!(run_test(&TESTS["push31"]));
}
#[test]
fn test_push32() {
    assert!(run_test(&TESTS["push32"]));
}
// #[test]
// fn test_push32AndSuicide() {
//     assert!(run_test(&TESTS["push32AndSuicide"]));
// }
#[test]
fn test_push32FillUpInputWithZerosAtTheEnd() {
    assert!(run_test(&TESTS["push32FillUpInputWithZerosAtTheEnd"]));
}
#[test]
fn test_push32Undefined() {
    assert!(run_test(&TESTS["push32Undefined"]));
}
#[test]
fn test_push32Undefined2() {
    assert!(run_test(&TESTS["push32Undefined2"]));
}
#[test]
fn test_push32Undefined3() {
    assert!(run_test(&TESTS["push32Undefined3"]));
}
#[test]
fn test_push33() {
    assert!(run_test(&TESTS["push33"]));
}
#[test]
fn test_push4() {
    assert!(run_test(&TESTS["push4"]));
}
#[test]
fn test_push5() {
    assert!(run_test(&TESTS["push5"]));
}
#[test]
fn test_push6() {
    assert!(run_test(&TESTS["push6"]));
}
#[test]
fn test_push7() {
    assert!(run_test(&TESTS["push7"]));
}
#[test]
fn test_push8() {
    assert!(run_test(&TESTS["push8"]));
}
#[test]
fn test_push9() {
    assert!(run_test(&TESTS["push9"]));
}
#[test]
fn test_swap1() {
    assert!(run_test(&TESTS["swap1"]));
}
#[test]
fn test_swap10() {
    assert!(run_test(&TESTS["swap10"]));
}
#[test]
fn test_swap11() {
    assert!(run_test(&TESTS["swap11"]));
}
#[test]
fn test_swap12() {
    assert!(run_test(&TESTS["swap12"]));
}
#[test]
fn test_swap13() {
    assert!(run_test(&TESTS["swap13"]));
}
#[test]
fn test_swap14() {
    assert!(run_test(&TESTS["swap14"]));
}
#[test]
fn test_swap15() {
    assert!(run_test(&TESTS["swap15"]));
}
#[test]
fn test_swap16() {
    assert!(run_test(&TESTS["swap16"]));
}
#[test]
fn test_swap2() {
    assert!(run_test(&TESTS["swap2"]));
}
#[test]
fn test_swap2error() {
    assert!(run_test(&TESTS["swap2error"]));
}
#[test]
fn test_swap3() {
    assert!(run_test(&TESTS["swap3"]));
}
#[test]
fn test_swap4() {
    assert!(run_test(&TESTS["swap4"]));
}
#[test]
fn test_swap5() {
    assert!(run_test(&TESTS["swap5"]));
}
#[test]
fn test_swap6() {
    assert!(run_test(&TESTS["swap6"]));
}
#[test]
fn test_swap7() {
    assert!(run_test(&TESTS["swap7"]));
}
#[test]
fn test_swap8() {
    assert!(run_test(&TESTS["swap8"]));
}
#[test]
fn test_swap9() {
    assert!(run_test(&TESTS["swap9"]));
}
#[test]
fn test_swapjump1() {
    assert!(run_test(&TESTS["swapjump1"]));
}
