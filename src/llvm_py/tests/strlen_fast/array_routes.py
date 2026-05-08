from .support import NyashLLVMBuilder, StrlenFastTestCase


class TestStrlenFastArrayRoutes(StrlenFastTestCase):
    def test_hakocli_run_args_param_seeds_array_routes_without_metadata(self):
        mir = {
            "functions": [
                {
                    "name": "HakoCli.run/2",
                    "params": [1, 2],
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "boxcall", "dst": 3, "box": 2, "method": "size", "args": []},
                                {"op": "const", "dst": 4, "value": {"type": "i64", "value": 0}},
                                {"op": "boxcall", "dst": 5, "box": 2, "method": "get", "args": [4]},
                                {"op": "ret", "value": 5},
                            ],
                        }
                    ],
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ''
        self.assertIn('call i64 @"nyash.array.slot_len_h"', ir_txt, msg=ir_txt)
        self.assertIn('call i64 @"nyash.array.slot_load_hi"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.any.length_h"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.map.slot_load_hh"', ir_txt, msg=ir_txt)

    def test_hakocli_cmd_build_exe_args_phi_self_carry_keeps_array_slot_load_hi(self):
        mir = {
            "functions": [
                {
                    "name": "HakoCli.cmd_build_exe/2",
                    "params": [1, 2],
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {"op": "const", "dst": 3, "value": {"type": "i64", "value": 0}},
                                {"op": "const", "dst": 7, "value": {"type": "i64", "value": 0}},
                                {"op": "jump", "target": 1},
                            ],
                        },
                        {
                            "id": 1,
                            "instructions": [
                                {"op": "phi", "dst": 5, "incoming": [[2, 0], [5, 2]]},
                                {"op": "boxcall", "dst": 6, "box": 5, "method": "get", "args": [3]},
                                {"op": "const", "dst": 8, "value": {"type": "i64", "value": 1}},
                                {"op": "compare", "dst": 9, "lhs": 7, "rhs": 8, "operation": "<"},
                                {"op": "branch", "cond": 9, "then": 2, "else": 3},
                            ],
                        },
                        {
                            "id": 2,
                            "instructions": [
                                {"op": "jump", "target": 1},
                            ],
                        },
                        {
                            "id": 3,
                            "instructions": [
                                {"op": "ret", "value": 6},
                            ],
                        },
                    ],
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ''
        self.assertIn('call i64 @"nyash.array.slot_load_hi"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.map.slot_load_hh"', ir_txt, msg=ir_txt)

    def test_mir_call_runtime_data_push_arrayish_prefers_array_route(self):
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
                                {"op": "mir_call", "dst": 3, "mir_call": {
                                    "callee": {
                                        "type": "Method",
                                        "box_name": "RuntimeDataBox",
                                        "name": "push",
                                        "receiver": 1
                                    },
                                    "args": [2]
                                }},
                                {"op": "ret", "value": 3},
                            ],
                        }
                    ],
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ''
        self.assertIn('call i64 @"nyash.array.slot_append_hh"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.runtime_data.push_hh"', ir_txt, msg=ir_txt)

    def test_mir_call_runtime_data_get_arrayish_integer_key_prefers_array_route(self):
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
                                {"op": "mir_call", "dst": 3, "mir_call": {
                                    "callee": {
                                        "type": "Method",
                                        "box_name": "RuntimeDataBox",
                                        "name": "get",
                                        "receiver": 1
                                    },
                                    "args": [2]
                                }},
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
        self.assertNotIn('call i64 @"nyash.runtime_data.get_hh"', ir_txt, msg=ir_txt)

    def test_mir_call_runtime_data_get_arrayish_non_integer_key_uses_runtime_data_facade(self):
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
                                {"op": "mir_call", "dst": 3, "mir_call": {
                                    "callee": {
                                        "type": "Method",
                                        "box_name": "RuntimeDataBox",
                                        "name": "get",
                                        "receiver": 1
                                    },
                                    "args": [2]
                                }},
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
        self.assertNotIn('call i64 @"nyash.array.get_hh"', ir_txt, msg=ir_txt)

    def test_mir_call_runtime_data_get_newbox_array_receiver_without_metadata(self):
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
                                {"op": "const", "dst": 2, "value": {"type": "i64", "value": 0}},
                                {"op": "mir_call", "dst": 3, "mir_call": {
                                    "callee": {
                                        "type": "Method",
                                        "box_name": "RuntimeDataBox",
                                        "name": "get",
                                        "receiver": 1
                                    },
                                    "args": [2]
                                }},
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
        self.assertNotIn('call i64 @"nyash.runtime_data.get_hh"', ir_txt, msg=ir_txt)

    def test_mir_call_runtime_data_set_arrayish_integer_key_prefers_array_int_key_route(self):
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [1, 2, 3],
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
                                {"op": "mir_call", "dst": 4, "mir_call": {
                                    "callee": {
                                        "type": "Method",
                                        "box_name": "RuntimeDataBox",
                                        "name": "set",
                                        "receiver": 1
                                    },
                                    "args": [2, 3]
                                }},
                                {"op": "ret", "value": 4},
                            ],
                        }
                    ],
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ''
        self.assertIn('call i64 @"nyash.array.slot_store_hih"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.array.set_hhh"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.runtime_data.set_hhh"', ir_txt, msg=ir_txt)

    def test_mir_call_runtime_data_set_arrayish_integer_key_and_value_prefers_slot_store_hii(self):
        mir = {
            "functions": [
                {
                    "name": "main",
                    "params": [1, 2, 3],
                    "metadata": {
                        "value_types": {
                            "1": {"kind": "handle", "box_type": "ArrayBox"},
                            "2": "i64",
                            "3": "i64",
                        }
                    },
                    "blocks": [
                        {
                            "id": 0,
                            "instructions": [
                                {
                                    "op": "mir_call",
                                    "dst": 4,
                                    "mir_call": {
                                        "callee": {
                                            "type": "Method",
                                            "box_name": "RuntimeDataBox",
                                            "name": "set",
                                            "receiver": 1,
                                        },
                                        "args": [2, 3],
                                    },
                                },
                                {"op": "ret", "value": 4},
                            ],
                        }
                    ],
                }
            ]
        }

        b = NyashLLVMBuilder()
        ir_txt = b.build_from_mir(mir) or ""
        self.assertIn('call i64 @"nyash.array.slot_store_hii"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.array.slot_store_hih"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.array.set_hhh"', ir_txt, msg=ir_txt)
        self.assertNotIn('call i64 @"nyash.runtime_data.set_hhh"', ir_txt, msg=ir_txt)

