#!/usr/bin/bash
cd rust
cargo build
cd ../integration
cargo test
cd ..

