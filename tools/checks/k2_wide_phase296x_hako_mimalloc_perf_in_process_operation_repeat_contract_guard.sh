#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-in-process-operation-repeat-contract"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_54="docs/development/current/main/phases/phase-296x/296x-54-HAKO-MIMALLOC-PERF-IN-PROCESS-OPERATION-REPEAT-CONTRACT.md"
CARD_55="docs/development/current/main/phases/phase-296x/296x-55-HAKO-MIMALLOC-PERF-IN-PROCESS-OPERATION-REPEAT-PILOT.md"
CARD_56="docs/development/current/main/phases/phase-296x/296x-56-HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
CONTRACT="tools/allocator/hako_mimalloc_in_process_operation_repeat_contract.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_in_process_operation_repeat_contract_guard.sh"

echo "[$TAG] checking phase-296x in-process operation repeat contract"

guard_require_files "$TAG" "$CARD_54" "$CARD_55" "$CARD_56" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$CONTRACT" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$CONTRACT" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_54" "contract card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_55" "pilot card must be current"
guard_expect_fixed_in_file "$TAG" 'Status: Planned' "$CARD_56" "first keeper optimization card must be planned"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-in-process-operation-repeat-contract-v0' "$CARD_54" "contract card must define output contract"
guard_expect_fixed_in_file "$TAG" 'timing_repeat_kind=in-process-operation-loop-v0' "$CARD_54" "contract card must define in-process repeat"
guard_expect_fixed_in_file "$TAG" 'process_invocation_repeat=0' "$CARD_54" "contract card must close process repeat"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-54-HAKO-MIMALLOC-PERF-IN-PROCESS-OPERATION-REPEAT-CONTRACT"' "$CURRENT_STATE" "current state latest card must advance to row 54"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-IN-PROCESS-OPERATION-REPEAT-PILOT-296X-001"' "$CURRENT_STATE" "current state must select row 55"
guard_expect_fixed_in_file "$TAG" '| 54 | `HAKO-MIMALLOC-PERF-IN-PROCESS-OPERATION-REPEAT-CONTRACT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 54 must be landed"
guard_expect_fixed_in_file "$TAG" '| 55 | `HAKO-MIMALLOC-PERF-IN-PROCESS-OPERATION-REPEAT-PILOT-296X-001` | Current |' "$TASKBOARD" "taskboard row 55 must be current"
guard_expect_fixed_in_file "$TAG" '| 56 | `HAKO-MIMALLOC-PERF-FIRST-KEEPER-OPTIMIZATION-296X-001` | Planned |' "$TASKBOARD" "taskboard row 56 must be planned"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$CONTRACT" "$INDEX" "check index must list contract tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_in_process_contract.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/contract.out"
python3 "$CONTRACT" \
  --workload representative-small-block-v0 \
  --operation-repeat 8192 \
  --process-repeat 3 \
  --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-in-process-operation-repeat-contract-v0' "$report" "contract tool must emit contract"
guard_expect_fixed_in_file "$TAG" 'measurement_profile=hako-mimalloc-in-process-operation-repeat-v0' "$report" "contract tool must emit profile"
guard_expect_fixed_in_file "$TAG" 'timing_repeat_kind=in-process-operation-loop-v0' "$report" "contract tool must emit in-process timing kind"
guard_expect_fixed_in_file "$TAG" 'operation_repeat=8192' "$report" "contract tool must emit operation repeat"
guard_expect_fixed_in_file "$TAG" 'process_repeat=3' "$report" "contract tool must emit process repeat"
guard_expect_fixed_in_file "$TAG" 'process_invocation_repeat=0' "$report" "contract tool must close process invocation repeat"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "contract tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$report" "contract tool must keep provider closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "contract tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "contract tool must end ok"

echo "[$TAG] ok"
