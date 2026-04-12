from typing import Any, Dict, Optional
import hashlib

import llvmlite.ir as ir

from instructions.primitive_handles import resolver_value_type, unbox_primitive_handle_if_needed
from instructions.thin_entry_selection import (
    thin_entry_prefers_inline_scalar_subject,
)
from utils.values import resolve_i64_strict


_UNSET_LOCAL_FIELD = object()

_LOCAL_LAYOUT_NAMES = {
    "integer": "inline_i64",
    "int": "inline_i64",
    "i64": "inline_i64",
    "integerbox": "inline_i64",
    "bool": "inline_bool",
    "boolean": "inline_bool",
    "boolbox": "inline_bool",
    "float": "inline_f64",
    "f64": "inline_f64",
    "floatbox": "inline_f64",
}


def _declare(module: ir.Module, name: str, ret, args):
    for func in module.functions:
        if func.name == name:
            return func
    return ir.Function(module, ir.FunctionType(ret, args), name=name)


def _field_ptr(builder: ir.IRBuilder, module: ir.Module, field_name: str) -> ir.Value:
    i8 = ir.IntType(8)
    i32 = ir.IntType(32)
    text = str(field_name or "")
    digest = hashlib.sha1(text.encode("utf-8")).hexdigest()[:12]
    global_name = f".field_lit_{digest}"
    data = (text + "\0").encode("utf-8")
    arr_ty = ir.ArrayType(i8, len(data))

    existing = None
    for global_value in module.global_values:
        if global_value.name == global_name:
            existing = global_value
            break

    if existing is None:
        global_var = ir.GlobalVariable(module, arr_ty, name=global_name)
        global_var.linkage = "private"
        global_var.global_constant = True
        global_var.initializer = ir.Constant(arr_ty, bytearray(data))
    else:
        global_var = existing

    c0 = ir.Constant(i32, 0)
    return builder.gep(global_var, [c0, c0], inbounds=True)


def _canonical_i64(builder: ir.IRBuilder, value, *, name_hint: str):
    i64 = ir.IntType(64)
    if value is None:
        return ir.Constant(i64, 0)
    try:
        vtype = value.type
    except Exception:
        vtype = None
    if isinstance(vtype, ir.PointerType):
        return builder.ptrtoint(value, i64, name=f"{name_hint}_p2i")
    if isinstance(vtype, ir.IntType):
        if vtype.width < 64:
            return builder.zext(value, i64, name=f"{name_hint}_zext")
        if vtype.width > 64:
            return builder.trunc(value, i64, name=f"{name_hint}_trunc")
    return value


def _canonical_bool_i1(builder: ir.IRBuilder, value, *, name_hint: str):
    i1 = ir.IntType(1)
    if value is None:
        return ir.Constant(i1, 0)
    try:
        vtype = value.type
    except Exception:
        vtype = None
    if isinstance(vtype, ir.IntType):
        if vtype.width == 1:
            return value
        zero = ir.Constant(vtype, 0)
        return builder.icmp_unsigned("!=", value, zero, name=f"{name_hint}_i1")
    if isinstance(vtype, ir.PointerType):
        i64 = ir.IntType(64)
        as_i64 = builder.ptrtoint(value, i64, name=f"{name_hint}_p2i")
        return builder.icmp_unsigned("!=", as_i64, ir.Constant(i64, 0), name=f"{name_hint}_i1")
    i64 = ir.IntType(64)
    as_i64 = _canonical_i64(builder, value, name_hint=f"{name_hint}_coerce")
    return builder.icmp_unsigned("!=", as_i64, ir.Constant(i64, 0), name=f"{name_hint}_i1")


def _canonical_bool_i64(builder: ir.IRBuilder, value, *, name_hint: str):
    i64 = ir.IntType(64)
    if value is None:
        return ir.Constant(i64, 0)
    bool_i1 = _canonical_bool_i1(builder, value, name_hint=name_hint)
    return builder.zext(bool_i1, i64, name=f"{name_hint}_i64")


def _canonical_f64(builder: ir.IRBuilder, value, *, name_hint: str):
    f64 = ir.DoubleType()
    if value is None:
        return ir.Constant(f64, 0.0)
    try:
        vtype = value.type
    except Exception:
        vtype = None
    if isinstance(vtype, ir.DoubleType):
        return value
    raise RuntimeError(
        f"[user_box_local] expected f64 for {name_hint}, got {vtype}"
    )


def _new_user_box_handle(
    builder: ir.IRBuilder,
    module: ir.Module,
    box_type: str,
    *,
    name_hint: str,
) -> ir.Value:
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()
    new_i64x = _declare(
        module,
        "nyash.env.box.new_i64x",
        i64,
        [i8p, i64, i64, i64, i64, i64],
    )

    type_bytes = (box_type + "\0").encode("utf-8")
    arr_ty = ir.ArrayType(ir.IntType(8), len(type_bytes))
    global_name = f".user_box_ty_{hashlib.sha1(box_type.encode('utf-8')).hexdigest()[:12]}"
    existing = None
    for global_value in module.global_values:
        if global_value.name == global_name:
            existing = global_value
            break
    if existing is None:
        global_var = ir.GlobalVariable(module, arr_ty, name=global_name)
        global_var.linkage = "private"
        global_var.global_constant = True
        global_var.initializer = ir.Constant(arr_ty, bytearray(type_bytes))
    else:
        global_var = existing

    c0 = ir.Constant(ir.IntType(32), 0)
    ptr = builder.gep(global_var, [c0, c0], inbounds=True)
    zero = ir.Constant(i64, 0)
    return builder.call(
        new_i64x,
        [ptr, zero, zero, zero, zero, zero],
        name=f"{name_hint}_newbox",
    )


def _set_i64_field(
    builder: ir.IRBuilder,
    module: ir.Module,
    recv_h: ir.Value,
    field_name: str,
    value,
    *,
    name_hint: str,
) -> None:
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()
    callee = _declare(module, "nyash.instance.set_i64_field_h", i64, [i64, i8p, i64])
    builder.call(
        callee,
        [recv_h, _field_ptr(builder, module, field_name), _canonical_i64(builder, value, name_hint=name_hint)],
        name=f"{name_hint}_set_i64",
    )


def _set_bool_field(
    builder: ir.IRBuilder,
    module: ir.Module,
    recv_h: ir.Value,
    field_name: str,
    value,
    *,
    name_hint: str,
) -> None:
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()
    callee = _declare(module, "nyash.instance.set_bool_field_h", i64, [i64, i8p, i64])
    builder.call(
        callee,
        [
            recv_h,
            _field_ptr(builder, module, field_name),
            _canonical_bool_i64(builder, value, name_hint=name_hint),
        ],
        name=f"{name_hint}_set_bool",
    )


def _set_float_field(
    builder: ir.IRBuilder,
    module: ir.Module,
    recv_h: ir.Value,
    field_name: str,
    value,
    *,
    name_hint: str,
) -> None:
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()
    f64 = ir.DoubleType()
    callee = _declare(module, "nyash.instance.set_float_field_h", i64, [i64, i8p, f64])
    builder.call(
        callee,
        [recv_h, _field_ptr(builder, module, field_name), _canonical_f64(builder, value, name_hint=name_hint)],
        name=f"{name_hint}_set_float",
    )


def _layout_store(resolver) -> Dict[int, Dict[str, Any]]:
    layouts = getattr(resolver, "user_box_local_aggregate_layouts", None)
    if isinstance(layouts, dict):
        return layouts
    layouts = {}
    setattr(resolver, "user_box_local_aggregate_layouts", layouts)
    return layouts


def _normalize_declared_type_name(raw: Any) -> Optional[str]:
    if not isinstance(raw, str):
        return None
    lower = raw.strip().lower()
    return lower if lower else None


def _declared_type_to_local_layout(raw: Any) -> Optional[str]:
    lower = _normalize_declared_type_name(raw)
    if lower is None:
        return None
    return _LOCAL_LAYOUT_NAMES.get(lower)


def _inline_scalar_selected(resolver, surface: str, subject: str) -> bool:
    return bool(
        thin_entry_prefers_inline_scalar_subject(
            resolver=resolver,
            surface=surface,
            subject=subject,
        )
    )


def _resolve_root_alias(value_id: Any, aliases: Dict[int, int]) -> Optional[int]:
    if not isinstance(value_id, int):
        return None
    current = int(value_id)
    seen = set()
    while current in aliases and aliases[current] != current:
        if current in seen:
            break
        seen.add(current)
        current = aliases[current]
    if current in aliases:
        return current
    return None


def seed_local_user_box_layouts_from_function_data(builder, func_data: Dict[str, Any]) -> None:
    resolver = getattr(builder, "resolver", None)
    if resolver is None:
        return

    box_decls = getattr(builder, "user_box_decls", []) or []
    decl_layouts: Dict[str, Dict[str, Any]] = {}
    for box_decl in box_decls:
        if not isinstance(box_decl, dict):
            continue
        box_name = box_decl.get("name")
        field_decls = box_decl.get("field_decls")
        if not isinstance(box_name, str) or not isinstance(field_decls, list):
            continue
        field_order = []
        field_layouts: Dict[str, str] = {}
        supported = True
        for field_decl in field_decls:
            if not isinstance(field_decl, dict):
                supported = False
                break
            field_name = field_decl.get("name")
            if not isinstance(field_name, str) or bool(field_decl.get("is_weak")):
                supported = False
                break
            layout_name = _declared_type_to_local_layout(field_decl.get("declared_type"))
            if layout_name is None:
                supported = False
                break
            field_order.append(field_name)
            field_layouts[field_name] = layout_name
        if supported and field_layouts:
            decl_layouts[box_name] = {
                "box_name": box_name,
                "field_order": field_order,
                "field_layouts": field_layouts,
            }

    blocks = func_data.get("blocks", [])
    aliases: Dict[int, int] = {}
    candidates: Dict[int, Dict[str, Any]] = {}

    def disallow(root: int, reason: str) -> None:
        if root in candidates and not candidates[root].get("disallowed_reason"):
            candidates[root]["disallowed_reason"] = reason

    for block_index, block in enumerate(blocks):
        instructions = block.get("instructions", [])
        block_id = block.get("id", block_index)
        if not isinstance(block_id, int):
            block_id = block_index
        for instruction_index, inst in enumerate(instructions):
            if not isinstance(inst, dict):
                continue
            op = inst.get("op")
            if op == "newbox":
                dst = inst.get("dst")
                box_type = inst.get("type")
                args = inst.get("args", [])
                if (
                    isinstance(dst, int)
                    and isinstance(box_type, str)
                    and box_type in decl_layouts
                    and isinstance(args, list)
                    and len(args) == 0
                ):
                    layout_info = decl_layouts[box_type]
                    candidates[int(dst)] = {
                        "box_name": box_type,
                        "block": int(block_id),
                        "instruction_index": int(instruction_index),
                        "initialized_fields": set(),
                        "disallowed_reason": None,
                        "field_order": list(layout_info["field_order"]),
                        "field_layouts": dict(layout_info["field_layouts"]),
                    }
                    aliases[int(dst)] = int(dst)
                continue

            if op == "copy":
                dst = inst.get("dst")
                src = inst.get("src")
                src_root = _resolve_root_alias(src, aliases)
                if isinstance(dst, int) and src_root is not None:
                    aliases[int(dst)] = src_root
                continue

            if op == "field_set":
                base_root = _resolve_root_alias(inst.get("box"), aliases)
                value_root = _resolve_root_alias(inst.get("value"), aliases)
                if value_root is not None and value_root != base_root:
                    disallow(
                        value_root,
                        "local primitive user-box route stops when the box value itself is stored through another field.set",
                    )
                if base_root is None or base_root not in candidates:
                    continue
                candidate = candidates[base_root]
                field_name = inst.get("field")
                if not isinstance(field_name, str) or field_name not in candidate["field_layouts"]:
                    disallow(base_root, "field.set touched a field that is outside the selected primitive layout")
                    continue
                subject = f"{candidate['box_name']}.{field_name}"
                if not _inline_scalar_selected(resolver, "user_box_field_set", subject):
                    disallow(base_root, "field.set lacks an inline-scalar thin-entry selection for this primitive route")
                    continue
                if (
                    int(block_id) == candidate["block"]
                    and instruction_index > candidate["instruction_index"]
                ):
                    candidate["initialized_fields"].add(field_name)
                elif field_name not in candidate["initialized_fields"]:
                    disallow(
                        base_root,
                        "field.set would leave the birth block before this primitive field is initialized",
                    )
                continue

            if op == "field_get":
                base_root = _resolve_root_alias(inst.get("box"), aliases)
                if base_root is None or base_root not in candidates:
                    continue
                candidate = candidates[base_root]
                field_name = inst.get("field")
                if not isinstance(field_name, str) or field_name not in candidate["field_layouts"]:
                    disallow(base_root, "field.get touched a field that is outside the selected primitive layout")
                    continue
                subject = f"{candidate['box_name']}.{field_name}"
                if not _inline_scalar_selected(resolver, "user_box_field_get", subject):
                    disallow(base_root, "field.get lacks an inline-scalar thin-entry selection for this primitive route")
                    continue
                if field_name not in candidate["initialized_fields"]:
                    disallow(
                        base_root,
                        "field.get would read a primitive field before the local aggregate is fully initialized",
                    )
                continue

            if op in ("call", "ret", "boxcall"):
                candidate_roots = []
                if op == "ret":
                    candidate_roots.append(_resolve_root_alias(inst.get("value"), aliases))
                elif op == "call":
                    for arg in inst.get("args", []) or []:
                        candidate_roots.append(_resolve_root_alias(arg, aliases))
                elif op == "boxcall":
                    candidate_roots.append(_resolve_root_alias(inst.get("box"), aliases))
                    for arg in inst.get("args", []) or []:
                        candidate_roots.append(_resolve_root_alias(arg, aliases))
                for root in candidate_roots:
                    if root is None or root not in candidates:
                        continue
                    candidate = candidates[root]
                    if set(candidate["field_order"]) != set(candidate["initialized_fields"]):
                        disallow(
                            root,
                            f"{op} would materialize the local user-box route before all primitive fields finish birth-block initialization",
                        )
                continue

            value_keys = []
            if op == "binop":
                value_keys = ["lhs", "rhs"]
            elif op == "compare":
                value_keys = ["lhs", "rhs", "left", "right"]
            elif op == "unop":
                value_keys = ["src", "operand"]
            elif op == "typeop":
                value_keys = ["value", "src"]
            elif op == "select":
                value_keys = ["cond", "then_val", "else_val"]
            elif op == "branch":
                value_keys = ["cond"]
            elif op == "load":
                value_keys = ["ptr"]
            elif op == "store":
                value_keys = ["value", "ptr"]
            elif op == "mir_call":
                value_keys = ["callee"]
                args = inst.get("args")
                if isinstance(args, list):
                    value_keys.extend(args)
            elif op == "keepalive" or op == "release_strong":
                args = inst.get("values")
                if isinstance(args, list):
                    value_keys.extend(args)
            elif op == "phi":
                incoming = inst.get("values")
                if incoming is None:
                    incoming = inst.get("incoming")
                if isinstance(incoming, list):
                    for ent in incoming:
                        if isinstance(ent, list) and ent:
                            root = _resolve_root_alias(ent[0], aliases)
                            if root is not None:
                                disallow(root, "phi merge keeps the current user-box local aggregate slice on compat fallback")
                        elif isinstance(ent, dict):
                            root = _resolve_root_alias(ent.get("value"), aliases)
                            if root is not None:
                                disallow(root, "phi merge keeps the current user-box local aggregate slice on compat fallback")
                continue

            for key in value_keys:
                if isinstance(key, int):
                    root = _resolve_root_alias(key, aliases)
                else:
                    root = _resolve_root_alias(inst.get(key), aliases)
                if root is not None:
                    disallow(
                        root,
                        f"{op} uses the box value outside newbox/field/call/ret boundaries, so this slice keeps compat handle lowering",
                    )

    selected: Dict[int, Dict[str, Any]] = {}
    for root, candidate in candidates.items():
        if candidate.get("disallowed_reason"):
            continue
        if set(candidate["field_order"]) != set(candidate["initialized_fields"]):
            continue
        selected[int(root)] = {
            "box_name": candidate["box_name"],
            "field_order": list(candidate["field_order"]),
            "field_layouts": dict(candidate["field_layouts"]),
            "reason": "primitive user-box fields initialize in the birth block and only leave the local route through explicit call/ret boundaries",
        }

    resolver.user_box_local_aggregate_layouts = selected


def _is_local_user_box_aggregate(value: Any) -> bool:
    return isinstance(value, dict) and value.get("kind") == "local_user_box_aggregate"


def uses_local_user_box_aggregate(resolver, value_id: Optional[int], box_type: Optional[str] = None) -> bool:
    if not isinstance(value_id, int):
        return False
    layout = _layout_store(resolver).get(int(value_id))
    if not isinstance(layout, dict):
        return False
    if isinstance(box_type, str) and layout.get("box_name") != box_type:
        return False
    return True


def build_local_user_box_aggregate_for_newbox(
    resolver,
    dst_vid: Optional[int],
    box_type: Optional[str],
):
    if not isinstance(dst_vid, int) or not isinstance(box_type, str):
        return None
    layout = _layout_store(resolver).get(int(dst_vid))
    if not isinstance(layout, dict) or layout.get("box_name") != box_type:
        return None
    return {
        "kind": "local_user_box_aggregate",
        "box_name": box_type,
        "field_order": list(layout.get("field_order", [])),
        "field_layouts": dict(layout.get("field_layouts", {})),
        "fields": {
            field_name: _UNSET_LOCAL_FIELD
            for field_name in layout.get("field_order", [])
        },
    }


def _resolve_local_user_box_aggregate(value_vid: int, vmap: Dict[int, Any], resolver):
    direct = vmap.get(int(value_vid))
    if _is_local_user_box_aggregate(direct):
        return direct

    try:
        global_vmap = getattr(resolver, "global_vmap", None)
        if isinstance(global_vmap, dict):
            global_value = global_vmap.get(int(value_vid))
            if _is_local_user_box_aggregate(global_value):
                return global_value
    except Exception:
        pass

    try:
        current_bid = getattr(resolver, "current_block_id", None)
        ctx = getattr(resolver, "context", None)
        if current_bid is not None and ctx is not None and hasattr(ctx, "get_block_snapshot"):
            snapshot = ctx.get_block_snapshot(int(current_bid))
            snap_value = snapshot.get(int(value_vid))
            if _is_local_user_box_aggregate(snap_value):
                return snap_value
    except Exception:
        pass

    return None


def _copy_local_user_box_metadata_alias(resolver, src_vid: int, dst_vid: int) -> None:
    if resolver is None:
        return
    layouts = _layout_store(resolver)
    if int(src_vid) in layouts:
        layouts[int(dst_vid)] = layouts[int(src_vid)]


def _resolve_i64_value(
    builder: ir.IRBuilder,
    value_vid: Optional[int],
    vmap: Dict[int, Any],
    resolver,
    preds,
    block_end_values,
    bb_map,
    *,
    hot_scope: str,
):
    if not isinstance(value_vid, int):
        return ir.Constant(ir.IntType(64), 0)
    return resolve_i64_strict(
        resolver,
        int(value_vid),
        builder.block,
        preds,
        block_end_values,
        vmap,
        bb_map,
        hot_scope=hot_scope,
    )


def _resolve_f64_value(
    builder: ir.IRBuilder,
    value_vid: Optional[int],
    vmap: Dict[int, Any],
    resolver,
    preds,
    block_end_values,
    bb_map,
):
    if not isinstance(value_vid, int):
        return ir.Constant(ir.DoubleType(), 0.0)
    value = vmap.get(int(value_vid))
    if value is None:
        value = _resolve_i64_value(
            builder,
            int(value_vid),
            vmap,
            resolver,
            preds,
            block_end_values,
            bb_map,
            hot_scope="user_box_field_float",
        )
    value_meta = resolver_value_type(resolver, int(value_vid))
    value = unbox_primitive_handle_if_needed(
        builder,
        value,
        value_meta,
        name_hint=f"user_box_local_float_{value_vid}",
    )
    return _canonical_f64(builder, value, name_hint=f"user_box_local_float_{value_vid}")


def lower_local_user_box_field_get(
    builder: ir.IRBuilder,
    box_vid: Optional[int],
    field_name: str,
    dst_vid: Optional[int],
    vmap: Dict[int, Any],
    resolver,
    mark_integer,
    mark_bool,
    mark_float,
):
    if not isinstance(box_vid, int):
        return None
    local_box = _resolve_local_user_box_aggregate(int(box_vid), vmap, resolver)
    if local_box is None:
        return None

    field_layouts = local_box.get("field_layouts", {})
    if field_name not in field_layouts:
        raise RuntimeError(
            f"[user_box_local] missing field layout for {local_box.get('box_name')}.{field_name}"
        )

    stored = local_box.get("fields", {}).get(field_name, _UNSET_LOCAL_FIELD)
    if stored is _UNSET_LOCAL_FIELD:
        raise RuntimeError(
            f"[user_box_local] local aggregate read before initialization: {local_box.get('box_name')}.{field_name}"
        )

    layout_name = field_layouts[field_name]
    if layout_name == "inline_i64":
        result = _canonical_i64(builder, stored, name_hint=f"user_box_local_get_{field_name}")
        if dst_vid is not None:
            vmap[int(dst_vid)] = result
            mark_integer(resolver, int(dst_vid))
        return result
    if layout_name == "inline_bool":
        result = _canonical_bool_i1(builder, stored, name_hint=f"user_box_local_get_{field_name}")
        if dst_vid is not None:
            vmap[int(dst_vid)] = result
            mark_bool(resolver, int(dst_vid))
        return result
    if layout_name == "inline_f64":
        result = _canonical_f64(builder, stored, name_hint=f"user_box_local_get_{field_name}")
        if dst_vid is not None:
            vmap[int(dst_vid)] = result
            mark_float(resolver, int(dst_vid))
        return result
    raise RuntimeError(f"[user_box_local] unsupported local field layout: {layout_name}")


def lower_local_user_box_field_set(
    builder: ir.IRBuilder,
    box_vid: Optional[int],
    field_name: str,
    value_vid: Optional[int],
    vmap: Dict[int, Any],
    resolver,
    preds,
    block_end_values,
    bb_map,
):
    if not isinstance(box_vid, int):
        return False
    local_box = _resolve_local_user_box_aggregate(int(box_vid), vmap, resolver)
    if local_box is None:
        return False

    field_layouts = local_box.get("field_layouts", {})
    if field_name not in field_layouts:
        raise RuntimeError(
            f"[user_box_local] missing field layout for {local_box.get('box_name')}.{field_name}"
        )

    layout_name = field_layouts[field_name]
    if layout_name == "inline_i64":
        raw_value = _resolve_i64_value(
            builder,
            value_vid,
            vmap,
            resolver,
            preds,
            block_end_values,
            bb_map,
            hot_scope="user_box_field_i64",
        )
        value_meta = resolver_value_type(resolver, int(value_vid)) if isinstance(value_vid, int) else None
        raw_value = unbox_primitive_handle_if_needed(
            builder,
            _canonical_i64(builder, raw_value, name_hint=f"user_box_local_set_{field_name}"),
            value_meta,
            name_hint=f"user_box_local_set_i64_{value_vid}",
        )
        local_box["fields"][field_name] = _canonical_i64(
            builder,
            raw_value,
            name_hint=f"user_box_local_store_{field_name}",
        )
        return True

    if layout_name == "inline_bool":
        raw_value = vmap.get(int(value_vid)) if isinstance(value_vid, int) else None
        if raw_value is None:
            raw_value = _resolve_i64_value(
                builder,
                value_vid,
                vmap,
                resolver,
                preds,
                block_end_values,
                bb_map,
                hot_scope="user_box_field_bool",
            )
        value_meta = resolver_value_type(resolver, int(value_vid)) if isinstance(value_vid, int) else None
        raw_value = unbox_primitive_handle_if_needed(
            builder,
            raw_value,
            value_meta,
            name_hint=f"user_box_local_set_bool_{value_vid}",
        )
        local_box["fields"][field_name] = _canonical_bool_i1(
            builder,
            raw_value,
            name_hint=f"user_box_local_store_{field_name}",
        )
        return True

    if layout_name == "inline_f64":
        local_box["fields"][field_name] = _resolve_f64_value(
            builder,
            value_vid,
            vmap,
            resolver,
            preds,
            block_end_values,
            bb_map,
        )
        return True

    raise RuntimeError(f"[user_box_local] unsupported local field layout: {layout_name}")


def materialize_local_user_box_aggregate(
    builder: ir.IRBuilder,
    module: ir.Module,
    local_box,
    *,
    name_hint: str,
):
    if not _is_local_user_box_aggregate(local_box):
        return local_box
    box_name = local_box.get("box_name")
    if not isinstance(box_name, str) or not box_name:
        raise RuntimeError("[user_box_local] local aggregate is missing box_name")

    recv_h = _new_user_box_handle(builder, module, box_name, name_hint=name_hint)
    field_layouts = local_box.get("field_layouts", {})
    fields = local_box.get("fields", {})
    for field_name in local_box.get("field_order", []):
        value = fields.get(field_name, _UNSET_LOCAL_FIELD)
        if value is _UNSET_LOCAL_FIELD:
            raise RuntimeError(
                f"[user_box_local] attempted to materialize {box_name}.{field_name} before initialization"
            )
        layout_name = field_layouts.get(field_name)
        if layout_name == "inline_i64":
            _set_i64_field(
                builder,
                module,
                recv_h,
                field_name,
                value,
                name_hint=f"{name_hint}_{field_name}",
            )
        elif layout_name == "inline_bool":
            _set_bool_field(
                builder,
                module,
                recv_h,
                field_name,
                value,
                name_hint=f"{name_hint}_{field_name}",
            )
        elif layout_name == "inline_f64":
            _set_float_field(
                builder,
                module,
                recv_h,
                field_name,
                value,
                name_hint=f"{name_hint}_{field_name}",
            )
        else:
            raise RuntimeError(
                f"[user_box_local] unsupported local field layout during materialization: {layout_name}"
            )
    return recv_h


def materialize_user_box_escape_value_if_needed(
    builder: ir.IRBuilder,
    module: ir.Module,
    value_id: Optional[int],
    vmap: Dict[int, Any],
    resolver=None,
    *,
    name_hint: str = "user_box_escape",
):
    if not isinstance(value_id, int):
        return None
    local_box = _resolve_local_user_box_aggregate(int(value_id), vmap, resolver)
    if local_box is None:
        return None
    return materialize_local_user_box_aggregate(
        builder,
        module,
        local_box,
        name_hint=name_hint,
    )
