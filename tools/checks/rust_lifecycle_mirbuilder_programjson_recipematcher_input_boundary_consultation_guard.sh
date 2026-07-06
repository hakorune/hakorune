#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipematcher-input-boundary-consultation-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-input-boundary-consultation-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3225-MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-INPUT-BOUNDARY-CONSULTATION-001.md"
DESIGN_STOP_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_execution_boundary_input_design_stop_guard.sh"
PUBLICATION_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_runtime_publication_bridge_gate.sh"
RUST_MATCHER="$ROOT_DIR/src/mir/builder/control_flow/plan/recipe_tree/matcher/mod.rs"
PHASE_STATE="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_v0_phase_state_box.hako"
LOOP_HANDLER="$ROOT_DIR/lang/src/compiler/mirbuilder/stmt_handlers/loop_stmt_handler.hako"
PUBLICATION_IMPL="$ROOT_DIR/lang/src/compiler/mirbuilder/program_json_recipebodies_runtime_publication_bridge.hako"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$DESIGN_STOP_GUARD" "$PUBLICATION_GATE" "$RUST_MATCHER" "$PHASE_STATE" "$LOOP_HANDLER" "$PUBLICATION_IMPL" "$TASK_ORDER"

DESIGN_OUT="$(guard_cached_run "$TAG" bash "$DESIGN_STOP_GUARD")"
PUB_OUT="$(guard_cached_run "$TAG" bash "$PUBLICATION_GATE")"
if ! grep -q '^design_stop=1$' <<<"$DESIGN_OUT"; then
  printf '%s\n' "$DESIGN_OUT" >&2
  guard_fail "$TAG" "RecipeMatcher input design stop is not green"
fi
if ! grep -q '^runtime_recipe_bodies_publication_bridge=1$' <<<"$PUB_OUT"; then
  printf '%s\n' "$PUB_OUT" >&2
  guard_fail "$TAG" "runtime publication bridge is not green"
fi

python3 - "$FIXTURE" "$CARD" "$RUST_MATCHER" "$PHASE_STATE" "$LOOP_HANDLER" "$PUBLICATION_IMPL" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, card_path, rust_matcher_path, phase_state_path, loop_handler_path, publication_path, task_order_path = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
rust_matcher = Path(rust_matcher_path).read_text(encoding="utf-8")
phase_state = Path(phase_state_path).read_text(encoding="utf-8")
loop_handler = Path(loop_handler_path).read_text(encoding="utf-8")
publication = Path(publication_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")

token = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-INPUT-BOUNDARY-CONSULTATION-001"
next_card = "MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-INPUT-SNAPSHOT-001"

if fixture.get("kind") != "MirBuilderProgramJsonRecipeMatcherInputBoundaryConsultationV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")

inventory = fixture.get("worker_inventory") or {}
if inventory.get("rust_matcher_input") != "CanonicalLoopFacts":
    raise SystemExit("matcher input drift")
if inventory.get("publication_snapshot_enough_for_matcher_input") is not False:
    raise SystemExit("publication snapshot must not be enough")
if inventory.get("verified_recipe_path_has_narrow_input_data") is not True:
    raise SystemExit("verified recipe path must be the selected source")

if "pub fn try_match_loop(facts: &CanonicalLoopFacts)" not in rust_matcher:
    raise SystemExit("Rust matcher input authority missing")
for needle in ["recipe_root", "assign_rhs_kind", "return_kind"]:
    if needle not in phase_state:
        raise SystemExit(f"phase state evidence missing: {needle}")
for needle in ["cond_kind", "VarLtInt", "cond_var_name", "step_int"]:
    if needle not in loop_handler:
        raise SystemExit(f"loop handler evidence missing: {needle}")
for needle in ["RecipeBodiesPublicationSnapshotV1", "body_count", "def_count", "update_count"]:
    if needle not in publication:
        raise SystemExit(f"publication evidence missing: {needle}")

contracts = {row.get("id"): row for row in fixture.get("candidate_next_contracts") or []}
if contracts.get("A_PROGRAMJSON_TO_CANONICAL_LOOP_FACTS_INPUT_SNAPSHOT", {}).get("state") != "SelectedNext":
    raise SystemExit("A must be selected")
if contracts.get("A_PROGRAMJSON_TO_CANONICAL_LOOP_FACTS_INPUT_SNAPSHOT", {}).get("selected_next_card_if_eligible") != next_card:
    raise SystemExit("selected next card drift")
for rejected in [
    "B_PUBLICATION_SNAPSHOT_TO_MATCHER_INPUT_ADAPTER",
    "C_MINIMAL_HAKO_RECIPEMATCHER_OVER_PUBLICATION_SNAPSHOT",
]:
    if contracts.get(rejected, {}).get("state") != "RejectedForNow":
        raise SystemExit(f"{rejected} must be rejected for now")

snap = fixture.get("selected_snapshot_contract") or {}
if snap.get("name") != "ProgramJsonCanonicalLoopFactsInputSnapshotV1":
    raise SystemExit("snapshot contract name drift")
if snap.get("source") != "verified_recipe":
    raise SystemExit("snapshot source drift")
for field in [
    "matcher_input_present",
    "exit_has_continue",
    "exit_has_return",
    "loop_cond_continue_with_return_present",
    "cond_kind",
    "update_kind",
    "non_claims",
]:
    if field not in (snap.get("fields") or []):
        raise SystemExit(f"snapshot field missing: {field}")

decision = fixture.get("decision") or {}
if decision.get("kind") != "SelectProgramJsonCanonicalLoopFactsInputSnapshot":
    raise SystemExit("bad decision kind")
if decision.get("selected_next_card") != next_card:
    raise SystemExit("bad decision next card")

claims = fixture.get("claims") or {}
if claims.get("next_contract_selected") != 1:
    raise SystemExit("next contract claim missing")
for key, value in claims.items():
    if key == "next_contract_selected":
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [token, next_card, "A_PROGRAMJSON_TO_CANONICAL_LOOP_FACTS_INPUT_SNAPSHOT"]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
if "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-INPUT-BOUNDARY-CONSULTATION-001" not in task_order:
    raise SystemExit("task-order consultation marker missing")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipematcher-input-boundary-consultation-guard-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-INPUT-BOUNDARY-CONSULTATION-001
selected_option=A_PROGRAMJSON_TO_CANONICAL_LOOP_FACTS_INPUT_SNAPSHOT
selected_next_card=MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-INPUT-SNAPSHOT-001
snapshot_contract=ProgramJsonCanonicalLoopFactsInputSnapshotV1
snapshot_source=verified_recipe
publication_snapshot_enough_for_matcher_input=0
canonical_loop_facts_input_snapshot_implemented=0
recipe_matcher_execution=0
route_selection=0
mir_lowering=0
mir_mutation=0
id_allocation=0
runtime_route_switch=0
source_selfhost_claim=0
summary=ok
REPORT
