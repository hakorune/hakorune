#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-direct-state-plan-refresh-hako-adoption-decision-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-direct-state-plan-refresh-hako-adoption-decision-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1993-MIRBUILDER-DIRECT-STATE-PLAN-REFRESH-HAKO-ADOPTION-DECISION-001.md"
SEED_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-direct-state-plan-refresh-hako-native-source-seed-v0.json"
SEED_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_direct_state_plan_refresh_hako_native_source_seed_guard.sh"
SELECTION="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-converter-emission-native-seed-candidate-selection-rerun-002-v0.json"
BRIDGE_V2="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-emission-to-native-seed-bridge-policy-v2-v0.json"
VERIFIER="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-direct-state-plan-refresh-derived-hako-verifier-result-v0.json"
NATIVE_SOURCE="$ROOT_DIR/lang/src/compiler/lib/direct_state_plan_refresh_native_seed.hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$SEED_FIXTURE" "$SEED_GUARD" "$SELECTION" "$BRIDGE_V2" "$VERIFIER" "$NATIVE_SOURCE"

bash "$SEED_GUARD" >/tmp/direct_state_plan_refresh_native_seed_guard.out

python3 - <<'PY'
import json
from pathlib import Path

token = "MIRBUILDER-DIRECT-STATE-PLAN-REFRESH-HAKO-ADOPTION-DECISION-001"
owner = "hakorune_mir_builder::direct_state_plan_refresh"

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-direct-state-plan-refresh-hako-adoption-decision-v0.json").read_text())
seed = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-direct-state-plan-refresh-hako-native-source-seed-v0.json").read_text())
selection = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-converter-emission-native-seed-candidate-selection-rerun-002-v0.json").read_text())
bridge = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-emission-to-native-seed-bridge-policy-v2-v0.json").read_text())
verifier = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-direct-state-plan-refresh-derived-hako-verifier-result-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1993-MIRBUILDER-DIRECT-STATE-PLAN-REFRESH-HAKO-ADOPTION-DECISION-001.md").read_text()
native = Path("lang/src/compiler/lib/direct_state_plan_refresh_native_seed.hako").read_text()

if fixture.get("kind") != "MirBuilderDirectStatePlanRefreshHakoAdoptionDecisionV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card token mismatch")
if fixture.get("family_id") != owner:
    raise SystemExit("family id mismatch")

if seed["seed_status"]["native_source_owner_seed_present"] != 1:
    raise SystemExit("native source seed must be present")
if seed["native_source_seed"]["generator_overwrite_guard"] is not True:
    raise SystemExit("seed generator overwrite guard missing")
if selection["decision"]["selected_owner_edge_id"] != owner:
    raise SystemExit("candidate selection must select direct_state_plan_refresh")
if selection["decision"]["selected_next_card"] != "MIRBUILDER-DIRECT-STATE-PLAN-REFRESH-HAKO-NATIVE-SOURCE-SEED-001":
    raise SystemExit("candidate selection next card drift")

policy = bridge["v2_policy"]
if policy["mention_only_forbidden_nonclaim_is_seed_evidence"] is not False:
    raise SystemExit("mention-only forbidden nonclaim must not be seed evidence")
if policy["mention_only_forbidden_nonclaim_blocks_clean_narrow_seed_surface"] is not False:
    raise SystemExit("mention-only forbidden nonclaim must not block clean seed")
if policy["required_forbidden_nonclaim_blocks_seed"] is not True:
    raise SystemExit("required forbidden nonclaim must still block seed")

if verifier["result"] != "VerifiedHakoFamilyIR":
    raise SystemExit("direct_state verifier must be VerifiedHakoFamilyIR")
checks = verifier["checks"]
for key in ["direct_state_plan_refresh", "direct_state_plan_refresh_only", "canonical_json_parity"]:
    if checks.get(key) != 1:
        raise SystemExit(f"verifier check must be 1: {key}")
for key in ["runtime_fallback", "full_finalize_module"]:
    if checks.get(key) != 0:
        raise SystemExit(f"verifier check must be 0: {key}")

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
    "box DirectStatePlanRefreshPayloadBox",
    "box DirectStatePlanRefreshResultBox",
    "static box DirectStatePlanRefreshApi",
]:
    if needle not in native:
        raise SystemExit(f"native source missing adoption marker: {needle}")
for forbidden in ["@generated", "manual-edit: forbidden", "static box Main"]:
    if forbidden in native:
        raise SystemExit(f"adopted native source contains forbidden text: {forbidden}")

claims = fixture["claims"]
for key in [
    "hako_adopted",
    "native_hako_source_owner_present",
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
    "new_canonical_mir_instruction",
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
output_contract=rust-lifecycle-mirbuilder-direct-state-plan-refresh-hako-adoption-decision-v0
family_id=hakorune_mir_builder::direct_state_plan_refresh
decision=Adopt
selected_next_route=native_hako_source_owner
native_hako_source_owner_present=1
rust_bootstrap_retained=1
rust_oracle_retained=1
manual_family_selection=0
source_selfhost_claim=0
rust_deletion=0
runtime_fallback=0
new_backend_route=0
new_abi=0
new_canonical_mir_instruction=0
summary=ok
REPORT
