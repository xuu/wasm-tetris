#!/bin/sh

set -ex

cargo build --target wasm32-unknown-unknown

wasm-bindgen ./target/wasm32-unknown-unknown/debug/wasm_tetris.wasm --out-dir ./wasm

# npm install
# npm run serve