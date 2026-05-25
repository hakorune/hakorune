#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mimalloc-parallel-substrate-stress-evidence"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-211-MIMALLOC-COMPARISON-PAR-STRESS-EVIDENCE.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-210-MIMALLOC-COMPARISON-PAR-STRESS-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mimalloc_parallel_substrate_stress_evidence_guard.sh"
RUNNER="tools/allocator/mimalloc_parallel_substrate_stress_runner.py"
TEST_FILE="crates/nyash_kernel/src/tests/mimalloc_parallel_substrate.rs"

echo "[$TAG] checking phase-295x par-stress evidence"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$RUNNER" "$TEST_FILE"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$RUNNER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-PAR-STRESS-EVIDENCE-295X-002' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-PAR-STRESS-PRESENTATION-295X-002' "$CARD" "card must select presentation follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-PAR-STRESS-EVIDENCE-295X-002' "$PREV_CARD" "previous row must select this evidence row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" 'mimalloc_parallel_substrate_stress_runner=1' "$RUNNER" "runner must publish stable stress evidence"
guard_expect_in_file "$TAG" 'output_contract=mimalloc-comparison-par-stress-evidence-v0' "$RUNNER" "runner must publish evidence contract"
guard_expect_in_file "$TAG" 'cargo_test_filter=mimalloc_parallel_substrate_stress' "$RUNNER" "runner must target the native stress test"
guard_expect_fixed_in_file "$TAG" 'println!("mimalloc_parallel_substrate_stress=1");' "$TEST_FILE" "stress test must emit a stable evidence marker"
guard_expect_fixed_in_file "$TAG" 'println!("payload_sum_nonzero={}", if payload_sum != 0 { 1 } else { 0 });' "$TEST_FILE" "stress test must emit payload summary"
guard_expect_in_file "$TAG" '| 211 | `295x-211` | Landed |' "$TASKBOARD" "taskboard must retain the evidence row as landed"
guard_expect_in_file "$TAG" '| 212 | `295x-212` | Current |' "$TASKBOARD" "taskboard must expose the presentation row"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-PAR-STRESS-PRESENTATION-295X-002:' "$TASKBOARD" "taskboard must track the current presentation blocker"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_par_stress_evidence.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
out="$tmp_dir/par-stress.out"

python3 "$RUNNER" --out "$out"

rg -F -q 'mimalloc_parallel_substrate_stress_runner=1' "$out"
rg -F -q 'output_contract=mimalloc-comparison-par-stress-evidence-v0' "$out"
rg -F -q 'cargo_test_target=nyash_kernel' "$out"
rg -F -q 'cargo_test_filter=mimalloc_parallel_substrate_stress' "$out"
rg -F -q 'cargo_test_passed=1' "$out"
rg -F -q 'worker_count=4' "$out"
rg -F -q 'iterations_per_worker=64' "$out"
rg -F -q 'expected_remote_free_count=256' "$out"
rg -F -q 'observed_remote_free_count=256' "$out"
rg -F -q 'drained_nodes=256' "$out"
rg -F -q 'payload_sum_nonzero=1' "$out"
rg -F -q 'winner_claim=0' "$out"
rg -F -q 'summary=ok' "$out"

echo "[$TAG] ok"
