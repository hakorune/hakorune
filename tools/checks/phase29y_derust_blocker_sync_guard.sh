#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
CURRENT_TASK_DOC="$ROOT_DIR/CURRENT_TASK.md"
NEXT_PLAN_DOC="$ROOT_DIR/docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md"
LANE_MAP_DOC="$ROOT_DIR/docs/development/current/main/design/de-rust-lane-map-ssot.md"
MATRIX_DOC="$ROOT_DIR/docs/development/current/main/phases/phase-29y/81-RUST-VM-TO-HAKO-VM-FEATURE-MATRIX.md"
PHASE_README_DOC="$ROOT_DIR/docs/development/current/main/phases/phase-29y/README.md"
LANE_GATE_DOC="$ROOT_DIR/docs/development/current/main/phases/phase-29y/50-LANE-GATE-SSOT.md"
LANE_GATE_SCRIPT="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh"
LANE_GATE_QUICK_SCRIPT="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_quick_vm.sh"
PIPELINE_PARITY_GATE="tools/smokes/v2/profiles/integration/apps/phase29y_hako_using_resolver_parity_vm.sh"
source "$(dirname "$0")/lib/guard_common.sh"

TAG="phase29y-derust-blocker-sync-guard"

cd "$ROOT_DIR"

echo "[$TAG] checking de-rust blocker sync"

guard_require_command "$TAG" rg
guard_require_command "$TAG" sed
guard_require_files "$TAG" "$CURRENT_TASK_DOC" "$NEXT_PLAN_DOC" "$LANE_MAP_DOC" "$MATRIX_DOC" "$PHASE_README_DOC" "$LANE_GATE_DOC" "$LANE_GATE_SCRIPT" "$LANE_GATE_QUICK_SCRIPT"
guard_require_exec_files "$TAG" "$LANE_GATE_SCRIPT" "$ROOT_DIR/$PIPELINE_PARITY_GATE"

runtime_line=$(rg -n '^- runtime lane:' "$CURRENT_TASK_DOC" | head -n1 | cut -d: -f2- || true)
if [ -z "$runtime_line" ]; then
  guard_fail "$TAG" "CURRENT_TASK.md missing runtime lane line"
fi

task_id=$(printf '%s' "$runtime_line" | sed -n 's/.*phase-29y \/ \([^`]*\)`.*/\1/p')
capability=$(printf '%s' "$runtime_line" | sed -n 's/.*current blocker: `\([^`]*\)`.*/\1/p')

if [ -z "$task_id" ]; then
  guard_fail "$TAG" "failed to parse task id from CURRENT_TASK.md runtime lane"
fi
if [ -z "$capability" ]; then
  guard_fail "$TAG" "failed to parse blocker capability from CURRENT_TASK.md runtime lane"
fi

plan_next=$(rg -n '^- next-1:' "$NEXT_PLAN_DOC" | head -n1 | sed -n 's/.*`\([^`]*\)`.*/\1/p' || true)
if [ -z "$plan_next" ]; then
  guard_fail "$TAG" "failed to parse next-1 from 60-NEXT-TASK-PLAN.md"
fi

if [ "$plan_next" != "$task_id" ]; then
  guard_fail "$TAG" "next-task mismatch: CURRENT_TASK=$task_id, 60-NEXT-TASK-PLAN next-1=$plan_next"
fi

if ! rg -Fq "$task_id" "$NEXT_PLAN_DOC"; then
  guard_fail "$TAG" "60-NEXT-TASK-PLAN.md missing task id: $task_id"
fi
if ! rg -Fq "$task_id" "$LANE_MAP_DOC"; then
  guard_fail "$TAG" "de-rust-lane-map-ssot.md missing task id: $task_id"
fi
if ! rg -Fq "$capability" "$NEXT_PLAN_DOC"; then
  guard_fail "$TAG" "60-NEXT-TASK-PLAN.md missing blocker capability: $capability"
fi
if ! rg -Fq "$capability" "$LANE_MAP_DOC"; then
  guard_fail "$TAG" "de-rust-lane-map-ssot.md missing blocker capability: $capability"
fi

if ! rg -Fq '60-NEXT-TASK-PLAN.md' "$MATRIX_DOC"; then
  guard_fail "$TAG" "81-RUST-VM-TO-HAKO-VM-FEATURE-MATRIX.md must point to 60-NEXT-TASK-PLAN.md as next-order SSOT"
fi
if ! rg -Fq '60-NEXT-TASK-PLAN.md' "$PHASE_README_DOC"; then
  guard_fail "$TAG" "phase-29y/README.md must point to 60-NEXT-TASK-PLAN.md as blocker/next SSOT"
fi
if rg -n '^Status: .*current blocker' "$MATRIX_DOC" >/dev/null 2>&1; then
  guard_fail "$TAG" "81-RUST-VM-TO-HAKO-VM-FEATURE-MATRIX.md Status must not own current blocker text"
fi
if rg -n '^Status: .*current blocker' "$PHASE_README_DOC" >/dev/null 2>&1; then
  guard_fail "$TAG" "phase-29y/README.md Status must not own current blocker text"
fi

guard_expect_in_file "$TAG" "$PIPELINE_PARITY_GATE" "$LANE_GATE_DOC" "50-LANE-GATE-SSOT.md missing compiler pipeline parity gate reference"
if rg -Fq "$PIPELINE_PARITY_GATE" "$LANE_GATE_SCRIPT"; then
  :
elif rg -Fq "phase29y_lane_gate_quick_vm.sh" "$LANE_GATE_SCRIPT" && rg -Fq "$PIPELINE_PARITY_GATE" "$LANE_GATE_QUICK_SCRIPT"; then
  :
else
  guard_fail "$TAG" "phase29y lane gate missing compiler pipeline parity gate step (direct or via quick profile)"
fi

echo "[$TAG] ok"
