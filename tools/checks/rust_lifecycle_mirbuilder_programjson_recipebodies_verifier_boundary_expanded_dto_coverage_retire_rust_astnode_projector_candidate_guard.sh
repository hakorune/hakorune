#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipebodies-verifier-boundary-expanded-dto-coverage-retire-rust-astnode-projector-candidate-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-verifier-boundary-expanded-dto-coverage-retire-rust-astnode-projector-candidate-v0.json"
PARITY_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipebodies-verifier-boundary-expanded-dto-coverage-parity-v0.json"
PARITY_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_verifier_boundary_expanded_dto_coverage_parity_gate.sh"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3221-MIRBUILDER-PROGRAMJSON-RECIPEBODIES-VERIFIER-BOUNDARY-EXPANDED-DTO-COVERAGE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$PARITY_FIXTURE" "$PARITY_GATE" "$CARD" "$TASK_ORDER"

PARITY_OUT="$(guard_cached_run "$TAG" bash "$PARITY_GATE")"
if ! grep -q '^summary=ok$' <<<"$PARITY_OUT"; then
  printf '%s\n' "$PARITY_OUT" >&2
  guard_fail "$TAG" "expanded DTO coverage parity gate is not green"
fi

python3 - "$FIXTURE" "$PARITY_FIXTURE" "$CARD" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, parity_path, card_path, task_order_path = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
parity = json.loads(Path(parity_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")

token = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-VERIFIER-BOUNDARY-EXPANDED-DTO-COVERAGE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001"
next_card = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-EXPANDED-DTO-COVERAGE-NEXT-CONTRACT-SELECTION-001"
rows = [
    "local_loop_body_if_branch_return",
    "local_loop_body_if_branch_return_alt_names",
]

if fixture.get("kind") != "MirBuilderProgramJsonRecipeBodiesVerifierBoundaryExpandedDtoCoverageRustAstNodeProjectorRetireCandidateV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")
if fixture.get("input_state", {}).get("snapshot_kind") != "ProgramJsonRecipeBodiesVerifierBoundarySnapshotV1":
    raise SystemExit("snapshot kind drift")
if fixture.get("input_state", {}).get("programjson_owner") != "ProgramJsonRecipeBodiesVerifierBoundarySnapshotBox":
    raise SystemExit("owner drift")

parity_rows = [row.get("row_id") for row in parity.get("rows") or []]
if parity_rows != rows:
    raise SystemExit("parity row drift")
if parity.get("claims", {}).get("expanded_dto_coverage_parity_green") != 1:
    raise SystemExit("expanded DTO coverage parity not proven")

candidate = fixture.get("retire_candidate") or {}
if candidate.get("covered_rows") != rows:
    raise SystemExit("covered rows drift")
if candidate.get("rust_projector_runtime_dependency_removed") != 0:
    raise SystemExit("runtime dependency removal must remain 0")
if candidate.get("rust_projector_oracle_only_candidate") != 1:
    raise SystemExit("oracle-only candidate flag missing")
if candidate.get("full_astnode_projector_retired") != 0:
    raise SystemExit("full projector retirement must remain 0")

evidence = fixture.get("evidence") or {}
for key in [
    "runtime_parity_green",
    "existing_snapshot_owner_reused",
    "recipe_verifier_boundary_used",
    "verified_recipe_present",
]:
    if evidence.get(key) != 1:
        raise SystemExit(f"evidence missing: {key}")
if evidence.get("expanded_dto_coverage_rows") != 2:
    raise SystemExit("expanded row count must be 2")
if evidence.get("body_count") != 4:
    raise SystemExit("body_count must be 4")
if evidence.get("def_count") != 1 or evidence.get("update_count") != 2:
    raise SystemExit("port signature count drift")
if evidence.get("directabi_route_publication_claim") != 0:
    raise SystemExit("DirectAbi route publication must remain unclaimed")

claims = fixture.get("claims") or {}
if claims.get("retire_candidate_recorded") != 1:
    raise SystemExit("retire candidate claim missing")
for key, value in claims.items():
    if key == "retire_candidate_recorded":
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

if fixture.get("next_card", {}).get("selected_next_card") != next_card:
    raise SystemExit("next card drift")
for needle in [token, next_card, "full_astnode_projector_retired=0"]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
if "3221 marks the expanded RecipeBodies verifier-boundary DTO coverage rows" not in task_order:
    raise SystemExit("task-order 3221 marker missing")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipebodies-verifier-boundary-expanded-dto-coverage-retire-rust-astnode-projector-candidate-guard-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-VERIFIER-BOUNDARY-EXPANDED-DTO-COVERAGE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
retire_candidate_recorded=1
covered_row_count=2
covered_rows=local_loop_body_if_branch_return,local_loop_body_if_branch_return_alt_names
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only_candidate=1
full_astnode_projector_retired=0
existing_snapshot_owner_reused=1
recipe_verifier_boundary_used=1
verified_recipe_present=1
body_count=4
def_count=1
update_count=2
directabi_route_publication_claim=0
recipe_bodies_materialization=0
runtime_recipe_bodies_arena=0
full_recipe_matcher_execution=0
verifier_policy_reimplementation=0
route_selection=0
mir_mutation=0
id_allocation=0
backend_lowering_claim=0
runtime_route_switch=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-EXPANDED-DTO-COVERAGE-NEXT-CONTRACT-SELECTION-001
summary=ok
REPORT
