#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-first-keeper-optimization"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_58="docs/development/current/main/phases/phase-296x/296x-58-HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION.md"
CARD_59="docs/development/current/main/phases/phase-296x/296x-59-HAKO-MIMALLOC-PERF-POST-KEEPER-TAXONOMY-REFRESH.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
APP="apps/hako-alloc-mimalloc-comparison-in-process-small-block-proof/main.hako"
PILOT="tools/allocator/hako_mimalloc_in_process_operation_repeat_pilot.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_first_keeper_optimization_guard.sh"

echo "[$TAG] checking phase-296x first keeper optimization"

guard_require_files "$TAG" "$CARD_58" "$CARD_59" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$APP" "$PILOT" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$PILOT" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_58" "first keeper card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_59" "post-keeper taxonomy card must be current"
guard_expect_fixed_in_file "$TAG" 'optimization_kind=page_model_reuse_via_reset_to_fresh' "$CARD_58" "card must name optimization"
guard_expect_fixed_in_file "$TAG" 'hako_external_elapsed_median_ms=280' "$CARD_58" "card must record after median"
guard_expect_fixed_in_file "$TAG" 'external_elapsed_median_gap_ms=276' "$CARD_58" "card must record remaining gap"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_58" "card must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'page.resetToFresh()' "$APP" "app must reuse page via resetToFresh"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-58-HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION"' "$CURRENT_STATE" "current state latest card must advance to row 58"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-POST-KEEPER-TAXONOMY-REFRESH-296X-001"' "$CURRENT_STATE" "current state must select row 59"
guard_expect_fixed_in_file "$TAG" '| 58 | `HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION-296X-001` | Landed |' "$TASKBOARD" "taskboard row 58 must be landed"
guard_expect_fixed_in_file "$TAG" '| 59 | `HAKO-MIMALLOC-PERF-POST-KEEPER-TAXONOMY-REFRESH-296X-001` | Current |' "$TASKBOARD" "taskboard row 59 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_first_keeper.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/pilot.out"
python3 "$PILOT" \
  --out "$report" \
  --c-library /lib/x86_64-linux-gnu/libmimalloc.so.2 \
  --operation-repeat 8192 \
  --process-repeat 3 >/dev/null

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-in-process-operation-repeat-measurement-v0' "$report" "pilot must emit measurement"
guard_expect_fixed_in_file "$TAG" 'operation_repeat=8192' "$report" "pilot must preserve operation repeat"
guard_expect_fixed_in_file "$TAG" 'process_invocation_repeat=0' "$report" "pilot must keep process repeat closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "pilot must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$report" "pilot must keep provider closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "pilot must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "pilot must end ok"

echo "[$TAG] ok"
