from __future__ import annotations

import subprocess
import sys
import tempfile
from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parents[3]
TOOL = ROOT / "tools" / "hako_check" / "callsite_canonicalize_entry_inventory.py"


class CallsiteCanonicalizeEntryInventoryTest(unittest.TestCase):
    def test_reports_known_production_entries_and_ignores_tests(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            repo = Path(tmp)
            self._write(
                repo / "src" / "mir" / "compiler" / "mod.rs",
                "super::passes::callsite_canonicalize::canonicalize_for_site(&mut module, Site::MirCompilerPostRc);\n",
            )
            self._write(
                repo / "src" / "mir" / "optimizer" / "core.rs",
                "crate::mir::passes::callsite_canonicalize::canonicalize_for_site(module, Site::MirOptimizerLateCallAndInline);\n",
            )
            self._write(
                repo / "src" / "runner" / "json_v0_bridge" / "core.rs",
                "crate::mir::passes::callsite_canonicalize::canonicalize_for_site(&mut module, Site::ProgramJsonV0Bridge);\n",
            )
            self._write(
                repo / "src" / "runner" / "mir_json_v0.rs",
                "crate::mir::passes::callsite_canonicalize::canonicalize_for_site(&mut module, Site::MirJsonV0Loader);\n",
            )
            self._write(
                repo / "src" / "mir" / "passes" / "callsite_canonicalize" / "tests" / "mcl.rs",
                "let rewritten = canonicalize_callsites(&mut module);\n",
            )

            rows = self._run_tool(repo)

        self.assertEqual(rows["output_contract"], "hako-callsite-canonicalize-entry-inventory-v0")
        self.assertEqual(rows["production_entry_count"], "4")
        self.assertEqual(rows["known_entry_count"], "4")
        self.assertEqual(rows["unknown_entry_count"], "0")
        self.assertEqual(rows["mir_compiler_entry"], "1")
        self.assertEqual(rows["mir_optimizer_entry"], "1")
        self.assertEqual(rows["program_json_v0_bridge_entry"], "1")
        self.assertEqual(rows["mir_json_v0_loader_entry"], "1")
        self.assertEqual(rows["single_transform_owner"], "1")
        self.assertEqual(rows["centralized_schedule_owner"], "1")
        self.assertEqual(rows["behavior_changed"], "0")
        self.assertEqual(rows["canonicalize_entry_refactor_allowed"], "0")
        self.assertEqual(rows["entry_removal_enabled"], "0")
        self.assertEqual(rows["schedule_reorder_enabled"], "0")
        self.assertEqual(rows["entry_0_call_kind"], "schedule_facade")

    def test_unknown_production_entry_is_visible(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            repo = Path(tmp)
            self._write(
                repo / "src" / "somewhere" / "else.rs",
                "crate::mir::passes::callsite_canonicalize::canonicalize_for_site(&mut module, Site::Unknown);\n",
            )

            rows = self._run_tool(repo)

        self.assertEqual(rows["production_entry_count"], "1")
        self.assertEqual(rows["known_entry_count"], "0")
        self.assertEqual(rows["unknown_entry_count"], "1")
        self.assertEqual(rows["entry_0_entry_kind"], "unknown")
        self.assertEqual(rows["centralized_schedule_owner"], "0")

    def _run_tool(self, repo: Path) -> dict[str, str]:
        result = subprocess.run(
            [sys.executable, str(TOOL), "--repo-root", str(repo)],
            check=True,
            capture_output=True,
            text=True,
        )
        return dict(line.split("=", 1) for line in result.stdout.splitlines() if "=" in line)

    @staticmethod
    def _write(path: Path, text: str) -> None:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")


if __name__ == "__main__":
    unittest.main()
