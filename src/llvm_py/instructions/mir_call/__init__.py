"""
Unified MIR Call instruction dispatcher - Phase 134-A Modular Design

This module provides the canonical dispatcher for MIR Call instructions,
routing to specialized handlers based on callee type.

Architecture:
- global_call.py: Global function calls
- method_call.py: Box method calls (BoxCall)
- constructor_call.py: Box constructors (NewBox)
- closure_call.py: Closure creation (NewClosure)
- value_call.py: Dynamic function value calls
- extern_call.py: External C ABI calls

Design Philosophy:
- Unified dispatch: One entry point (lower_mir_call)
- Canonical schema only: legacy callee keys are rejected
- Fail-fast: No legacy fallbacks (NotImplementedError removed)
- Modular: Each callee type has dedicated handler
"""

from typing import Dict, Any, Optional
from llvmlite import ir
import os
import json
from trace import hot_count as trace_hot_count

# Import specialized handlers
from .global_call import lower_global_call
from .method_call import lower_method_call
from .constructor_call import lower_constructor_call
from .closure_call import lower_closure_creation
from .value_call import lower_value_call
from .extern_call import lower_extern_call

# Import compatibility layer
import sys
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..'))
from mir_call_compat import MirCallCompat


def lower_mir_call(owner, builder: ir.IRBuilder, mir_call: Dict[str, Any], dst_vid: Optional[int], vmap: Dict, resolver):
    """
    Lower unified MirCall instruction - CANONICAL DISPATCHER

    This is the single entry point for all MIR Call instruction lowering.
    It dispatches to specialized handlers based on callee type.

    Parameters:
    - owner: NyashLLVMBuilder instance
    - builder: LLVM IR builder
    - mir_call: MirCall dict containing 'callee', 'args', 'flags', 'effects'
    - dst_vid: Optional destination register
    - vmap: Value mapping dict
    - resolver: Value resolver instance

    Callee Types:
    - Global: Global function call (e.g., print, panic)
    - Method: Box method call (e.g., str.substring())
    - Constructor: Box constructor (e.g., new StringBox())
    - Closure: Closure creation
    - Value: Dynamic function value call
    - Extern: External C ABI function call

    Raises:
        ValueError: If callee type is unknown or missing
    """

    # Guard: avoid emitting after a terminator; if current block is closed, create continuation.
    trace_hot_count(resolver, "call_total")

    # Guard: avoid emitting after a terminator; if current block is closed, create continuation.
    try:
        if builder.block is not None and getattr(builder.block, 'terminator', None) is not None:
            func = builder.block.parent
            cont = func.append_basic_block(name=f"cont_bb_{builder.block.name}")
            builder.position_at_end(cont)
    except Exception:
        pass

    # Extract callee and arguments
    callee = mir_call.get("callee", {})
    args = mir_call.get("args", [])
    flags = mir_call.get("flags", {})
    effects = mir_call.get("effects", {})

    # Normalize callee JSON (canonical-only)
    if callee:
        callee = MirCallCompat.normalize_callee(callee)

    # Parse callee type
    callee_type = callee.get("type") if callee else None

    # Optional trace: dump callee info (including certainty for Method)
    if os.getenv('NYASH_LLVM_TRACE_CALLS') == '1':
        try:
            evt = {'type': callee_type}
            if callee_type == 'Global':
                evt.update({'name': callee.get('name')})
            elif callee_type == 'Method':
                evt.update({
                    'box_name': callee.get('box_name'),
                    'method': callee.get('name'),
                    'receiver': callee.get('receiver'),
                    'certainty': callee.get('certainty'),
                })
            elif callee_type == 'Extern':
                evt.update({'name': callee.get('name')})
            print(json.dumps({'phase': 'llvm', 'cat': 'mir_call', 'event': evt}))
        except Exception:
            pass

    # Dispatch to specialized handler based on callee type
    if callee_type == "Global":
        # Global function call (e.g., print, panic)
        func_name = callee.get("name")
        lower_global_call(builder, owner.module, func_name, args, dst_vid, vmap, resolver, owner)

    elif callee_type == "Method":
        # Box method call
        method = callee.get("name")
        box_name = callee.get("box_name")
        receiver = callee.get("receiver")
        certainty = callee.get("certainty")

        # SSOT: Method calls split into two routes:
        # - Static method (receiver=null, certainty=Known): lower as direct function call `Box.method/$arity`
        # - Instance method (receiver omitted in v1 JSON): receiver is implicit as first arg
        if receiver is None:
            if certainty == "Known" and box_name and method:
                func_name = f"{box_name}.{method}/{len(args)}"
                lower_global_call(builder, owner.module, func_name, args, dst_vid, vmap, resolver, owner)
                return
            if args:
                receiver = args[0]
                args = args[1:]  # Remove receiver from args

        lower_method_call(builder, owner.module, box_name, method, receiver, args, dst_vid, vmap, resolver, owner)

    elif callee_type == "Constructor":
        # Box constructor (NewBox)
        box_type = callee.get("name")
        if box_type is None:
            raise ValueError(f"Constructor callee requires 'name': {callee}")
        lower_constructor_call(builder, owner.module, box_type, args, dst_vid, vmap, resolver, owner)

    elif callee_type == "Closure":
        # Closure creation (NewClosure)
        params = callee.get("params", [])
        captures = callee.get("captures", [])
        me_capture = callee.get("me_capture")
        lower_closure_creation(builder, owner.module, params, captures, me_capture, dst_vid, vmap, resolver, owner)

    elif callee_type == "Value":
        # Dynamic function value call
        func_vid = callee.get("value")
        if func_vid is None:
            raise ValueError(f"Value callee requires 'value': {callee}")
        lower_value_call(builder, owner.module, func_vid, args, dst_vid, vmap, resolver, owner)

    elif callee_type == "Extern":
        # External C ABI function call
        extern_name = callee.get("name")
        lower_extern_call(builder, owner.module, extern_name, args, dst_vid, vmap, resolver, owner)

    else:
        # Fail-fast: No legacy fallback
        raise ValueError(f"Unknown or missing callee type: {callee_type} (callee={callee})")


# Export dispatcher as the main interface
__all__ = ['lower_mir_call']
