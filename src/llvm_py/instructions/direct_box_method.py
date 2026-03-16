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


def resolve_known_box_name(
    box_name: Optional[str],
    receiver_literal: Optional[str] = None,
) -> Optional[str]:
    """Resolve a direct-call box name from the explicit box or module-string receiver."""
    if box_name:
        return box_name
    if not receiver_literal:
        return None
    return _MODULE_RECEIVER_BOX_ALIASES.get(receiver_literal)


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
    callee = resolve_known_box_method(
        module,
        box_name,
        method_name,
        (len(args) + 1, len(args)),
        receiver_literal,
    )
    if callee is None:
        return None

    want_receiver = len(callee.args) == len(args) + 1
    argv = [recv_h] if want_receiver else []
    for arg_vid in args:
        arg_val = resolve_arg(arg_vid)
        if arg_val is None:
            arg_val = ir.Constant(i64, 0)
        argv.append(ensure_handle(arg_val))
    return builder.call(callee, argv, name=call_name)
