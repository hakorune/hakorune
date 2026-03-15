import unittest
import llvmlite.ir as ir

from src.llvm_py.phi_wiring import setup_phi_placeholders


class DummyResolver:
    def __init__(self):
        self.marked = set()
        self.array_ids = set()
        self.value_types = {}

    def mark_string(self, vid: int):
        self.marked.add(int(vid))

    def is_arrayish(self, vid: int) -> bool:
        return int(vid) in self.array_ids


class DummyBuilder:
    def __init__(self, bb_map):
        self.i64 = ir.IntType(64)
        self.vmap = {}
        self.def_blocks = {}
        self.block_phi_incomings = {}
        self.phi_trivial_aliases = {}
        self.predeclared_ret_phis = {}
        self.resolver = DummyResolver()
        self.bb_map = bb_map


class TestPhiTagging(unittest.TestCase):
    def _mk_blocks_and_bbs(self):
        mod = ir.Module(name="m")
        fnty = ir.FunctionType(ir.VoidType(), [])
        fn = ir.Function(mod, fnty, name="f")
        b0 = fn.append_basic_block(name="b0")
        b1 = fn.append_basic_block(name="b1")
        return mod, {0: b0, 1: b1}

    def test_mark_by_dst_type(self):
        _mod, bb_map = self._mk_blocks_and_bbs()
        builder = DummyBuilder(bb_map)
        blocks = [
            {
                "id": 1,
                "instructions": [
                    {
                        "op": "phi",
                        "dst": 42,
                        "dst_type": {"kind": "handle", "box_type": "StringBox"},
                        "incoming": [[7, 0]],
                    }
                ],
            }
        ]
        setup_phi_placeholders(builder, blocks)
        self.assertIn(42, builder.resolver.marked)

    def test_mark_by_incoming_stringish(self):
        _mod, bb_map = self._mk_blocks_and_bbs()
        builder = DummyBuilder(bb_map)
        blocks = [
            {
                "id": 0,
                "instructions": [
                    {"op": "const", "dst": 7, "value": {"type": "string", "value": "hi"}}
                ],
            },
            {
                "id": 1,
                "instructions": [
                    {
                        "op": "phi",
                        "dst": 43,
                        # no dst_type string; inference should happen via incoming
                        "incoming": [[7, 0]],
                    }
                ],
            },
        ]
        setup_phi_placeholders(builder, blocks)
        self.assertIn(43, builder.resolver.marked)

    def test_mark_arrayish_by_dst_type(self):
        _mod, bb_map = self._mk_blocks_and_bbs()
        builder = DummyBuilder(bb_map)
        blocks = [
            {
                "id": 1,
                "instructions": [
                    {
                        "op": "phi",
                        "dst": 44,
                        "dst_type": {"kind": "handle", "box_type": "ArrayBox"},
                        "incoming": [[7, 0]],
                    }
                ],
            }
        ]
        setup_phi_placeholders(builder, blocks)
        self.assertIn(44, builder.resolver.array_ids)
        self.assertEqual(
            builder.resolver.value_types.get(44),
            {"kind": "handle", "box_type": "ArrayBox"},
        )

    def test_mark_arrayish_by_incoming_arrayish(self):
        _mod, bb_map = self._mk_blocks_and_bbs()
        builder = DummyBuilder(bb_map)
        builder.resolver.array_ids.add(7)
        builder.resolver.value_types[7] = {"kind": "handle", "box_type": "ArrayBox"}
        blocks = [
            {
                "id": 1,
                "instructions": [
                    {
                        "op": "phi",
                        "dst": 45,
                        "incoming": [[7, 0]],
                    }
                ],
            }
        ]
        setup_phi_placeholders(builder, blocks)
        self.assertIn(45, builder.resolver.array_ids)
        self.assertEqual(
            builder.resolver.value_types.get(45),
            {"kind": "handle", "box_type": "ArrayBox"},
        )


if __name__ == "__main__":
    unittest.main()
