#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

bash tools/checks/rust_lifecycle_binding_context_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_binding_context_derived_route_selection_guard.sh

bash tools/checks/rust_lifecycle_variable_context_simple_map_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_variable_context_simple_map_derived_route_selection_guard.sh

bash tools/checks/rust_lifecycle_variable_context_immutable_borrow_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_variable_context_immutable_borrow_derived_route_selection_guard.sh

bash tools/checks/rust_lifecycle_variable_context_snapshot_restore_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_variable_context_snapshot_restore_derived_route_selection_guard.sh
bash tools/checks/rust_lifecycle_variable_context_snapshot_restore_guard.sh

cat <<'REPORT'
matrix=rust-mirbuilder-converter-v0
binding_context=green
variable_context_simple_map=green
variable_context_immutable_borrow=green
variable_context_snapshot_restore=green
shared_emitter=green
summary=ok
REPORT
