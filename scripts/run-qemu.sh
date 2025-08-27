#!/bin/bash
qemu-system-x86_64 \
  -cdrom ProjectGhost.iso \
  -m 2048 \
  -serial stdio \
  -no-reboot \
  -d int
