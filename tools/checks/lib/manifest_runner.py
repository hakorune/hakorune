#!/usr/bin/env python3
"""Shared manifest-backed guard/proof runner.

The shell entrypoints stay as stable human-facing commands. This module owns the
common TOML parsing and argv-array subprocess dispatch so row/proof runners do
not drift.
"""

from __future__ import annotations

import argparse
import pathlib
import subprocess
import sys
import tomllib
from collections.abc import Sequence


def fail(tag: str, message: str, code: int = 2) -> None:
    print(f"[{tag}] ERROR: {message}", file=sys.stderr)
    raise SystemExit(code)


def parse_args(argv: Sequence[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Run manifest-backed guard commands",
    )
    parser.add_argument("--root", required=True, help="repository root")
    parser.add_argument("--manifest", required=True, help="manifest path")
    parser.add_argument("--table", required=True, help="TOML array table name")
    parser.add_argument("--tag", required=True, help="output/error tag")
    parser.add_argument("--item-name", required=True, help="human-readable item name")
    parser.add_argument("--app-key", help="optional app directory field to validate")
    parser.add_argument("--profile", help="run all entries tagged with this profile")
    parser.add_argument("--validation-profile", help="run entries with this validation_profile field")
    parser.add_argument("--row-kind", help="run entries with this row_kind field")
    parser.add_argument("--closeout-pack", help="run entries assigned to this closeout_pack field")
    parser.add_argument("--only", help="comma-separated entry ids to run")
    parser.add_argument("--list", action="store_true", help="list entries instead of running")
    parser.add_argument("--dry-run", action="store_true", help="print selected entries without running")
    parser.add_argument("--allow-positional", action="store_true", help=argparse.SUPPRESS)
    parser.add_argument("ids", nargs="*", help="entry ids to run")
    return parser.parse_args(argv)


def as_nonempty_string(value: object, label: str, tag: str) -> str:
    if not isinstance(value, str) or not value:
        fail(tag, f"{label} must be a non-empty string")
    return value


def as_string_list(value: object, label: str, tag: str, *, nonempty: bool) -> list[str]:
    if not isinstance(value, list) or not all(isinstance(item, str) and item for item in value):
        fail(tag, f"{label} must be a list of non-empty strings")
    if nonempty and not value:
        fail(tag, f"{label} must not be empty")
    return value


def as_optional_string(value: object, label: str, tag: str) -> str | None:
    if value is None:
        return None
    if not isinstance(value, str) or not value:
        fail(tag, f"{label} must be a non-empty string when present")
    return value


def as_optional_bool(value: object, label: str, tag: str) -> bool | None:
    if value is None:
        return None
    if not isinstance(value, bool):
        fail(tag, f"{label} must be a boolean when present")
    return value


def resolve_repo_path(root: pathlib.Path, path_text: str) -> pathlib.Path:
    path = pathlib.Path(path_text)
    if not path.is_absolute():
        path = root / path
    return path


def command_target(root: pathlib.Path, cmd: list[str]) -> pathlib.Path:
    if len(cmd) >= 2 and cmd[0] in {"bash", "sh"}:
        return resolve_repo_path(root, cmd[-1])
    return resolve_repo_path(root, cmd[0])


def load_manifest(args: argparse.Namespace) -> list[dict[str, object]]:
    tag = args.tag
    root = pathlib.Path(args.root).resolve()
    manifest_path = resolve_repo_path(root, args.manifest)
    if not manifest_path.is_file():
        fail(tag, f"manifest missing: {manifest_path}")

    try:
        data = tomllib.loads(manifest_path.read_text(encoding="utf-8"))
    except tomllib.TOMLDecodeError as exc:
        fail(tag, f"manifest parse failed: {manifest_path}: {exc}")

    if data.get("schema_version") != 0:
        fail(tag, "manifest schema_version must be 0")

    raw_entries = data.get(args.table)
    if not isinstance(raw_entries, list):
        fail(tag, f"manifest must contain [[{args.table}]] entries")

    seen: set[str] = set()
    entries: list[dict[str, object]] = []
    for index, raw_entry in enumerate(raw_entries):
        if not isinstance(raw_entry, dict):
            fail(tag, f"{args.item_name} #{index} must be a table")

        entry_id = as_nonempty_string(
            raw_entry.get("id"),
            f"{args.item_name} #{index} id",
            tag,
        )
        if entry_id in seen:
            fail(tag, f"duplicate {args.item_name} id: {entry_id}")
        seen.add(entry_id)

        label = as_nonempty_string(raw_entry.get("label"), f"{args.item_name} {entry_id} label", tag)
        profiles = as_string_list(
            raw_entry.get("profiles"),
            f"{args.item_name} {entry_id} profiles",
            tag,
            nonempty=False,
        )
        cmd = as_string_list(
            raw_entry.get("cmd"),
            f"{args.item_name} {entry_id} cmd",
            tag,
            nonempty=True,
        )

        entry: dict[str, object] = {
            "id": entry_id,
            "label": label,
            "profiles": profiles,
            "cmd": cmd,
        }

        optional_string_fields = [
            "row_kind",
            "validation_profile",
            "closeout_pack",
            "exe",
            "exe_skip_reason",
        ]
        for field in optional_string_fields:
            value = as_optional_string(
                raw_entry.get(field),
                f"{args.item_name} {entry_id} {field}",
                tag,
            )
            if value is not None:
                entry[field] = value

        first_pattern = as_optional_bool(
            raw_entry.get("first_pattern"),
            f"{args.item_name} {entry_id} first_pattern",
            tag,
        )
        if first_pattern is not None:
            entry["first_pattern"] = first_pattern

        if args.app_key:
            app = as_nonempty_string(
                raw_entry.get(args.app_key),
                f"{args.item_name} {entry_id} {args.app_key}",
                tag,
            )
            if not resolve_repo_path(root, app).is_dir():
                fail(tag, f"{args.item_name} {entry_id} app directory missing: {app}")
            entry[args.app_key] = app

        target = command_target(root, cmd)
        if not target.exists():
            fail(tag, f"{args.item_name} {entry_id} command target missing: {' '.join(cmd)}")

        entries.append(entry)

    return entries


def requested_ids(args: argparse.Namespace) -> list[str]:
    requested: list[str] = []
    if args.only:
        requested.extend(item.strip() for item in args.only.split(",") if item.strip())
    if args.ids and not args.allow_positional:
        fail(args.tag, f"select {args.item_name} entries with --only; positional ids are disabled")
    requested.extend(args.ids)
    return requested


def select_entries(entries: list[dict[str, object]], args: argparse.Namespace) -> list[dict[str, object]]:
    tag = args.tag
    requested = requested_ids(args)
    if requested:
        by_id = {str(entry["id"]): entry for entry in entries}
        missing = [entry_id for entry_id in requested if entry_id not in by_id]
        if missing:
            fail(tag, f"unknown {args.item_name} id(s): {', '.join(missing)}")
        return [by_id[entry_id] for entry_id in requested]

    if args.profile:
        selected = [entry for entry in entries if args.profile in entry["profiles"]]
        if not selected:
            fail(tag, f"profile has no {args.item_name} entries: {args.profile}")
        return selected

    if args.validation_profile:
        selected = [
            entry
            for entry in entries
            if entry.get("validation_profile") == args.validation_profile
        ]
        if not selected:
            fail(tag, f"validation_profile has no {args.item_name} entries: {args.validation_profile}")
        return selected

    if args.row_kind:
        selected = [entry for entry in entries if entry.get("row_kind") == args.row_kind]
        if not selected:
            fail(tag, f"row_kind has no {args.item_name} entries: {args.row_kind}")
        return selected

    if args.closeout_pack:
        selected = [
            entry
            for entry in entries
            if entry.get("closeout_pack") == args.closeout_pack
        ]
        if not selected:
            fail(tag, f"closeout_pack has no {args.item_name} entries: {args.closeout_pack}")
        return selected

    if args.list:
        return entries

    fail(tag, f"select {args.item_name} entries with ids, --profile, --only, or --list")
    raise AssertionError("unreachable")


def format_entry(entry: dict[str, object], args: argparse.Namespace) -> str:
    profiles = ",".join(entry["profiles"])
    cmd = " ".join(entry["cmd"])
    parts = [
        str(entry["id"]),
        f"profiles={profiles}",
    ]
    if args.app_key:
        parts.append(f"{args.app_key}={entry[args.app_key]}")
    for field in [
        "row_kind",
        "validation_profile",
        "closeout_pack",
        "first_pattern",
        "exe",
        "exe_skip_reason",
    ]:
        if field in entry:
            parts.append(f"{field}={entry[field]}")
    parts.extend([
        f"label={entry['label']}",
        f"cmd={cmd}",
    ])
    return "\t".join(parts)


def main(argv: Sequence[str] | None = None) -> int:
    args = parse_args(sys.argv[1:] if argv is None else argv)
    root = pathlib.Path(args.root).resolve()
    entries = load_manifest(args)
    selected = select_entries(entries, args)

    if args.list:
        for entry in selected:
            print(format_entry(entry, args))
        return 0

    for entry in selected:
        print(f"[{args.tag}] >>> {entry['id']} :: {entry['label']}", flush=True)
        if args.dry_run:
            print(format_entry(entry, args))
            continue
        result = subprocess.run(entry["cmd"], cwd=root)
        if result.returncode != 0:
            print(
                f"[{args.tag}] FAILED {entry['id']} exit={result.returncode}",
                file=sys.stderr,
            )
            return result.returncode

    print(f"[{args.tag}] ok rows={len(selected)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
