#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-packed-record-backend-failfast"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
source "$ROOT_DIR/tools/checks/lib/phase_card_paths.sh"

CARD="$(guard_require_phase293x_card "$TAG" "293x-229-C212-PACKED-RECORD-BACKEND-FAILFAST-HARDENING.md")"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
RECORD_SSOT="docs/development/current/main/design/record-and-packed-array-lowering-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
MIR_MOD="src/mir/mod.rs"
SHARED_GATE="src/mir/backend_capability.rs"
ARRAY_BACKEND_GATE="src/mir/array_record_backend_capability.rs"
EXACT_BACKEND_GATE="src/mir/exact_numeric_backend_capability.rs"
WASM_BACKEND="src/backend/wasm/mod.rs"
WASM_CODEGEN="src/backend/wasm/codegen/mod.rs"
WASM_V2="src/backend/wasm_v2/mod.rs"
LLVM_EXEC="src/runner/modes/common_util/exec.rs"
PYVM_EXEC="src/runner/product/llvm/pyvm_executor.rs"
LLVM_MOD="src/runner/product/llvm/mod.rs"
LLVM_FALLBACK="src/runner/product/llvm/fallback_executor.rs"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_packed_record_backend_failfast_guard.sh"

echo "[$TAG] checking C212 packed record backend fail-fast hardening"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PLAN" \
  "$RECORD_SSOT" \
  "$INDEX" \
  "$MIR_MOD" \
  "$SHARED_GATE" \
  "$ARRAY_BACKEND_GATE" \
  "$EXACT_BACKEND_GATE" \
  "$WASM_BACKEND" \
  "$WASM_CODEGEN" \
  "$WASM_V2" \
  "$LLVM_EXEC" \
  "$PYVM_EXEC" \
  "$LLVM_MOD" \
  "$LLVM_FALLBACK" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "C212 card must be complete"
guard_expect_in_file "$TAG" 'C212 status:' "$PLAN" "mimalloc plan must record C212 status"
guard_expect_in_file "$TAG" '`C212` is complete as' "$RECORD_SSOT" "record SSOT must mark C212 complete"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list C212 guard"

guard_expect_in_file "$TAG" 'array_record_backend_capability' "$MIR_MOD" "MIR root must expose array-record backend gate"
guard_expect_in_file "$TAG" 'backend_capability' "$MIR_MOD" "MIR root must expose shared backend gate"
guard_expect_in_file "$TAG" 'enforce_mir_backend_supported' "$SHARED_GATE" "shared backend gate must exist"
guard_expect_in_file "$TAG" 'enforce_exact_numeric_backend_supported' "$SHARED_GATE" "shared gate must keep exact numeric checks"
guard_expect_in_file "$TAG" 'enforce_array_record_backend_supported' "$SHARED_GATE" "shared gate must add packed record checks"
guard_expect_in_file "$TAG" 'ARRAY_RECORD_BACKEND_PACKED_ROUTE_UNSUPPORTED_TAG' "$ARRAY_BACKEND_GATE" "packed record backend tag must be named"
guard_expect_in_file "$TAG" '\[freeze:backend\]\[array-record/packed-route-unsupported\]' "$ARRAY_BACKEND_GATE" "packed record backend tag must be stable"
guard_expect_in_file "$TAG" 'arraybox.inline_record_columns_v0' "$ARRAY_BACKEND_GATE" "packed record consumer capability must be stable"
guard_expect_in_file "$TAG" 'silent_fallback_allowed=false' "$ARRAY_BACKEND_GATE" "packed record backend gate must reject silent fallback"
guard_expect_in_file "$TAG" 'filter(|plan| plan.backend_lowering_enabled)' "$ARRAY_BACKEND_GATE" "packed record gate must only require explicit backend routes"
guard_expect_in_file "$TAG" 'backend_supports_packed_record_inline_columns' "$ARRAY_BACKEND_GATE" "packed record gate must define backend support boundary"

guard_expect_in_file "$TAG" 'enforce_mir_backend_supported' "$WASM_BACKEND" "wasm backend must use shared backend gate"
guard_expect_in_file "$TAG" 'enforce_wasm_mir_backend_supported' "$WASM_CODEGEN" "wasm codegen must use shared wasm gate"
guard_expect_in_file "$TAG" 'enforce_mir_backend_supported' "$WASM_V2" "wasm-v2 backend must use shared backend gate"
guard_expect_in_file "$TAG" 'enforce_mir_backend_supported' "$LLVM_EXEC" "llvm emit helpers must use shared backend gate"
guard_expect_in_file "$TAG" 'enforce_mir_backend_supported' "$PYVM_EXEC" "pyvm harness must use shared backend gate"
guard_expect_in_file "$TAG" 'enforce_mir_backend_supported' "$LLVM_MOD" "legacy llvm object emit must use shared backend gate"
guard_expect_in_file "$TAG" 'enforce_mir_backend_supported' "$LLVM_FALLBACK" "llvm fallback must use shared backend gate"

cargo test -q mir::array_record_backend_capability
cargo test -q mir::backend_capability

if rg -n 'crate::mir::exact_numeric_backend_capability::enforce_exact_numeric_backend_supported' \
  src/backend src/runner >/tmp/"$TAG".direct_exact 2>&1; then
  echo "[$TAG] ERROR: backend call sites must use enforce_mir_backend_supported" >&2
  cat /tmp/"$TAG".direct_exact >&2
  rm -f /tmp/"$TAG".direct_exact
  exit 1
fi
rm -f /tmp/"$TAG".direct_exact

if rg -n 'array-record/packed-route-unsupported|arraybox\.inline_record_columns_v0|packed_record_required_routes|enforce_mir_backend_supported' \
  lang/src/hako_alloc lang/c-abi/shims src/llvm_py/instructions \
  >/tmp/"$TAG".leak 2>&1; then
  echo "[$TAG] ERROR: C212 backend capability vocabulary leaked into hako_alloc/backend shim surfaces" >&2
  cat /tmp/"$TAG".leak >&2
  rm -f /tmp/"$TAG".leak
  exit 1
fi
rm -f /tmp/"$TAG".leak

if rg -n 'k2_wide_packed_record_backend_failfast_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: C212 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

echo "[$TAG] ok"
