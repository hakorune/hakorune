#!/usr/bin/env python3
"""Unit coverage for the FACT0-P0 direct-writer scanner."""

from __future__ import annotations

import tempfile
import unittest
import json
from pathlib import Path

import mirbuilder_type_fact_producer_inventory as inventory


class TypeFactProducerInventoryTests(unittest.TestCase):
    def test_lexer_ignores_comments_and_literals(self) -> None:
        source = '''
// type_ctx.value_types.insert(value, ty)
let regular = "type_ctx.value_types.insert(value, ty)";
let raw = r#"type_ctx.set_type(value, ty)"#;
/* type_ctx.value_types.remove(&value) */
type_ctx.value_types.insert(value, ty);
'''
        clean = inventory.code_only(source)
        self.assertEqual(len(inventory.TYPE_WRITE.findall(clean)), 1)

    def test_cfg_test_modules_are_excluded_without_removing_production(self) -> None:
        source = '''
fn production() { type_ctx.value_types.insert(value, ty); }
#[cfg(test)] mod inline { fn check() { type_ctx.value_types.clear(); } }
#[cfg(test)] mod external;
'''
        production = inventory.strip_cfg_test_modules(inventory.code_only(source))
        self.assertEqual(len(inventory.TYPE_WRITE.findall(production)), 1)
        self.assertIn("production", production)

    def test_writer_counts_ignore_inline_test_writers(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            source = root / "src/mir/builder/example.rs"
            source.parent.mkdir(parents=True)
            source.write_text(
                '''
fn production() { type_ctx.set_type(value, ty); }
#[cfg(test)] mod tests { fn fixture() { type_ctx.value_types.insert(value, ty); } }
''',
                encoding="utf-8",
            )
            self.assertEqual(
                inventory.writer_counts(root),
                {"src/mir/builder/example.rs": 1},
            )

    def test_schema_v2_accepts_shared_semantic_profiles(self) -> None:
        fixture = json.loads(
            (Path(__file__).parents[1] / "fixtures/mirbuilder_type_fact_partition_schema_v2.json")
            .read_text(encoding="utf-8")
        )
        inventory.validate_partition_schema_v2(fixture)

    def test_schema_v2_rejects_shared_site_without_reason(self) -> None:
        fixture = {
            "write_inventory": {"src/mir/builder/example.rs": 1},
            "partition_profiles": {
                "first": {
                    "family": "parameter",
                    "evidence_owner": "signature",
                    "commit_boundary": "entry",
                    "failure_residual": "none",
                    "retirement_prerequisite": "RCV0",
                    "status": "scoped_cutover",
                },
                "second": {
                    "family": "parameter",
                    "evidence_owner": "legacy",
                    "commit_boundary": "entry",
                    "failure_residual": "legacy",
                    "retirement_prerequisite": "PARAMETER-UNKNOWN0-D0",
                    "status": "legacy",
                },
            },
            "writer_partitions": [
                {
                    "source_file": "src/mir/builder/example.rs",
                    "slices": [
                        {
                            "first_ordinal": 1,
                            "last_ordinal": 1,
                            "producer_profiles": ["first", "second"],
                        }
                    ],
                }
            ],
        }
        with self.assertRaises(SystemExit):
            inventory.validate_partition_schema_v2(fixture)


if __name__ == "__main__":
    unittest.main()
