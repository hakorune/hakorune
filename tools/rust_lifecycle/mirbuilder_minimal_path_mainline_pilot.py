#!/usr/bin/env python3
"""Select the minimal-path composed execution closure on the mainline route.

This is a route-selection surface, not a new semantic projector. It consumes
the readiness result, the selected family route manifest, and the composed
execution closure artifact to record that the narrow family route is selected
on mainline without claiming Source Selfhost or HakoAdopted.
"""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from shared_family_generator import read_json, sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUT_DIR = ROOT / "lang/generated/rust_derived/hakorune_mir_builder"

READINESS_PATH = FIXTURES / "mirbuilder-minimal-path-mainline-readiness-resolution-v0.json"
ROUTE_MANIFEST_PATH = OUT_DIR / "family_routes.json"
CLOSURE_ARTIFACT_PATH = OUT_DIR / "mirbuilder_minimal_path_composed_execution_closure.artifact.json"
OUTPUT_PATH = FIXTURES / "mirbuilder-minimal-path-mainline-pilot-v0.json"

EXPECTED_FAMILY_ID = "hakorune_mir_builder::minimal_path_composed_execution_closure"
EXPECTED_ROUTE = "derived_hako"
EXPECTED_SCOPE = "MinimalMirBuilderComposedExecutionClosure_prepared_state_only"
EXPECTED_ROUTE_STATE = "DerivedMainline"


class PilotError(RuntimeError):
    pass


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def require(condition: bool, message: str) -> None:
    if not condition:
        raise PilotError(message)


def validate_readiness() -> dict[str, Any]:
    readiness = read_json(READINESS_PATH)
    require(
        readiness.get("kind") == "MinimalMirBuilderExecutionPathMainlineReadinessResolutionV1",
        "readiness resolution has wrong kind",
    )
    decision = readiness.get("decision") or {}
    require(decision.get("kind") == "ReadyForMinimalPathMainlinePilot", "readiness must stay ReadyForMinimalPathMainlinePilot")
    require(decision.get("next_slice_token") == "MIRBUILDER-MINIMAL-PATH-MAINLINE-PILOT-001", "readiness next slice token drift")
    require(decision.get("reason_token") == "GeneratedHakoExecutableClosureClosed", "readiness reason token drift")
    mainline = readiness.get("mainline_readiness") or {}
    require(mainline.get("readiness_state") == "Ready", "mainline readiness must stay ready")
    require(mainline.get("generated_hako_executable_closure") == "Closed", "generated Hako executable closure must remain closed")
    require(mainline.get("allocation_policy_adoption") == "Adopt", "allocation-policy adoption must stay Adopt")
    require(mainline.get("next_unconsumed_edge_classification") == "Closed", "next unconsumed edge must stay closed")
    require(readiness.get("claims", {}).get("manual_next_owner_selection") == 0, "manual next owner selection must remain off")
    return {
        "kind": readiness["kind"],
        "path": rel(READINESS_PATH),
        "sha256": sha256_file(READINESS_PATH),
        "decision": decision,
        "mainline_readiness": mainline,
    }


def validate_route_manifest() -> dict[str, Any]:
    routes = read_json(ROUTE_MANIFEST_PATH)
    require(routes.get("kind") == "RustDerivedHakoFamilyRouteManifest", "route manifest has wrong kind")
    route_entries = [
        route
        for route in routes.get("routes", [])
        if route.get("artifact_manifest") == rel(CLOSURE_ARTIFACT_PATH)
    ]
    require(len(route_entries) == 1, "composed execution closure route entry must be unique")
    route = route_entries[0]
    require(route.get("family_id") == EXPECTED_FAMILY_ID, "route family drift")
    require(route.get("route") == EXPECTED_ROUTE, "route label drift")
    require(route.get("state") == EXPECTED_ROUTE_STATE, "route state drift")
    require(route.get("selected_on_mainline") is True, "route must be selected on mainline")
    require(route.get("fallback_policy") == "forbidden", "route fallback must remain forbidden")
    require(route.get("rust_bootstrap_route") == "retained", "route must retain rust bootstrap")
    require(route.get("rust_oracle_route") == "retained", "route must retain rust oracle")
    require(route.get("mainline_selection_scope") == EXPECTED_SCOPE, "route scope drift")
    require(route.get("guard_command") == "bash tools/checks/rust_lifecycle_mirbuilder_minimal_path_mainline_pilot_guard.sh", "route guard drift")

    claims = routes.get("claims") or {}
    require(claims.get("minimal_path_composed_execution_closure_selected") == 1, "route manifest claim drift")
    require(claims.get("source_selfhost_claim") == 0, "route manifest must not claim source selfhost")
    require(claims.get("runtime_try_hako_then_rust_fallback", 0) == 0, "route manifest must keep fallback off")
    require(claims.get("backend_behavior_changed") == 0, "route manifest must keep backend behavior unchanged")
    return {
        "kind": routes["kind"],
        "path": rel(ROUTE_MANIFEST_PATH),
        "sha256": sha256_file(ROUTE_MANIFEST_PATH),
        "route": route,
        "claims": claims,
    }


def validate_closure_artifact() -> dict[str, Any]:
    artifact = read_json(CLOSURE_ARTIFACT_PATH)
    require(artifact.get("kind") == "RustDerivedHakoArtifact", "closure artifact has wrong kind")
    require(artifact.get("family_id") == EXPECTED_FAMILY_ID, "closure artifact family drift")
    require(artifact.get("state") == EXPECTED_ROUTE_STATE, "closure artifact state drift")
    require(artifact.get("pilot_scope") == EXPECTED_SCOPE, "closure artifact scope drift")
    claims = artifact.get("claims") or {}
    require(claims.get("generated_hako_change") == 1, "closure artifact must keep generated_hako_change")
    require(claims.get("generated_hako_executable_closure") == 1, "closure artifact must keep executable closure")
    require(claims.get("selected_existing_contracts_consumed") == 1, "closure artifact must keep existing contracts consumed")
    require(claims.get("route_chain_closed") == 1, "closure artifact must keep route chain closed")
    require(claims.get("mainline_selected") == 1, "closure artifact must remain selected on mainline")
    require(claims.get("generated_hako_manual_edit") == 0, "closure artifact manual edit must remain off")
    require(claims.get("runtime_fallback") == 0, "closure artifact fallback must remain off")
    require(claims.get("new_backend_route") == 0, "closure artifact backend route must remain off")
    require(claims.get("new_abi") == 0, "closure artifact ABI must remain off")
    require(claims.get("source_selfhost_claim") == 0, "closure artifact must not claim source selfhost")
    output = artifact.get("output") or {}
    hako_path = ROOT / output.get("hako_path", "")
    require(output.get("hako_path"), "closure artifact missing hako output path")
    require(hako_path.exists(), f"closure artifact hako output missing: {hako_path}")
    require(output.get("hako_sha256") == sha256_file(hako_path), "closure artifact hako hash is stale")
    return {
        "kind": artifact["kind"],
        "path": rel(CLOSURE_ARTIFACT_PATH),
        "sha256": sha256_file(CLOSURE_ARTIFACT_PATH),
        "artifact": artifact,
        "claims": claims,
        "output": output,
    }


def build_pilot_result() -> dict[str, Any]:
    readiness = validate_readiness()
    route_manifest = validate_route_manifest()
    closure_artifact = validate_closure_artifact()
    route = route_manifest["route"]
    artifact = closure_artifact["artifact"]
    return {
        "schema_version": 0,
        "kind": "MinimalMirBuilderMainlinePilotSelectionV1",
        "family_id": EXPECTED_FAMILY_ID,
        "selected_scope": EXPECTED_SCOPE,
        "selected_route": route["route"],
        "route_state": route["state"],
        "selected_on_mainline": int(bool(route["selected_on_mainline"])),
        "fallback_policy": route["fallback_policy"],
        "input_profile": {"ast": "ASTNode::Literal(Integer(0))"},
        "source_authority": {
            "readiness_resolution": readiness,
            "route_manifest": route_manifest,
            "closure_artifact": closure_artifact,
        },
        "route": {
            "artifact_manifest": route["artifact_manifest"],
            "mainline_selection_scope": route["mainline_selection_scope"],
            "guard_command": route["guard_command"],
            "rust_bootstrap_route": route["rust_bootstrap_route"],
            "rust_oracle_route": route["rust_oracle_route"],
        },
        "artifact": {
            "manifest_path": rel(CLOSURE_ARTIFACT_PATH),
            "manifest_sha256": sha256_file(CLOSURE_ARTIFACT_PATH),
            "hako_path": artifact.get("output", {}).get("hako_path"),
            "hako_sha256": artifact.get("output", {}).get("hako_sha256"),
            "state": artifact.get("state"),
        },
        "claims": {
            "route_manifest_verified": 1,
            "readiness_resolution_verified": 1,
            "selected_on_mainline": 1,
            "rust_bootstrap_retained": 1,
            "rust_oracle_retained": 1,
            "generated_artifact_manual_edit": 0,
            "runtime_try_hako_then_rust_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "backend_behavior_changed": 0,
            "source_selfhost_claim": 0,
            "full_minimal_path_mainline_selected": 0,
        },
    }


def run(*, check: bool) -> None:
    result = build_pilot_result()
    result_text = stable_json(result)
    if check:
        if not OUTPUT_PATH.exists() or OUTPUT_PATH.read_text() != result_text:
            raise PilotError(f"{OUTPUT_PATH.relative_to(ROOT)} is stale")
    else:
        write_if_changed(OUTPUT_PATH, result_text)

    print("output_contract=rust-lifecycle-mirbuilder-minimal-path-mainline-pilot-v0")
    print(f"family_id={EXPECTED_FAMILY_ID}")
    print(f"selected_route={result['selected_route']}")
    print(f"route_state={result['route_state']}")
    print(f"selected_on_mainline={result['selected_on_mainline']}")
    print(f"route_manifest_verified={result['claims']['route_manifest_verified']}")
    print(f"readiness_resolution_verified={result['claims']['readiness_resolution_verified']}")
    print(f"rust_bootstrap_retained={result['claims']['rust_bootstrap_retained']}")
    print(f"rust_oracle_retained={result['claims']['rust_oracle_retained']}")
    print(f"runtime_try_hako_then_rust_fallback={result['claims']['runtime_try_hako_then_rust_fallback']}")
    print(f"new_backend_route={result['claims']['new_backend_route']}")
    print(f"new_abi={result['claims']['new_abi']}")
    print(f"source_selfhost_claim={result['claims']['source_selfhost_claim']}")
    print(f"backend_behavior_changed={result['claims']['backend_behavior_changed']}")
    print(f"full_minimal_path_mainline_selected={result['claims']['full_minimal_path_mainline_selected']}")
    print("summary=ok")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    try:
        run(check=args.check)
    except PilotError as exc:
        raise SystemExit(f"error: {exc}") from exc


if __name__ == "__main__":
    main()
