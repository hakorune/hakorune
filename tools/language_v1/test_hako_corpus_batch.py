#!/usr/bin/env python3

import unittest

from tools.language_v1.grammar_contract_corpus import fixtures_by_id
from tools.language_v1.hako_corpus_batch import (
    BATCH_SCHEMA,
    batch_environment,
    compare_batch,
)


class HakoCorpusBatchTests(unittest.TestCase):
    def test_batch_environment_projects_corpus_rows_by_index(self) -> None:
        corpus = fixtures_by_id()
        rows = [
            corpus["match_user_enum_canonical"],
            corpus["match_user_enum_non_exhaustive"],
        ]
        environment = batch_environment(rows, base={})
        self.assertEqual(environment["HAKO_GRAMMAR_CONTRACT_BATCH_COUNT"], "2")
        self.assertEqual(
            environment["HAKO_GRAMMAR_CONTRACT_BATCH_PROFILE_0"], "canonical"
        )
        self.assertIn(
            '"name":"ProbeState"',
            environment["HAKO_GRAMMAR_CONTRACT_BATCH_INVENTORY_JSON_1"],
        )

    def test_compare_batch_requires_exact_status_and_tag(self) -> None:
        corpus = fixtures_by_id()
        fixture_ids = [
            "match_user_enum_canonical",
            "match_user_enum_non_exhaustive",
        ]
        rows = [corpus[fixture_id] for fixture_id in fixture_ids]
        payload = {
            "schema": BATCH_SCHEMA,
            "raw_program_json_authority": False,
            "observations": [
                {
                    "schema": "language-v1-hako-raw-evidence-v0",
                    "status": "ok",
                    "stable_reject_tag": "",
                    "deterministic": True,
                    "raw_program_json_authority": False,
                    "program": {"version": 0, "kind": "Program", "body": []},
                },
                {
                    "schema": "language-v1-hako-raw-evidence-v0",
                    "status": "error",
                    "stable_reject_tag": "parser/hako_enum_match_non_exhaustive",
                    "deterministic": True,
                    "raw_program_json_authority": False,
                },
            ],
        }
        report = compare_batch(fixture_ids, rows, payload)
        self.assertEqual(report["status"], "ok")
        self.assertEqual(report["adapter_process_count"], 1)

    def test_corpus_declares_peek_match_program_equivalence(self) -> None:
        corpus = fixtures_by_id()
        self.assertEqual(
            corpus["peek_compat_normalizable"]["hako_equivalent_fixture_id"],
            "match_compat",
        )


if __name__ == "__main__":
    unittest.main()
