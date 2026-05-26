#!/usr/bin/env python3
"""Validate a provider-package manifest without loading a shared library."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any


HEX64_RE = re.compile(r"^[0-9a-fA-F]{64}$")
EXPORT_NAME = "hakorune_provider_get_api_v1"
DESCRIPTOR_EXPORT_NAME = "hakorune_provider_descriptor_v1"


def fail(message: str) -> None:
    raise SystemExit(f"[provider-package-metadata-preflight] {message}")


def require_string(data: dict[str, Any], key: str) -> str:
    value = data.get(key)
    if not isinstance(value, str) or not value:
        fail(f"{key} must be a non-empty string")
    return value


def optional_string(data: dict[str, Any], key: str) -> str | None:
    value = data.get(key)
    if value is None:
        return None
    if not isinstance(value, str) or not value:
        fail(f"{key} must be a non-empty string when present")
    return value


def require_hex64(data: dict[str, Any], key: str) -> str:
    value = require_string(data, key)
    if HEX64_RE.fullmatch(value) is None:
        fail(f"{key} must be 64 hex characters")
    return value


def require_hex64_value(key: str, value: str) -> str:
    if HEX64_RE.fullmatch(value) is None:
        fail(f"{key} must be 64 hex characters")
    return value


def load_manifest(path: Path) -> dict[str, Any]:
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        fail(f"invalid JSON: {exc}")
    if not isinstance(data, dict):
        fail("manifest root must be an object")
    return data


def validate_manifest(data: dict[str, Any]) -> dict[str, str]:
    schema_version = optional_string(data, "schema_version") or "legacy-compact-provider-package"
    provider_name = require_string(data, "provider_name")
    abi = optional_string(data, "abi") or require_string(data, "abi_version")
    target = optional_string(data, "target") or require_string(data, "target_triple")
    profile = optional_string(data, "profile") or "speed"

    artifact = data.get("artifact")
    if artifact is None:
        binary = require_string(data, "binary")
        binary_sha256 = require_hex64(data, "binary_sha256")
    elif isinstance(artifact, dict):
        binary = require_string(artifact, "path")
        binary_sha256 = require_hex64_value("artifact.sha256", require_string(artifact, "sha256"))
    else:
        fail("artifact must be an object when present")

    contract_hash = require_hex64(data, "contract_hash")

    if abi not in {"hakorune-provider-v1", "hakorune-provider-abi-v1"}:
        fail("abi must be hakorune-provider-v1 or hakorune-provider-abi-v1")
    if profile not in {"speed", "diagnostic"}:
        fail("profile must be speed or diagnostic")
    if not (binary.endswith(".dll") or binary.endswith(".so") or binary.endswith(".dylib")):
        fail("binary must name a shared-library artifact")

    features = data.get("features")
    capabilities = data.get("capabilities")
    if features is None:
        if not isinstance(capabilities, list) or "descriptor" not in capabilities:
            fail("capabilities must include descriptor when features is absent")
    elif isinstance(features, dict):
        speed_lane = features.get("speed_lane")
        if profile == "speed" and speed_lane is not True:
            fail("speed profile requires features.speed_lane=true")
    else:
        fail("features must be an object when present")

    exports = data.get("required_exports", data.get("exports"))
    if not isinstance(exports, list):
        fail("required_exports/exports must be a list")
    if EXPORT_NAME not in exports and DESCRIPTOR_EXPORT_NAME not in exports:
        fail(f"required_exports/exports must include {DESCRIPTOR_EXPORT_NAME} or {EXPORT_NAME}")

    activation = data.get("activation")
    if activation is not None:
        if not isinstance(activation, dict):
            fail("activation must be an object when present")
        for key in (
            "allocator_replacement_allowed",
            "hook_allowed",
            "global_allocator_allowed",
        ):
            if activation.get(key) is not False:
                fail(f"activation.{key} must be false in metadata preflight")

    return {
        "schema_version": schema_version,
        "provider_name": provider_name,
        "abi": abi,
        "target": target,
        "profile": profile,
        "binary": binary,
        "binary_sha256": binary_sha256.lower(),
        "contract_hash": contract_hash.lower(),
        "export": DESCRIPTOR_EXPORT_NAME if DESCRIPTOR_EXPORT_NAME in exports else EXPORT_NAME,
    }


def emit(fields: dict[str, str], source: Path) -> str:
    lines = [
        "output_contract=hakorune-provider-package-metadata-preflight-v0",
        "dll_mode=metadata-preflight",
        f"source_path={source}",
        f"schema_version={fields['schema_version']}",
        f"provider_name={fields['provider_name']}",
        f"abi={fields['abi']}",
        f"target={fields['target']}",
        f"profile={fields['profile']}",
        f"binary={fields['binary']}",
        f"binary_sha256={fields['binary_sha256']}",
        f"contract_hash={fields['contract_hash']}",
        f"required_export={fields['export']}",
        "manifest_ready=1",
        "descriptor_ready=0",
        "binary_hash_ready=1",
        "shared_library_load_executed=0",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "winner_claim=0",
        "summary=ok",
    ]
    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    manifest = args.manifest.resolve()
    if not manifest.exists():
        fail(f"missing manifest: {manifest}")
    report = emit(validate_manifest(load_manifest(manifest)), manifest)
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
