#!/usr/bin/env python3
"""Compare the fixed quick ``cargo test --lib`` red baseline.

This runner is intentionally a verifier, not a re-baseliner.  It observes one
complete Cargo summary and one complete ``--list`` inventory, then compares
their sorted names and hashes with the checked-in receipt.  A known red result
is acceptable only when every field is identical; compile/link/abort and any
name-set drift remain failures.
"""

from __future__ import annotations

import argparse
from dataclasses import dataclass
import hashlib
import os
from pathlib import Path
import re
import subprocess
import sys
import tomllib


DEFAULT_MANIFEST = Path("tools/checks/manifests/cargo_lib_red_baseline.toml")
SUMMARY_RE = re.compile(
    r"test result:\s+(?P<status>ok|FAILED)\.\s+"
    r"(?P<passed>\d+) passed;\s+"
    r"(?P<failed>\d+) failed;\s+"
    r"(?P<ignored>\d+) ignored;\s+"
    r"(?P<measured>\d+) measured;\s+"
    r"(?P<filtered>\d+) filtered out;"
)
FAILURE_RE = re.compile(r"^\s*test\s+(.+?)\s+\.\.\.\s+FAILED\s*$")
LIST_RE = re.compile(r"^\s*(.+?)\s*:\s+test\s*$")


class BaselineError(ValueError):
    """A malformed, incomplete, or drifted baseline observation."""


@dataclass(frozen=True)
class CargoSummary:
    status: str
    passed: int
    failed: int
    ignored: int
    measured: int
    filtered: int

    @property
    def total(self) -> int:
        return self.passed + self.failed + self.ignored + self.measured


@dataclass(frozen=True)
class Observation:
    summary: CargoSummary
    inventory: tuple[str, ...]
    failures: tuple[str, ...]
    exit_code: int


@dataclass(frozen=True)
class BaselineSpec:
    test_command: tuple[str, ...]
    list_command: tuple[str, ...]
    environment: tuple[tuple[str, str], ...]
    expected_status: str
    expected_exit_code: int
    expected_summary: CargoSummary
    inventory_path: str
    inventory_sha256: str
    failures_path: str
    failures_sha256: str


def parse_test_summary(output: str) -> CargoSummary:
    matches = list(SUMMARY_RE.finditer(output))
    if len(matches) != 1:
        raise BaselineError(f"expected exactly one Cargo summary, found {len(matches)}")
    match = matches[0]
    values = {name: int(match.group(name)) for name in (
        "passed", "failed", "ignored", "measured", "filtered"
    )}
    return CargoSummary(match.group("status"), **values)


def parse_failure_names(output: str) -> tuple[str, ...]:
    names = [match.group(1) for line in output.splitlines() if (match := FAILURE_RE.match(line))]
    if len(names) != len(set(names)):
        raise BaselineError("Cargo output contains duplicate failure names")
    return tuple(sorted(names))


def parse_test_output(output: str) -> tuple[CargoSummary, tuple[str, ...]]:
    summary = parse_test_summary(output)
    failures = parse_failure_names(output)
    if len(failures) != summary.failed:
        raise BaselineError(
            f"failure-name count {len(failures)} does not match summary failed={summary.failed}"
        )
    return summary, failures


def parse_test_list(output: str) -> tuple[str, ...]:
    names = [match.group(1) for line in output.splitlines() if (match := LIST_RE.match(line))]
    if not names:
        raise BaselineError("cargo --list produced an empty test inventory")
    if len(names) != len(set(names)):
        raise BaselineError("cargo --list produced duplicate test names")
    return tuple(sorted(names))


def canonical_sha256(lines: tuple[str, ...]) -> str:
    payload = "" if not lines else "\n".join(lines) + "\n"
    return hashlib.sha256(payload.encode("utf-8")).hexdigest()


def read_receipt_lines(path: Path, *, allow_empty: bool) -> tuple[str, ...]:
    if not path.is_file():
        raise BaselineError(f"baseline receipt is missing: {path}")
    lines = tuple(line for line in path.read_text(encoding="utf-8").splitlines() if line.strip())
    if not lines and not allow_empty:
        raise BaselineError(f"baseline receipt is empty: {path}")
    if len(lines) != len(set(lines)):
        raise BaselineError(f"baseline receipt contains duplicate names: {path}")
    if lines != tuple(sorted(lines)):
        raise BaselineError(f"baseline receipt is not sorted: {path}")
    return lines


def _require_int(table: dict, name: str) -> int:
    value = table.get(name)
    if not isinstance(value, int) or value < 0:
        raise BaselineError(f"manifest field {name} must be a non-negative integer")
    return value


def _require_command(table: dict, name: str) -> tuple[str, ...]:
    value = table.get(name)
    if not isinstance(value, list) or not value or not all(
        isinstance(item, str) and item for item in value
    ):
        raise BaselineError(f"manifest field {name} must be a non-empty string list")
    return tuple(value)


def load_manifest(path: Path) -> BaselineSpec:
    try:
        with path.open("rb") as stream:
            table = tomllib.load(stream)
    except (OSError, tomllib.TOMLDecodeError) as exc:
        raise BaselineError(f"cannot load baseline manifest {path}: {exc}") from exc
    if table.get("schema_version") != 1 or table.get("state") != "accepted":
        raise BaselineError("baseline manifest must be schema_version=1 and state=accepted")
    test_command = _require_command(table, "test_command")
    list_command = _require_command(table, "list_command")
    expected_test = ("cargo", "test", "--profile", "quick", "--lib", "--", "--test-threads=1")
    expected_list = ("cargo", "test", "--profile", "quick", "--lib", "--", "--list")
    if test_command != expected_test or list_command != expected_list:
        raise BaselineError("baseline commands are not the fixed quick --lib commands")
    environment = table.get("environment")
    if not isinstance(environment, dict) or set(environment) != {
        "RUST_MIN_STACK", "CARGO_BUILD_JOBS", "CARGO_INCREMENTAL"
    } or not all(isinstance(value, str) and value for value in environment.values()):
        raise BaselineError("baseline environment must fix stack, jobs, and incremental")
    expected_status = table.get("expected_status")
    if expected_status not in {"ok", "FAILED"}:
        raise BaselineError("manifest expected_status must be ok or FAILED")
    expected_summary = CargoSummary(
        expected_status,
        _require_int(table, "expected_passed"),
        _require_int(table, "expected_failed"),
        _require_int(table, "expected_ignored"),
        _require_int(table, "expected_measured"),
        _require_int(table, "expected_filtered"),
    )
    inventory_path = table.get("inventory_file")
    failures_path = table.get("failures_file")
    inventory_sha256 = table.get("inventory_sha256")
    failures_sha256 = table.get("failures_sha256")
    if not all(isinstance(value, str) and value.strip() for value in (
        inventory_path, failures_path, inventory_sha256, failures_sha256
    )):
        raise BaselineError("baseline receipt paths and hashes must be non-empty strings")
    return BaselineSpec(
        test_command,
        list_command,
        tuple(sorted(environment.items())),
        expected_status,
        _require_int(table, "expected_exit_code"),
        expected_summary,
        inventory_path,
        inventory_sha256,
        failures_path,
        failures_sha256,
    )


def compare_observation(
    observation: Observation,
    spec: BaselineSpec,
    expected_inventory: tuple[str, ...],
    expected_failures: tuple[str, ...],
) -> None:
    if observation.exit_code != spec.expected_exit_code:
        raise BaselineError(
            f"cargo exit code changed: {observation.exit_code} != {spec.expected_exit_code}"
        )
    if observation.summary != spec.expected_summary:
        raise BaselineError(
            f"cargo summary changed: {observation.summary!r} != {spec.expected_summary!r}"
        )
    if observation.summary.filtered != 0:
        raise BaselineError("filtered test count is nonzero")
    if len(observation.inventory) != observation.summary.total:
        raise BaselineError(
            f"inventory size {len(observation.inventory)} does not match summary total {observation.summary.total}"
        )
    if observation.inventory != expected_inventory:
        added = sorted(set(observation.inventory) - set(expected_inventory))
        removed = sorted(set(expected_inventory) - set(observation.inventory))
        raise BaselineError(f"test inventory changed: added={added[:3]} removed={removed[:3]}")
    if observation.failures != expected_failures:
        added = sorted(set(observation.failures) - set(expected_failures))
        removed = sorted(set(expected_failures) - set(observation.failures))
        raise BaselineError(f"failure names changed: added={added[:3]} removed={removed[:3]}")
    if canonical_sha256(observation.inventory) != spec.inventory_sha256:
        raise BaselineError("test inventory hash does not match manifest")
    if canonical_sha256(observation.failures) != spec.failures_sha256:
        raise BaselineError("failure-name hash does not match manifest")


def _run(command: tuple[str, ...], root: Path, environment: tuple[tuple[str, str], ...]) -> subprocess.CompletedProcess[str]:
    env = os.environ.copy()
    env.update(dict(environment))
    try:
        return subprocess.run(command, cwd=root, env=env, capture_output=True, text=True, check=False)
    except OSError as exc:
        raise BaselineError(f"cannot execute {' '.join(command)}: {exc}") from exc


def observe(root: Path, spec: BaselineSpec) -> Observation:
    listed = _run(spec.list_command, root, spec.environment)
    if listed.returncode != 0:
        raise BaselineError(f"cargo --list failed with exit={listed.returncode}")
    inventory = parse_test_list(listed.stdout + "\n" + listed.stderr)
    tested = _run(spec.test_command, root, spec.environment)
    output = tested.stdout + "\n" + tested.stderr
    try:
        summary, failures = parse_test_output(output)
    except BaselineError as exc:
        raise BaselineError(f"cargo run has no complete summary (exit={tested.returncode}): {exc}") from exc
    return Observation(summary, inventory, failures, tested.returncode)


def run(root: Path, manifest_path: Path) -> int:
    try:
        spec = load_manifest(manifest_path)
        expected_inventory = read_receipt_lines(
            root / spec.inventory_path, allow_empty=False
        )
        expected_failures = read_receipt_lines(
            root / spec.failures_path, allow_empty=True
        )
        if canonical_sha256(expected_inventory) != spec.inventory_sha256:
            raise BaselineError("checked-in test inventory hash is stale")
        if canonical_sha256(expected_failures) != spec.failures_sha256:
            raise BaselineError("checked-in failure-name hash is stale")
        observation = observe(root, spec)
        compare_observation(observation, spec, expected_inventory, expected_failures)
    except BaselineError as exc:
        print(f"[cargo-lib-red-baseline] ERROR: {exc}", file=sys.stderr)
        return 1
    summary = observation.summary
    print(
        "[cargo-lib-red-baseline] KNOWN BASELINE "
        f"status={summary.status} passed={summary.passed} failed={summary.failed} "
        f"ignored={summary.ignored} measured={summary.measured} inventory={len(observation.inventory)} "
        f"failure_sha256={canonical_sha256(observation.failures)}"
    )
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path("."))
    parser.add_argument("--manifest", type=Path, default=DEFAULT_MANIFEST)
    args = parser.parse_args(argv)
    root = args.root.resolve()
    manifest = args.manifest if args.manifest.is_absolute() else root / args.manifest
    return run(root, manifest)


if __name__ == "__main__":
    raise SystemExit(main())
