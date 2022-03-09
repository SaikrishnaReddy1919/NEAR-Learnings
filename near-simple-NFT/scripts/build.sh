!#/bin/bash
env 'RUSTFLAGS=-C link-arg=-s'
cargo build --target wasm32-unknown-unknown --release
cp ../target/wasm32-unknown-unknown/release/near_simple_NFT.wasm ../result/result.wasm