from typing import Any, Dict, List, Optional


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
_ATOMIC_REMOTE_HEAD_ALLOWED_CLASS = "atomic_remote_head"


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
        if kind == "free_head_push":
            if len(operands) < 2:
                continue
            if plan.get("page") != int(operands[0]) or plan.get("block_value") != int(operands[1]):
                continue
        if kind == "atomic_remote_head_push":
            if len(operands) < 2:
                continue
            if plan.get("page") != int(operands[0]) or plan.get("block_value") != int(operands[1]):
                continue
        if kind == "atomic_remote_head_drain":
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


def _require_complete_free_head_push_plan(plan: Optional[Dict[str, Any]]) -> Dict[str, Any]:
    if not isinstance(plan, dict):
        raise RuntimeError("[llvm/fastmem:missing-verified-free-head-push-plan]")
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
            raise RuntimeError(f"[llvm/fastmem:missing-free-head-push-plan-field] {key}")
    if plan.get("lowerable") is not True:
        raise RuntimeError("[llvm/fastmem:free-head-push-plan-not-lowerable]")
    if plan.get("same_owner_proof_valid") is not True:
        raise RuntimeError("[llvm/fastmem:free-head-push-same-owner-proof-missing]")
    if plan.get("block_next_proof_valid") is not True:
        raise RuntimeError("[llvm/fastmem:free-head-push-block-next-proof-missing]")
    if plan.get("remote_owner_rejected") is not True:
        raise RuntimeError("[llvm/fastmem:free-head-push-remote-owner-not-rejected]")
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
                    f"[llvm/fastmem:invalid-free-head-push-plan-number] {key}"
                )
        except ValueError as exc:
            raise RuntimeError("[llvm/fastmem:invalid-free-head-push-plan-number]") from exc
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
    if str(plan.get("free_head_field_type")) not in _FIELD_STORE_I64_TYPES:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-free-head-type] "
            f"{plan.get('free_head_field_type')}"
        )
    if str(plan.get("block_next_field_type")) not in _FIELD_STORE_I64_TYPES:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-free-head-block-next-type] "
            f"{plan.get('block_next_field_type')}"
        )
    return plan


def _require_complete_atomic_remote_head_push_plan(
    plan: Optional[Dict[str, Any]]
) -> Dict[str, Any]:
    if not isinstance(plan, dict):
        raise RuntimeError("[llvm/fastmem:missing-verified-atomic-remote-head-push-plan]")
    for key in (
        "remote_head_layout_id",
        "remote_head_field_id",
        "remote_head_field_class",
        "remote_head_byte_offset",
        "remote_head_field_size",
        "remote_head_field_type",
        "remote_head_alignment",
        "block_next_layout_id",
        "block_next_field_id",
        "block_next_field_class",
        "block_next_byte_offset",
        "block_next_field_size",
        "block_next_field_type",
        "block_next_alignment",
        "memory_order_policy",
        "retry_attempt_limit",
        "region",
    ):
        if plan.get(key) is None:
            raise RuntimeError(
                f"[llvm/fastmem:missing-atomic-remote-head-push-plan-field] {key}"
            )
    if plan.get("lowerable") is not True:
        raise RuntimeError("[llvm/fastmem:atomic-remote-head-push-plan-not-lowerable]")
    if plan.get("remote_owner_proof_valid") is not True:
        raise RuntimeError("[llvm/fastmem:atomic-remote-head-remote-owner-proof-missing]")
    if plan.get("block_next_proof_valid") is not True:
        raise RuntimeError("[llvm/fastmem:atomic-remote-head-block-next-proof-missing]")
    if str(plan.get("remote_head_field_class")) != _ATOMIC_REMOTE_HEAD_ALLOWED_CLASS:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-atomic-remote-head-class] "
            f"{plan.get('remote_head_field_class')}"
        )
    if str(plan.get("block_next_field_class")) != _LOCAL_FREE_BLOCK_NEXT_ALLOWED_CLASS:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-atomic-remote-block-next-class] "
            f"{plan.get('block_next_field_class')}"
        )
    if str(plan.get("memory_order_policy")) != "acq_rel":
        raise RuntimeError(
            "[llvm/fastmem:unsupported-atomic-remote-head-memory-order] "
            f"{plan.get('memory_order_policy')}"
        )
    for key in (
        "remote_head_field_size",
        "remote_head_alignment",
        "block_next_field_size",
        "block_next_alignment",
    ):
        try:
            if int(plan.get(key)) <= 0:
                raise RuntimeError(
                    f"[llvm/fastmem:invalid-atomic-remote-head-plan-number] {key}"
                )
        except ValueError as exc:
            raise RuntimeError(
                "[llvm/fastmem:invalid-atomic-remote-head-plan-number]"
            ) from exc
    try:
        if int(plan.get("retry_attempt_limit")) <= 0:
            raise RuntimeError(
                "[llvm/fastmem:invalid-atomic-remote-head-retry-attempt-limit]"
            )
    except ValueError as exc:
        raise RuntimeError(
            "[llvm/fastmem:invalid-atomic-remote-head-retry-attempt-limit]"
        ) from exc
    if int(plan.get("remote_head_field_size")) != 8:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-atomic-remote-head-size] "
            f"{plan.get('remote_head_field_size')}"
        )
    if int(plan.get("block_next_field_size")) != 8:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-atomic-remote-block-next-size] "
            f"{plan.get('block_next_field_size')}"
        )
    if str(plan.get("remote_head_field_type")) not in _FIELD_STORE_I64_TYPES:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-atomic-remote-head-type] "
            f"{plan.get('remote_head_field_type')}"
        )
    if str(plan.get("block_next_field_type")) not in _FIELD_STORE_I64_TYPES:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-atomic-remote-block-next-type] "
            f"{plan.get('block_next_field_type')}"
        )
    return plan


def _require_complete_atomic_remote_head_drain_plan(
    plan: Optional[Dict[str, Any]]
) -> Dict[str, Any]:
    if not isinstance(plan, dict):
        raise RuntimeError("[llvm/fastmem:missing-verified-atomic-remote-head-drain-plan]")
    for key in (
        "remote_head_layout_id",
        "remote_head_field_id",
        "remote_head_field_class",
        "remote_head_byte_offset",
        "remote_head_field_size",
        "remote_head_field_type",
        "remote_head_alignment",
        "memory_order_policy",
        "region",
    ):
        if plan.get(key) is None:
            raise RuntimeError(
                f"[llvm/fastmem:missing-atomic-remote-head-drain-plan-field] {key}"
            )
    if plan.get("lowerable") is not True:
        raise RuntimeError("[llvm/fastmem:atomic-remote-head-drain-plan-not-lowerable]")
    if str(plan.get("remote_head_field_class")) != _ATOMIC_REMOTE_HEAD_ALLOWED_CLASS:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-atomic-remote-head-drain-class] "
            f"{plan.get('remote_head_field_class')}"
        )
    if str(plan.get("memory_order_policy")) != "acquire_exchange":
        raise RuntimeError(
            "[llvm/fastmem:unsupported-atomic-remote-head-drain-memory-order] "
            f"{plan.get('memory_order_policy')}"
        )
    for key in ("remote_head_field_size", "remote_head_alignment"):
        try:
            if int(plan.get(key)) <= 0:
                raise RuntimeError(
                    f"[llvm/fastmem:invalid-atomic-remote-head-drain-plan-number] {key}"
                )
        except ValueError as exc:
            raise RuntimeError(
                "[llvm/fastmem:invalid-atomic-remote-head-drain-plan-number]"
            ) from exc
    if int(plan.get("remote_head_field_size")) != 8:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-atomic-remote-head-drain-size] "
            f"{plan.get('remote_head_field_size')}"
        )
    if str(plan.get("remote_head_field_type")) not in _FIELD_LOAD_I64_TYPES:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-atomic-remote-head-drain-type] "
            f"{plan.get('remote_head_field_type')}"
        )
    return plan


def _require_complete_drain_remote_list_to_local_plan(
    plan: Optional[Dict[str, Any]]
) -> Dict[str, Any]:
    if not isinstance(plan, dict):
        raise RuntimeError("[llvm/fastmem:missing-verified-drain-remote-list-to-local-plan]")
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
        "publication_order",
        "region",
    ):
        if plan.get(key) is None:
            raise RuntimeError(
                f"[llvm/fastmem:missing-drain-remote-list-to-local-plan-field] {key}"
            )
    if plan.get("lowerable") is not True:
        raise RuntimeError("[llvm/fastmem:drain-remote-list-to-local-plan-not-lowerable]")
    if plan.get("token_provenance_valid") is not True:
        raise RuntimeError("[llvm/fastmem:drain-remote-list-token-provenance-missing]")
    if plan.get("page_operand_valid") is not True:
        raise RuntimeError("[llvm/fastmem:drain-remote-list-page-operand-invalid]")
    if plan.get("head_class_resolved") is not True:
        raise RuntimeError("[llvm/fastmem:drain-remote-list-head-class-unresolved]")
    if plan.get("block_next_access_resolved") is not True:
        raise RuntimeError("[llvm/fastmem:drain-remote-list-block-next-access-unresolved]")
    if str(plan.get("local_free_head_field_class")) != _LOCAL_FREE_HEAD_ALLOWED_CLASS:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-drain-remote-local-head-class] "
            f"{plan.get('local_free_head_field_class')}"
        )
    if str(plan.get("block_next_field_class")) != _LOCAL_FREE_BLOCK_NEXT_ALLOWED_CLASS:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-drain-remote-block-next-class] "
            f"{plan.get('block_next_field_class')}"
        )
    if str(plan.get("publication_order")) != "verifier_owned_acquire_then_owner_local":
        raise RuntimeError(
            "[llvm/fastmem:unsupported-drain-remote-publication-order] "
            f"{plan.get('publication_order')}"
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
                    f"[llvm/fastmem:invalid-drain-remote-list-plan-number] {key}"
                )
        except ValueError as exc:
            raise RuntimeError("[llvm/fastmem:invalid-drain-remote-list-plan-number]") from exc
    if int(plan.get("local_free_head_field_size")) != 8:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-drain-remote-local-head-size] "
            f"{plan.get('local_free_head_field_size')}"
        )
    if int(plan.get("block_next_field_size")) != 8:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-drain-remote-block-next-size] "
            f"{plan.get('block_next_field_size')}"
        )
    if str(plan.get("local_free_head_field_type")) not in _FIELD_STORE_I64_TYPES:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-drain-remote-local-head-type] "
            f"{plan.get('local_free_head_field_type')}"
        )
    if str(plan.get("block_next_field_type")) not in _FIELD_STORE_I64_TYPES:
        raise RuntimeError(
            "[llvm/fastmem:unsupported-drain-remote-block-next-type] "
            f"{plan.get('block_next_field_type')}"
        )
    return plan
