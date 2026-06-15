#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-770-MIMALLOC-CURRENT-2X-ASM-BOUNDARY-ATTRIBUTION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-769-MIMALLOC-RUNTIME-BOUNDARY-INVENTORY-FOR-CURRENT-2X-GAP-001.md"
HELPER_CARD="docs/development/current/main/phases/phase-296x/296x-709-MIMALLOC-POST-ARRAY-LEN-HELPER-OWNER-REFRESH-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_current_2x_asm_boundary_attribution_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-current-2x-asm-boundary] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[mimalloc-current-2x-asm-boundary] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$HELPER_CARD" ]] || { echo "[mimalloc-current-2x-asm-boundary] missing helper owner card: $HELPER_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-current-2x-asm-boundary] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[mimalloc-current-2x-asm-boundary] previous card must be Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$HELPER_CARD" || {
  echo "[mimalloc-current-2x-asm-boundary] helper owner card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-current-2x-asm-boundary] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-current-2x-asm-boundary] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-current-2x-asm-boundary-attribution-v0" \
  "source_evidence=296x-769,296x-709" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "runs=20" \
  "in_process_operation_repeat=65536" \
  "body_elapsed_ns=54000000" \
  "top_symbol=nyash_array_length_h" \
  "top_symbol_percent=70.71" \
  "target_symbol=nyash_array_length_h" \
  "top_symbol_is_target=1" \
  "array_length_helper_uses_borrowed_ready=1" \
  "helper_local_fastpath_already_applied=1" \
  "helper_local_fastpath_remaining=0" \
  "remaining_owner=handle_registry_typed_handle_boundary" \
  "remaining_owner_confidence=high" \
  "selected_owner=handle_registry_typed_handle_boundary" \
  "selected_owner_confidence=high" \
  "implementation_allowed=0" \
  "design_required=1" \
  "closed_world_route_required=1" \
  "object_substrate_required=1" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "benchmark_name_special_case=0" \
  "helper_name_special_case=0" \
  "winner_claim=0" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-DESIGN-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

grep -F -q "lock xadd %rax,0x8(%r14)" "$CARD" || {
  echo "[mimalloc-current-2x-asm-boundary] missing lock xadd evidence" >&2
  exit 1
}
grep -F -q "lock cmpxchg %rcx,0x8(%r14)" "$CARD" || {
  echo "[mimalloc-current-2x-asm-boundary] missing lock cmpxchg evidence" >&2
  exit 1
}
grep -F -q "MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-DESIGN-001:" "$CARD" || {
  echo "[mimalloc-current-2x-asm-boundary] next design row is not documented" >&2
  exit 1
}

rg -n "pub extern \"C\" fn nyash_array_length_h" crates/nyash_kernel/src/plugin/array_compat.rs >/dev/null || {
  echo "[mimalloc-current-2x-asm-boundary] missing nyash_array_length_h helper" >&2
  exit 1
}
python3 - <<'PY'
from pathlib import Path
text = Path("crates/nyash_kernel/src/plugin/array_compat.rs").read_text(encoding="utf-8")
needle = "pub extern \"C\" fn nyash_array_length_h(handle: i64) -> i64 {\n    with_array_box_ready(handle, |arr| arr.len() as i64).unwrap_or(0)\n}"
if needle not in text:
    raise SystemExit("[mimalloc-current-2x-asm-boundary] nyash_array_length_h is not on borrowed-ready path")
PY

require_line_in_file "$HELPER_CARD" "remaining_owner=handle_registry_typed_handle_boundary"
require_line_in_file "$HELPER_CARD" "closed_world_route_required=1"
require_line_in_file "$HELPER_CARD" "object_substrate_required=1"

echo "[mimalloc-current-2x-asm-boundary] ok"
