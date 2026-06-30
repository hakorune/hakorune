#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-unconverted-surface-cfg-test-filter-repair-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

REPORT="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json"
PRIORITY="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-projection-policy-cluster-priority-resolution-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1922-MIRBUILDER-UNCONVERTED-SURFACE-CFG-TEST-FILTER-REPAIR-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$REPORT" "$PRIORITY" "$CARD"

python3 - <<'PY'
import json
from pathlib import Path

token = "MIRBUILDER-UNCONVERTED-SURFACE-CFG-TEST-FILTER-REPAIR-001"
report = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json").read_text())
priority = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-projection-policy-cluster-priority-resolution-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1922-MIRBUILDER-UNCONVERTED-SURFACE-CFG-TEST-FILTER-REPAIR-001.md").read_text()

if token not in card:
    raise SystemExit("repair card token missing")

summary = report["summary"]
if summary["missing_projection_policy_count"] != 1384:
    raise SystemExit("missing projection count must exclude cfg(test) helpers")
if summary["test_only_count"] < 20:
    raise SystemExit("test-only count must include cfg(test) helpers")

expected = {
    "src/mir/builder/control_flow/plan/recipe_tree/verified.rs::return_port_contains:L106",
    "src/mir/builder/control_flow/plan/recipe_tree/verified.rs::break_port_contains:L111",
    "src/mir/builder/control_flow/plan/recipe_tree/verified.rs::continue_port_contains:L116",
}
items = {item["source_id"]: item for item in report["items"]}
for source_id in expected:
    item = items.get(source_id)
    if not item:
        raise SystemExit(f"expected cfg(test) helper missing from report: {source_id}")
    if item.get("classification") != "TestOnlySurface":
        raise SystemExit(f"cfg(test) helper not classified TestOnlySurface: {source_id}")
    if item.get("reason_token") != "TestOnlySurfaceIgnored":
        raise SystemExit(f"cfg(test) helper reason token drift: {source_id}")
    if item.get("cfg_test_surface") is not True:
        raise SystemExit(f"cfg_test_surface flag missing: {source_id}")
    if item.get("next_owner_kind") != "None" or item.get("next_card") is not None:
        raise SystemExit(f"cfg(test) helper selected next owner: {source_id}")

selected_next = priority["decision"]["selected_next_card"]
if selected_next == "MIRBUILDER-RECIPE-TREE-MATCHER-PROJECTION-POLICY-001":
    raise SystemExit("test-only RecipeTreeMatcher helpers must not drive next projection policy")
if selected_next != "MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-PROJECTION-POLICY-001":
    raise SystemExit("priority next card drift after cfg(test) filtering")

claims = report["claims"]
for key in [
    "manual_family_selection",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"non-claim must be 0: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-unconverted-surface-cfg-test-filter-repair
cfg_test_surface_excluded_from_projection_queue=1
selected_next_card=MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-PROJECTION-POLICY-001
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
