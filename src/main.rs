#[macro_use]
extern crate afl;
extern crate lodepng;

// Use system allocator so we can substitute it with a custom one via LD_PRELOAD
use std::alloc::System;
#[global_allocator]
static GLOBAL: System = System;

fn main() {
    fuzz!(|data| {
        let first_run_result = lodepng::decode32(&data);
        let second_run_result = lodepng::decode32(&data);
        match (first_run_result, second_run_result) {
            (Ok(a), Ok(b)) => assert_eq!(a.buffer, b.buffer),
            (Err(err_a), Err(err_b)) => assert_eq!(err_a, err_b),
            _ => panic!("One decoding run succeeded while the other failed!"),
        }
    });
}