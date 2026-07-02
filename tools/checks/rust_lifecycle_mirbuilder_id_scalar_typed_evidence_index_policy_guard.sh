#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-typed-evidence-index-policy-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_typed_evidence_index_policy.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2041-MIRBUILDER-ID-SCALAR-TYPED-EVIDENCE-INDEX-POLICY-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" "$TOOL" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
state = tomllib.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")
tool = Path(sys.argv[5]).read_text(encoding="utf-8")

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

token = "MIRBUILDER-ID-SCALAR-TYPED-EVIDENCE-INDEX-POLICY-001"
next_card = "MIRBUILDER-ID-SCALAR-OPERATION-VOCABULARY-AUTHORITY-SPLIT-001"

need(fixture.get("kind") == "MirBuilderIdScalarTypedEvidenceIndexPolicyV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

policy = fixture.get("policy") or {}
need(policy.get("typed_evidence_index_required") is True, "typed index required drift")
need(policy.get("mention_only_owner_edge_text_is_not_evidence") is True, "mention-only policy drift")
need(policy.get("owner_edge_substring_search_allowed") is False, "owner substring search drift")
need(policy.get("fixture_path_substring_search_allowed") is False, "fixture substring search drift")
need(policy.get("typed_fixture_refs_only") is True, "typed fixture refs drift")
need(policy.get("source_plan_materialization") is False, "source plan materialization drift")

pool = fixture.get("candidate_pool") or {}
need(pool.get("input_tied_owner_count") == 2, "tied owner count drift")
need(pool.get("typed_evidence_complete_owner_count") == 2, "typed complete count drift")
need(pool.get("selection_eligible_count") == 0, "selection eligible drift")

required_kinds = {
    "SourceSurfaceInventory",
    "OperationVocabularyInventory",
    "OwnerScopeBoundedness",
    "NativeSeedFileBoundary",
    "IdDomainBoundary",
    "StateMutationFrame",
    "ErrorSemantics",
    "DeterministicOrder",
    "BehaviorRecipeEffectCoverage",
    "VerifierInputContract",
}
rows = fixture.get("typed_evidence_rows") or []
need(len(rows) == 2, "typed row count drift")
for row in rows:
    need(row.get("typed_evidence_complete") is True, "typed row not complete")
    need(row.get("selection_eligible") is False, "typed index selected an owner")
    kinds = {entry.get("artifact_kind") for entry in row.get("evidence_entries") or []}
    need(kinds == required_kinds, f"evidence kind drift for {row.get('owner_edge_id')}")
    for entry in row.get("evidence_entries") or []:
        need(entry.get("typed_ref_count", 0) > 0, "missing typed ref")
        need(entry.get("typed_refs_complete") is True, "typed ref incomplete")
        need(entry.get("mention_only_owner_edge_text") is False, "mention-only evidence drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "PolicyDefined", "decision kind drift")
need(decision.get("selected_next_card") == next_card, "next card drift")

claims = fixture.get("claims") or {}
for key in [
    "owner_edge_text_mention_as_evidence",
    "fixture_path_substring_as_evidence",
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

for forbidden in [
    "owner_edge_id in text",
    " in text",
    "read_text(encoding=\"utf-8\") if",
]:
    need(forbidden not in tool, f"substring evidence pattern present: {forbidden}")

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
for needle in [
    token,
    "selected_next_card = MIRBUILDER-ID-SCALAR-OPERATION-VOCABULARY-AUTHORITY-SPLIT-001",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-typed-evidence-index-policy")
print("input_tied_owner_count=2")
print("typed_evidence_complete_owner_count=2")
print("selection_eligible_count=0")
print("selected_next_card=MIRBUILDER-ID-SCALAR-OPERATION-VOCABULARY-AUTHORITY-SPLIT-001")
print("source_selfhost_claim=0")
print("summary=ok")
PY
