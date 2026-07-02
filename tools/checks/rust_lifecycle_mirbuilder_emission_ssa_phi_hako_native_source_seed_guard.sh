#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-emission-ssa-phi-hako-native-source-seed-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_emission_ssa_phi_hako_native_source_seed.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2052-MIRBUILDER-EMISSION_SSA_PHI-HAKO-NATIVE-SOURCE-SEED-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
NATIVE="$ROOT/lang/src/compiler/lib/mirbuilder/emission_ssa_phi_native_seed.hako"
MODULE="$ROOT/lang/src/compiler/hako_module.toml"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" "$NATIVE" "$MODULE" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
state = tomllib.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")
native = Path(sys.argv[5]).read_text(encoding="utf-8")
module = Path(sys.argv[6]).read_text(encoding="utf-8")

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

token = "MIRBUILDER-EMISSION_SSA_PHI-HAKO-NATIVE-SOURCE-SEED-001"
next_card = "MIRBUILDER-EMISSION_SSA_PHI-HAKO-ADOPTION-DECISION-001"

need(fixture.get("kind") == "MirBuilderEmissionSsaPhiHakoNativeSourceSeedV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
seed = fixture.get("native_source_seed") or {}
need(seed.get("native_source_seed_path") == "lang/src/compiler/lib/mirbuilder/emission_ssa_phi_native_seed.hako", "bad seed path")
need(seed.get("module_export") == "lib.mirbuilder.emission_ssa_phi_native_seed", "bad module export")
need(seed.get("native_source_seed_outside_generated_tree") is True, "seed under generated tree")
need(seed.get("generator_overwrite_guard") is True, "missing overwrite guard")

for needle in [
    "native-source-seed: MIRBUILDER-EMISSION_SSA_PHI-HAKO-NATIVE-SOURCE-SEED-001",
    "source-family: mirbuilder::emission_ssa_phi",
    "source-input-state: DerivedArtifactSeedDraftInput",
    "hako-adopted: 0",
    "source-selfhost-claim: 0",
    "box EmissionSsaPhiState",
    "static box EmissionSsaPhiApi",
    "define_phi(state, block_id, dst_value_id, inputs): i64",
    "patch_phi(state, dst_value_id, inputs): i64",
    "lifecycle_patch(state, token_id, inputs): i64",
]:
    need(needle in native, f"native seed missing {needle}")
need("@generated" not in native, "native seed must not be generated")
need('lib.mirbuilder.emission_ssa_phi_native_seed = "lib/mirbuilder/emission_ssa_phi_native_seed.hako"' in module, "module export missing")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "HakoAdoptionDecisionDeferred", "bad decision kind")
need(decision.get("reason_token") == "EmissionSsaPhiNativeSourceSeedMaterialized", "bad reason")
need(decision.get("selected_next_card") == next_card, "bad next card")

claims = fixture.get("claims") or {}
need(claims.get("native_seed_materialization") == 1, "native seed not materialized")
for key in [
    "generated_artifact_as_native_edit_authority",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "rust_deletion",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
for needle in [
    token,
    "reason_token = EmissionSsaPhiNativeSourceSeedMaterialized",
    "selected_next_card = MIRBUILDER-EMISSION_SSA_PHI-HAKO-ADOPTION-DECISION-001",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-mirbuilder-emission-ssa-phi-hako-native-source-seed")
print("native_source_seed_path=lang/src/compiler/lib/mirbuilder/emission_ssa_phi_native_seed.hako")
print("reason_token=EmissionSsaPhiNativeSourceSeedMaterialized")
print("selected_next_card=" + next_card)
print("source_selfhost_claim=0")
print("summary=ok")
PY
