#!/bin/bash
set -xue

# QEMUのファイルパス
QEMU=qemu-system-riscv32

# QEMUを起動
$QEMU -machine virt -bios default -nographic -serial mon:stdio --no-reboot \
  -d guest_errors \
  -kernel ./target/riscv32i-unknown-none-elf/release/kernel_elf

