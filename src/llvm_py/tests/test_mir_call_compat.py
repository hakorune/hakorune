import os
import sys
import unittest


THIS_DIR = os.path.dirname(__file__)
LLVM_PY_DIR = os.path.abspath(os.path.join(THIS_DIR, ".."))
if LLVM_PY_DIR not in sys.path:
    sys.path.insert(0, LLVM_PY_DIR)

from mir_call_compat import MirCallCompat


class TestMirCallCompat(unittest.TestCase):
    def test_normalize_method_canonical(self):
        callee = {
            "type": "Method",
            "name": "length",
            "box_name": "StringBox",
            "receiver": 1,
        }
        normalized = MirCallCompat.normalize_callee(callee)
        self.assertEqual(normalized["type"], "Method")
        self.assertEqual(normalized["name"], "length")
        self.assertEqual(normalized["box_name"], "StringBox")
        self.assertEqual(normalized["receiver"], 1)

    def test_normalize_constructor_canonical(self):
        callee = {"type": "Constructor", "name": "StringBox"}
        normalized = MirCallCompat.normalize_callee(callee)
        self.assertEqual(normalized, {"type": "Constructor", "name": "StringBox"})

    def test_normalize_value_canonical(self):
        callee = {"type": "Value", "value": 42}
        normalized = MirCallCompat.normalize_callee(callee)
        self.assertEqual(normalized, {"type": "Value", "value": 42})

    def test_detect_format_version_canonical(self):
        self.assertEqual(
            MirCallCompat.detect_format_version({"type": "Method", "name": "x", "box_name": "Y"}),
            1,
        )

    def test_reject_legacy_method_key(self):
        with self.assertRaises(ValueError):
            MirCallCompat.normalize_callee({"type": "Method", "method": "length", "box_name": "StringBox"})

    def test_reject_legacy_box_type_key(self):
        with self.assertRaises(ValueError):
            MirCallCompat.normalize_callee({"type": "Constructor", "box_type": "StringBox"})

    def test_reject_legacy_function_value_key(self):
        with self.assertRaises(ValueError):
            MirCallCompat.normalize_callee({"type": "Value", "function_value": 7})

    def test_reject_legacy_func_key(self):
        with self.assertRaises(ValueError):
            MirCallCompat.detect_format_version({"type": "Value", "func": 7})


if __name__ == "__main__":
    unittest.main()
