#!/usr/bin/env python3

import unittest

from tools.language_v1.grammar_contract_corpus import fixtures_by_id
from tools.language_v1.hako_corpus_batch import (
    BATCH_SCHEMA,
    batch_environment,
    compare_batch,
    report_without_adapter,
    select_hako_semantic_fixtures,
)
from tools.language_v1.grammar_contract_registry import (
    HAKO_TRANSPORT_EXCLUSION_TAG,
    RUST_MIGRATION_TRANSPORT_OWNER,
    fixture_ids_for_row,
    hako_transport_fixture_ids,
    registry_rows_by_key,
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

    def test_transport_exclusion_is_derived_from_registry_rows(self) -> None:
        corpus = fixtures_by_id()
        transport_ids = list(hako_transport_fixture_ids())
        fixtures = [corpus[fixture_id] for fixture_id in transport_ids]

        selection = select_hako_semantic_fixtures(transport_ids, fixtures)

        self.assertEqual(selection["included_ids"], [])
        self.assertEqual(selection["failures"], [])
        self.assertEqual(
            {row["fixture_id"] for row in selection["excluded_rows"]},
            set(transport_ids),
        )
        self.assertTrue(
            all(
                row["stable_reject_tag"] == HAKO_TRANSPORT_EXCLUSION_TAG
                and row["transport_owner"] == RUST_MIGRATION_TRANSPORT_OWNER
                and row["hako_adapter_invoked"] is False
                for row in selection["excluded_rows"]
            )
        )

    def test_transport_rows_never_enter_batch_environment(self) -> None:
        corpus = fixtures_by_id()
        fixture_ids = [
            "match_canonical",
            *hako_transport_fixture_ids(),
        ]
        fixtures = [corpus[fixture_id] for fixture_id in fixture_ids]
        selection = select_hako_semantic_fixtures(fixture_ids, fixtures)

        environment = batch_environment(selection["included_fixtures"], base={})

        self.assertEqual(selection["included_ids"], ["match_canonical"])
        self.assertEqual(environment["HAKO_GRAMMAR_CONTRACT_BATCH_COUNT"], "1")
        self.assertNotIn("box Child from Parent", environment.values())
        self.assertNotIn("from Parent.method()", environment.values())

    def test_exclusion_only_report_is_explicit_and_process_free(self) -> None:
        corpus = fixtures_by_id()
        transport_ids = list(hako_transport_fixture_ids())
        selection = select_hako_semantic_fixtures(
            transport_ids,
            [corpus[fixture_id] for fixture_id in transport_ids],
        )

        report = report_without_adapter(
            selection["excluded_rows"],
            selection["failures"],
            fixture_count=len(transport_ids),
        )

        self.assertEqual(report["status"], "ok")
        self.assertEqual(report["adapter_process_count"], 0)
        self.assertEqual(report["adapter_fixture_count"], 0)
        self.assertEqual(report["excluded_fixture_count"], len(transport_ids))

    def test_selection_distinguishes_missing_row_from_profile_drift(self) -> None:
        corpus = fixtures_by_id()
        fixture = dict(corpus["match_canonical"])
        registry = registry_rows_by_key()

        fixture["profile"] = "UnknownProfile"
        profile_drift = select_hako_semantic_fixtures(
            ["match_canonical"], [fixture], registry=registry
        )
        missing_row = select_hako_semantic_fixtures(
            ["match_canonical"], [fixture], registry={}
        )

        self.assertEqual(
            profile_drift["failures"][0]["reason"], "parser/profile_mismatch"
        )
        self.assertEqual(
            missing_row["failures"][0]["reason"], "parser/registry_row_missing"
        )

    def test_match_row_expansion_and_scrutinee_shape_are_registry_driven(self) -> None:
        corpus = fixtures_by_id()
        fixture_id = "match_name_scrutinee_canonical"
        self.assertIn(fixture_id, fixture_ids_for_row("match", "Canonical"))
        fixture = corpus[fixture_id]
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
                    "program": {
                        "body": [
                            {
                                "expr": {
                                    "type": "EnumMatch",
                                    "scrutinee": {"type": "RecordLiteral"},
                                }
                            }
                        ]
                    },
                }
            ],
        }

        report = compare_batch([fixture_id], [fixture], payload)

        self.assertEqual(report["status"], "error")
        self.assertEqual(
            report["failures"][0]["reason"], "parser/hako_witness_projection_drift"
        )


if __name__ == "__main__":
    unittest.main()
