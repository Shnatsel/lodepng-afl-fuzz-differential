## Sample differential fuzzing harness for Rust

This is an example of a differential fuzzing harness for Rust code using [AFL](https://github.com/rust-fuzz/afl.rs) and [libdiffuzz](https://github.com/Shnatsel/libdiffuzz). It tests [lodepng-rust](https://github.com/kornelski/lodepng-rust) for memory disclosure vulnerabilities.

### Quick start
```bash
# Install AFL
cargo install --force afl
# Download and build libdiffuzz
git clone https://github.com/Shnatsel/libdiffuzz
cd libdiffuzz
make
cd ..
# Download lodepng repository (optional, you can use http://schaik.com/pngsuite/pngsuite.html instead)
git clone https://github.com/kornelski/lodepng-rust
# Download this fuzzing harness, build the code with fuzzing instrumentation
git clone https://github.com/Shnatsel/lodepng-afl-fuzz-differential
cd lodepng-afl-fuzz-differential
cargo afl build --release
# Run the fuzzer with libdiffuzz injected into the target binary
AFL_PRELOAD=../libdiffuzz/libdiffuzz.so cargo afl fuzz -i ../lodepng-rust/fuzz/seeds/decode32 -o afl_results -M master target/release/lodepng-afl-fuzz-differential
# If you want to fuzz the process in parallel, run a few more jobs like this in other terminal windows:
AFL_PRELOAD=../libdiffuzz/libdiffuzz.so cargo afl fuzz -i ../lodepng-rust/fuzz/seeds/decode32 -o afl_results -S slave1 target/release/lodepng-afl-fuzz-differential
AFL_PRELOAD=../libdiffuzz/libdiffuzz.so cargo afl fuzz -i ../lodepng-rust/fuzz/seeds/decode32 -o afl_results -S slave2 target/release/lodepng-afl-fuzz-differential
# ...and so on
# Once the "Cycles done" counter turns green, it means the fuzzer has not discovered any new state transitions in a while and it's time to stop
```

For more information on using AFL consult [Rust Fuzz book](https://fuzz.rs/book/afl/tutorial.html) and [AFL README](http://lcamtuf.coredump.cx/afl/README.txt). For documentation on libdiffuzz see [its repository](https://github.com/Shnatsel/libdiffuzz).

### License

This code is licensed under [Creative Commons Zero 1.0](https://creativecommons.org/publicdomain/zero/1.0/)
