#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-loop-cond-co-root-pipeline-projection-policy-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-co-root-pipeline-projection-policy-v0.json"
REPORT="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1849-MIRBUILDER-LOOP-COND-CO-ROOT-PIPELINE-PROJECTION-POLICY-001.md"
SOURCE="$ROOT_DIR/src/mir/builder/control_flow/plan/features/loop_cond_co_pipeline.rs"
CALLSITE_A="$ROOT_DIR/src/mir/builder/control_flow/plan/recipe_tree/loop_cond_composer.rs"
CALLSITE_B="$ROOT_DIR/src/mir/builder/control_flow/plan/normalizer/mod.rs"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$REPORT" "$CARD" "$SOURCE" "$CALLSITE_A" "$CALLSITE_B"

python3 - <<'PY'
import json
import re
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-co-root-pipeline-projection-policy-v0.json").read_text())
report = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1849-MIRBUILDER-LOOP-COND-CO-ROOT-PIPELINE-PROJECTION-POLICY-001.md").read_text()
source = Path("src/mir/builder/control_flow/plan/features/loop_cond_co_pipeline.rs").read_text()
callsites = {
    "src/mir/builder/control_flow/plan/recipe_tree/loop_cond_composer.rs": Path("src/mir/builder/control_flow/plan/recipe_tree/loop_cond_composer.rs").read_text(),
    "src/mir/builder/control_flow/plan/normalizer/mod.rs": Path("src/mir/builder/control_flow/plan/normalizer/mod.rs").read_text(),
}

token = "MIRBUILDER-LOOP-COND-CO-ROOT-PIPELINE-PROJECTION-POLICY-001"
if fixture.get("kind") != "MirBuilderLoopCondCoRootPipelineProjectionPolicyV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token or token not in card:
    raise SystemExit("token mismatch")
if fixture.get("selected_policy", {}).get("policy") != "RootPipelineIntegrationOwner":
    raise SystemExit("selected policy drift")
if fixture.get("selected_policy", {}).get("projection_surface_selected") is not False:
    raise SystemExit("projection surface must not be selected")
if fixture.get("decision", {}).get("kind") != "KeepParentOwner":
    raise SystemExit("decision kind drift")
if fixture.get("decision", {}).get("selected_next_card") != "MIRBUILDER-LOOP-COND-CO-BLOCK-LOWERING-PROJECTION-POLICY-001":
    raise SystemExit("selected next card drift")

report_items = {
    item["source_id"]: item
    for item in report.get("items", [])
    if item.get("loop_cond_co_subcluster") == "LoopCondCoRootPipelineCluster"
}
surfaces = fixture.get("source_surfaces") or []
if len(surfaces) != 1:
    raise SystemExit("expected exactly one continue-only root pipeline surface")
if set(report_items) != {item["source_id"] for item in surfaces}:
    raise SystemExit("fixture surfaces do not match source report cluster")

for item in surfaces:
    symbol = item["symbol"]
    if not re.search(rf"pub\(in crate::mir::builder\) fn {symbol}\b", source):
        raise SystemExit(f"source visibility drift: {symbol}")
    for callsite in item.get("expected_callsites", []):
        if symbol not in callsites.get(callsite, ""):
            raise SystemExit(f"expected callsite missing: {callsite}")
    report_item = report_items[item["source_id"]]
    if report_item.get("return_type") != item["return_type"]:
        raise SystemExit(f"return type drift: {symbol}")

for marker in fixture.get("root_pipeline_evidence") or []:
    if marker not in source:
        raise SystemExit(f"root pipeline evidence marker missing: {marker}")

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
output_contract=rust-lifecycle-mirbuilder-loop-cond-co-root-pipeline-projection-policy-v0
policy=RootPipelineIntegrationOwner
decision=KeepParentOwner
projection_surface_selected=0
source_selfhost_claim=0
runtime_fallback=0
summary=ok
REPORT
