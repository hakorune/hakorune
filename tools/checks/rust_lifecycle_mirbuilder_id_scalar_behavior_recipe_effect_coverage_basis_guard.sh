#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-behavior-recipe-effect-coverage-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_behavior_recipe_effect_coverage_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2037-MIRBUILDER-ID-SCALAR-BEHAVIOR-RECIPE-EFFECT-COVERAGE-BASIS-001.md"
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

token = "MIRBUILDER-ID-SCALAR-BEHAVIOR-RECIPE-EFFECT-COVERAGE-BASIS-001"
next_card = "MIRBUILDER-ID-SCALAR-VERIFIER-INPUT-CONTRACT-BASIS-001"

need(fixture.get("kind") == "MirBuilderIdScalarBehaviorRecipeEffectCoverageBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

policy = fixture.get("coverage_policy") or {}
need(policy.get("operation_tokens_normalized_to_effect_classes") is True, "normalization drift")
need(policy.get("all_bounded_owner_operation_tokens_covered") is True, "coverage drift")
need(policy.get("behavior_recipe_materialization") is False, "behavior recipe materialization drift")

pool = fixture.get("candidate_pool") or {}
need(pool.get("bounded_owner_count") == 2, "bounded owner drift")
need(pool.get("operation_token_count") == 9, "operation token count drift")
need(pool.get("effect_class_count") == 6, "effect class count drift")
need(pool.get("effect_row_count") == 14, "effect row count drift")
need(pool.get("mutation_frame_count") == 3, "mutation frame count drift")
need(pool.get("error_semantics_count") == 6, "error semantics count drift")

effect_classes = {row["effect_class"] for row in fixture.get("effect_class_summary") or []}
for expected in [
    "OwnerStateWrite",
    "PhiInstructionAppend",
    "PhiInstructionPatch",
    "PredicateRead",
    "DiagnosticBuild",
    "VerifierContractCheck",
]:
    need(expected in effect_classes, f"missing effect class {expected}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "BehaviorRecipeEffectCoverageBasisDefined", "decision kind drift")
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
    "effect_class_count = 6",
    "selected_next_card = MIRBUILDER-ID-SCALAR-VERIFIER-INPUT-CONTRACT-BASIS-001",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-behavior-recipe-effect-coverage-basis")
print("effect_class_count=6")
print("selected_next_card=MIRBUILDER-ID-SCALAR-VERIFIER-INPUT-CONTRACT-BASIS-001")
print("source_selfhost_claim=0")
print("summary=ok")
PY
