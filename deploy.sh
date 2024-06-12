#!/bin/bash

cargo build --release --target wasm32-wasi

wasi2ic target/wasm32-wasi/release/demo1_backend.wasm

dfx canister create demo1_backend

dfx canister install --mode reinstall --wasm no_wasi.wasm demo1_backend 

