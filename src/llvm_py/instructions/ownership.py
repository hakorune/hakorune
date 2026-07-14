"""Exact nyash_kernel handle materialization for Ownership SSA V1."""

from __future__ import annotations

from typing import Any, Dict

from llvmlite import ir

from ownership_lowering import OwnershipLoweringContractError
from utils.values import resolve_i64_strict, safe_vmap_write


def _fail(reason: str, detail: str) -> None:
    raise OwnershipLoweringContractError(
        f"[freeze:contract][llvm-py/ownership:{reason}] {detail}"
    )


def _declare(module: ir.Module, name: str, result: ir.Type) -> ir.Function:
    i64 = ir.IntType(64)
    expected = ir.FunctionType(result, [i64])
    existing = module.globals.get(name)
    if existing is not None:
        if not isinstance(existing, ir.Function) or existing.function_type != expected:
            _fail("provider_abi_mismatch", f"symbol={name}")
        return existing
    return ir.Function(module, expected, name=name)


def _site(resolver) -> tuple[int, int]:
    block = getattr(resolver, "current_block_id", None)
    index = getattr(resolver, "current_instruction_index", None)
    if not isinstance(block, int) or not isinstance(index, int):
        _fail("missing_source_site", "ownership lowering requires exact block/instruction site")
    return block, index


def _resolve_handle(builder, src, vmap, resolver, preds, block_end_values, bb_map):
    value = resolve_i64_strict(
        resolver, src, builder.block, preds, block_end_values, vmap, bb_map
    )
    if not isinstance(value.type, ir.IntType) or value.type.width != 64:
        _fail("representation_mismatch", f"%{src} is not an i64 BoxRef handle")
    return value


def lower_copy_owned(owner, builder: ir.IRBuilder, inst: Dict[str, Any]) -> None:
    context = getattr(owner, "context", None)
    session = getattr(context, "ownership_ssa_v1", None)
    if session is None:
        _fail("missing_witness", "copy_owned executed without verified session")
    block, index = _site(owner.resolver)
    session.claim(block, index, inst)
    dst, src = int(inst["dst"]), int(inst["src"])
    vmap = getattr(owner, "_current_vmap", owner.vmap)
    if dst in vmap:
        _fail("dst_already_defined", f"dst=%{dst}")
    source = _resolve_handle(
        builder, src, vmap, owner.resolver, owner.preds, owner.block_end_values, owner.bb_map
    )
    retain = _declare(owner.module, "nyrt_handle_retain_h", ir.IntType(64))
    result = builder.call(retain, [source], name=f"copy_owned_{dst}")
    safe_vmap_write(vmap, dst, result, "copy_owned", resolver=owner.resolver, block_id=block)


def lower_destroy_owned(owner, builder: ir.IRBuilder, inst: Dict[str, Any]) -> None:
    context = getattr(owner, "context", None)
    session = getattr(context, "ownership_ssa_v1", None)
    if session is None:
        _fail("missing_witness", "destroy_owned executed without verified session")
    block, index = _site(owner.resolver)
    session.claim(block, index, inst)
    value_id = int(inst["value"])
    vmap = getattr(owner, "_current_vmap", owner.vmap)
    value = _resolve_handle(
        builder, value_id, vmap, owner.resolver, owner.preds, owner.block_end_values, owner.bb_map
    )
    release = _declare(owner.module, "nyrt_handle_release_h", ir.VoidType())
    builder.call(release, [value])
