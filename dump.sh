#!/bin/sh

set -e

# export RUSTFLAGS="-C prefer-dynamic"
cargo clean
cargo build --all

objdump -t target/debug/libL.so  | grep -i hello

cargo run
