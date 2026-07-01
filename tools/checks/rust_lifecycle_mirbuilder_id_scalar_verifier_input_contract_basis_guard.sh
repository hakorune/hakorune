#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-verifier-input-contract-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_verifier_input_contract_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2038-MIRBUILDER-ID-SCALAR-VERIFIER-INPUT-CONTRACT-BASIS-001.md"
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

token = "MIRBUILDER-ID-SCALAR-VERIFIER-INPUT-CONTRACT-BASIS-001"
next_card = "MIRBUILDER-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-DERIVABILITY-RESOLUTION-003"

need(fixture.get("kind") == "MirBuilderIdScalarVerifierInputContractBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

policy = fixture.get("contract_policy") or {}
need(policy.get("verifier_input_contract_declared") is True, "contract declared drift")
need(policy.get("verifier_result_materialization") is False, "verifier result materialization drift")
need(policy.get("source_plan_materialization") is False, "source plan materialization drift")

pool = fixture.get("candidate_pool") or {}
need(pool.get("input_fact_set_count") == 6, "fact set count drift")
need(pool.get("effect_row_count") == 14, "effect row count drift")
need(pool.get("mutation_frame_count") == 3, "mutation frame count drift")
need(pool.get("id_domain_boundary_count") == 3, "ID domain count drift")
need(pool.get("native_seed_file_boundary_count") == 2, "file boundary count drift")

fact_sets = {row["fact_set"] for row in fixture.get("input_fact_sets") or []}
for expected in [
    "EffectCoverageRows",
    "MutationFrameRows",
    "IdDomainBoundaryRows",
    "ErrorSemanticsRows",
    "DeterministicOrderRows",
    "NativeSeedFileBoundaryRows",
]:
    need(expected in fact_sets, f"missing fact set {expected}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "VerifierInputContractBasisDefined", "decision kind drift")
need(decision.get("selected_next_card") == next_card, "next drift")

claims = fixture.get("claims") or {}
for key in [
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
    "input_fact_set_count = 6",
    "selected_next_card = MIRBUILDER-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-DERIVABILITY-RESOLUTION-003",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-verifier-input-contract-basis")
print("input_fact_set_count=6")
print("selected_next_card=MIRBUILDER-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-DERIVABILITY-RESOLUTION-003")
print("source_selfhost_claim=0")
print("summary=ok")
PY
