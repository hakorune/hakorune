#!/usr/bin/env python3
import unittest

from src.llvm_py.phi_wiring.wiring import _propagate_finalized_phi_facts


class DummyResolver:
    def __init__(self):
        self.stringish_ids = set()
        self.marked_strings = []
        self.array_ids = set()
        self.value_types = {}
        self.newbox_string_args = {}
        self.string_literals = {}

    def is_stringish(self, vid: int) -> bool:
        return int(vid) in self.stringish_ids

    def mark_string(self, vid: int) -> None:
        self.marked_strings.append(int(vid))


class DummyBuilder:
    def __init__(self):
        self.resolver = DummyResolver()


class TestPhiWiringFinalize(unittest.TestCase):
    def test_propagates_post_wire_tags_and_origin_maps(self):
        builder = DummyBuilder()
        builder.resolver.stringish_ids.add(7)
        builder.resolver.array_ids.add(8)
        builder.resolver.value_types[8] = {"kind": "handle", "box_type": "ArrayBox"}
        builder.resolver.newbox_string_args[7] = 123
        builder.resolver.string_literals[7] = "hello"

        _propagate_finalized_phi_facts(builder, 100, [(1, 7), (2, 8), (3, 100)])

        self.assertEqual(builder.resolver.marked_strings, [100])
        self.assertIn(100, builder.resolver.array_ids)
        self.assertEqual(
            builder.resolver.value_types[100],
            {"kind": "handle", "box_type": "ArrayBox"},
        )
        self.assertEqual(builder.resolver.newbox_string_args[100], 123)
        self.assertEqual(builder.resolver.string_literals[100], "hello")

    def test_conflicting_origin_maps_do_not_propagate(self):
        builder = DummyBuilder()
        builder.resolver.newbox_string_args.update({7: 123, 8: 456})
        builder.resolver.string_literals.update({7: "left", 8: "right"})

        _propagate_finalized_phi_facts(builder, 101, [(1, 7), (2, 8)])

        self.assertNotIn(101, builder.resolver.newbox_string_args)
        self.assertNotIn(101, builder.resolver.string_literals)


if __name__ == "__main__":
    unittest.main()
