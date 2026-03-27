import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from llvm_builder import _sanitize_empty_phi_rows


class TestEmptyPhiSanitize(unittest.TestCase):
    def test_drops_empty_phi_rows_for_any_type(self):
        ir_text = "\n".join(
            [
                'bb1:',
                '  %"phi_strptr_11" = phi  i8*',
                '  %"phi_11" = phi  i64 [%"newbox_string_from_h", %"bb0"], [%"newbox_string_from_h", %"bb3"]',
                '  %"phi_13" = phi  i64',
                '  ret i64 %"phi_11"',
            ]
        )

        sanitized = _sanitize_empty_phi_rows(ir_text)

        self.assertNotIn('%"phi_strptr_11" = phi  i8*', sanitized)
        self.assertNotIn('%"phi_13" = phi  i64', sanitized)
        self.assertIn('%"phi_11" = phi  i64 [%"newbox_string_from_h", %"bb0"], [%"newbox_string_from_h", %"bb3"]', sanitized)

    def test_keeps_non_phi_lines_without_incoming_lists(self):
        ir_text = "\n".join(
            [
                'bb1:',
                '  %"cmp_15" = icmp slt i64 %"phi_9", 5000',
                '  br i1 %"cond_i1", label %"bb2", label %"bb4"',
            ]
        )

        sanitized = _sanitize_empty_phi_rows(ir_text)

        self.assertEqual(ir_text, sanitized)


if __name__ == "__main__":
    unittest.main()
