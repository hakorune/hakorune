#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-return-emission-hako-adoption-decision-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-return-emission-hako-adoption-decision-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1816-MIRBUILDER-RETURN-EMISSION-HAKO-ADOPTION-DECISION-001.md"
SEED_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-return-emission-hako-native-source-seed-v0.json"
SEED_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_return_emission_hako_native_source_seed_guard.sh"
PROMOTION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/return-emission-hako-shadow-promotion-decision-v0.json"
VERIFIER="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-return-emission-derived-hako-verifier-result-v0.json"
NATIVE_SOURCE="$ROOT_DIR/lang/src/compiler/lib/return_emission_native_seed.hako"

guard_require_command "$TAG" python3
guard_require_command "$TAG" rg
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$SEED_FIXTURE" "$SEED_GUARD" "$PROMOTION" "$VERIFIER" "$NATIVE_SOURCE"

bash "$SEED_GUARD" >/tmp/return_emission_native_seed_guard.out

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-return-emission-hako-adoption-decision-v0.json").read_text())
seed = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-return-emission-hako-native-source-seed-v0.json").read_text())
promotion = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/return-emission-hako-shadow-promotion-decision-v0.json").read_text())
verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-return-emission-derived-hako-verifier-result-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1816-MIRBUILDER-RETURN-EMISSION-HAKO-ADOPTION-DECISION-001.md").read_text()
native = Path("lang/src/compiler/lib/return_emission_native_seed.hako").read_text()

token = "MIRBUILDER-RETURN-EMISSION-HAKO-ADOPTION-DECISION-001"
if fixture.get("kind") != "MirBuilderReturnEmissionHakoAdoptionDecisionV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if f"# 1816 - {token}" not in card:
    raise SystemExit("card token mismatch")
if fixture.get("family_id") != "hakorune_mir_builder::return_emission":
    raise SystemExit("family id mismatch")

if seed["seed_status"]["native_source_owner_seed_present"] != 1:
    raise SystemExit("native source seed must be present")
if seed["native_source_seed"]["generator_overwrite_guard"] is not True:
    raise SystemExit("seed generator overwrite guard missing")
if promotion["decision"]["kind"] != "Promote" or promotion["selected_stage"] != "HakoMainline":
    raise SystemExit("ReturnEmission must be HakoMainline before adoption")
if verifier["checks"]["return_emission_only"] != 1:
    raise SystemExit("verifier must be limited to ReturnEmission")
if verifier["checks"]["runtime_fallback"] != 0:
    raise SystemExit("verifier must forbid runtime fallback")

target = fixture["target"]
if target["family_scope"] != "LeafSemanticOwner":
    raise SystemExit("adoption target must be leaf semantic owner")
if target["native_source_owner_present"] != 1:
    raise SystemExit("native source owner must be present")
if target["support_lane_projector_as_hako_adoption_candidate"] != 0:
    raise SystemExit("support lane projector must not be adoption candidate")

decision = fixture["decision"]
if decision["value"] != "Adopt":
    raise SystemExit("decision must be Adopt")
if decision["selected_next_route"] != "native_hako_source_owner":
    raise SystemExit("selected next route drift")

for needle in [
    "hako-adopted: 1",
    "source-selfhost-claim: 0",
    "static box ReturnEmissionNativeSeedApi",
    "static box ReturnEmissionApi",
]:
    if needle not in native:
        raise SystemExit(f"native source missing adoption marker: {needle}")
if "@generated" in native or "manual-edit: forbidden" in native:
    raise SystemExit("adopted native source must not be generated/manual-edit forbidden")

claims = fixture["claims"]
for key in [
    "hako_adopted",
    "native_hako_source_owner_present",
    "generator_overwrite_guard",
    "rust_bootstrap_retained",
    "rust_oracle_retained",
]:
    if claims.get(key) != 1:
        raise SystemExit(f"positive claim must be 1: {key}")
for key in [
    "manual_family_selection",
    "source_selfhost_claim",
    "rust_deletion",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
    "generated_artifact_as_edit_authority",
    "support_lane_projector_as_hako_adoption_candidate",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"non-claim must be 0: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-return-emission-hako-adoption-decision-v0
family_id=hakorune_mir_builder::return_emission
decision=Adopt
selected_next_route=native_hako_source_owner
native_hako_source_owner_present=1
generator_overwrite_guard=1
rust_bootstrap_retained=1
rust_oracle_retained=1
manual_family_selection=0
support_lane_projector_as_hako_adoption_candidate=0
source_selfhost_claim=0
rust_deletion=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
