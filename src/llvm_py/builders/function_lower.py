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
    _load_map_lookup_fusion_metadata,
    _load_map_repr_metadata,
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



from builders.function_lower_prepass import (
    _collect_branch_only_compare_dsts,
    _emit_hot_summary,
    _mark_arrayish_param_fact,
    _seed_hakocli_args_array_fact,
    _hako_cli_method_name,
    _propagate_arrayish_value_facts,
    _dedup_non_self_preds,
    _as_int_or_none,
    _collect_block_defs,
    _collect_block_uses,
    _seed_multi_pred_block_phi_incomings,
    _seed_if_merge_ret_phi_incomings,
    _run_if_merge_prepass,
    _run_loop_prepass,
    _try_annotate_numeric_loop_plan,
    _try_build_loop_simd_contract,
    _determine_entry_block_id,
    _compute_lower_order,
    _compute_successors_and_dominators,
    _int_values,
    _enforce_phi_ordering_contract,
    _run_finalize_tail,
    _build_function_type,
    _add_int_value,
    _add_int_values,
    _get_or_create_function,
    _collect_param_candidate_value_ids,
    _map_function_params_to_vmap,
    _build_predecessor_map,
    _create_basic_blocks,
    _index_blocks_by_id,
    _reset_function_lower_state,
    _clear_or_reset_attr,
    _create_function_context,
    _merge_ipo_contracts_into_builder,
    _merge_context_dict,
    _set_resolver_attr,
    _seed_resolver_fact_sets,
    _map_params_and_seed_entry_facts,
    _seed_fast_branch_compare_contract,
    _set_entry_metadata,
    _set_reachable_metadata,
    _run_optional_function_prepasses,
)

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
    _load_map_lookup_fusion_metadata(builder, func_data)
    _load_map_repr_metadata(builder, func_data)
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
