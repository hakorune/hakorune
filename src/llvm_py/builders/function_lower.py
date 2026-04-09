from typing import Dict, Any, List, Set, Tuple

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
from instructions.user_box_local import seed_local_user_box_layouts_from_function_data
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
            elif op == "field_get":
                add_use(ins.get("box"))
            elif op == "field_set":
                add_use(ins.get("box"))
                add_use(ins.get("value"))
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
        for key in ("lhs", "rhs", "value", "cond", "box_val", "box"):
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


def _seed_if_merge_ret_phi_incomings(builder, plan: Dict[int, int]) -> None:
    for bbid, ret_vid in (plan or {}).items():
        preds_list = _dedup_non_self_preds(getattr(builder, "preds", {}) or {}, int(bbid))
        try:
            builder.block_phi_incomings.setdefault(int(bbid), {})[int(ret_vid)] = [
                (int(pred_bid), int(ret_vid)) for pred_bid in preds_list
            ]
        except Exception:
            pass
        try:
            trace_debug(f"[prepass] if-merge: plan metadata at bb{bbid} for v{ret_vid} preds={preds_list}")
        except Exception:
            pass
    try:
        builder.resolver.block_phi_incomings = builder.block_phi_incomings
    except Exception:
        pass


def _run_if_merge_prepass(builder, block_by_id: Dict[int, Dict[str, Any]]) -> None:
    import os

    if os.environ.get("NYASH_LLVM_PREPASS_IFMERGE") != "1":
        return
    plan = plan_ret_phi_predeclare(block_by_id)
    if plan:
        _seed_if_merge_ret_phi_incomings(builder, plan)


def _run_loop_prepass(block_by_id: Dict[int, Dict[str, Any]]):
    import os

    if os.environ.get("NYASH_LLVM_PREPASS_LOOP") != "1":
        return None
    loop_plan = detect_simple_while(block_by_id)
    if loop_plan is not None:
        trace_debug(
            f"[prepass] detect loop header=bb{loop_plan['header']} then=bb{loop_plan['then']} "
            f"latch=bb{loop_plan['latch']} exit=bb{loop_plan['exit']}"
        )
    return loop_plan


def _determine_entry_block_id(preds_map: Dict[int, List[int]], blocks: List[Dict[str, Any]]):
    for bid, preds in preds_map.items():
        if len(preds) == 0:
            return bid
    if blocks:
        return blocks[0].get("id", 0)
    return None


def _compute_lower_order(
    block_by_id: Dict[int, Dict[str, Any]],
    entry_bid,
) -> Tuple[List[int], Set[int], Dict[int, Set[int]]]:
    visited: set[int] = set()
    post: List[int] = []
    succs2: Dict[int, List[int]] = {}
    block_dominators: Dict[int, Set[int]] = {}

    try:
        from cfg.utils import (
            build_preds_succs as _build_preds_succs,
            compute_dominators as _compute_dominators,
        )

        _preds2, succs2 = _build_preds_succs(block_by_id)
        if entry_bid is not None:
            block_dominators = _compute_dominators(int(entry_bid), _preds2, succs2)
    except Exception:
        succs2 = {}
        block_dominators = {}

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
        for succ_bid in succ_list:
            dfs(succ_bid)
        post.append(bid)

    reachable_from_entry: Set[int] = set()
    if entry_bid is not None:
        dfs(int(entry_bid))
        reachable_from_entry = set(visited)
    for bid in sorted(block_by_id.keys()):
        if bid not in visited:
            dfs(int(bid))

    return list(reversed(post)), reachable_from_entry, block_dominators


def _enforce_phi_ordering_contract(builder) -> None:
    from phi_placement import verify_phi_ordering
    from phi_wiring.debug_helper import is_phi_strict_enabled, is_phi_debug_enabled
    import sys

    ordering_results = verify_phi_ordering(builder)
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
        return

    if is_phi_debug_enabled():
        print(f"[function_lower/PHI] ✅ All {len(ordering_results)} blocks have correct PHI ordering", file=sys.stderr)


def _run_finalize_tail(builder, func: ir.Function, block_by_id: Dict[int, Dict[str, Any]], context: FunctionLowerContext) -> None:
    from builders.block_lower import lower_terminators as _lower_terminators

    _finalize_phis(builder, context)
    _lower_terminators(builder, func)
    _enforce_phi_ordering_contract(builder)
    try:
        _enforce_terminators(builder, func, block_by_id)
    except Exception:
        pass
    try:
        _emit_hot_summary(context)
    except Exception:
        pass


def _build_function_type(builder, name: str, params: List[Any]) -> ir.FunctionType:
    import re

    if name == "ny_main":
        return ir.FunctionType(builder.i64, [])

    m = re.search(r"/(\d+)$", name)
    arity = int(m.group(1)) if m else len(params)
    if arity == 0 and "." in name:
        try:
            arity = int(builder.call_arities.get(name, 0))
        except Exception:
            pass
    return ir.FunctionType(builder.i64, [builder.i64] * arity)


def _get_or_create_function(builder, name: str, func_ty: ir.FunctionType) -> ir.Function:
    for func in builder.module.functions:
        if func.name == name:
            return func
    return ir.Function(builder.module, func_ty, name=name)


def _collect_param_candidate_value_ids(blocks: List[Dict[str, Any]]) -> List[int]:
    defs = set()
    uses = set()
    for block in (blocks or []):
        for ins in (block.get("instructions") or []):
            try:
                dstv = ins.get("dst")
                if isinstance(dstv, int):
                    defs.add(int(dstv))
            except Exception:
                pass

            for key in ("lhs", "rhs", "value", "cond", "box_val", "box", "src"):
                try:
                    value = ins.get(key)
                    if isinstance(value, int):
                        uses.add(int(value))
                except Exception:
                    pass

            try:
                args = ins.get("args")
                if isinstance(args, list):
                    for value in args:
                        if isinstance(value, int):
                            uses.add(int(value))
            except Exception:
                pass

            try:
                mir_call = ins.get("mir_call")
                if isinstance(mir_call, dict):
                    args = mir_call.get("args")
                    if isinstance(args, list):
                        for value in args:
                            if isinstance(value, int):
                                uses.add(int(value))
            except Exception:
                pass

    candidates = [vid for vid in uses if vid not in defs]
    candidates.sort()
    return candidates


def _map_function_params_to_vmap(builder, func, params_list: List[Any], blocks: List[Dict[str, Any]]) -> List[int]:
    arity = len(func.args)
    param_value_ids: List[int] = []

    if (
        isinstance(params_list, list)
        and len(params_list) == arity
        and all(isinstance(value, int) for value in params_list)
    ):
        for index in range(arity):
            builder.vmap[int(params_list[index])] = func.args[index]
            param_value_ids.append(int(params_list[index]))
        return param_value_ids

    candidates = _collect_param_candidate_value_ids(blocks)
    for index in range(min(arity, len(candidates))):
        builder.vmap[int(candidates[index])] = func.args[index]
        param_value_ids.append(int(candidates[index]))

    for index in range(arity):
        if index not in builder.vmap:
            builder.vmap[index] = func.args[index]
        if len(param_value_ids) <= index:
            param_value_ids.append(index)

    return param_value_ids


def _build_predecessor_map(blocks: List[Dict[str, Any]]) -> Dict[int, List[int]]:
    preds: Dict[int, List[int]] = {}
    for block_data in blocks:
        bid = block_data.get("id", 0)
        preds.setdefault(bid, [])
    for block_data in blocks:
        src = block_data.get("id", 0)
        for inst in block_data.get("instructions", []):
            op = inst.get("op")
            if op == "jump":
                target = inst.get("target")
                if target is not None:
                    preds.setdefault(target, []).append(src)
            elif op == "branch":
                then_bid = inst.get("then")
                else_bid = inst.get("else")
                if then_bid is not None:
                    preds.setdefault(then_bid, []).append(src)
                if else_bid is not None:
                    preds.setdefault(else_bid, []).append(src)
    return preds


def _create_basic_blocks(builder, func: ir.Function, blocks: List[Dict[str, Any]]) -> None:
    for block_data in blocks:
        bid = block_data.get("id", 0)
        builder.bb_map[bid] = func.append_basic_block(f"bb{bid}")


def _index_blocks_by_id(blocks: List[Dict[str, Any]]) -> Dict[int, Dict[str, Any]]:
    return {block_data.get("id", 0): block_data for block_data in blocks}


def _reset_function_lower_state(builder) -> None:
    try:
        builder.vmap.clear()
    except Exception:
        builder.vmap = {}
    try:
        builder.bb_map.clear()
    except Exception:
        builder.bb_map = {}
    try:
        builder.predeclared_ret_phis.clear()
    except Exception:
        try:
            builder.predeclared_ret_phis = {}
        except Exception:
            pass


def _create_function_context(builder, name: str) -> FunctionLowerContext:
    context = FunctionLowerContext(name)
    context.phi_manager = PhiManager()

    builder.phi_manager = context.phi_manager
    builder.block_phi_incomings = context.block_phi_incomings
    builder.phi_trivial_aliases = context.phi_trivial_aliases
    builder.def_blocks = context.def_blocks
    builder.block_end_values = context.block_end_values
    builder.resolver.bind_context(context)
    builder.context = context
    return context


def _load_value_types_metadata(builder, func_data: Dict[str, Any]) -> None:
    try:
        metadata = func_data.get("metadata", {})
        value_types_json = metadata.get("value_types", {})
        builder.resolver.value_types = {}
        for vid_str, vtype in value_types_json.items():
            try:
                builder.resolver.value_types[int(vid_str)] = vtype
            except (ValueError, TypeError):
                pass
    except Exception:
        builder.resolver.value_types = {}


def _load_thin_entry_selection_metadata(builder, func_data: Dict[str, Any]) -> None:
    try:
        metadata = func_data.get("metadata", {})
        rows = metadata.get("thin_entry_selections", [])

        normalized_rows = []
        by_value = {}
        by_subject = {}
        for row in rows:
            try:
                if not isinstance(row, dict):
                    continue
                normalized = dict(row)
                surface = normalized.get("surface")
                subject = normalized.get("subject")
                value = normalized.get("value")
                if isinstance(value, int):
                    normalized["value"] = int(value)
                    by_value.setdefault(int(value), []).append(normalized)
                else:
                    normalized["value"] = None
                if isinstance(surface, str) and isinstance(subject, str):
                    by_subject.setdefault((surface, subject), []).append(normalized)
                normalized_rows.append(normalized)
            except Exception:
                pass

        builder.resolver.thin_entry_selections = normalized_rows
        builder.resolver.thin_entry_selection_by_value = by_value
        builder.resolver.thin_entry_selection_by_subject = by_subject
    except Exception:
        builder.resolver.thin_entry_selections = []
        builder.resolver.thin_entry_selection_by_value = {}
        builder.resolver.thin_entry_selection_by_subject = {}


def _load_sum_placement_metadata(builder, func_data: Dict[str, Any]) -> None:
    try:
        metadata = func_data.get("metadata", {})
        selections = metadata.get("sum_placement_selections", [])
        layouts = metadata.get("sum_placement_layouts", [])

        local_paths = {}
        for row in selections:
            try:
                if row.get("surface") != "sum_make":
                    continue
                if row.get("selected_path") != "local_aggregate":
                    continue
                value = row.get("value")
                if isinstance(value, int):
                    local_paths[int(value)] = "local_aggregate"
            except Exception:
                pass

        local_layouts = {}
        for row in layouts:
            try:
                if row.get("surface") != "sum_make":
                    continue
                value = row.get("value")
                layout = row.get("layout")
                if isinstance(value, int) and isinstance(layout, str):
                    local_layouts[int(value)] = layout
            except Exception:
                pass

        builder.resolver.sum_local_aggregate_paths = local_paths
        builder.resolver.sum_local_aggregate_layouts = local_layouts
    except Exception:
        builder.resolver.sum_local_aggregate_paths = {}
        builder.resolver.sum_local_aggregate_layouts = {}


def _load_user_box_local_aggregate_metadata(builder, func_data: Dict[str, Any]) -> None:
    try:
        seed_local_user_box_layouts_from_function_data(builder, func_data)
    except Exception:
        builder.resolver.user_box_local_aggregate_layouts = {}


def _seed_resolver_fact_sets(builder, context: FunctionLowerContext, blocks: List[Dict[str, Any]]) -> None:
    try:
        context.non_negative_value_ids = collect_non_negative_value_ids(blocks)
        builder.resolver.non_negative_ids = context.non_negative_value_ids
    except Exception:
        context.non_negative_value_ids = set()
        builder.resolver.non_negative_ids = context.non_negative_value_ids

    try:
        context.integerish_value_ids = collect_integerish_value_ids(blocks)
        builder.resolver.integerish_ids = context.integerish_value_ids
    except Exception:
        context.integerish_value_ids = set()
        builder.resolver.integerish_ids = context.integerish_value_ids

    try:
        context.resolver_array_ids = collect_arrayish_value_ids(blocks)
        builder.resolver.array_ids = context.resolver_array_ids
    except Exception:
        context.resolver_array_ids = set()
        builder.resolver.array_ids = context.resolver_array_ids

    try:
        inferred_stringish = collect_stringish_value_ids(blocks)
        context.resolver_string_ids.clear()
        context.resolver_string_ids.update(inferred_stringish)
        builder.resolver.string_ids = context.resolver_string_ids
    except Exception:
        context.resolver_string_ids.clear()
        builder.resolver.string_ids = context.resolver_string_ids


def lower_function(builder, func_data: Dict[str, Any]):
    """Lower a single MIR function to LLVM IR using the given builder context.
    This is a faithful extraction of NyashLLVMBuilder.lower_function.
    """
    import os

    name = func_data.get("name", "unknown")
    builder.current_function_name = name
    params = func_data.get("params", [])
    blocks = func_data.get("blocks", [])

    # Determine function signature
    func_ty = _build_function_type(builder, name, params)

    # Reset per-function maps and resolver caches to avoid cross-function collisions
    _reset_function_lower_state(builder)

    # Phase 132-P1: Create function-local context Box
    # This automatically isolates all function-scoped state
    context = _create_function_context(builder, name)

    # Phase 131-15-P1: Load value_types metadata from JSON into resolver
    _load_value_types_metadata(builder, func_data)
    _load_thin_entry_selection_metadata(builder, func_data)
    _load_sum_placement_metadata(builder, func_data)
    _load_user_box_local_aggregate_metadata(builder, func_data)

    # Conservative sign analysis for power-of-two modulo fast path.
    _seed_resolver_fact_sets(builder, context, blocks)

    # Create or reuse function
    func = _get_or_create_function(builder, name, func_ty)

    # Map parameters to vmap.
    #
    # SSOT: If `func_data["params"]` is present, it defines the ValueId ↔ arg position contract.
    # Use it first to avoid heuristic mis-mapping (which can silently ignore some parameters).
    #
    # Fallback: If params are missing (older JSON / legacy emit), use a heuristic:
    # - map "used but not defined" ValueIds to args in ascending ValueId order.
    try:
        params_list = func_data.get("params", []) or []
        param_value_ids = _map_function_params_to_vmap(builder, func, params_list, blocks)

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
    builder.preds = _build_predecessor_map(blocks)

    # Create all blocks first
    _create_basic_blocks(builder, func, blocks)

    # Build quick lookup for blocks by id
    block_by_id = _index_blocks_by_id(blocks)

    # FAST compare contract: identify compare results consumed only by branch cond.
    # This allows compare lowering to keep those values as i1 in hot loops.
    try:
        context.fast_branch_only_compare_dsts = _collect_branch_only_compare_dsts(blocks)
        builder.resolver.fast_branch_only_compare_dsts = context.fast_branch_only_compare_dsts
    except Exception:
        context.fast_branch_only_compare_dsts = set()

    # Determine entry block: first with no predecessors; fallback to first block
    entry_bid = _determine_entry_block_id(builder.preds, blocks)

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
    order, reachable_from_entry, context.block_dominators = _compute_lower_order(block_by_id, entry_bid)

    try:
        context.reachable_block_ids = reachable_from_entry
        builder.resolver.reachable_block_ids = reachable_from_entry
    except Exception:
        context.reachable_block_ids = set()

    # Prepass: collect PHI metadata and placeholders
    _setup_phi_placeholders(builder, blocks)

    # Optional: if-merge prepass (gate NYASH_LLVM_PREPASS_IFMERGE)
    try:
        _run_if_merge_prepass(builder, block_by_id)
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
    try:
        loop_plan = _run_loop_prepass(block_by_id)
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

    # Finalize PHIs, lower deferred terminators, verify PHI ordering,
    # then synthesize missing terminators / hot summary as non-fatal tail work.
    _run_finalize_tail(builder, func, block_by_id, context)


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
