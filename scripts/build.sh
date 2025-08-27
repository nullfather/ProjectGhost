#!/bin/sh
set -e
OUT=build
mkdir -p $OUT/iso
cargo build --target x86_64-unknown-none --release
cp target/x86_64-unknown-none/release/kernel $OUT/iso/
