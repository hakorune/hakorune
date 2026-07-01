#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-error-and-deterministic-order-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_error_and_deterministic_order_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2036-MIRBUILDER-ID-SCALAR-ERROR-AND-DETERMINISTIC-ORDER-BASIS-001.md"
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

token = "MIRBUILDER-ID-SCALAR-ERROR-AND-DETERMINISTIC-ORDER-BASIS-001"
next_card = "MIRBUILDER-ID-SCALAR-BEHAVIOR-RECIPE-EFFECT-COVERAGE-BASIS-001"

need(fixture.get("kind") == "MirBuilderIdScalarErrorAndDeterministicOrderBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

policy = fixture.get("basis_policy") or {}
need(policy.get("error_semantics_declared") is True, "error basis drift")
need(policy.get("deterministic_order_declared") is True, "order basis drift")
need(policy.get("runtime_fallback") is False, "runtime fallback drift")

pool = fixture.get("candidate_pool") or {}
need(pool.get("error_semantics_count") == 6, "error count drift")
need(pool.get("deterministic_order_count") == 3, "order count drift")
need(pool.get("runtime_fallback_count") == 0, "runtime fallback count drift")
need(pool.get("diagnostic_prefix_required_count") == 3, "diagnostic prefix count drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "ErrorAndDeterministicOrderBasisDefined", "decision kind drift")
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
    "error_semantics_count = 6",
    "deterministic_order_count = 3",
    "selected_next_card = MIRBUILDER-ID-SCALAR-BEHAVIOR-RECIPE-EFFECT-COVERAGE-BASIS-001",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-error-and-deterministic-order-basis")
print("error_semantics_count=6")
print("deterministic_order_count=3")
print("selected_next_card=MIRBUILDER-ID-SCALAR-BEHAVIOR-RECIPE-EFFECT-COVERAGE-BASIS-001")
print("source_selfhost_claim=0")
print("summary=ok")
PY
