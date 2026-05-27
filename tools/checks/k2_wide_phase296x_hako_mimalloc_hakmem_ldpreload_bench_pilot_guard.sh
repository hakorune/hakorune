#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-hakmem-ldpreload-bench-pilot"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_74="docs/development/current/main/phases/phase-296x/296x-74-HAKO-MIMALLOC-HAKMEM-LDPRELOAD-BENCH-PILOT.md"
CARD_75="docs/development/current/main/phases/phase-296x/296x-75-HAKO-MIMALLOC-PERF-PARITY-SELFHOST-HANDOFF-GATE.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_hakmem_ldpreload_bench_pilot.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_hakmem_ldpreload_bench_pilot_guard.sh"
HAKMEM_ROOT="/home/tomoaki/git/hakmem_20260525_extracted/hakmem"

echo "[$TAG] checking phase-296x hakmem LD_PRELOAD bench pilot"

guard_require_files "$TAG" "$CARD_74" "$CARD_75" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT" "$HAKMEM_ROOT/bench_random_mixed_system"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT" "$HAKMEM_ROOT/bench_random_mixed_system"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_74" "bench pilot card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_75" "selfhost handoff card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-hakmem-ldpreload-bench-pilot-v0' "$CARD_74" "card must record bench pilot contract"
guard_expect_fixed_in_file "$TAG" 'hakmem_script_compatible=probe-only' "$CARD_74" "card must keep probe-only scope"
guard_expect_fixed_in_file "$TAG" 'ld_preload_env_applied=1' "$CARD_74" "card must apply LD_PRELOAD"
guard_expect_fixed_in_file "$TAG" 'benchmark_sample_executed=1' "$CARD_74" "card must execute benchmark sample"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_74" "card must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$CARD_74" "card must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_74" "card must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_74" "card must keep winner closed"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-74-HAKO-MIMALLOC-HAKMEM-LDPRELOAD-BENCH-PILOT"' "$CURRENT_STATE" "current state latest card must advance to row 74"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-PARITY-SELFHOST-HANDOFF-GATE-296X-001"' "$CURRENT_STATE" "current state must select selfhost handoff gate"
guard_expect_fixed_in_file "$TAG" '| 74 | `HAKO-MIMALLOC-HAKMEM-LDPRELOAD-BENCH-PILOT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 74 must be landed"
guard_expect_fixed_in_file "$TAG" '| 75 | `HAKO-MIMALLOC-PERF-PARITY-SELFHOST-HANDOFF-GATE-296X-001` | Current |' "$TASKBOARD" "taskboard row 75 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list bench pilot tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_ldpreload_bench.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --hakmem-root "$HAKMEM_ROOT" --out-dir "$tmp_dir/pilot" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-hakmem-ldpreload-bench-pilot-v0' "$report" "tool must emit bench pilot contract"
guard_expect_fixed_in_file "$TAG" 'hakmem_script_compatible=probe-only' "$report" "tool must keep probe-only scope"
guard_expect_fixed_in_file "$TAG" 'benchmark_id=bench_random_mixed_system' "$report" "tool must select benchmark"
guard_expect_fixed_in_file "$TAG" 'ld_preload_env_applied=1' "$report" "tool must apply LD_PRELOAD"
guard_expect_fixed_in_file "$TAG" 'benchmark_sample_executed=1' "$report" "tool must execute sample"
guard_expect_fixed_in_file "$TAG" 'benchmark_exit_code=0' "$report" "tool must exit ok"
guard_expect_fixed_in_file "$TAG" 'throughput_ops_per_sec=' "$report" "tool must capture throughput"
guard_expect_fixed_in_file "$TAG" 'hakorune_default_replacement_active=0' "$report" "tool must keep Hakorune default unchanged"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$report" "tool must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$report" "tool must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'next_row=HAKO-MIMALLOC-PERF-PARITY-SELFHOST-HANDOFF-GATE-296X-001' "$report" "tool must select handoff gate"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
