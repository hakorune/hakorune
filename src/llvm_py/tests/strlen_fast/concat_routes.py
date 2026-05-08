from .support import NyashLLVMBuilder, StrlenFastTestCase


class TestStrlenFastConcatRoutes(StrlenFastTestCase):
    def test_binop_string_concat_chain_prefers_pointer_concat_ss(self):
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [],
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "const", "dst": 1, "value": {"type": "string", "value": "ha"}},
                                {"op": "newbox", "dst": 2, "type": "StringBox", "args": [1]},
                                {"op": "const", "dst": 3, "value": {"type": "string", "value": "ko"}},
                                {"op": "newbox", "dst": 4, "type": "StringBox", "args": [3]},
                                {"op": "binop", "dst": 5, "lhs": 2, "rhs": 4, "operation": "+"},
                                {"op": "const", "dst": 6, "value": {"type": "string", "value": "run"}},
                                {"op": "newbox", "dst": 7, "type": "StringBox", "args": [6]},
                                {"op": "binop", "dst": 8, "lhs": 5, "rhs": 7, "operation": "+"},
                                {"op": "ret", "value": 8},
                            ],
                        }
                    ],
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ''
        self.assertIn('call i8* @"nyash.string.concat_ss"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.string.concat3_hhh"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.string.concat_hh"', ir_txt, msg=ir_txt)

    def test_binop_string_concat_chain_right_assoc_prefers_pointer_concat_ss(self):
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [],
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "const", "dst": 1, "value": {"type": "string", "value": "ha"}},
                                {"op": "newbox", "dst": 2, "type": "StringBox", "args": [1]},
                                {"op": "const", "dst": 3, "value": {"type": "string", "value": "ko"}},
                                {"op": "newbox", "dst": 4, "type": "StringBox", "args": [3]},
                                {"op": "const", "dst": 5, "value": {"type": "string", "value": "run"}},
                                {"op": "newbox", "dst": 6, "type": "StringBox", "args": [5]},
                                {"op": "binop", "dst": 7, "lhs": 4, "rhs": 6, "operation": "+"},
                                {"op": "binop", "dst": 8, "lhs": 2, "rhs": 7, "operation": "+"},
                                {"op": "ret", "value": 8},
                            ],
                        }
                    ],
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ''
        self.assertIn('call i8* @"nyash.string.concat_ss"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.string.concat3_hhh"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.string.concat_hh"', ir_txt, msg=ir_txt)

    def test_binop_string_concat_non_chain_prefers_pointer_concat_ss(self):
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [],
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "const", "dst": 1, "value": {"type": "string", "value": "ha"}},
                                {"op": "newbox", "dst": 2, "type": "StringBox", "args": [1]},
                                {"op": "const", "dst": 3, "value": {"type": "string", "value": "ko"}},
                                {"op": "newbox", "dst": 4, "type": "StringBox", "args": [3]},
                                {"op": "binop", "dst": 5, "lhs": 2, "rhs": 4, "operation": "+"},
                                {"op": "ret", "value": 5},
                            ],
                        }
                    ],
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ''
        self.assertIn('call i8* @"nyash.string.concat_ss"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.string.concat_hh"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.string.concat3_hhh"', ir_txt, msg=ir_txt)

    def test_fast_substring_concat_chain_prefers_pointer_route(self):
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [],
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "const", "dst": 1, "value": {"type": "string", "value": "line-seed-abcdef"}},
                                {"op": "newbox", "dst": 2, "type": "StringBox", "args": [1]},
                                {"op": "const", "dst": 3, "value": {"type": "i64", "value": 0}},
                                {"op": "const", "dst": 4, "value": {"type": "i64", "value": 8}},
                                {"op": "const", "dst": 5, "value": {"type": "i64", "value": 16}},
                                {"op": "boxcall", "dst": 6, "box": 2, "method": "substring", "args": [3, 4]},
                                {"op": "boxcall", "dst": 7, "box": 2, "method": "substring", "args": [4, 5]},
                                {"op": "const", "dst": 8, "value": {"type": "string", "value": "xx"}},
                                {"op": "newbox", "dst": 9, "type": "StringBox", "args": [8]},
                                {"op": "binop", "dst": 10, "lhs": 6, "rhs": 9, "operation": "+"},
                                {"op": "binop", "dst": 11, "lhs": 10, "rhs": 7, "operation": "+"},
                                {"op": "ret", "value": 11},
                            ],
                        }
                    ],
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ''
        self.assertIn('call i8* @"nyash.string.substring_sii"', ir_txt, msg=ir_txt)
        self.assertIn('call i64 @"nyash.box.from_i8_string"', ir_txt, msg=ir_txt)
        self.assertIn('call i8* @"nyash.string.concat_ss"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.string.substring_hii"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.string.concat_hh"', ir_txt, msg=ir_txt)

    def test_fast_substring_concat_copy_chain_keeps_pointer_route(self):
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [],
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "const", "dst": 1, "value": {"type": "string", "value": "line-seed-abcdef"}},
                                {"op": "newbox", "dst": 2, "type": "StringBox", "args": [1]},
                                {"op": "copy", "dst": 3, "src": 2},
                                {"op": "const", "dst": 4, "value": {"type": "i64", "value": 0}},
                                {"op": "const", "dst": 5, "value": {"type": "i64", "value": 8}},
                                {"op": "const", "dst": 6, "value": {"type": "i64", "value": 16}},
                                {"op": "boxcall", "dst": 7, "box": 3, "method": "substring", "args": [4, 5]},
                                {"op": "boxcall", "dst": 8, "box": 3, "method": "substring", "args": [5, 6]},
                                {"op": "const", "dst": 9, "value": {"type": "string", "value": "xx"}},
                                {"op": "newbox", "dst": 10, "type": "StringBox", "args": [9]},
                                {"op": "binop", "dst": 11, "lhs": 7, "rhs": 10, "operation": "+"},
                                {"op": "binop", "dst": 12, "lhs": 11, "rhs": 8, "operation": "+"},
                                {"op": "ret", "value": 12},
                            ],
                        }
                    ],
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ''
        self.assertIn('call i8* @"nyash.string.substring_sii"', ir_txt, msg=ir_txt)
        self.assertIn('call i8* @"nyash.string.concat_ss"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.string.substring_hii"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.string.concat_hh"', ir_txt, msg=ir_txt)

    def test_mircall_substring_concat_prefers_pointer_route(self):
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [],
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "const", "dst": 1, "value": {"type": {"kind": "handle", "box_type": "StringBox"}, "value": "line-seed-abcdef"}},
                                {"op": "newbox", "dst": 2, "type": "StringBox", "args": [1]},
                                {"op": "copy", "dst": 3, "src": 2},
                                {"op": "const", "dst": 4, "value": {"type": "i64", "value": 0}},
                                {"op": "const", "dst": 5, "value": {"type": "i64", "value": 8}},
                                {"op": "const", "dst": 6, "value": {"type": "i64", "value": 16}},
                                {"op": "mir_call", "dst": 7, "mir_call": {
                                    "callee": {
                                        "type": "Method",
                                        "box_name": "RuntimeDataBox",
                                        "name": "substring",
                                        "receiver": 3,
                                        "certainty": "Union"
                                    },
                                    "args": [4, 5]
                                }},
                                {"op": "mir_call", "dst": 8, "mir_call": {
                                    "callee": {
                                        "type": "Method",
                                        "box_name": "RuntimeDataBox",
                                        "name": "substring",
                                        "receiver": 3,
                                        "certainty": "Union"
                                    },
                                    "args": [5, 6]
                                }},
                                {"op": "const", "dst": 9, "value": {"type": {"kind": "handle", "box_type": "StringBox"}, "value": "xx"}},
                                {"op": "newbox", "dst": 10, "type": "StringBox", "args": [9]},
                                {"op": "binop", "dst": 11, "lhs": 7, "rhs": 10, "operation": "+"},
                                {"op": "binop", "dst": 12, "lhs": 11, "rhs": 8, "operation": "+"},
                                {"op": "ret", "value": 12},
                            ],
                        }
                    ],
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ''
        self.assertIn('call i8* @"nyash.string.substring_sii"', ir_txt, msg=ir_txt)
        self.assertIn('call i8* @"nyash.string.concat_ss"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.string.substring_hii"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.string.concat3_hhh"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.string.concat_hh"', ir_txt, msg=ir_txt)

    def test_mir_call_length_phi_self_carry_uses_fast_path(self):
        # Bench-like shape: receiver goes through loop PHI self-carry.
        # Contract: FAST lowering must avoid any.length_h and safepoint in this route.
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [{"id": 1, "name": "arg0", "type": "i64"}],
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "const", "dst": 2, "value": {"type": "i64", "value": 0}},
                                {"op": "const", "dst": 3, "value": {"type": {"kind": "handle", "box_type": "StringBox"}, "value": "nyash"}},
                                {"op": "newbox", "dst": 4, "type": "StringBox", "args": [3]},
                                {"op": "const", "dst": 5, "value": {"type": "i64", "value": 0}},
                                {"op": "copy", "dst": 6, "src": 1},
                                {"op": "copy", "dst": 8, "src": 2},
                                {"op": "copy", "dst": 10, "src": 4},
                                {"op": "copy", "dst": 12, "src": 5},
                                {"op": "jump", "target": 1},
                            ],
                        },
                        {
                            "id": 1,
                            "instructions": [
                                {"op": "phi", "dst": 7, "incoming": [[6, 0], [7, 3]]},
                                {"op": "phi", "dst": 9, "incoming": [[8, 0], [19, 3]]},
                                {"op": "phi", "dst": 11, "incoming": [[10, 0], [11, 3]]},
                                {"op": "phi", "dst": 13, "incoming": [[12, 0], [17, 3]]},
                                {"op": "const", "dst": 14, "value": {"type": "i64", "value": 1000}},
                                {"op": "compare", "dst": 15, "lhs": 9, "rhs": 14, "operation": "<"},
                                {"op": "branch", "cond": 15, "then": 2, "else": 4},
                            ],
                        },
                        {
                            "id": 2,
                            "instructions": [
                                {"op": "mir_call", "dst": 16, "mir_call": {
                                    "callee": {
                                        "type": "Method",
                                        "box_name": "RuntimeDataBox",
                                        "name": "length",
                                        "receiver": 11
                                    },
                                    "args": []
                                }},
                                {"op": "binop", "dst": 17, "lhs": 13, "rhs": 16, "operation": "+"},
                                {"op": "const", "dst": 18, "value": {"type": "i64", "value": 1}},
                                {"op": "binop", "dst": 19, "lhs": 9, "rhs": 18, "operation": "+"},
                                {"op": "jump", "target": 3},
                            ],
                        },
                        {"id": 3, "instructions": [{"op": "jump", "target": 1}]},
                        {"id": 4, "instructions": [{"op": "ret", "value": 13}]},
                    ],
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ''
        self.assertNotIn('call i64 @"nyash.any.length_h"', ir_txt)
        self.assertNotIn('call void @"ny_check_safepoint"', ir_txt)
        self.assertTrue(
            ('call i64 @"nyash.string.len_h"' in ir_txt)
            or ('call i64 @"nyrt_string_length"' in ir_txt),
            msg=ir_txt,
        )

    def test_substring_concat_phi_self_carry_keeps_pointer_route(self):
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [],
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "const", "dst": 1, "value": {"type": {"kind": "handle", "box_type": "StringBox"}, "value": "line-seed-abcdef"}},
                                {"op": "newbox", "dst": 2, "type": "StringBox", "args": [1]},
                                {"op": "const", "dst": 3, "value": {"type": "i64", "value": 0}},
                                {"op": "const", "dst": 4, "value": {"type": "i64", "value": 16}},
                                {"op": "copy", "dst": 5, "src": 3},
                                {"op": "copy", "dst": 6, "src": 3},
                                {"op": "copy", "dst": 7, "src": 2},
                                {"op": "jump", "target": 1},
                            ],
                        },
                        {
                            "id": 1,
                            "instructions": [
                                {"op": "phi", "dst": 8, "incoming": [[5, 0], [27, 3]]},
                                {"op": "phi", "dst": 9, "incoming": [[6, 0], [25, 3]]},
                                {"op": "phi", "dst": 10, "incoming": [[4, 0], [10, 3]]},
                                {"op": "phi", "dst": 11, "incoming": [[7, 0], [29, 3]]},
                                {"op": "const", "dst": 12, "value": {"type": "i64", "value": 4}},
                                {"op": "compare", "dst": 13, "lhs": 8, "rhs": 12, "operation": "<"},
                                {"op": "branch", "cond": 13, "then": 2, "else": 4},
                            ],
                        },
                        {
                            "id": 2,
                            "instructions": [
                                {"op": "const", "dst": 14, "value": {"type": "i64", "value": 0}},
                                {"op": "const", "dst": 15, "value": {"type": "i64", "value": 2}},
                                {"op": "const", "dst": 16, "value": {"type": {"kind": "handle", "box_type": "StringBox"}, "value": "xx"}},
                                {"op": "newbox", "dst": 17, "type": "StringBox", "args": [16]},
                                {"op": "binop", "dst": 18, "lhs": 10, "rhs": 15, "operation": "/"},
                                {"op": "boxcall", "dst": 19, "box": 11, "method": "substring", "dst_type": {"kind": "handle", "box_type": "StringBox"}, "args": [14, 18]},
                                {"op": "boxcall", "dst": 20, "box": 11, "method": "substring", "dst_type": {"kind": "handle", "box_type": "StringBox"}, "args": [18, 10]},
                                {"op": "binop", "dst": 21, "lhs": 19, "rhs": 17, "operation": "+"},
                                {"op": "binop", "dst": 22, "lhs": 21, "rhs": 20, "operation": "+"},
                                {"op": "boxcall", "dst": 23, "box": 22, "method": "length", "args": []},
                                {"op": "binop", "dst": 24, "lhs": 9, "rhs": 23, "operation": "+"},
                                {"op": "const", "dst": 26, "value": {"type": "i64", "value": 1}},
                                {"op": "binop", "dst": 27, "lhs": 8, "rhs": 26, "operation": "+"},
                                {"op": "binop", "dst": 28, "lhs": 10, "rhs": 26, "operation": "+"},
                                {"op": "boxcall", "dst": 29, "box": 22, "method": "substring", "dst_type": {"kind": "handle", "box_type": "StringBox"}, "args": [26, 28]},
                                {"op": "copy", "dst": 25, "src": 24},
                                {"op": "jump", "target": 3},
                            ],
                        },
                        {"id": 3, "instructions": [{"op": "jump", "target": 1}]},
                        {"id": 4, "instructions": [{"op": "ret", "value": 9}]},
                    ],
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ""
        self.assertIn('call i8* @"nyash.string.substring_sii"', ir_txt, msg=ir_txt)
        self.assertIn('call i8* @"nyash.string.concat_ss"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.string.substring_hii"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.string.concat_hh"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.string.concat3_hhh"', ir_txt, msg=ir_txt)

