#!/bin/bash
# S3 (C‑API pure): threeblock collect → rc=44（pureフラグON）
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/profiles/integration/core/phase2120/boundary_pure_helper.sh"
phase2120_boundary_pure_prepare "$ROOT" "s3_link_run_llvmcapi_pure_ternary_collect_canary_vm"

json=$(bash "$ROOT/tools/selfhost/examples/gen_v1_threeblock_collect.sh")
export _MIR_JSON="$json"
phase2120_boundary_pure_run "$json" 44 "s3_exe_ternary_capi_pure"
echo "[PASS] s3_link_run_llvmcapi_pure_ternary_collect_canary_vm"
exit 0
