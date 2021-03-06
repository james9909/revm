extern crate revm;

extern crate bigint;
extern crate hexutil;
extern crate serde_json;

use self::revm::vm::block::Block;
use self::revm::vm::{VMResult, VM};

use self::bigint::{Address, Gas, U256};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::prelude::*;
use std::path::PathBuf;

fn read_serde_hex(data: &Value) -> Vec<u8> {
    read_hex(data.as_str().unwrap())
}

fn read_hex(s: &str) -> Vec<u8> {
    hexutil::read_hex(s).unwrap()
}

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

fn setup_vm(test: &Value) -> VM {
    let env = &test["env"];
    let coinbase = Address::from(&read_serde_hex(&env["currentCoinbase"])[..]);
    let difficulty = U256::from(&read_serde_hex(&env["currentDifficulty"])[..]);
    let gas_limit = U256::from(&read_serde_hex(&env["currentGasLimit"])[..]);
    let number = U256::from(&read_serde_hex(&env["currentNumber"])[..]);
    let timestamp = U256::from(&read_serde_hex(&env["currentTimestamp"])[..]);
    let block = Block {
        beneficiary: coinbase,
        difficulty: difficulty,
        gas_limit: gas_limit,
        number: number,
        timestamp: timestamp,
    };

    let exec = &test["exec"];
    let code = read_serde_hex(&exec["code"]);
    let gas = read_serde_hex(&exec["gas"]);
    let gas_price = read_serde_hex(&exec["gasPrice"]);
    let mut vm = VM::new(code, block, Gas::from(&gas_price[..]), Gas::from(&gas[..]));

    let caller = Address::from(&read_serde_hex(&exec["caller"])[..]);
    let code = read_serde_hex(&exec["code"]);
    let data = read_serde_hex(&exec["data"]);
    let origin = Address::from(&read_serde_hex(&exec["origin"])[..]);
    let owner = Address::from(&read_serde_hex(&exec["address"])[..]);
    let value = U256::from(&read_serde_hex(&exec["value"])[..]);

    vm.state.caller = caller;
    vm.state.code = code.clone();
    vm.state.data = data;
    vm.state.origin = origin;
    vm.state.owner = owner;
    vm.state.value = value;

    let pre = &test["pre"];
    for (address, account_spec) in pre.as_object().unwrap() {
        let address = Address::from(&read_hex(address)[..]);
        let code = &read_serde_hex(&account_spec["code"]);
        let balance = U256::from(&read_serde_hex(&account_spec["balance"])[..]);
        vm.state
            .account_manager
            .create_account(&address, code.to_vec(), balance);
    }
    vm
}

fn validate_results(data: &Value, vm: &VM) -> bool {
    let post = data["post"].as_object();
    if post.is_none() {
        // Execution should have failed
        return false;
    }
    for (address, expected) in post.unwrap() {
        let address = Address::from(&read_hex(address)[..]);
        let account = vm.state.account_manager.get_account(&address).unwrap();

        let code = read_serde_hex(&expected["code"]);
        if code != *account.get_code() {
            println!("\nIncorrect code for address 0x{:x}", address);
            println!("Got {:?}", code);
            println!("Expected {:?}", *account.get_code());
            return false;
        }

        let balance = U256::from(&read_serde_hex(&expected["balance"])[..]);
        if balance != account.get_balance() {
            println!("\nIncorrect balance for address 0x{:x}", address);
            println!("Got 0x{:x}", balance);
            println!("Expected 0x{:x}", account.get_balance());
            return false;
        }

        let nonce = U256::from(&read_serde_hex(&expected["nonce"])[..]);
        if nonce.low_u32() != account.get_nonce() {
            println!("\nIncorrect nonce for address 0x{:x}", address);
            println!("Got {}", nonce.low_u32());
            println!("Expected {}", account.get_nonce());
            return false;
        }

        let gas = Gas::from(&read_serde_hex(&data["gas"])[..]);
        let remaining_gas = vm.state.gas_meter.remaining_gas();
        if gas != remaining_gas {
            println!("\nIncorrect gas calculations");
            println!("Got 0x{:x}", remaining_gas);
            println!("Expected 0x{:x}", gas);
            return false;
        }

        let expected_log_hash = U256::from(&read_serde_hex(&data["logs"])[..]);
        let actual_log_hash = vm.log_hash();
        if expected_log_hash != actual_log_hash {
            println!("\nIncorrect log hash");
            println!("Got 0x{:x}", actual_log_hash);
            println!("Expected 0x{:x}", expected_log_hash);
            return false;
        }

        let expected_out = &read_serde_hex(&data["out"]);
        let actual_out = &vm.state.out;
        if expected_out != actual_out {
            println!("\nIncorrect out");
            println!("Got {:?}", actual_out);
            println!("Expected {:?}", expected_out);
            return false;
        }

        let storage = expected["storage"].as_object().unwrap();
        for (offset, value) in storage {
            let offset = U256::from(&read_hex(offset)[..]);
            let value = U256::from(&read_serde_hex(value)[..]);

            let candidate = vm
                .state
                .account_manager
                .get_storage(&address, &offset)
                .unwrap();
            if candidate != value {
                println!(
                    "\nStorage check failed for address 0x{:x} at offset 0x{:x}",
                    address, offset
                );
                println!("Got 0x{:x}", candidate);
                println!("Expected 0x{:x}", value);
                return false;
            }
        }
    }
    true
}

pub fn run_test(data: &Value) -> bool {
    let mut vm = setup_vm(&data);
    match vm.run() {
        VMResult::SUCCESS => validate_results(&data, &vm),
        VMResult::FAILURE(_e) => {
            // If a test doesn't contain postconditions, then a failure is intended
            data["post"].as_object().is_none()
        }
    }
}
