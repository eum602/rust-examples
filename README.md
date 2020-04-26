# Rust examples #
Rust is a type-safe programming language that brings the best from operating system languages. I mean in C/C++ it is enough
if your syntax is correct. But for Rust it is not enough, so it also verifies undefined behaviors during compilation time. Whilst C/C++ languages lets you do anything with pointers even when it crashes  your program rust doesn't.
PD. Rust can also perform unsafe-code.

## Install Rust
* Make sure you have a 'c' compiler.
* curl https://sh.rustup.rs -sSf | sh
* restart then, because rust automatically adds ~/.cargo to your path.

## Rust commands
* cargo --version ; cargo is the rust compilation manager
* cargo new --bin project_name ;  creates a new project
* cargo run ; runs the program from anywhere into the package. Invokes the rustc and the executes the producer binary.
* cargo clean ; clears generated files from "cargo run" that goes into the "target" folder.

* rustdoc --version ; rustdoc is the rust documentation

* rustc --version ; rustc is the rust compiler
* rustc main.rs; compiles the code. Then execute the binary output file.

* rustup doc; opens a built in documentation.
* rustup update; updates the version
* rustup self uninstall
