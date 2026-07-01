#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-native-seed-file-boundary-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_native_seed_file_boundary_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2032-MIRBUILDER-ID-SCALAR-NATIVE-SEED-FILE-BOUNDARY-BASIS-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
state = tomllib.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-ID-SCALAR-NATIVE-SEED-FILE-BOUNDARY-BASIS-001"
next_card = "MIRBUILDER-ID-SCALAR-SOURCE-PLAN-BASIS-COMPONENT-PRIORITY-RESOLUTION-002"

need(fixture.get("kind") == "MirBuilderIdScalarNativeSeedFileBoundaryBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

policy = fixture.get("boundary_policy") or {}
need(policy.get("source_path_alone_as_authority") is False, "source path authority drift")
need(policy.get("surface_count_as_proof") is False, "surface count proof drift")
need(policy.get("owner_name_alone_as_authority") is False, "owner name authority drift")
need(policy.get("native_seed_materialization") is False, "native seed materialization drift")

pool = fixture.get("candidate_pool") or {}
need(pool.get("input_candidate_count") == 4, "candidate count drift")
need(pool.get("owner_scope_bounded_count") == 2, "bounded count drift")
need(pool.get("native_seed_file_boundary_derivable_count") == 2, "boundary count drift")
need(pool.get("cross_owner_boundary_blocked_count") == 2, "blocked count drift")

rows = {row["owner_edge_id"]: row for row in fixture.get("boundary_rows") or []}
need(rows["mirbuilder::context_registry"]["native_seed_file_boundary_derivable"] is True, "context boundary drift")
need(rows["mirbuilder::emission_ssa_phi"]["native_seed_file_boundary_derivable"] is True, "emission boundary drift")
need(rows["mirbuilder::join_i_r_plan"]["native_seed_file_boundary_derivable"] is False, "plan boundary drift")
need(rows["mirbuilder::join_i_r_route_verify"]["native_seed_file_boundary_derivable"] is False, "route verify boundary drift")
need(rows["mirbuilder::context_registry"]["module_export"] == "lib.mirbuilder.context_registry_native_seed", "context export drift")
need(rows["mirbuilder::emission_ssa_phi"]["module_export"] == "lib.mirbuilder.emission_ssa_phi_native_seed", "emission export drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "NativeSeedFileBoundaryBasisDefined", "decision kind drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "manual_owner_selection",
    "surface_count_as_proof",
    "source_path_alone_as_authority",
    "owner_name_alone_as_authority",
    "source_plan_materialization",
    "behavior_recipe_materialization",
    "verifier_result_materialization",
    "derived_artifact_seed_draft_materialization",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
for needle in [
    token,
    "native_seed_file_boundary_derivable_count = 2",
    "selected_next_card = MIRBUILDER-ID-SCALAR-SOURCE-PLAN-BASIS-COMPONENT-PRIORITY-RESOLUTION-002",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-native-seed-file-boundary-basis")
print("native_seed_file_boundary_derivable_count=2")
print("selected_next_card=MIRBUILDER-ID-SCALAR-SOURCE-PLAN-BASIS-COMPONENT-PRIORITY-RESOLUTION-002")
print("source_selfhost_claim=0")
print("summary=ok")
PY
