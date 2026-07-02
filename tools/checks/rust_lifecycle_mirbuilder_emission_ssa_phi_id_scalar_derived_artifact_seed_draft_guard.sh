#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-emission-ssa-phi-id-scalar-derived-artifact-seed-draft-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_emission_ssa_phi_id_scalar_derived_artifact_seed_draft.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2050-MIRBUILDER-EMISSION_SSA_PHI-ID-SCALAR-DERIVED-ARTIFACT-SEED-DRAFT-001.md"
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

token = "MIRBUILDER-EMISSION_SSA_PHI-ID-SCALAR-DERIVED-ARTIFACT-SEED-DRAFT-001"
next_card = "MIRBUILDER-ID-SCALAR-DOMAIN-SEED-READINESS-RESOLUTION-003"

need(fixture.get("kind") == "MirBuilderEmissionSsaPhiIdScalarDerivedArtifactSeedDraftV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
draft = fixture.get("seed_draft_input") or {}
need(draft.get("state") == "DerivedArtifactSeedDraftInput", "bad draft state")
need(draft.get("generated_artifact_as_native_edit_authority") is False, "edit authority drift")
need(draft.get("native_source_seed") is False, "native seed drift")
need(draft.get("hako_adopted") is False, "adoption drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "DerivedArtifactSeedDraftInputMaterialized", "bad decision kind")
need(decision.get("reason_token") == "EmissionSsaPhiIdScalarDerivedArtifactSeedDraftInputMaterialized", "bad reason")
need(decision.get("selected_next_card") == next_card, "bad next card")

claims = fixture.get("claims") or {}
need(claims.get("derived_artifact_seed_draft_materialization") == 1, "draft not materialized")
for key in [
    "generated_artifact_as_native_edit_authority",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
for needle in [
    token,
    "reason_token = EmissionSsaPhiIdScalarDerivedArtifactSeedDraftInputMaterialized",
    "selected_next_card = MIRBUILDER-ID-SCALAR-DOMAIN-SEED-READINESS-RESOLUTION-003",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-emission-ssa-phi-id-scalar-derived-artifact-seed-draft")
print("seed_draft_input_state=DerivedArtifactSeedDraftInput")
print("reason_token=EmissionSsaPhiIdScalarDerivedArtifactSeedDraftInputMaterialized")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
