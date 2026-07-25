#!/usr/bin/env python3
"""Run the one real-binary Raw VM-reference canary matrix.

The script is a proof fixture family, not a second selector or a per-row
guard.  The feature-enabled binary must already be built by the caller.
"""

from __future__ import annotations

import argparse
import os
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
FIXTURES = ROOT / "tools" / "checks" / "fixtures" / "raw_vm_reference_canary"


@dataclass(frozen=True)
class Case:
    name: str
    status: int
    diagnostic: str | None = None
    stdout: str | None = None


CASES = (
    Case("empty_script", 0),
    Case("integer_0", 0),
    Case("integer_255", 255),
    Case("integer_negative_1", 70, "[process/exit-code-out-of-range] value=-1 accepted=0..=255"),
    Case("integer_256", 70, "[process/exit-code-out-of-range] value=256 accepted=0..=255"),
    Case("bool", 70, "[process/unsupported-result] kind=Bool"),
    Case("float", 70, "[process/unsupported-result] kind=Float"),
    Case("string", 70, "[process/unsupported-result] kind=String"),
    Case("print", 0, stdout="1"),
    Case("local", 0),
    Case("assignment", 0),
    Case("compound_assignment", 0),
    Case("app_empty", 0),
    Case("app_fallthrough", 0),
    Case("division_fault", 70, "[process/source-fault] code=vm-division-by-zero detail=Division by zero"),
    Case("unsupported_loop", 1, "[raw-vm-reference/source/parse]"),
)


def run(
    binary: Path,
    args: list[str],
    env: dict[str, str] | None = None,
) -> subprocess.CompletedProcess[str]:
    merged = os.environ.copy()
    if env:
        merged.update(env)
    return subprocess.run(
        [str(binary), *args],
        cwd=ROOT,
        env=merged,
        text=True,
        capture_output=True,
        check=False,
    )


def expect_case(binary: Path, case: Case) -> None:
    fixture = FIXTURES / f"{case.name}.hako"
    result = run(binary, ["--backend", "raw-vm-reference", str(fixture)])
    if result.returncode != case.status:
        raise AssertionError(
            f"{case.name}: status {result.returncode}, expected {case.status}; "
            f"stderr={result.stderr!r}"
        )
    if case.diagnostic and case.diagnostic not in result.stderr:
        raise AssertionError(f"{case.name}: missing {case.diagnostic!r}: {result.stderr!r}")
    if case.stdout is not None and result.stdout.strip() != case.stdout:
        raise AssertionError(f"{case.name}: stdout {result.stdout!r}, expected {case.stdout!r}")
    if case.status == 0 and result.stderr:
        raise AssertionError(f"{case.name}: unexpected stderr {result.stderr!r}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, default=ROOT / "target" / "debug" / "hakorune")
    parser.add_argument(
        "--disabled-binary",
        type=Path,
        required=True,
        help="binary built without vm-reference; verifies pre-I/O feature rejection",
    )
    args = parser.parse_args()
    binary = args.binary.resolve()
    if not binary.is_file():
        raise SystemExit(f"missing feature-enabled binary: {binary}")

    for case in CASES:
        expect_case(binary, case)

    parse_rejection = run(
        binary,
        ["--backend", "raw-vm-reference", str(FIXTURES / "parse_invalid.hako")],
    )
    if parse_rejection.returncode != 1 or "[raw-vm-reference/source/parse]" not in parse_rejection.stderr:
        raise AssertionError(f"parse rejection did not use status 1: {parse_rejection!r}")

    compile_rejection = run(
        binary,
        ["--backend", "raw-vm-reference", str(FIXTURES / "missing_variable.hako")],
    )
    if compile_rejection.returncode != 1 or "[raw-vm-reference/invocation]" not in compile_rejection.stderr:
        raise AssertionError(f"Raw compile rejection did not use status 1: {compile_rejection!r}")

    missing = run(
        binary,
        ["--backend", "raw-vm-reference", str(FIXTURES / "does-not-exist.hako")],
    )
    if missing.returncode != 2 or "[raw-vm-reference/source/missing]" not in missing.stderr:
        raise AssertionError(f"missing source did not use status 2: {missing!r}")

    decoy = run(
        binary,
        ["--backend", "raw-vm-reference", str(FIXTURES / "integer_0.hako")],
        {"NYASH_ENTRY": "decoy"},
    )
    if decoy.returncode != 0 or decoy.stderr:
        raise AssertionError(f"sealed Main target changed under NYASH_ENTRY: {decoy!r}")

    conflict = run(
        binary,
        ["--backend", "raw-vm-reference", "--using", "foo", str(FIXTURES / "integer_0.hako")],
    )
    if conflict.returncode != 2 or "[raw-vm-reference/profile/rejected]" not in conflict.stderr:
        raise AssertionError(f"profile conflict did not fail with status 2: {conflict!r}")

    diagnostic_conflict = run(
        binary,
        ["--backend", "raw-vm-reference", "--verbose", str(FIXTURES / "integer_0.hako")],
    )
    if diagnostic_conflict.returncode != 2 or "diagnostic-route-requested" not in diagnostic_conflict.stderr:
        raise AssertionError(f"diagnostic profile conflict did not use status 2: {diagnostic_conflict!r}")

    default_route = run(binary, [str(FIXTURES / "integer_negative_1.hako")])
    if default_route.returncode != 255 or "[raw-vm-reference/" in default_route.stderr:
        raise AssertionError(f"default route changed: {default_route!r}")

    disabled = args.disabled_binary.resolve()
    if not disabled.is_file():
        raise SystemExit(f"missing feature-disabled binary: {disabled}")
    disabled_result = run(
        disabled,
        ["--backend", "raw-vm-reference", str(FIXTURES / "does-not-exist.hako")],
    )
    if disabled_result.returncode != 2 or "feature-unavailable" not in disabled_result.stderr:
        raise AssertionError(f"feature rejection did not precede file I/O: {disabled_result!r}")

    print(
        f"[entry-result-projection0-s3-canary-parity] ok cases={len(CASES)} "
        "decoy=1 conflict=2 default=1 disabled=1"
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"[entry-result-projection0-s3-canary-parity] fail: {error}", file=sys.stderr)
        raise SystemExit(1)
