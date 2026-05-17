#!/usr/bin/env python3
"""Inventory guard manifest migration state without running guards."""

from __future__ import annotations

import argparse
import os
from pathlib import Path, PurePosixPath
import sys
import tomllib
from typing import Any


TAG = "guard-manifest-inventory"
CLOSEOUT_PROFILE = "hako-alloc-closeout"
IMPL_PREFIX = "tools/checks/impl/"
WRAPPER_PREFIX = "tools/checks/"
PUBLIC_CLOSEOUT_GLOB = "k2_wide_hako_alloc_*closeout_guard.sh"


def fail(message: str) -> None:
    print(f"[{TAG}] ERROR: {message}", file=sys.stderr)
    raise SystemExit(1)


def load_rows(manifest: Path) -> list[dict[str, Any]]:
    data = tomllib.loads(manifest.read_text(encoding="utf-8"))
    rows = data.get("rows")
    if not isinstance(rows, list):
        fail("guard_rows.toml must contain [[rows]] entries")
    result: list[dict[str, Any]] = []
    for idx, row in enumerate(rows, start=1):
        if not isinstance(row, dict):
            fail(f"row {idx} must be a table")
        result.append(row)
    return result


def row_id(row: dict[str, Any], idx: int) -> str:
    value = row.get("id")
    if not isinstance(value, str) or not value:
        fail(f"row {idx} must have a non-empty string id")
    return value


def row_profiles(row: dict[str, Any], rid: str) -> list[str]:
    profiles = row.get("profiles")
    if not isinstance(profiles, list) or not all(isinstance(v, str) for v in profiles):
        fail(f"{rid}: profiles must be a string array")
    return profiles


def row_cmd(row: dict[str, Any], rid: str) -> list[str]:
    cmd = row.get("cmd")
    if not isinstance(cmd, list) or not all(isinstance(v, str) for v in cmd):
        fail(f"{rid}: cmd must be a string array")
    return cmd


def collect_closeout_manifest(root: Path, rows: list[dict[str, Any]]) -> tuple[dict[str, dict[str, str]], list[str]]:
    expected: dict[str, dict[str, str]] = {}
    seen_wrappers: set[str] = set()
    seen_impls: set[str] = set()
    errors: list[str] = []

    for idx, row in enumerate(rows, start=1):
        rid = row_id(row, idx)
        profiles = row_profiles(row, rid)
        if CLOSEOUT_PROFILE not in profiles:
            continue

        cmd = row_cmd(row, rid)
        if not (
            len(cmd) == 2
            and cmd[0] == "bash"
            and cmd[1].startswith(IMPL_PREFIX)
            and cmd[1].endswith("_closeout_guard.sh")
        ):
            errors.append(
                f"{rid}: closeout row cmd must be "
                "['bash', 'tools/checks/impl/*_closeout_guard.sh']"
            )
            continue

        impl_path = cmd[1]
        wrapper_path = WRAPPER_PREFIX + PurePosixPath(impl_path).name
        if wrapper_path in seen_wrappers:
            errors.append(f"{rid}: duplicate public wrapper path: {wrapper_path}")
        if impl_path in seen_impls:
            errors.append(f"{rid}: duplicate implementation path: {impl_path}")
        seen_wrappers.add(wrapper_path)
        seen_impls.add(impl_path)

        wrapper = root / wrapper_path
        impl = root / impl_path
        if not wrapper.is_file():
            errors.append(f"{rid}: wrapper missing: {wrapper_path}")
        if not impl.is_file():
            errors.append(f"{rid}: implementation command missing: {impl_path}")
        if wrapper.is_file() and not os.access(wrapper, os.X_OK):
            errors.append(f"{rid}: wrapper is not executable: {wrapper_path}")
        if impl.is_file() and not os.access(impl, os.X_OK):
            errors.append(f"{rid}: implementation command is not executable: {impl_path}")

        expected[rid] = {"wrapper": wrapper_path, "impl": impl_path}

    if not expected:
        errors.append(f"manifest profile has no rows: {CLOSEOUT_PROFILE}")
    return expected, errors


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", default=".", help="repository root")
    parser.add_argument("--min-guard-rows", type=int, default=0)
    parser.add_argument("--min-impl-files", type=int, default=0)
    parser.add_argument("--min-public-k2-wide", type=int, default=0)
    parser.add_argument(
        "--require-hako-alloc-closeout-covered",
        action="store_true",
        help="fail if public hako_alloc closeout wrappers are not manifest-backed",
    )
    args = parser.parse_args()

    root = Path(args.root).resolve()
    manifest = root / "tools/checks/guard_rows.toml"
    if not manifest.is_file():
        fail(f"required file missing: {manifest.relative_to(root)}")

    rows = load_rows(manifest)
    impl_files = sorted((root / "tools/checks/impl").glob("*.sh"))
    public_k2_wide = sorted((root / "tools/checks").glob("k2_wide_*.sh"))
    top_level_check_sh = sorted((root / "tools/checks").glob("*.sh"))
    closeout_expected, closeout_errors = collect_closeout_manifest(root, rows)

    public_closeout_wrappers = {
        str(path.relative_to(root))
        for path in (root / "tools/checks").glob(PUBLIC_CLOSEOUT_GLOB)
    }
    expected_closeout_wrappers = {spec["wrapper"] for spec in closeout_expected.values()}
    non_manifest_closeout = sorted(public_closeout_wrappers - expected_closeout_wrappers)
    missing_closeout = sorted(expected_closeout_wrappers - public_closeout_wrappers)

    errors = list(closeout_errors)
    if len(rows) < args.min_guard_rows:
        errors.append(f"guard_rows below minimum: {len(rows)} < {args.min_guard_rows}")
    if len(impl_files) < args.min_impl_files:
        errors.append(f"impl .sh files below minimum: {len(impl_files)} < {args.min_impl_files}")
    if len(public_k2_wide) < args.min_public_k2_wide:
        errors.append(
            f"public k2_wide guards below minimum: {len(public_k2_wide)} < {args.min_public_k2_wide}"
        )
    if args.require_hako_alloc_closeout_covered:
        for wrapper in non_manifest_closeout:
            errors.append(f"public hako_alloc closeout wrapper is not manifest-backed: {wrapper}")
        for wrapper in missing_closeout:
            errors.append(f"manifest hako_alloc closeout wrapper missing: {wrapper}")

    rows_by_profile: dict[str, int] = {}
    for idx, row in enumerate(rows, start=1):
        rid = row_id(row, idx)
        for profile in row_profiles(row, rid):
            rows_by_profile[profile] = rows_by_profile.get(profile, 0) + 1

    output = {
        "guard_rows": len(rows),
        "top_level_check_sh": len(top_level_check_sh),
        "public_k2_wide": len(public_k2_wide),
        "impl_sh": len(impl_files),
        "hako_alloc_closeout_rows": len(closeout_expected),
        "manifest_backed_hako_alloc_closeout_wrappers": len(expected_closeout_wrappers),
        "non_manifest_hako_alloc_closeout_wrappers": len(non_manifest_closeout),
        "missing_manifest_hako_alloc_closeout_wrappers": len(missing_closeout),
    }
    for key, value in sorted(rows_by_profile.items()):
        output[f"profile_{key}_rows"] = value

    for key, value in output.items():
        print(f"{key}={value}")

    if errors:
        for error in errors:
            print(f"[{TAG}] ERROR: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
