import unittest
import llvmlite.ir as ir

from src.llvm_py.build_ctx import build_ctx_from_owner


class DummyOwner:
    def __init__(self):
        self.module = ir.Module(name="m")
        self.i64 = ir.IntType(64)
        self.i32 = ir.IntType(32)
        self.i8 = ir.IntType(8)
        self.i1 = ir.IntType(1)
        self.i8p = self.i8.as_pointer()
        self.vmap = {1: "v1"}
        self.bb_map = {0: "bb0"}
        self.preds = {0: []}
        self.block_end_values = {0: {1: "v1"}}
        self.def_blocks = {1: {0}}
        self.resolver = object()
        self._current_vmap = {2: "v2"}
        self.ctx = object()


class TestBuildCtxOwner(unittest.TestCase):
    def test_build_ctx_from_owner_collects_lowering_maps(self):
        owner = DummyOwner()
        ctx = build_ctx_from_owner(owner)
        self.assertIs(ctx.module, owner.module)
        self.assertEqual(ctx.vmap[1], "v1")
        self.assertEqual(ctx.current_vmap[2], "v2")
        self.assertEqual(ctx.bb_map[0], "bb0")
        self.assertEqual(ctx.def_blocks[1], {0})
        self.assertIs(ctx.lower_ctx, owner.ctx)


if __name__ == "__main__":
    unittest.main()
