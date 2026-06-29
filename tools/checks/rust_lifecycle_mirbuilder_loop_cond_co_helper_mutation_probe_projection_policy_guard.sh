#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-loop-cond-co-helper-mutation-probe-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-co-helper-mutation-probe-projection-policy-v0.json"
REPORT="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1860-MIRBUILDER-LOOP-COND-CO-HELPER-MUTATION-PROBE-PROJECTION-POLICY-001.md"
SOURCE="$ROOT_DIR/src/mir/builder/control_flow/plan/features/loop_cond_co_helpers.rs"
CALLSITE="$ROOT_DIR/src/mir/builder/control_flow/plan/features/loop_cond_co_group_if.rs"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$REPORT" "$CARD" "$SOURCE" "$CALLSITE"

python3 - <<'PY'
import json
import re
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-co-helper-mutation-probe-projection-policy-v0.json").read_text())
report = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1860-MIRBUILDER-LOOP-COND-CO-HELPER-MUTATION-PROBE-PROJECTION-POLICY-001.md").read_text()
source = Path("src/mir/builder/control_flow/plan/features/loop_cond_co_helpers.rs").read_text()
callsite = Path("src/mir/builder/control_flow/plan/features/loop_cond_co_group_if.rs").read_text()

token = "MIRBUILDER-LOOP-COND-CO-HELPER-MUTATION-PROBE-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderLoopCondCoHelperMutationProbeProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token or token not in card:
    raise SystemExit("token mismatch")
if fixture.get("selected_policy", {}).get("policy") != "PrivateMutationProbeHelper":
    raise SystemExit("selected policy drift")
if fixture.get("selected_policy", {}).get("projection_surface_selected") is not False:
    raise SystemExit("projection surface must not be selected")
if fixture.get("decision", {}).get("kind") != "KeepParentOwner":
    raise SystemExit("decision kind drift")
if fixture.get("decision", {}).get("selected_next_card") != "MIRBUILDER-LOOP-COND-CO-HELPER-CARRIER-SYNC-PROJECTION-POLICY-001":
    raise SystemExit("next card drift")

report_items = {
    item["source_id"]: item
    for item in report.get("items", [])
    if item.get("loop_cond_co_helper_subcluster") == "LoopCondCoHelperMutationProbeCluster"
}
surfaces = fixture.get("source_surfaces") or []
if len(surfaces) != 1:
    raise SystemExit("expected exactly one mutation-probe surface")
if set(report_items) != {item["source_id"] for item in surfaces}:
    raise SystemExit("fixture surfaces do not match source report cluster")

for item in surfaces:
    symbol = item["symbol"]
    if not re.search(rf"pub\(super\) fn {symbol}\b", source):
        raise SystemExit(f"source visibility drift: {symbol}")
    if symbol not in callsite:
        raise SystemExit(f"expected external callsite missing: {symbol}")
    report_item = report_items[item["source_id"]]
    if report_item.get("return_type") != item["return_type"]:
        raise SystemExit(f"return type drift: {symbol}")
    if report_item.get("visibility") != item["visibility"]:
        raise SystemExit(f"visibility drift: {symbol}")

for marker in fixture.get("mutation_probe_evidence") or []:
    if marker not in source:
        raise SystemExit(f"mutation-probe evidence marker missing: {marker}")

claims = fixture.get("claims") or {}
for key in [
    "manual_family_selection",
    "projection_surface_selected",
    "hako_generation",
    "hako_adopted_decision",
    "native_seed_materialization",
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
output_contract=rust-lifecycle-mirbuilder-loop-cond-co-helper-mutation-probe-projection-policy-v0
policy=PrivateMutationProbeHelper
decision=KeepParentOwner
projection_surface_selected=0
source_selfhost_claim=0
runtime_fallback=0
summary=ok
REPORT
