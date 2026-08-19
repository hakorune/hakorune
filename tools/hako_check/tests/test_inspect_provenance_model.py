from __future__ import annotations

import json
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
TOOLS = ROOT / "tools" / "hako_check"
if str(TOOLS) not in sys.path:
    sys.path.insert(0, str(TOOLS))

from inspect_provenance_model import build_provenance, parse_raw_events


class InspectProvenanceModelTest(unittest.TestCase):
    def _artifacts(self, root: Path) -> tuple[Path, Path, Path]:
        mir = root / "mir.json"
        mir.write_text(
            json.dumps(
                {
                    "functions": [
                        {
                            "name": "Main.f/0",
                            "blocks": [
                                {
                                    "id": 0,
                                    "instructions": [
                                        {"op": "branch", "cond": 1, "then": 1, "else": 2}
                                    ],
                                },
                                {"id": 1, "instructions": [{"op": "ret", "value": 2}]},
                                {"id": 2, "instructions": [{"op": "ret", "value": 3}]},
                            ],
                        }
                    ]
                },
                sort_keys=True,
            )
            + "\n",
            encoding="utf-8",
        )
        llvm = root / "final.ll"
        llvm.write_text(
            "define i64 @f() {\nbb0:\n  br i1 true, label %bb1, label %bb2\n"
            "bb1:\n  ret i64 1\nbb2:\n  ret i64 2\n}\n",
            encoding="utf-8",
        )
        raw = root / "raw.tsv"
        raw.write_text(
            "block\t0\t-1\tnone\t-1\tbb0\t\tpreserved\tbase_block\n"
            "block\t1\t-1\tnone\t-1\tbb1\t\tpreserved\tbase_block\n"
            "block\t2\t-1\tnone\t-1\tbb2\t\tpreserved\tbase_block\n"
            "edge\t0\t0\tthen\t1\tbb0\tbb1\tpreserved\tbranch\n"
            "edge\t0\t0\telse\t2\tbb0\tbb2\tpreserved\tbranch\n",
            encoding="utf-8",
        )
        return mir, llvm, raw

    def test_exact_issuer_rows_cover_both_cfgs(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            mir, llvm, raw = self._artifacts(Path(tmp))
            report = build_provenance(
                raw_path=raw, mir_path=mir, llvm_path=llvm,
                mir_function="Main.f/0", llvm_function="f",
            )
            self.assertEqual(
                report["coverage"],
                {"mir_blocks": 3, "mir_edges": 2, "llvm_blocks": 3, "llvm_edges": 2},
            )
            self.assertEqual(report["asm"]["correspondence"], "unavailable")
            self.assertEqual(report["candidate_input"]["llvm_boundary"], "final")

    def test_missing_mir_edge_and_unissued_llvm_edge_reject(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            mir, llvm, raw = self._artifacts(Path(tmp))
            rows = raw.read_text(encoding="utf-8").splitlines()
            raw.write_text("\n".join(rows[:-1]) + "\n", encoding="utf-8")
            with self.assertRaisesRegex(SystemExit, "MIR coverage mismatch"):
                build_provenance(
                    raw_path=raw, mir_path=mir, llvm_path=llvm,
                    mir_function="Main.f/0", llvm_function="f",
                )

    def test_foreign_endpoint_and_vocabulary_reject(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            mir, llvm, raw = self._artifacts(Path(tmp))
            text = raw.read_text(encoding="utf-8").replace("bb0\tbb1", "bb0\tforeign")
            raw.write_text(text, encoding="utf-8")
            with self.assertRaisesRegex(SystemExit, "final LLVM coverage mismatch"):
                build_provenance(
                    raw_path=raw, mir_path=mir, llvm_path=llvm,
                    mir_function="Main.f/0", llvm_function="f",
                )
            raw.write_text("block\t0\t-1\tnone\t-1\tbb0\t\tguessed\tbad\n")
            with self.assertRaisesRegex(SystemExit, "vocabulary mismatch"):
                parse_raw_events(raw)

    def test_malformed_or_empty_journal_rejects(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "raw.tsv"
            path.write_text("", encoding="utf-8")
            with self.assertRaisesRegex(SystemExit, "journal is empty"):
                parse_raw_events(path)
            path.write_text("too\tfew\n", encoding="utf-8")
            with self.assertRaisesRegex(SystemExit, "9 fields"):
                parse_raw_events(path)

    def test_checked_callout_edges_and_explicit_lowered_issuer(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            mir, llvm, raw = self._artifacts(root)
            value = json.loads(mir.read_text(encoding="utf-8"))
            value["functions"][0]["blocks"][0]["instructions"][0] = {
                "op": "checked_callout", "normal": 1, "fault": 2,
            }
            mir.write_text(json.dumps(value), encoding="utf-8")
            raw.write_text(
                raw.read_text(encoding="utf-8")
                .replace("then\t1", "normal\t1")
                .replace("else\t2", "fault\t2")
                .replace("\tbranch\n", "\tchecked_callout\n"),
                encoding="utf-8",
            )
            report = build_provenance(
                raw_path=raw, mir_path=mir, llvm_path=llvm,
                mir_function="Main.f/0", llvm_function="f",
                issuer="selected_dynamic_c1_lowerer",
                llvm_boundary="lowered_pre_opt",
            )
            self.assertEqual(
                {row["issuer"] for row in report["relations"]},
                {"selected_dynamic_c1_lowerer"},
            )
            self.assertEqual(
                report["candidate_input"]["llvm_boundary"], "lowered_pre_opt",
            )

    def test_duplicate_and_endpoint_disposition_drift_reject(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            _mir, _llvm, raw = self._artifacts(Path(tmp))
            first = raw.read_text(encoding="utf-8").splitlines()[0]
            raw.write_text(first + "\n" + first + "\n", encoding="utf-8")
            with self.assertRaisesRegex(SystemExit, "duplicated"):
                parse_raw_events(raw)
            raw.write_text(
                "block\t0\t-1\tnone\t-1\tbb0\t\tdeleted\tbad\n",
                encoding="utf-8",
            )
            with self.assertRaisesRegex(SystemExit, "deleted endpoint mismatch"):
                parse_raw_events(raw)

    def test_conflicting_target_ownership_rejects(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            mir, llvm, raw = self._artifacts(Path(tmp))
            raw.write_text(
                raw.read_text(encoding="utf-8")
                + "block\t0\t0\tnone\t-1\tbb0\t\tsplit\tsecond_owner\n",
                encoding="utf-8",
            )
            with self.assertRaisesRegex(SystemExit, "ownership is duplicated"):
                build_provenance(
                    raw_path=raw, mir_path=mir, llvm_path=llvm,
                    mir_function="Main.f/0", llvm_function="f",
                )


if __name__ == "__main__":
    unittest.main()
