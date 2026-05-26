#!/usr/bin/env python3
"""Load a provider package shared library without resolving exports."""

from __future__ import annotations

import argparse
import ctypes
import hashlib
from pathlib import Path

from provider_package_metadata_preflight import fail, load_manifest, validate_manifest


def resolve_binary_path(manifest_path: Path, binary: str) -> Path:
    path = Path(binary)
    if not path.is_absolute():
        path = manifest_path.parent / path
    return path.resolve()


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def emit(fields: dict[str, str], manifest_path: Path, binary_path: Path) -> str:
    lines = [
        "output_contract=hakorune-provider-package-load-only-smoke-v0",
        "dll_mode=load-only",
        f"source_path={manifest_path}",
        f"binary_path={binary_path}",
        f"schema_version={fields['schema_version']}",
        f"provider_name={fields['provider_name']}",
        f"abi={fields['abi']}",
        f"target={fields['target']}",
        f"profile={fields['profile']}",
        f"binary={fields['binary']}",
        f"binary_sha256={fields['binary_sha256']}",
        f"contract_hash={fields['contract_hash']}",
        "manifest_ready=1",
        "descriptor_ready=0",
        "binary_hash_ready=1",
        "shared_library_load_executed=1",
        "required_export_resolved=0",
        "descriptor_read_executed=0",
        "provider_call_executed=0",
        "allocator_entrypoint_called=0",
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

    manifest_path = args.manifest.resolve()
    if not manifest_path.exists():
        fail(f"missing manifest: {manifest_path}")
    fields = validate_manifest(load_manifest(manifest_path))
    binary_path = resolve_binary_path(manifest_path, fields["binary"])
    if not binary_path.exists():
        fail(f"missing shared library: {binary_path}")
    actual_sha = sha256_file(binary_path)
    if actual_sha != fields["binary_sha256"]:
        fail("artifact.sha256/binary_sha256 does not match shared library bytes")

    # Load only. Do not call getattr(), dlsym/GetProcAddress, descriptor exports,
    # provider APIs, or allocator entrypoints in this row.
    ctypes.CDLL(str(binary_path))

    report = emit(fields, manifest_path, binary_path)
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
