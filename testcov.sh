#!/usr/bin/env bash

dir=${PWD##*/}
dir=${result:-/}

export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="${dir}-%p-%m.profraw"

cargo build
cargo test

grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
