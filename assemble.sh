#!/bin/bash

set -euxo pipefail

crate=esp32c-rt

# remove existing blobs because otherwise this will append object files to the old blobs
rm -f bin/*.a

# rv32imc (RISC-V 32-bit bare metal instruction set with the MC extensions)
# this is the instruction set for the ESP32-C SoC series, no need to build artifacts for other instruction sets
riscv64-unknown-elf-gcc -ggdb3 -fdebug-prefix-map=$(pwd)=/esp32c-rt -c -mabi=ilp32 -march=rv32imc asm.S -o bin/$crate.o
riscv64-unknown-elf-ar crs bin/riscv32imc-unknown-none-elf.a bin/$crate.o

rm bin/$crate.o
