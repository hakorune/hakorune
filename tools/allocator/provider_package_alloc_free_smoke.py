#!/usr/bin/env python3
"""Call explicit provider alloc/free without process replacement."""

from __future__ import annotations

import argparse
from ctypes import byref, c_size_t
from pathlib import Path

from provider_package_api_bind_smoke import emit as emit_bind
from provider_package_api_bind_smoke import load_api, run


def emit(
    base_report: str,
    ptr: int,
    owns_result: int,
    free_claim_result: int,
    usable_size_claim_result: int,
    usable_size_claim_value: int,
    size: int,
    align: int,
) -> str:
    replacements = {
        "output_contract=hakorune-provider-package-api-bind-smoke-v0": (
            "output_contract=hakorune-provider-package-alloc-free-smoke-v0"
        ),
        "dll_mode=provider-api-bind": "dll_mode=provider-alloc-free",
        "provider_call_executed=0": "provider_call_executed=1",
        "allocator_entrypoint_called=0": "allocator_entrypoint_called=1",
    }
    text = base_report
    for old, new in replacements.items():
        text = text.replace(old, new)
    lines = text.rstrip("\n").splitlines()
    insert_at = lines.index("allocator_entrypoint_called=1") + 1
    lines[insert_at:insert_at] = [
        "provider_alloc_executed=1",
        "provider_free_executed=1",
        "provider_free_claim_executed=1",
        f"provider_free_claim_result={free_claim_result}",
        "provider_usable_size_claim_executed=1",
        f"provider_usable_size_claim_result={usable_size_claim_result}",
        f"provider_usable_size_claim_value={usable_size_claim_value}",
        f"provider_owns_result={owns_result}",
        "allocation_count=1",
        "free_count=1",
        f"requested_bytes={size}",
        f"requested_align={align}",
        f"allocated_pointer_nonzero={1 if ptr else 0}",
    ]
    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    parser.add_argument("--size", type=int, default=32)
    parser.add_argument("--align", type=int, default=8)
    args = parser.parse_args()
    if args.size < 1:
        raise SystemExit("--size must be positive")
    if args.align < 1:
        raise SystemExit("--align must be positive")

    manifest_path = args.manifest.resolve()
    fields, descriptor, api_info, binary_path = run(manifest_path)
    api = load_api(binary_path)
    ptr = int(api.alloc(args.size, args.align) or 0)
    if ptr == 0:
        raise SystemExit("[provider-package-metadata-preflight] provider alloc returned null")
    owns_result = int(api.owns(ptr))
    usable_size_claim = getattr(api, "usable_size_claim", None)
    if usable_size_claim is not None:
        size_out = c_size_t(0)
        usable_size_claim_result = int(usable_size_claim(ptr, byref(size_out)))
        usable_size_claim_value = int(size_out.value)
    else:
        usable_size_claim_result = 0
        usable_size_claim_value = 0
    free_claim = getattr(api, "free_claim", None)
    if free_claim is not None:
        free_claim_result = int(free_claim(ptr))
        if free_claim_result != 1:
            raise SystemExit("[provider-package-metadata-preflight] provider free_claim failed")
    else:
        free_claim_result = 0
        api.free(ptr)

    base = emit_bind(fields, descriptor, api_info, manifest_path, binary_path)
    report = emit(
        base,
        ptr,
        owns_result,
        free_claim_result,
        usable_size_claim_result,
        usable_size_claim_value,
        args.size,
        args.align,
    )
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
