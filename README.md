# stm32h7b3i-dk-demo
Minimal project showing how easy it is to get started with Embedded Rust.

## Install Rust Compiler
https://www.rust-lang.org/learn/get-started

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Install probe-rs
https://probe.rs/

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

## Flash and Run
```sh
cargo run
```

This will download all dependencies, build the project and then call probe-rs to flash the binary.
