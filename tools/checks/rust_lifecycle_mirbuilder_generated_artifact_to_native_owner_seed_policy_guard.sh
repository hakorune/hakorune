#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-generated-artifact-to-native-owner-seed-policy-v0.json"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/1813-MIRBUILDER-GENERATED-ARTIFACT-TO-NATIVE-OWNER-SEED-POLICY-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

python3 - "$ROOT" "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" <<'PY'
import json
import sys
from pathlib import Path
import tomllib

root, fixture_path, card_path, state_path, task_order_path = map(Path, sys.argv[1:])

TOKEN = "MIRBUILDER-GENERATED-ARTIFACT-TO-NATIVE-OWNER-SEED-POLICY-001"
BLOCKER = "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001"
NEXT = "MIRBUILDER-GENERATED-ARTIFACT-NATIVE-OWNER-SEED-CANDIDATE-RESOLUTION-001"

def die(message: str) -> None:
    print(f"[generated-artifact-to-native-owner-seed-policy] {message}", file=sys.stderr)
    raise SystemExit(1)

def read(path: Path) -> str:
    if not path.exists():
        die(f"missing path: {path.relative_to(root)}")
    return path.read_text()

fixture = json.loads(read(fixture_path))
card = read(card_path)
state = tomllib.loads(read(state_path))
task_order = read(task_order_path)

if fixture.get("kind") != "MirBuilderGeneratedArtifactToNativeOwnerSeedPolicyV1":
    die("fixture kind mismatch")
if fixture.get("token") != TOKEN:
    die("fixture token mismatch")
if TOKEN not in card:
    die("card missing token")
if state.get("current_blocker_token") != BLOCKER:
    die("CURRENT_STATE blocker drift")
latest_card = state.get("latest_card") or ""
if latest_card not in state.get("latest_card_path", ""):
    die("CURRENT_STATE latest path drift")

for rel in (fixture.get("input_authority") or {}).values():
    if not (root / rel).exists():
        die(f"input authority missing: {rel}")

inventory = json.loads(read(root / fixture["input_authority"]["native_owner_seed_inventory"]))
if inventory.get("decision", {}).get("reason_token") != "NoNativeOwnerSeedCandidate":
    die("native owner seed inventory input drift")
if inventory.get("candidate_pool", {}).get("native_owner_seed_candidate_count") != 0:
    die("input inventory should not already expose a seed candidate")

model_text = read(root / fixture["input_authority"]["derived_to_native_model"])
for needle in [
    "generated Hako artifact",
    "final semantic/edit authority",
    "requires native Hako source",
]:
    if needle not in model_text:
        die(f"derived-to-native model missing policy phrase: {needle}")

policy = fixture.get("policy") or {}
if policy.get("source_classification") != "GeneratedArtifactOnly":
    die("source classification drift")
if policy.get("target_classification") != "NativeOwnerSeedCandidate":
    die("target classification drift")
if policy.get("generated_artifact_is_edit_authority") is not False:
    die("generated artifact must not be edit authority")

conditions = set(policy.get("seed_candidate_required_conditions") or [])
for required in [
    "LeafSemanticOwner",
    "NotCompositionOwner",
    "NotSupportLaneOnly",
    "VerifiedArtifactManifestPresent",
    "DeterministicRegenerationGreen",
    "OracleOrContractGreen",
    "ExecutableGateGreen",
    "SourcePlanOrEquivalentAuthorityPresent",
    "NoRuntimeFallback",
    "NoNewBackendRoute",
    "NoNewAbi",
    "GeneratorOverwriteGuardPlanned",
]:
    if required not in conditions:
        die(f"missing required seed condition: {required}")

denials = set(policy.get("explicit_denials") or [])
for required in [
    "CompositionOwnerAsSemanticOwner",
    "GeneratedArtifactAsEditAuthority",
    "SupportLaneProjectorAsFamilyAdoptionCandidate",
    "ManualFamilySelection",
    "RuntimeTryHakoThenRustFallback",
    "SourceSelfhostClaim",
]:
    if required not in denials:
        die(f"missing explicit denial: {required}")

resolution = fixture.get("next_resolution") or {}
if resolution.get("card") != NEXT:
    die("next resolution card drift")
if resolution.get("selection_rule") != "exactly_one_machine_derived_leaf_semantic_owner":
    die("selection rule must forbid manual family selection")
outcomes = set(resolution.get("allowed_outcomes") or [])
for required in ["SelectSeedMaterialization", "KeepStopped", "SelectFurtherLeafDecomposition"]:
    if required not in outcomes:
        die(f"missing allowed outcome: {required}")

decision = fixture.get("decision") or {}
if decision.get("kind") != "PolicyDefined":
    die("decision kind drift")
if decision.get("selected_family") is not None:
    die("policy must not select a family")
if decision.get("selected_next_card") != NEXT:
    die("policy must select the candidate resolution card")

claims = fixture.get("claims") or {}
for key in [
    "manual_family_selection",
    "native_source_owner_materialized",
    "family_adoption_decision",
    "generated_artifact_as_edit_authority",
    "composition_owner_as_semantic_owner",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    if claims.get(key) != 0:
        die(f"claim must be zero: {key}")

for needle in [
    TOKEN,
    NEXT,
    "generated_artifact_to_native_owner_seed_policy",
    "generated_artifact_as_edit_authority = 0",
    "composition_owner_as_semantic_owner = 0",
    "source_selfhost_claim",
]:
    if needle not in task_order:
        die(f"task-order missing: {needle}")

print("[generated-artifact-to-native-owner-seed-policy] OK")
PY
