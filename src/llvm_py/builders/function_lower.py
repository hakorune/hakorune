import re
from typing import Dict, Any, List, Optional, Set, Tuple

from llvmlite import ir
from trace import debug as trace_debug
from trace import hot as trace_hot
from trace import hot_enabled as trace_hot_enabled
from trace import format_hot_summary as trace_format_hot_summary
from prepass.if_merge import plan_ret_phi_predeclare
from prepass.loops import annotate_numeric_loop_plan, detect_simple_while
from builders.loop_simd_contract import build_loop_simd_contract
from cfg.utils import (
    collect_arrayish_value_ids,
    collect_integerish_value_ids,
    collect_non_negative_value_ids,
    propagate_arrayish_value_ids,
    collect_stringish_value_ids,
)
from builders.function_metadata import (
    _load_direct_array_access_plan_metadata,
    _load_exact_numeric_route_metadata,
    _load_fastmem_access_plan_metadata,
    _load_sum_placement_metadata,
    _load_thin_entry_selection_metadata,
    _load_user_box_local_aggregate_metadata,
    _load_value_types_metadata,
    _seed_resolver_fact_sets as _metadata_seed_resolver_fact_sets,
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

    resolver = getattr(builder, "resolver", None)
    if resolver is None:
        return

    array_ids = getattr(resolver, "array_ids", None)
    if not isinstance(array_ids, set):
        array_ids = set(array_ids or [])
        resolver.array_ids = array_ids
    array_ids.add(vid)

    value_types = getattr(resolver, "value_types", None)
    if not isinstance(value_types, dict):
        value_types = {}
        resolver.value_types = value_types
    value_types[vid] = {"kind": "handle", "box_type": "ArrayBox"}


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

    method_name = _hako_cli_method_name(name)
    if method_name is None:
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


def _hako_cli_method_name(func_name: str) -> str | None:
    parts = str(func_name or "").split(".", 1)
    if len(parts) != 2:
        return None
    return parts[1].split("/", 1)[0]


def _propagate_arrayish_value_facts(builder, blocks: List[Dict[str, Any]]) -> None:
    """Expand seeded ArrayBox facts across copy/phi carrier chains."""
    resolver = getattr(builder, "resolver", None)
    if resolver is None:
        return

    seeded = set(getattr(resolver, "array_ids", set()) or set())
    propagated = propagate_arrayish_value_ids(blocks, seeded)

    array_ids = getattr(resolver, "array_ids", None)
    if not isinstance(array_ids, set):
        resolver.array_ids = set(propagated)
    else:
        array_ids.clear()
        array_ids.update(propagated)

    for vid in propagated:
        _mark_arrayish_param_fact(builder, int(vid))


def _dedup_non_self_preds(preds_map: Dict[int, List[int]], block_id: int) -> List[int]:
    bid = _as_int_or_none(block_id)
    if bid is None:
        return []
    preds_raw = [p for p in preds_map.get(bid, []) if p != bid]
    seen = set()
    preds_list: List[int] = []
    for pred_bid in preds_raw:
        if pred_bid not in seen:
            preds_list.append(pred_bid)
            seen.add(pred_bid)
    return preds_list


def _as_int_or_none(value) -> int | None:
    try:
        return int(value)
    except (TypeError, ValueError):
        return None


def _collect_block_defs(block: Dict[str, Any]) -> set[int]:
    defs: set[int] = set()
    for ins in block.get("instructions") or []:
        dstv = ins.get("dst")
        if isinstance(dstv, int):
            defs.add(int(dstv))
    return defs


def _collect_block_uses(block: Dict[str, Any]) -> set[int]:
    uses: set[int] = set()
    for ins in block.get("instructions") or []:
        for key in ("lhs", "rhs", "value", "cond", "box_val", "box"):
            value = ins.get(key)
            if isinstance(value, int):
                uses.add(int(value))
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
            builder.block_phi_incomings.setdefault(int(bid), {})[int(vid)] = [
                (int(pred_bid), int(vid)) for pred_bid in preds_list
            ]
    resolver = getattr(builder, "resolver", None)
    if resolver is not None:
        resolver.block_phi_incomings = builder.block_phi_incomings


def _seed_if_merge_ret_phi_incomings(builder, plan: Dict[int, int]) -> None:
    for bbid, ret_vid in (plan or {}).items():
        preds_list = _dedup_non_self_preds(getattr(builder, "preds", {}) or {}, int(bbid))
        builder.block_phi_incomings.setdefault(int(bbid), {})[int(ret_vid)] = [
            (int(pred_bid), int(ret_vid)) for pred_bid in preds_list
        ]
        trace_debug(f"[prepass] if-merge: plan metadata at bb{bbid} for v{ret_vid} preds={preds_list}")
    resolver = getattr(builder, "resolver", None)
    if resolver is not None:
        resolver.block_phi_incomings = builder.block_phi_incomings


def _run_if_merge_prepass(builder, block_by_id: Dict[int, Dict[str, Any]]) -> None:
    import os

    if os.environ.get("NYASH_LLVM_PREPASS_IFMERGE") != "1":
        return
    plan = plan_ret_phi_predeclare(block_by_id)
    if plan:
        _seed_if_merge_ret_phi_incomings(builder, plan)


def _run_loop_prepass(block_by_id: Dict[int, Dict[str, Any]], context: FunctionLowerContext | None = None):
    import os

    if os.environ.get("NYASH_LLVM_PREPASS_LOOP") != "1":
        return None
    loop_plan = detect_simple_while(block_by_id)
    if loop_plan is not None:
        trace_debug(
            f"[prepass] detect loop header=bb{loop_plan['header']} then=bb{loop_plan['then']} "
            f"latch=bb{loop_plan['latch']} exit=bb{loop_plan['exit']}"
        )
        if context is not None:
            annotated = _try_annotate_numeric_loop_plan(block_by_id, loop_plan, context)
            if annotated is not None:
                loop_plan = annotated
                header_bid = int(loop_plan.get("header"))
                context.numeric_loop_plans[header_bid] = loop_plan
                trace_debug(
                    "[prepass] numeric-loop induction "
                    f"header=bb{loop_plan['header']} candidates={loop_plan.get('numeric_induction_value_ids', [])}"
                )
                contract = _try_build_loop_simd_contract(loop_plan)
                if contract is not None:
                    context.loop_simd_contracts[header_bid] = contract
    return loop_plan


def _try_annotate_numeric_loop_plan(
    block_by_id: Dict[int, Dict[str, Any]],
    loop_plan: Dict[str, Any],
    context: FunctionLowerContext,
):
    try:
        return annotate_numeric_loop_plan(
            block_by_id,
            loop_plan,
            integerish_ids=getattr(context, "integerish_value_ids", None),
            non_negative_ids=getattr(context, "non_negative_value_ids", None),
        )
    except (AttributeError, KeyError, NotImplementedError, RuntimeError, TypeError, ValueError) as exc:
        trace_debug(f"[function-lower/numeric-loop-annotation-skip] fn={context.func_name}: {exc}")
        return None


def _try_build_loop_simd_contract(loop_plan):
    try:
        return build_loop_simd_contract(loop_plan)
    except (AttributeError, KeyError, NotImplementedError, RuntimeError, TypeError, ValueError) as exc:
        trace_debug(f"[function-lower/loop-simd-contract-skip] header={loop_plan.get('header')}: {exc}")
        return None


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
        succs2, block_dominators = _compute_successors_and_dominators(block_by_id, entry_bid)
    except (AttributeError, KeyError, NotImplementedError, RuntimeError, TypeError, ValueError) as exc:
        trace_debug(f"[function-lower/lower-order-fallback] entry={entry_bid}: {exc}")
        succs2 = {}
        block_dominators = {}

    def dfs(bid: int):
        if bid in visited:
            return
        visited.add(bid)
        succ_list = sorted(_int_values(succs2.get(bid, []) or []))
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


def _compute_successors_and_dominators(
    block_by_id: Dict[int, Dict[str, Any]],
    entry_bid,
) -> Tuple[Dict[int, List[int]], Dict[int, Set[int]]]:
    from cfg.utils import (
        build_preds_succs as _build_preds_succs,
        compute_dominators as _compute_dominators,
    )

    preds_map, succs = _build_preds_succs(block_by_id)
    dominators: Dict[int, Set[int]] = {}
    if entry_bid is not None:
        dominators = _compute_dominators(int(entry_bid), preds_map, succs)
    return succs, dominators


def _int_values(values) -> List[int]:
    result: List[int] = []
    for value in values:
        converted = _as_int_or_none(value)
        if converted is not None:
            result.append(converted)
    return result


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
    func_name = getattr(context, "func_name", "<unknown>")

    _finalize_phis(builder, context)
    _lower_terminators(builder, func)
    _enforce_phi_ordering_contract(builder)
    try:
        _enforce_terminators(builder, func, block_by_id)
    except (AttributeError, KeyError, NotImplementedError, RuntimeError, TypeError, ValueError) as exc:
        trace_debug(f"[function-lower/enforce-terminators-skip] fn={func_name}: {exc}")
    try:
        _emit_hot_summary(context)
    except (AttributeError, KeyError, NotImplementedError, RuntimeError, TypeError, ValueError) as exc:
        trace_debug(f"[function-lower/hot-summary-skip] fn={func_name}: {exc}")


def _build_function_type(builder, name: str, params: List[Any]) -> ir.FunctionType:
    import re

    if name == "ny_main":
        return ir.FunctionType(builder.i64, [])

    m = re.search(r"/(\d+)$", name)
    arity = int(m.group(1)) if m else len(params)
    if arity == 0 and "." in name:
        arity = int(getattr(builder, "call_arities", {}).get(name, 0))
    return ir.FunctionType(builder.i64, [builder.i64] * arity)


def _add_int_value(target: Set[int], value) -> None:
    if isinstance(value, int):
        target.add(int(value))


def _add_int_values(target: Set[int], values) -> None:
    if isinstance(values, list):
        for value in values:
            _add_int_value(target, value)


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
            _add_int_value(defs, ins.get("dst"))

            for key in ("lhs", "rhs", "value", "cond", "box_val", "box", "src"):
                _add_int_value(uses, ins.get(key))

            _add_int_values(uses, ins.get("args"))

            mir_call = ins.get("mir_call")
            if isinstance(mir_call, dict):
                _add_int_values(uses, mir_call.get("args"))

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
    _clear_or_reset_attr(builder, "vmap")
    _clear_or_reset_attr(builder, "bb_map")
    _clear_or_reset_attr(builder, "predeclared_ret_phis")


def _clear_or_reset_attr(owner, name: str) -> None:
    value = getattr(owner, name, None)
    if hasattr(value, "clear"):
        value.clear()
    else:
        setattr(owner, name, {})


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


def _merge_ipo_contracts_into_builder(builder, context: FunctionLowerContext) -> None:
    _merge_context_dict(
        builder,
        "ipo_callable_contracts_by_function",
        context.func_name,
        getattr(context, "ipo_callable_contracts", {}) or {},
    )
    _merge_context_dict(
        builder,
        "ipo_call_edge_contracts_by_function",
        context.func_name,
        getattr(context, "ipo_call_edge_contracts", {}) or {},
    )


def _merge_context_dict(builder, attr_name: str, func_name: str, values) -> None:
    target = getattr(builder, attr_name, None)
    if not isinstance(target, dict):
        target = {}
        setattr(builder, attr_name, target)
    target[func_name] = dict(values or {})


def _set_resolver_attr(builder, name: str, value) -> None:
    resolver = getattr(builder, "resolver", None)
    if resolver is not None:
        setattr(resolver, name, value)


def _seed_resolver_fact_sets(builder, context: FunctionLowerContext, blocks: List[Dict[str, Any]]) -> None:
    _metadata_seed_resolver_fact_sets(
        builder,
        context,
        blocks,
        collect_non_negative=collect_non_negative_value_ids,
        collect_integerish=collect_integerish_value_ids,
        collect_arrayish=collect_arrayish_value_ids,
        collect_stringish=collect_stringish_value_ids,
    )


def _map_params_and_seed_entry_facts(
    builder,
    func,
    *,
    func_name: str,
    func_data: Dict[str, Any],
    blocks: List[Dict[str, Any]],
) -> None:
    try:
        params_list = func_data.get("params", []) or []
        param_value_ids = _map_function_params_to_vmap(builder, func, params_list, blocks)
        _seed_hakocli_args_array_fact(
            func_name=func_name,
            params_list=params_list if isinstance(params_list, list) else [],
            param_value_ids=param_value_ids,
            builder=builder,
        )
        _propagate_arrayish_value_facts(builder, blocks)
    except (AttributeError, KeyError, NotImplementedError, RuntimeError, TypeError, ValueError) as exc:
        trace_debug(f"[function-lower/param-map-fallback] fn={func_name}: {exc}")


def _seed_fast_branch_compare_contract(builder, context: FunctionLowerContext, blocks: List[Dict[str, Any]]) -> None:
    try:
        context.fast_branch_only_compare_dsts = _collect_branch_only_compare_dsts(blocks)
        _set_resolver_attr(builder, "fast_branch_only_compare_dsts", context.fast_branch_only_compare_dsts)
    except (AttributeError, KeyError, NotImplementedError, RuntimeError, TypeError, ValueError) as exc:
        trace_debug(f"[function-lower/fast-compare-contract-skip] fn={context.func_name}: {exc}")
        context.fast_branch_only_compare_dsts = set()


def _set_entry_metadata(builder, context: FunctionLowerContext, entry_bid) -> None:
    try:
        context.entry_block_id = int(entry_bid) if entry_bid is not None else None
        context.entry_block = builder.bb_map.get(int(entry_bid)) if entry_bid is not None else None
        _set_resolver_attr(builder, "entry_block_id", context.entry_block_id)
        _set_resolver_attr(builder, "entry_block", context.entry_block)
    except (AttributeError, KeyError, NotImplementedError, RuntimeError, TypeError, ValueError) as exc:
        trace_debug(f"[function-lower/entry-metadata-skip] fn={context.func_name}: {exc}")
        context.entry_block_id = None
        context.entry_block = None


def _set_reachable_metadata(builder, context: FunctionLowerContext, reachable_from_entry: Set[int]) -> None:
    try:
        context.reachable_block_ids = reachable_from_entry
        _set_resolver_attr(builder, "reachable_block_ids", reachable_from_entry)
    except (AttributeError, KeyError, NotImplementedError, RuntimeError, TypeError, ValueError) as exc:
        trace_debug(f"[function-lower/reachable-metadata-skip] fn={context.func_name}: {exc}")
        context.reachable_block_ids = set()


def _run_optional_function_prepasses(builder, block_by_id: Dict[int, Dict[str, Any]], context: FunctionLowerContext):
    try:
        _run_if_merge_prepass(builder, block_by_id)
    except (AttributeError, KeyError, NotImplementedError, RuntimeError, TypeError, ValueError) as exc:
        trace_debug(f"[function-lower/if-merge-prepass-skip] fn={context.func_name}: {exc}")

    try:
        _seed_multi_pred_block_phi_incomings(builder, block_by_id)
    except (AttributeError, KeyError, NotImplementedError, RuntimeError, TypeError, ValueError) as exc:
        trace_debug(f"[function-lower/multi-pred-phi-seed-skip] fn={context.func_name}: {exc}")

    try:
        return _run_loop_prepass(block_by_id, context)
    except (AttributeError, KeyError, NotImplementedError, RuntimeError, TypeError, ValueError) as exc:
        trace_debug(f"[function-lower/loop-prepass-skip] fn={context.func_name}: {exc}")
        return None


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
    _load_exact_numeric_route_metadata(builder, func_data)
    _load_direct_array_access_plan_metadata(builder, func_data)
    _load_fastmem_access_plan_metadata(builder, func_data)

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
    _map_params_and_seed_entry_facts(
        builder,
        func,
        func_name=name,
        func_data=func_data,
        blocks=blocks,
    )

    # Build predecessor map from control-flow edges
    builder.preds = _build_predecessor_map(blocks)

    # Create all blocks first
    _create_basic_blocks(builder, func, blocks)

    # Build quick lookup for blocks by id
    block_by_id = _index_blocks_by_id(blocks)

    # FAST compare contract: identify compare results consumed only by branch cond.
    # This allows compare lowering to keep those values as i1 in hot loops.
    _seed_fast_branch_compare_contract(builder, context, blocks)

    # Determine entry block: first with no predecessors; fallback to first block
    entry_bid = _determine_entry_block_id(builder.preds, blocks)

    # Function-local entry metadata for dominance-safe hoist paths.
    _set_entry_metadata(builder, context, entry_bid)

    # Compute reverse-postorder over successors (SSOT):
    # - Ensures a stable, mostly-forward lowering order (preds before succs) even with loops.
    # - Avoids lowering a block before its dominating setup/copies when possible.
    order, reachable_from_entry, context.block_dominators = _compute_lower_order(block_by_id, entry_bid)

    _set_reachable_metadata(builder, context, reachable_from_entry)

    # Prepass: collect PHI metadata and placeholders
    _setup_phi_placeholders(builder, blocks)

    loop_plan = _run_optional_function_prepasses(builder, block_by_id, context)

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
    _merge_ipo_contracts_into_builder(builder, context)


def _enforce_terminators(builder, func: ir.Function, block_by_id: Dict[int, Dict[str, Any]]):
    succs = _build_succs(getattr(builder, 'preds', {}) or {})
    for bb in func.blocks:
        if bb.terminator is not None:
            continue
        bid = _parse_basic_block_id(bb)
        target_bb = _select_open_successor(builder, succs, bid, bb)
        ib = ir.IRBuilder(bb)
        if target_bb is not None:
            ib.position_at_end(bb)
            ib.branch(target_bb)
            trace_debug(f"[llvm-py] enforce_terminators: br from {bb.name} -> {target_bb.name}")
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
            trace_debug(f"[llvm-py] enforce_terminators: ret/br injected in {bb.name}")
        except (AttributeError, KeyError, NotImplementedError, RuntimeError, TypeError, ValueError) as exc:
            # Last resort: do nothing
            trace_debug(f"[llvm-py] enforce_terminators: skip {bb.name}: {exc}")


def _parse_basic_block_id(bb) -> int | None:
    m = re.match(r"bb(\d+)$", str(bb.name))
    return int(m.group(1)) if m else None


def _select_open_successor(builder, succs: Dict[int, List[int]], bid: int | None, current_bb):
    if bid is None:
        return None
    for succ_bid in (succs.get(int(bid), []) or []):
        cand = builder.bb_map.get(int(succ_bid))
        if cand is not None and cand is not current_bb:
            return cand
    return None
