#!/bin/sh
set -e
mkdir -p logs
qemu-system-x86_64 -m 2048 -serial file:logs/qemu_output.txt -vga std -kernel build/iso/kernel
