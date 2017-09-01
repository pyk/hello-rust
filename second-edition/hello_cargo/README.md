# Hello, cargo!

To build this program, run the following command:

    cargo build

It will generate the binaries on the `target/debug` directory.

    ./target/debug/hello_cargo

You can also use the following command to build and run this
program:

    cargo run

To release the program, use the following command:

    cargo build --release

This will generate the binaries on `target/release`, it will
take longer than the `cargo build` because of the optimalization
that happen on the release. In exchange, this will make your program
run faster.
