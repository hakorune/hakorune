#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-minimal-path-composed-execution-closure-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

SCRIPT="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_minimal_path_composed_execution_closure_artifacts.py"
PLAN="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-composed-execution-closure-plan-v0.json"
PROJECTION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-composed-execution-closure-execution-projection-v0.json"
ORACLE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-composed-execution-closure-derived-hako-oracle-v0.json"
RECIPE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-composed-execution-closure-derived-hako-recipe-v0.json"
VERIFIER="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-composed-execution-closure-derived-hako-verifier-result-v0.json"
HAKO="$ROOT_DIR/lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_minimal_path_composed_execution_closure.hako"
ARTIFACT="$ROOT_DIR/lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_minimal_path_composed_execution_closure.artifact.json"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$SCRIPT" "$PLAN" "$PROJECTION" "$ORACLE" "$RECIPE" "$VERIFIER" "$HAKO" "$ARTIFACT"

python3 -m py_compile "$SCRIPT"
python3 "$SCRIPT" --check
git diff --check

echo "[$TAG] ok"
