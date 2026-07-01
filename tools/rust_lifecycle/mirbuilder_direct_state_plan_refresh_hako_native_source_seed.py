#!/usr/bin/env python3
"""Materialize the DirectStatePlanRefresh native Hako source seed.

This is a seed-materialization helper, not a SemanticProjector. It consumes
already-verified strict emission evidence and writes a native seed outside the
generated tree.
"""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]

TOKEN = "MIRBUILDER-DIRECT-STATE-PLAN-REFRESH-HAKO-NATIVE-SOURCE-SEED-001"
FAMILY_ID = "hakorune_mir_builder::direct_state_plan_refresh"
NEXT_CARD = "MIRBUILDER-DIRECT-STATE-PLAN-REFRESH-HAKO-ADOPTION-DECISION-001"

SELECTION = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-converter-emission-native-seed-candidate-selection-rerun-002-v0.json"
BRIDGE_V2 = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-emission-to-native-seed-bridge-policy-v2-v0.json"
VERIFIER = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-direct-state-plan-refresh-derived-hako-verifier-result-v0.json"
GENERATED = ROOT / "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_direct_state_plan_refresh.hako"
GENERATED_MANIFEST = ROOT / "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_direct_state_plan_refresh.artifact.json"
NATIVE_SEED = ROOT / "lang/src/compiler/lib/direct_state_plan_refresh_native_seed.hako"
FIXTURE = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-direct-state-plan-refresh-hako-native-source-seed-v0.json"


def rel(path: Path) -> str:
    return path.relative_to(ROOT).as_posix()


def sha256_text(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def load_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def render_native_seed() -> str:
    generated = GENERATED.read_text(encoding="utf-8")
    marker = "\nstatic box Main {"
    if marker not in generated:
        raise SystemExit("generated artifact is missing smoke Main marker")
    generated = generated[: generated.index(marker)].rstrip() + "\n"

    lines = generated.splitlines()
    body_start = None
    for index, line in enumerate(lines):
        if line.startswith("using ") or line.startswith("box ") or line.startswith("static box "):
            body_start = index
            break
    if body_start is None:
        raise SystemExit("generated artifact has no Hako body")
    body = "\n".join(lines[body_start:]).rstrip() + "\n"

    header = "\n".join(
        [
            f"// native-source-seed: {TOKEN}",
            f"// source-family: {FAMILY_ID}",
            "// source-input-state: DerivedArtifactSeedDraftInput",
            "// hako-adopted: 0",
            "// source-selfhost-claim: 0",
            "",
        ]
    )
    native = header + body
    for forbidden in ["@generated", "manual-edit: forbidden", "static box Main"]:
        if forbidden in native:
            raise SystemExit(f"native seed contains forbidden generated marker: {forbidden}")
    return native


def render_fixture(native_text: str) -> dict:
    selection = load_json(SELECTION)
    bridge = load_json(BRIDGE_V2)
    verifier = load_json(VERIFIER)
    manifest = load_json(GENERATED_MANIFEST)

    if selection["decision"]["selected_owner_edge_id"] != FAMILY_ID:
        raise SystemExit("selection rerun 002 did not select direct_state_plan_refresh")
    if selection["decision"]["selected_next_card"] != TOKEN:
        raise SystemExit("selection rerun 002 next card mismatch")
    if bridge["v2_policy"]["mention_only_forbidden_nonclaim_blocks_clean_narrow_seed_surface"] is not False:
        raise SystemExit("BridgePolicyV2 must scope mention-only forbidden nonclaims out of blocking")
    if bridge["v2_policy"]["mention_only_forbidden_nonclaim_is_seed_evidence"] is not False:
        raise SystemExit("BridgePolicyV2 must not treat forbidden nonclaims as seed evidence")
    if verifier["result"] != "VerifiedHakoFamilyIR":
        raise SystemExit("direct_state verifier result is not VerifiedHakoFamilyIR")
    if manifest["family_id"] != FAMILY_ID:
        raise SystemExit("artifact manifest family mismatch")
    if manifest["claims"]["direct_state_plan_refresh"] != 1:
        raise SystemExit("artifact manifest does not claim direct_state_plan_refresh evidence")
    for key in ["runtime_fallback", "new_backend_route", "new_abi", "new_canonical_mir_instruction", "source_selfhost_claim"]:
        if manifest["claims"].get(key) != 0:
            raise SystemExit(f"artifact manifest forbidden claim is not zero: {key}")

    return {
        "schema_version": 0,
        "kind": "MirBuilderDirectStatePlanRefreshHakoNativeSourceSeedV1",
        "token": TOKEN,
        "family_id": FAMILY_ID,
        "input_authority": {
            "strict_candidate_selection_rerun_002": rel(SELECTION),
            "bridge_policy_v2": rel(BRIDGE_V2),
            "verifier_result": rel(VERIFIER),
            "generated_artifact": rel(GENERATED),
            "generated_artifact_manifest": rel(GENERATED_MANIFEST),
            "derived_to_native_model": "docs/development/current/main/design/derived-to-native-hako-artifact-model-ssot.md",
        },
        "native_source_seed": {
            "path": rel(NATIVE_SEED),
            "module_export": "lib.direct_state_plan_refresh_native_seed",
            "api": "DirectStatePlanRefreshApi.project_shadow_record",
            "native_seed_api": "DirectStatePlanRefreshApi.project_shadow_record",
            "outside_generated_tree": True,
            "generator_overwrite_guard": True,
            "native_seed_sha256": sha256_text(native_text),
        },
        "selected_behavior": {
            "operation": "DirectStatePlanRefreshPreparedOnly",
            "pilot_scope": "DirectStatePlanRefresh_prepared_direct_state_plan_refresh_only",
            "entrypoint": "direct_state_plan::refresh_module_direct_state_plans",
            "result_transport": "DirectStatePlanRefreshResultBox",
            "result_contract": {
                "direct_state_plan_refresh": 1,
                "direct_state_plan_refresh_only": 1,
                "canonical_json_parity": 1,
                "mutation_target_count": 1,
                "publication_target_count": 1,
                "projected_field_count": 8,
                "runtime_fallback": 0,
            },
        },
        "seed_status": {
            "native_source_owner_seed_present": 1,
            "hako_adopted_decision": 0,
            "native_edit_authority_claim": 0,
            "generated_artifact_as_edit_authority": 0,
            "source_selfhost_claim": 0,
        },
        "next_action": {
            "kind": "HakoAdoptionDecisionDeferred",
            "next_card": NEXT_CARD,
            "reason_token": "DirectStatePlanRefreshNativeSourceSeedMaterialized",
        },
        "provenance": {
            "strict_candidate_selection_rerun_002_hash": sha256_file(SELECTION),
            "bridge_policy_v2_hash": sha256_file(BRIDGE_V2),
            "verifier_result_hash": sha256_file(VERIFIER),
            "generated_artifact_hash": sha256_file(GENERATED),
            "generated_artifact_manifest_hash": sha256_file(GENERATED_MANIFEST),
        },
        "claims": {
            "manual_family_selection": 0,
            "native_seed_materialization": 1,
            "family_adoption_decision": 0,
            "source_selfhost_claim": 0,
            "generated_artifact_as_edit_authority": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_canonical_mir_instruction": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
            "rust_deletion": 0,
        },
    }


def write_if_changed(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    if path.exists() and path.read_text(encoding="utf-8") == text:
        return
    path.write_text(text, encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true", help="verify checked-in outputs")
    args = parser.parse_args()

    native_text = render_native_seed()
    fixture_text = json.dumps(render_fixture(native_text), indent=2, sort_keys=True) + "\n"

    if args.check:
        mismatches = []
        if not NATIVE_SEED.exists() or NATIVE_SEED.read_text(encoding="utf-8") != native_text:
            mismatches.append(rel(NATIVE_SEED))
        if not FIXTURE.exists() or FIXTURE.read_text(encoding="utf-8") != fixture_text:
            mismatches.append(rel(FIXTURE))
        if mismatches:
            raise SystemExit("stale generated outputs: " + ", ".join(mismatches))
    else:
        write_if_changed(NATIVE_SEED, native_text)
        write_if_changed(FIXTURE, fixture_text)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
