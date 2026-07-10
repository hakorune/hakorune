#!/usr/bin/env python3

import pathlib
import tempfile
import unittest

from tools.language_v1.source_migration_report import scan_paths, tokenize_evidence


class SourceMigrationReportTests(unittest.TestCase):
    def test_evidence_lexer_ignores_comments_and_string_contents(self) -> None:
        tokens = tokenize_evidence('// while\n"peek while"\nwhile ready { }')
        self.assertEqual([token.text for token in tokens if token.text == "while"], ["while"])

    def test_scan_classifies_rejected_spellings_by_registry_row(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            path = pathlib.Path(temp_dir) / "probe.hako"
            path.write_text(
                'while ready { }\nweak(value)\nlocal m = {"key": 1}\n',
                encoding="utf-8",
            )
            findings = scan_paths([path])
        self.assertEqual(len(findings["while_loop_condition"]), 1)
        self.assertEqual(len(findings["weak_paren_expr"]), 1)
        self.assertEqual(len(findings["map_literal_legacy_brace_colon"]), 1)

    def test_scan_does_not_treat_names_or_string_values_as_legacy_syntax(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            path = pathlib.Path(temp_dir) / "probe.hako"
            path.write_text(
                'peek() { return "from" }\nlocal m = %{ "from" => 1 }\n',
                encoding="utf-8",
            )
            findings = scan_paths([path])
        self.assertEqual(findings["peek"], [])
        self.assertEqual(findings["from_super_call"], [])

    def test_scan_detects_closed_legacy_from_call(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            path = pathlib.Path(temp_dir) / "probe.hako"
            path.write_text("from Parent.birth()\n", encoding="utf-8")
            findings = scan_paths([path])
        self.assertEqual(len(findings["from_super_call"]), 1)


if __name__ == "__main__":
    unittest.main()
