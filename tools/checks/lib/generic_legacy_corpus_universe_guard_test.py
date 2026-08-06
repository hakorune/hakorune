#!/usr/bin/env python3
"""Focused contract tests for the Generic legacy corpus P0 checker."""

from __future__ import annotations

import csv
import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import generic_legacy_corpus_universe_guard as guard


ROOT = Path(__file__).resolve().parents[3]
MANIFEST = ROOT / "docs/development/current/main/design/fixtures/generic-loop-legacy-disposition-v1.tsv"
RECEIPT = ROOT / "docs/development/current/main/design/fixtures/generic-legacy-observation-front-g0-v1.json"


class GenericLegacyCorpusGuardTest(unittest.TestCase):
    def _rewrite(self, mutate):
        with MANIFEST.open(newline="") as source:
            rows = list(csv.reader(source, delimiter="\t"))
        mutate(rows)
        stream = tempfile.NamedTemporaryFile("w", newline="", suffix=".tsv", delete=False)
        try:
            csv.writer(stream, delimiter="\t", lineterminator="\n").writerows(rows)
            stream.flush()
            return Path(stream.name)
        finally:
            stream.close()

    def test_current_manifest_is_closed_inventory_shape(self):
        records = guard.validate_manifest(MANIFEST, ROOT)
        self.assertEqual(sum(item.values["record_kind"] == "case" for item in records), 389)
        self.assertEqual(sum(item.values["alias_of"] != "-" for item in records if item.values["record_kind"] == "case"), 4)

    def test_front_receipt_is_a_named_pre_loop_failure(self):
        guard.validate_front_receipt(RECEIPT, MANIFEST, ROOT)

    def test_front_receipt_rejects_smoke_wrapper_command(self):
        value = json.loads(RECEIPT.read_text())
        value["command_argv"][-1] = value["canonical_fixture"]
        stream = tempfile.NamedTemporaryFile("w", suffix=".json", delete=False)
        path = Path(stream.name)
        try:
            stream.write(json.dumps(value))
            stream.close()
            with self.assertRaises(guard.ManifestError):
                guard.validate_front_receipt(path, MANIFEST, ROOT)
        finally:
            if not stream.file.closed:
                stream.close()
            path.unlink()

    def test_duplicate_source_provenance_rejects(self):
        def mutate(rows):
            rows[3][11] = rows[4][11]

        path = self._rewrite(mutate)
        try:
            with self.assertRaises(guard.ManifestError):
                guard.validate_manifest(path, ROOT)
        finally:
            path.unlink()

    def test_alias_chain_rejects(self):
        def mutate(rows):
            alias_rows = [row for row in rows if row and row[0] == "case" and row[6] != "-"]
            alias_rows[0][6] = alias_rows[1][1]

        path = self._rewrite(mutate)
        try:
            with self.assertRaises(guard.ManifestError):
                guard.validate_manifest(path, ROOT)
        finally:
            path.unlink()

    def test_edge_case_fields_must_use_sentinel(self):
        def mutate(rows):
            rows.append(["edge", "edge-test", "src/test.rs", "phase29bq", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "symbol", "role", "0", "0", "effect", "action", "retire", "owner"])

        path = self._rewrite(mutate)
        try:
            with self.assertRaises(guard.ManifestError):
                guard.validate_manifest(path, ROOT)
        finally:
            path.unlink()


if __name__ == "__main__":
    unittest.main()
