#!/bin/bash
# Compatibility wrapper: historical `_vm` name only.
# Mainline quick-gate callers should use `phase21_5_perf_chip8_kernel_crosslang_contract.sh`.
set -euo pipefail
exec bash "$(dirname "$0")/phase21_5_perf_chip8_kernel_crosslang_contract.sh" "$@"
