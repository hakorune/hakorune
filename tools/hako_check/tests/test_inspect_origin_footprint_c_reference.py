#!/usr/bin/env python3

from __future__ import annotations

import hashlib
import json
import sys
import tempfile
import unittest
from pathlib import Path

TOOLS = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(TOOLS))

from inspect_origin_footprint_c_reference import derive_report
from inspect_selected_dynamic_provenance import FUNCTION, seal_product
from tools.hako_check.tests.test_inspect_selected_dynamic_provenance import (
    SelectedDynamicProvenanceIngressTests,
)


class OriginFootprintCReferenceTests(unittest.TestCase):
    def sealed_fixture(self, root: Path) -> tuple[Path, Path]:
        helper = SelectedDynamicProvenanceIngressTests()
        producer, llvm, origins, object_path, asm = helper.fixture(root)
        bundle = root / "bundle"
        seal_product(
            producer=producer, lowered=llvm, raw=origins,
            object_path=object_path, asm_path=asm, out=bundle,
        )
        c_asm = root / "c.asm"
        c_asm.write_text(
            "0000000000000020 <c_loop>:\n"
            "  20:\t48 83 c0 01          \tadd    $0x1,%rax\n"
            "  24:\t75 fa                \tjne    20 <c_loop>\n"
            "  26:\tc3                   \tret\n",
            encoding="utf-8",
        )
        return bundle, c_asm

    def test_emits_separate_digest_bound_report_without_mutating_bundle(self) -> None:
        with tempfile.TemporaryDirectory() as raw_root:
            root = Path(raw_root)
            bundle, c_asm = self.sealed_fixture(root)
            before = {
                path.name: hashlib.sha256(path.read_bytes()).hexdigest()
                for path in bundle.iterdir()
            }
            report = derive_report(
                bundle=bundle, c_asm=c_asm, c_symbol="c_loop", out=root / "report"
            )
            self.assertEqual(report["output_contract"], "hako-origin-footprint-c-reference-v0")
            self.assertEqual(report["correspondence"], "unavailable")
            self.assertEqual(report["external_reference"]["authority"], "external_reference_only")
            self.assertEqual(report["columns"]["hako_asm"]["symbol"], FUNCTION)
            self.assertEqual(report["columns"]["c_asm"]["shape"]["instructions"], 3)
            self.assertEqual(report["columns"]["c_asm"]["shape"]["branches"], 1)
            self.assertEqual(
                {path.name for path in (root / "report").iterdir()},
                {"comparison.json", "summary.md"},
            )
            self.assertEqual(before, {
                path.name: hashlib.sha256(path.read_bytes()).hexdigest()
                for path in bundle.iterdir()
            })
            summary = (root / "report/summary.md").read_text()
            self.assertIn("correspondence: unavailable", summary)
            self.assertNotIn("delta", summary.lower())

    def test_missing_or_duplicate_c_symbol_rejects_without_output(self) -> None:
        with tempfile.TemporaryDirectory() as raw_root:
            root = Path(raw_root)
            bundle, c_asm = self.sealed_fixture(root)
            original = c_asm.read_text()
            for index, text in enumerate(("no symbol\n", original + original)):
                c_asm.write_text(text)
                out = root / f"report-{index}"
                with self.assertRaisesRegex(SystemExit, "must be unique"):
                    derive_report(bundle=bundle, c_asm=c_asm, c_symbol="c_loop", out=out)
                self.assertFalse(out.exists())

    def test_missing_c_artifact_and_existing_output_reject(self) -> None:
        with tempfile.TemporaryDirectory() as raw_root:
            root = Path(raw_root)
            bundle, c_asm = self.sealed_fixture(root)
            with self.assertRaisesRegex(SystemExit, "regular file"):
                derive_report(
                    bundle=bundle, c_asm=root / "missing", c_symbol="c_loop",
                    out=root / "report",
                )
            out = root / "existing"
            out.mkdir()
            with self.assertRaisesRegex(SystemExit, "already exists"):
                derive_report(bundle=bundle, c_asm=c_asm, c_symbol="c_loop", out=out)

    def test_tampered_or_extra_hako_payload_rejects(self) -> None:
        with tempfile.TemporaryDirectory() as raw_root:
            root = Path(raw_root)
            bundle, c_asm = self.sealed_fixture(root)
            footprint = bundle / "origin-footprint.json"
            footprint.write_text(footprint.read_text() + " ")
            with self.assertRaisesRegex(SystemExit, "artifact digest mismatch"):
                derive_report(
                    bundle=bundle, c_asm=c_asm, c_symbol="c_loop", out=root / "report"
                )
            self.assertFalse((root / "report").exists())

        with tempfile.TemporaryDirectory() as raw_root:
            root = Path(raw_root)
            bundle, c_asm = self.sealed_fixture(root)
            (bundle / "foreign.txt").write_text("foreign")
            with self.assertRaisesRegex(SystemExit, "published inventory mismatch"):
                derive_report(
                    bundle=bundle, c_asm=c_asm, c_symbol="c_loop", out=root / "report"
                )

    def test_report_summary_digest_is_write_last_bound(self) -> None:
        with tempfile.TemporaryDirectory() as raw_root:
            root = Path(raw_root)
            bundle, c_asm = self.sealed_fixture(root)
            derive_report(bundle=bundle, c_asm=c_asm, c_symbol="c_loop", out=root / "report")
            report = json.loads((root / "report/comparison.json").read_text())
            self.assertEqual(
                report["summary_file"]["sha256"],
                hashlib.sha256((root / "report/summary.md").read_bytes()).hexdigest(),
            )
            self.assertFalse(report["keeper_selection"])
            self.assertFalse(report["measurement_authority"])


if __name__ == "__main__":
    unittest.main()
