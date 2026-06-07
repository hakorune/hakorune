from __future__ import annotations

import json
import subprocess
import sys
import tempfile
from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parents[3]
EXPLAIN = ROOT / "tools" / "hako_check" / "fastpath_explain.py"


class FastPathExplainProofEnvelopeTest(unittest.TestCase):
    def test_counts_proof_envelopes(self) -> None:
        payload = {
            "functions": [
                {
                    "name": "main",
                    "metadata": {
                        "proof_envelopes": [
                            {
                                "profile": "direct_array",
                                "producer": "mir_json",
                                "proof_ids": ["exact_front_contract"],
                                "obligation_ids": ["exact_front_contract"],
                                "verifier_flags": {
                                    "bounds_policy": "checked",
                                },
                                "failure_reason": None,
                            },
                            {
                                "profile": "fastmem",
                                "producer": "mir_json",
                                "proof_ids": ["table_length_resolved"],
                                "obligation_ids": ["table_index"],
                                "verifier_flags": {
                                    "status": "verified",
                                },
                                "failure_reason": None,
                            },
                        ]
                    },
                }
            ]
        }

        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "mir.json"
            path.write_text(json.dumps(payload), encoding="utf-8")
            result = subprocess.run(
                [
                    sys.executable,
                    str(EXPLAIN),
                    "--format",
                    "json",
                    "--topn",
                    "3",
                    "--mir-json",
                    str(path),
                ],
                check=True,
                capture_output=True,
                text=True,
            )

        report = json.loads(result.stdout)
        counts = report["counts"]
        self.assertEqual(counts["proof_envelope_count"], "2")
        self.assertEqual(counts["proof_envelope_direct_array_count"], "1")
        self.assertEqual(counts["proof_envelope_fastmem_count"], "1")


if __name__ == "__main__":
    unittest.main()
