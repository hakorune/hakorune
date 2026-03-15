from typing import Dict, Any, List, Set

from llvmlite import ir
from trace import debug as trace_debug
from trace import hot as trace_hot
from trace import hot_enabled as trace_hot_enabled
from trace import format_hot_summary as trace_format_hot_summary
from prepass.if_merge import plan_ret_phi_predeclare
from prepass.loops import detect_simple_while
from cfg.utils import (
    collect_arrayish_value_ids,
    collect_integerish_value_ids,
    collect_non_negative_value_ids,
    propagate_arrayish_value_ids,
    collect_stringish_value_ids,
)
from phi_wiring import (
    setup_phi_placeholders as _setup_phi_placeholders,
    finalize_phis as _finalize_phis,
    build_succs as _build_succs,
)
from context import FunctionLowerContext
from phi_manager import PhiManager


def _collect_branch_only_compare_dsts(blocks: List[Dict[str, Any]]) -> Set[int]:
    """Return compare dst ValueIds that are consumed only as branch conditions."""
    compare_dsts: Set[int] = set()
    total_use: Dict[int, int] = {}
    branch_use: Dict[int, int] = {}

    def add_use(vid, *, branch: bool = False):
        if not isinstance(vid, int):
            return
        total_use[vid] = total_use.get(vid, 0) + 1
        if branch:
            branch_use[vid] = branch_use.get(vid, 0) + 1

    for blk in (blocks or []):
        for ins in (blk.get("instructions") or []):
            op = ins.get("op")
            dst = ins.get("dst")
            if op == "compare" and isinstance(dst, int):
                compare_dsts.add(int(dst))

            if op in ("binop", "compare"):
                add_use(ins.get("lhs"))
                add_use(ins.get("rhs"))
            elif op == "unop":
                add_use(ins.get("src"))
                add_use(ins.get("operand"))
            elif op == "copy":
                add_use(ins.get("src"))
            elif op == "branch":
                add_use(ins.get("cond"), branch=True)
            elif op == "ret":
                add_use(ins.get("value"))
            elif op in ("call", "boxcall", "externcall", "newbox", "while"):
                if op == "call":
                    add_use(ins.get("func"))
                if op == "boxcall":
                    add_use(ins.get("box"))
                    add_use(ins.get("box_val"))
                for v in (ins.get("args") or []):
                    add_use(v)
            elif op == "mir_call":
                mir_call = ins.get("mir_call")
                if isinstance(mir_call, dict):
                    callee = mir_call.get("callee")
                    if isinstance(callee, dict):
                        add_use(callee.get("receiver"))
                    for v in (mir_call.get("args") or []):
                        add_use(v)
                else:
                    for v in (ins.get("args") or []):
                        add_use(v)
            elif op == "typeop":
                add_use(ins.get("src"))
            elif op == "select":
                add_use(ins.get("cond"))
                add_use(ins.get("then_val"))
                add_use(ins.get("else_val"))
            elif op == "phi":
                for inc in (ins.get("incoming") or []):
                    if isinstance(inc, (list, tuple)) and len(inc) >= 1:
                        add_use(inc[0])
            elif op in ("keepalive", "release_strong"):
                for v in (ins.get("values") or []):
                    add_use(v)
            elif op == "safepoint":
                for v in (ins.get("live") or []):
                    add_use(v)
            elif op == "weak_new":
                add_use(ins.get("src"))
            elif op == "weak_load":
                add_use(ins.get("weak"))

    result: Set[int] = set()
    for vid in compare_dsts:
        uses = total_use.get(vid, 0)
        if uses > 0 and uses == branch_use.get(vid, 0):
            result.add(vid)
    return result


def _emit_hot_summary(context: FunctionLowerContext) -> None:
    if not trace_hot_enabled():
        return
    counts = getattr(context, "hot_trace_counts", {})
    trace_hot(trace_format_hot_summary(context.func_name, counts))


def _mark_arrayish_param_fact(builder, value_id: int) -> None:
    try:
        vid = int(value_id)
    except (TypeError, ValueError):
        return

    try:
        builder.resolver.array_ids.add(vid)
    except Exception:
        pass

    try:
        if not hasattr(builder.resolver, "value_types") or not isinstance(builder.resolver.value_types, dict):
            builder.resolver.value_types = {}
        builder.resolver.value_types[vid] = {"kind": "handle", "box_type": "ArrayBox"}
    except Exception:
        pass


def _seed_hakocli_args_array_fact(
    *,
    func_name: str,
    params_list: List[Any],
    param_value_ids: List[int],
    builder,
) -> None:
    """
    Seed the Stage1 launcher CLI argv contract in one place.

    HakoCli.run/2 and HakoCli.cmd_*/2 receive the argv array as their second
    parameter. The current MIR emit path does not attach metadata for that
    parameter, so LLVM lowering must freeze the contract here until caller-side
    metadata becomes the SSOT.
    """
    name = str(func_name or "")
    if not name.startswith("HakoCli."):
        return

    try:
        method_with_arity = name.split(".", 1)[1]
        method_name = method_with_arity.split("/", 1)[0]
    except Exception:
        return

    if method_name != "run" and not method_name.startswith("cmd_"):
        return

    if len(param_value_ids) < 2:
        return

    param_name = None
    if isinstance(params_list, list) and len(params_list) >= 2 and isinstance(params_list[1], str):
        param_name = str(params_list[1])
    if param_name is not None and param_name != "args":
        return

    _mark_arrayish_param_fact(builder, param_value_ids[1])


def _propagate_arrayish_value_facts(builder, blocks: List[Dict[str, Any]]) -> None:
    """Expand seeded ArrayBox facts across copy/phi carrier chains."""
    try:
        seeded = set(getattr(builder.resolver, "array_ids", set()) or set())
        propagated = propagate_arrayish_value_ids(blocks, seeded)
    except Exception:
        return

    try:
        builder.resolver.array_ids.clear()
        builder.resolver.array_ids.update(propagated)
    except Exception:
        try:
            builder.resolver.array_ids = set(propagated)
        except Exception:
            pass

    for vid in propagated:
        _mark_arrayish_param_fact(builder, int(vid))


def _dedup_non_self_preds(preds_map: Dict[int, List[int]], block_id: int) -> List[int]:
    try:
        preds_raw = [p for p in preds_map.get(int(block_id), []) if p != int(block_id)]
    except Exception:
        preds_raw = []
    seen = set()
    preds_list: List[int] = []
    for pred_bid in preds_raw:
        if pred_bid not in seen:
            preds_list.append(pred_bid)
            seen.add(pred_bid)
    return preds_list


def _collect_block_defs(block: Dict[str, Any]) -> set[int]:
    defs: set[int] = set()
    for ins in block.get("instructions") or []:
        try:
            dstv = ins.get("dst")
            if isinstance(dstv, int):
                defs.add(int(dstv))
        except Exception:
            continue
    return defs


def _collect_block_uses(block: Dict[str, Any]) -> set[int]:
    uses: set[int] = set()
    for ins in block.get("instructions") or []:
        for key in ("lhs", "rhs", "value", "cond", "box_val"):
            try:
                value = ins.get(key)
                if isinstance(value, int):
                    uses.add(int(value))
            except Exception:
                continue
    return uses


def _seed_multi_pred_block_phi_incomings(builder, block_by_id: Dict[int, Dict[str, Any]]) -> None:
    from cfg.utils import build_preds_succs

    local_preds, _ = build_preds_succs(block_by_id)
    for bid, blk in block_by_id.items():
        preds_list = _dedup_non_self_preds(local_preds, int(bid))
        if len(preds_list) <= 1:
            continue
        defs = _collect_block_defs(blk)
        need = [vid for vid in _collect_block_uses(blk) if vid not in defs]
        if not need:
            continue
        for vid in need:
            try:
                builder.block_phi_incomings.setdefault(int(bid), {})[int(vid)] = [
                    (int(pred_bid), int(vid)) for pred_bid in preds_list
                ]
            except Exception:
                pass
    try:
        builder.resolver.block_phi_incomings = builder.block_phi_incomings
    except Exception:
        pass


def lower_function(builder, func_data: Dict[str, Any]):
    """Lower a single MIR function to LLVM IR using the given builder context.
    This is a faithful extraction of NyashLLVMBuilder.lower_function.
    """
    import os, re

    name = func_data.get("name", "unknown")
    builder.current_function_name = name
    params = func_data.get("params", [])
    blocks = func_data.get("blocks", [])

    # Determine function signature
    if name == "ny_main":
        # Special case: ny_main returns i64 to match runtime (nyrt) expectations
        func_ty = ir.FunctionType(builder.i64, [])
    else:
        # Default: i64(i64, ...) signature; derive arity from '/N' suffix when params missing
        m = re.search(r"/(\d+)$", name)
        arity = int(m.group(1)) if m else len(params)
        # Dev fallback: when params are missing for global (Box.method) functions,
        # use observed call-site arity if available (scanned in builder.build_from_mir)
        if arity == 0 and '.' in name:
            try:
                arity = int(builder.call_arities.get(name, 0))
            except Exception:
                pass
        param_types = [builder.i64] * arity
        func_ty = ir.FunctionType(builder.i64, param_types)

    # Reset per-function maps and resolver caches to avoid cross-function collisions
    try:
        builder.vmap.clear()
    except Exception:
        builder.vmap = {}
    try:
        builder.bb_map.clear()
    except Exception:
        builder.bb_map = {}
    # Phase 132-P1: Clear per-function predeclared PHI placeholders (avoid cross-function leakage)
    try:
        builder.predeclared_ret_phis.clear()
    except Exception:
        try:
            builder.predeclared_ret_phis = {}
        except Exception:
            pass

    # Phase 132-P1: Create function-local context Box
    # This automatically isolates all function-scoped state
    context = FunctionLowerContext(name)

    # Initialize PHI manager within context
    context.phi_manager = PhiManager()

    # Connect builder attributes to context storage (for backward compatibility)
    builder.phi_manager = context.phi_manager
    builder.block_phi_incomings = context.block_phi_incomings
    builder.phi_trivial_aliases = context.phi_trivial_aliases
    builder.def_blocks = context.def_blocks
    builder.block_end_values = context.block_end_values

    # Bind resolver to context (redirects caches to context storage)
    builder.resolver.bind_context(context)

    # Store context in builder for access by sub-components
    builder.context = context

    # Phase 131-15-P1: Load value_types metadata from JSON into resolver
    try:
        metadata = func_data.get('metadata', {})
        value_types_json = metadata.get('value_types', {})
        # Convert string keys to integers and store in resolver
        builder.resolver.value_types = {}
        for vid_str, vtype in value_types_json.items():
            try:
                vid = int(vid_str)
                builder.resolver.value_types[vid] = vtype
            except (ValueError, TypeError):
                pass
    except Exception:
        # If metadata loading fails, ensure value_types exists (empty fallback)
        builder.resolver.value_types = {}

    # Conservative sign analysis for power-of-two modulo fast path.
    try:
        context.non_negative_value_ids = collect_non_negative_value_ids(blocks)
        builder.resolver.non_negative_ids = context.non_negative_value_ids
    except Exception:
        context.non_negative_value_ids = set()
        builder.resolver.non_negative_ids = context.non_negative_value_ids

    # Conservative integer-like VID analysis for RuntimeData integer-key routes.
    try:
        context.integerish_value_ids = collect_integerish_value_ids(blocks)
        builder.resolver.integerish_ids = context.integerish_value_ids
    except Exception:
        context.integerish_value_ids = set()
        builder.resolver.integerish_ids = context.integerish_value_ids

    # Conservative ArrayBox-handle analysis for RuntimeData mono-route (AS-03).
    try:
        context.resolver_array_ids = collect_arrayish_value_ids(blocks)
        builder.resolver.array_ids = context.resolver_array_ids
    except Exception:
        context.resolver_array_ids = set()
        builder.resolver.array_ids = context.resolver_array_ids

    # Conservative StringBox-handle analysis (cleanup-11):
    # infer stringish receiver/value facts before lowering order reaches length/size sites.
    try:
        inferred_stringish = collect_stringish_value_ids(blocks)
        context.resolver_string_ids.clear()
        context.resolver_string_ids.update(inferred_stringish)
        builder.resolver.string_ids = context.resolver_string_ids
    except Exception:
        context.resolver_string_ids.clear()
        builder.resolver.string_ids = context.resolver_string_ids

    # Create or reuse function
    func = None
    for f in builder.module.functions:
        if f.name == name:
            func = f
            break
    if func is None:
        func = ir.Function(builder.module, func_ty, name=name)

    # Map parameters to vmap.
    #
    # SSOT: If `func_data["params"]` is present, it defines the ValueId ↔ arg position contract.
    # Use it first to avoid heuristic mis-mapping (which can silently ignore some parameters).
    #
    # Fallback: If params are missing (older JSON / legacy emit), use a heuristic:
    # - map "used but not defined" ValueIds to args in ascending ValueId order.
    try:
        arity = len(func.args)
        params_list = func_data.get("params", []) or []
        param_value_ids: List[int] = []

        if (
            isinstance(params_list, list)
            and len(params_list) == arity
            and all(isinstance(v, int) for v in params_list)
        ):
            for i in range(arity):
                builder.vmap[int(params_list[i])] = func.args[i]
                param_value_ids.append(int(params_list[i]))
        else:
            # Collect defined and used ids
            defs = set()
            uses = set()
            for bb in (blocks or []):
                for ins in (bb.get('instructions') or []):
                    try:
                        dstv = ins.get('dst')
                        if isinstance(dstv, int):
                            defs.add(int(dstv))
                    except Exception:
                        pass

                    for k in ('lhs', 'rhs', 'value', 'cond', 'box_val', 'box', 'src'):
                        try:
                            v = ins.get(k)
                            if isinstance(v, int):
                                uses.add(int(v))
                        except Exception:
                            pass

                    # List operands
                    try:
                        a = ins.get('args')
                        if isinstance(a, list):
                            for v in a:
                                if isinstance(v, int):
                                    uses.add(int(v))
                    except Exception:
                        pass

                    # Unified calls: mir_call.args
                    try:
                        mc = ins.get('mir_call')
                        if isinstance(mc, dict):
                            a = mc.get('args')
                            if isinstance(a, list):
                                for v in a:
                                    if isinstance(v, int):
                                        uses.add(int(v))
                    except Exception:
                        pass

            cand = [vid for vid in uses if vid not in defs]
            cand.sort()
            for i in range(min(arity, len(cand))):
                builder.vmap[int(cand[i])] = func.args[i]
                param_value_ids.append(int(cand[i]))

            # Legacy fallback: map positional 0..arity-1 only when params are missing.
            for i in range(arity):
                if i not in builder.vmap:
                    builder.vmap[i] = func.args[i]
                if len(param_value_ids) <= i:
                    param_value_ids.append(i)

        _seed_hakocli_args_array_fact(
            func_name=name,
            params_list=params_list if isinstance(params_list, list) else [],
            param_value_ids=param_value_ids,
            builder=builder,
        )
        _propagate_arrayish_value_facts(builder, blocks)
    except Exception:
        pass

    # Build predecessor map from control-flow edges
    builder.preds = {}
    for block_data in blocks:
        bid = block_data.get("id", 0)
        builder.preds.setdefault(bid, [])
    for block_data in blocks:
        src = block_data.get("id", 0)
        for inst in block_data.get("instructions", []):
            op = inst.get("op")
            if op == "jump":
                t = inst.get("target")
                if t is not None:
                    builder.preds.setdefault(t, []).append(src)
            elif op == "branch":
                th = inst.get("then")
                el = inst.get("else")
                if th is not None:
                    builder.preds.setdefault(th, []).append(src)
                if el is not None:
                    builder.preds.setdefault(el, []).append(src)

    # Create all blocks first
    for block_data in blocks:
        bid = block_data.get("id", 0)
        block_name = f"bb{bid}"
        bb = func.append_basic_block(block_name)
        builder.bb_map[bid] = bb

    # Build quick lookup for blocks by id
    block_by_id: Dict[int, Dict[str, Any]] = {}
    for block_data in blocks:
        block_by_id[block_data.get("id", 0)] = block_data

    # FAST compare contract: identify compare results consumed only by branch cond.
    # This allows compare lowering to keep those values as i1 in hot loops.
    try:
        context.fast_branch_only_compare_dsts = _collect_branch_only_compare_dsts(blocks)
        builder.resolver.fast_branch_only_compare_dsts = context.fast_branch_only_compare_dsts
    except Exception:
        context.fast_branch_only_compare_dsts = set()

    # Determine entry block: first with no predecessors; fallback to first block
    entry_bid = None
    for bid, preds in builder.preds.items():
        if len(preds) == 0:
            entry_bid = bid
            break
    if entry_bid is None and blocks:
        entry_bid = blocks[0].get("id", 0)

    # Function-local entry metadata for dominance-safe hoist paths.
    try:
        context.entry_block_id = int(entry_bid) if entry_bid is not None else None
        context.entry_block = builder.bb_map.get(int(entry_bid)) if entry_bid is not None else None
        builder.resolver.entry_block_id = context.entry_block_id
        builder.resolver.entry_block = context.entry_block
    except Exception:
        context.entry_block_id = None
        context.entry_block = None

    # Compute reverse-postorder over successors (SSOT):
    # - Ensures a stable, mostly-forward lowering order (preds before succs) even with loops.
    # - Avoids lowering a block before its dominating setup/copies when possible.
    visited: set[int] = set()
    post: List[int] = []
    try:
        from cfg.utils import (
            build_preds_succs as _build_preds_succs,
            compute_dominators as _compute_dominators,
        )
        _preds2, succs2 = _build_preds_succs(block_by_id)
        if entry_bid is not None:
            context.block_dominators = _compute_dominators(int(entry_bid), _preds2, succs2)
        else:
            context.block_dominators = {}
    except Exception:
        succs2 = {}
        context.block_dominators = {}

    def dfs(bid: int):
        if bid in visited:
            return
        visited.add(bid)
        try:
            succ_list = list(succs2.get(bid, []) or [])
            succ_list = [int(x) for x in succ_list]
            succ_list.sort()
        except Exception:
            succ_list = []
        for s in succ_list:
            dfs(s)
        post.append(bid)

    reachable_from_entry: Set[int] = set()
    if entry_bid is not None:
        dfs(int(entry_bid))
        reachable_from_entry = set(visited)
    # Include unreachable blocks deterministically
    for bid in sorted(block_by_id.keys()):
        if bid not in visited:
            dfs(int(bid))
    order: List[int] = list(reversed(post))

    try:
        context.reachable_block_ids = reachable_from_entry
        builder.resolver.reachable_block_ids = reachable_from_entry
    except Exception:
        context.reachable_block_ids = set()

    # Prepass: collect PHI metadata and placeholders
    _setup_phi_placeholders(builder, blocks)

    # Optional: if-merge prepass (gate NYASH_LLVM_PREPASS_IFMERGE)
    try:
        if os.environ.get('NYASH_LLVM_PREPASS_IFMERGE') == '1':
            plan = plan_ret_phi_predeclare(block_by_id)
            if plan:
                # Phase 132-P1: block_phi_incomings already points to context storage
                # No need to reassign - just ensure it exists
                pass
                for bbid, ret_vid in plan.items():
                    try:
                        preds_raw = [p for p in builder.preds.get(bbid, []) if p != bbid]
                    except Exception:
                        preds_raw = []
                    seen = set(); preds_list = []
                    for p in preds_raw:
                        if p not in seen:
                            preds_list.append(p); seen.add(p)
                    try:
                        builder.block_phi_incomings.setdefault(int(bbid), {})[int(ret_vid)] = [
                            (int(p), int(ret_vid)) for p in preds_list
                        ]
                    except Exception:
                        pass
                    try:
                        trace_debug(f"[prepass] if-merge: plan metadata at bb{bbid} for v{ret_vid} preds={preds_list}")
                    except Exception:
                        pass
    except Exception:
        pass

    # Predeclare PHIs for used-in-block values defined in predecessors (multi-pred only)
    try:
        # Phase 132-P1: block_phi_incomings already points to context storage
        # No need to reassign - it's already initialized
        _seed_multi_pred_block_phi_incomings(builder, block_by_id)
    except Exception:
        pass

    # Optional: simple loop prepass
    loop_plan = None
    try:
        if os.environ.get('NYASH_LLVM_PREPASS_LOOP') == '1':
            loop_plan = detect_simple_while(block_by_id)
            if loop_plan is not None:
                trace_debug(f"[prepass] detect loop header=bb{loop_plan['header']} then=bb{loop_plan['then']} latch=bb{loop_plan['latch']} exit=bb{loop_plan['exit']}")
    except Exception:
        loop_plan = None

    # Phase 131-4 Pass A: Lower non-terminator instructions (terminators deferred)
    # Phase 132-P1: Pass context Box for function-local state isolation
    from builders.block_lower import lower_blocks as _lower_blocks
    _lower_blocks(builder, func, block_by_id, order, loop_plan, context)

    # Phase 131-14-B Pass B: Resolve jump-only block snapshots (BEFORE PHI finalization)
    # Phase 132-P1: Pass context Box for function-local state isolation
    from builders.block_lower import resolve_jump_only_snapshots as _resolve_jump_only_snapshots
    _resolve_jump_only_snapshots(builder, block_by_id, context)

    # Phase 132-P2: Dict ctx removed; FunctionLowerContext is now SSOT
    # All context access goes through owner.context (passed to instruction handlers)

    # Phase 131-4 Pass B (now Pass B2): Finalize PHIs (wires incoming edges)
    # Phase 132-P1: Pass context Box for function-local state isolation
    _finalize_phis(builder, context)

    # Phase 131-4 Pass C: Lower deferred terminators (after PHIs are placed)
    from builders.block_lower import lower_terminators as _lower_terminators
    _lower_terminators(builder, func)

    # Phase 277 P1: Verify PHI ordering after all instructions are emitted
    from phi_placement import verify_phi_ordering
    from phi_wiring.debug_helper import is_phi_strict_enabled, is_phi_debug_enabled
    import sys

    ordering_results = verify_phi_ordering(builder)

    # Check results
    failed_blocks = [bid for bid, ok in ordering_results.items() if not ok]

    if failed_blocks:
        msg = f"[function_lower/PHI] {len(failed_blocks)} blocks have incorrect PHI ordering: {failed_blocks}"

        if is_phi_strict_enabled():
            print(f"[CRITICAL] {msg}", file=sys.stderr)
            print(f"  → Blocks: {failed_blocks}", file=sys.stderr)
            print(f"  → Required order: PHI → non-PHI → terminator", file=sys.stderr)
            raise RuntimeError(msg)

        if is_phi_debug_enabled():
            print(f"[WARNING] {msg}", file=sys.stderr)

    elif is_phi_debug_enabled():
        print(f"[function_lower/PHI] ✅ All {len(ordering_results)} blocks have correct PHI ordering", file=sys.stderr)

    # Safety pass: ensure every basic block ends with a terminator.
    # This avoids llvmlite IR parse errors like "expected instruction opcode" on empty blocks.
    try:
        _enforce_terminators(builder, func, block_by_id)
    except Exception:
        # Non-fatal in bring-up; better to emit IR than crash
        pass
    try:
        _emit_hot_summary(context)
    except Exception:
        pass


def _enforce_terminators(builder, func: ir.Function, block_by_id: Dict[int, Dict[str, Any]]):
    import re
    succs = _build_succs(getattr(builder, 'preds', {}) or {})
    for bb in func.blocks:
        try:
            if bb.terminator is not None:
                continue
        except Exception:
            # If property access fails, try to add a branch/ret anyway
            pass
        # Parse block id from name like "bb123"
        bid = None
        try:
            m = re.match(r"bb(\d+)$", str(bb.name))
            bid = int(m.group(1)) if m else None
        except Exception:
            bid = None
        # Choose a reasonable successor if any
        target_bb = None
        if bid is not None:
            for s in (succs.get(int(bid), []) or []):
                try:
                    cand = builder.bb_map.get(int(s))
                except Exception:
                    cand = None
                if cand is not None and cand is not bb:
                    target_bb = cand
                    break
        ib = ir.IRBuilder(bb)
        if target_bb is not None:
            try:
                ib.position_at_end(bb)
            except Exception:
                pass
            ib.branch(target_bb)
            try:
                trace_debug(f"[llvm-py] enforce_terminators: br from {bb.name} -> {target_bb.name}")
            except Exception:
                pass
            continue
        # Fallback: insert a return of 0 matching function return type (i32 for ny_main, else i64)
        try:
            rty = func.function_type.return_type
            if str(rty) == str(builder.i32):
                ib.ret(ir.Constant(builder.i32, 0))
            elif str(rty) == str(builder.i64):
                ib.ret(ir.Constant(builder.i64, 0))
            else:
                # Unknown/void – synthesize a dummy br to self to keep parser happy (unreachable in practice)
                ib.branch(bb)
            try:
                trace_debug(f"[llvm-py] enforce_terminators: ret/br injected in {bb.name}")
            except Exception:
                pass
        except Exception:
            # Last resort: do nothing
            pass
