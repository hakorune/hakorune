#!/usr/bin/env python3

import unittest

from tools.language_v1.hako_witness_projection import (
    HakoProjectionError,
    project_hako_normalized_form,
)
from tools.language_v1.rust_witness_projection import (
    RustProjectionError,
    project_rust_normalized_form,
)


class WitnessProjectionTests(unittest.TestCase):
    def test_loop_condition_projects_identically(self) -> None:
        rust = {
            "statements": [
                {
                    "kind": "Loop",
                    "condition": {"kind": "Variable", "name": "ready"},
                    "body": [{"kind": "Continue"}],
                }
            ]
        }
        hako = {
            "body": [
                {
                    "type": "Loop",
                    "cond": {"type": "Var", "name": "ready"},
                    "body": [{"type": "Continue"}],
                }
            ]
        }
        expected = {
            "kind": "LoopCondition",
            "children": [
                {"kind": "Condition", "children": []},
                {
                    "kind": "LoopBody",
                    "children": [{"kind": "Continue", "children": []}],
                },
            ],
        }
        self.assertEqual(project_rust_normalized_form("loop_condition", rust), expected)
        self.assertEqual(project_hako_normalized_form("loop_condition", hako), expected)

    def test_map_literal_projects_identically(self) -> None:
        rust = {
            "statements": [
                {
                    "kind": "Map",
                    "entries": [{"k": "key", "v": {"type": "Int", "value": 1}}],
                }
            ]
        }
        hako = {
            "body": [
                {
                    "type": "Expr",
                    "expr": {
                        "type": "Call",
                        "name": "map.of",
                        "args": [
                            {"type": "Str", "value": "key"},
                            {"type": "Int", "value": 1},
                        ],
                    },
                }
            ]
        }
        expected = {
            "kind": "MapLiteral",
            "children": [
                {"kind": "StringKey", "children": []},
                {"kind": "IntegerLiteral", "children": []},
            ],
        }
        self.assertEqual(project_rust_normalized_form("map_literal_percent_brace", rust), expected)
        self.assertEqual(project_hako_normalized_form("map_literal_percent_brace", hako), expected)

    def test_postfix_cleanup_projects_identically(self) -> None:
        rust = {
            "statements": [
                {"kind": "TryCatch", "try": [{}], "catch": [], "cleanup": [{}]}
            ]
        }
        hako = {
            "body": [
                {"type": "Try", "try": [{}], "catches": [], "finally": [{}]}
            ]
        }
        expected = {
            "kind": "PostfixCleanup",
            "children": [
                {"kind": "Body", "children": []},
                {"kind": "CleanupBody", "children": []},
            ],
        }
        self.assertEqual(project_rust_normalized_form("postfix_cleanup", rust), expected)
        self.assertEqual(project_hako_normalized_form("postfix_cleanup", hako), expected)

    def test_hako_guard_requires_structural_no_fallthrough(self) -> None:
        accepted = {
            "body": [
                {
                    "type": "If",
                    "cond": {"type": "Var", "name": "ready"},
                    "then": [],
                    "else": [{"type": "Return", "expr": {"type": "Unit"}}],
                }
            ]
        }
        expected = {
            "kind": "GuardElse",
            "children": [
                {"kind": "Condition", "children": []},
                {"kind": "NoFallthroughElse", "children": []},
            ],
        }
        self.assertEqual(
            project_hako_normalized_form("guard_expr_else", accepted), expected
        )

        fallthrough = {
            "body": [
                {
                    "type": "If",
                    "cond": {"type": "Var", "name": "ready"},
                    "then": [],
                    "else": [{"type": "Expr", "expr": {"type": "Int", "value": 1}}],
                }
            ]
        }
        with self.assertRaises(HakoProjectionError):
            project_hako_normalized_form("guard_expr_else", fallthrough)

    def test_guard_let_projects_independently_and_rejects_fallthrough(self) -> None:
        rust = {
            "statements": [
                {
                    "kind": "ScopeBox",
                    "body": [
                        {"kind": "Local"},
                        {"kind": "If", "then": [{"kind": "Return"}]},
                        {"kind": "Local"},
                    ],
                }
            ]
        }
        hako = {
            "body": [
                {"type": "Local"},
                {
                    "type": "If",
                    "cond": {"type": "EnumMatch"},
                    "then": [{"type": "Return"}],
                },
                {"type": "Local", "expr": {"type": "EnumMatch"}},
            ]
        }
        expected = {
            "kind": "GuardLetElse",
            "children": [
                {"kind": "Pattern", "children": []},
                {"kind": "Expr", "children": []},
                {"kind": "NoFallthroughElse", "children": []},
            ],
        }
        self.assertEqual(project_rust_normalized_form("guard_let_else", rust), expected)
        self.assertEqual(project_hako_normalized_form("guard_let_else", hako), expected)

        rust["statements"][0]["body"][1]["then"] = [{"kind": "FunctionCall"}]
        hako["body"][1]["then"] = [{"type": "Expr"}]
        with self.assertRaises(RustProjectionError) as rust_error:
            project_rust_normalized_form("guard_let_else", rust)
        with self.assertRaises(HakoProjectionError) as hako_error:
            project_hako_normalized_form("guard_let_else", hako)
        self.assertEqual(
            rust_error.exception.stable_reject_tag,
            "parser/guard_let_no_fallthrough_required",
        )
        self.assertEqual(
            hako_error.exception.stable_reject_tag,
            "parser/guard_let_no_fallthrough_required",
        )

    def test_hako_loop_rejects_record_literal_condition(self) -> None:
        malformed = {
            "body": [
                {
                    "type": "Loop",
                    "cond": {"type": "RecordLiteral"},
                    "body": [{"type": "Break"}],
                }
            ]
        }
        with self.assertRaises(HakoProjectionError):
            project_hako_normalized_form("loop_condition", malformed)

    def test_unknown_rows_fail_instead_of_copying_expected_form(self) -> None:
        with self.assertRaises(RustProjectionError):
            project_rust_normalized_form("unknown", {"statements": []})
        with self.assertRaises(HakoProjectionError):
            project_hako_normalized_form("unknown", {"body": []})


if __name__ == "__main__":
    unittest.main()
