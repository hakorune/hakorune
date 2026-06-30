#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-core-context-hako-adoption-decision-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-core-context-hako-adoption-decision-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1966-MIRBUILDER-CORE-CONTEXT-HAKO-ADOPTION-DECISION-001.md"
SEED_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-core-context-hako-native-source-seed-v0.json"
SEED_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_core_context_hako_native_source_seed_guard.sh"
SELECTION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-converter-emission-native-seed-candidate-selection-v0.json"
VERIFIER="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/core-context-derived-artifact-verifier-result-v0.json"
NATIVE_SOURCE="$ROOT_DIR/lang/src/compiler/lib/core_context_native_seed.hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$SEED_FIXTURE" "$SEED_GUARD" "$SELECTION" "$VERIFIER" "$NATIVE_SOURCE"

bash "$SEED_GUARD" >/tmp/core_context_native_seed_guard.out

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-core-context-hako-adoption-decision-v0.json").read_text())
seed = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-core-context-hako-native-source-seed-v0.json").read_text())
selection = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-converter-emission-native-seed-candidate-selection-v0.json").read_text())
verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/core-context-derived-artifact-verifier-result-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1966-MIRBUILDER-CORE-CONTEXT-HAKO-ADOPTION-DECISION-001.md").read_text()
native = Path("lang/src/compiler/lib/core_context_native_seed.hako").read_text()

token = "MIRBUILDER-CORE-CONTEXT-HAKO-ADOPTION-DECISION-001"
if fixture.get("kind") != "MirBuilderCoreContextHakoAdoptionDecisionV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card token mismatch")
if fixture.get("family_id") != "hakorune_mir_builder::core_context":
    raise SystemExit("family id mismatch")

if seed["seed_status"]["native_source_owner_seed_present"] != 1:
    raise SystemExit("native source seed must be present")
if seed["native_source_seed"]["generator_overwrite_guard"] is not True:
    raise SystemExit("seed generator overwrite guard missing")
if selection["decision"]["selected_owner_edge_id"] != "hakorune_mir_builder::core_context":
    raise SystemExit("candidate selection must select core_context")
if selection["decision"]["selected_next_card"] != "MIRBUILDER-CORE-CONTEXT-HAKO-NATIVE-SOURCE-SEED-001":
    raise SystemExit("candidate selection next card drift")
if verifier["result"] != "VerifiedHakoFamilyIR":
    raise SystemExit("core_context verifier must be VerifiedHakoFamilyIR")
checks = verifier["checks"]
if checks["unmapped_mir_side_effects"] != 0:
    raise SystemExit("unmapped MIR side effects must be zero")
if checks["unclassified_drop_obligations"] != 0:
    raise SystemExit("unclassified drop obligations must be zero")

target = fixture["target"]
if target["family_scope"] != "LeafSemanticOwner":
    raise SystemExit("adoption target must be leaf semantic owner")
if target["native_source_owner_present"] != 1:
    raise SystemExit("native source owner must be present")
if target["strict_emission_bridge_candidate"] != 1:
    raise SystemExit("strict bridge candidate must be present")
if target["generated_artifact_as_edit_authority"] != 0:
    raise SystemExit("generated artifact must not be edit authority")

decision = fixture["decision"]
if decision["value"] != "Adopt":
    raise SystemExit("decision must be Adopt")
if decision["selected_next_route"] != "native_hako_source_owner":
    raise SystemExit("selected next route drift")

for needle in [
    "hako-adopted: 1",
    "source-selfhost-claim: 0",
    "box CoreContext",
    "static box CoreContextApi",
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
    "cluster_size_as_proof",
    "coverage_percentage_as_proof",
    "route_membership_alone_as_proof",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"non-claim must be 0: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-core-context-hako-adoption-decision-v0
family_id=hakorune_mir_builder::core_context
decision=Adopt
selected_next_route=native_hako_source_owner
native_hako_source_owner_present=1
generator_overwrite_guard=1
rust_bootstrap_retained=1
rust_oracle_retained=1
manual_family_selection=0
source_selfhost_claim=0
rust_deletion=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
