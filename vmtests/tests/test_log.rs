#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate vmtests;

use serde_json::Value;
use std::collections::HashMap;
use vmtests::{load_tests, run_test};

lazy_static! {
    static ref TESTS: HashMap<String, Value> = load_tests("tests/vmLogTest/");
}

#[test]
fn test_log0_emptyMem() {
    assert!(run_test(&TESTS["log0_emptyMem"]));
}
#[test]
fn test_log0_logMemStartTooHigh() {
    assert!(run_test(&TESTS["log0_logMemStartTooHigh"]));
}
#[test]
fn test_log0_logMemsizeTooHigh() {
    assert!(run_test(&TESTS["log0_logMemsizeTooHigh"]));
}
#[test]
fn test_log0_logMemsizeZero() {
    assert!(run_test(&TESTS["log0_logMemsizeZero"]));
}
#[test]
fn test_log0_nonEmptyMem() {
    assert!(run_test(&TESTS["log0_nonEmptyMem"]));
}
#[test]
fn test_log0_nonEmptyMem_logMemSize1() {
    assert!(run_test(&TESTS["log0_nonEmptyMem_logMemSize1"]));
}
#[test]
fn test_log0_nonEmptyMem_logMemSize1_logMemStart31() {
    assert!(run_test(
        &TESTS["log0_nonEmptyMem_logMemSize1_logMemStart31"]
    ));
}
#[test]
fn test_log1_Caller() {
    assert!(run_test(&TESTS["log1_Caller"]));
}
#[test]
fn test_log1_MaxTopic() {
    assert!(run_test(&TESTS["log1_MaxTopic"]));
}
#[test]
fn test_log1_emptyMem() {
    assert!(run_test(&TESTS["log1_emptyMem"]));
}
#[test]
fn test_log1_logMemStartTooHigh() {
    assert!(run_test(&TESTS["log1_logMemStartTooHigh"]));
}
#[test]
fn test_log1_logMemsizeTooHigh() {
    assert!(run_test(&TESTS["log1_logMemsizeTooHigh"]));
}
#[test]
fn test_log1_logMemsizeZero() {
    assert!(run_test(&TESTS["log1_logMemsizeZero"]));
}
#[test]
fn test_log1_nonEmptyMem() {
    assert!(run_test(&TESTS["log1_nonEmptyMem"]));
}
#[test]
fn test_log1_nonEmptyMem_logMemSize1() {
    assert!(run_test(&TESTS["log1_nonEmptyMem_logMemSize1"]));
}
#[test]
fn test_log1_nonEmptyMem_logMemSize1_logMemStart31() {
    assert!(run_test(
        &TESTS["log1_nonEmptyMem_logMemSize1_logMemStart31"]
    ));
}
#[test]
fn test_log2_Caller() {
    assert!(run_test(&TESTS["log2_Caller"]));
}
#[test]
fn test_log2_MaxTopic() {
    assert!(run_test(&TESTS["log2_MaxTopic"]));
}
#[test]
fn test_log2_emptyMem() {
    assert!(run_test(&TESTS["log2_emptyMem"]));
}
#[test]
fn test_log2_logMemStartTooHigh() {
    assert!(run_test(&TESTS["log2_logMemStartTooHigh"]));
}
#[test]
fn test_log2_logMemsizeTooHigh() {
    assert!(run_test(&TESTS["log2_logMemsizeTooHigh"]));
}
#[test]
fn test_log2_logMemsizeZero() {
    assert!(run_test(&TESTS["log2_logMemsizeZero"]));
}
#[test]
fn test_log2_nonEmptyMem() {
    assert!(run_test(&TESTS["log2_nonEmptyMem"]));
}
#[test]
fn test_log2_nonEmptyMem_logMemSize1() {
    assert!(run_test(&TESTS["log2_nonEmptyMem_logMemSize1"]));
}
#[test]
fn test_log2_nonEmptyMem_logMemSize1_logMemStart31() {
    assert!(run_test(
        &TESTS["log2_nonEmptyMem_logMemSize1_logMemStart31"]
    ));
}
#[test]
fn test_log3_Caller() {
    assert!(run_test(&TESTS["log3_Caller"]));
}
#[test]
fn test_log3_MaxTopic() {
    assert!(run_test(&TESTS["log3_MaxTopic"]));
}
#[test]
fn test_log3_PC() {
    assert!(run_test(&TESTS["log3_PC"]));
}
#[test]
fn test_log3_emptyMem() {
    assert!(run_test(&TESTS["log3_emptyMem"]));
}
#[test]
fn test_log3_logMemStartTooHigh() {
    assert!(run_test(&TESTS["log3_logMemStartTooHigh"]));
}
#[test]
fn test_log3_logMemsizeTooHigh() {
    assert!(run_test(&TESTS["log3_logMemsizeTooHigh"]));
}
#[test]
fn test_log3_logMemsizeZero() {
    assert!(run_test(&TESTS["log3_logMemsizeZero"]));
}
#[test]
fn test_log3_nonEmptyMem() {
    assert!(run_test(&TESTS["log3_nonEmptyMem"]));
}
#[test]
fn test_log3_nonEmptyMem_logMemSize1() {
    assert!(run_test(&TESTS["log3_nonEmptyMem_logMemSize1"]));
}
#[test]
fn test_log3_nonEmptyMem_logMemSize1_logMemStart31() {
    assert!(run_test(
        &TESTS["log3_nonEmptyMem_logMemSize1_logMemStart31"]
    ));
}
#[test]
fn test_log4_Caller() {
    assert!(run_test(&TESTS["log4_Caller"]));
}
#[test]
fn test_log4_MaxTopic() {
    assert!(run_test(&TESTS["log4_MaxTopic"]));
}
#[test]
fn test_log4_PC() {
    assert!(run_test(&TESTS["log4_PC"]));
}
#[test]
fn test_log4_emptyMem() {
    assert!(run_test(&TESTS["log4_emptyMem"]));
}
#[test]
fn test_log4_logMemStartTooHigh() {
    assert!(run_test(&TESTS["log4_logMemStartTooHigh"]));
}
#[test]
fn test_log4_logMemsizeTooHigh() {
    assert!(run_test(&TESTS["log4_logMemsizeTooHigh"]));
}
#[test]
fn test_log4_logMemsizeZero() {
    assert!(run_test(&TESTS["log4_logMemsizeZero"]));
}
#[test]
fn test_log4_nonEmptyMem() {
    assert!(run_test(&TESTS["log4_nonEmptyMem"]));
}
#[test]
fn test_log4_nonEmptyMem_logMemSize1() {
    assert!(run_test(&TESTS["log4_nonEmptyMem_logMemSize1"]));
}
#[test]
fn test_log4_nonEmptyMem_logMemSize1_logMemStart31() {
    assert!(run_test(
        &TESTS["log4_nonEmptyMem_logMemSize1_logMemStart31"]
    ));
}
#[test]
fn test_log_2logs() {
    assert!(run_test(&TESTS["log_2logs"]));
}
