"""
Return instruction lowering
Handles void and value returns
"""

import os
import sys
import llvmlite.ir as ir
from typing import Dict, Optional, Any
from instructions.sum_escape import materialize_sum_escape_value_if_needed
from instructions.user_box_local import materialize_user_box_escape_value_if_needed
try:
    # Create PHIs at block head to satisfy LLVM invariant
    from ..phi_wiring.wiring import phi_at_block_head as _phi_at_block_head
except Exception:
    _phi_at_block_head = None


class UnreachableReturnHandlerBox:
    """
    Box-First principle: Single Responsibility - Handle unreachable block returns
    Phase 284-P2: Implements Fail-Fast principle for unreachable blocks
    """

    @staticmethod
    def handle_null_return(builder: ir.IRBuilder, return_type: ir.Type) -> None:
        """
        Handle unreachable blocks with Fail-Fast principle

        MIR may generate unreachable blocks with null return values (e.g., loop(true) exit).
        LLVM type-checks all blocks, so we must match the function signature.
        Use 'unreachable' to satisfy type system AND crash if executed (Fail-Fast).

        Args:
            builder: Current LLVM IR builder
            return_type: Expected return type
        """
        if isinstance(return_type, ir.VoidType):
            # True void function: keep original behavior
            builder.ret_void()
        else:
            # Non-void function with null return: emit unreachable
            # This satisfies LLVM's type checker (no return needed after unreachable)
            # AND crashes immediately if the "unreachable" block is ever reached
            builder.unreachable()


class ReturnTypeAdjusterBox:
    """
    Box-First principle: Single Responsibility - Adjust return value types to match function signature
    Handles ptr↔int conversion and int width adjustment
    """

    @staticmethod
    def adjust_type(builder: ir.IRBuilder, ret_val: ir.Value, return_type: ir.Type) -> ir.Value:
        """
        Adjust return value type if needed

        Args:
            builder: Current LLVM IR builder
            ret_val: Return value to adjust
            return_type: Expected return type

        Returns:
            Adjusted return value
        """
        # Type adjustment if needed
        if hasattr(ret_val, 'type') and ret_val.type != return_type:
            if isinstance(return_type, ir.IntType) and ret_val.type.is_pointer:
                # ptr to int
                ret_val = builder.ptrtoint(ret_val, return_type, name="ret_p2i")
            elif isinstance(return_type, ir.PointerType) and isinstance(ret_val.type, ir.IntType):
                # int to ptr
                ret_val = builder.inttoptr(ret_val, return_type, name="ret_i2p")
            elif isinstance(return_type, ir.IntType) and isinstance(ret_val.type, ir.IntType):
                # int to int conversion
                if return_type.width < ret_val.type.width:
                    # Truncate
                    ret_val = builder.trunc(ret_val, return_type)
                elif return_type.width > ret_val.type.width:
                    # Zero extend
                    ret_val = builder.zext(ret_val, return_type)
        return ret_val


class StringBoxerBox:
    """
    Box-First principle: Single Responsibility - Box string pointers to handles
    Converts i8* string pointers to i64 box handles via nyash.box.from_i8_string
    """

    @staticmethod
    def box_string_pointer(builder: ir.IRBuilder, string_ptr: ir.Value) -> ir.Value:
        """
        Box a string pointer (i8*) to handle (i64)

        Args:
            builder: Current LLVM IR builder
            string_ptr: i8* string pointer

        Returns:
            i64 box handle
        """
        i8p = ir.IntType(8).as_pointer()
        i64 = ir.IntType(64)

        # Find or declare nyash.box.from_i8_string
        boxer = None
        for f in builder.module.functions:
            if f.name == 'nyash.box.from_i8_string':
                boxer = f
                break

        if boxer is None:
            boxer = ir.Function(
                builder.module,
                ir.FunctionType(i64, [i8p]),
                name='nyash.box.from_i8_string'
            )

        return builder.call(boxer, [string_ptr], name='ret_ptr2h')


class ReturnPhiSynthesizerBox:
    """
    Box-First principle: Single Responsibility - Synthesize PHI nodes for return values
    Creates PHI at block head when return value is zero-like and has predecessors
    Phase 131-4: Respects _disable_phi_synthesis flag
    """

    @staticmethod
    def should_synthesize_phi(ret_val: ir.Value, return_type: ir.Type) -> bool:
        """
        Check if return value is zero-like and needs PHI synthesis

        Args:
            ret_val: Return value to check
            return_type: Expected return type

        Returns:
            True if PHI synthesis is needed
        """
        if not isinstance(ret_val, ir.Constant):
            return False

        # Check if zero-like
        if isinstance(return_type, ir.IntType):
            return str(ret_val) == str(ir.Constant(return_type, 0))
        elif isinstance(return_type, ir.DoubleType):
            return str(ret_val) == str(ir.Constant(return_type, 0.0))
        elif isinstance(return_type, ir.PointerType):
            return str(ret_val) == str(ir.Constant(return_type, None))

        return False

    @staticmethod
    def synthesize_phi(
        builder: ir.IRBuilder,
        value_id: int,
        return_type: ir.Type,
        preds: Dict[int, list],
        block_end_values: Dict[int, Dict[int, ir.Value]],
        bb_map: Dict[int, ir.Block],
        resolver=None
    ) -> Optional[ir.Value]:
        """
        Synthesize PHI node at block head for return value

        Args:
            builder: Current LLVM IR builder
            value_id: Value ID to synthesize PHI for
            return_type: Expected return type
            preds: Predecessor map
            block_end_values: Block end value snapshots
            bb_map: Block ID to LLVM block map
            resolver: Optional resolver for disable flag

        Returns:
            PHI node if synthesized, None otherwise
        """
        # Check if PHI synthesis is disabled (Phase 131-4)
        if resolver is not None and hasattr(resolver, '_disable_phi_synthesis'):
            if getattr(resolver, '_disable_phi_synthesis', False):
                return None

        # Derive current block ID from name like 'bb3'
        cur_bid = None
        try:
            cur_bid = int(str(builder.block.name).replace('bb', ''))
        except Exception:
            return None

        if cur_bid is None:
            return None

        # Collect incoming values from predecessors
        incoming = []
        for p in preds.get(cur_bid, []):
            if p == cur_bid:
                continue

            v = None
            try:
                v = block_end_values.get(p, {}).get(value_id)
            except Exception:
                v = None

            if v is None:
                v = ir.Constant(return_type, 0)

            bblk = bb_map.get(p)
            if bblk is not None:
                incoming.append((v, bblk))

        if not incoming:
            return None

        # Create PHI at block head
        if _phi_at_block_head is not None:
            phi = _phi_at_block_head(builder.block, return_type, name=f"ret_phi_{value_id}")
        else:
            # Fallback: create PHI at block head using a temporary builder
            try:
                _b = ir.IRBuilder(builder.block)
                _b.position_at_start(builder.block)
                phi = _b.phi(return_type, name=f"ret_phi_{value_id}")
            except Exception:
                # As a last resort, create via current builder (may still succeed)
                phi = builder.phi(return_type, name=f"ret_phi_{value_id}")

        # Add incoming values
        for (v, bblk) in incoming:
            phi.add_incoming(v, bblk)

        return phi


def lower_return(
    builder: ir.IRBuilder,
    value_id: Optional[int],
    vmap: Dict[int, ir.Value],
    return_type: ir.Type,
    resolver=None,
    preds=None,
    block_end_values=None,
    bb_map=None,
    ctx: Optional[Any] = None,
) -> None:
    """
    Lower MIR Return instruction
    
    Args:
        builder: Current LLVM IR builder
        value_id: Optional return value ID
        vmap: Value map
        return_type: Expected return type
    """
    # Prefer BuildCtx maps if provided
    if ctx is not None:
        try:
            if getattr(ctx, 'resolver', None) is not None:
                resolver = ctx.resolver
            if getattr(ctx, 'preds', None) is not None and preds is None:
                preds = ctx.preds
            if getattr(ctx, 'block_end_values', None) is not None and block_end_values is None:
                block_end_values = ctx.block_end_values
            if getattr(ctx, 'bb_map', None) is not None and bb_map is None:
                bb_map = ctx.bb_map
        except Exception:
            pass
    if value_id is None:
        # Delegate to UnreachableReturnHandlerBox (Box-First principle)
        UnreachableReturnHandlerBox.handle_null_return(builder, return_type)
    else:
        # Get return value (prefer resolver)
        ret_val = None
        if isinstance(value_id, int):
            ret_val = materialize_sum_escape_value_if_needed(
                builder,
                builder.block.parent.module,
                int(value_id),
                vmap,
                resolver,
                name_hint=f"ret_{value_id}",
            )
        if ret_val is None and isinstance(value_id, int):
            ret_val = materialize_user_box_escape_value_if_needed(
                builder,
                builder.block.parent.module,
                int(value_id),
                vmap,
                resolver,
                name_hint=f"ret_{value_id}",
            )
        # Fast path (dominance-safe):
        # only reuse vmap value when it is guaranteed to dominate this return block.
        if ret_val is None and isinstance(value_id, int):
            tmp0 = vmap.get(value_id)
            cur_bid = None
            pred_ids = []
            try:
                cur_bid = int(str(builder.block.name).replace('bb', ''))
            except Exception:
                cur_bid = None
            try:
                if cur_bid is not None and isinstance(preds, dict):
                    pred_ids = [p for p in preds.get(cur_bid, []) if p != cur_bid]
            except Exception:
                pred_ids = []

            # Phase 132 Debug: trace vmap lookup
            if os.environ.get('NYASH_LLVM_VMAP_TRACE') == '1':
                found = "FOUND" if tmp0 is not None else "MISSING"
                print(f"[vmap/ret] value_id={value_id} {found} in vmap, keys={sorted(list(vmap.keys())[:20])}", file=sys.stderr)
                if tmp0 is not None:
                    is_phi = hasattr(tmp0, 'add_incoming')
                    print(f"[vmap/ret] tmp0 type={'PHI' if is_phi else 'VALUE'}", file=sys.stderr)
            if tmp0 is not None:
                is_phi = hasattr(tmp0, 'add_incoming')
                phi_in_current = False
                if is_phi:
                    try:
                        phi_bb = getattr(getattr(tmp0, 'basic_block', None), 'name', None)
                        cur_bb = getattr(builder.block, 'name', None)
                        if isinstance(phi_bb, bytes):
                            phi_bb = phi_bb.decode()
                        if isinstance(cur_bb, bytes):
                            cur_bb = cur_bb.decode()
                        phi_in_current = phi_bb == cur_bb
                    except Exception:
                        phi_in_current = False

                defined_here = False
                if resolver is not None and cur_bid is not None and hasattr(resolver, 'def_blocks'):
                    try:
                        defs = resolver.def_blocks.get(value_id, set())
                        defined_here = cur_bid in defs
                    except Exception:
                        defined_here = False

                # Entry/no-pred block can safely reuse args/constants from local vmap.
                entry_like = len(pred_ids) == 0

                if is_phi:
                    if phi_in_current:
                        ret_val = tmp0
                elif defined_here or entry_like:
                    ret_val = tmp0
        # Fallback: consult builder-global vmap (via resolver) for predeclared PHIs
        if ret_val is None and resolver is not None and hasattr(resolver, 'global_vmap'):
            try:
                g = resolver.global_vmap.get(int(value_id)) if isinstance(value_id, int) else None
                if g is not None:
                    ret_val = g
            except Exception:
                pass
        if ret_val is None:
            if resolver is not None and preds is not None and block_end_values is not None and bb_map is not None:
                # Resolve direct value; PHIは finalize_phis に一任
                if isinstance(return_type, ir.PointerType):
                    ret_val = resolver.resolve_ptr(value_id, builder.block, preds, block_end_values, vmap)
                else:
                    is_stringish = False
                    if hasattr(resolver, 'is_stringish'):
                        try:
                            is_stringish = resolver.is_stringish(int(value_id))
                        except Exception:
                            is_stringish = False
                    if is_stringish and hasattr(resolver, 'string_ptrs') and int(value_id) in getattr(resolver, 'string_ptrs'):
                        # Delegate to StringBoxerBox (Box-First principle)
                        p = resolver.string_ptrs[int(value_id)]
                        ret_val = StringBoxerBox.box_string_pointer(builder, p)
                    else:
                        ret_val = resolver.resolve_i64(value_id, builder.block, preds, block_end_values, vmap, bb_map)
                
        if ret_val is None:
            # Default to vmap (non-PHI) if available
            tmp = vmap.get(value_id)
            try:
                is_phi = hasattr(tmp, 'add_incoming')
            except Exception:
                is_phi = False
            if tmp is not None and not is_phi:
                ret_val = tmp
        if not ret_val:
            # Default based on return type
            if isinstance(return_type, ir.IntType):
                ret_val = ir.Constant(return_type, 0)
            elif isinstance(return_type, ir.DoubleType):
                ret_val = ir.Constant(return_type, 0.0)
            else:
                # Pointer type - null
                ret_val = ir.Constant(return_type, None)

        # Delegate PHI synthesis to ReturnPhiSynthesizerBox (Box-First principle)
        # Phase 131-4: Skip PHI synthesis if disabled (e.g., during Pass C terminator lowering)
        try:
            if ReturnPhiSynthesizerBox.should_synthesize_phi(ret_val, return_type):
                if preds is not None and block_end_values is not None and bb_map is not None and isinstance(value_id, int):
                    phi = ReturnPhiSynthesizerBox.synthesize_phi(
                        builder, value_id, return_type, preds, block_end_values, bb_map, resolver
                    )
                    if phi is not None:
                        ret_val = phi
        except Exception:
            pass

        # Delegate type adjustment to ReturnTypeAdjusterBox (Box-First principle)
        ret_val = ReturnTypeAdjusterBox.adjust_type(builder, ret_val, return_type)

        # Emit return; no further instructions should be emitted in this block
        builder.ret(ret_val)
