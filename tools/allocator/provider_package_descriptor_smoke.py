#!/usr/bin/env python3
"""Resolve and call only the provider descriptor export."""

from __future__ import annotations

import argparse
import ctypes
from pathlib import Path

from provider_package_load_only_smoke import resolve_binary_path, sha256_file
from provider_package_metadata_preflight import (
    DESCRIPTOR_EXPORT_NAME,
    fail,
    load_manifest,
    validate_manifest,
)


DESCRIPTOR_MAGIC = 0x484B5250
ABI_MAJOR = 1


class HakoProviderDescriptorV1(ctypes.Structure):
    _fields_ = [
        ("magic", ctypes.c_uint32),
        ("abi_major", ctypes.c_uint16),
        ("abi_minor", ctypes.c_uint16),
        ("descriptor_size", ctypes.c_uint32),
        ("provider_id", ctypes.c_char_p),
        ("provider_kind", ctypes.c_char_p),
        ("provider_version", ctypes.c_char_p),
        ("capability_bits", ctypes.c_uint64),
        ("safety_flags", ctypes.c_uint64),
        ("contract_hash", ctypes.c_char_p),
        ("function_table_hash", ctypes.c_char_p),
        ("api_table_size", ctypes.c_uint32),
        ("reserved", ctypes.c_uint32),
    ]


def decode(value: bytes | None, field: str) -> str:
    if value is None:
        fail(f"descriptor.{field} must not be null")
    return value.decode("utf-8")


def load_descriptor(binary_path: Path) -> HakoProviderDescriptorV1:
    lib = ctypes.CDLL(str(binary_path))
    try:
        fn = getattr(lib, DESCRIPTOR_EXPORT_NAME)
    except AttributeError:
        fail(f"missing required export: {DESCRIPTOR_EXPORT_NAME}")
    fn.argtypes = []
    fn.restype = ctypes.POINTER(HakoProviderDescriptorV1)
    ptr = fn()
    if not ptr:
        fail("descriptor export returned null")
    return ptr.contents


def validate_descriptor(desc: HakoProviderDescriptorV1, fields: dict[str, str]) -> dict[str, str]:
    if desc.magic != DESCRIPTOR_MAGIC:
        fail("descriptor.magic mismatch")
    if desc.abi_major != ABI_MAJOR:
        fail("descriptor.abi_major mismatch")
    if desc.descriptor_size < ctypes.sizeof(HakoProviderDescriptorV1):
        fail("descriptor.descriptor_size too small")

    provider_id = decode(desc.provider_id, "provider_id")
    provider_kind = decode(desc.provider_kind, "provider_kind")
    provider_version = decode(desc.provider_version, "provider_version")
    contract_hash = decode(desc.contract_hash, "contract_hash").lower()
    function_table_hash = decode(desc.function_table_hash, "function_table_hash").lower()

    if provider_kind != "allocator":
        fail("descriptor.provider_kind must be allocator for this row")
    if contract_hash != fields["contract_hash"]:
        fail("descriptor.contract_hash does not match manifest")

    return {
        "provider_id": provider_id,
        "provider_kind": provider_kind,
        "provider_version": provider_version,
        "contract_hash": contract_hash,
        "function_table_hash": function_table_hash,
        "abi_major": str(desc.abi_major),
        "abi_minor": str(desc.abi_minor),
        "descriptor_size": str(desc.descriptor_size),
        "capability_bits": str(desc.capability_bits),
        "safety_flags": str(desc.safety_flags),
        "api_table_size": str(desc.api_table_size),
    }


def emit(fields: dict[str, str], descriptor: dict[str, str], manifest_path: Path, binary_path: Path) -> str:
    lines = [
        "output_contract=hakorune-provider-package-descriptor-smoke-v0",
        "dll_mode=descriptor-smoke",
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
        f"required_export={DESCRIPTOR_EXPORT_NAME}",
        f"descriptor_provider_id={descriptor['provider_id']}",
        f"descriptor_provider_kind={descriptor['provider_kind']}",
        f"descriptor_provider_version={descriptor['provider_version']}",
        f"descriptor_abi_major={descriptor['abi_major']}",
        f"descriptor_abi_minor={descriptor['abi_minor']}",
        f"descriptor_size={descriptor['descriptor_size']}",
        f"descriptor_capability_bits={descriptor['capability_bits']}",
        f"descriptor_safety_flags={descriptor['safety_flags']}",
        f"descriptor_contract_hash={descriptor['contract_hash']}",
        f"descriptor_function_table_hash={descriptor['function_table_hash']}",
        f"descriptor_api_table_size={descriptor['api_table_size']}",
        "manifest_ready=1",
        "descriptor_ready=1",
        "binary_hash_ready=1",
        "shared_library_load_executed=1",
        "required_export_resolved=1",
        "descriptor_read_executed=1",
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

    descriptor = validate_descriptor(load_descriptor(binary_path), fields)
    report = emit(fields, descriptor, manifest_path, binary_path)
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
