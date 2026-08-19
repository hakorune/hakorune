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

from inspect_scope_identity import validate_identity_contract
from inspect_selected_dynamic_provenance import (
    FUNCTION, seal_product, validate_producer, validate_product_inventory,
)


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


class SelectedDynamicProvenanceIngressTests(unittest.TestCase):
    def fixture(self, root: Path) -> tuple[Path, Path, Path, Path, Path]:
        producer = root / "producer"
        producer.mkdir()
        source = producer / "source.full.hako"
        source.write_text("box ParserScanLoopBox {}\n", encoding="utf-8")
        mir = producer / "real.json"
        mir.write_text(json.dumps({
            "functions": [{
                "name": FUNCTION,
                "blocks": [
                    {"id": 0, "instructions": [{"op": "jump", "target": 1}]},
                    {"id": 1, "instructions": [{"op": "ret"}]},
                ],
            }],
        }), encoding="utf-8")
        (producer / "producer.json").write_text(json.dumps({
            "output_contract": "hako-inspect-selected-dynamic-producer-v1",
            "source_kind": "source_backed_fixture",
            "launch_kind": "route_admission_scaffold_non_authority",
            "source_path": "lang/src/compiler/parser/scan/parser_scan_loop_box.hako",
            "source_file": "source.full.hako",
            "source_sha256": digest(source),
            "mir_json_file": "real.json",
            "mir_json_sha256": digest(mir),
            "mir_function": FUNCTION,
        }), encoding="utf-8")
        llvm = root / "lowered.ll"
        llvm.write_text(
            f"define i64 @\"{FUNCTION}\"() {{\n"
            "bb0:\n  br label %bb1\n"
            "bb1:\n  ret i64 0\n}\n",
            encoding="utf-8",
        )
        raw = root / "origins.tsv"
        raw.write_text(
            "block\t0\t-1\tnone\t-1\tbb0\t\tpreserved\tbase_block\n"
            "edge\t0\t0\ttarget\t1\tbb0\tbb1\tpreserved\tjump\n"
            "block\t1\t-1\tnone\t-1\tbb1\t\tpreserved\tbase_block\n",
            encoding="utf-8",
        )
        object_path = root / "real.o"
        object_path.write_bytes(b"same-emission-object")
        asm = root / "real.asm"
        asm.write_text(
            "0000000000000010 <ParserScanLoopBox.skip_while/4>:\n"
            "  10:\t48 89 c0             \tmov    %rax,%rax\n"
            "  13:\tc3                   \tret\n",
            encoding="utf-8",
        )
        return producer, llvm, raw, object_path, asm

    def test_seals_exact_lowered_boundary_without_machine_claim(self) -> None:
        with tempfile.TemporaryDirectory() as raw_root:
            root = Path(raw_root)
            producer, llvm, raw, object_path, asm = self.fixture(root)
            identity = seal_product(
                producer=producer, lowered=llvm, raw=raw,
                object_path=object_path, asm_path=asm, out=root / "bundle",
            )
            self.assertEqual(
                identity["mappings"],
                {
                    "source_to_mir": "exact",
                    "mir_to_llvm": "issuer_exact_lowered_pre_opt",
                    "lowered_llvm_to_final_llvm": "unavailable",
                    "llvm_to_asm": "unavailable",
                },
            )
            provenance = json.loads(
                (root / "bundle/lowering.provenance.json").read_text()
            )
            self.assertEqual(
                provenance["candidate_input"]["issuer"],
                "selected_dynamic_c1_lowerer",
            )
            self.assertEqual(
                provenance["candidate_input"]["llvm_boundary"],
                "lowered_pre_opt",
            )
            self.assertFalse(identity["shape_ready"])
            self.assertEqual(
                set(identity["artifacts"]),
                {
                    "producer.json", "source.full.hako", "mir.raw.json",
                    "llvm.lowered-pre-opt.ir", "lowering.origins.tsv",
                    "lowering.provenance.json", "object.bin", "asm.s",
                    "origin-footprint.json", "summary.md",
                },
            )
            footprint = json.loads(
                (root / "bundle/origin-footprint.json").read_text()
            )
            self.assertEqual(footprint["llvm_boundary"], "lowered_pre_opt")
            self.assertEqual(
                footprint["lowered_llvm_to_machine"], "unavailable"
            )
            self.assertEqual(footprint["asm"]["symbol"], FUNCTION)
            self.assertEqual(
                footprint["asm"]["origin_attribution"], "unavailable"
            )
            self.assertEqual(footprint["asm"]["shape"]["instructions"], 2)
            self.assertEqual(footprint["asm"]["shape"]["returns"], 1)
            self.assertEqual(
                {path.name for path in (root / "bundle").iterdir()},
                set(identity["artifacts"]) | {"identity.json"},
            )

    def test_summary_and_producer_are_inside_identity_seal(self) -> None:
        with tempfile.TemporaryDirectory() as raw_root:
            root = Path(raw_root)
            producer, llvm, raw, object_path, asm = self.fixture(root)
            identity = seal_product(
                producer=producer, lowered=llvm, raw=raw,
                object_path=object_path, asm_path=asm, out=root / "bundle",
            )
            for name in (
                "summary.md", "producer.json", "object.bin", "asm.s",
                "origin-footprint.json",
            ):
                artifact = root / "bundle" / name
                original = artifact.read_bytes()
                artifact.write_bytes(original + b"tampered\n")
                with self.subTest(name=name), self.assertRaisesRegex(
                    SystemExit, "artifact digest mismatch"
                ):
                    validate_identity_contract(root / "bundle", identity)
                artifact.write_bytes(original)

    def test_missing_or_duplicate_selected_symbol_rejects(self) -> None:
        with tempfile.TemporaryDirectory() as raw_root:
            root = Path(raw_root)
            producer, llvm, raw, object_path, asm = self.fixture(root)
            original = asm.read_text()
            for text in ("no symbol\n", original + original):
                asm.write_text(text)
                with self.assertRaisesRegex(SystemExit, "must be unique"):
                    seal_product(
                        producer=producer, lowered=llvm, raw=raw,
                        object_path=object_path, asm_path=asm,
                        out=root / "bundle",
                    )
                self.assertFalse((root / "bundle").exists())

    def test_extra_published_sibling_rejects(self) -> None:
        with tempfile.TemporaryDirectory() as raw_root:
            root = Path(raw_root)
            producer, llvm, raw, object_path, asm = self.fixture(root)
            identity = seal_product(
                producer=producer, lowered=llvm, raw=raw,
                object_path=object_path, asm_path=asm, out=root / "bundle",
            )
            (root / "bundle" / "foreign.txt").write_text("foreign")
            with self.assertRaisesRegex(SystemExit, "published inventory mismatch"):
                validate_product_inventory(root / "bundle", identity)

    def test_foreign_producer_digest_rejects_before_publication(self) -> None:
        with tempfile.TemporaryDirectory() as raw_root:
            root = Path(raw_root)
            producer, llvm, raw, object_path, asm = self.fixture(root)
            (producer / "source.full.hako").write_text("foreign\n")
            with self.assertRaises(SystemExit):
                seal_product(
                    producer=producer, lowered=llvm, raw=raw,
                    object_path=object_path, asm_path=asm, out=root / "bundle",
                )
            self.assertFalse((root / "bundle").exists())

    def test_duplicate_origin_rejects_and_removes_staging(self) -> None:
        with tempfile.TemporaryDirectory() as raw_root:
            root = Path(raw_root)
            producer, llvm, raw, object_path, asm = self.fixture(root)
            first = raw.read_text().splitlines()[0]
            raw.write_text(raw.read_text() + first + "\n")
            with self.assertRaises(SystemExit):
                seal_product(
                    producer=producer, lowered=llvm, raw=raw,
                    object_path=object_path, asm_path=asm, out=root / "bundle",
                )
            self.assertFalse((root / "bundle").exists())
            self.assertEqual(list(root.glob(".bundle.*")), [])

    def test_launch_kind_is_mandatory(self) -> None:
        with tempfile.TemporaryDirectory() as raw_root:
            root = Path(raw_root)
            producer, _llvm, _raw, _object, _asm = self.fixture(root)
            manifest = json.loads((producer / "producer.json").read_text())
            manifest["launch_kind"] = "semantic_authority"
            (producer / "producer.json").write_text(json.dumps(manifest))
            with self.assertRaises(SystemExit):
                validate_producer(producer)


if __name__ == "__main__":
    unittest.main()
