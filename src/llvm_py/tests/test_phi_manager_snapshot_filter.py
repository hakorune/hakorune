import unittest

import llvmlite.ir as ir

from phi_manager import PhiManager


class _FakeContext:
    def __init__(self, def_blocks, dominators):
        self.def_blocks = def_blocks
        self._dominators = dominators

    def dominates(self, def_bid: int, use_bid: int) -> bool:
        return def_bid in self._dominators.get(use_bid, set())


class PhiManagerSnapshotFilterTests(unittest.TestCase):
    def setUp(self):
        self.module = ir.Module(name="phi_manager_filter_test")
        fn_ty = ir.FunctionType(ir.VoidType(), [ir.IntType(64)])
        self.func = ir.Function(self.module, fn_ty, name="main")
        self.entry = self.func.append_basic_block("bb0")
        self.left = self.func.append_basic_block("bb1")
        self.right = self.func.append_basic_block("bb2")
        self.join = self.func.append_basic_block("bb3")

        builder = ir.IRBuilder(self.entry)
        self.entry_add = builder.add(self.func.args[0], ir.Constant(ir.IntType(64), 1), name="entry_add")

        builder.position_at_end(self.left)
        self.left_add = builder.add(self.func.args[0], ir.Constant(ir.IntType(64), 2), name="left_add")

        builder.position_at_end(self.right)
        self.right_add = builder.add(self.func.args[0], ir.Constant(ir.IntType(64), 3), name="right_add")

        builder.position_at_end(self.join)
        self.join_phi = builder.phi(ir.IntType(64), name="join_phi")
        self.join_phi.add_incoming(self.left_add, self.left)
        self.join_phi.add_incoming(self.right_add, self.right)

    def test_preserves_phi_argument_constant_and_single_dominating_def(self):
        manager = PhiManager()
        manager.register_phi(3, 40, self.join_phi)
        context = _FakeContext(
            def_blocks={10: {0}, 40: {3}},
            dominators={3: {0, 3}},
        )
        filtered = manager.filter_vmap_preserve_phis(
            {
                1: self.func.args[0],
                2: ir.Constant(ir.IntType(64), 7),
                10: self.entry_add,
                40: self.join_phi,
            },
            3,
            context,
        )
        self.assertIn(1, filtered)
        self.assertIn(2, filtered)
        self.assertIn(10, filtered)
        self.assertIn(40, filtered)

    def test_drops_multi_def_value_from_cross_block_snapshot(self):
        manager = PhiManager()
        context = _FakeContext(
            def_blocks={20: {1, 2}},
            dominators={3: {0, 3}},
        )
        filtered = manager.filter_vmap_preserve_phis(
            {20: self.right_add},
            3,
            context,
        )
        self.assertNotIn(20, filtered)

    def test_drops_non_dominating_phi_from_cross_block_snapshot(self):
        manager = PhiManager()
        manager.register_phi(3, 40, self.join_phi)
        context = _FakeContext(
            def_blocks={40: {3}},
            dominators={1: {0, 1}},
        )
        filtered = manager.filter_vmap_preserve_phis(
            {40: self.join_phi},
            1,
            context,
        )
        self.assertNotIn(40, filtered)


if __name__ == "__main__":
    unittest.main()
