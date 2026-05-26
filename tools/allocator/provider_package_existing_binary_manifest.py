#!/usr/bin/env python3
"""Package an existing provider shared library with a manifest."""

from __future__ import annotations

import argparse
import hashlib
import json
import shutil
from pathlib import Path
from typing import Any


SCHEMA_VERSION = "hakorune-provider-package-v1"
ABI_VERSION = "hakorune-provider-abi-v1"
DESCRIPTOR_EXPORT = "hakorune_provider_descriptor_v1"
OUTPUT_CONTRACT = "hakorune-provider-package-existing-binary-manifest-v0"


def fail(message: str) -> None:
    raise SystemExit(f"[provider-package-existing-binary-manifest] {message}")


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def stable_sha256(data: dict[str, Any]) -> str:
    encoded = json.dumps(data, sort_keys=True, separators=(",", ":")).encode("utf-8")
    return hashlib.sha256(encoded).hexdigest()


def parse_csv(value: str) -> list[str]:
    items = [item.strip() for item in value.split(",") if item.strip()]
    if not items:
        fail("comma-separated list must not be empty")
    return items


def require_shared_library_name(path: Path) -> None:
    if path.name != path.name.strip() or not path.name:
        fail("binary path must have a stable file name")
    if path.suffix not in {".so", ".dll", ".dylib"}:
        fail("binary must be a shared-library artifact ending in .so, .dll, or .dylib")


def build_manifest(args: argparse.Namespace, artifact_name: str, artifact_sha: str, artifact_size: int) -> dict[str, Any]:
    required_exports = parse_csv(args.required_exports)
    capabilities = parse_csv(args.capabilities)
    if DESCRIPTOR_EXPORT not in required_exports:
        fail(f"required exports must include {DESCRIPTOR_EXPORT}")
    if "descriptor" not in capabilities:
        fail("capabilities must include descriptor")

    activation = {
        "provider_call_allowed": bool(args.provider_call_allowed),
        "allocator_replacement_allowed": False,
        "hook_allowed": False,
        "global_allocator_allowed": False,
    }
    contract = {
        "abi_version": ABI_VERSION,
        "provider_kind": args.provider_kind,
        "capabilities": capabilities,
        "required_exports": required_exports,
        "descriptor_schema_version": "hakorune-provider-descriptor-v1",
        "api_table_schema_version": "hakorune-provider-api-v1",
        "activation": activation,
        "memory_ownership_policy": "provider_alloc_provider_free",
    }
    contract_hash = args.contract_hash or stable_sha256(contract)

    return {
        "schema_version": SCHEMA_VERSION,
        "package_id": args.package_id,
        "provider_kind": args.provider_kind,
        "provider_name": args.provider_name,
        "provider_version": args.provider_version,
        "abi_version": ABI_VERSION,
        "target_triple": args.target_triple,
        "platform": args.platform,
        "profile": args.profile,
        "artifact": {
            "path": artifact_name,
            "sha256": artifact_sha,
            "size_bytes": artifact_size,
        },
        "contract_hash": contract_hash,
        "required_exports": required_exports,
        "capabilities": capabilities,
        "activation": activation,
    }


def emit_report(manifest: dict[str, Any], out_dir: Path, source_binary: Path) -> str:
    artifact = manifest["artifact"]
    lines = [
        f"output_contract={OUTPUT_CONTRACT}",
        "package_mode=existing-binary-manifest",
        f"package_dir={out_dir}",
        f"source_binary={source_binary}",
        f"manifest_path={out_dir / 'hakorune_provider.json'}",
        f"sha256_path={out_dir / 'hakorune_provider.sha256'}",
        f"schema_version={manifest['schema_version']}",
        f"package_id={manifest['package_id']}",
        f"provider_kind={manifest['provider_kind']}",
        f"provider_name={manifest['provider_name']}",
        f"provider_version={manifest['provider_version']}",
        f"abi_version={manifest['abi_version']}",
        f"target_triple={manifest['target_triple']}",
        f"platform={manifest['platform']}",
        f"profile={manifest['profile']}",
        f"artifact_path={artifact['path']}",
        f"artifact_sha256={artifact['sha256']}",
        f"artifact_size_bytes={artifact['size_bytes']}",
        f"contract_hash={manifest['contract_hash']}",
        f"required_exports={','.join(manifest['required_exports'])}",
        f"capabilities={','.join(manifest['capabilities'])}",
        f"provider_call_allowed={int(manifest['activation']['provider_call_allowed'])}",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "shared_library_load_executed=0",
        "required_export_resolved=0",
        "descriptor_read_executed=0",
        "provider_call_executed=0",
        "winner_claim=0",
        "summary=ok",
    ]
    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--artifact-name")
    parser.add_argument("--package-id", required=True)
    parser.add_argument("--provider-kind", default="allocator")
    parser.add_argument("--provider-name", required=True)
    parser.add_argument("--provider-version", default="0.1.0")
    parser.add_argument("--target-triple", required=True)
    parser.add_argument("--platform", required=True)
    parser.add_argument("--profile", choices=("speed", "diagnostic"), default="speed")
    parser.add_argument("--required-exports", default=DESCRIPTOR_EXPORT)
    parser.add_argument("--capabilities", default="descriptor,explicit_allocator_api")
    parser.add_argument("--contract-hash")
    parser.add_argument("--provider-call-allowed", action="store_true")
    parser.add_argument("--force", action="store_true")
    parser.add_argument("--report-out", type=Path)
    args = parser.parse_args()

    binary = args.binary.resolve()
    if not binary.exists() or not binary.is_file():
        fail(f"missing provider binary: {binary}")
    require_shared_library_name(binary)

    artifact_name = args.artifact_name or binary.name
    artifact_rel = Path(artifact_name)
    if artifact_rel.is_absolute() or len(artifact_rel.parts) != 1:
        fail("artifact name must be a single relative file name")
    require_shared_library_name(artifact_rel)

    out_dir = args.out_dir.resolve()
    manifest_path = out_dir / "hakorune_provider.json"
    sha_path = out_dir / "hakorune_provider.sha256"
    artifact_path = out_dir / artifact_name
    if out_dir.exists() and not args.force:
        existing = [manifest_path, sha_path, artifact_path]
        if any(path.exists() for path in existing):
            fail("package output already exists; pass --force to replace package files")
    out_dir.mkdir(parents=True, exist_ok=True)

    shutil.copy2(binary, artifact_path)
    artifact_sha = sha256_file(artifact_path)
    artifact_size = artifact_path.stat().st_size
    manifest = build_manifest(args, artifact_name, artifact_sha, artifact_size)

    manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    sha_path.write_text(f"{artifact_sha}  {artifact_name}\n", encoding="utf-8")

    report = emit_report(manifest, out_dir, binary)
    if args.report_out is None:
        print(report, end="")
    else:
        args.report_out.parent.mkdir(parents=True, exist_ok=True)
        args.report_out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
