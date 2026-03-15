#!/usr/bin/env python3
import unittest

from src.llvm_py.mir_analysis import scan_call_arities


class TestMirAnalysis(unittest.TestCase):
    def test_scan_call_arities_collects_max_arity_per_name(self):
        funcs = [
            {
                "blocks": [
                    {
                        "instructions": [
                            {
                                "op": "const",
                                "dst": 1,
                                "value": {"type": "string", "value": "print"},
                            },
                            {
                                "op": "call",
                                "func": 1,
                                "args": [10],
                            },
                            {
                                "op": "call",
                                "func": 1,
                                "args": [10, 11],
                            },
                        ]
                    }
                ]
            }
        ]

        self.assertEqual(scan_call_arities(funcs), {"print": 2})

    def test_scan_call_arities_accepts_stringbox_handle_metadata(self):
        funcs = [
            {
                "blocks": [
                    {
                        "instructions": [
                            {
                                "op": "const",
                                "dst": 2,
                                "value": {
                                    "type": {"kind": "handle", "box_type": "StringBox"},
                                    "value": "emit",
                                },
                            },
                            {
                                "op": "call",
                                "func": 2,
                                "args": [],
                            },
                        ]
                    }
                ]
            }
        ]

        self.assertEqual(scan_call_arities(funcs), {"emit": 0})

    def test_scan_call_arities_ignores_non_string_constants(self):
        funcs = [
            {
                "blocks": [
                    {
                        "instructions": [
                            {
                                "op": "const",
                                "dst": 3,
                                "value": {"type": "i64", "value": 7},
                            },
                            {
                                "op": "call",
                                "func": 3,
                                "args": [1, 2, 3],
                            },
                        ]
                    }
                ]
            }
        ]

        self.assertEqual(scan_call_arities(funcs), {})


if __name__ == "__main__":
    unittest.main()
