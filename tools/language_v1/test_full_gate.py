#!/usr/bin/env python3

import unittest

from tools.language_v1.grammar_contract_full_gate import support_matrix


class FullGateTests(unittest.TestCase):
    def test_support_matrix_distinguishes_support_and_transport(self) -> None:
        rust = [
            {
                "row_id": "match",
                "profile": "Canonical",
                "row_status": "observed",
                "ok": True,
            },
            {
                "row_id": "from_super_call",
                "profile": "Compat2025",
                "row_status": "migration_transport_owned",
                "ok": True,
            },
        ]
        hako = [
            {
                "row_id": "match",
                "profile": "Canonical",
                "row_status": "observed",
                "ok": True,
            },
            {
                "row_id": "from_super_call",
                "profile": "Compat2025",
                "row_status": "excluded",
                "ok": True,
            },
        ]
        matrix = support_matrix(rust, hako)
        statuses = {
            (row["parser"], row["row_id"]): row["status"] for row in matrix
        }
        self.assertEqual(statuses[("Rust", "match")], "supported")
        self.assertEqual(
            statuses[("Rust", "from_super_call")], "migration_transport_owned"
        )
        self.assertEqual(
            statuses[("Hako", "from_super_call")], "explicitly_excluded"
        )


if __name__ == "__main__":
    unittest.main()
