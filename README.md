# Rust-battles

## Build

1. Install `lld`(`deprecated`, skip this step)
    > Fast compilation is enabled, please make sure that lld is installed.
    + Ubuntu: `sudo apt-get install lld`
    + Arch: `sudo pacman -S lld`
    + Windows: Ensure you have the latest `cargo-binutils`
        + `cargo install -f cargo-binutils`
        + `rustup component add llvm-tools-preview`
    + MacOS: Modern LLD does not yet support MacOS, but we can use `zld` instead.
        + `brew install michaeleisel/zld/zld`

2. `cargo build --release`

## Run

## Reference

[bevy tutor](https://bevyengine.org/learn/book/getting-started/setup/)
