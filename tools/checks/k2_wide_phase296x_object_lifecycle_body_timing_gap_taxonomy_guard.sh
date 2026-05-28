#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-175-OBJECT-LIFECYCLE-BODY-TIMING-GAP-TAXONOMY.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-174-OBJECT-LIFECYCLE-BODY-TIMING-PAIR-ADAPTER.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
PAIR_ADAPTER="tools/allocator/hako_mimalloc_object_lifecycle_body_timing_pair_adapter.py"
TAXONOMY="tools/allocator/hako_mimalloc_object_lifecycle_body_timing_gap_taxonomy.py"
HAKO_RUNNER="tools/allocator/hako_exe_memory_runner.sh"
C_RUNNER="tools/allocator/c_mimalloc_explicit_runner.sh"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_object_lifecycle_body_timing_gap_taxonomy_guard.sh"

[[ -f "$CARD" ]] || { echo "[row175-body-taxonomy] missing card: $CARD" >&2; exit 1; }
[[ -f "$TAXONOMY" ]] || { echo "[row175-body-taxonomy] missing taxonomy adapter: $TAXONOMY" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || { echo "[row175-body-taxonomy] row175 card must be Current" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[row175-body-taxonomy] row174 card must be Landed" >&2; exit 1; }
grep -q 'latest_card = "296x-175-OBJECT-LIFECYCLE-BODY-TIMING-GAP-TAXONOMY"' "$STATE" || { echo "[row175-body-taxonomy] CURRENT_STATE latest_card must point to row175" >&2; exit 1; }
grep -q 'current_blocker_token = "OBJECT-LIFECYCLE-BODY-TIMING-GAP-TAXONOMY-296X-001"' "$STATE" || { echo "[row175-body-taxonomy] CURRENT_STATE blocker must point to row175" >&2; exit 1; }
grep -q '| 174 | `OBJECT-LIFECYCLE-BODY-TIMING-PAIR-ADAPTER-296X-001` | Landed |' "$TASKBOARD" || { echo "[row175-body-taxonomy] taskboard row174 must be Landed" >&2; exit 1; }
grep -q '| 175 | `OBJECT-LIFECYCLE-BODY-TIMING-GAP-TAXONOMY-296X-001` | Current |' "$TASKBOARD" || { echo "[row175-body-taxonomy] taskboard row175 must be Current" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[row175-body-taxonomy] check index missing guard entry" >&2; exit 1; }

tmp_dir="$(mktemp -d /tmp/hakorune_row175_body_taxonomy.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
hako_report="$tmp_dir/hako.out"
c_report="$tmp_dir/c.out"
pair_report="$tmp_dir/pair.out"
taxonomy_report="$tmp_dir/taxonomy.out"

bash "$HAKO_RUNNER" --app "$APP" --workload representative-object-lifecycle-small-block-v0 --runtime-config empty --operation-repeat 1 --out "$hako_report" >/dev/null
bash "$C_RUNNER" --out "$c_report" --allow-ldconfig-discovery --workload representative-object-lifecycle-small-block-v0 --in-process-repeat 8192 --operation-repeat 1 >/dev/null
python3 "$PAIR_ADAPTER" --hako-report "$hako_report" --c-report "$c_report" --out "$pair_report"
python3 "$TAXONOMY" --input "$pair_report" --out "$taxonomy_report"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$taxonomy_report"; then
    echo "[row175-body-taxonomy] missing report line: $expected" >&2
    cat "$taxonomy_report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-object-lifecycle-body-timing-gap-taxonomy-v0"
require_line "workload_id=representative-object-lifecycle-small-block-v0"
require_line "operation_sequence_id=representative-object-lifecycle-small-block-v0-seq"
require_line "free_order_id=even-odd-release-v0"
require_line "in_process_operation_repeat=8192"
require_line "gap_owner=compiler_lowering"
require_line "gap_confidence=medium"
require_line "evidence_quality=single_sample_large_gap"
require_line "gap_reason=body_gap_large_hako_exact_exe_hot_loop"
require_line "next_diagnostic=object_lifecycle_mir_body_owner_selection"
require_line "next_optimization_allowed=0"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

echo "[row175-body-taxonomy] ok"
