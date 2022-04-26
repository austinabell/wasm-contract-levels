#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(target_arch = "wasm32", feature(alloc_error_handler))]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use miniserde::{json, Deserialize, Serialize};

// Input struct used as parameters.
#[derive(Serialize, Deserialize, Debug)]
struct Input {
    name: String,
}

#[no_mangle]
pub fn hello() {
    let input: Input = json::from_str(core::str::from_utf8(&input()).unwrap()).unwrap();
    let message = ["Hello ", &input.name].concat();
    log_str(&message);
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

fn log_str(msg: &str) {
    unsafe { sys::log_utf8(msg.len() as u64, msg.as_ptr() as u64) }
}

mod sys {
    extern "C" {
        pub fn read_register(register_id: u64, ptr: u64);
        pub fn register_len(register_id: u64) -> u64;
        pub fn input(register_id: u64);
        pub fn log_utf8(len: u64, ptr: u64);
    }
}

// Set up global allocator by default in wasm32 architecture.
// This overwritten allocator is better for Wasm contexts.
#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Alloc error handler override.
#[cfg(all(target_arch = "wasm32", not(test)))]
#[alloc_error_handler]
fn oom(_: core::alloc::Layout) -> ! {
    core::arch::wasm32::unreachable()
}

// Update panic handler in wasm32 environments.
#[cfg(all(target_arch = "wasm32", not(test)))]
#[panic_handler]
#[allow(unused_variables)]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}
