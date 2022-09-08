#!/bin/bash

echo "Starting Rust Server"

cd /opt/rust-server

source "$HOME/.cargo/env"

cargo run --release

