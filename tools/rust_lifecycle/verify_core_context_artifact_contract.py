#!/usr/bin/env python3
"""Verify CoreContext generated artifact against its verified family contract."""

from __future__ import annotations

import argparse
import json
import re
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from mirbuilder_core_context_artifacts import ROOT, core_context_contract


@dataclass(frozen=True)
class HakoObservation:
    fields: tuple[str, ...]
    api_methods: tuple[str, ...]


def _read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def observe_core_context_hako(path: Path) -> HakoObservation:
    text = path.read_text(encoding="utf-8")
    fields: list[str] = []
    in_box = False
    for line in text.splitlines():
        stripped = line.strip()
        if stripped == "box CoreContext {":
            in_box = True
            continue
        if not in_box:
            continue
        if stripped.startswith("birth("):
            break
        match = re.match(r"^([A-Za-z_][A-Za-z0-9_]*):\s*i64$", stripped)
        if match:
            fields.append(match.group(1))

    api_methods: list[str] = []
    in_api = False
    depth = 0
    for line in text.splitlines():
        stripped = line.strip()
        if stripped == "static box CoreContextApi {":
            in_api = True
            depth = 1
            continue
        if not in_api:
            continue
        match = re.match(r"^([A-Za-z_][A-Za-z0-9_]*)\(ctx\):\s*i64\s*\{$", stripped)
        if match:
            api_methods.append(match.group(1))
        depth += stripped.count("{")
        depth -= stripped.count("}")
        if depth == 0:
            break

    return HakoObservation(fields=tuple(fields), api_methods=tuple(api_methods))


def verify_contract(
    *,
    manifest: dict[str, Any],
    verifier: dict[str, Any],
    observation: HakoObservation,
) -> None:
    contract = core_context_contract()
    if manifest.get("family_id") != contract.family_id:
        raise AssertionError("manifest family_id mismatch")
    if manifest.get("pilot_scope") != contract.artifact.pilot_scope:
        raise AssertionError("manifest pilot_scope mismatch")
    if manifest.get("excluded_methods") != list(contract.denied_method_ids):
        raise AssertionError("manifest excluded_methods mismatch")
    if verifier.get("family_id") != contract.family_id:
        raise AssertionError("verifier family_id mismatch")
    checks = verifier.get("checks") or {}
    if checks.get("selected_body_count") != contract.selected_body_count_label:
        raise AssertionError("verifier selected_body_count mismatch")
    if checks.get("excluded_methods") != list(contract.denied_method_ids):
        raise AssertionError("verifier excluded_methods mismatch")
    notes = verifier.get("transport_notes") or {}
    for key, value in contract.semantic_transports.items():
        if notes.get(key) != value:
            raise AssertionError(f"transport note mismatch: {key}")
    if set(observation.fields) != set(contract.expected_fields):
        raise AssertionError(
            f"hako field mismatch: expected={sorted(contract.expected_fields)} actual={sorted(observation.fields)}"
        )
    if tuple(observation.api_methods) != contract.selected_api_method_names:
        raise AssertionError(
            f"hako api method mismatch: expected={contract.selected_api_method_names} actual={observation.api_methods}"
        )


def _expect_failure(label: str, fn) -> None:
    try:
        fn()
    except AssertionError:
        return
    raise AssertionError(f"drift probe did not fail: {label}")


def run_drift_probes(manifest: dict[str, Any], verifier: dict[str, Any], observation: HakoObservation) -> None:
    contract = core_context_contract()
    bad_verifier = json.loads(json.dumps(verifier))
    bad_verifier.setdefault("transport_notes", {})["value_id_transport"] = "BasicBlockIdAsI64"
    _expect_failure(
        "transport mismatch",
        lambda: verify_contract(manifest=manifest, verifier=bad_verifier, observation=observation),
    )

    bad_observation = HakoObservation(
        fields=observation.fields,
        api_methods=tuple(name for name in observation.api_methods if name != "peek_next_block"),
    )
    _expect_failure(
        "missing api method",
        lambda: verify_contract(manifest=manifest, verifier=verifier, observation=bad_observation),
    )

    bad_manifest = json.loads(json.dumps(manifest))
    bad_manifest["excluded_methods"] = [contract.selected_method_ids[0]]
    _expect_failure(
        "selected method denied",
        lambda: verify_contract(manifest=bad_manifest, verifier=verifier, observation=observation),
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--drift-probes", action="store_true")
    args = parser.parse_args()

    manifest = _read_json(ROOT / "lang/generated/rust_derived/hakorune_mir_builder/core_context.artifact.json")
    verifier = _read_json(
        ROOT
        / "docs/development/current/main/design/fixtures/rust-lifecycle/core-context-derived-artifact-verifier-result-v0.json"
    )
    observation = observe_core_context_hako(ROOT / "lang/generated/rust_derived/hakorune_mir_builder/core_context.hako")
    verify_contract(manifest=manifest, verifier=verifier, observation=observation)
    if args.drift_probes:
        run_drift_probes(manifest, verifier, observation)
    print("core_context_artifact_contract=green")


if __name__ == "__main__":
    main()
