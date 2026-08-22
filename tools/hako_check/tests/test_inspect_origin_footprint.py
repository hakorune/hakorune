from __future__ import annotations

import sys
import unittest
from copy import deepcopy
from pathlib import Path

TOOLS = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(TOOLS))

from inspect_origin_footprint import (
    build_origin_footprint, render_origin_footprint_markdown,
)


LLVM = """define i64 @f() {
bb0:
  %x = load i64, ptr null
  br label %bb1
bb1:
  ret i64 %x
}
"""
ASM = """0000000000000000 <f>:
   0:\t48 89 c0             \tmov    %rax,%rax
   3:\te8 00 00 00 00       \tcall   8 <f+0x8>
   8:\tc3                   \tret
"""


def provenance():
    return {
        "output_contract": "hako-lowering-provenance-v0",
        "candidate_input": {
            "llvm_boundary": "lowered_pre_opt", "llvm_function": "f",
        },
        "relations": [
            {
                "entity": "block", "reason_kind": "base_block",
                "mir": {"block": 0, "instruction": -1},
                "llvm": {"from": "bb0", "to": ""},
            },
            {
                "entity": "edge", "reason_kind": "jump",
                "mir": {"block": 0, "instruction": 1},
                "llvm": {"from": "bb0", "to": "bb1"},
            },
            {
                "entity": "block", "reason_kind": "base_block",
                "mir": {"block": 1, "instruction": -1},
                "llvm": {"from": "bb1", "to": ""},
            },
        ],
    }


class InspectOriginFootprintTest(unittest.TestCase):
    def test_exact_origins_and_symbol_only_asm(self):
        report = build_origin_footprint(
            provenance=provenance(), llvm_text=LLVM, asm_text=ASM, asm_symbol="f",
        )
        self.assertEqual(report["llvm_boundary"], "lowered_pre_opt")
        self.assertEqual(report["lowered_llvm_to_machine"], "unavailable")
        self.assertEqual(report["asm"]["origin_attribution"], "unavailable")
        self.assertEqual(report["asm"]["shape"]["instructions"], 3)
        origins = {
            (row["mir_origin"]["block"], row["mir_origin"]["instruction"]): row
            for row in report["origins"]
        }
        self.assertEqual(origins[(0, -1)]["lowered_llvm"]["shape"]["loads"], 1)
        self.assertEqual(origins[(1, -1)]["lowered_llvm"]["shape"]["returns"], 1)
        summary = render_origin_footprint_markdown(report)
        self.assertIn("| bb0:-1 | base_block | 1 | 0 | 2 | 0 | 1 |", summary)
        self.assertIn("MIR/LLVM origin attribution: unavailable", summary)
        self.assertIn("lowered_pre_opt → machine: unavailable", summary)

    def test_duplicate_or_uncovered_llvm_block_rejects(self):
        value = provenance()
        duplicate = deepcopy(value["relations"][0])
        duplicate["mir"]["block"] = 9
        value["relations"].append(duplicate)
        with self.assertRaisesRegex(SystemExit, "multiple origins"):
            build_origin_footprint(
                provenance=value, llvm_text=LLVM, asm_text=ASM, asm_symbol="f",
            )
        value = provenance()
        value["relations"].pop()
        with self.assertRaisesRegex(SystemExit, "block coverage mismatch"):
            build_origin_footprint(
                provenance=value, llvm_text=LLVM, asm_text=ASM, asm_symbol="f",
            )

    def test_foreign_boundary_or_symbol_rejects(self):
        value = provenance()
        value["candidate_input"]["llvm_boundary"] = "machine"
        with self.assertRaisesRegex(SystemExit, "boundary mismatch"):
            build_origin_footprint(
                provenance=value, llvm_text=LLVM, asm_text=ASM, asm_symbol="f",
            )
        with self.assertRaisesRegex(SystemExit, "must be unique"):
            build_origin_footprint(
                provenance=provenance(), llvm_text=LLVM,
                asm_text="no symbol\n", asm_symbol="f",
            )


if __name__ == "__main__":
    unittest.main()
