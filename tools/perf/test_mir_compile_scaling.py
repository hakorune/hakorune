#!/usr/bin/env python3

import unittest

from tools.perf.mir_compile_scaling import TIMING_RE, loop_source, method_source


class MirCompileScalingTests(unittest.TestCase):
    def test_method_source_has_requested_closed_method_set(self) -> None:
        source = method_source(3)
        self.assertEqual(source.count("value_"), 3)
        self.assertNotIn("loop(", source)

    def test_loop_sources_differ_only_at_bound_owner(self) -> None:
        literal = loop_source(False)
        dynamic = loop_source(True)
        self.assertIn("local max = 200000", literal)
        self.assertIn("local max = source.length() + 1", dynamic)

    def test_timing_parser_accepts_elapsed_and_count_rows(self) -> None:
        text = "\n".join(
            [
                "[mir-compile/timing] stage=build_module elapsed_ms=42",
                "[mir-compile/timing] stage=semantic.route.outer_iterations count=2",
            ]
        )
        self.assertEqual(
            [(match.group(1), match.group(2)) for match in TIMING_RE.finditer(text)],
            [("build_module", "42"), ("semantic.route.outer_iterations", "2")],
        )


if __name__ == "__main__":
    unittest.main()
