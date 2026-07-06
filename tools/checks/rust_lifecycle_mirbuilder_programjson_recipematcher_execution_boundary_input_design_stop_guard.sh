#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipematcher-execution-boundary-input-design-stop-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-execution-boundary-input-design-stop-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3224-MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-EXECUTION-BOUNDARY-INPUT-DESIGN-STOP-001.md"
RUST_MATCHER="$ROOT_DIR/src/mir/builder/control_flow/plan/recipe_tree/matcher/mod.rs"
RUST_CONTRACT="$ROOT_DIR/src/mir/builder/control_flow/plan/recipe_tree/contracts.rs"
RUST_RULES="$ROOT_DIR/src/mir/builder/control_flow/plan/single_planner/rules.rs"
PUBLICATION_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_recipebodies_runtime_publication_bridge.hako"
PUBLICATION_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_runtime_publication_bridge_gate.sh"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$RUST_MATCHER" "$RUST_CONTRACT" "$RUST_RULES" "$PUBLICATION_IMPL" "$PUBLICATION_GATE" "$TASK_ORDER"

PUB_OUT="$(guard_cached_run "$TAG" bash "$PUBLICATION_GATE")"
if ! grep -q '^runtime_recipe_bodies_publication_bridge=1$' <<<"$PUB_OUT"; then
  printf '%s\n' "$PUB_OUT" >&2
  guard_fail "$TAG" "publication bridge prerequisite is not green"
fi

python3 - "$FIXTURE" "$CARD" "$RUST_MATCHER" "$RUST_CONTRACT" "$RUST_RULES" "$PUBLICATION_IMPL" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, card_path, matcher_path, contract_path, rules_path, publication_path, task_order_path = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
matcher = Path(matcher_path).read_text(encoding="utf-8")
contract = Path(contract_path).read_text(encoding="utf-8")
rules = Path(rules_path).read_text(encoding="utf-8")
publication = Path(publication_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")

token = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-EXECUTION-BOUNDARY-INPUT-DESIGN-STOP-001"

if fixture.get("kind") != "MirBuilderProgramJsonRecipeMatcherExecutionBoundaryInputDesignStopV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")

for needle in [
    "pub fn try_match_loop(facts: &CanonicalLoopFacts)",
    "RecipeContractKind::LoopWithExit",
]:
    if needle not in matcher:
        raise SystemExit(f"Rust matcher authority missing: {needle}")
for needle in [
    "enum RecipeContractKind",
    "LoopWithExit",
]:
    if needle not in contract:
        raise SystemExit(f"Rust contract authority missing: {needle}")
for needle in [
    "RecipeMatcher::try_match_loop(facts)",
    "outcome.recipe_contract",
]:
    if needle not in rules:
        raise SystemExit(f"planner call site missing: {needle}")
for needle in [
    "RecipeBodiesPublicationSnapshotV1",
    "recipe_matcher_executed",
]:
    if needle not in publication:
        raise SystemExit(f"publication boundary missing: {needle}")

source = fixture.get("source_authority") or {}
if source.get("matcher_input") != "CanonicalLoopFacts":
    raise SystemExit("fixture matcher input drift")
if source.get("matcher_output") != "RecipeContractKind::LoopWithExit":
    raise SystemExit("fixture matcher output drift")
boundary = fixture.get("current_programjson_boundary") or {}
if boundary.get("publication_output") != "RecipeBodiesPublicationSnapshotV1":
    raise SystemExit("publication output drift")
if boundary.get("publication_snapshot_is_not_matcher_input") != 1:
    raise SystemExit("publication must not be treated as matcher input")

states = {row.get("id"): row.get("state") for row in fixture.get("candidate_next_contracts") or []}
if states.get("A_PROGRAMJSON_TO_CANONICAL_LOOP_FACTS_PROJECTION_BRIDGE") != "ConsultationRequired":
    raise SystemExit("A must require consultation")
if states.get("B_PUBLICATION_SNAPSHOT_TO_MATCHER_INPUT_ADAPTER") != "ConsultationRequired":
    raise SystemExit("B must require consultation")
if states.get("C_MINIMAL_HAKO_RECIPEMATCHER_OVER_PUBLICATION_SNAPSHOT") != "Risky":
    raise SystemExit("C must be marked risky")

claims = fixture.get("claims") or {}
if claims.get("design_stop") != 1:
    raise SystemExit("design stop claim missing")
for key, value in claims.items():
    if key == "design_stop":
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [
    token,
    "RecipeMatcher::try_match_loop(facts: &CanonicalLoopFacts)",
    "RecipeBodiesPublicationSnapshotV1",
    "CONSULTATION_REQUIRED",
]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
if "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-EXECUTION-BOUNDARY-MINIMAL-001" not in task_order:
    raise SystemExit("task-order attempted matcher boundary marker missing")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipematcher-execution-boundary-input-design-stop-guard-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-EXECUTION-BOUNDARY-INPUT-DESIGN-STOP-001
design_stop=1
rust_matcher_input=CanonicalLoopFacts
rust_matcher_output=RecipeContractKind::LoopWithExit
publication_snapshot_is_not_matcher_input=1
candidate_a=PROGRAMJSON_TO_CANONICAL_LOOP_FACTS_PROJECTION_BRIDGE
candidate_b=PUBLICATION_SNAPSHOT_TO_MATCHER_INPUT_ADAPTER
candidate_c=MINIMAL_HAKO_RECIPEMATCHER_OVER_PUBLICATION_SNAPSHOT_RISKY
selected_next_card=CONSULTATION_REQUIRED
recipe_matcher_execution=0
route_selection=0
mir_lowering=0
mir_mutation=0
id_allocation=0
runtime_route_switch=0
source_selfhost_claim=0
summary=ok
REPORT
