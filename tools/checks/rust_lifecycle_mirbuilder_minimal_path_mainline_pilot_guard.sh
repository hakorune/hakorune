#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-minimal-path-mainline-pilot-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

SCRIPT="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_minimal_path_mainline_pilot.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-mainline-pilot-v0.json"
READINESS="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-mainline-readiness-resolution-v0.json"
ROUTE_MANIFEST="$ROOT_DIR/lang/generated/rust_derived/hakorune_mir_builder/family_routes.json"
CLOSURE_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_minimal_path_composed_execution_closure_guard.sh"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$SCRIPT" "$FIXTURE" "$READINESS" "$ROUTE_MANIFEST" "$CLOSURE_GUARD" "$STATE" "$TASK_ORDER"

python3 -m py_compile "$SCRIPT"
bash "$CLOSURE_GUARD"
python3 "$SCRIPT" --check
bash tools/checks/current_state_pointer_guard.sh
git diff --check

echo "[$TAG] ok"
