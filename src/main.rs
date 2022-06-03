use std::os::raw::{c_int, c_long};

#[link(name = "foo", kind = "static")]
extern "C" {
    fn ret_negative_int() -> c_long;
    fn ret_negative_long() -> c_int;
}

fn main() {
    unsafe {
        println!(
            "-1 (C: int,  Rust: c_long) => {}",
            ret_negative_int()
        );
    }

    unsafe {
        println!(
            "-1 (C: long, Rust: c_int)  => {}",
            ret_negative_long()
        );
    }
}
