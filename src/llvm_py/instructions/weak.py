"""
Phase 285LLVM-1: WeakRef instruction lowering
Handles weak reference creation and upgrade (weak_new, weak_load)

SSOT: docs/reference/language/lifecycle.md:179
"""

import llvmlite.ir as ir
from typing import Dict, Optional, Any


def lower_weak_new(
    builder: ir.IRBuilder,
    module: ir.Module,
    dst_vid: int,
    box_val_vid: int,
    vmap: Dict[int, ir.Value],
    ctx: Optional[Any] = None
) -> None:
    """
    Lower MIR WeakRef(New) instruction

    Converts strong BoxRef to WeakRef.

    MIR: WeakRef { dst: ValueId(10), op: New, value: ValueId(5) }
    LLVM IR: %10 = call i64 @nyrt_weak_new(i64 %5)

    Args:
        builder: Current LLVM IR builder
        module: LLVM module
        dst_vid: Destination value ID for weak handle
        box_val_vid: Source BoxRef value ID
        vmap: Value map
        ctx: Optional context
    """
    i64 = ir.IntType(64)

    # Get or declare nyrt_weak_new function
    nyrt_weak_new = None
    for f in module.functions:
        if f.name == "nyrt_weak_new":
            nyrt_weak_new = f
            break

    if not nyrt_weak_new:
        # Declare: i64 @nyrt_weak_new(i64 strong_handle)
        func_type = ir.FunctionType(i64, [i64])
        nyrt_weak_new = ir.Function(module, func_type, name="nyrt_weak_new")

    # Get strong handle from vmap
    strong_handle = vmap.get(box_val_vid)
    if strong_handle is None:
        # Fallback: treat as literal 0 (invalid)
        strong_handle = ir.Constant(i64, 0)

    # Call nyrt_weak_new
    weak_handle = builder.call(nyrt_weak_new, [strong_handle], name=f"weak_{dst_vid}")

    # Store result in vmap
    vmap[dst_vid] = weak_handle


def lower_weak_load(
    builder: ir.IRBuilder,
    module: ir.Module,
    dst_vid: int,
    weak_ref_vid: int,
    vmap: Dict[int, ir.Value],
    ctx: Optional[Any] = None
) -> None:
    """
    Lower MIR WeakRef(Load) instruction

    Upgrades WeakRef to BoxRef (returns 0/Void on failure).

    MIR: WeakRef { dst: ValueId(20), op: Load, value: ValueId(10) }
    LLVM IR: %20 = call i64 @nyrt_weak_to_strong(i64 %10)

    Args:
        builder: Current LLVM IR builder
        module: LLVM module
        dst_vid: Destination value ID for strong handle (or 0)
        weak_ref_vid: Source WeakRef value ID
        vmap: Value map
        ctx: Optional context
    """
    i64 = ir.IntType(64)

    # Get or declare nyrt_weak_to_strong function
    nyrt_weak_to_strong = None
    for f in module.functions:
        if f.name == "nyrt_weak_to_strong":
            nyrt_weak_to_strong = f
            break

    if not nyrt_weak_to_strong:
        # Declare: i64 @nyrt_weak_to_strong(i64 weak_handle)
        func_type = ir.FunctionType(i64, [i64])
        nyrt_weak_to_strong = ir.Function(module, func_type, name="nyrt_weak_to_strong")

    # Get weak handle from vmap
    weak_handle = vmap.get(weak_ref_vid)
    if weak_handle is None:
        # Fallback: treat as literal 0 (invalid)
        weak_handle = ir.Constant(i64, 0)

    # Call nyrt_weak_to_strong
    strong_handle = builder.call(nyrt_weak_to_strong, [weak_handle], name=f"strong_{dst_vid}")

    # Store result in vmap
    vmap[dst_vid] = strong_handle
