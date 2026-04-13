import unittest

from src.llvm_py.builders.closure_split_contract import build_closure_split_contract
from src.llvm_py.builders.ipo_callable_contract import build_ipo_callable_contract
from src.llvm_py.builders.ipo_call_edge_contract import build_ipo_call_edge_contract
from src.llvm_py.instructions.mir_call.closure_call import lower_closure_creation
import llvmlite.ir as ir


class _ContextStub:
    def __init__(self):
        self.hot_trace_counts = {}


class _ResolverStub:
    def __init__(self, i64_type: ir.IntType):
        self.context = _ContextStub()
        self._i64_type = i64_type
        self.string_literals = {}
        self.string_ptrs = {}

    def resolve_i64(self, value_id, block, preds, block_end_values, vmap, bb_map):
        return ir.Constant(self._i64_type, int(value_id) + 1)


class _OwnerStub:
    def __init__(self, bb):
        self.preds = {0: []}
        self.block_end_values = {}
        self.bb_map = {0: bb}


def _new_builder():
    i64 = ir.IntType(64)
    module = ir.Module(name="test_ipo_callable_edge_contract")
    fn = ir.Function(module, ir.FunctionType(i64, []), name="main")
    bb = fn.append_basic_block("entry")
    builder = ir.IRBuilder(bb)
    return i64, module, builder, bb


class TestIpoCallableEdgeContract(unittest.TestCase):
    def test_builds_cross_module_direct_callable_for_empty_env(self):
        closure_contract = build_closure_split_contract(params=[{"id": 1}], captures=[], me_capture=None)

        callable_contract = build_ipo_callable_contract(closure_contract)
        edge_contract = build_ipo_call_edge_contract(callable_contract)

        self.assertEqual(callable_contract["proof"]["thin_surface"], "cross_module_direct")
        self.assertEqual(callable_contract["proof"]["env_surface"], "empty")
        self.assertEqual(callable_contract["policy"]["import_class"], "cross_module_candidate")
        self.assertEqual(edge_contract["proof"]["call_shape"], "DirectThin")

    def test_builds_local_only_callable_for_single_scalar_env(self):
        closure_contract = build_closure_split_contract(params=[{"id": 1}], captures=[{"id": 40}], me_capture=None)

        callable_contract = build_ipo_callable_contract(closure_contract)
        edge_contract = build_ipo_call_edge_contract(callable_contract)

        self.assertEqual(callable_contract["proof"]["thin_surface"], "local_only")
        self.assertEqual(callable_contract["proof"]["env_surface"], "single_scalar")
        self.assertEqual(callable_contract["policy"]["import_class"], "module_local_only")
        self.assertEqual(edge_contract["proof"]["call_shape"], "DirectThick")

    def test_builds_public_only_callable_for_aggregate_env(self):
        closure_contract = build_closure_split_contract(
            params=[{"id": 1}],
            captures=[{"id": 40}, 41],
            me_capture=42,
        )

        callable_contract = build_ipo_callable_contract(closure_contract)
        edge_contract = build_ipo_call_edge_contract(callable_contract)

        self.assertEqual(callable_contract["proof"]["thin_surface"], "none")
        self.assertEqual(callable_contract["proof"]["env_surface"], "aggregate_handle")
        self.assertEqual(callable_contract["policy"]["import_class"], "public_only")
        self.assertEqual(edge_contract["proof"]["call_shape"], "DirectThick")

    def test_lower_closure_creation_records_ipo_contracts_on_context(self):
        i64, module, builder, bb = _new_builder()
        resolver = _ResolverStub(i64)
        owner = _OwnerStub(bb)
        vmap = {}

        lower_closure_creation(
            builder,
            module,
            params=[{"id": 1}],
            captures=[],
            me_capture=None,
            dst_vid=90,
            vmap=vmap,
            resolver=resolver,
            owner=owner,
        )

        self.assertIn(90, resolver.context.ipo_callable_contracts)
        self.assertIn(90, resolver.context.ipo_call_edge_contracts)
        self.assertEqual(
            resolver.context.ipo_callable_contracts[90]["proof"]["thin_surface"],
            "cross_module_direct",
        )
        self.assertEqual(
            resolver.context.ipo_call_edge_contracts[90]["proof"]["call_shape"],
            "DirectThin",
        )


if __name__ == "__main__":
    unittest.main()
