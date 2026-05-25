#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mimalloc-parallel-substrate-stress-presentation"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-212-MIMALLOC-COMPARISON-PAR-STRESS-PRESENTATION.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-211-MIMALLOC-COMPARISON-PAR-STRESS-EVIDENCE.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mimalloc_parallel_substrate_stress_presentation_guard.sh"
EVIDENCE_RUNNER="tools/allocator/mimalloc_parallel_substrate_stress_runner.py"
PRESENTATION="tools/allocator/mimalloc_parallel_substrate_stress_presentation.py"

echo "[$TAG] checking phase-295x par-stress presentation"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$EVIDENCE_RUNNER" "$PRESENTATION"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$EVIDENCE_RUNNER" "$PRESENTATION"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-PAR-STRESS-PRESENTATION-295X-002' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-PAR-STRESS-CLOSEOUT-295X-002' "$CARD" "card must select closeout follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-PAR-STRESS-PRESENTATION-295X-002' "$PREV_CARD" "previous row must select this presentation row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" 'mimalloc_parallel_substrate_stress_presentation=1' "$PRESENTATION" "presentation must publish stable contract"
guard_expect_in_file "$TAG" 'input_contract=mimalloc-comparison-par-stress-evidence-v0' "$PRESENTATION" "presentation must consume evidence contract"
guard_expect_in_file "$TAG" 'output_contract=mimalloc-comparison-par-stress-presentation-v0' "$PRESENTATION" "presentation must publish presentation contract"
guard_expect_in_file "$TAG" 'mimalloc_parallel_substrate_stress_runner=1' "$EVIDENCE_RUNNER" "presentation depends on evidence runner"
guard_expect_in_file "$TAG" '| 212 | `295x-212` | Current |' "$TASKBOARD" "taskboard must expose the presentation row"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-PAR-STRESS-PRESENTATION-295X-002:' "$TASKBOARD" "taskboard must track the presentation blocker"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_par_stress_presentation.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
evidence="$tmp_dir/par-stress.evidence"
out="$tmp_dir/par-stress.presentation"

python3 "$EVIDENCE_RUNNER" --out "$evidence" >/dev/null
python3 "$PRESENTATION" --report "$evidence" --out "$out"

rg -F -q 'mimalloc_parallel_substrate_stress_presentation=1' "$out"
rg -F -q 'output_contract=mimalloc-comparison-par-stress-presentation-v0' "$out"
rg -F -q 'input_contract=mimalloc-comparison-par-stress-evidence-v0' "$out"
rg -F -q 'worker_count=4' "$out"
rg -F -q 'iterations_per_worker=64' "$out"
rg -F -q 'expected_remote_free_count=256' "$out"
rg -F -q 'observed_remote_free_count=256' "$out"
rg -F -q 'drained_nodes=256' "$out"
rg -F -q 'payload_sum_nonzero=1' "$out"
rg -F -q 'presentation_only=1' "$out"
rg -F -q 'winner_claim=0' "$out"
rg -F -q 'summary=ok' "$out"

echo "[$TAG] ok"
