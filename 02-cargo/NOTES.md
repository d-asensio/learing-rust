# Cargo

Cargo is the official Rust dependency manager, it also helps you build and run your rust app, some useful commands are:

- `cargo build` Builds the app and puts the resulting binary into `target/debug`. Take into account that this will build an unoptimized version of the program.
- `cargo run` Builds and runs the app (also unoptimized version).
- `cargo check` Checks code correctness without actually taking the time to compile it (faster for development).
- `cargo build --release` Build the code with optimizations (and without debugging information) putting the resulting binary into `target/release`.
