#!/usr/bin/env python3
"""Small selftest for rust-subset-to-hako converter."""

import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent
CONVERT = ROOT / "convert.py"


def run_convert(input_path: Path) -> subprocess.CompletedProcess:
    return subprocess.run(
        [sys.executable, str(CONVERT), str(input_path)],
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )


def assert_golden(name: str) -> None:
    input_path = ROOT / "examples" / f"{name}_subset.json"
    expected_path = ROOT / "examples" / f"{name}_expected.hako"
    result = run_convert(input_path)
    if result.returncode != 0:
        raise AssertionError(f"{name}: converter failed: {result.stderr}")
    expected = expected_path.read_text()
    if result.stdout != expected:
        raise AssertionError(
            f"{name}: output mismatch\n--- expected ---\n{expected}\n--- actual ---\n{result.stdout}"
        )


def assert_fail_fast_case(name: str, expected_stderr: str) -> None:
    result = run_convert(ROOT / "examples" / f"{name}.json")
    if result.returncode == 0:
        raise AssertionError(f"{name}: expected non-zero exit")
    if expected_stderr not in result.stderr:
        raise AssertionError(f"{name}: unexpected stderr: {result.stderr}")


def main() -> None:
    assert_golden("simple")
    assert_golden("edge")
    assert_golden("while")
    assert_golden("vec")
    assert_golden("index")
    assert_golden("break_continue")
    assert_golden("generic_function")
    assert_golden("generic_impl_target")
    assert_golden("reference_type")
    assert_golden("unsupported_trait")
    assert_golden("path_name")
    assert_fail_fast_case("invalid_schema_version", "unsupported schema_version")
    assert_fail_fast_case("invalid_document_kind", "unsupported document kind")
    assert_fail_fast_case("invalid_unknown_kind", "unknown item kind: Trait")
    assert_fail_fast_case("invalid_unknown_statement_kind", "unknown statement kind: Mystery")
    assert_fail_fast_case("invalid_unknown_expression_kind", "unknown expression kind: MysteryExpr")
    print("summary=ok")


if __name__ == "__main__":
    main()
