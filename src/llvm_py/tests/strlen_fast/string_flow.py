from .support import NyashLLVMBuilder, StrlenFastTestCase


class TestStrlenFastStringFlow(StrlenFastTestCase):
    def test_mir_call_length_receiver_marked_stringish_by_later_substring_use(self):
        # cleanup-11 contract:
        # even when length appears before substring in MIR order, pre-analysis
        # marks the shared receiver as stringish and avoids generic any.length_h.
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [1],
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "mir_call", "dst": 2, "mir_call": {
                                    "callee": {
                                        "type": "Method",
                                        "box_name": "RuntimeDataBox",
                                        "name": "length",
                                        "receiver": 1
                                    },
                                    "args": []
                                }},
                                {"op": "const", "dst": 3, "value": {"type": "i64", "value": 0}},
                                {"op": "const", "dst": 4, "value": {"type": "i64", "value": 2}},
                                {"op": "mir_call", "dst": 5, "mir_call": {
                                    "callee": {
                                        "type": "Method",
                                        "box_name": "RuntimeDataBox",
                                        "name": "substring",
                                        "receiver": 1
                                    },
                                    "args": [3, 4]
                                }},
                                {"op": "ret", "value": 2},
                            ],
                        }
                    ],
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ""
        self.assertNotIn('call i64 @"nyash.any.length_h"', ir_txt, msg=ir_txt)

    def test_mir_call_length_get_result_infers_stringish_from_string_set(self):
        # cleanup-11 contract:
        # RuntimeData set/push with string values marks receiver element kind,
        # so later get-result length avoids generic any.length_h.
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [],
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "newbox", "dst": 1, "type": "ArrayBox", "args": []},
                                {"op": "const", "dst": 2, "value": {"type": "string", "value": "abcd"}},
                                {"op": "const", "dst": 3, "value": {"type": "i64", "value": 0}},
                                {"op": "mir_call", "dst": 4, "mir_call": {
                                    "callee": {
                                        "type": "Method",
                                        "box_name": "RuntimeDataBox",
                                        "name": "set",
                                        "receiver": 1
                                    },
                                    "args": [3, 2]
                                }},
                                {"op": "mir_call", "dst": 5, "mir_call": {
                                    "callee": {
                                        "type": "Method",
                                        "box_name": "RuntimeDataBox",
                                        "name": "get",
                                        "receiver": 1
                                    },
                                    "args": [3]
                                }},
                                {"op": "mir_call", "dst": 6, "mir_call": {
                                    "callee": {
                                        "type": "Method",
                                        "box_name": "RuntimeDataBox",
                                        "name": "length",
                                        "receiver": 5
                                    },
                                    "args": []
                                }},
                                {"op": "ret", "value": 6},
                            ],
                        }
                    ],
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ""
        self.assertNotIn('call i64 @"nyash.any.length_h"', ir_txt, msg=ir_txt)

    def test_boxcall_get_result_seeds_string_ptr_for_substring_concat(self):
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [],
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "newbox", "dst": 1, "type": "ArrayBox", "args": []},
                                {"op": "const", "dst": 2, "value": {"type": "string", "value": "abcdef"}},
                                {"op": "const", "dst": 3, "value": {"type": "i64", "value": 0}},
                                {"op": "boxcall", "dst": 4, "box": 1, "method": "set", "args": [3, 2]},
                                {"op": "boxcall", "dst": 5, "box": 1, "method": "get", "args": [3]},
                                {"op": "const", "dst": 6, "value": {"type": "i64", "value": 2}},
                                {"op": "boxcall", "dst": 7, "box": 5, "method": "substring", "dst_type": {"kind": "handle", "box_type": "StringBox"}, "args": [3, 6]},
                                {"op": "const", "dst": 8, "value": {"type": "string", "value": "xx"}},
                                {"op": "newbox", "dst": 9, "type": "StringBox", "args": [8]},
                                {"op": "binop", "dst": 10, "lhs": 7, "rhs": 9, "operation": "+"},
                                {"op": "ret", "value": 10},
                            ],
                        }
                    ],
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ""
        self.assertIn('call i64 @"nyash.array.slot_load_hi"', ir_txt, msg=ir_txt)
        self.assertIn('call i8* @"nyash.string.to_i8p_h"', ir_txt, msg=ir_txt)
        self.assertIn('call i8* @"nyash.string.substring_sii"', ir_txt, msg=ir_txt)
        self.assertIn('call i8* @"nyash.string.concat_ss"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.string.substring_hii"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.string.concat_hh"', ir_txt, msg=ir_txt)
