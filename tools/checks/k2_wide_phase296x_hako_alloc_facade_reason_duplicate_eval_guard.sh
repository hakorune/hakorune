#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-alloc-facade-reason-duplicate-eval-guard"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_126="docs/development/current/main/phases/phase-296x/296x-126-HAKO-ALLOC-FACADE-REASON-DUPLICATE-EVAL-GUARD.md"
CARD_127="docs/development/current/main/phases/phase-296x/296x-127-GENERIC-NESTED-ARGUMENT-SINGLE-EVAL-FIXTURE.md"
SSOT="docs/development/current/main/design/nested-argument-single-evaluation-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_alloc_facade_reason_duplicate_eval_guard.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_alloc_facade_reason_duplicate_eval_guard.sh"

echo "[$TAG] checking hako-alloc facade reason duplicate-eval guard"

guard_require_files "$TAG" "$CARD_126" "$CARD_127" "$SSOT" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_126" "row126 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_127" "row127 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-alloc-facade-reason-duplicate-eval-guard-v0' "$CARD_126" "row126 must record output contract"
guard_expect_fixed_in_file "$TAG" 'known_current_failure_count=7' "$CARD_126" "row126 must record known failures"
guard_expect_fixed_in_file "$TAG" 'known_current_unused_duplicate_reason_call_count=20' "$CARD_126" "row126 must record duplicate total"
guard_expect_fixed_in_file "$TAG" 'selected_next=generic_nested_argument_single_eval_fixture' "$CARD_126" "row126 must select generic fixture"
guard_expect_fixed_in_file "$TAG" 'Nested call arguments must be evaluated exactly once.' "$SSOT" "SSOT must define invariant"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-126-HAKO-ALLOC-FACADE-REASON-DUPLICATE-EVAL-GUARD"' "$CURRENT_STATE" "current state latest card must advance to row126"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "GENERIC-NESTED-ARGUMENT-SINGLE-EVAL-FIXTURE-296X-001"' "$CURRENT_STATE" "current state must select row127"
guard_expect_fixed_in_file "$TAG" '| 126 | `HAKO-ALLOC-FACADE-REASON-DUPLICATE-EVAL-GUARD-296X-001` | Landed |' "$TASKBOARD" "taskboard row126 must be landed"
guard_expect_fixed_in_file "$TAG" '| 127 | `GENERIC-NESTED-ARGUMENT-SINGLE-EVAL-FIXTURE-296X-001` | Current |' "$TASKBOARD" "taskboard row127 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_reason_duplicate_eval_guard.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
cat > "$tmp_dir/inventory.out" <<'REPORT'
output_contract=hako-alloc-facade-reason-duplicate-inventory-v0
method_0=objectLifecycleSmallAlloc
method_0_unused_duplicate_reason_call_count=0
failing_method_count=7
total_unused_duplicate_reason_call_count=20
failing_methods=objectLifecycleReleaseBlock
summary=ok
REPORT

report="$tmp_dir/report.out"
python3 "$TOOL" --inventory-report "$tmp_dir/inventory.out" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-alloc-facade-reason-duplicate-eval-guard-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'guard_scope=hako_alloc_object_lifecycle_facade_reason_calls' "$report" "tool must name scope"
guard_expect_fixed_in_file "$TAG" 'small_alloc_fixed=1' "$report" "tool must preserve smallAlloc fixed status"
guard_expect_fixed_in_file "$TAG" 'known_current_failure_count=7' "$report" "tool must preserve known failure count"
guard_expect_fixed_in_file "$TAG" 'selected_next=generic_nested_argument_single_eval_fixture' "$report" "tool must select generic fixture"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
