This repo is a small code to reproduce the issue of FFI cast problem.

## Issue

- When Rust casts the C function returning `int` value to `c_long`, the cast fails. The value is treated as unsigned long int.
- On the other hand, When Rust casts the C function returning `long` value to `c_int`, the cast succeeds.

## How to run

Just executes `cargo run`:

```sh
$ cargo run
-1 (C: int,  Rust: c_long) => 4294967295
-1 (C: long, Rust: c_int)  => -1
```

The C functions returns negative value (`-1`) and Rust fails to cast only from int to c_long.
