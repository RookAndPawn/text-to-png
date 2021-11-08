#!/bin/sh

set -e

cargo fmt
cargo test
cargo clippy

cp README.md text-to-png/README-COPY.md

