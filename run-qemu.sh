#!/usr/bin/env bash

# Boot the build on CHERI QEMU's virt machine with a Morello CPU. Plain virt
# enters the ELF at EL1, where the runtime's hybrid entry enables capabilities
# and switches to C64 on its own. Leave with ctrl+a x.

: "${CRABILITY_BIN:=$HOME/.crability/bin}"

ELF="${1:-target/aarch64-unknown-none-purecap/release/cheri}"

echo "Starting QEMU. To quit: press ctrl+a, then x." >&2

exec "$CRABILITY_BIN/qemu-system-morello" \
    -M virt \
    -cpu morello \
    -nographic \
    -semihosting \
    -kernel "$ELF"
