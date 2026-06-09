from __future__ import annotations

import json
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

ROOT = Path(__file__).resolve().parents[3]
TOOLS = ROOT / "tools" / "hako_check"
if str(TOOLS) not in sys.path:
    sys.path.insert(0, str(TOOLS))

import inspect_scope_dump


class InspectScopeDumpTest(unittest.TestCase):
    def _mir(self) -> dict[str, object]:
        return {
            "functions": [
                {
                    "name": "main",
                    "metadata": {
                        "route_decisions": [
                            {
                                "source_plan_kind": "TypedObjectExactSlotRoute",
                                "semantic_op": "FieldGet",
                                "selected_lowering_form": "exact_helper_bridge",
                                "selected_storage": "i64",
                                "selected_route": "hako.typed_object.slot_load_i64",
                                "selected_bridge_symbol": "hako.object.exact_slot_get_i64_hii",
                            }
                        ],
                        "array_text_state_residence_route": {
                            "function": "main",
                            "selected_route": "hako.array_text.session_indexof_const_utf8",
                            "selected_bridge_symbol": "hako.array_text.session_indexof_const_utf8",
                            "fallback_route": "nyash.array.string_indexof_hisi",
                            "fallback_policy": "fail_fast",
                        },
                        "array_text_residence_sessions": [
                            {
                                "function": "main",
                                "begin_block": 0,
                                "end_block": 1,
                                "publication_boundary": "none",
                                "carrier": "session_cached",
                            }
                        ],
                        "array_text_observer_routes": [
                            {
                                "function": "main",
                                "observer_kind": "indexof",
                                "consumer_shape": "direct_scalar",
                                "proof_region": "array_get_receiver_indexof",
                                "publication_boundary": "none",
                                "selected_route": "hako.array_text.session_indexof_const_utf8",
                                "selected_bridge_symbol": "hako.array_text.session_indexof_const_utf8",
                                "fallback_route": "nyash.array.string_indexof_hisi",
                                "fallback_policy": "fail_fast",
                            }
                        ],
                    },
                }
            ]
        }

    def test_scope_bundle_writes_source_and_report(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            tmp_path = Path(tmp)
            source = tmp_path / "scope.hako"
            source.write_text(
                "\n".join(
                    [
                        "box Example {",
                        "// hako:inspect begin region_a",
                        "__mir__.mark(\"region_a\")",
                        "local a",
                        "a = a + 1",
                        "// hako:inspect end region_a",
                        "}",
                    ]
                )
                + "\n",
                encoding="utf-8",
            )
            mir_json = tmp_path / "mir.json"
            mir_json.write_text(json.dumps(self._mir()), encoding="utf-8")
            out_dir = tmp_path / "bundle"

            rc = inspect_scope_dump.main(
                [
                    "scope",
                    "--source-file",
                    str(source),
                    "--span",
                    f"{source}:2:5",
                    "--mir-json",
                    str(mir_json),
                    "--out",
                    str(out_dir),
                ]
            )
            self.assertEqual(rc, 0)
            self.assertTrue((out_dir / "manifest.json").is_file())
            self.assertTrue((out_dir / "source.slice.hako").is_file())
            self.assertTrue((out_dir / "report.kv").is_file())
            report = (out_dir / "report.kv").read_text(encoding="utf-8")
            self.assertIn("output_contract=hako-check-inspect-scope-v0", report)
            self.assertIn("array_text_selected_route_count=1", report)
            self.assertIn("typed_object_exact_route_decision_count=1", report)
            self.assertIn("summary=ok", report)

    def test_scope_bundle_writes_backend_artifacts(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            tmp_path = Path(tmp)
            source = tmp_path / "scope.hako"
            source.write_text(
                "\n".join(
                    [
                        "box Example {",
                        "// hako:inspect begin region_a",
                        "__mir__.mark(\"region_a\")",
                        "local a",
                        "a = a + 1",
                        "// hako:inspect end region_a",
                        "}",
                    ]
                )
                + "\n",
                encoding="utf-8",
            )
            mir_json = tmp_path / "mir.json"
            mir_json.write_text(json.dumps(self._mir()), encoding="utf-8")
            out_dir = tmp_path / "bundle"

            def fake_emit_backend_bundle(mir_json_path: Path, function_name: str, timeout_secs: int):
                backend_dir = tmp_path / "backend"
                backend_dir.mkdir(exist_ok=True)
                (backend_dir / "lowered.ll").write_text("; ModuleID = 'test'\n", encoding="utf-8")
                (backend_dir / "objdump.txt").write_text(
                    "\n".join(
                        [
                            "0000000000001000 <ny_main>:",
                            "   1000:  c3                    ret",
                        ]
                    )
                    + "\n",
                    encoding="utf-8",
                )
                return backend_dir, "[bundle] llvm_ir=/tmp/test.ll\n"

            with mock.patch.object(inspect_scope_dump, "emit_llvm_asm_bundle", side_effect=fake_emit_backend_bundle):
                rc = inspect_scope_dump.main(
                    [
                        "scope",
                        "--source-file",
                        str(source),
                        "--span",
                        f"{source}:2:5",
                        "--mir-json",
                        str(mir_json),
                        "--emit",
                        "mir,mir-json,llvm,asm,report",
                        "--out",
                        str(out_dir),
                    ]
                )

            self.assertEqual(rc, 0)
            self.assertTrue((out_dir / "llvm.ir").is_file())
            self.assertTrue((out_dir / "asm.s").is_file())
            self.assertTrue((out_dir / "asm.map.json").is_file())
            report = (out_dir / "report.kv").read_text(encoding="utf-8")
            self.assertIn("emit_llvm=1", report)
            self.assertIn("emit_asm=1", report)
            self.assertIn("mir_to_llvm_mapping=block", report)
            self.assertIn("llvm_to_asm_mapping=symbol", report)
            asm_map = json.loads((out_dir / "asm.map.json").read_text(encoding="utf-8"))
            self.assertEqual(asm_map["output_contract"], "hako-inspect-asm-map-v0")
            self.assertEqual(asm_map["mapping_quality"], "symbol")

    def test_route_bundle_filters_selected_route(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            tmp_path = Path(tmp)
            mir_json = tmp_path / "mir.json"
            mir_json.write_text(json.dumps(self._mir()), encoding="utf-8")
            out_dir = tmp_path / "route"

            rc = inspect_scope_dump.main(
                [
                    "route",
                    "--mir-json",
                    str(mir_json),
                    "--selected-route",
                    "hako.array_text.session_indexof_const_utf8",
                    "--out",
                    str(out_dir),
                ]
            )
            self.assertEqual(rc, 0)
            report = (out_dir / "report.kv").read_text(encoding="utf-8")
            self.assertIn("output_contract=hako-check-inspect-route-v0", report)
            self.assertIn("selected_route_filter=hako.array_text.session_indexof_const_utf8", report)
            self.assertIn("route_row_count=2", report)
            self.assertIn("summary=ok", report)

    def test_mark_bundle_finds_anchor_and_window(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            tmp_path = Path(tmp)
            source = tmp_path / "scope.hako"
            source.write_text(
                "\n".join(
                    [
                        "box Example {",
                        "local a",
                        "__mir__.mark(\"region_a\")",
                        "a = a + 1",
                        "}",
                    ]
                )
                + "\n",
                encoding="utf-8",
            )
            out_dir = tmp_path / "mark"

            rc = inspect_scope_dump.main(
                [
                    "mark",
                    "--source-file",
                    str(source),
                    "--label",
                    "region_a",
                    "--window",
                    "1",
                    "--out",
                    str(out_dir),
                ]
            )
            self.assertEqual(rc, 0)
            report = (out_dir / "report.kv").read_text(encoding="utf-8")
            self.assertIn("output_contract=hako-check-inspect-mark-v0", report)
            self.assertIn("label=region_a", report)
            self.assertIn("summary=ok", report)
            slice_text = (out_dir / "source.slice.hako").read_text(encoding="utf-8")
            self.assertIn('__mir__.mark("region_a")', slice_text)

    def test_diff_bundle_reports_changed_keys(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            tmp_path = Path(tmp)
            before = tmp_path / "before"
            after = tmp_path / "after"
            before.mkdir()
            after.mkdir()
            (before / "report.kv").write_text("a=1\nb=2\n", encoding="utf-8")
            (after / "report.kv").write_text("a=1\nb=3\nc=4\n", encoding="utf-8")
            out_dir = tmp_path / "diff"

            rc = inspect_scope_dump.main(
                [
                    "diff",
                    "--before",
                    str(before),
                    "--after",
                    str(after),
                    "--out",
                    str(out_dir),
                ]
            )
            self.assertEqual(rc, 0)
            report = (out_dir / "report.kv").read_text(encoding="utf-8")
            self.assertIn("output_contract=hako-check-inspect-diff-v0", report)
            self.assertIn("changed_count=2", report)
            diff = json.loads((out_dir / "diff.json").read_text(encoding="utf-8"))
            self.assertEqual(diff["changed"][0]["key"], "b")
            self.assertEqual(diff["changed"][0]["before"], "2")
            self.assertEqual(diff["changed"][0]["after"], "3")


if __name__ == "__main__":
    unittest.main()
