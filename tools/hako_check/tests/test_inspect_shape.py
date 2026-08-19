from __future__ import annotations

import json
import sys
import tempfile
import unittest
from argparse import Namespace
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
TOOLS = ROOT / "tools" / "hako_check"
if str(TOOLS) not in sys.path:
    sys.path.insert(0, str(TOOLS))

from inspect_scope_identity import build_identity_contract
from inspect_shape import run_shape
from inspect_shape_model import asm_shape, llvm_shape, mir_shape


class InspectShapeTest(unittest.TestCase):
    def _bundle(self, root: Path) -> tuple[Path, dict[str, object]]:
        bundle = root / "bundle"
        bundle.mkdir()
        source = bundle / "source.full.hako"
        source.write_text("box Main {}\n", encoding="utf-8")
        (bundle / "source.slice.hako").write_text("box Main {}\n", encoding="utf-8")
        mir = {
            "functions": [
                {
                    "name": "main",
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "phi", "dst": 1},
                                {"op": "mir_call", "dst": 2},
                                {"op": "branch", "cond": 2, "then": 1, "else": 2},
                            ],
                        },
                        {"id": 1, "instructions": [{"op": "ret", "value": 1}]},
                        {"id": 2, "instructions": [{"op": "jump", "target": 1}]},
                    ],
                }
            ]
        }
        (bundle / "mir.raw.json").write_text(
            json.dumps(mir, sort_keys=True) + "\n", encoding="utf-8"
        )
        (bundle / "llvm.ir").write_text(
            "define i64 @ny_main() {\n"
            "entry:\n"
            "  %x = phi i64 [ 0, %entry ]\n"
            "  %v = load i64, ptr null\n"
            "  br i1 true, label %yes, label %no\n"
            "yes:\n"
            "  tail call void @sink()\n"
            "  ret i64 %v\n"
            "no:\n"
            "  store i64 0, ptr null\n"
            "  ret i64 0\n"
            "}\n",
            encoding="utf-8",
        )
        (bundle / "executable.bin").write_bytes(b"sealed-executable")
        (bundle / "asm.s").write_text(
            "0000000000001000 <ny_main>:\n"
            " 1000: 74 02 je 1004\n"
            " 1002: e8 00 00 00 00 call 1007\n"
            " 1007: c3 ret\n",
            encoding="utf-8",
        )
        identity = build_identity_contract(
            out_dir=bundle,
            source_file=source,
            selector={"kind": "function", "region_id": "main", "start_line": 1, "end_line": 1},
            artifact_names=[
                "source.full.hako",
                "source.slice.hako",
                "mir.raw.json",
                "llvm.ir",
                "executable.bin",
                "asm.s",
            ],
            mappings={
                "source_to_mir": "exact",
                "mir_to_llvm": "block",
                "llvm_to_asm": "symbol",
            },
            mir_function="main",
            llvm_function="ny_main",
            asm_symbol="ny_main",
        )
        (bundle / "identity.json").write_text(
            json.dumps(identity, indent=2, sort_keys=True) + "\n", encoding="utf-8"
        )
        return bundle, identity

    def test_shape_command_renders_sealed_layers_and_external_c(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            bundle, identity = self._bundle(root)
            c_asm = root / "c.asm"
            c_asm.write_text("00000001 <c_loop>:\n 1: c3 ret\n", encoding="utf-8")
            out = root / "shape"
            rc = run_shape(
                Namespace(bundle=bundle, c_asm=c_asm, c_symbol="c_loop", out=out)
            )
            self.assertEqual(rc, 0)
            report = json.loads((out / "shape.json").read_text(encoding="utf-8"))
            self.assertEqual(report["candidate_seal"], identity["candidate_seal"])
            self.assertEqual(report["layers"]["mir"]["blocks"], 3)
            self.assertEqual(report["layers"]["mir"]["edges"], 3)
            self.assertEqual(report["layers"]["llvm"]["calls"], 1)
            self.assertEqual(report["layers"]["llvm"]["loads"], 1)
            self.assertEqual(report["layers"]["asm"]["branches"], 1)
            self.assertIsNone(report["layers"]["asm"]["loads"])
            self.assertEqual(report["layers"]["c_asm"]["returns"], 1)
            self.assertEqual(report["external_reference"]["symbol"], "c_loop")
            self.assertEqual(report["cross_layer_correspondence"], "unclaimed")
            self.assertFalse(report["keeper_selection"])
            self.assertFalse(report["measurement_authority"])

    def test_shape_rejects_tamper_before_output(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            bundle, _ = self._bundle(root)
            (bundle / "llvm.ir").write_text("tampered\n", encoding="utf-8")
            out = root / "shape"
            with self.assertRaisesRegex(SystemExit, "artifact digest mismatch"):
                run_shape(Namespace(bundle=bundle, c_asm=None, c_symbol=None, out=out))
            self.assertFalse(out.exists())

    def test_shape_rejects_below_floor_and_partial_external_c(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            bundle, identity = self._bundle(root)
            identity["shape_ready"] = False
            (bundle / "identity.json").write_text(
                json.dumps(identity, sort_keys=True) + "\n", encoding="utf-8"
            )
            with self.assertRaisesRegex(SystemExit, "shape-ready"):
                run_shape(Namespace(bundle=bundle, c_asm=None, c_symbol=None, out=root / "a"))
            other_root = root / "other"
            other_root.mkdir()
            bundle, _ = self._bundle(other_root)
            with self.assertRaisesRegex(SystemExit, "requires both"):
                run_shape(
                    Namespace(bundle=bundle, c_asm=root / "c.asm", c_symbol=None, out=root / "b")
                )

    def test_pure_shape_counters_reject_ambiguous_selection(self) -> None:
        with self.assertRaisesRegex(SystemExit, "MIR function must be unique"):
            mir_shape({"functions": [{"name": "main"}, {"name": "main"}]}, "main")
        with self.assertRaisesRegex(SystemExit, "LLVM function must be unique"):
            llvm_shape(
                "define void @f() { ret void }\ndefine void @f() { ret void }\n", "f"
            )
        with self.assertRaisesRegex(SystemExit, "assembly symbol must be unique"):
            asm_shape("0001 <f>:\n0002 <f>:\n", "f")
