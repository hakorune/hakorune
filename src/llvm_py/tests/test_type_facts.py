#!/usr/bin/env python3
import unittest

from src.llvm_py.type_facts import (
    TypeFactsBox,
    is_arrayish_fact,
    is_box_handle_fact,
    is_stringish_fact,
    make_box_handle_fact,
)


class TestTypeFactsHelpers(unittest.TestCase):
    def test_box_handle_fact_helpers(self):
        fact = make_box_handle_fact("StringBox")
        self.assertEqual(fact, {"kind": "handle", "box_type": "StringBox"})
        self.assertTrue(is_box_handle_fact(fact, "StringBox"))
        self.assertFalse(is_box_handle_fact(fact, "ArrayBox"))

    def test_stringish_fact_accepts_legacy_and_handle_forms(self):
        self.assertTrue(is_stringish_fact("string"))
        self.assertTrue(is_stringish_fact({"kind": "string"}))
        self.assertTrue(is_stringish_fact(make_box_handle_fact("StringBox")))
        self.assertFalse(is_stringish_fact(make_box_handle_fact("ArrayBox")))

    def test_arrayish_fact_accepts_handle_forms(self):
        self.assertTrue(is_arrayish_fact("ArrayBox"))
        self.assertTrue(is_arrayish_fact(make_box_handle_fact("ArrayBox")))
        self.assertFalse(is_arrayish_fact(make_box_handle_fact("StringBox")))


class TestTypeFactsBox(unittest.TestCase):
    def test_propagate_phi_marks_only_when_all_inputs_are_stringish(self):
        facts = TypeFactsBox()
        facts.mark_string(1)
        facts.mark_string(2)

        facts.propagate_phi(3, [1, 2])
        self.assertTrue(facts.is_stringish(3))

        facts.propagate_phi(4, [1, 99])
        self.assertFalse(facts.is_stringish(4))


if __name__ == "__main__":
    unittest.main()
