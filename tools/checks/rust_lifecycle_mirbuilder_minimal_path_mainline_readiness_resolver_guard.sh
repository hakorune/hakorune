#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-minimal-path-mainline-readiness-resolver-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

SCRIPT="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_minimal_path_mainline_readiness_resolver.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-mainline-readiness-resolution-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-1763-MIRBUILDER-MINIMAL-PATH-MAINLINE-READINESS-RESOLVER-001.md"
REPORT="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/minimal-mirbuilder-execution-path-semantic-closure-report-v0.json"
CONTINUATION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-composed-execution-continuation-v2.json"
COMPOSED_CLOSURE_VERIFIER="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-composed-execution-closure-derived-hako-verifier-result-v0.json"
COMPOSED_CLOSURE_ARTIFACT="$ROOT_DIR/lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_minimal_path_composed_execution_closure.artifact.json"
FRONTIER="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-execution-path-frontier-resolution-v0.json"
ADOPTION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-allocation-policy-hako-adoption-decision-recheck-v1.json"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
ROLE_SSOT="$ROOT_DIR/docs/development/current/main/design/rust-to-hako-converter-implementation-role-ssot.md"
CONTRACT="$ROOT_DIR/tools/checks/current_state_design_stop_contract.txt"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$SCRIPT" "$FIXTURE" "$CARD" "$REPORT" "$CONTINUATION" "$COMPOSED_CLOSURE_VERIFIER" "$COMPOSED_CLOSURE_ARTIFACT" "$FRONTIER" "$ADOPTION" "$STATE" "$TASK_ORDER" "$ROLE_SSOT" "$CONTRACT"

python3 -m py_compile "$SCRIPT"
python3 "$SCRIPT" --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
git diff --check

echo "[$TAG] ok"
