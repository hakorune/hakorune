from __future__ import annotations

import subprocess
import sys
from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parents[3]
AUDIT = ROOT / "tools" / "hako_check" / "object_storage_plan_vocab_audit.py"


class ObjectStoragePlanVocabAuditTest(unittest.TestCase):
    def test_reports_keep_and_merge_candidates_without_merging(self) -> None:
        result = subprocess.run(
            [sys.executable, str(AUDIT)],
            check=True,
            capture_output=True,
            text=True,
        )
        rows = dict(line.split("=", 1) for line in result.stdout.splitlines() if "=" in line)

        self.assertEqual(rows["output_contract"], "hako-object-storage-plan-vocab-audit-v0")
        self.assertEqual(rows["keep_separate_count"], "6")
        self.assertEqual(rows["merge_candidate_count"], "6")
        self.assertEqual(rows["immediate_merge_allowed"], "0")
        self.assertEqual(rows["vocabulary_merge_count"], "0")
        self.assertEqual(rows["fact_fallback_separation_preserved"], "1")
        self.assertEqual(rows["exact_stack_object_external_producer_count"], "0")
        self.assertGreaterEqual(int(rows["fastpath_decision_non_test_consumer_count"]), 1)
        self.assertEqual(rows["fastpath_reachability_non_test_consumer_count"], "0")
        self.assertEqual(rows["passive_vocab_execution_enabled"], "0")
        self.assertEqual(rows["vocab_retire_allowed"], "0")
        self.assertEqual(rows["first_safe_followup"], "LOCALFIRSTOBJECTPLAN-ALIAS-RETIRE-PREFLIGHT-001")
        self.assertEqual(rows["row_3_name"], "local_fastpath_fact")
        self.assertEqual(rows["row_3_action"], "keep")
        self.assertEqual(rows["row_6_name"], "LocalFirstObjectPlan")
        self.assertEqual(rows["row_6_action"], "audit_before_retire")
        self.assertEqual(rows["row_7_name"], "reason_enums")
        self.assertEqual(rows["row_7_action"], "defer")
        self.assertEqual(rows["row_10_name"], "ExactStackObject")
        self.assertEqual(rows["row_10_action"], "defer_to_design")
        self.assertEqual(rows["row_11_name"], "fastpath_decision_reachability")
        self.assertEqual(rows["row_11_action"], "defer_to_resolver_or_retire_design")


if __name__ == "__main__":
    unittest.main()
