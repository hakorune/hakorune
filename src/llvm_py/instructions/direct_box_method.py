"""
Direct lowering helpers for known user-box method calls.

When the receiver box type is statically known and the lowered module already
contains a matching `Box.method/arity` function, prefer a direct call over the
generic plugin invoke shim. This keeps user-defined box methods executable on
native LLVM/AOT routes without relying on plugin-host method resolution.

Contract:
- lowered user-box methods may appear as `Box.method/<args>` or
  `Box.method/<args+1>`
- the latter expects the receiver as implicit `me`, the former does not
"""

from typing import Callable, List, Optional, Tuple

from llvmlite import ir

from naming_helper import encode_static_method


_MODULE_RECEIVER_BOX_ALIASES = {
    "lang.compiler.build.build_box": "BuildBox",
    "lang.compiler.entry.func_scanner": "FuncScannerBox",
    "lang.compiler.entry.stageb.stageb_json_builder_box": "StageBJsonBuilderBox",
    "lang.compiler.entry.using_resolver": "Stage1UsingResolverBox",
    "lang.compiler.entry.using_resolver_box": "Stage1UsingResolverBox",
    "lang.mir.builder.MirBuilderBox": "MirBuilderBox",
    "selfhost.shared.backend.llvm_backend": "LlvmBackendBox",
    "selfhost.shared.common.box_type_inspector": "BoxTypeInspectorBox",
    "selfhost.shared.common.string_helpers": "StringHelpers",
}
_DIRECT_BOX_NAMES = frozenset(_MODULE_RECEIVER_BOX_ALIASES.values())
_PLUGIN_FALLBACK_METHODS = {
    "FileBox": frozenset(
        ("open", "read", "readBytes", "write", "writeBytes", "close")
    ),
}


def _declare(module: ir.Module, name: str, ret, args):
    for f in module.functions:
        if f.name == name:
            return f
    fnty = ir.FunctionType(ret, args)
    return ir.Function(module, fnty, name=name)


def resolve_known_box_name(
    box_name: Optional[str],
    receiver_literal: Optional[str] = None,
) -> Optional[str]:
    """Resolve a direct-call box name from the explicit box or module-string receiver."""
    receiver_box_name = None
    if receiver_literal:
        receiver_box_name = _MODULE_RECEIVER_BOX_ALIASES.get(receiver_literal)

    if box_name:
        if box_name in _DIRECT_BOX_NAMES:
            return box_name
        mapped = _MODULE_RECEIVER_BOX_ALIASES.get(box_name)
        if mapped:
            return mapped
        if receiver_box_name:
            return receiver_box_name
        return box_name

    if receiver_box_name:
        return receiver_box_name
    return None


def resolve_known_box_method(
    module: ir.Module,
    box_name: Optional[str],
    method_name: Optional[str],
    arities: Tuple[int, ...],
    receiver_literal: Optional[str] = None,
):
    """Return the matching lowered function for a known box method, if present."""
    resolved_box_name = resolve_known_box_name(box_name, receiver_literal)
    if not resolved_box_name or not method_name:
        return None

    for arity in arities:
        candidates = [
            encode_static_method(resolved_box_name, method_name, arity),
            f"{resolved_box_name}.{method_name}/{arity}",
        ]
        for candidate in candidates:
            for func in module.functions:
                if func.name == candidate:
                    return func
    plain_candidate = f"{resolved_box_name}.{method_name}"
    for func in module.functions:
        if func.name == plain_candidate:
            return func
    return None


def _unique_global_name(module: ir.Module, base: str) -> str:
    existing = {g.name for g in module.global_values}
    name = base
    suffix = 1
    while name in existing:
        name = f"{base}.{suffix}"
        suffix += 1
    return name


def _declare_method_string(builder: ir.IRBuilder, module: ir.Module, method_name: str):
    i8 = ir.IntType(8)
    i8p = i8.as_pointer()
    arr_bytes = method_name.encode("utf-8") + b"\0"
    arr_ty = ir.ArrayType(i8, len(arr_bytes))
    gname = _unique_global_name(module, f".plugin.meth.{method_name}")
    g = ir.GlobalVariable(module, arr_ty, name=gname)
    g.initializer = ir.Constant(arr_ty, bytearray(arr_bytes))
    g.linkage = "private"
    g.global_constant = True
    c0 = ir.Constant(ir.IntType(32), 0)
    return builder.gep(g, [c0, c0], inbounds=True)


def _lower_plugin_invoke_by_name(
    *,
    builder: ir.IRBuilder,
    module: ir.Module,
    recv_h: ir.Value,
    method_name: str,
    args: List[int],
    resolve_arg: Callable[[int], Optional[ir.Value]],
    ensure_handle: Callable[[ir.Value], ir.Value],
    call_name: str,
):
    i64 = ir.IntType(64)
    i8p = ir.IntType(8).as_pointer()
    method_cstr = _declare_method_string(builder, module, method_name)
    argc = ir.Constant(i64, len(args))
    a1 = ir.Constant(i64, 0)
    a2 = ir.Constant(i64, 0)
    if len(args) > 0:
        arg_val = resolve_arg(args[0])
        if arg_val is None:
            arg_val = ir.Constant(i64, 0)
        a1 = ensure_handle(arg_val)
    if len(args) > 1:
        arg_val = resolve_arg(args[1])
        if arg_val is None:
            arg_val = ir.Constant(i64, 0)
        a2 = ensure_handle(arg_val)
    callee = _declare(module, "nyash.plugin.invoke_by_name_i64", i64, [i64, i8p, i64, i64, i64])
    return builder.call(callee, [recv_h, method_cstr, argc, a1, a2], name=call_name)


def try_lower_known_box_method_call(
    *,
    builder: ir.IRBuilder,
    module: ir.Module,
    box_name: Optional[str],
    method_name: Optional[str],
    recv_h: ir.Value,
    args: List[int],
    resolve_arg: Callable[[int], Optional[ir.Value]],
    ensure_handle: Callable[[ir.Value], ir.Value],
    call_name: str,
    receiver_literal: Optional[str] = None,
):
    """Lower to a direct `Box.method/arity` call when the target exists."""
    i64 = ir.IntType(64)
    resolved_box_name = resolve_known_box_name(box_name, receiver_literal)
    if not resolved_box_name or not method_name:
        return None
    callee = resolve_known_box_method(
        module,
        box_name,
        method_name,
        (len(args) + 1, len(args)),
        receiver_literal,
    )
    if callee is None:
        allow_plugin_fallback = resolved_box_name in _DIRECT_BOX_NAMES
        if not allow_plugin_fallback:
            allowed_methods = _PLUGIN_FALLBACK_METHODS.get(resolved_box_name)
            allow_plugin_fallback = (
                allowed_methods is not None and method_name in allowed_methods
            )
        if not allow_plugin_fallback:
            return None
        return _lower_plugin_invoke_by_name(
            builder=builder,
            module=module,
            recv_h=recv_h,
            method_name=method_name,
            args=args,
            resolve_arg=resolve_arg,
            ensure_handle=ensure_handle,
            call_name=call_name,
        )

    want_receiver = len(callee.args) == len(args) + 1
    argv = [recv_h] if want_receiver else []
    for arg_vid in args:
        arg_val = resolve_arg(arg_vid)
        if arg_val is None:
            arg_val = ir.Constant(i64, 0)
        argv.append(ensure_handle(arg_val))
    return builder.call(callee, argv, name=call_name)
