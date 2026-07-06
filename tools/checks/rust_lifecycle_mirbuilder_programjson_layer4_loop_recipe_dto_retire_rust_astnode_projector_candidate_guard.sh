#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-layer4-loop-recipe-dto-retire-rust-astnode-projector-candidate-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-layer4-loop-recipe-dto-retire-rust-astnode-projector-candidate-v0.json"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_loop_recipe_dto_parity_gate.sh"
HEAVY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_loop_recipe_dto_heavy_exe_readiness_gate.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$PARITY_GATE" "$HEAVY_GATE"

bash "$PARITY_GATE" >/dev/null
HEAVY_OUT="$(bash "$HEAVY_GATE")"
if ! grep -q '^runtime_parity_green=1$' <<<"$HEAVY_OUT"; then
  printf '%s\n' "$HEAVY_OUT" >&2
  guard_fail "$TAG" "heavy runtime parity is not green"
fi
if ! grep -q '^exact_first_blocker=none$' <<<"$HEAVY_OUT"; then
  printf '%s\n' "$HEAVY_OUT" >&2
  guard_fail "$TAG" "heavy readiness still has a blocker"
fi

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))

if fixture.get("kind") != "MirBuilderProgramJsonLayer4LoopRecipeDtoRustAstNodeProjectorRetireCandidateV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "MIRBUILDER-PROGRAMJSON-LAYER4-LOOP-RECIPE-DTO-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001":
    raise SystemExit("bad fixture token")

evidence = fixture.get("evidence") or {}
expected_hashes = {
    "hako_snapshot_source_hash": "sha256:55936df225b43256df55023d83c4ed486ab3f9e7a70b4d43b7935ef482327c68",
    "parity_fixture_hash": "sha256:8387e9cf31bd857182a75de4db57059f314c137df515f99e3ad98d1dd52a645a",
    "parity_gate_hash": "sha256:84fbbe77e419e07b68d22fba1dc8dae3eaf7ec08a3ca74ca6200caf770066168",
    "heavy_readiness_gate_hash": "sha256:db42d7a96b21eb6349369311e5ab748740bf2aecb873599ac75ebf9e7a2fb7f1",
    "scanner_source_hash": "sha256:2a90e9325f5536518f1c210152265ede0ee349fd34ae2291624db779cb2fb964",
}
for key, expected in expected_hashes.items():
    if evidence.get(key) != expected:
        raise SystemExit(f"hash drift: {key}")

scope = fixture.get("retire_candidate_scope") or {}
if scope.get("retire_candidate") != "LoopRecipeDtoSnapshotV1":
    raise SystemExit("bad retire candidate")
if scope.get("shape_scope") != "covered ProgramJSON Layer4 loop Recipe DTO rows":
    raise SystemExit("bad shape scope")
if scope.get("programjson_snapshot_owner") != "ProgramJsonLoopRecipeDtoSnapshotV1":
    raise SystemExit("bad snapshot owner")

expected_rows = [
    "local_loop_assignment_return_var",
    "local_loop_assignment_return_int",
    "local_loop_if_then_assignment_return_var",
    "loop_without_local_reject",
]
if scope.get("covered_rows") != expected_rows:
    raise SystemExit("covered rows drift")
if scope.get("rust_projector_runtime_dependency_removed") != 0:
    raise SystemExit("runtime dependency removal must stay unclaimed")
if scope.get("rust_projector_oracle_only") != 1:
    raise SystemExit("rust oracle marker missing")
if scope.get("full_astnode_projector_retired") != 0:
    raise SystemExit("full ASTNode projector retirement must stay unclaimed")

criteria = fixture.get("criteria") or {}
expected_criteria = {
    "programjson_snapshot_parity_gate": "Green",
    "heavy_runtime_parity_gate": "Green",
    "heavy_runtime_parity_green": 1,
    "programjson_route_traverses_programjson": 1,
    "structured_recipe_dto_constructed": 1,
    "programjson_route_uses_string_only_facade": 0,
    "covered_row_count": 4,
    "full_recipe_matcher_execution": 0,
    "route_selection": 0,
    "mir_mutation": 0,
    "id_allocation": 0,
    "backend_lowering": 0,
}
for key, expected in expected_criteria.items():
    if criteria.get(key) != expected:
        raise SystemExit(f"criteria drift: {key}")

claims = fixture.get("claims") or {}
for key in [
    "source_selfhost_claim",
    "mir_mutation",
    "id_allocation",
    "backend_lowering",
    "route_selection",
    "full_recipe_matcher_execution",
    "runtime_route_switch",
    "hako_adopted_decision",
    "programjson_full_parser_claim",
    "programjson_all_shapes_supported",
    "rust_astnode_projector_fully_retired",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

decision = fixture.get("decision") or {}
if decision.get("kind") != "RetireCandidateScoped":
    raise SystemExit("bad decision kind")
if decision.get("selected_next_card") != "MIRBUILDER-PROGRAMJSON-LAYER4-NEXT-RECIPE-DTO-CAPABILITY-SELECTION-001":
    raise SystemExit("bad selected next card")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-layer4-loop-recipe-dto-retire-rust-astnode-projector-candidate-guard-v0
token=MIRBUILDER-PROGRAMJSON-LAYER4-LOOP-RECIPE-DTO-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
retire_candidate=LoopRecipeDtoSnapshotV1
shape_scope=covered ProgramJSON Layer4 loop Recipe DTO rows
covered_rows=4
decision=RetireCandidateScoped
parity_gate=green
heavy_runtime_parity_green=1
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
full_recipe_matcher_execution=0
route_selection=0
mir_mutation=0
id_allocation=0
backend_lowering=0
source_selfhost_claim=0
hako_adopted_decision=0
programjson_full_parser_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-LAYER4-NEXT-RECIPE-DTO-CAPABILITY-SELECTION-001
summary=ok
REPORT
