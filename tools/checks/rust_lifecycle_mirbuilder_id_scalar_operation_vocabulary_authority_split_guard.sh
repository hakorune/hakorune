#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-operation-vocabulary-authority-split-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_operation_vocabulary_authority_split.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2042-MIRBUILDER-ID-SCALAR-OPERATION-VOCABULARY-AUTHORITY-SPLIT-001.md"
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

token = "MIRBUILDER-ID-SCALAR-OPERATION-VOCABULARY-AUTHORITY-SPLIT-001"
next_card = "MIRBUILDER-SEMANTIC-SELECTOR-NO-LEXICAL-TIEBREAK-GUARD-001"

need(fixture.get("kind") == "MirBuilderIdScalarOperationVocabularyAuthoritySplitV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

policy = fixture.get("authority_policy") or {}
need(policy.get("semantic_operation_authority") == "FixtureDeclaredRoleMapped", "semantic authority drift")
need(policy.get("diagnostic_operation_suggestion") == "SymbolReturnTypeMapped", "diagnostic authority drift")
need(policy.get("symbol_return_type_fallback_is_semantic_authority") is False, "fallback authority drift")
need(policy.get("diagnostic_suggestion_may_select_source_plan_owner") is False, "diagnostic selection drift")
need(policy.get("fixture_declared_role_required_for_semantic_operation_mapping") is True, "role required drift")
need(policy.get("operation_name_fallback_is_diagnostic_only") is True, "fallback diagnostic drift")
need(policy.get("source_plan_materialization") is False, "source plan materialization drift")

pool = fixture.get("candidate_pool") or {}
need(pool.get("input_owner_count") == 4, "owner count drift")
need(pool.get("tied_derivable_owner_count") == 2, "tied count drift")
need(pool.get("semantic_role_mapped_operation_count") == 94, "semantic operation count drift")
need(pool.get("diagnostic_suggestion_operation_count") == 8, "diagnostic count drift")
need(pool.get("unknown_operation_count") == 0, "unknown operation drift")
need(pool.get("tied_semantic_authority_complete_owner_count") == 2, "tied semantic complete drift")
need(pool.get("selection_eligible_count") == 0, "selection eligible drift")

for row in fixture.get("owner_rows") or []:
    need(row.get("selection_eligible") is False, "authority split selected an owner")
    if row.get("is_tied_derivable_owner"):
        need(row.get("semantic_operation_authority_complete") is True, "tied owner semantic authority drift")
        need(row.get("diagnostic_suggestion_count") == 0, "tied owner diagnostic fallback drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "PolicyDefined", "decision kind drift")
need(decision.get("selected_next_card") == next_card, "next card drift")

claims = fixture.get("claims") or {}
for key in [
    "symbol_return_type_fallback_as_semantic_authority",
    "diagnostic_suggestion_as_source_plan_selection_proof",
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
    "selected_next_card = MIRBUILDER-SEMANTIC-SELECTOR-NO-LEXICAL-TIEBREAK-GUARD-001",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-operation-vocabulary-authority-split")
print("semantic_role_mapped_operation_count=94")
print("diagnostic_suggestion_operation_count=8")
print("tied_semantic_authority_complete_owner_count=2")
print("selection_eligible_count=0")
print("selected_next_card=MIRBUILDER-SEMANTIC-SELECTOR-NO-LEXICAL-TIEBREAK-GUARD-001")
print("source_selfhost_claim=0")
print("summary=ok")
PY
