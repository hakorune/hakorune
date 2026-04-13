"""
LLVM attribute policy helpers for compat/probe keep lowering.

This module keeps the policy narrow and explicit:
- pure read-only query helpers get `readonly`
- pointer arguments for non-capturing runtime bridges get `nocapture`

The policy is applied late by the builder so individual lowering helpers do not
need to duplicate attribute wiring.
"""

from typing import Mapping, Sequence

import llvmlite.ir as ir


_READONLY_FUNCTIONS = {
    "nyash.string.len_h",
    "nyash.string.charCodeAt_h",
    "nyash.string.eq_hh",
    "nyash.string.indexOf_hh",
    "nyash.string.lastIndexOf_hh",
    "nyash.integer.get_h",
    "nyash.bool.get_h",
    "nyash.float.unbox_to_f64",
    "nyash.any.length_h",
}


_NOCAPTURE_POINTER_ARGS: Mapping[str, Sequence[int]] = {
    "nyash.console.log": (0,),
    "nyash.console.warn": (0,),
    "nyash.console.error": (0,),
    "nyash.string.concat_ss": (0, 1),
    "nyash.string.concat_si": (0,),
    "nyash.string.concat_is": (1,),
    "nyash.string.substring_sii": (0,),
    "nyash.string.indexOf_ss": (0, 1),
    "nyash.string.lastIndexOf_ss": (0, 1),
    "nyash.box.from_i8_string": (0,),
    "nyrt_string_length": (0,),
}


def _add_function_attr(func: ir.Function, attr_name: str) -> None:
    try:
        func.attributes.add(attr_name)
    except Exception:
        pass


def _add_arg_attr(arg: ir.Argument, attr_name: str) -> None:
    try:
        arg.add_attribute(attr_name)
    except Exception:
        pass


def _looks_pointer_typed(value) -> bool:
    try:
        return isinstance(value.type, ir.PointerType)
    except Exception:
        return False


def apply_runtime_llvm_attrs(module: ir.Module) -> None:
    """
    Apply a narrow LLVM attribute policy to known runtime helper declarations.

    The policy is intentionally conservative and name-based so it can be applied
    after all lowering has completed without threading extra policy state through
    every instruction helper.
    """
    for func in module.functions:
        if func.name in _READONLY_FUNCTIONS:
            _add_function_attr(func, "readonly")

        arg_indexes = _NOCAPTURE_POINTER_ARGS.get(func.name)
        if not arg_indexes:
            continue

        for idx in arg_indexes:
            if idx < len(func.args):
                arg = func.args[idx]
                if _looks_pointer_typed(arg):
                    _add_arg_attr(arg, "nocapture")
