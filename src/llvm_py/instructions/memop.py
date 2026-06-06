from typing import Any, Dict, List, Optional

import llvmlite.ir as ir

from instructions.llvm_decl import declare_function
from utils.values import resolve_i64_strict, safe_vmap_write

_SAFE_MEMOP_EXC = (AttributeError, KeyError, RuntimeError, TypeError, ValueError)
_TABLE_INDEX_REQUIRED_PROOF_FLAGS = (
    "table_length_resolved",
    "bounds_proof_valid",
    "stride_resolved",
    "field_offset_resolved",
    "overflow_proof_valid",
    "alignment_valid",
    "element_layout_verified",
)
_FIELD_LOAD_ALLOWED_CLASSES = frozenset(("plain_scalar", "plain_pointer"))
_FIELD_LOAD_I64_TYPES = frozenset(("usize", "u64", "i64", "pointer"))
_FIELD_STORE_ALLOWED_CLASSES = frozenset(("plain_scalar", "plain_pointer"))
_FIELD_STORE_I64_TYPES = _FIELD_LOAD_I64_TYPES
_LOCAL_FREE_HEAD_ALLOWED_CLASS = "local_free_head"
_LOCAL_FREE_BLOCK_NEXT_ALLOWED_CLASS = "local_free_block_next"
_FREE_HEAD_ALLOWED_CLASS = "plain_pointer"
_CURRENT_ALLOC_OWNER_HELPER = "hako_fastmem_current_alloc_owner_id"


def _is_fastmem_layout_ref(resolver, value_id: int) -> bool:
    refs = getattr(resolver, "fastmem_layout_refs", None)
    return isinstance(refs, dict) and int(value_id) in refs


def _resolve_i64_operand(
    builder: ir.IRBuilder,
    resolver,
    value_id: int,
    vmap: Dict[int, ir.Value],
    current_block,
    preds,
    block_end_values,
    bb_map,
    *,
    name_hint: str,
):
    if _is_fastmem_layout_ref(resolver, int(value_id)):
        raise RuntimeError(
            f"[llvm/fastmem:layout-ref-as-ordinary-value] v{int(value_id)}"
        )
    i64 = ir.IntType(64)
    value = vmap.get(value_id)
    if value is None:
        value = resolve_i64_strict(
            resolver,
            value_id,
            current_block,
            preds,
            block_end_values,
            vmap,
            bb_map,
            hot_scope="fastmem_memop",
        )
    try:
        value_type = value.type
    except _SAFE_MEMOP_EXC:
        value_type = None
    if isinstance(value_type, ir.PointerType):
        value = builder.ptrtoint(value, i64, name=f"{name_hint}_p2i")
    elif isinstance(value_type, ir.IntType):
        if value_type.width < 64:
            value = builder.zext(value, i64, name=f"{name_hint}_zext")
        elif value_type.width > 64:
            value = builder.trunc(value, i64, name=f"{name_hint}_trunc")
    return value if value is not None else ir.Constant(i64, 0)


def _require_operands(kind: str, operands: List[Any], expected: int) -> None:
    if len(operands) != expected:
        raise RuntimeError(
            f"[llvm/fastmem:arity] kind={kind} expected={expected} actual={len(operands)}"
        )


def _current_fastmem_access_plan(
    resolver, kind: str, dst, operands: List[Any]
) -> Optional[Dict[str, Any]]:
    if resolver is None:
        return None
    try:
        block_id = int(getattr(resolver, "current_block_id"))
        instruction_index = int(getattr(resolver, "current_instruction_index"))
    except (TypeError, ValueError):
        return None
    plans_by_site = getattr(resolver, "fastmem_access_plans_by_site", None)
    if not isinstance(plans_by_site, dict):
        return None
    plans = plans_by_site.get((block_id, instruction_index), [])
    for plan in plans:
        if not isinstance(plan, dict):
            continue
        if plan.get("kind") != kind:
            continue
        if plan.get("verified") is not True or plan.get("status") != "verified":
            continue
        result = plan.get("result")
        if dst is not None and result is not None and int(result) != int(dst):
            continue
        if kind == "table_index":
            if len(operands) < 2:
                continue
            if plan.get("table") != int(operands[0]) or plan.get("index") != int(operands[1]):
                continue
        if kind == "field_load":
            if len(operands) < 1:
                continue
            if plan.get("base") != int(operands[0]):
                continue
        if kind == "field_store":
            if len(operands) < 2:
                continue
            if plan.get("base") != int(operands[0]) or plan.get("value") != int(operands[1]):
                continue
        if kind == "local_free_push":
            if len(operands) < 2:
                continue
            if plan.get("page") != int(operands[0]) or plan.get("block_value") != int(operands[1]):
                continue
        if kind == "local_free_pop":
            if len(operands) < 1:
                continue
            if plan.get("page") != int(operands[0]):
                continue
        if kind == "free_head_pop":
            if len(operands) < 1:
                continue
            if plan.get("page") != int(operands[0]):
                continue
        return plan
    return None


def _require_complete_table_index_plan(plan: Optional[Dict[str, Any]]) -> Dict[str, Any]:
    if not isinstance(plan, dict):
        raise RuntimeError("[llvm/fastmem:missing-verified-table-plan]")
    missing = [flag for flag in _TABLE_INDEX_REQUIRED_PROOF_FLAGS if plan.get(flag) is not True]
    if missing:
        raise RuntimeError(
            f"[llvm/fastmem:incomplete-table-plan] missing={','.join(missing)}"
        )
    if plan.get("element_repr") != "pointer_to_element":
        raise RuntimeError(
            f"[llvm/fastmem:unsupported-table-element-repr] {plan.get('element_repr')}"
        )
    for key in ("element_stride", "element_layout_id", "table_id", "region"):
        if plan.get(key) is None:
            raise RuntimeError(f"[llvm/fastmem:missing-table-plan-field] {key}")
    return plan


def _require_complete_field_load_plan(plan: Optional[Dict[str, Any]]) -> Dict[str, Any]:
    if not isinstance(plan, dict):
        raise RuntimeError("[llvm/fastmem:missing-verified-field-load-plan]")
    for key in (
        "layout_id",
        "field_id",
        "byte_offset",
        "field_size",
        "field_type",
        "alignment",
        "field_class",
        "region",
    ):
        if plan.get(key) is None:
            raise RuntimeError(f"[llvm/fastmem:missing-field-load-plan-field] {key}")
    if plan.get("access") not in (None, "load", "read"):
        raise RuntimeError(
            f"[llvm/fastmem:unsupported-field-load-access] {plan.get('access')}"
        )
    field_class = str(plan.get("field_class"))
    if field_class not in _FIELD_LOAD_ALLOWED_CLASSES:
        raise RuntimeError(
            f"[llvm/fastmem:unsupported-field-load-class] {field_class}"
        )
    field_type = str(plan.get("field_type"))
    if field_type not in _FIELD_LOAD_I64_TYPES:
        raise RuntimeError(f"[llvm/fastmem:unsupported-field-load-type] {field_type}")
    try:
        if int(plan.get("field_size")) != 8:
            raise RuntimeError(
                f"[llvm/fastmem:unsupported-field-load-size] {plan.get('field_size')}"
            )
        if int(plan.get("alignment")) <= 0:
            raise RuntimeError(
                f"[llvm/fastmem:invalid-field-load-alignment] {plan.get('alignment')}"
            )
    except ValueError as exc:
        raise RuntimeError("[llvm/fastmem:invalid-field-load-plan-number]") from exc
    return plan


def _require_complete_field_store_plan(plan: Optional[Dict[str, Any]]) -> Dict[str, Any]:
    if not isinstance(plan, dict):
        raise RuntimeError("[llvm/fastmem:missing-verified-field-store-plan]")
    for key in (
        "layout_id",
        "field_id",
        "byte_offset",
        "field_size",
        "field_type",
        "alignment",
        "mutability",
        "field_class",
        "region",
    ):
        if plan.get(key) is None:
            raise RuntimeError(f"[llvm/fastmem:missing-field-store-plan-field] {key}")
    if plan.get("access") not in (None, "store", "write"):
        raise RuntimeError(
            f"[llvm/fastmem:unsupported-field-store-access] {plan.get('access')}"
        )
    mutability = str(plan.get("mutability"))
    if mutability != "mutable":
        raise RuntimeError(
            f"[llvm/fastmem:unsupported-field-store-mutability] {mutability}"
        )
    field_class = str(plan.get("field_class"))
    if field_class not in _FIELD_STORE_ALLOWED_CLASSES:
        raise RuntimeError(
            f"[llvm/fastmem:unsupported-field-store-class] {field_class}"
        )
    field_type = str(plan.get("field_type"))
    if field_type not in _FIELD_STORE_I64_TYPES:
        raise RuntimeError(f"[llvm/fastmem:unsupported-field-store-type] {field_type}")
    try:
        if int(plan.get("field_size")) != 8:
            raise RuntimeError(
                f"[llvm/fastmem:unsupported-field-store-size] {plan.get('field_size')}"
            )
        if int(plan.get("alignment")) <= 0:
            raise RuntimeError(
                f"[llvm/fastmem:invalid-field-store-alignment] {plan.get('alignment')}"
            )
    except ValueError as exc:
        raise RuntimeError("[llvm/fastmem:invalid-field-store-plan-number]") from exc
    return plan


def _require_complete_local_free_push_plan(plan: Optional[Dict[str, Any]]) -> Dict[str, Any]:
    if not isinstance(plan, dict):
        raise RuntimeError("[llvm/fastmem:missing-verified-local-free-push-plan]")
    for key in (
        "local_free_head_layout_id",
        "local_free_head_field_id",
        "local_free_head_field_class",
        "local_free_head_byte_offset",
        "local_free_head_field_size",
        "local_free_head_field_type",
        "local_free_head_alignment",
        "block_next_layout_id",
        "block_next_field_id",
        "block_next_field_class",
        "block_next_byte_offset",
        "block_next_field_size",
        "block_next_field_type",
        "block_next_alignment",
        "region",
    ):
        if plan.get(key) is None:
            raise RuntimeError(f"[llvm/fastmem:missing-local-free-push-plan-field] {key}")
    if plan.get("lowerable") is not True:
        raise RuntimeError("[llvm/fastmem:local-free-push-plan-not-lowerable]")
    if plan.get("same_owner_proof_valid") is not True:
        raise RuntimeError("[llvm/fastmem:local-free-push-same-owner-proof-missing]")
    if plan.get("block_next_proof_valid") is not True:
        raise RuntimeError("[llvm/fastmem:local-free-push-block-next-proof-missing]")
    if plan.get("remote_owner_rejected") is not True:
        raise RuntimeError("[llvm/fastmem:local-free-push-remote-owner-not-rejected]")
    if str(plan.get("local_free_head_field_class")) != _LOCAL_FREE_HEAD_ALLOWED_CLASS:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-local-free-head-class] "
            f"{plan.get('local_free_head_field_class')}"
        )
    if str(plan.get("block_next_field_class")) != _LOCAL_FREE_BLOCK_NEXT_ALLOWED_CLASS:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-local-free-block-next-class] "
            f"{plan.get('block_next_field_class')}"
        )
    for key in (
        "local_free_head_field_size",
        "local_free_head_alignment",
        "block_next_field_size",
        "block_next_alignment",
    ):
        try:
            if int(plan.get(key)) <= 0:
                raise RuntimeError(
                    f"[llvm/fastmem:invalid-local-free-push-plan-number] {key}"
                )
        except ValueError as exc:
            raise RuntimeError("[llvm/fastmem:invalid-local-free-push-plan-number]") from exc
    if int(plan.get("local_free_head_field_size")) != 8:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-local-free-head-size] "
            f"{plan.get('local_free_head_field_size')}"
        )
    if int(plan.get("block_next_field_size")) != 8:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-local-free-block-next-size] "
            f"{plan.get('block_next_field_size')}"
        )
    if str(plan.get("local_free_head_field_type")) not in _FIELD_STORE_I64_TYPES:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-local-free-head-type] "
            f"{plan.get('local_free_head_field_type')}"
        )
    if str(plan.get("block_next_field_type")) not in _FIELD_STORE_I64_TYPES:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-local-free-block-next-type] "
            f"{plan.get('block_next_field_type')}"
        )
    return plan


def _require_complete_local_free_pop_plan(plan: Optional[Dict[str, Any]]) -> Dict[str, Any]:
    if not isinstance(plan, dict):
        raise RuntimeError("[llvm/fastmem:missing-verified-local-free-pop-plan]")
    for key in (
        "local_free_head_layout_id",
        "local_free_head_field_id",
        "local_free_head_field_class",
        "local_free_head_byte_offset",
        "local_free_head_field_size",
        "local_free_head_field_type",
        "local_free_head_alignment",
        "block_next_layout_id",
        "block_next_field_id",
        "block_next_field_class",
        "block_next_byte_offset",
        "block_next_field_size",
        "block_next_field_type",
        "block_next_alignment",
        "region",
    ):
        if plan.get(key) is None:
            raise RuntimeError(f"[llvm/fastmem:missing-local-free-pop-plan-field] {key}")
    if plan.get("lowerable") is not True:
        raise RuntimeError("[llvm/fastmem:local-free-pop-plan-not-lowerable]")
    if plan.get("same_owner_proof_valid") is not True:
        raise RuntimeError("[llvm/fastmem:local-free-pop-same-owner-proof-missing]")
    if plan.get("non_empty_proof_valid") is not True:
        raise RuntimeError("[llvm/fastmem:local-free-pop-non-empty-proof-missing]")
    if plan.get("remote_owner_rejected") is not True:
        raise RuntimeError("[llvm/fastmem:local-free-pop-remote-owner-not-rejected]")
    if str(plan.get("local_free_head_field_class")) != _LOCAL_FREE_HEAD_ALLOWED_CLASS:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-local-free-head-class] "
            f"{plan.get('local_free_head_field_class')}"
        )
    if str(plan.get("block_next_field_class")) != _LOCAL_FREE_BLOCK_NEXT_ALLOWED_CLASS:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-local-free-block-next-class] "
            f"{plan.get('block_next_field_class')}"
        )
    for key in (
        "local_free_head_field_size",
        "local_free_head_alignment",
        "block_next_field_size",
        "block_next_alignment",
    ):
        try:
            if int(plan.get(key)) <= 0:
                raise RuntimeError(
                    f"[llvm/fastmem:invalid-local-free-pop-plan-number] {key}"
                )
        except ValueError as exc:
            raise RuntimeError("[llvm/fastmem:invalid-local-free-pop-plan-number]") from exc
    if int(plan.get("local_free_head_field_size")) != 8:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-local-free-head-size] "
            f"{plan.get('local_free_head_field_size')}"
        )
    if int(plan.get("block_next_field_size")) != 8:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-local-free-block-next-size] "
            f"{plan.get('block_next_field_size')}"
        )
    if str(plan.get("local_free_head_field_type")) not in _FIELD_LOAD_I64_TYPES:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-local-free-head-type] "
            f"{plan.get('local_free_head_field_type')}"
        )
    if str(plan.get("block_next_field_type")) not in _FIELD_LOAD_I64_TYPES:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-local-free-block-next-type] "
            f"{plan.get('block_next_field_type')}"
        )
    return plan


def _require_complete_free_head_pop_plan(plan: Optional[Dict[str, Any]]) -> Dict[str, Any]:
    if not isinstance(plan, dict):
        raise RuntimeError("[llvm/fastmem:missing-verified-free-head-pop-plan]")
    for key in (
        "free_head_layout_id",
        "free_head_field_id",
        "free_head_field_class",
        "free_head_byte_offset",
        "free_head_field_size",
        "free_head_field_type",
        "free_head_alignment",
        "block_next_layout_id",
        "block_next_field_id",
        "block_next_field_class",
        "block_next_byte_offset",
        "block_next_field_size",
        "block_next_field_type",
        "block_next_alignment",
        "region",
    ):
        if plan.get(key) is None:
            raise RuntimeError(f"[llvm/fastmem:missing-free-head-pop-plan-field] {key}")
    if plan.get("lowerable") is not True:
        raise RuntimeError("[llvm/fastmem:free-head-pop-plan-not-lowerable]")
    if plan.get("same_owner_proof_valid") is not True:
        raise RuntimeError("[llvm/fastmem:free-head-pop-same-owner-proof-missing]")
    if plan.get("non_empty_proof_valid") is not True:
        raise RuntimeError("[llvm/fastmem:free-head-pop-non-empty-proof-missing]")
    if plan.get("remote_owner_rejected") is not True:
        raise RuntimeError("[llvm/fastmem:free-head-pop-remote-owner-not-rejected]")
    if str(plan.get("free_head_field_class")) != _FREE_HEAD_ALLOWED_CLASS:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-free-head-class] "
            f"{plan.get('free_head_field_class')}"
        )
    if str(plan.get("block_next_field_class")) != _LOCAL_FREE_BLOCK_NEXT_ALLOWED_CLASS:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-free-head-block-next-class] "
            f"{plan.get('block_next_field_class')}"
        )
    for key in (
        "free_head_field_size",
        "free_head_alignment",
        "block_next_field_size",
        "block_next_alignment",
    ):
        try:
            if int(plan.get(key)) <= 0:
                raise RuntimeError(
                    f"[llvm/fastmem:invalid-free-head-pop-plan-number] {key}"
                )
        except ValueError as exc:
            raise RuntimeError("[llvm/fastmem:invalid-free-head-pop-plan-number]") from exc
    if int(plan.get("free_head_field_size")) != 8:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-free-head-size] "
            f"{plan.get('free_head_field_size')}"
        )
    if int(plan.get("block_next_field_size")) != 8:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-free-head-block-next-size] "
            f"{plan.get('block_next_field_size')}"
        )
    if str(plan.get("free_head_field_type")) not in _FIELD_LOAD_I64_TYPES:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-free-head-type] "
            f"{plan.get('free_head_field_type')}"
        )
    if str(plan.get("block_next_field_type")) not in _FIELD_LOAD_I64_TYPES:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-free-head-block-next-type] "
            f"{plan.get('block_next_field_type')}"
        )
    return plan


def _layout_refs(resolver) -> Dict[int, Dict[str, Any]]:
    refs = getattr(resolver, "fastmem_layout_refs", None)
    if not isinstance(refs, dict):
        refs = {}
        setattr(resolver, "fastmem_layout_refs", refs)
    return refs


def _get_layout_ref(resolver, value_id: int, expected_layout_id: str) -> Dict[str, Any]:
    refs = _layout_refs(resolver)
    ref = refs.get(int(value_id))
    if not isinstance(ref, dict):
        raise RuntimeError(f"[llvm/fastmem:expected-layout-ref] v{int(value_id)}")
    actual_layout_id = ref.get("layout_id")
    if actual_layout_id != expected_layout_id:
        raise RuntimeError(
            "[llvm/fastmem:layout-ref-mismatch] "
            f"v{int(value_id)} expected={expected_layout_id} actual={actual_layout_id}"
        )
    ptr = ref.get("ptr")
    if ptr is None:
        raise RuntimeError(f"[llvm/fastmem:layout-ref-missing-ptr] v{int(value_id)}")
    return ref


def _lower_current_alloc_owner_id(builder: ir.IRBuilder, dst: int) -> ir.Value:
    i64 = ir.IntType(64)
    helper = declare_function(builder.module, _CURRENT_ALLOC_OWNER_HELPER, i64, [])
    return builder.call(helper, [], name=f"fastmem_current_alloc_owner_id_{dst}")


def _lower_table_index_to_layout_ref(
    builder: ir.IRBuilder,
    resolver,
    dst: int,
    operands: List[Any],
    vmap: Dict[int, ir.Value],
    current_block,
    preds,
    block_end_values,
    bb_map,
) -> None:
    _require_operands("table_index", operands, 2)
    plan = _require_complete_table_index_plan(
        _current_fastmem_access_plan(resolver, "table_index", dst, operands)
    )
    i64 = ir.IntType(64)
    table_addr = _resolve_i64_operand(
        builder,
        resolver,
        int(operands[0]),
        vmap,
        current_block,
        preds,
        block_end_values,
        bb_map,
        name_hint=f"fastmem_table_base_{dst}",
    )
    index = _resolve_i64_operand(
        builder,
        resolver,
        int(operands[1]),
        vmap,
        current_block,
        preds,
        block_end_values,
        bb_map,
        name_hint=f"fastmem_table_index_{dst}",
    )
    stride = int(plan["element_stride"])
    byte_index = builder.mul(index, ir.Constant(i64, stride), name=f"fastmem_table_index_bytes_{dst}")
    slot_addr = builder.add(table_addr, byte_index, name=f"fastmem_table_slot_addr_{dst}")
    slot_ptr = builder.inttoptr(slot_addr, i64.as_pointer(), name=f"fastmem_table_slot_ptr_{dst}")
    element_addr = builder.load(slot_ptr, name=f"fastmem_layout_ref_addr_{dst}")
    element_ptr = builder.inttoptr(element_addr, ir.IntType(8).as_pointer(), name=f"fastmem_layout_ref_ptr_{dst}")
    _layout_refs(resolver)[int(dst)] = {
        "ptr": element_ptr,
        "layout_id": plan.get("element_layout_id"),
        "table_id": plan.get("table_id"),
        "region": plan.get("region"),
        "source_site": (plan.get("block"), plan.get("instruction_index")),
    }


def _lower_field_load_from_layout_ref(
    builder: ir.IRBuilder,
    resolver,
    dst: int,
    operands: List[Any],
    vmap: Dict[int, ir.Value],
) -> None:
    _require_operands("field_load", operands, 1)
    plan = _require_complete_field_load_plan(
        _current_fastmem_access_plan(resolver, "field_load", dst, operands)
    )
    layout_ref = _get_layout_ref(resolver, int(operands[0]), str(plan["layout_id"]))
    i64 = ir.IntType(64)
    i8_ptr = ir.IntType(8).as_pointer()
    base_ptr = layout_ref["ptr"]
    try:
        base_type = base_ptr.type
    except _SAFE_MEMOP_EXC:
        base_type = None
    if base_type != i8_ptr:
        base_ptr = builder.bitcast(base_ptr, i8_ptr, name=f"fastmem_field_base_{dst}")
    byte_offset = int(plan["byte_offset"])
    field_addr = builder.gep(
        base_ptr,
        [ir.Constant(i64, byte_offset)],
        name=f"fastmem_field_addr_{dst}",
    )
    field_ptr = builder.bitcast(
        field_addr,
        i64.as_pointer(),
        name=f"fastmem_field_ptr_{dst}",
    )
    loaded = builder.load(field_ptr, name=f"fastmem_field_load_{dst}")
    safe_vmap_write(vmap, int(dst), loaded, "fastmem_field_load", resolver=resolver)


def _lower_field_store_from_layout_ref(
    builder: ir.IRBuilder,
    resolver,
    operands: List[Any],
    vmap: Dict[int, ir.Value],
    current_block,
    preds,
    block_end_values,
    bb_map,
) -> None:
    _require_operands("field_store", operands, 2)
    plan = _require_complete_field_store_plan(
        _current_fastmem_access_plan(resolver, "field_store", None, operands)
    )
    layout_ref = _get_layout_ref(resolver, int(operands[0]), str(plan["layout_id"]))
    i64 = ir.IntType(64)
    i8_ptr = ir.IntType(8).as_pointer()
    value = _resolve_i64_operand(
        builder,
        resolver,
        int(operands[1]),
        vmap,
        current_block,
        preds,
        block_end_values,
        bb_map,
        name_hint=f"fastmem_field_store_value_{operands[1]}",
    )
    base_ptr = layout_ref["ptr"]
    try:
        base_type = base_ptr.type
    except _SAFE_MEMOP_EXC:
        base_type = None
    if base_type != i8_ptr:
        base_ptr = builder.bitcast(base_ptr, i8_ptr, name="fastmem_field_store_base")
    byte_offset = int(plan["byte_offset"])
    field_addr = builder.gep(
        base_ptr,
        [ir.Constant(i64, byte_offset)],
        name="fastmem_field_store_addr",
    )
    field_ptr = builder.bitcast(
        field_addr,
        i64.as_pointer(),
        name="fastmem_field_store_ptr",
    )
    builder.store(value, field_ptr)


def _gep_i64_field_ptr(
    builder: ir.IRBuilder,
    base_ptr,
    byte_offset: int,
    *,
    name_prefix: str,
):
    i64 = ir.IntType(64)
    i8_ptr = ir.IntType(8).as_pointer()
    try:
        base_type = base_ptr.type
    except _SAFE_MEMOP_EXC:
        base_type = None
    if base_type != i8_ptr:
        base_ptr = builder.bitcast(base_ptr, i8_ptr, name=f"{name_prefix}_base")
    field_addr = builder.gep(
        base_ptr,
        [ir.Constant(i64, byte_offset)],
        name=f"{name_prefix}_addr",
    )
    return builder.bitcast(field_addr, i64.as_pointer(), name=f"{name_prefix}_ptr")


def _lower_local_free_push(
    builder: ir.IRBuilder,
    resolver,
    operands: List[Any],
    vmap: Dict[int, ir.Value],
    current_block,
    preds,
    block_end_values,
    bb_map,
) -> None:
    _require_operands("local_free_push", operands, 2)
    plan = _require_complete_local_free_push_plan(
        _current_fastmem_access_plan(resolver, "local_free_push", None, operands)
    )
    page_ref = _get_layout_ref(
        resolver,
        int(operands[0]),
        str(plan["local_free_head_layout_id"]),
    )
    i64 = ir.IntType(64)
    block_addr = _resolve_i64_operand(
        builder,
        resolver,
        int(operands[1]),
        vmap,
        current_block,
        preds,
        block_end_values,
        bb_map,
        name_hint=f"fastmem_local_free_block_{operands[1]}",
    )
    block_ptr = builder.inttoptr(
        block_addr,
        ir.IntType(8).as_pointer(),
        name=f"fastmem_local_free_block_ptr_{operands[1]}",
    )
    head_ptr = _gep_i64_field_ptr(
        builder,
        page_ref["ptr"],
        int(plan["local_free_head_byte_offset"]),
        name_prefix="fastmem_local_free_head",
    )
    old_head = builder.load(head_ptr, name="fastmem_local_free_old_head")
    block_next_ptr = _gep_i64_field_ptr(
        builder,
        block_ptr,
        int(plan["block_next_byte_offset"]),
        name_prefix="fastmem_local_free_block_next",
    )
    builder.store(old_head, block_next_ptr)
    builder.store(block_addr, head_ptr)


def _lower_local_free_pop(
    builder: ir.IRBuilder,
    resolver,
    dst: int,
    operands: List[Any],
) -> ir.Value:
    _require_operands("local_free_pop", operands, 1)
    plan = _require_complete_local_free_pop_plan(
        _current_fastmem_access_plan(resolver, "local_free_pop", dst, operands)
    )
    page_ref = _get_layout_ref(
        resolver,
        int(operands[0]),
        str(plan["local_free_head_layout_id"]),
    )
    i8_ptr = ir.IntType(8).as_pointer()
    head_ptr = _gep_i64_field_ptr(
        builder,
        page_ref["ptr"],
        int(plan["local_free_head_byte_offset"]),
        name_prefix="fastmem_local_free_head",
    )
    old_head = builder.load(head_ptr, name="fastmem_local_free_pop_old_head")
    block_ptr = builder.inttoptr(
        old_head,
        i8_ptr,
        name=f"fastmem_local_free_pop_block_ptr_{dst}",
    )
    block_next_ptr = _gep_i64_field_ptr(
        builder,
        block_ptr,
        int(plan["block_next_byte_offset"]),
        name_prefix="fastmem_local_free_pop_block_next",
    )
    next_head = builder.load(block_next_ptr, name="fastmem_local_free_pop_next_head")
    builder.store(next_head, head_ptr)
    return old_head


def _lower_free_head_pop(
    builder: ir.IRBuilder,
    resolver,
    dst: int,
    operands: List[Any],
) -> ir.Value:
    _require_operands("free_head_pop", operands, 1)
    plan = _require_complete_free_head_pop_plan(
        _current_fastmem_access_plan(resolver, "free_head_pop", dst, operands)
    )
    page_ref = _get_layout_ref(
        resolver,
        int(operands[0]),
        str(plan["free_head_layout_id"]),
    )
    i8_ptr = ir.IntType(8).as_pointer()
    head_ptr = _gep_i64_field_ptr(
        builder,
        page_ref["ptr"],
        int(plan["free_head_byte_offset"]),
        name_prefix="fastmem_free_head",
    )
    old_head = builder.load(head_ptr, name="fastmem_free_head_pop_old_head")
    block_ptr = builder.inttoptr(
        old_head,
        i8_ptr,
        name=f"fastmem_free_head_pop_block_ptr_{dst}",
    )
    block_next_ptr = _gep_i64_field_ptr(
        builder,
        block_ptr,
        int(plan["block_next_byte_offset"]),
        name_prefix="fastmem_free_head_pop_block_next",
    )
    next_head = builder.load(block_next_ptr, name="fastmem_free_head_pop_next_head")
    builder.store(next_head, head_ptr)
    return old_head


def lower_memop(
    builder: ir.IRBuilder,
    inst: Dict[str, Any],
    vmap: Dict[int, ir.Value],
    resolver,
    current_block,
    preds,
    block_end_values,
    bb_map,
) -> None:
    kind = str(inst.get("kind") or "")
    dst: Optional[int] = inst.get("dst")
    operands = list(inst.get("operands") or [])
    i64 = ir.IntType(64)

    result = None
    if kind == "table_index":
        if dst is None:
            raise RuntimeError("[llvm/fastmem:table-index-missing-dst]")
        _lower_table_index_to_layout_ref(
            builder,
            resolver,
            int(dst),
            operands,
            vmap,
            current_block,
            preds,
            block_end_values,
            bb_map,
        )
        return
    if kind == "field_load":
        if dst is None:
            raise RuntimeError("[llvm/fastmem:field-load-missing-dst]")
        _lower_field_load_from_layout_ref(builder, resolver, int(dst), operands, vmap)
        return
    if kind == "field_store":
        if dst is not None:
            raise RuntimeError("[llvm/fastmem:field-store-has-dst]")
        _lower_field_store_from_layout_ref(
            builder,
            resolver,
            operands,
            vmap,
            current_block,
            preds,
            block_end_values,
            bb_map,
        )
        return
    if kind == "local_free_push":
        if dst is not None:
            raise RuntimeError("[llvm/fastmem:local-free-push-has-dst]")
        _lower_local_free_push(
            builder,
            resolver,
            operands,
            vmap,
            current_block,
            preds,
            block_end_values,
            bb_map,
        )
        return
    if kind == "local_free_pop":
        if dst is None:
            raise RuntimeError("[llvm/fastmem:local-free-pop-missing-dst]")
        result = _lower_local_free_pop(builder, resolver, int(dst), operands)
        safe_vmap_write(vmap, int(dst), result, "fastmem_local_free_pop", resolver=resolver)
        return
    if kind == "free_head_pop":
        if dst is None:
            raise RuntimeError("[llvm/fastmem:free-head-pop-missing-dst]")
        result = _lower_free_head_pop(builder, resolver, int(dst), operands)
        safe_vmap_write(vmap, int(dst), result, "fastmem_free_head_pop", resolver=resolver)
        return
    if kind == "current_alloc_owner_id":
        _require_operands(kind, operands, 0)
        if dst is None:
            raise RuntimeError("[llvm/fastmem:current-alloc-owner-id-missing-dst]")
        result = _lower_current_alloc_owner_id(builder, int(dst))
    elif kind == "addr_of":
        _require_operands(kind, operands, 1)
        result = _resolve_i64_operand(
            builder,
            resolver,
            int(operands[0]),
            vmap,
            current_block,
            preds,
            block_end_values,
            bb_map,
            name_hint=f"fastmem_addr_{dst}",
        )
    elif kind in ("logical_shr", "bit_and", "add", "sub", "owner_eq"):
        _require_operands(kind, operands, 2)
        lhs = _resolve_i64_operand(
            builder,
            resolver,
            int(operands[0]),
            vmap,
            current_block,
            preds,
            block_end_values,
            bb_map,
            name_hint=f"fastmem_{kind}_lhs_{dst}",
        )
        rhs = _resolve_i64_operand(
            builder,
            resolver,
            int(operands[1]),
            vmap,
            current_block,
            preds,
            block_end_values,
            bb_map,
            name_hint=f"fastmem_{kind}_rhs_{dst}",
        )
        if kind == "logical_shr":
            result = builder.lshr(lhs, rhs, name=f"fastmem_lshr_{dst}")
        elif kind == "bit_and":
            result = builder.and_(lhs, rhs, name=f"fastmem_and_{dst}")
        elif kind == "add":
            result = builder.add(lhs, rhs, name=f"fastmem_add_{dst}")
        elif kind == "sub":
            result = builder.sub(lhs, rhs, name=f"fastmem_sub_{dst}")
        else:
            result = builder.icmp_unsigned("==", lhs, rhs, name=f"fastmem_owner_eq_{dst}")
    else:
        raise RuntimeError(f"[llvm/fastmem:unsupported-kind] {kind}")

    if dst is None:
        return
    if result is None:
        result = ir.Constant(i64, 0)
    safe_vmap_write(vmap, int(dst), result, f"fastmem_{kind}", resolver=resolver)
