import unittest
import llvmlite.ir as ir

from src.llvm_py.instructions.by_name_method import (
    _get_or_create_method_global,
    mark_string_result_if_needed,
)


class DummyResolver:
    def __init__(self):
        self.marked = []

    def mark_string(self, vid):
        self.marked.append(vid)


class TestByNameMethod(unittest.TestCase):
    def test_mark_string_result_if_needed_marks_only_string_methods(self):
        resolver = DummyResolver()
        mark_string_result_if_needed(resolver, 7, "esc_json")
        mark_string_result_if_needed(resolver, 8, "push")
        self.assertEqual(resolver.marked, [7])

    def test_method_global_name_is_reused_by_codegen(self):
        mod = ir.Module(name="m")
        first = _get_or_create_method_global(mod, "size")
        second = _get_or_create_method_global(mod, "size")
        self.assertIs(first, second)


if __name__ == "__main__":
    unittest.main()
