#!/usr/bin/env python3
"""Bind the provider API table without calling provider functions."""

from __future__ import annotations

import argparse
import ctypes
from pathlib import Path

from provider_package_descriptor_smoke import load_descriptor, validate_descriptor
from provider_package_load_only_smoke import resolve_binary_path, sha256_file
from provider_package_metadata_preflight import fail, load_manifest, validate_manifest


GET_API_EXPORT_NAME = "hakorune_provider_get_api_v1"
API_MAGIC = 0x484B5241
API_MAJOR = 1

PING_FN = ctypes.CFUNCTYPE(ctypes.c_int)
ALLOC_FN = ctypes.CFUNCTYPE(ctypes.c_void_p, ctypes.c_size_t, ctypes.c_size_t)
FREE_FN = ctypes.CFUNCTYPE(None, ctypes.c_void_p)
OWNS_FN = ctypes.CFUNCTYPE(ctypes.c_int, ctypes.c_void_p)
FREE_CLAIM_FN = ctypes.CFUNCTYPE(ctypes.c_int, ctypes.c_void_p)


class HakoProviderApiV1Base(ctypes.Structure):
    _fields_ = [
        ("magic", ctypes.c_uint32),
        ("abi_major", ctypes.c_uint16),
        ("abi_minor", ctypes.c_uint16),
        ("api_table_size", ctypes.c_uint32),
        ("ping", PING_FN),
        ("alloc", ALLOC_FN),
        ("free", FREE_FN),
        ("owns", OWNS_FN),
    ]


class HakoProviderApiV1Claim(ctypes.Structure):
    _fields_ = HakoProviderApiV1Base._fields_ + [
        ("free_claim", FREE_CLAIM_FN),
    ]


def load_api(binary_path: Path) -> HakoProviderApiV1Base | HakoProviderApiV1Claim:
    lib = ctypes.CDLL(str(binary_path))
    try:
        fn = getattr(lib, GET_API_EXPORT_NAME)
    except AttributeError:
        fail(f"missing required export: {GET_API_EXPORT_NAME}")
    fn.argtypes = []
    fn.restype = ctypes.POINTER(HakoProviderApiV1Base)
    ptr = fn()
    if not ptr:
        fail("provider API export returned null")
    base = ptr.contents
    if base.api_table_size >= ctypes.sizeof(HakoProviderApiV1Claim):
        return ctypes.cast(ptr, ctypes.POINTER(HakoProviderApiV1Claim)).contents
    return base


def validate_api(api: HakoProviderApiV1Base | HakoProviderApiV1Claim) -> dict[str, str]:
    if api.magic != API_MAGIC:
        fail("api.magic mismatch")
    if api.abi_major != API_MAJOR:
        fail("api.abi_major mismatch")
    if api.api_table_size < ctypes.sizeof(HakoProviderApiV1Base):
        fail("api.api_table_size too small")
    if not api.ping or not api.alloc or not api.free or not api.owns:
        fail("api table function pointers must be non-null")
    free_claim = getattr(api, "free_claim", None)
    return {
        "api_abi_major": str(api.abi_major),
        "api_abi_minor": str(api.abi_minor),
        "api_table_size": str(api.api_table_size),
        "provider_free_claim_bound": "1" if free_claim else "0",
    }


def emit(
    fields: dict[str, str],
    descriptor: dict[str, str],
    api: dict[str, str],
    manifest_path: Path,
    binary_path: Path,
) -> str:
    lines = [
        "output_contract=hakorune-provider-package-api-bind-smoke-v0",
        "dll_mode=provider-api-bind",
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
        f"descriptor_provider_id={descriptor['provider_id']}",
        f"descriptor_provider_kind={descriptor['provider_kind']}",
        f"descriptor_contract_hash={descriptor['contract_hash']}",
        f"provider_api_export={GET_API_EXPORT_NAME}",
        f"api_abi_major={api['api_abi_major']}",
        f"api_abi_minor={api['api_abi_minor']}",
        f"api_table_size={api['api_table_size']}",
        f"provider_free_claim_bound={api['provider_free_claim_bound']}",
        "provider_abi_claim_ops_v1=1",
        "provider_free_claim_enabled=1",
        "provider_realloc_claim_enabled=0",
        "provider_usable_size_claim_enabled=0",
        "compat_alloc_free_owns_still_supported=1",
        "compat_owns_free_mainline=0",
        "manifest_ready=1",
        "descriptor_ready=1",
        "binary_hash_ready=1",
        "shared_library_load_executed=1",
        "required_export_resolved=1",
        "descriptor_read_executed=1",
        "provider_api_bound=1",
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


def run(manifest_path: Path) -> tuple[dict[str, str], dict[str, str], dict[str, str], Path]:
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
    api = validate_api(load_api(binary_path))
    return fields, descriptor, api, binary_path


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    manifest_path = args.manifest.resolve()
    fields, descriptor, api, binary_path = run(manifest_path)
    report = emit(fields, descriptor, api, manifest_path, binary_path)
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
