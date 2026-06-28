#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-minimal-execution-path-frontier-resolution-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

SCRIPT="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_minimal_execution_path_frontier_resolution.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-execution-path-frontier-resolution-v0.json"
REPORT="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/minimal-mirbuilder-execution-path-semantic-closure-report-v0.json"
ROUTE="$ROOT_DIR/lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_minimal_path_composed_execution.route.json"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
ROLE_SSOT="$ROOT_DIR/docs/development/current/main/design/rust-to-hako-converter-implementation-role-ssot.md"
CONTRACT="$ROOT_DIR/tools/checks/current_state_design_stop_contract.txt"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$SCRIPT" "$FIXTURE" "$REPORT" "$ROUTE" "$STATE" "$TASK_ORDER" "$ROLE_SSOT" "$CONTRACT"

python3 -m py_compile "$SCRIPT"
python3 "$SCRIPT" --check

echo "[$TAG] ok"
