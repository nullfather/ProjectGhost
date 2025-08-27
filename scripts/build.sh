#!/bin/bash
set -e
cargo +nightly build -Z build-std=core,alloc --target x86_64-unknown-none
cp limine.cfg target/x86_64-unknown-none/debug
cp -r limine/target/* target/x86_64-unknown-none/debug
grub-mkrescue -o ProjectGhost.iso target/x86_64-unknown-none/debug
