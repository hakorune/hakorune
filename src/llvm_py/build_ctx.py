"""
Build context for instruction lowering and helpers.

This structure aggregates frequently passed references so call sites can
remain concise as we gradually refactor instruction signatures.
"""

from dataclasses import dataclass
from typing import Any, Dict, List, Optional
import llvmlite.ir as ir

@dataclass
class BuildCtx:
    # Core IR handles
    module: ir.Module
    i64: ir.IntType
    i32: ir.IntType
    i8: ir.IntType
    i1: ir.IntType
    i8p: ir.PointerType

    # SSA maps and CFG
    vmap: Dict[int, ir.Value]
    current_vmap: Dict[int, ir.Value]
    bb_map: Dict[int, ir.Block]
    preds: Dict[int, List[int]]
    block_end_values: Dict[int, Dict[int, ir.Value]]
    def_blocks: Dict[int, Any]

    # Resolver (value queries, casts, string-ish tags)
    resolver: Any
    lower_ctx: Any

    # Optional diagnostics toggles (read from env by the builder)
    trace_phi: bool = False
    verbose: bool = False


def build_ctx_from_owner(owner: Any) -> BuildCtx:
    """Collect the current lowering context from NyashLLVMBuilder."""
    return BuildCtx(
        module=owner.module,
        i64=owner.i64,
        i32=owner.i32,
        i8=owner.i8,
        i1=owner.i1,
        i8p=owner.i8p,
        vmap=owner.vmap,
        current_vmap=getattr(owner, "_current_vmap", owner.vmap),
        bb_map=owner.bb_map,
        preds=owner.preds,
        block_end_values=owner.block_end_values,
        def_blocks=owner.def_blocks,
        resolver=owner.resolver,
        lower_ctx=getattr(owner, "ctx", None),
        trace_phi=bool(getattr(getattr(owner, "context", None), "trace_phi", False)),
        verbose=False,
    )
