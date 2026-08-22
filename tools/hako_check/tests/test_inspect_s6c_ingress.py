from __future__ import annotations

import hashlib
import json
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
TOOLS = ROOT / "tools" / "hako_check"
if str(TOOLS) not in sys.path:
    sys.path.insert(0, str(TOOLS))

from inspect_s6c_ingress import seal_ingress, validate_producer_dir
from inspect_scope_identity import validate_identity_contract
from inspect_shape_model import build_shape_report


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


class InspectS6CIngressTest(unittest.TestCase):
    def _producer(self, root: Path) -> Path:
        producer = root / "producer"
        producer.mkdir()
        source = producer / "source.full.hako"
        source.write_text("box Main { find_ok(s, ch) { return -1 } }\n", encoding="utf-8")
        mir = {
            "functions": [
                {
                    "name": "Main.find_ok/2",
                    "blocks": [{"id": 0, "instructions": [{"op": "ret", "value": 0}]}],
                }
            ]
        }
        mir_path = producer / "real.json"
        mir_path.write_text(json.dumps(mir, sort_keys=True) + "\n", encoding="utf-8")
        manifest = {
            "output_contract": "hako-inspect-s6c-producer-v1",
            "source_kind": "source_backed_fixture",
            "source_path": "apps/tests/scan_with_init_typed_ok_min.hako",
            "source_file": "source.full.hako",
            "source_sha256": sha256(source),
            "mir_json_file": "real.json",
            "mir_json_sha256": sha256(mir_path),
            "mir_function": "Main.find_ok/2",
            "summary": "ok",
        }
        (producer / "producer.json").write_text(
            json.dumps(manifest, sort_keys=True) + "\n", encoding="utf-8"
        )
        return producer

    def _backend(self, root: Path) -> tuple[Path, Path, Path]:
        final_llvm = root / "final.ll"
        final_llvm.write_text(
            "define i64 @ny_main() {\nentry:\n  ret i64 -1\n}\n", encoding="utf-8"
        )
        object_file = root / "final.o"
        object_file.write_bytes(b"one-selected-object")
        disassembly = root / "objdump.txt"
        disassembly.write_text(
            "0000000000001000 <ny_main>:\n 1000: c3 ret\n", encoding="utf-8"
        )
        return final_llvm, object_file, disassembly

    def _provenance(self, root: Path) -> Path:
        path = root / "provenance.tsv"
        path.write_text(
            "block\t0\t-1\tnone\t-1\tentry\t\tpreserved\tbase_block\n",
            encoding="utf-8",
        )
        return path

    def test_ingress_seals_exact_source_json_and_object(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            producer = self._producer(root)
            final_llvm, object_file, disassembly = self._backend(root)
            out = root / "bundle"
            identity = seal_ingress(
                producer=producer,
                final_llvm=final_llvm,
                object_file=object_file,
                disassembly=disassembly,
                provenance_raw=self._provenance(root),
                llvm_function="ny_main",
                asm_symbol="ny_main",
                out=out,
            )
            self.assertTrue(identity["shape_ready"])
            self.assertIn("object.bin", identity["artifacts"])
            self.assertEqual(identity["mappings"]["mir_to_llvm"], "issuer_exact")
            validate_identity_contract(out, identity)
            report = build_shape_report(
                identity=identity,
                mir=json.loads((out / "mir.raw.json").read_text(encoding="utf-8")),
                llvm_text=(out / "llvm.ir").read_text(encoding="utf-8"),
                asm_text=(out / "asm.s").read_text(encoding="utf-8"),
                provenance=json.loads(
                    (out / "lowering.provenance.json").read_text(encoding="utf-8")
                ),
            )
            self.assertEqual(report["layers"]["mir"]["returns"], 1)
            self.assertEqual(report["layers"]["llvm"]["returns"], 1)
            self.assertEqual(report["layers"]["asm"]["returns"], 1)

    def test_producer_digest_drift_rejects_without_identity(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            producer = self._producer(root)
            (producer / "source.full.hako").write_text("foreign\n", encoding="utf-8")
            out = root / "bundle"
            out.mkdir()
            (out / "identity.json").write_text("stale\n", encoding="utf-8")
            final_llvm, object_file, disassembly = self._backend(root)
            with self.assertRaisesRegex(SystemExit, "source digest mismatch"):
                seal_ingress(
                    producer=producer,
                    final_llvm=final_llvm,
                    object_file=object_file,
                    disassembly=disassembly,
                    provenance_raw=self._provenance(root),
                    llvm_function="ny_main",
                    asm_symbol="ny_main",
                    out=out,
                )
            self.assertFalse((out / "identity.json").exists())

    def test_json_drift_and_incomplete_manifest_reject(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            producer = self._producer(root)
            (producer / "real.json").write_text("{}\n", encoding="utf-8")
            with self.assertRaisesRegex(SystemExit, "MIR digest mismatch"):
                validate_producer_dir(producer)
            (producer / "producer.json").unlink()
            with self.assertRaisesRegex(SystemExit, "JSON missing"):
                validate_producer_dir(producer)

    def test_projected_or_foreign_backend_selector_rejects(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            producer = self._producer(root)
            final_llvm, object_file, disassembly = self._backend(root)
            final_llvm.write_text(
                "define i64 @hako_s6c_meso() { ret i64 -1 }\n", encoding="utf-8"
            )
            disassembly.write_text(
                "0000000000001000 <hako_s6c_meso>:\n 1000: c3 ret\n", encoding="utf-8"
            )
            out = root / "bundle"
            with self.assertRaisesRegex(SystemExit, "LLVM function must be unique"):
                seal_ingress(
                    producer=producer,
                    final_llvm=final_llvm,
                    object_file=object_file,
                    disassembly=disassembly,
                    provenance_raw=self._provenance(root),
                    llvm_function="ny_main",
                    asm_symbol="ny_main",
                    out=out,
                )
            self.assertFalse((out / "identity.json").exists())
