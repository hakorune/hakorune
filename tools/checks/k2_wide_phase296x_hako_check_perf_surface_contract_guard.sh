#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-check-perf-surface-contract"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_76="docs/development/current/main/phases/phase-296x/296x-76-HAKO-CHECK-PERF-SURFACE-CONTRACT.md"
CARD_77="docs/development/current/main/phases/phase-296x/296x-77-HAKO-CHECK-PERF-SURFACE-INVENTORY.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
README="tools/hako_check/README.md"
TOOL="tools/hako_check/perf_surface_contract.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_check_perf_surface_contract_guard.sh"

echo "[$TAG] checking hako_check perf-surface contract"

guard_require_files "$TAG" "$CARD_76" "$CARD_77" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$README" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_76" "contract card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_77" "inventory card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-check-perf-surface-contract-v0' "$CARD_76" "card must record contract"
guard_expect_fixed_in_file "$TAG" 'observation_only=1' "$CARD_76" "card must keep observation only"
guard_expect_fixed_in_file "$TAG" 'rewrite_executed=0' "$CARD_76" "card must forbid rewrite"
guard_expect_fixed_in_file "$TAG" 'hako_check perf-surface' "$README" "README must document perf-surface"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-check-perf-surface-contract-v0' "$README" "README must include contract"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-76-HAKO-CHECK-PERF-SURFACE-CONTRACT"' "$CURRENT_STATE" "current state latest card must advance to row 76"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-CHECK-PERF-SURFACE-INVENTORY-296X-001"' "$CURRENT_STATE" "current state must select row 77"
guard_expect_fixed_in_file "$TAG" '| 76 | `HAKO-CHECK-PERF-SURFACE-CONTRACT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 76 must be landed"
guard_expect_fixed_in_file "$TAG" '| 77 | `HAKO-CHECK-PERF-SURFACE-INVENTORY-296X-001` | Current |' "$TASKBOARD" "taskboard row 77 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_hako_check_contract.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-check-perf-surface-contract-v0' "$report" "tool must emit contract"
guard_expect_fixed_in_file "$TAG" 'tool_surface=hako_check_perf_surface' "$report" "tool must name surface"
guard_expect_fixed_in_file "$TAG" 'observation_only=1' "$report" "tool must be observation only"
guard_expect_fixed_in_file "$TAG" 'rewrite_executed=0' "$report" "tool must not rewrite"
guard_expect_fixed_in_file "$TAG" 'linear_search_candidate=0|1' "$report" "tool must define linear search field"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
