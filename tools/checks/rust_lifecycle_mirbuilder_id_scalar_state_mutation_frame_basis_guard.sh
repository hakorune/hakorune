#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-state-mutation-frame-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_state_mutation_frame_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2035-MIRBUILDER-ID-SCALAR-STATE-MUTATION-FRAME-BASIS-001.md"
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

token = "MIRBUILDER-ID-SCALAR-STATE-MUTATION-FRAME-BASIS-001"
next_card = "MIRBUILDER-ID-SCALAR-ERROR-AND-DETERMINISTIC-ORDER-BASIS-001"

need(fixture.get("kind") == "MirBuilderIdScalarStateMutationFrameBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

policy = fixture.get("mutation_frame_policy") or {}
need(policy.get("requires_bounded_owner") is True, "bounded owner policy drift")
need(policy.get("requires_native_seed_file_boundary") is True, "file boundary policy drift")
need(policy.get("cross_owner_targets_excluded_until_recipe_authority_split") is True, "cross-owner policy drift")

pool = fixture.get("candidate_pool") or {}
need(pool.get("bounded_owner_count") == 2, "bounded owner count drift")
need(pool.get("mutation_frame_count") == 3, "frame count drift")
need(pool.get("rollback_declared_count") == 3, "rollback count drift")
need(pool.get("cleanup_declared_count") == 3, "cleanup count drift")
need(pool.get("owner_return_state_declared_count") == 3, "owner return count drift")

frames = fixture.get("mutation_frames") or []
need(len(frames) == 3, "frame row drift")
for frame in frames:
    need(frame["owner_edge_id"] in ["mirbuilder::context_registry", "mirbuilder::emission_ssa_phi"], "unexpected owner")
    need(frame["rollback_requirement"] == "NoRollbackDeclared", "rollback drift")
    need(frame["cleanup_requirement"] == "NoCleanupDeclared", "cleanup drift")
    need(frame["owner_return_state"] == "OwnerRetainedAfterFrame", "owner return drift")
    need(frame["read_set_declared"] or frame["write_set_declared"], "empty frame sets")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "StateMutationFrameBasisDefined", "decision kind drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
    "manual_owner_selection",
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
    "mutation_frame_count = 3",
    "selected_next_card = MIRBUILDER-ID-SCALAR-ERROR-AND-DETERMINISTIC-ORDER-BASIS-001",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-state-mutation-frame-basis")
print("mutation_frame_count=3")
print("selected_next_card=MIRBUILDER-ID-SCALAR-ERROR-AND-DETERMINISTIC-ORDER-BASIS-001")
print("source_selfhost_claim=0")
print("summary=ok")
PY
