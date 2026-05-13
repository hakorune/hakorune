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
    print(f"[proof-app] ERROR: {message}", file=sys.stderr)
    raise SystemExit(code)


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Run proof app guards from tools/checks/proof_apps.toml",
    )
    parser.add_argument("ids", nargs="*", help="proof app ids to run")
    parser.add_argument(
        "--manifest",
        default="tools/checks/proof_apps.toml",
        help="manifest path relative to repo root",
    )
    parser.add_argument("--profile", help="run all proof apps tagged with this profile")
    parser.add_argument("--only", help="comma-separated proof app ids to run")
    parser.add_argument("--list", action="store_true", help="list entries instead of running them")
    parser.add_argument("--dry-run", action="store_true", help="print selected entries without executing them")
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

    entries = data.get("proof_apps")
    if not isinstance(entries, list):
        fail("manifest must contain [[proof_apps]] entries")

    seen: set[str] = set()
    normalized: list[dict] = []
    for index, entry in enumerate(entries):
        if not isinstance(entry, dict):
            fail(f"entry #{index} must be a table")
        proof_id = entry.get("id")
        app = entry.get("app")
        label = entry.get("label")
        profiles = entry.get("profiles")
        cmd = entry.get("cmd")

        if not isinstance(proof_id, str) or not proof_id:
            fail(f"entry #{index} has invalid id")
        if proof_id in seen:
            fail(f"duplicate proof app id: {proof_id}")
        seen.add(proof_id)

        if not isinstance(app, str) or not app:
            fail(f"entry {proof_id} has invalid app")
        if not (root / app).is_dir():
            fail(f"entry {proof_id} app directory missing: {app}")

        if not isinstance(label, str) or not label:
            fail(f"entry {proof_id} has invalid label")
        if not isinstance(profiles, list) or not all(isinstance(item, str) for item in profiles):
            fail(f"entry {proof_id} profiles must be a list of strings")
        if not isinstance(cmd, list) or not cmd or not all(isinstance(item, str) and item for item in cmd):
            fail(f"entry {proof_id} cmd must be a non-empty argv string list")

        executable = root / cmd[-1] if len(cmd) >= 2 and cmd[0] in {"bash", "sh"} else root / cmd[0]
        if not executable.exists():
            fail(f"entry {proof_id} command target missing: {' '.join(cmd)}")

        normalized.append({
            "id": proof_id,
            "app": app,
            "label": label,
            "profiles": profiles,
            "cmd": cmd,
        })
    return normalized


def select_entries(entries: list[dict], args: argparse.Namespace) -> list[dict]:
    requested: list[str] = []
    if args.only:
        requested.extend(item.strip() for item in args.only.split(",") if item.strip())
    requested.extend(args.ids)

    if requested:
        by_id = {entry["id"]: entry for entry in entries}
        missing = [proof_id for proof_id in requested if proof_id not in by_id]
        if missing:
            fail(f"unknown proof app id(s): {', '.join(missing)}")
        return [by_id[proof_id] for proof_id in requested]

    if args.profile:
        selected = [entry for entry in entries if args.profile in entry["profiles"]]
        if not selected:
            fail(f"profile has no proof apps: {args.profile}")
        return selected

    if args.list:
        return entries

    fail("select proof apps with ids, --profile, --only, or --list")


def print_entry(entry: dict) -> None:
    profiles = ",".join(entry["profiles"])
    cmd = " ".join(entry["cmd"])
    print(f"{entry['id']}\tprofiles={profiles}\tapp={entry['app']}\tlabel={entry['label']}\tcmd={cmd}")


def main() -> int:
    if len(sys.argv) < 2:
        fail("internal error: missing repo root")
    root = pathlib.Path(sys.argv[1]).resolve()
    args = parse_args(sys.argv[2:])
    entries = load_manifest(root, args.manifest)
    selected = select_entries(entries, args)

    if args.list:
        for entry in selected:
            print_entry(entry)
        return 0

    for entry in selected:
        print(f"[proof-app] >>> {entry['id']} :: {entry['label']}", flush=True)
        if args.dry_run:
            print_entry(entry)
            continue
        result = subprocess.run(entry["cmd"], cwd=root)
        if result.returncode != 0:
            print(
                f"[proof-app] FAILED {entry['id']} exit={result.returncode}",
                file=sys.stderr,
            )
            return result.returncode

    print(f"[proof-app] ok rows={len(selected)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
PY
