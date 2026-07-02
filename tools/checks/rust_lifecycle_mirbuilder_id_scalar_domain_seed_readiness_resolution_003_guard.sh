#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-id-scalar-domain-seed-readiness-resolution-003-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_id_scalar_domain_seed_readiness_resolution_003.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2051-MIRBUILDER-ID-SCALAR-DOMAIN-SEED-READINESS-RESOLUTION-003.md"
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

token = "MIRBUILDER-ID-SCALAR-DOMAIN-SEED-READINESS-RESOLUTION-003"
next_card = "MIRBUILDER-EMISSION_SSA_PHI-HAKO-NATIVE-SOURCE-SEED-001"

need(fixture.get("kind") == "MirBuilderIdScalarDomainSeedReadinessResolutionV3", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
pool = fixture.get("candidate_pool") or {}
need(pool.get("seed_materialization_ready_count") == 1, "ready count drift")
need(pool.get("selected_owner_count") == 1, "selected owner count drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "SelectNativeSeedMaterialization", "bad decision kind")
need(decision.get("reason_token") == "ExactlyOneIdScalarSeedMaterializationReadyOwnerEdgeAfterSeedPacket", "bad reason")
need(decision.get("selected_owner_edge_id") == "mirbuilder::emission_ssa_phi", "bad selected owner")
need(decision.get("selected_next_card") == next_card, "bad next card")

claims = fixture.get("claims") or {}
for key in [
    "manual_owner_selection",
    "cluster_size_as_proof",
    "directable_row_count_as_proof",
    "coverage_percentage_as_proof",
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
    "reason_token = ExactlyOneIdScalarSeedMaterializationReadyOwnerEdgeAfterSeedPacket",
    "selected_owner_edge_id = mirbuilder::emission_ssa_phi",
    "selected_next_card = MIRBUILDER-EMISSION_SSA_PHI-HAKO-NATIVE-SOURCE-SEED-001",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-id-scalar-domain-seed-readiness-resolution-003")
print("selected_owner_edge_id=mirbuilder::emission_ssa_phi")
print("reason_token=ExactlyOneIdScalarSeedMaterializationReadyOwnerEdgeAfterSeedPacket")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
