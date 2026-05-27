#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-small-alloc-phi-copy-lowering-probe"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_115="docs/development/current/main/phases/phase-296x/296x-115-HAKO-MIMALLOC-SMALL-ALLOC-PHI-COPY-LOWERING-PROBE.md"
CARD_116="docs/development/current/main/phases/phase-296x/296x-116-HAKO-MIMALLOC-SINGLE-INCOMING-PHI-COPY-ELISION-OWNER-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_small_alloc_phi_copy_lowering_probe.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_small_alloc_phi_copy_lowering_probe_guard.sh"

echo "[$TAG] checking small-alloc phi/copy lowering probe"

guard_require_files "$TAG" "$CARD_115" "$CARD_116" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_115" "row115 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_116" "row116 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-small-alloc-phi-copy-lowering-probe-v0' "$CARD_115" "row115 must record output contract"
guard_expect_fixed_in_file "$TAG" 'candidate_source=local_copy_churn' "$CARD_115" "row115 must classify local copy churn"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=single_incoming_phi_copy_elision_owner_selection' "$CARD_115" "row115 must select owner selection"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-115-HAKO-MIMALLOC-SMALL-ALLOC-PHI-COPY-LOWERING-PROBE"' "$CURRENT_STATE" "current state latest card must advance to row115"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-SINGLE-INCOMING-PHI-COPY-ELISION-OWNER-SELECTION-296X-001"' "$CURRENT_STATE" "current state must select row116"
guard_expect_fixed_in_file "$TAG" '| 115 | `HAKO-MIMALLOC-SMALL-ALLOC-PHI-COPY-LOWERING-PROBE-296X-001` | Landed |' "$TASKBOARD" "taskboard row115 must be landed"
guard_expect_fixed_in_file "$TAG" '| 116 | `HAKO-MIMALLOC-SINGLE-INCOMING-PHI-COPY-ELISION-OWNER-SELECTION-296X-001` | Current |' "$TASKBOARD" "taskboard row116 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_small_alloc_phi_copy_probe.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
cat > "$tmp_dir/sample.mir.json" <<'JSON'
{
  "functions": [
    {
      "name": "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
      "blocks": [
        {
          "instructions": [
            {"op": "phi", "dst": 10, "incoming": [[1, 1]]},
            {"op": "phi", "dst": 11, "incoming": [[2, 1]]},
            {"op": "phi", "dst": 13, "incoming": [[5, 1]]},
            {"op": "phi", "dst": 12, "incoming": [[3, 1], [4, 2]]},
            {"op": "copy", "dst": 20, "src": 10},
            {"op": "copy", "dst": 21, "src": 20},
            {"op": "ret", "value": 21}
          ]
        }
      ]
    }
  ]
}
JSON

report="$tmp_dir/report.out"
python3 "$TOOL" --mir-json "$tmp_dir/sample.mir.json" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-small-alloc-phi-copy-lowering-probe-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1' "$report" "tool must select owner"
guard_expect_fixed_in_file "$TAG" 'phi_count=4' "$report" "tool must count phi"
guard_expect_fixed_in_file "$TAG" 'copy_count=2' "$report" "tool must count copy"
guard_expect_fixed_in_file "$TAG" 'single_incoming_phi_count=3' "$report" "tool must count single incoming phi"
guard_expect_fixed_in_file "$TAG" 'multi_incoming_phi_count=1' "$report" "tool must count multi incoming phi"
guard_expect_fixed_in_file "$TAG" 'copy_from_phi_count=1' "$report" "tool must count copy from phi"
guard_expect_fixed_in_file "$TAG" 'candidate_source=local_copy_churn' "$report" "tool must classify local copy churn"
guard_expect_fixed_in_file "$TAG" 'next_action=mirbuilder_owner_probe' "$report" "tool must select owner probe"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
