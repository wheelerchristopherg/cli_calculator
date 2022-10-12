#!/usr/bin/bash
cd rust
cargo build
cd ../e2e
cargo test
cd ..

