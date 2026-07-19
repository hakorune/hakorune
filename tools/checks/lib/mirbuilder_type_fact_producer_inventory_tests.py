#!/usr/bin/env python3
"""Unit coverage for the FACT0-P0 direct-writer scanner."""

from __future__ import annotations

import tempfile
import unittest
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


if __name__ == "__main__":
    unittest.main()
