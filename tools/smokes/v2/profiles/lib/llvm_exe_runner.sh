#!/bin/bash
# Compatibility shim for legacy scripts under tools/smokes/v2/profiles/**.
source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)/lib/llvm_exe_runner.sh"
