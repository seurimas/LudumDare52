rustc --target wasm32-unknown-unknown --crate-type cdylib assets/source/$1.rs
wasm-gc $1.wasm
mv $1.wasm assets/scripts/
