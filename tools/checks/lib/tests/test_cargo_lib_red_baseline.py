#!/usr/bin/env python3
"""Synthetic fail-closed tests for the full-lib baseline comparator."""

from __future__ import annotations

import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from cargo_lib_red_baseline import (  # noqa: E402
    BaselineError,
    BaselineSpec,
    CargoSummary,
    Observation,
    canonical_sha256,
    compare_observation,
    load_manifest,
    parse_test_list,
    parse_test_output,
    read_receipt_lines,
)


INVENTORY = ("test::alpha", "test::beta")
FAILURES = ("test::beta",)
SUMMARY = CargoSummary("FAILED", 1, 1, 0, 0, 0)


def make_spec(
    *, summary: CargoSummary = SUMMARY, inventory=INVENTORY, failures=FAILURES
) -> BaselineSpec:
    return BaselineSpec(
        ("cargo", "test", "--profile", "quick", "--lib", "--", "--test-threads=1"),
        ("cargo", "test", "--profile", "quick", "--lib", "--", "--list"),
        (("CARGO_BUILD_JOBS", "4"), ("CARGO_INCREMENTAL", "0"), ("RUST_MIN_STACK", "16777216")),
        summary.status,
        101 if summary.failed else 0,
        summary,
        "inventory.txt",
        canonical_sha256(tuple(inventory)),
        "failures.txt",
        canonical_sha256(tuple(failures)),
    )


def make_observation(
    *, summary: CargoSummary = SUMMARY, inventory=INVENTORY, failures=FAILURES, exit_code=101
) -> Observation:
    return Observation(summary, tuple(inventory), tuple(failures), exit_code)


class CargoLibRedBaselineTest(unittest.TestCase):
    def test_parse_known_red_summary_and_failure_names(self) -> None:
        output = (
            "running 2 tests\n"
            "test test::alpha ... ok\n"
            "test test::beta ... FAILED\n"
            "test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out;"
        )
        summary, failures = parse_test_output(output)
        self.assertEqual(summary, SUMMARY)
        self.assertEqual(failures, FAILURES)

    def test_parse_rejects_missing_summary_for_stack_abort(self) -> None:
        with self.assertRaisesRegex(BaselineError, "exactly one Cargo summary"):
            parse_test_output("thread 'main' has overflowed its stack\nAborted (core dumped)\n")

    def test_parse_rejects_multiple_summaries(self) -> None:
        output = "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;\n" * 2
        with self.assertRaisesRegex(BaselineError, "exactly one Cargo summary"):
            parse_test_output(output)

    def test_parse_rejects_duplicate_failure_names(self) -> None:
        output = (
            "test test::beta ... FAILED\n"
            "test test::beta ... FAILED\n"
            "test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out;"
        )
        with self.assertRaisesRegex(BaselineError, "duplicate failure"):
            parse_test_output(output)

    def test_parse_rejects_failure_count_mismatch(self) -> None:
        output = "test test::beta ... FAILED\ntest result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out;"
        with self.assertRaisesRegex(BaselineError, "does not match"):
            parse_test_output(output)

    def test_parse_list_rejects_empty_and_duplicate_inventory(self) -> None:
        with self.assertRaisesRegex(BaselineError, "empty test inventory"):
            parse_test_list("0 tests, 0 benchmarks\n")
        with self.assertRaisesRegex(BaselineError, "duplicate test names"):
            parse_test_list("test::alpha: test\ntest::alpha: test\n")

    def test_compare_accepts_exact_known_red(self) -> None:
        compare_observation(make_observation(), make_spec(), INVENTORY, FAILURES)

    def test_compare_rejects_added_failure(self) -> None:
        observation = make_observation(
            summary=CargoSummary("FAILED", 1, 2, 0, 0, 0),
            inventory=("test::alpha", "test::beta", "test::gamma"),
            failures=("test::beta", "test::gamma"),
        )
        expected_summary = CargoSummary("FAILED", 1, 2, 0, 0, 0)
        with self.assertRaisesRegex(BaselineError, "failure names changed"):
            compare_observation(
                observation,
                make_spec(
                    summary=expected_summary,
                    inventory=observation.inventory,
                    failures=FAILURES,
                ),
                observation.inventory,
                FAILURES,
            )

    def test_compare_rejects_disappeared_failure(self) -> None:
        observation = make_observation(
            summary=CargoSummary("ok", 2, 0, 0, 0, 0),
            failures=(),
            exit_code=0,
        )
        with self.assertRaisesRegex(BaselineError, "cargo exit code changed"):
            compare_observation(observation, make_spec(), INVENTORY, FAILURES)

    def test_compare_rejects_summary_drift(self) -> None:
        observation = make_observation(summary=CargoSummary("FAILED", 2, 1, 0, 0, 0))
        with self.assertRaisesRegex(BaselineError, "summary changed"):
            compare_observation(observation, make_spec(), INVENTORY, FAILURES)

    def test_compare_rejects_filtered_run(self) -> None:
        summary = CargoSummary("FAILED", 1, 1, 0, 0, 1)
        observation = make_observation(summary=summary)
        with self.assertRaisesRegex(BaselineError, "filtered"):
            compare_observation(observation, make_spec(summary=summary), INVENTORY, FAILURES)

    def test_compare_rejects_inventory_drift(self) -> None:
        observation = make_observation(inventory=("test::alpha", "test::other"))
        with self.assertRaisesRegex(BaselineError, "inventory changed"):
            compare_observation(observation, make_spec(), INVENTORY, FAILURES)

    def test_compare_rejects_inventory_size_mismatch(self) -> None:
        observation = make_observation(inventory=("test::alpha",))
        with self.assertRaisesRegex(BaselineError, "inventory size"):
            compare_observation(observation, make_spec(), INVENTORY, FAILURES)

    def test_receipt_lines_require_sorted_unique_names(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            path = root / "receipt.txt"
            path.write_text("z\na\n", encoding="utf-8")
            with self.assertRaisesRegex(BaselineError, "not sorted"):
                read_receipt_lines(path, allow_empty=False)
            path.write_text("a\na\n", encoding="utf-8")
            with self.assertRaisesRegex(BaselineError, "duplicate"):
                read_receipt_lines(path, allow_empty=False)

    def test_manifest_rejects_unaccepted_state(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "baseline.toml"
            path.write_text("schema_version = 1\nstate = 'candidate'\n", encoding="utf-8")
            with self.assertRaisesRegex(BaselineError, "state=accepted"):
                load_manifest(path)


if __name__ == "__main__":
    unittest.main()
