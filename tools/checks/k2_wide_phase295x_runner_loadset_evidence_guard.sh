#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-runner-loadset-evidence"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-58-MIMALLOC-COMPARISON-RUNNER-LOADSET-EVIDENCE.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-57-MIMALLOC-COMPARISON-PLUGIN-LOADSET-PREFLIGHT-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_runner_loadset_evidence_guard.sh"
RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"
PLAN_TOOL="tools/allocator/hako_plugin_loadset_plan.py"

echo "[$TAG] checking phase-295x runner loadset evidence"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$RUNNER" "$PLAN_TOOL"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$RUNNER" "$PLAN_TOOL"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-RUNNER-LOADSET-EVIDENCE-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-RUNNER-LOADSET-EVIDENCE-CLOSEOUT-295X-001' "$CARD" "card must select closeout follow-on"
guard_expect_in_file "$TAG" 'hako_selected_loadset' "$CARD" "card must define selected loadset evidence"
guard_expect_in_file "$TAG" 'hako_selected_library_count' "$CARD" "card must define library count evidence"
guard_expect_in_file "$TAG" 'hako_plugin_load_policy=eager_selected' "$CARD" "card must define eager selected evidence"
guard_expect_in_file "$TAG" 'hako_selected_loadset' "$RUNNER" "runner must emit selected loadset"
guard_expect_in_file "$TAG" 'hako_selected_library_count' "$RUNNER" "runner must emit selected library count"
guard_expect_in_file "$TAG" 'load_hako_loadset_plan' "$RUNNER" "runner must consume preflight plan"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-RUNNER-LOADSET-EVIDENCE-CLOSEOUT-295X-001' "$TASKBOARD" "taskboard must expose selected follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_runner_loadset_evidence.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
out="$tmp_dir/repeated.out"
library_path="$(guard_find_mimalloc_library "$TAG")"

python3 "$RUNNER" \
  --out "$out" \
  --sample-count 1 \
  --warmup-count 0 \
  --hako-runtime-config empty \
  --workload representative-small-block-v0 \
  --c-library "$library_path" >/dev/null

rg -F -q 'output_contract=mimalloc-comparison-repeated-measurement-v0' "$out"
rg -F -q 'hako_runtime_config_profile=empty' "$out"
rg -F -q 'hako_selected_loadset=empty' "$out"
rg -F -q 'hako_plugin_load_policy=eager_selected' "$out"
rg -F -q 'hako_selected_library_count=0' "$out"
rg -F -q 'hako_missing_library_count=0' "$out"
rg -F -q 'hako_loadset_preflight_ok=1' "$out"
rg -F -q "c_library_path=$library_path" "$out"
rg -F -q 'winner_claim=0' "$out"
rg -F -q 'summary=ok' "$out"

echo "[$TAG] ok"
