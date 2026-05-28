#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-176-OBJECT-LIFECYCLE-MIR-BODY-OWNER-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-175-OBJECT-LIFECYCLE-BODY-TIMING-GAP-TAXONOMY.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_object_lifecycle_mir_body_owner_selection_guard.sh"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
PAIR_ADAPTER="tools/allocator/hako_mimalloc_object_lifecycle_body_timing_pair_adapter.py"
TAXONOMY="tools/allocator/hako_mimalloc_object_lifecycle_body_timing_gap_taxonomy.py"
SELECTION="tools/allocator/hako_mimalloc_object_lifecycle_mir_body_owner_selection.py"
ATTRIBUTION="tools/allocator/mir_callsite_copy_attribution.py"
HAKO_RUNNER="tools/allocator/hako_exe_memory_runner.sh"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.sh"

[[ -f "$CARD" ]] || { echo "[row176-mir-owner] missing card: $CARD" >&2; exit 1; }
[[ -f "$SELECTION" ]] || { echo "[row176-mir-owner] missing selection adapter: $SELECTION" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || { echo "[row176-mir-owner] row176 card must be Current" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[row176-mir-owner] row175 card must be Landed" >&2; exit 1; }
grep -q 'latest_card = "296x-176-OBJECT-LIFECYCLE-MIR-BODY-OWNER-SELECTION"' "$STATE" || { echo "[row176-mir-owner] CURRENT_STATE latest_card must point to row176" >&2; exit 1; }
grep -q 'current_blocker_token = "OBJECT-LIFECYCLE-MIR-BODY-OWNER-SELECTION-296X-001"' "$STATE" || { echo "[row176-mir-owner] CURRENT_STATE blocker must point to row176" >&2; exit 1; }
grep -q '| 175 | `OBJECT-LIFECYCLE-BODY-TIMING-GAP-TAXONOMY-296X-001` | Landed |' "$TASKBOARD" || { echo "[row176-mir-owner] taskboard row175 must be Landed" >&2; exit 1; }
grep -q '| 176 | `OBJECT-LIFECYCLE-MIR-BODY-OWNER-SELECTION-296X-001` | Current |' "$TASKBOARD" || { echo "[row176-mir-owner] taskboard row176 must be Current" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[row176-mir-owner] check index missing guard entry" >&2; exit 1; }

tmp_dir="$(mktemp -d /tmp/hakorune_row176_mir_owner.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
hako_report="$tmp_dir/hako.out"
c_report="$tmp_dir/c.out"
pair_report="$tmp_dir/pair.out"
taxonomy_report="$tmp_dir/taxonomy.out"
mir_json="$tmp_dir/app.mir.json"
attr_report="$tmp_dir/attr.out"
selection_report="$tmp_dir/selection.out"

bash "$HAKO_RUNNER" --app "$APP" --workload representative-object-lifecycle-small-block-v0 --runtime-config empty --operation-repeat 1 --out "$hako_report" >/dev/null
bash "$C_RUNNER" --out "$c_report" --allow-ldconfig-discovery --workload representative-object-lifecycle-small-block-v0 --in-process-repeat 8192 --operation-repeat 1 >/dev/null
python3 "$PAIR_ADAPTER" --hako-report "$hako_report" --c-report "$c_report" --out "$pair_report"
python3 "$TAXONOMY" --input "$pair_report" --out "$taxonomy_report"

NYASH_FEATURES="${NYASH_FEATURES:-rune}" NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null
python3 "$ATTRIBUTION" --mir-json "$mir_json" --out "$attr_report"
python3 "$SELECTION" --taxonomy "$taxonomy_report" --attribution "$attr_report" --out "$selection_report"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$selection_report"; then
    echo "[row176-mir-owner] missing report line: $expected" >&2
    cat "$selection_report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-object-lifecycle-mir-body-owner-selection-v0"
require_line "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "body_gap_owner=compiler_lowering"
require_line "selected_mir_body_owner=local_ssa_copy_materialization"
require_line "rejected_recent_nonkeeper=local_ssa_same_block_field_get_reuse"
require_line "rejected_reason=prior_structural_win_regressed_exact_exe_body"
require_line "next_diagnostic=local_ssa_dynamic_weight_probe"
require_line "optimization_open=0"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

echo "[row176-mir-owner] ok"
