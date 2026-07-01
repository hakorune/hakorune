#!/usr/bin/env python3
"""Materialize the TypedObjectPlanRefresh native Hako source seed."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]

TOKEN = "MIRBUILDER-TYPED-OBJECT-PLAN-REFRESH-HAKO-NATIVE-SOURCE-SEED-001"
FAMILY_ID = "hakorune_mir_builder::typed_object_plan_refresh"
NEXT_CARD = "MIRBUILDER-TYPED-OBJECT-PLAN-REFRESH-HAKO-ADOPTION-DECISION-001"

SELECTION = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-strict-converter-emission-native-seed-candidate-selection-rerun-004-v0.json"
VERIFIER = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-typed-object-plan-refresh-derived-hako-verifier-result-v0.json"
GENERATED = ROOT / "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_typed_object_plan_refresh.hako"
GENERATED_MANIFEST = ROOT / "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_typed_object_plan_refresh.artifact.json"
NATIVE_SEED = ROOT / "lang/src/compiler/lib/typed_object_plan_refresh_native_seed.hako"
FIXTURE = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-typed-object-plan-refresh-hako-native-source-seed-v0.json"
ADOPTION_FIXTURE = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-typed-object-plan-refresh-hako-adoption-decision-v0.json"


def rel(path: Path) -> str:
    return path.relative_to(ROOT).as_posix()


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

    hako_adopted = 0
    if ADOPTION_FIXTURE.exists():
        adoption = load_json(ADOPTION_FIXTURE)
        if adoption.get("decision", {}).get("value") == "Adopt":
            hako_adopted = 1

    header = "\n".join(
        [
            f"// native-source-seed: {TOKEN}",
            f"// source-family: {FAMILY_ID}",
            "// source-input-state: DerivedArtifactSeedDraftInput",
            f"// hako-adopted: {hako_adopted}",
            "// source-selfhost-claim: 0",
            "",
        ]
    )
    native = header + body
    for forbidden in ["@generated", "manual-edit: forbidden", "static box Main"]:
        if forbidden in native:
            raise SystemExit(f"native seed contains forbidden generated marker: {forbidden}")
    return native


def render_fixture() -> dict:
    selection = load_json(SELECTION)
    verifier = load_json(VERIFIER)
    manifest = load_json(GENERATED_MANIFEST)

    if selection["decision"]["selected_owner_edge_id"] != FAMILY_ID:
        raise SystemExit("selection rerun 004 did not select typed_object_plan_refresh")
    if selection["decision"]["selected_next_card"] != TOKEN:
        raise SystemExit("selection rerun 004 next card mismatch")
    if verifier["result"] != "VerifiedHakoFamilyIR":
        raise SystemExit("typed_object verifier result is not VerifiedHakoFamilyIR")
    if manifest["family_id"] != FAMILY_ID:
        raise SystemExit("artifact manifest family mismatch")
    if manifest["claims"]["typed_object_plan_refresh"] != 1:
        raise SystemExit("artifact manifest does not claim typed_object_plan_refresh evidence")
    for key in ["runtime_fallback", "new_backend_route", "new_abi", "new_canonical_mir_instruction", "source_selfhost_claim"]:
        if manifest["claims"].get(key) != 0:
            raise SystemExit(f"artifact manifest forbidden claim is not zero: {key}")

    return {
        "schema_version": 0,
        "kind": "MirBuilderTypedObjectPlanRefreshHakoNativeSourceSeedV1",
        "token": TOKEN,
        "family_id": FAMILY_ID,
        "input_authority": {
            "strict_candidate_selection_rerun_004": rel(SELECTION),
            "verifier_result": rel(VERIFIER),
            "generated_artifact": rel(GENERATED),
            "generated_artifact_manifest": rel(GENERATED_MANIFEST),
            "derived_to_native_model": "docs/development/current/main/design/derived-to-native-hako-artifact-model-ssot.md",
        },
        "native_source_seed": {
            "path": rel(NATIVE_SEED),
            "module_export": "lib.typed_object_plan_refresh_native_seed",
            "api": "TypedObjectPlanRefreshApi.project_shadow_record",
            "native_seed_api": "TypedObjectPlanRefreshApi.project_shadow_record",
            "outside_generated_tree": True,
            "generator_overwrite_guard": True,
        },
        "selected_behavior": {
            "operation": "TypedObjectPlanRefreshPreparedOnly",
            "pilot_scope": "TypedObjectPlanRefresh_prepared_typed_object_plan_refresh_only",
            "entrypoint": "typed_object_plan::refresh_module_typed_object_plans",
            "result_transport": "TypedObjectPlanRefreshResultBox",
            "result_contract": {
                "typed_object_plan_refresh_only": 1,
                "direct_state_plan_refresh": 0,
                "canonical_json_parity": 1,
                "mutation_target_count": 1,
                "publication_target_count": 1,
                "projected_field_count": 5,
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
            "reason_token": "TypedObjectPlanRefreshNativeSourceSeedMaterialized",
        },
        "provenance": {
            "strict_candidate_selection_rerun_004_hash": sha256_file(SELECTION),
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
    fixture_text = json.dumps(render_fixture(), indent=2, sort_keys=True) + "\n"

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
