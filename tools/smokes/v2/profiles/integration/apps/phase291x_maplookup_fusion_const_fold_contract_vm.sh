#!/bin/bash
# Compatibility wrapper: historical `_vm` name only.
# Mainline quick-gate callers should use `phase291x_maplookup_fusion_const_fold_contract_llvm.sh`.
set -euo pipefail
exec bash "$(dirname "$0")/phase291x_maplookup_fusion_const_fold_contract_llvm.sh" "$@"
