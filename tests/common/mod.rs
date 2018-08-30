extern crate revm;

extern crate bigint;
extern crate hexutil;
extern crate serde_json;

use self::revm::vm::{VMResult, VM};

use self::bigint::U256;
use self::hexutil::read_hex;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::prelude::*;
use std::path::PathBuf;

fn load_test(path: PathBuf) -> HashMap<String, Value> {
    let mut file = File::open(path).expect("unable to open test file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("failed to read file");
    serde_json::from_str(&contents).unwrap()
}

pub fn load_tests(path: &str) -> HashMap<String, Value> {
    let files = read_dir(path).unwrap();
    let mut all_tests: HashMap<String, Value> = HashMap::new();
    for file in files {
        all_tests.extend(load_test(file.unwrap().path()));
    }
    all_tests
}

fn setup_vm(data: &Value) -> VM {
    let exec = &data["exec"];
    let code = read_hex(exec["code"].as_str().unwrap()).unwrap();
    println!("{:?}", code);
    let gas = read_hex(exec["gas"].as_str().unwrap()).unwrap();
    VM::new(code, U256::from(&gas[..]))
}

pub fn run_test(data: &Value) -> bool {
    let mut vm = setup_vm(&data);
    match vm.run() {
        VMResult::SUCCESS => true,
        VMResult::FAILURE(e) => false,
    }
}
