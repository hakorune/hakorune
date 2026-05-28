#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
TOOL="tools/allocator/mir_callsite_copy_attribution.py"
CARD="docs/development/current/main/phases/phase-296x/296x-156-OBJECT-LIFECYCLE-SMALL-HOTPATH-CALLSITE-COPY-ATTRIBUTION.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-155-MIR-BUILDER-SAME-MODULE-HELPER-CALL-LOWERING-SEAM.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_object_lifecycle_small_hotpath_callsite_copy_attribution_guard.sh"

[[ -f "$APP" ]] || { echo "[row156-callsite-copy-attribution] missing app: $APP" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[row156-callsite-copy-attribution] missing tool: $TOOL" >&2; exit 1; }
[[ -f "$CARD" ]] || { echo "[row156-callsite-copy-attribution] missing card: $CARD" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || {
  echo "[row156-callsite-copy-attribution] row156 card must be Current" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[row156-callsite-copy-attribution] row155 card must be Landed" >&2
  exit 1
}
grep -q 'latest_card = "296x-156-OBJECT-LIFECYCLE-SMALL-HOTPATH-CALLSITE-COPY-ATTRIBUTION"' "$STATE" || {
  echo "[row156-callsite-copy-attribution] CURRENT_STATE latest_card must point to row156" >&2
  exit 1
}
grep -q 'current_blocker_token = "OBJECT-LIFECYCLE-SMALL-HOTPATH-CALLSITE-COPY-ATTRIBUTION-296X-001"' "$STATE" || {
  echo "[row156-callsite-copy-attribution] CURRENT_STATE blocker must point to row156" >&2
  exit 1
}
grep -q '| 155 | `MIR-BUILDER-SAME-MODULE-HELPER-CALL-LOWERING-SEAM-296X-001` | Landed |' "$TASKBOARD" || {
  echo "[row156-callsite-copy-attribution] taskboard row155 must be Landed" >&2
  exit 1
}
grep -q '| 156 | `OBJECT-LIFECYCLE-SMALL-HOTPATH-CALLSITE-COPY-ATTRIBUTION-296X-001` | Current |' "$TASKBOARD" || {
  echo "[row156-callsite-copy-attribution] taskboard row156 must be Current" >&2
  exit 1
}
grep -q "$TOOL" "$INDEX" || {
  echo "[row156-callsite-copy-attribution] check index missing tool entry" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[row156-callsite-copy-attribution] check index missing guard entry" >&2
  exit 1
}

tmp_dir="$(mktemp -d /tmp/hakorune_row156_callsite_copy_attr.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/app.mir.json"
report="$tmp_dir/report.out"

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null

python3 "$TOOL" --mir-json "$mir_json" --out "$report"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$report"; then
    echo "[row156-callsite-copy-attribution] missing report line: $expected" >&2
    echo "[row156-callsite-copy-attribution] report follows:" >&2
    cat "$report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-callsite-copy-attribution-v0"
require_line "input_contract=same-module-helper-call-lowering-seam-v0"
require_line "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "block_count=20"
require_line "instruction_count=190"
require_line "call_count=12"
require_line "copy_count=98"
require_line "phi_count=18"
require_line "helper_call_count=6"
require_line "helper_copy_count=25"
require_line "receiver_copy_count=27"
require_line "arg_copy_count=7"
require_line "result_copy_count=9"
require_line "local_ssa_copy_count=48"
require_line "phi_edge_copy_count=10"
require_line "dominant_callee_family=page_hotpath_helpers"
require_line "dominant_copy_owner=local_ssa_copy_materialization"
require_line "callsite_0_callee=acquire_usize"
require_line "callsite_0_callee_family=page_hotpath_helpers"
require_line "callsite_0_attributed_copy_count=9"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

echo "[row156-callsite-copy-attribution] ok"
