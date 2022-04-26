#![cfg_attr(target_arch = "wasm32", no_std)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use miniserde::{json, Deserialize, Serialize};
use nesdie::{env, sys};

// Input struct used as parameters.
#[derive(Serialize, Deserialize, Debug)]
struct Input {
    name: String,
}

#[no_mangle]
pub fn hello() {
    let input: Input = json::from_str(core::str::from_utf8(&input()).unwrap()).unwrap();
    let message = ["Hello ", &input.name].concat();
    env::log_str(&message);
}

fn input() -> Vec<u8> {
    const REGISTER_ID: u64 = 0;

    unsafe {
        // Load input into register.
        sys::input(REGISTER_ID);

        // Create buffer with capacity for register read.
        let buf_len = sys::register_len(REGISTER_ID).try_into().unwrap();
        let mut buffer = Vec::with_capacity(buf_len);

        // Read register into buffer.
        sys::read_register(REGISTER_ID, buffer.as_mut_ptr() as u64);

        // Set updated length after writing to buffer.
        buffer.set_len(buf_len);
        buffer
    }
}
