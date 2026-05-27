#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-single-pred-phi-elision-implementation"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_118="docs/development/current/main/phases/phase-296x/296x-118-HAKO-MIMALLOC-SINGLE-PRED-PHI-ELISION-IMPLEMENTATION.md"
CARD_119="docs/development/current/main/phases/phase-296x/296x-119-HAKO-MIMALLOC-SMALL-ALLOC-MULTI-RETURN-COPY-PROBE.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
OWNER="src/mir/builder/emission/phi.rs"
TOOL="tools/allocator/hako_mimalloc_single_pred_phi_elision_implementation.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_single_pred_phi_elision_implementation_guard.sh"

echo "[$TAG] checking single-pred PHI elision implementation"

guard_require_files "$TAG" "$CARD_118" "$CARD_119" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$OWNER" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_118" "row118 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_119" "row119 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-single-pred-phi-elision-implementation-v0' "$CARD_118" "row118 must record output contract"
guard_expect_fixed_in_file "$TAG" 'after_single_incoming_phi_count=0' "$CARD_118" "row118 must remove single incoming PHIs"
guard_expect_fixed_in_file "$TAG" 'after_hako_elapsed_median_ms=620' "$CARD_118" "row118 must record measurement"
if rg -F -q -- 'insert_phi_single(pre_branch_bb, pre_v)' "$OWNER"; then
  guard_fail "$TAG" "owner must not emit single-pred PHI here"
fi
guard_expect_fixed_in_file "$TAG" '.insert(name.clone(), pre_v)' "$OWNER" "owner must map variable to pre_v"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-118-HAKO-MIMALLOC-SINGLE-PRED-PHI-ELISION-IMPLEMENTATION"' "$CURRENT_STATE" "current state latest card must advance to row118"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-SMALL-ALLOC-MULTI-RETURN-COPY-PROBE-296X-001"' "$CURRENT_STATE" "current state must select row119"
guard_expect_fixed_in_file "$TAG" '| 118 | `HAKO-MIMALLOC-SINGLE-PRED-PHI-ELISION-IMPLEMENTATION-296X-001` | Landed |' "$TASKBOARD" "taskboard row118 must be landed"
guard_expect_fixed_in_file "$TAG" '| 119 | `HAKO-MIMALLOC-SMALL-ALLOC-MULTI-RETURN-COPY-PROBE-296X-001` | Current |' "$TASKBOARD" "taskboard row119 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_single_pred_phi_impl.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
cat > "$tmp_dir/shape.out" <<'EOF'
output_contract=hako-mimalloc-small-alloc-phi-copy-lowering-probe-v0
single_incoming_phi_count=0
phi_count=15
copy_count=99
candidate_source=multi_return_join
summary=ok
EOF
cat > "$tmp_dir/measurement.out" <<'EOF'
output_contract=hako-mimalloc-post-rollback-inline-success-result-measurement-v0
after_hako_elapsed_median_ms=620
winner_claim=0
replacement_active=0
summary=ok
EOF

report="$tmp_dir/report.out"
python3 "$TOOL" --shape-report "$tmp_dir/shape.out" --measurement-report "$tmp_dir/measurement.out" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-single-pred-phi-elision-implementation-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'single_pred_phi_elision_enabled=1' "$report" "tool must confirm implementation"
guard_expect_fixed_in_file "$TAG" 'before_single_incoming_phi_count=61' "$report" "tool must record before count"
guard_expect_fixed_in_file "$TAG" 'after_single_incoming_phi_count=0' "$report" "tool must record after count"
guard_expect_fixed_in_file "$TAG" 'after_hako_elapsed_median_ms=620' "$report" "tool must carry measurement"
guard_expect_fixed_in_file "$TAG" 'semantic_summary=ok' "$report" "tool must record semantic summary"
guard_expect_fixed_in_file "$TAG" 'measurement_summary=ok' "$report" "tool must record measurement summary"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
