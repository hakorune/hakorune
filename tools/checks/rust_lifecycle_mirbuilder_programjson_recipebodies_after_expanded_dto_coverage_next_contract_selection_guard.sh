#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipebodies-after-expanded-dto-coverage-next-contract-selection-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-after-expanded-dto-coverage-next-contract-selection-v0.json"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_verifier_boundary_expanded_dto_coverage_parity_gate.sh"
RETIRE_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_verifier_boundary_expanded_dto_coverage_retire_rust_astnode_projector_candidate_guard.sh"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3222-MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-EXPANDED-DTO-COVERAGE-NEXT-CONTRACT-SELECTION-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$PARITY_GATE" "$RETIRE_GUARD" "$CARD" "$TASK_ORDER" "$CURRENT_STATE"

PARITY_OUT="$(guard_cached_run "$TAG" bash "$PARITY_GATE")"
RETIRE_OUT="$(guard_cached_run "$TAG" bash "$RETIRE_GUARD")"
if ! grep -q '^expanded_dto_coverage_rows=2$' <<<"$PARITY_OUT"; then
  printf '%s\n' "$PARITY_OUT" >&2
  guard_fail "$TAG" "expanded DTO coverage row count drift"
fi
if ! grep -q '^retire_candidate_recorded=1$' <<<"$RETIRE_OUT"; then
  printf '%s\n' "$RETIRE_OUT" >&2
  guard_fail "$TAG" "expanded DTO retire-candidate is not green"
fi
if ! grep -q '^rust_projector_runtime_dependency_removed=0$' <<<"$RETIRE_OUT"; then
  printf '%s\n' "$RETIRE_OUT" >&2
  guard_fail "$TAG" "runtime dependency removal must still be unclaimed"
fi

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, card_path, task_order_path, current_state_path = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")
current_state = Path(current_state_path).read_text(encoding="utf-8")

token = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-EXPANDED-DTO-COVERAGE-NEXT-CONTRACT-SELECTION-001"
next_card = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RUNTIME-PUBLICATION-BRIDGE-001"
rows = [
    "local_loop_body_if_branch_return",
    "local_loop_body_if_branch_return_alt_names",
]

if fixture.get("kind") != "MirBuilderProgramJsonRecipeBodiesAfterExpandedDtoCoverageNextContractSelectionV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")

previous = fixture.get("previous_state") or {}
expected_previous = {
    "expanded_dto_coverage_rows": 2,
    "retire_candidate_recorded": 1,
    "runtime_dependency_removed": 0,
    "runtime_recipe_bodies_publication": 0,
    "full_recipe_matcher_execution": 0,
    "runtime_route_switch": 0,
    "source_selfhost_claim": 0,
}
for key, value in expected_previous.items():
    if previous.get(key) != value:
        raise SystemExit(f"previous state drift: {key}")

contracts = {row.get("contract"): row for row in fixture.get("candidate_next_contracts") or []}
if contracts.get("RuntimeRecipeBodiesPublicationBridge", {}).get("selection_eligible") is not True:
    raise SystemExit("runtime publication bridge must be selected eligible")
if contracts.get("RuntimeRecipeBodiesPublicationBridge", {}).get("selected_next_card_if_eligible") != next_card:
    raise SystemExit("selected next card drift")
for rejected in [
    "MoreVerifierBoundaryDtoCoverage",
    "FullRecipeMatcherExecutionMinimal",
    "DesignCleanupOnly",
]:
    if contracts.get(rejected, {}).get("selection_eligible") is not False:
        raise SystemExit(f"{rejected} must not be selected")

decision = fixture.get("decision") or {}
if decision.get("kind") != "SelectRuntimeRecipeBodiesPublicationBridge":
    raise SystemExit("bad decision kind")
if decision.get("selected_next_card") != next_card:
    raise SystemExit("bad decision next card")

acceptance = fixture.get("acceptance_for_next_card") or {}
for key in [
    "must_publish_recipe_bodies_publication_snapshot_v1",
    "must_use_readonly_result_map_or_map_handle_boundary",
    "must_preserve_verifier_boundary_used",
    "must_preserve_verified_recipe_present",
    "must_keep_recipe_matcher_executed_zero",
    "must_keep_runtime_route_switch_zero",
]:
    if acceptance.get(key) != 1:
        raise SystemExit(f"acceptance missing: {key}")
if acceptance.get("covered_rows") != rows:
    raise SystemExit("covered rows drift")

for key, value in (fixture.get("forbidden_in_next_card") or {}).items():
    if value != 1:
        raise SystemExit(f"forbidden flag drift: {key}")

claims = fixture.get("claims") or {}
if claims.get("next_contract_selected") != 1:
    raise SystemExit("next contract claim missing")
for key, value in claims.items():
    if key == "next_contract_selected":
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for needle in [token, next_card, "B_RUNTIME_RECIPEBODIES_PUBLICATION_BRIDGE"]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
for needle in [
    "3222 selects runtime RecipeBodies publication bridge as the next contract",
    "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RUNTIME-PUBLICATION-BRIDGE-001",
    "3221 marks the expanded RecipeBodies verifier-boundary DTO coverage rows",
]:
    if needle not in task_order:
        raise SystemExit(f"task-order missing: {needle}")
allowed_latest = [
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-VERIFIER-BOUNDARY-EXPANDED-DTO-COVERAGE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001"',
    f'latest_card = "{token}"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RUNTIME-PUBLICATION-BRIDGE-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-EXECUTION-BOUNDARY-INPUT-DESIGN-STOP-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-INPUT-BOUNDARY-CONSULTATION-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-INPUT-SNAPSHOT-AOT-BOUNDARY-DESIGN-STOP-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-INPUT-SNAPSHOT-MAPBOX-PUBLICATION-BRIDGE-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-EXECUTION-BOUNDARY-MINIMAL-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-SHADOW-PARITY-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RUNTIME-ROUTE-SHADOW-SWITCH-DESIGN-STOP-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-DUAL-RUN-SHADOW-GUARD-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-SHADOW-PARITY-EXPANDED-ROWS-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RUNTIME-ROUTE-SHADOW-SWITCH-CONSULTATION-002"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-ROUTE-ADJACENT-SHADOW-GUARD-001"',
    'latest_card = "MIRBUILDER-COMPARE-BOOLRECIPE-TO-MIR-COMPARE-BRANCH-CLOSEOUT-001"',
    'latest_card = "MIRBUILDER-COMPARE-RUNTIME-ROUTE-AUTHORITY-DESIGN-STOP-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-ROUTE-ADJACENT-SHADOW-GUARD-REFRESH-001"',
    'latest_card = "GUARD-CACHE-EMIT-EXE-AND-DIRTY-MEMO-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-AUTHORITY-SWITCH-COVERAGE-FLOOR-SELECTION-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ACCEPTED-FLOOR-MATRIX-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-CONTINUE-PRESENT-ROW-SHAPE-DESIGN-STOP-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-LOOP-BODY-IFCONTINUE-IFRETURN-ASSIGNMENT-BOXCOUNT-ACCEPTED-FLOOR-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-BREAK-PRESENT-VERIFIED-RECIPE-SUPPORT-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-BREAK-CONTINUE-PRESENT-VERIFIED-RECIPE-SUPPORT-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-DECISION-ROW-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-ROUTE-RELEASE-CONSULTATION-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-LOOP-BODY-RETURN-ABSENT-SCAN-ONLY-DIAGNOSTIC-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-FINAL-TOPLEVEL-RETURN-DECOUPLE-SNAPSHOT-BOUNDARY-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-ACCEPTED-FLOOR-001"',
]
if not any(needle in current_state for needle in allowed_latest):
    raise SystemExit("current state latest card drift")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipebodies-after-expanded-dto-coverage-next-contract-selection-guard-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-EXPANDED-DTO-COVERAGE-NEXT-CONTRACT-SELECTION-001
selected_option=B_RUNTIME_RECIPEBODIES_PUBLICATION_BRIDGE
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RUNTIME-PUBLICATION-BRIDGE-001
expanded_dto_coverage_rows=2
retire_candidate_recorded=1
runtime_dependency_removed=0
runtime_recipe_bodies_publication_bridge=0
read_only_publication_snapshot=0
full_recipe_matcher_execution=0
route_selection=0
mir_lowering=0
mir_mutation=0
id_allocation=0
runtime_route_switch=0
runtime_fallback=0
source_selfhost_claim=0
summary=ok
REPORT
