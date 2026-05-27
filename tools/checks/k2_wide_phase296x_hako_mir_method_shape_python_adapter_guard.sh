#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mir-method-shape-python-adapter"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_85="docs/development/current/main/phases/phase-296x/296x-85-HAKO-MIR-METHOD-SHAPE-PYTHON-ADAPTER.md"
CARD_86="docs/development/current/main/phases/phase-296x/296x-86-HAKO-SOURCE-MIR-SHAPE-JOIN-ADAPTER.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/mir_check/method_shape_report.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mir_method_shape_python_adapter_guard.sh"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"

echo "[$TAG] checking MIR method shape Python adapter"

guard_require_files "$TAG" "$CARD_85" "$CARD_86" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT" "$APP" "target/release/hakorune"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT" "target/release/hakorune"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_85" "MIR adapter card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_86" "source/MIR join card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mir-method-shape-v0' "$CARD_85" "card must record output contract"
guard_expect_fixed_in_file "$TAG" 'input_kind=mir_json' "$CARD_85" "card must record MIR JSON input"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-85-HAKO-MIR-METHOD-SHAPE-PYTHON-ADAPTER"' "$CURRENT_STATE" "current state latest card must advance to row 85"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-SOURCE-MIR-SHAPE-JOIN-ADAPTER-296X-001"' "$CURRENT_STATE" "current state must select row 86"
guard_expect_fixed_in_file "$TAG" '| 85 | `HAKO-MIR-METHOD-SHAPE-PYTHON-ADAPTER-296X-001` | Landed |' "$TASKBOARD" "taskboard row 85 must be landed"
guard_expect_fixed_in_file "$TAG" '| 86 | `HAKO-SOURCE-MIR-SHAPE-JOIN-ADAPTER-296X-001` | Current |' "$TASKBOARD" "taskboard row 86 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_mir_shape.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
report="$tmp_dir/report.out"
NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null
python3 "$TOOL" --mir-json "$mir_json" --method 'HakoAllocObjectLifecyclePageQueue.selectPage/0' --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mir-method-shape-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'input_kind=mir_json' "$report" "tool must record input kind"
guard_expect_fixed_in_file "$TAG" 'selected_method=HakoAllocObjectLifecyclePageQueue.selectPage/0' "$report" "tool must select method"
guard_expect_fixed_in_file "$TAG" 'mir_instruction_count=' "$report" "tool must count instructions"
guard_expect_fixed_in_file "$TAG" 'call_count=' "$report" "tool must count calls"
guard_expect_fixed_in_file "$TAG" 'field_get_count=' "$report" "tool must count field get"
guard_expect_fixed_in_file "$TAG" 'field_set_count=' "$report" "tool must count field set"
guard_expect_fixed_in_file "$TAG" 'phi_count=' "$report" "tool must count phi"
guard_expect_fixed_in_file "$TAG" 'copy_count=' "$report" "tool must count copy"
guard_expect_fixed_in_file "$TAG" 'branch_count=' "$report" "tool must count branch"
guard_expect_fixed_in_file "$TAG" 'return_count=' "$report" "tool must count return"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
