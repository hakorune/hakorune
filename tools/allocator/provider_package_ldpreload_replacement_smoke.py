#!/usr/bin/env python3
"""Smoke a provider-backed LD_PRELOAD malloc-family replacement pilot."""

from __future__ import annotations

import argparse
import os
import subprocess
from pathlib import Path

from provider_package_api_bind_smoke import run
from provider_package_ldpreload_replacement_smoke_report import emit_report
from provider_package_ldpreload_replacement_smoke_sources import SHIM_C, SMOKE_C


def parse_report(path: Path) -> dict[str, str]:
    fields: dict[str, str] = {}
    if not path.exists():
        return fields
    for line in path.read_text(encoding="utf-8").splitlines():
        if "=" not in line:
            continue
        key, value = line.split("=", 1)
        fields[key] = value
    return fields


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    parser.add_argument(
        "--use-provider-usable-size",
        action="store_true",
        help="measurement-only: bypass shim pointer tracking through provider usable_size symbol",
    )
    parser.add_argument(
        "--assume-provider-owned",
        action="store_true",
        help="measurement-only: with usable-size mode, skip provider owns checks before free/realloc",
    )
    args = parser.parse_args()
    if args.assume_provider_owned and not args.use_provider_usable_size:
        raise SystemExit("--assume-provider-owned requires --use-provider-usable-size")

    manifest_path = args.manifest.resolve()
    _fields, _descriptor, _api, binary_path = run(manifest_path)

    out_dir = args.out_dir.resolve()
    out_dir.mkdir(parents=True, exist_ok=True)
    shim_source = out_dir / "hako_provider_ldpreload_replacement_pilot.c"
    shim_binary = out_dir / "libhako_provider_ldpreload_replacement_pilot.so"
    smoke_source = out_dir / "hako_provider_ldpreload_replacement_smoke.c"
    smoke_binary = out_dir / "hako_provider_ldpreload_replacement_smoke"
    shim_report = out_dir / "shim_counts.out"

    shim_source.write_text(SHIM_C.lstrip(), encoding="utf-8")
    smoke_source.write_text(SMOKE_C.lstrip(), encoding="utf-8")
    subprocess.run(
        [
            "cc",
            "-shared",
            "-fPIC",
            "-O2",
            "-Wall",
            "-Wextra",
            str(shim_source),
            "-ldl",
            "-o",
            str(shim_binary),
        ],
        check=True,
    )
    subprocess.run(
        ["cc", "-O2", "-Wall", "-Wextra", str(smoke_source), "-o", str(smoke_binary)],
        check=True,
    )

    env = os.environ.copy()
    env["LD_PRELOAD"] = str(shim_binary)
    env["HAKORUNE_PROVIDER_LIBRARY"] = str(binary_path)
    env["HAKORUNE_PROVIDER_LDPRELOAD_REPORT"] = str(shim_report)
    if args.use_provider_usable_size:
        env["HAKORUNE_PROVIDER_LDPRELOAD_USE_USABLE_SIZE"] = "1"
    if args.assume_provider_owned:
        env["HAKORUNE_PROVIDER_LDPRELOAD_ASSUME_PROVIDER_OWNED"] = "1"
    proc = subprocess.run([str(smoke_binary)], env=env, check=False)
    report = emit_report(
        manifest_path=manifest_path,
        binary_path=binary_path,
        shim_source=shim_source,
        shim_binary=shim_binary,
        smoke_source=smoke_source,
        smoke_binary=smoke_binary,
        shim_report=shim_report,
        smoke_exit_code=proc.returncode,
        shim_fields=parse_report(shim_report),
    )
    if "summary=ok" not in report:
        if args.out is not None:
            args.out.parent.mkdir(parents=True, exist_ok=True)
            args.out.write_text(report, encoding="utf-8")
        print(report, end="")
        return 1
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
