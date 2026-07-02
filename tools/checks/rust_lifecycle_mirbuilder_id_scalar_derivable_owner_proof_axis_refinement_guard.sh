#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-derivable-owner-proof-axis-refinement-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_derivable_owner_proof_axis_refinement.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2046-MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-PROOF-AXIS-REFINEMENT-001.md"
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

token = "MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-PROOF-AXIS-REFINEMENT-001"
next_card = "MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-RESOLUTION-002"

need(fixture.get("kind") == "MirBuilderIdScalarDerivableOwnerProofAxisRefinementV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

axes = fixture.get("candidate_axes") or []
need(len(axes) == 4, "axis count drift")
expected = {
    "PriorProjectionPolicyDisposition": "StandaloneProjectionSubjectEstablished",
    "ContractLifecycleDescriptorPresence": "LifecycleContractDescriptorCompleteness",
    "LifecycleMutationShape": "MutationFrameSemanticCompleteness",
    "VerifierEffectClassPresence": "VerifierEffectClassCoverageCompleteness",
}
for raw, refined in expected.items():
    row = next((axis for axis in axes if axis.get("axis_name") == raw), None)
    need(row is not None, f"missing raw axis {raw}")
    need(row.get("decision") == "ReplaceWithRefinedProofAxis", f"bad decision {raw}")
    need(row.get("refined_axis") == refined, f"bad refined axis {raw}")
    need(row.get("proof_allowed") is True, f"proof not allowed {raw}")
    need(row.get("conditions"), f"missing conditions {raw}")
    need(row.get("forbidden_interpretation"), f"missing forbidden interpretation {raw}")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "ProofAxesRefined", "bad decision kind")
need(decision.get("reason_token") == "IdScalarDerivableOwnerProofAxesRefinedWithoutCountOrNameProof", "bad reason")
need(decision.get("selected_next_card") == next_card, "bad next card")

claims = fixture.get("claims") or {}
for key in [
    "owner_name_as_proof",
    "historical_descriptor_presence_as_preference",
    "lifecycle_richness_as_proof",
    "mutation_complexity_as_proof",
    "effect_class_count_as_proof",
    "surface_count_as_proof",
    "row_count_as_proof",
    "coverage_percentage_as_proof",
    "source_plan_materialization",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
for needle in [
    token,
    "reason_token = IdScalarDerivableOwnerProofAxesRefinedWithoutCountOrNameProof",
    "selected_next_card = MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-RESOLUTION-002",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-derivable-owner-proof-axis-refinement")
print("refined_axis_count=4")
print("reason_token=IdScalarDerivableOwnerProofAxesRefinedWithoutCountOrNameProof")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
