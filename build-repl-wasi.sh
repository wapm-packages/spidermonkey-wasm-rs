#!/bin/bash

./crates/repl-example/build.sh +1.56.1 build --target=wasm32-wasi --release

mkdir -p build
cp target/wasm32-wasi/release/repl-example.wasm build/spidermonkey.wasm
wasm-strip build/spidermonkey.wasm
wasm-opt build/spidermonkey.wasm -O3 -o build/spidermonkey.wasm
