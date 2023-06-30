#!/bin/sh

echo ">> Building contract"

rustup target add wasm32-unknown-unknown
cargo build --all --target wasm32-unknown-unknown --release

mkdir -p res
cp target/wasm32-unknown-unknown/release/sweat_burn.wasm res/sweat_burn.wasm
