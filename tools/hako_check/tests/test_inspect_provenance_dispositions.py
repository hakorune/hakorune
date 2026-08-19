from __future__ import annotations

import sys
import unittest
from copy import deepcopy
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
TOOLS = ROOT / "tools" / "hako_check"
if str(TOOLS) not in sys.path:
    sys.path.insert(0, str(TOOLS))

from inspect_provenance_dispositions import validate_disposition_closure


def row(entity, block, instruction, arm, target, llvm_from, llvm_to,
        disposition, reason, issuer="selected_pinned_text_lowerer"):
    return {
        "entity": entity,
        "mir": {"block": block, "instruction": instruction,
                "arm": arm, "target": target},
        "llvm": {"from": llvm_from, "to": llvm_to},
        "disposition": disposition, "reason_kind": reason, "issuer": issuer,
    }


class InspectProvenanceDispositionsTest(unittest.TestCase):
    def _valid(self):
        rows = [
            row("block", 0, -1, "none", -1, "bb0", "",
                "preserved", "base_block"),
            row("block", 0, 0, "none", -1, "width_a", "",
                "split", "utf8_width_at"),
            row("block", 0, 0, "none", -1, "width_b", "",
                "split", "utf8_width_at"),
            row("edge", 0, 0, "none", -1, "bb0", "width_a",
                "split", "utf8_width_at"),
            row("edge", 0, 0, "none", -1, "width_a", "width_b",
                "split", "utf8_width_at"),
        ]
        args = dict(
            issuer="selected_pinned_text_lowerer", mir_blocks={0},
            mir_sites={(0, 0)}, mir_edges=set(),
            llvm_blocks={"bb0", "width_a", "width_b"},
            llvm_edges={("bb0", "width_a"), ("width_a", "width_b")},
        )
        return rows, args

    def test_preserved_and_instruction_origin_split_close(self):
        rows, args = self._valid()
        validate_disposition_closure(rows, **args)

    def test_unsupported_dispositions_and_reason_reject(self):
        for disposition in ("merged", "deleted", "introduced"):
            rows, args = self._valid()
            rows[1]["disposition"] = disposition
            with self.subTest(disposition=disposition), self.assertRaisesRegex(
                SystemExit, "reason contract mismatch"
            ):
                validate_disposition_closure(rows, **args)
        rows, args = self._valid()
        rows[1]["reason_kind"] = "guessed"
        with self.assertRaisesRegex(SystemExit, "reason is unsupported"):
            validate_disposition_closure(rows, **args)
        rows, args = self._valid()
        rows[0]["issuer"] = "foreign"
        with self.assertRaisesRegex(SystemExit, "issuer mismatch"):
            validate_disposition_closure(rows, **args)

    def test_singleton_and_dangling_split_reject(self):
        rows, args = self._valid()
        rows.pop(2)
        args["llvm_blocks"].remove("width_b")
        rows.pop()
        args["llvm_edges"].remove(("width_a", "width_b"))
        with self.assertRaisesRegex(SystemExit, "split cohort is singleton"):
            validate_disposition_closure(rows, **args)
        rows, args = self._valid()
        rows[1]["mir"]["instruction"] = 9
        with self.assertRaisesRegex(SystemExit, "split site mismatch"):
            validate_disposition_closure(rows, **args)

    def test_preserved_internal_and_duplicate_llvm_owner_reject(self):
        rows, args = self._valid()
        rows[3]["disposition"] = "preserved"
        rows[3]["reason_kind"] = "jump"
        with self.assertRaisesRegex(SystemExit, "preserved edge source mismatch"):
            validate_disposition_closure(rows, **args)
        rows, args = self._valid()
        duplicate = deepcopy(rows[2])
        duplicate["mir"]["instruction"] = 0
        rows.append(duplicate)
        with self.assertRaisesRegex(SystemExit, "LLVM block ownership mismatch"):
            validate_disposition_closure(rows, **args)


if __name__ == "__main__":
    unittest.main()
