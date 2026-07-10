#!/usr/bin/env python3
"""Focused tests for the Hako grammar-adapter process boundary."""

from __future__ import annotations

import os
import pathlib
import subprocess
import sys
import unittest

from tools.language_v1.hako_adapter_health import (
    AdapterProcessResult,
    compare_repeated_results,
    health_envelope,
    probe_command,
    run_adapter_process,
)


def python_command(source: str) -> list[str]:
    return [sys.executable, "-c", source]


class HakoAdapterHealthTests(unittest.TestCase):
    def test_observation_command_carries_explicit_profile_argument(self) -> None:
        command = probe_command(pathlib.Path("hakorune"), "observation", "compat2025")
        self.assertIsNotNone(command)
        self.assertEqual(
            command[-3:], ["--", "--grammar-profile", "compat2025"]
        )

    def test_single_json_object_is_accepted(self) -> None:
        result = run_adapter_process(
            python_command('print("{\\\"status\\\":\\\"ok\\\"}")'),
            timeout_seconds=1.0,
        )
        self.assertEqual(result, AdapterProcessResult("ok", ""))

    def test_empty_and_malformed_output_have_distinct_tags(self) -> None:
        empty = run_adapter_process(python_command("pass"), timeout_seconds=1.0)
        malformed = run_adapter_process(
            python_command('print("not-json")'), timeout_seconds=1.0
        )
        self.assertEqual(empty.stable_reject_tag, "parser/hako_adapter_no_output")
        self.assertEqual(
            malformed.stable_reject_tag, "parser/hako_adapter_malformed_output"
        )

    def test_stdout_contamination_is_rejected(self) -> None:
        result = run_adapter_process(
            python_command('print("log"); print("{}")'), timeout_seconds=1.0
        )
        self.assertEqual(
            result.stable_reject_tag, "parser/hako_adapter_stdout_contaminated"
        )

    def test_raw_evidence_requires_internal_determinism_and_non_authority(self) -> None:
        nondeterministic = run_adapter_process(
            python_command(
                'print("{\\\"schema\\\":\\\"language-v1-hako-raw-evidence-v0\\\",'
                '\\\"deterministic\\\":false,\\\"raw_program_json_authority\\\":false}")'
            ),
            timeout_seconds=1.0,
        )
        authority = run_adapter_process(
            python_command(
                'print("{\\\"schema\\\":\\\"language-v1-hako-raw-evidence-v0\\\",'
                '\\\"deterministic\\\":true,\\\"raw_program_json_authority\\\":true}")'
            ),
            timeout_seconds=1.0,
        )
        self.assertEqual(
            nondeterministic.stable_reject_tag,
            "parser/hako_adapter_non_deterministic_output",
        )
        self.assertEqual(
            authority.stable_reject_tag,
            "parser/hako_raw_json_as_authority_forbidden",
        )

    def test_raw_evidence_propagates_stable_parser_rejection(self) -> None:
        rejected = run_adapter_process(
            python_command(
                'print("{\\\"schema\\\":\\\"language-v1-hako-raw-evidence-v0\\\",'
                '\\\"status\\\":\\\"error\\\",\\\"stable_reject_tag\\\":'
                '\\\"parser/hako_try_reserved\\\",\\\"deterministic\\\":true,'
                '\\\"raw_program_json_authority\\\":false}")'
            ),
            timeout_seconds=1.0,
        )
        self.assertEqual(rejected.stable_reject_tag, "parser/hako_try_reserved")

    def test_raw_program_digest_is_stable_and_non_authoritative(self) -> None:
        source = (
            'print("{\\\"schema\\\":\\\"language-v1-hako-raw-evidence-v0\\\",'
            '\\\"status\\\":\\\"ok\\\",\\\"stable_reject_tag\\\":\\\"\\\",'
            '\\\"deterministic\\\":true,\\\"raw_program_json_authority\\\":false,'
            '\\\"program\\\":{\\\"kind\\\":\\\"Program\\\",\\\"body\\\":[]}}")'
        )
        first = run_adapter_process(python_command(source), timeout_seconds=1.0)
        second = run_adapter_process(python_command(source), timeout_seconds=1.0)
        self.assertEqual(first.raw_program_digest, second.raw_program_digest)
        self.assertEqual(len(first.raw_program_digest), 64)
        envelope = health_envelope(
            probe_kind="observation", result=first, deterministic=True
        )
        self.assertEqual(envelope["raw_program_digest"], first.raw_program_digest)
        self.assertFalse(envelope["raw_program_json_authority"])

    def test_process_error_and_timeout_are_stable(self) -> None:
        process_error = run_adapter_process(
            python_command("raise SystemExit(3)"), timeout_seconds=1.0
        )
        timeout = run_adapter_process(
            python_command("import time; time.sleep(1)"), timeout_seconds=0.02
        )
        self.assertEqual(
            process_error.stable_reject_tag, "parser/hako_adapter_process_error"
        )
        self.assertEqual(timeout.stable_reject_tag, "parser/hako_adapter_timeout")

    def test_result_comparison_detects_nondeterminism(self) -> None:
        first = AdapterProcessResult("ok", "")
        second = AdapterProcessResult("error", "parser/hako_adapter_timeout")
        result = compare_repeated_results(first, second)
        self.assertEqual(
            result.stable_reject_tag,
            "parser/hako_adapter_non_deterministic_output",
        )

    def test_health_envelope_never_claims_grammar_authority(self) -> None:
        envelope = health_envelope(
            probe_kind="health",
            result=AdapterProcessResult("ok", ""),
            deterministic=True,
        )
        self.assertFalse(envelope["raw_program_json_authority"])
        self.assertFalse(envelope["parse_witness_conformance"])
        self.assertNotIn("grammar_profile", envelope)

    def test_nyash_features_does_not_change_boundary_result(self) -> None:
        command = python_command('print("{}")')
        base = run_adapter_process(command, timeout_seconds=1.0)
        environment = os.environ | {"NYASH_FEATURES": "no-try-compat"}
        with_features = run_adapter_process(
            command, timeout_seconds=1.0, environment=environment
        )
        self.assertEqual(base, with_features)


if __name__ == "__main__":
    unittest.main()
