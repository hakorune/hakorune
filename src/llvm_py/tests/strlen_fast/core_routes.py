import os

from .support import NyashLLVMBuilder, StrlenFastTestCase


class TestStrlenFastCore(StrlenFastTestCase):
    def test_newbox_string_length_fast_lowering(self):
        # Minimal MIR JSON v0-like: const "hello" → newbox StringBox(arg) → boxcall length
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [],
                    "blocks": [
                        {
                            "id": 1,
                            "instructions": [
                                {"op": "const", "dst": 10, "value": {"type": "string", "value": "hello"}},
                                {"op": "newbox", "dst": 20, "type": "StringBox", "args": [10]},
                                {"op": "boxcall", "dst": 30, "box": 20, "method": "length", "args": []},
                                {"op": "ret", "value": 30},
                            ]
                        }
                    ]
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir)
        # FAST lowering accepts either:
        # - literal fold to constant (ret i64 5), or
        # - nyash.string.len_h / nyrt_string_length helper path.
        self.assertTrue(
            ('ret i64 5' in ir_txt)
            or ('call i64 @"nyash.string.len_h"' in ir_txt)
            or ('call i64 @"nyrt_string_length"' in ir_txt),
            msg=ir_txt,
        )

    def test_newbox_string_constructor_avoids_env_box_new_i64x(self):
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [],
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "const", "dst": 1, "value": {"type": "string", "value": "x"}},
                                {"op": "newbox", "dst": 2, "type": "StringBox", "args": [1]},
                                {"op": "ret", "value": 2},
                            ]
                        }
                    ]
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ''
        self.assertIn('@"nyash.box.from_i8_string_const"', ir_txt)
        self.assertNotIn('@"nyash.env.box.new_i64x"', ir_txt)
        self.assertNotIn('@"nyash.string.to_i8p_h"', ir_txt)

    def test_const_string_uses_const_intern_helper_when_fast(self):
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [],
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "const", "dst": 1, "value": {"type": "string", "value": "k"}},
                                {"op": "ret", "value": 1},
                            ]
                        }
                    ]
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ''
        self.assertIn('@"nyash.box.from_i8_string_const"', ir_txt)
        self.assertNotIn('@"nyash.box.from_i8_string"(i8*', ir_txt)

    def test_mir_call_length_uses_fast_string_len_route_when_stringish(self):
        # MIR Call(Method) route (used by benches) should lower to a fast string-length helper.
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [],
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "const", "dst": 3, "value": {"type": "string", "value": "nyash"}},
                                {"op": "newbox", "dst": 4, "type": "StringBox", "args": [3]},
                                {"op": "mir_call", "dst": 5, "mir_call": {
                                    "callee": {
                                        "type": "Method",
                                        "box_name": "RuntimeDataBox",
                                        "name": "length",
                                        "receiver": 4
                                    },
                                    "args": []
                                }},
                                {"op": "ret", "value": 5},
                            ]
                        }
                    ]
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ''
        self.assertTrue(
            ('call i64 @"nyash.string.len_h"' in ir_txt)
            or ('call i64 @"nyrt_string_length"' in ir_txt)
            or ('ret i64 5' in ir_txt),
            msg=ir_txt,
        )
        self.assertNotIn('call i64 @"nyash.any.length_h"', ir_txt)
        self.assertNotIn('call void @"ny_check_safepoint"', ir_txt)

    def test_mir_call_size_uses_fast_strlen_when_stringish(self):
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [],
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "const", "dst": 3, "value": {"type": "string", "value": "nyash"}},
                                {"op": "newbox", "dst": 4, "type": "StringBox", "args": [3]},
                                {"op": "mir_call", "dst": 5, "mir_call": {
                                    "callee": {
                                        "type": "Method",
                                        "box_name": "RuntimeDataBox",
                                        "name": "size",
                                        "receiver": 4
                                    },
                                    "args": []
                                }},
                                {"op": "ret", "value": 5},
                            ]
                        }
                    ]
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ''
        self.assertTrue(
            'call i64 @"nyrt_string_length"' in ir_txt or 'ret i64 5' in ir_txt,
            msg=ir_txt,
        )
        self.assertNotIn('call i64 @"nyash.any.length_h"', ir_txt)

    def test_mir_call_size_stringish_prefers_len_h_when_fast_off(self):
        # AutoSpecialize v0 contract: when FAST is off but receiver is stringish,
        # prefer nyash.string.len_h over generic any.length_h.
        os.environ['NYASH_LLVM_FAST'] = '0'
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [],
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "const", "dst": 3, "value": {"type": "string", "value": "nyash"}},
                                {"op": "newbox", "dst": 4, "type": "StringBox", "args": [3]},
                                {"op": "mir_call", "dst": 5, "mir_call": {
                                    "callee": {
                                        "type": "Method",
                                        "box_name": "RuntimeDataBox",
                                        "name": "size",
                                        "receiver": 4
                                    },
                                    "args": []
                                }},
                                {"op": "ret", "value": 5},
                            ]
                        }
                    ]
                }
            ]
        }
        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ''
        self.assertIn('call i64 @"nyash.string.len_h"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.any.length_h"', ir_txt)
        # Restore default for the rest of this test class.
        os.environ['NYASH_LLVM_FAST'] = '1'

    def test_mir_call_size_arrayish_prefers_array_len_h(self):
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [1],
                    "metadata": {
                        "value_types": {
                            "1": {"kind": "handle", "box_type": "ArrayBox"},
                        }
                    },
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "mir_call", "dst": 2, "mir_call": {
                                    "callee": {
                                        "type": "Method",
                                        "box_name": "RuntimeDataBox",
                                        "name": "size",
                                        "receiver": 1
                                    },
                                    "args": []
                                }},
                                {"op": "ret", "value": 2},
                            ],
                        }
                    ],
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ''
        self.assertIn('call i64 @"nyash.array.slot_len_h"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.any.length_h"', ir_txt, msg=ir_txt)

    def test_boxcall_size_stringish_prefers_string_len_h(self):
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [1],
                    "metadata": {
                        "value_types": {
                            "1": {"kind": "handle", "box_type": "StringBox"},
                        }
                    },
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "boxcall", "dst": 2, "box": 1, "method": "size", "args": []},
                                {"op": "ret", "value": 2},
                            ],
                        }
                    ],
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ''
        self.assertIn('call i64 @"nyash.string.len_h"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.any.length_h"', ir_txt, msg=ir_txt)

    def test_boxcall_size_arrayish_prefers_array_len_h(self):
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [1],
                    "metadata": {
                        "value_types": {
                            "1": {"kind": "handle", "box_type": "ArrayBox"},
                        }
                    },
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "boxcall", "dst": 2, "box": 1, "method": "size", "args": []},
                                {"op": "ret", "value": 2},
                            ],
                        }
                    ],
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ''
        self.assertIn('call i64 @"nyash.array.slot_len_h"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.any.length_h"', ir_txt, msg=ir_txt)

    def test_boxcall_size_mapish_prefers_map_entry_count_i64(self):
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [1],
                    "metadata": {
                        "value_types": {
                            "1": {"kind": "handle", "box_type": "MapBox"},
                        }
                    },
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "boxcall", "dst": 2, "box": 1, "method": "size", "args": []},
                                {"op": "ret", "value": 2},
                            ],
                        }
                    ],
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ''
        self.assertIn('call i64 @"nyash.map.entry_count_i64"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.any.length_h"', ir_txt, msg=ir_txt)

    def test_boxcall_get_arrayish_integer_key_prefers_array_slot_load_hi(self):
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [1, 2],
                    "metadata": {
                        "value_types": {
                            "1": {"kind": "handle", "box_type": "ArrayBox"},
                            "2": "i64",
                        }
                    },
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "boxcall", "dst": 3, "box": 1, "method": "get", "args": [2]},
                                {"op": "ret", "value": 3},
                            ],
                        }
                    ],
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ''
        self.assertIn('call i64 @"nyash.array.slot_load_hi"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.array.get_hh"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.map.slot_load_hh"', ir_txt, msg=ir_txt)

    def test_boxcall_get_arrayish_non_integer_key_prefers_runtime_data_facade(self):
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [1, 2],
                    "metadata": {
                        "value_types": {
                            "1": {"kind": "handle", "box_type": "ArrayBox"},
                            "2": {"kind": "handle", "box_type": "StringBox"},
                        }
                    },
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "boxcall", "dst": 3, "box": 1, "method": "get", "args": [2]},
                                {"op": "ret", "value": 3},
                            ],
                        }
                    ],
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ''
        self.assertIn('call i64 @"nyash.runtime_data.get_hh"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.array.slot_load_hi"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.map.slot_load_hh"', ir_txt, msg=ir_txt)

    def test_boxcall_get_mapish_keeps_map_route(self):
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [1, 2],
                    "metadata": {
                        "value_types": {
                            "1": {"kind": "handle", "box_type": "MapBox"},
                            "2": {"kind": "handle", "box_type": "StringBox"},
                        }
                    },
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "boxcall", "dst": 3, "box": 1, "method": "get", "args": [2]},
                                {"op": "ret", "value": 3},
                            ],
                        }
                    ],
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ''
        self.assertIn('call i64 @"nyash.map.slot_load_hh"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.array.slot_load_hi"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.array.get_hh"', ir_txt, msg=ir_txt)
