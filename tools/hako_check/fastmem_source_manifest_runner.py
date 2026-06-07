#!/usr/bin/env python3
"""Run a compact manifest of fastmem source-syntax fixtures.

This seed runner keeps the existing shell smoke intact while letting new
hako_alloc fastmem bodies move to manifest-driven fixtures one at a time.
"""

from __future__ import annotations

import argparse
import os
import subprocess
import sys
import tempfile
import tomllib
from pathlib import Path

from report_kv import expected_kv_mismatches, format_expected_kv_mismatches, read_expected_kv


def fail(message: str, code: int = 2) -> None:
    print(f"[fastmem/source-manifest] ERROR: {message}", file=sys.stderr)
    raise SystemExit(code)


def repo_root() -> Path:
    return Path(__file__).resolve().parents[2]


def resolve_path(root: Path, raw: str) -> Path:
    path = Path(raw)
    if path.is_absolute():
        return path
    return root / path


def find_binary(root: Path) -> Path:
    candidates = []
    env = os.environ.get("NYASH_BIN")
    if env:
        candidates.append(Path(env))
    candidates.extend([root / "target/release/hakorune", root / "target/release/nyash"])
    for candidate in candidates:
        if candidate.exists() and candidate.is_file() and os.access(candidate, os.X_OK):
            return candidate
    fail(f"hakorune/nyash binary not found in {', '.join(str(path) for path in candidates)}")
    raise AssertionError("unreachable")


def load_manifest(path: Path) -> list[dict[str, object]]:
    if not path.is_file():
        fail(f"manifest missing: {path}")
    try:
        data = tomllib.loads(path.read_text(encoding="utf-8"))
    except tomllib.TOMLDecodeError as exc:
        fail(f"manifest parse failed: {path}: {exc}")
    if data.get("schema_version") != 0:
        fail(f"manifest schema_version must be 0: {path}")
    fixtures = data.get("fixtures", [])
    if not isinstance(fixtures, list) or not fixtures:
        fail(f"manifest must contain at least one [[fixtures]] entry: {path}")

    normalized: list[dict[str, object]] = []
    seen: set[str] = set()
    for index, raw_entry in enumerate(fixtures):
        if not isinstance(raw_entry, dict):
            fail(f"fixtures #{index} must be a table: {path}")
        fixture_id = raw_entry.get("id")
        label = raw_entry.get("label")
        source = raw_entry.get("source")
        if not isinstance(fixture_id, str) or not fixture_id:
            fail(f"fixtures #{index} id must be a non-empty string")
        if fixture_id in seen:
            fail(f"duplicate fixture id: {fixture_id}")
        seen.add(fixture_id)
        if not isinstance(label, str) or not label:
            fail(f"fixtures {fixture_id} label must be a non-empty string")
        if not isinstance(source, str) or not source:
            fail(f"fixtures {fixture_id} source must be a non-empty string")

        producers = raw_entry.get("producers", [])
        if not isinstance(producers, list) or not producers:
            fail(f"fixtures {fixture_id} must contain at least one [[fixtures.producers]] entry")
        normalized_producers: list[dict[str, object]] = []
        for p_index, producer in enumerate(producers):
            if not isinstance(producer, dict):
                fail(f"fixtures {fixture_id} producer #{p_index} must be a table")
            profile = producer.get("profile")
            report_expected = producer.get("report_expected")
            check_expected = producer.get("check_expected")
            expect_failure = bool(producer.get("expect_failure", False))
            check_expect_failure = bool(producer.get("check_expect_failure", False))
            stderr_expected = producer.get("stderr_expected")
            if not isinstance(profile, str) or not profile:
                fail(f"fixtures {fixture_id} producer #{p_index} profile must be a non-empty string")
            entry: dict[str, object] = {"profile": profile, "expect_failure": expect_failure}
            if check_expect_failure:
                if expect_failure:
                    fail(
                        f"fixtures {fixture_id} producer {profile} cannot set both expect_failure and check_expect_failure"
                    )
                entry["check_expect_failure"] = check_expect_failure
            if expect_failure:
                if not isinstance(stderr_expected, str) or not stderr_expected:
                    fail(
                        f"fixtures {fixture_id} producer {profile} expect_failure requires stderr_expected"
                    )
                entry["stderr_expected"] = stderr_expected
            else:
                if not isinstance(report_expected, str) or not report_expected:
                    fail(f"fixtures {fixture_id} producer {profile} report_expected must be a non-empty string")
                if not isinstance(check_expected, str) or not check_expected:
                    fail(f"fixtures {fixture_id} producer {profile} check_expected must be a non-empty string")
                entry["report_expected"] = report_expected
                entry["check_expected"] = check_expected
            normalized_producers.append(entry)  # type: ignore[arg-type]

        entry: dict[str, object] = {
            "id": fixture_id,
            "label": label,
            "source": source,
            "features": raw_entry.get("features", os.environ.get("FASTMEM_SOURCE_FEATURES", "stage3,rune")),
            "ast_expected": raw_entry.get("ast_expected"),
            "mir_expected": raw_entry.get("mir_expected"),
            "producers": normalized_producers,
        }
        for field in ("ast_expected", "mir_expected"):
            value = entry[field]
            if not isinstance(value, str) or not value:
                fail(f"fixtures {fixture_id} {field} must be a non-empty string")
        normalized.append(entry)
    return normalized


def run_command(cmd: list[str], *, cwd: Path, env: dict[str, str]) -> None:
    result = subprocess.run(cmd, cwd=cwd, env=env)
    if result.returncode != 0:
        fail(f"command failed ({result.returncode}): {' '.join(cmd)}", code=result.returncode)


def compare_expected_kv(actual_path: Path, expected_path: Path, tag: str, label: str) -> None:
    actual = read_expected_kv(actual_path)
    expected = read_expected_kv(expected_path)
    mismatches = expected_kv_mismatches(actual, expected)
    if mismatches:
        print(f"[{tag}] {label} mismatch:", file=sys.stderr)
        for line in format_expected_kv_mismatches(mismatches):
            print(f"[{tag}]   {line}", file=sys.stderr)
        raise SystemExit(1)


def read_expected_lines(path: Path) -> list[str]:
    if not path.is_file():
        fail(f"expected text missing: {path}")
    lines: list[str] = []
    for raw_line in path.read_text(encoding="utf-8").splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#"):
            continue
        lines.append(line)
    return lines


def compare_expected_text(actual_path: Path, expected_path: Path, tag: str, label: str) -> None:
    actual = actual_path.read_text(encoding="utf-8", errors="replace")
    expected_lines = read_expected_lines(expected_path)
    missing = [line for line in expected_lines if line not in actual]
    if missing:
        print(f"[{tag}] {label} mismatch:", file=sys.stderr)
        for line in missing:
            print(f"[{tag}]   missing substring: {line}", file=sys.stderr)
        raise SystemExit(1)


def run_fixture(root: Path, bin_path: Path, fixture: dict[str, object], tag: str) -> None:
    fixture_id = str(fixture["id"])
    label = str(fixture["label"])
    source = resolve_path(root, str(fixture["source"]))
    features = str(fixture["features"])
    ast_expected = resolve_path(root, str(fixture["ast_expected"]))
    mir_expected = resolve_path(root, str(fixture["mir_expected"]))
    producers = list(fixture["producers"])

    if not source.is_file():
        fail(f"fixture {fixture_id} source missing: {source}")
    if not ast_expected.is_file():
        fail(f"fixture {fixture_id} ast_expected missing: {ast_expected}")
    if not mir_expected.is_file():
        fail(f"fixture {fixture_id} mir_expected missing: {mir_expected}")

    with tempfile.TemporaryDirectory(prefix="hako_fastmem_source_manifest_") as tmp:
        tmpdir = Path(tmp)
        ast_json = tmpdir / f"{fixture_id}.ast.json"
        mir_json = tmpdir / f"{fixture_id}.mir.json"
        ast_inventory = tmpdir / f"{fixture_id}.ast.inventory.kv"
        mir_inventory = tmpdir / f"{fixture_id}.mir.inventory.kv"

        env = os.environ.copy()
        env["NYASH_FEATURES"] = features

        print(f"[{tag}] >>> {fixture_id} :: {label}")
        run_command([str(bin_path), "--emit-ast-json", str(ast_json), str(source)], cwd=root, env=env)
        run_command([str(bin_path), "--backend", "mir", "--emit-mir-json", str(mir_json), str(source)], cwd=root, env=env)

        run_command(
            ["bash", str(root / "tools/hako_check.sh"), "fastmem-capability-inventory", "--ast-json", str(ast_json), "--out", str(ast_inventory)],
            cwd=root,
            env=env,
        )
        compare_expected_kv(ast_inventory, ast_expected, tag, f"{fixture_id} ast inventory")

        run_command(
            ["bash", str(root / "tools/hako_check.sh"), "fastmem-capability-inventory", "--mir-json", str(mir_json), "--out", str(mir_inventory)],
            cwd=root,
            env=env,
        )
        compare_expected_kv(mir_inventory, mir_expected, tag, f"{fixture_id} mir inventory")

        for producer in producers:
            profile = producer["profile"]
            expect_failure = bool(producer.get("expect_failure", False))
            check_expect_failure = bool(producer.get("check_expect_failure", False))
            report_path = tmpdir / f"{fixture_id}.{profile}.report.kv"
            check_path = tmpdir / f"{fixture_id}.{profile}.check.kv"
            if expect_failure:
                stderr_expected = producer.get("stderr_expected")
                if not isinstance(stderr_expected, str) or not stderr_expected:
                    fail(f"fixture {fixture_id} producer {profile} expect_failure requires stderr_expected")
                stderr_path = tmpdir / f"{fixture_id}.{profile}.stderr"
                cmd = [
                    "bash",
                    str(root / "tools/hako_check.sh"),
                    "fastmem-mir-to-llvm-producer-report",
                    "--profile",
                    profile,
                    "--mir-json",
                    str(mir_json),
                    "--out",
                    str(report_path),
                ]
                with stderr_path.open("w", encoding="utf-8") as stderr_file:
                    result = subprocess.run(cmd, cwd=root, env=env, stdout=subprocess.DEVNULL, stderr=stderr_file)
                if result.returncode == 0:
                    fail(f"fixture {fixture_id} producer {profile} expected failure but succeeded")
                compare_expected_text(stderr_path, resolve_path(root, stderr_expected), tag, f"{fixture_id} {profile} stderr")
                continue

            report_expected = producer.get("report_expected")
            check_expected = producer.get("check_expected")
            if not isinstance(report_expected, str) or not report_expected:
                fail(f"fixture {fixture_id} producer {profile} report_expected must be set for success fixtures")
            if not isinstance(check_expected, str) or not check_expected:
                fail(f"fixture {fixture_id} producer {profile} check_expected must be set for success fixtures")
            report_expected_path = resolve_path(root, report_expected)
            check_expected_path = resolve_path(root, check_expected)
            if not report_expected_path.is_file():
                fail(f"fixture {fixture_id} report_expected missing for {profile}: {report_expected_path}")
            if not check_expected_path.is_file():
                fail(f"fixture {fixture_id} check_expected missing for {profile}: {check_expected_path}")

            run_command(
                [
                    "bash",
                    str(root / "tools/hako_check.sh"),
                    "fastmem-mir-to-llvm-producer-report",
                    "--profile",
                    profile,
                    "--mir-json",
                    str(mir_json),
                    "--out",
                    str(report_path),
                ],
                cwd=root,
                env=env,
            )
            compare_expected_kv(report_path, report_expected_path, tag, f"{fixture_id} {profile} report")

            check_result = subprocess.run(
                [
                    "bash",
                    str(root / "tools/hako_check.sh"),
                    "fastmem-check",
                    "--inventory",
                    str(report_path),
                    "--format",
                    "kv",
                    "--out",
                    str(check_path),
                ],
                cwd=root,
                env=env,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
            )
            if check_expect_failure:
                if check_result.returncode == 0:
                    fail(
                        f"fixture {fixture_id} producer {profile} expected fastmem-check failure but succeeded"
                    )
            elif check_result.returncode != 0:
                fail(
                    f"command failed ({check_result.returncode}): bash {root / 'tools/hako_check.sh'} fastmem-check --inventory {report_path} --format kv --out {check_path}",
                    code=check_result.returncode,
                )
            compare_expected_kv(check_path, check_expected_path, tag, f"{fixture_id} {profile} check")


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", required=True, help="manifest TOML path")
    parser.add_argument("--root", help="repository root override")
    parser.add_argument("--only", help="comma-separated fixture ids")
    parser.add_argument("--tag", default="fastmem/source-manifest", help="log tag")
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(sys.argv[1:] if argv is None else argv)
    root = Path(args.root).resolve() if args.root else repo_root()
    manifest_path = resolve_path(root, args.manifest)
    fixtures = load_manifest(manifest_path)
    requested = [item.strip() for item in args.only.split(",") if item.strip()] if args.only else []
    if requested:
        by_id = {str(item["id"]): item for item in fixtures}
        missing = [fixture_id for fixture_id in requested if fixture_id not in by_id]
        if missing:
            fail(f"unknown fixture id(s): {', '.join(missing)}")
        fixtures = [by_id[fixture_id] for fixture_id in requested]

    bin_path = find_binary(root)
    for fixture in fixtures:
        run_fixture(root, bin_path, fixture, args.tag)
    print(f"[{args.tag}] ok fixtures={len(fixtures)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
