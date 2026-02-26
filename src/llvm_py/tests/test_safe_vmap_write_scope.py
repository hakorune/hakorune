#!/usr/bin/env python3
import unittest
import sys
from pathlib import Path

import llvmlite.ir as ir

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from resolver import Resolver
from utils.values import safe_vmap_write


class TestSafeVmapWriteScope(unittest.TestCase):
    def setUp(self):
        self.i64 = ir.IntType(64)
        self.mod = ir.Module(name="safe_vmap_scope")
        fn = ir.Function(self.mod, ir.FunctionType(self.i64, []), name="main")
        entry = fn.append_basic_block("entry")
        builder = ir.IRBuilder(entry)

        self.global_vmap = {}
        self.resolver = Resolver(self.global_vmap, {})
        self.resolver.builder = builder

        phi = builder.phi(self.i64, name="phi_v10")
        phi.add_incoming(ir.Constant(self.i64, 1), entry)
        self.phi = phi

    def test_global_vmap_keeps_phi_protected(self):
        self.global_vmap[10] = self.phi
        safe_vmap_write(
            self.global_vmap,
            10,
            ir.Constant(self.i64, 99),
            "unit_global_phi_protect",
            resolver=self.resolver,
        )
        self.assertIs(self.global_vmap[10], self.phi)

    def test_local_vmap_allows_shadowing_phi(self):
        self.global_vmap[10] = self.phi
        local_vmap = dict(self.global_vmap)
        val = ir.Constant(self.i64, 42)
        safe_vmap_write(
            local_vmap,
            10,
            val,
            "unit_local_phi_shadow",
            resolver=self.resolver,
        )
        self.assertIs(local_vmap[10], val)
        self.assertIs(self.global_vmap[10], self.phi)


if __name__ == "__main__":
    unittest.main()
