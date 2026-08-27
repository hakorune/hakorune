#!/usr/bin/env python3
"""Inventory guard manifest migration state without running guards."""

from __future__ import annotations

import argparse
import os
from pathlib import Path, PurePosixPath
import re
import subprocess
import sys
import tomllib
from typing import Any


TAG = "guard-manifest-inventory"
CLOSEOUT_PROFILE = "hako-alloc-closeout"
IMPL_PREFIX = "tools/checks/impl/"
WRAPPER_PREFIX = "tools/checks/"
PUBLIC_CLOSEOUT_GLOB = "k2_wide_hako_alloc_*closeout_guard.sh"
INDEX_PATH = "docs/tools/check-scripts-index.md"
TOMBSTONE_MANIFEST_PATH = "tools/checks/manifests/guard_navigation_tombstones.toml"
COMPAT_BEGIN = "<!-- legacy-guard-name-compat-begin"
COMPAT_END = "legacy-guard-name-compat-end -->"


def fail(message: str) -> None:
    print(f"[{TAG}] ERROR: {message}", file=sys.stderr)
    raise SystemExit(1)


def load_rows(root: Path, manifest: Path, stack: tuple[Path, ...] = ()) -> list[dict[str, Any]]:
    if manifest in stack:
        cycle = " -> ".join(str(path) for path in (*stack, manifest))
        fail(f"guard_rows.toml include cycle: {cycle}")

    data = tomllib.loads(manifest.read_text(encoding="utf-8"))
    includes = data.get("includes", [])
    if not isinstance(includes, list) or not all(isinstance(item, str) and item for item in includes):
        fail(f"{manifest} includes must be a list of non-empty strings")

    result: list[dict[str, Any]] = []
    for include in includes:
        include_path = root / include
        if not include_path.is_file():
            fail(f"missing included manifest: {include}")
        result.extend(load_rows(root, include_path, (*stack, manifest)))

    rows = data.get("rows")
    if not isinstance(rows, list):
        fail("guard_rows.toml must contain [[rows]] entries")
    for idx, row in enumerate(rows, start=1):
        if not isinstance(row, dict):
            fail(f"row {idx} must be a table")
        result.append(row)
    return result


def git_blob(root: Path, revision: str, path: str) -> str:
    if not revision or revision.startswith("-"):
        fail("registry ratchet base must be an explicit revision")
    result = subprocess.run(
        ["git", "show", f"{revision}:{path}"],
        cwd=root,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        detail = result.stderr.strip() or "unknown git error"
        fail(f"cannot read {path} at base {revision}: {detail}")
    return result.stdout


def load_rows_at_revision(
    root: Path,
    revision: str,
    manifest: str,
    stack: tuple[str, ...] = (),
) -> list[dict[str, Any]]:
    if manifest in stack:
        cycle = " -> ".join((*stack, manifest))
        fail(f"base manifest include cycle: {cycle}")

    try:
        data = tomllib.loads(git_blob(root, revision, manifest))
    except tomllib.TOMLDecodeError as exc:
        fail(f"base manifest parse failed: {revision}:{manifest}: {exc}")

    includes = data.get("includes", [])
    if not isinstance(includes, list) or not all(isinstance(item, str) and item for item in includes):
        fail(f"base manifest includes must be a list: {revision}:{manifest}")

    result: list[dict[str, Any]] = []
    for include in includes:
        include_path = PurePosixPath(include)
        if include_path.is_absolute() or ".." in include_path.parts:
            fail(f"base manifest include escapes repository: {revision}:{manifest}: {include}")
        result.extend(load_rows_at_revision(root, revision, include_path.as_posix(), (*stack, manifest)))

    rows = data.get("rows")
    if not isinstance(rows, list):
        fail(f"base manifest must contain [[rows]] entries: {revision}:{manifest}")
    for idx, row in enumerate(rows, start=1):
        if not isinstance(row, dict):
            fail(f"base row {idx} must be a table: {revision}:{manifest}")
        result.append(row)
    return result


def git_tracked_paths(root: Path, revision: str | None = None) -> set[str]:
    if revision is None:
        command = ["git", "ls-files", "-z", "--", "tools/checks"]
    else:
        command = ["git", "ls-tree", "-r", "-z", "--name-only", revision, "--", "tools/checks"]
    result = subprocess.run(command, cwd=root, capture_output=True)
    if result.returncode != 0:
        detail = result.stderr.decode(errors="replace").strip() or "unknown git error"
        fail(f"cannot enumerate tracked paths: {detail}")
    return {
        item.decode(errors="strict")
        for item in result.stdout.split(b"\0")
        if item
    }


def eligible_public_guards(paths: set[str]) -> set[str]:
    parent = PurePosixPath("tools/checks")
    return {
        path
        for path in paths
        if PurePosixPath(path).parent == parent
        and PurePosixPath(path).name.endswith("_guard.sh")
    }


def command_target_text(command: list[str]) -> str | None:
    if not command:
        return None
    if command[0] in {"bash", "sh", "python", "python3"}:
        for argument in command[1:]:
            if not argument.startswith("-"):
                return argument
        return None
    return command[0]


def normalized_repo_path(value: str) -> str | None:
    path = PurePosixPath(value)
    if path.is_absolute() or ".." in path.parts:
        return None
    return path.as_posix()


def validate_graph_rows(rows: list[dict[str, Any]]) -> None:
    seen_ids: set[str] = set()
    seen_rows: list[dict[str, Any]] = []
    for idx, row in enumerate(rows, start=1):
        rid = row_id(row, idx)
        if rid in seen_ids:
            fail(f"duplicate flattened row id: {rid}")
        if row in seen_rows:
            fail(f"duplicate flattened row: {rid}")
        seen_ids.add(rid)
        seen_rows.append(row)


def graph_edges(
    root: Path,
    rows: list[dict[str, Any]],
    tracked: set[str],
    *,
    validate_current_paths: bool,
) -> tuple[set[str], set[str], list[str]]:
    direct: set[str] = set()
    aliases: set[str] = set()
    errors: list[str] = []

    for idx, row in enumerate(rows, start=1):
        rid = row_id(row, idx)
        profiles = row_profiles(row, rid)
        command = row_cmd(row, rid)
        target = command_target_text(command)
        if target is None:
            errors.append(f"{rid}: command has no target")
        elif validate_current_paths:
            target_path = Path(target)
            if not target_path.is_absolute() and not (root / target_path).exists():
                errors.append(f"{rid}: command target missing: {target}")

        if target is not None:
            normalized = normalized_repo_path(target)
            if normalized in eligible_public_guards(tracked):
                direct.add(normalized)

        if CLOSEOUT_PROFILE not in profiles:
            continue
        if not (
            len(command) == 2
            and command[0] == "bash"
            and command[1].startswith(IMPL_PREFIX)
            and command[1].endswith("_closeout_guard.sh")
        ):
            errors.append(f"{rid}: closeout row has invalid implementation command")
            continue
        wrapper = "tools/checks/" + PurePosixPath(command[1]).name
        if validate_current_paths and not (root / wrapper).is_file():
            errors.append(f"{rid}: derived closeout wrapper missing: {wrapper}")
        if wrapper in eligible_public_guards(tracked):
            aliases.add(wrapper)

    return direct, aliases, errors


def compatibility_names(root: Path) -> tuple[list[str], list[str]]:
    index = root / INDEX_PATH
    if not index.is_file():
        return [], [f"navigation index missing: {INDEX_PATH}"]
    text = index.read_text(encoding="utf-8")
    if text.count(COMPAT_BEGIN) != 1 or text.count(COMPAT_END) != 1:
        return [], [
            "navigation compatibility block must have exactly one begin/end marker"
        ]
    begin = text.index(COMPAT_BEGIN) + len(COMPAT_BEGIN)
    end = text.index(COMPAT_END)
    if begin >= end:
        return [], ["navigation compatibility block markers are reversed"]

    names: list[str] = []
    errors: list[str] = []
    for raw in re.findall(r"tools/checks/[^\s,]+", text[begin:end]):
        token = raw.rstrip("`.;:)]}")
        if "*" in token:
            continue
        if not token.startswith("tools/checks/") or normalized_repo_path(token) != token:
            errors.append(f"invalid navigation compatibility token: {raw}")
            continue
        if token in names:
            errors.append(f"duplicate navigation compatibility name: {token}")
            continue
        names.append(token)
    return names, errors


def git_revision_exists(root: Path, revision: str) -> bool:
    if not revision or revision.startswith("-"):
        return False
    result = subprocess.run(
        ["git", "rev-parse", "--verify", f"{revision}^{{commit}}"],
        cwd=root,
        capture_output=True,
        text=True,
    )
    return result.returncode == 0


def load_navigation_tombstones(root: Path) -> tuple[list[dict[str, str]], list[str]]:
    path = root / TOMBSTONE_MANIFEST_PATH
    if not path.is_file():
        return [], [f"navigation tombstone manifest missing: {TOMBSTONE_MANIFEST_PATH}"]
    try:
        data = tomllib.loads(path.read_text(encoding="utf-8"))
    except tomllib.TOMLDecodeError as exc:
        return [], [f"navigation tombstone manifest parse failed: {exc}"]
    if data.get("schema_version") != 0:
        return [], ["navigation tombstone manifest schema_version must be 0"]
    rows = data.get("tombstones")
    if not isinstance(rows, list):
        return [], ["navigation tombstone manifest must contain [[tombstones]] rows"]

    required = (
        "path",
        "owner",
        "disposition",
        "reason",
        "observed_revision",
        "successor",
        "reopen_trigger",
    )
    result: list[dict[str, str]] = []
    errors: list[str] = []
    seen: set[str] = set()
    for idx, row in enumerate(rows, start=1):
        if not isinstance(row, dict):
            errors.append(f"navigation tombstone row {idx} must be a table")
            continue
        values: dict[str, str] = {}
        for field in required:
            value = row.get(field)
            if not isinstance(value, str) or not value.strip():
                errors.append(f"navigation tombstone row {idx} missing {field}")
                continue
            values[field] = value
        if len(values) != len(required):
            continue
        tombstone_path = values["path"]
        if not tombstone_path.startswith("tools/checks/") or normalized_repo_path(tombstone_path) != tombstone_path:
            errors.append(f"navigation tombstone row {idx} has invalid path: {tombstone_path}")
        if tombstone_path in seen:
            errors.append(f"duplicate navigation tombstone path: {tombstone_path}")
        seen.add(tombstone_path)
        if values["disposition"] != "superseded":
            errors.append(
                f"navigation tombstone row {idx} disposition must be superseded"
            )
        result.append(values)
    return result, errors


def validate_navigation(
    root: Path,
    tracked: set[str],
) -> tuple[dict[str, int], list[str]]:
    names, errors = compatibility_names(root)
    tombstones, tombstone_errors = load_navigation_tombstones(root)
    errors.extend(tombstone_errors)
    names_set = set(names)
    tombstone_by_path: dict[str, dict[str, str]] = {}
    for tombstone in tombstones:
        path = tombstone["path"]
        if path in tombstone_by_path:
            continue
        tombstone_by_path[path] = tombstone
        if path not in names_set:
            errors.append(f"tombstone path is not in compatibility index: {path}")
        if path in tracked:
            errors.append(f"executable caller reappeared for tombstoned path: {path}")
        if not git_revision_exists(root, tombstone["observed_revision"]):
            errors.append(
                f"tombstone observed revision is not a commit: {path}: "
                f"{tombstone['observed_revision']}"
            )
        successor = tombstone["successor"]
        if successor not in tracked:
            errors.append(f"tombstone successor is not tracked: {path}: {successor}")
        if successor in tombstone_by_path:
            errors.append(f"tombstone successor is also tombstoned: {path}: {successor}")

    live = {name for name in names if name in tracked}
    tombstoned = {
        name for name in names if name not in tracked and name in tombstone_by_path
    }
    dangling = {
        name for name in names if name not in tracked and name not in tombstone_by_path
    }
    for name in names:
        if name in tracked and name in tombstone_by_path:
            errors.append(f"live compatibility path has tombstone: {name}")
    return {
        "navigation_compatibility_paths": len(names),
        "navigation_live_paths": len(live),
        "navigation_tombstoned_paths": len(tombstoned),
        "navigation_dangling_paths": len(dangling),
        "navigation_tombstone_rows": len(tombstones),
    }, errors


def run_registry_ratchet(root: Path, manifest: Path, base: str | None) -> int:
    rows = load_rows(root, manifest)
    validate_graph_rows(rows)
    current_tracked = git_tracked_paths(root)
    current_eligible = eligible_public_guards(current_tracked)
    current_direct, current_aliases, errors = graph_edges(
        root,
        rows,
        current_tracked,
        validate_current_paths=True,
    )
    navigation_output, navigation_errors = validate_navigation(root, current_tracked)
    errors.extend(navigation_errors)
    current_mapped = current_direct | current_aliases
    current_unmapped = current_eligible - current_mapped

    base_eligible: set[str] = set()
    base_mapped: set[str] = set()
    base_unmapped: set[str] = set()
    new_unmapped: set[str] = set()
    mapping_loss: set[str] = set()
    mode = "absolute"
    if base:
        mode = "comparative"
        base_rows = load_rows_at_revision(root, base, "tools/checks/guard_rows.toml")
        validate_graph_rows(base_rows)
        base_tracked = git_tracked_paths(root, base)
        base_eligible = eligible_public_guards(base_tracked)
        base_direct, base_aliases, base_errors = graph_edges(
            root,
            base_rows,
            base_tracked,
            validate_current_paths=False,
        )
        errors.extend(f"base: {error}" for error in base_errors)
        base_mapped = base_direct | base_aliases
        base_unmapped = base_eligible - base_mapped
        new_unmapped = current_unmapped - base_unmapped
        mapping_loss = (base_mapped & base_eligible & current_eligible) - current_mapped

    output = {
        **navigation_output,
        "registry_mode": mode,
        "registry_base": base or "none",
        "registry_current_eligible": len(current_eligible),
        "registry_current_direct_command_targets": len(current_direct),
        "registry_current_typed_wrapper_aliases": len(current_aliases),
        "registry_current_mapped": len(current_mapped),
        "registry_current_unmapped": len(current_unmapped),
        "registry_base_eligible": len(base_eligible),
        "registry_base_mapped": len(base_mapped),
        "registry_base_unmapped": len(base_unmapped),
        "registry_new_unmapped": len(new_unmapped),
        "registry_mapping_loss": len(mapping_loss),
    }
    for key, value in output.items():
        print(f"{key}={value}")
    if new_unmapped:
        errors.append("new unmapped eligible guards: " + ", ".join(sorted(new_unmapped)))
    if mapping_loss:
        errors.append("eligible guards lost their mapping: " + ", ".join(sorted(mapping_loss)))
    if errors:
        for error in errors:
            print(f"[{TAG}] ERROR: {error}", file=sys.stderr)
        return 1
    return 0


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
    parser.add_argument(
        "--registry-ratchet",
        action="store_true",
        help="derive the finite public guard graph and compare it with an explicit base",
    )
    parser.add_argument(
        "--base",
        help="explicit git revision for comparative registry-ratchet mode",
    )
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

    if args.base and not args.registry_ratchet:
        fail("--base requires --registry-ratchet")
    if args.registry_ratchet:
        return run_registry_ratchet(root, manifest, args.base)

    rows = load_rows(root, manifest)
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
