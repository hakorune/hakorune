#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

exec python3 - "$ROOT_DIR" "$@" <<'PY'
import argparse
import pathlib
import subprocess
import sys
import tomllib


def fail(message: str, code: int = 2) -> None:
    print(f"[row-guard] ERROR: {message}", file=sys.stderr)
    raise SystemExit(code)


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Run row guard scripts from tools/checks/guard_rows.toml",
    )
    parser.add_argument(
        "--manifest",
        default="tools/checks/guard_rows.toml",
        help="manifest path relative to repo root",
    )
    parser.add_argument(
        "--profile",
        help="run all rows tagged with this profile",
    )
    parser.add_argument(
        "--only",
        help="comma-separated row ids to run",
    )
    parser.add_argument(
        "--list",
        action="store_true",
        help="list rows instead of running them",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="print selected rows without executing them",
    )
    return parser.parse_args(argv)


def load_manifest(root: pathlib.Path, manifest_arg: str) -> list[dict]:
    path = pathlib.Path(manifest_arg)
    if not path.is_absolute():
        path = root / path
    if not path.is_file():
        fail(f"manifest missing: {path}")

    try:
        data = tomllib.loads(path.read_text(encoding="utf-8"))
    except tomllib.TOMLDecodeError as exc:
        fail(f"manifest parse failed: {path}: {exc}")

    if data.get("schema_version") != 0:
        fail("manifest schema_version must be 0")

    rows = data.get("rows")
    if not isinstance(rows, list):
        fail("manifest must contain [[rows]] entries")

    seen: set[str] = set()
    normalized: list[dict] = []
    for index, row in enumerate(rows):
        if not isinstance(row, dict):
            fail(f"row #{index} must be a table")
        row_id = row.get("id")
        label = row.get("label")
        profiles = row.get("profiles")
        cmd = row.get("cmd")

        if not isinstance(row_id, str) or not row_id:
            fail(f"row #{index} has invalid id")
        if row_id in seen:
            fail(f"duplicate row id: {row_id}")
        seen.add(row_id)

        if not isinstance(label, str) or not label:
            fail(f"row {row_id} has invalid label")
        if not isinstance(profiles, list) or not all(isinstance(item, str) for item in profiles):
            fail(f"row {row_id} profiles must be a list of strings")
        if not isinstance(cmd, list) or not cmd or not all(isinstance(item, str) and item for item in cmd):
            fail(f"row {row_id} cmd must be a non-empty argv string list")

        normalized.append({
            "id": row_id,
            "label": label,
            "profiles": profiles,
            "cmd": cmd,
        })
    return normalized


def select_rows(rows: list[dict], args: argparse.Namespace) -> list[dict]:
    if args.only:
        requested = [item.strip() for item in args.only.split(",") if item.strip()]
        if not requested:
            fail("--only did not contain any row ids")
        by_id = {row["id"]: row for row in rows}
        missing = [row_id for row_id in requested if row_id not in by_id]
        if missing:
            fail(f"unknown row id(s): {', '.join(missing)}")
        return [by_id[row_id] for row_id in requested]

    if args.profile:
        selected = [row for row in rows if args.profile in row["profiles"]]
        if not selected:
            fail(f"profile has no rows: {args.profile}")
        return selected

    if args.list:
        return rows

    fail("select rows with --profile, --only, or --list")


def print_row(row: dict) -> None:
    profiles = ",".join(row["profiles"])
    cmd = " ".join(row["cmd"])
    print(f"{row['id']}\tprofiles={profiles}\tlabel={row['label']}\tcmd={cmd}")


def main() -> int:
    if len(sys.argv) < 2:
        fail("internal error: missing repo root")
    root = pathlib.Path(sys.argv[1]).resolve()
    args = parse_args(sys.argv[2:])
    rows = load_manifest(root, args.manifest)
    selected = select_rows(rows, args)

    if args.list:
        for row in selected:
            print_row(row)
        return 0

    for row in selected:
        print(f"[row-guard] >>> {row['id']} :: {row['label']}", flush=True)
        if args.dry_run:
            print_row(row)
            continue
        result = subprocess.run(row["cmd"], cwd=root)
        if result.returncode != 0:
            print(
                f"[row-guard] FAILED {row['id']} exit={result.returncode}",
                file=sys.stderr,
            )
            return result.returncode

    print(f"[row-guard] ok rows={len(selected)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
PY
