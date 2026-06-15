from typing import Any, Dict, Optional

import llvmlite.ir as ir

from instructions.field_access import (
    _canonical_bool_i64,
    _canonical_i64,
    _declare,
    _ensure_handle,
    _field_ptr,
    _mark_bool_immediate,
    _mark_float_immediate,
    _mark_integer_immediate,
    _resolve_receiver,
    _resolve_typed_float_value,
)
from instructions.newbox import lower_newbox
from instructions.primitive_handles import resolver_value_type, unbox_primitive_handle_if_needed
from instructions.sum_runtime import ENUM_PAYLOAD_FIELD, ENUM_TAG_FIELD, runtime_box_name
from instructions.typeop import _emit_trap
from type_facts import is_box_handle_fact, make_box_handle_fact
from utils.resolver_helpers import mark_as_handle
from utils.values import safe_vmap_write

_SAFE_SUM_OPS_EXC = (AttributeError, KeyError, RuntimeError, TypeError, ValueError)



def _new_runtime_sum_handle(
    builder: ir.IRBuilder,
    module: ir.Module,
    enum_name: str,
    *,
    name_hint: str,
):
    box_type = runtime_box_name(enum_name)
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()
    new_i64x = _declare(
        module,
        "nyash.env.box.new_i64x",
        i64,
        [i8p, i64, i64, i64, i64, i64],
    )

    sbytes = (box_type + "\0").encode("utf-8")
    arr_ty = ir.ArrayType(ir.IntType(8), len(sbytes))
    try:
        fn = builder.block.parent
        fn_name = getattr(fn, "name", "fn")
    except _SAFE_SUM_OPS_EXC:
        fn_name = "fn"
    base = f".sum_box_ty_{fn_name}_{name_hint}"
    existing = {g.name for g in module.global_values}
    name = base
    suffix = 1
    while name in existing:
        name = f"{base}.{suffix}"
        suffix += 1

    g = ir.GlobalVariable(module, arr_ty, name=name)
    g.linkage = "private"
    g.global_constant = True
    g.initializer = ir.Constant(arr_ty, bytearray(sbytes))
    c0 = ir.Constant(ir.IntType(32), 0)
    ptr = builder.gep(g, [c0, c0], inbounds=True)
    zero = ir.Constant(i64, 0)
    return builder.call(
        new_i64x,
        [ptr, zero, zero, zero, zero, zero],
        name=f"new_{box_type}_{name_hint}",
    )



def _payload_kind(payload_type: Optional[str]) -> str:
    if payload_type in ("Integer", "int", "i64", "IntegerBox"):
        return "Integer"
    if payload_type in ("Bool", "bool", "BoolBox"):
        return "Bool"
    if payload_type in ("Float", "f64", "FloatBox"):
        return "Float"
    if payload_type in ("String", "StringBox"):
        return "String"
    if payload_type in ("Void", "Null", "VoidBox", "NullBox"):
        return "Void"
    if payload_type:
        return payload_type
    return "Handle"


def _payload_handle_value(
    builder: ir.IRBuilder,
    module: ir.Module,
    payload_vid: int,
    payload_value,
    payload_meta: Any,
    payload_fact: Any,
    payload_fallback_kind: str,
):
    if is_box_handle_fact(payload_fact, "StringBox"):
        return _ensure_handle(builder, module, payload_value)
    if isinstance(payload_fact, dict) and payload_fact.get("kind") == "handle":
        return _ensure_handle(builder, module, payload_value)

    vtype = getattr(payload_value, "type", None)
    if isinstance(vtype, ir.DoubleType):
        boxer = _declare(module, "nyash.box.from_f64", ir.IntType(64), [ir.DoubleType()])
        return builder.call(boxer, [payload_value], name=f"sum_payload_box_f64_{payload_vid}")
    if isinstance(vtype, ir.IntType) and vtype.width == 1:
        boxer = _declare(module, "nyash.box.from_bool", ir.IntType(64), [ir.IntType(64)])
        bool_i64 = _canonical_bool_i64(builder, payload_value, name_hint=f"sum_payload_bool_{payload_vid}")
        return builder.call(boxer, [bool_i64], name=f"sum_payload_box_bool_{payload_vid}")
    if isinstance(vtype, ir.IntType):
        if payload_fact == "Void":
            raise RuntimeError("[sum_ops] generic void payload boxing is not supported on LLVM fallback")
        if payload_fallback_kind not in {"Handle", "String"}:
            return _ensure_handle(builder, module, payload_value)
        boxer = _declare(module, "nyash.box.from_i64", ir.IntType(64), [ir.IntType(64)])
        int_value = _canonical_i64(builder, payload_value, name_hint=f"sum_payload_i64_{payload_vid}")
        return builder.call(boxer, [int_value], name=f"sum_payload_box_i64_{payload_vid}")
    return _ensure_handle(builder, module, payload_value)


def _resolve_payload_value(
    builder: ir.IRBuilder,
    payload_vid: int,
    vmap: Dict[int, Any],
    resolver,
    preds,
    block_end_values,
    bb_map,
):
    value = vmap.get(int(payload_vid))
    if value is not None:
        return value
    return _resolve_receiver(
        builder,
        int(payload_vid),
        vmap,
        resolver,
        preds,
        block_end_values,
        bb_map,
    )


def _payload_fact_store(resolver):
    if resolver is None:
        return None
    facts = getattr(resolver, "sum_payload_facts", None)
    if isinstance(facts, dict):
        return facts
    facts = {}
    setattr(resolver, "sum_payload_facts", facts)
    return facts


def _record_sum_payload_fact(resolver, sum_vid: int, payload_fact: Any) -> None:
    if payload_fact is None:
        return
    facts = _payload_fact_store(resolver)
    if facts is not None:
        facts[int(sum_vid)] = payload_fact


def _sum_payload_fact(resolver, sum_vid: int) -> Any:
    facts = _payload_fact_store(resolver)
    if facts is None:
        return None
    return facts.get(int(sum_vid))


def _resolved_payload_fact(
    resolver,
    payload_vid: int,
    payload_meta: Any,
    payload_value,
    payload_type: Optional[str],
) -> Any:
    declared_fact = _declared_payload_fact(payload_type)
    if declared_fact is not None:
        return declared_fact
    actual_fact = _runtime_payload_fact(payload_meta, payload_value)
    if actual_fact is not None:
        return actual_fact
    if resolver is not None:
        integerish_ids = getattr(resolver, "integerish_ids", None)
        if isinstance(integerish_ids, set) and int(payload_vid) in integerish_ids:
            return "i64"
    return None


def _project_payload_fact(resolver, value_vid: int, payload_type: Optional[str]) -> Any:
    declared_fact = _declared_payload_fact(payload_type)
    if declared_fact is not None:
        return declared_fact
    return _sum_payload_fact(resolver, int(value_vid))


def _declared_payload_fact(payload_type: Optional[str]) -> Any:
    kind = _payload_kind(payload_type)
    if kind == "Integer":
        return "i64"
    if kind == "Bool":
        return "Bool"
    if kind == "Float":
        return "Float"
    if kind == "String":
        return make_box_handle_fact("StringBox")
    if kind == "Void":
        return "Void"
    return None


def _runtime_payload_fact(payload_meta: Any, payload_value) -> Any:
    if payload_meta in ("Bool", "i1") or (
        isinstance(payload_meta, dict) and payload_meta.get("kind") in ("Bool", "i1")
    ):
        return "Bool"
    if is_box_handle_fact(payload_meta, "BoolBox"):
        return "Bool"
    if payload_meta in ("Float", "f64") or (
        isinstance(payload_meta, dict) and payload_meta.get("kind") in ("Float", "f64")
    ):
        return "Float"
    if is_box_handle_fact(payload_meta, "FloatBox"):
        return "Float"
    if payload_meta in ("Integer", "i64") or (
        isinstance(payload_meta, dict) and payload_meta.get("kind") in ("Integer", "i64")
    ):
        return "i64"
    if is_box_handle_fact(payload_meta, "IntegerBox"):
        return "i64"
    if is_box_handle_fact(payload_meta, "StringBox"):
        return make_box_handle_fact("StringBox")
    if isinstance(payload_meta, dict) and payload_meta.get("kind") == "handle":
        return dict(payload_meta)

    vtype = getattr(payload_value, "type", None)
    if isinstance(vtype, ir.DoubleType):
        return "Float"
    if isinstance(vtype, ir.IntType) and vtype.width == 1:
        return "Bool"
    return None


def _storage_kind_from_fact(payload_fact: Any) -> Optional[str]:
    if payload_fact in ("i64", "Integer") or (
        isinstance(payload_fact, dict) and payload_fact.get("kind") in ("i64", "Integer")
    ):
        return "Integer"
    if payload_fact in ("Bool", "i1") or (
        isinstance(payload_fact, dict) and payload_fact.get("kind") in ("Bool", "i1")
    ):
        return "Bool"
    if payload_fact in ("Float", "f64") or (
        isinstance(payload_fact, dict) and payload_fact.get("kind") in ("Float", "f64")
    ):
        return "Float"
    return None


def _apply_payload_fact_to_result(resolver, dst_vid: int, payload_fact: Any) -> None:
    if payload_fact in ("i64", "Integer") or (
        isinstance(payload_fact, dict) and payload_fact.get("kind") in ("i64", "Integer")
    ):
        _mark_integer_immediate(resolver, int(dst_vid))
        return
    if payload_fact in ("Bool", "i1") or (
        isinstance(payload_fact, dict) and payload_fact.get("kind") in ("Bool", "i1")
    ):
        _mark_bool_immediate(resolver, int(dst_vid))
        return
    if payload_fact in ("Float", "f64") or (
        isinstance(payload_fact, dict) and payload_fact.get("kind") in ("Float", "f64")
    ):
        _mark_float_immediate(resolver, int(dst_vid))
        return
    if is_box_handle_fact(payload_fact, "StringBox"):
        try:
            resolver.mark_string(int(dst_vid))
        except _SAFE_SUM_OPS_EXC:
            mark_as_handle(resolver, int(dst_vid), "StringBox")
        return
    if isinstance(payload_fact, dict) and payload_fact.get("kind") == "handle":
        mark_as_handle(resolver, int(dst_vid), payload_fact.get("box_type"))
        return
    mark_as_handle(resolver, int(dst_vid))


def _set_i64_field(builder: ir.IRBuilder, module: ir.Module, recv_h, field_name: str, value):
    callee = _declare(
        module,
        "nyash.instance.set_i64_field_h",
        ir.IntType(64),
        [ir.IntType(64), ir.IntType(8).as_pointer(), ir.IntType(64)],
    )
    builder.call(callee, [recv_h, _field_ptr(builder, module, field_name), value], name=f"sum_set_i64_{field_name}")


def _set_bool_field(builder: ir.IRBuilder, module: ir.Module, recv_h, field_name: str, value):
    callee = _declare(
        module,
        "nyash.instance.set_bool_field_h",
        ir.IntType(64),
        [ir.IntType(64), ir.IntType(8).as_pointer(), ir.IntType(64)],
    )
    builder.call(callee, [recv_h, _field_ptr(builder, module, field_name), value], name=f"sum_set_bool_{field_name}")


def _set_float_field(builder: ir.IRBuilder, module: ir.Module, recv_h, field_name: str, value):
    callee = _declare(
        module,
        "nyash.instance.set_float_field_h",
        ir.IntType(64),
        [ir.IntType(64), ir.IntType(8).as_pointer(), ir.DoubleType()],
    )
    builder.call(callee, [recv_h, _field_ptr(builder, module, field_name), value], name=f"sum_set_float_{field_name}")


def _set_handle_field(builder: ir.IRBuilder, module: ir.Module, recv_h, field_name: str, value):
    callee = _declare(
        module,
        "nyash.instance.set_field_h",
        ir.IntType(64),
        [ir.IntType(64), ir.IntType(8).as_pointer(), ir.IntType(64)],
    )
    builder.call(callee, [recv_h, _field_ptr(builder, module, field_name), value], name=f"sum_set_handle_{field_name}")


def _get_i64_field(builder: ir.IRBuilder, module: ir.Module, recv_h, field_name: str, *, name_hint: str):
    callee = _declare(
        module,
        "nyash.instance.get_i64_field_h",
        ir.IntType(64),
        [ir.IntType(64), ir.IntType(8).as_pointer()],
    )
    return builder.call(callee, [recv_h, _field_ptr(builder, module, field_name)], name=name_hint)


def _get_bool_field(builder: ir.IRBuilder, module: ir.Module, recv_h, field_name: str, *, name_hint: str):
    callee = _declare(
        module,
        "nyash.instance.get_bool_field_h",
        ir.IntType(64),
        [ir.IntType(64), ir.IntType(8).as_pointer()],
    )
    return builder.call(callee, [recv_h, _field_ptr(builder, module, field_name)], name=name_hint)


def _get_float_field(builder: ir.IRBuilder, module: ir.Module, recv_h, field_name: str, *, name_hint: str):
    callee = _declare(
        module,
        "nyash.instance.get_float_field_h",
        ir.DoubleType(),
        [ir.IntType(64), ir.IntType(8).as_pointer()],
    )
    return builder.call(callee, [recv_h, _field_ptr(builder, module, field_name)], name=name_hint)


def _get_handle_field(builder: ir.IRBuilder, module: ir.Module, recv_h, field_name: str, *, name_hint: str):
    callee = _declare(
        module,
        "nyash.instance.get_field_h",
        ir.IntType(64),
        [ir.IntType(64), ir.IntType(8).as_pointer()],
    )
    return builder.call(callee, [recv_h, _field_ptr(builder, module, field_name)], name=name_hint)
