#!/usr/bin/env python3
"""Live post-success/type-fact ownership checks for the FACT0 partition guard.

The parent FACT0 guard freezes the historical partition and retains the green
source-topology checks.  These five checks live here because their lexical
owners moved after that freeze; none of them refreshes the historical fixture.
"""

from __future__ import annotations

from pathlib import Path

from mirbuilder_type_fact_producer_inventory import (
    code_only,
    fail,
    read,
    strip_cfg_test_modules,
)


def _window(text: str, start: str, end: str) -> str:
    try:
        body = text.split(start, 1)[1]
        if end not in body:
            fail(f"missing live owner window end: {end!r}")
        return body.split(end, 1)[0]
    except (IndexError, ValueError):
        fail(f"missing live owner window: {start!r} -> {end!r}")


def _require_once(text: str, token: str, label: str) -> None:
    if text.count(token) != 1:
        fail(f"{label} requires exactly one {token!r}")


def _require_after(text: str, before: str, after: str, label: str) -> None:
    before_at = text.find(before)
    after_at = text.find(after)
    if before_at < 0 or after_at < 0 or before_at >= after_at:
        fail(f"{label} ordering drift")


def _check_line_caps(root: Path, paths: tuple[Path, ...], label: str) -> None:
    for path in paths:
        if len(read(path).splitlines()) >= 800:
            fail(f"{label} source/check file reached 800 lines: {path}")


def validate_const0_authority_v1(root: Path) -> None:
    constant = strip_cfg_test_modules(
        code_only(read(root / "src/mir/builder/emission/constant.rs"))
    )
    owner = code_only(read(root / "src/mir/builder/emission/constant_type.rs"))

    if "value_types.insert" in constant or "value_origin_newbox" in constant:
        fail("CONST0 direct type/origin writer survived in constant.rs")
    for label, start, end in (
        (
            "CONST0 fresh-destination emission",
            "fn emit_exact_const_at(",
            "fn emit_exact_const(",
        ),
        (
            "CONST0 caller-issued destination emission",
            "pub(in crate::mir) fn emit_integer_at_with_dst(",
            "pub fn emit_integer(",
        ),
    ):
        window = _window(constant, start, end)
        _require_once(window, "PreparedCanonicalConstTypeV1::prepare", label)
        _require_once(window, "MirInstruction::Const", label)
        _require_once(window, "prepared.commit(", label)
        _require_after(
            window,
            "MirInstruction::Const",
            "prepared.commit(",
            f"{label} prepare/emit/commit",
        )
    for delegate in (
        "emit_integer",
        "emit_bool",
        "emit_float",
        "emit_string",
        "emit_null",
        "emit_void",
    ):
        _require_once(constant, f"pub fn {delegate}", f"CONST0 delegate {delegate}")
    _require_once(constant, "string_literals.insert", "CONST0 String companion")
    _require_after(
        constant,
        "prepared.commit(",
        "string_literals.insert",
        "CONST0 String companion must follow the type commit",
    )
    if (
        owner.count("TypeFactDecisionV1::prepare") != 1
        or owner.count("type_ctx.set_type") != 1
    ):
        fail("CONST0 decision/commit owner drift")
    _check_line_caps(
        root,
        (
            root / "src/mir/builder/emission/constant.rs",
            root / "src/mir/builder/emission/constant_type.rs",
            Path(__file__),
        ),
        "CONST0",
    )


def validate_literal_postemit_retirement_v1(root: Path) -> None:
    literal = code_only(read(root / "src/mir/builder/literal_lowering.rs"))
    resolved_lowerer = read(root / "src/mir/builder/resolved_lowering/lowerer.rs")
    unary = read(root / "src/mir/builder/ops/unary.rs")

    literal_dispatch = _window(
        literal,
        "pub(in crate::mir::builder) fn build_literal(",
        "pub(in crate::mir::builder) fn emit_typed_integer_literal(",
    )
    resolved_literal = resolved_lowerer.split("fn lower_literal", 1)[1]
    folded_negative = unary.split("if operator.is_minus()", 1)[1].split(
        "let operand_val", 1
    )[0]

    if "value_types.insert" in literal_dispatch:
        fail("LITERAL-POSTEMIT-RET0 literal dispatch direct type writer survived")
    if "value_types.insert" in resolved_literal:
        fail("LITERAL-POSTEMIT-RET0 resolved Null/Void direct type writer survived")
    if "value_types.insert" in folded_negative:
        fail("LITERAL-POSTEMIT-RET0 folded negative direct type writer survived")
    if literal_dispatch.count("emission::constant::emit_") != 6:
        fail(
            "LITERAL-POSTEMIT-RET0 literal dispatch must retain six canonical Const delegates"
        )
    if "emit_typed_integer_literal" not in literal_dispatch:
        fail("LITERAL-POSTEMIT-RET0 TypedInteger canonical delegate missing")
    if "build_literal(literal.clone())" not in resolved_literal:
        fail(
            "LITERAL-POSTEMIT-RET0 resolved literal must delegate to canonical literal lowering"
        )
    if "emission::constant::emit_integer(builder, negated)" not in folded_negative:
        fail("LITERAL-POSTEMIT-RET0 folded negative must delegate to canonical Const")
    _check_line_caps(
        root,
        (
            root / "src/mir/builder/literal_lowering.rs",
            root / "src/mir/builder/resolved_lowering/lowerer.rs",
            root / "src/mir/builder/ops/unary.rs",
            Path(__file__),
        ),
        "LITERAL-POSTEMIT-RET0",
    )


def validate_resolved_direct_call_authority_v1(root: Path) -> None:
    direct_call = code_only(
        read(root / "src/mir/builder/resolved_lowering/trivial_ssa/direct_call.rs")
    )
    production = _window(
        direct_call,
        "pub(super) fn emit(",
        "#[cfg(test)]",
    )
    owner = code_only(
        read(root / "src/mir/builder/resolved_lowering/trivial_ssa/direct_call_type.rs")
    )

    if "value_types.insert" in production or "type_ctx.set_type" in production:
        fail("RESOLVED-DIRECT-CALL0 direct type writer survived in direct_call.rs")
    _require_once(
        production,
        "PreparedResolvedDirectCallIntegerTypeV1::prepare",
        "RESOLVED-DIRECT-CALL0 production",
    )
    _require_once(
        production,
        "prepared.commit(",
        "RESOLVED-DIRECT-CALL0 production",
    )
    _require_after(
        production,
        "builder.emit_instruction(instruction)?",
        "prepared.commit(",
        "RESOLVED-DIRECT-CALL0 production commit",
    )
    if owner.count("TypeFactDecisionV1::prepare") != 1 or owner.count(
        "type_ctx.set_type"
    ) != 1:
        fail("RESOLVED-DIRECT-CALL0 decision/commit owner drift")
    _check_line_caps(
        root,
        (
            root / "src/mir/builder/resolved_lowering/trivial_ssa/direct_call.rs",
            root / "src/mir/builder/resolved_lowering/trivial_ssa/direct_call_type.rs",
            Path(__file__),
        ),
        "RESOLVED-DIRECT-CALL0",
    )


def validate_call_receipt0_authority_v1(root: Path) -> None:
    emitter = strip_cfg_test_modules(
        code_only(read(root / "src/mir/builder/calls/unified_emitter.rs"))
    )
    physical = code_only(
        read(root / "src/mir/builder/calls/unified_emitter/physical_terminal.rs")
    )
    request = code_only(
        read(root / "src/mir/builder/calls/unified_emitter/request_boundary.rs")
    )
    post_success = strip_cfg_test_modules(
        code_only(read(root / "src/mir/builder/calls/unified_emitter/post_success.rs"))
    )

    _require_once(
        emitter,
        "physical_terminal::emit_finalized_generic_call_v1",
        "CALL-RECEIPT0",
    )
    _require_once(
        physical,
        "pub(super) fn emit_finalized_generic_call_v1",
        "CALL-RECEIPT0",
    )
    _require_once(physical, "MirInstruction::call(", "CALL-RECEIPT0")
    _require_once(
        physical,
        "PreparedUnifiedCallPostSuccessV1::prepare",
        "CALL-RECEIPT0",
    )
    _require_once(
        physical,
        "prepared_post_success.commit_after_success(builder)",
        "CALL-RECEIPT0",
    )
    constructor = (
        "CompletedUnifiedCallEmissionV1::Value("
        "CompletedUnifiedValueCallEmissionV1 {"
    )
    _require_once(physical, constructor, "CALL-RECEIPT0")
    _require_after(
        physical,
        "builder.emit_instruction(call_inst)?",
        "prepared_post_success.commit_after_success(builder)",
        "CALL-RECEIPT0 Call success -> post-success",
    )
    _require_after(
        physical,
        "prepared_post_success.commit_after_success(builder)",
        constructor,
        "CALL-RECEIPT0 post-success -> receipt",
    )
    _require_once(
        request,
        "fn emit_unified_value_call_with_lookup_receipt_v1",
        "CALL-RECEIPT0",
    )
    if "emit_legacy_call" in request:
        fail("CALL-RECEIPT0 receipt request must not retry through legacy emission")
    for forbidden in (
        "ASTNode",
        "MirType",
        "type_ctx",
        "annotate_call_result_from_func_name",
        "annotate_array_element_result",
        "annotate_map_get_result",
        "verify_after_call",
    ):
        if forbidden in physical:
            fail(f"CALL-RECEIPT0 forbidden physical-terminal authority: {forbidden}")
    _require_once(post_success, "fn commit_after_success", "CALL-RECEIPT0")
    for required in (
        "annotate_call_result_from_func_name_with_lookup(",
        "annotate_call_result_from_func_name(",
        "annotate_array_element_result(",
        "annotate_map_get_result(",
        "verify_after_call(",
    ):
        _require_once(post_success, required, "CALL-RECEIPT0 post-success owner")
    _check_line_caps(
        root,
        (
            root / "src/mir/builder/calls/unified_emitter.rs",
            root / "src/mir/builder/calls/unified_emitter/physical_terminal.rs",
            root / "src/mir/builder/calls/unified_emitter/post_success.rs",
            root / "src/mir/builder/calls/unified_emitter/request_boundary.rs",
            root / "src/mir/builder/calls/unified_emitter/physical_receipt_tests.rs",
            root / "src/mir/builder/calls/unified_emitter/temporal_witness_tests.rs",
            Path(__file__),
        ),
        "CALL-RECEIPT0",
    )


def validate_map_write_observe0_authority_v1(root: Path) -> None:
    emitter = strip_cfg_test_modules(
        code_only(read(root / "src/mir/builder/calls/unified_emitter.rs"))
    )
    receipt = strip_cfg_test_modules(
        code_only(read(root / "src/mir/builder/calls/unified_emitter/post_success.rs"))
    )
    boxcall = strip_cfg_test_modules(
        code_only(read(root / "src/mir/builder/utils/boxcall_emit.rs"))
    )
    replay = code_only(read(root / "src/mir/builder/types/map_value/post_success.rs"))

    _require_once(
        emitter,
        "PreparedMapWriteReplayV1::prepare",
        "MAP-WRITE-OBSERVE0 Unified",
    )
    if emitter.count("append_if_distinct_receiver") != 2:
        fail("MAP-WRITE-OBSERVE0 Unified S/L/R replay sequence drift")
    if "observe_map_write_call" in emitter:
        fail("MAP-WRITE-OBSERVE0 direct Unified pre-receipt observer survived")
    _require_once(
        receipt,
        "observe_map_write_call",
        "MAP-WRITE-OBSERVE0 Unified receipt replay",
    )

    _require_once(
        boxcall,
        "PreparedMapWriteReplayV1::prepare",
        "MAP-WRITE-OBSERVE0 BoxCall",
    )
    _require_once(
        boxcall,
        "emit_unified_call_with_map_replay",
        "MAP-WRITE-OBSERVE0 BoxCall",
    )
    _require_once(
        boxcall,
        "observe_map_write_call",
        "MAP-WRITE-OBSERVE0 BoxCall",
    )
    terminal = boxcall.split(
        "self.emit_instruction(crate::mir::ssot::method_call::method_call(",
        1,
    )[1]
    _require_after(
        terminal,
        "))?;",
        "observe_map_write_call",
        "MAP-WRITE-OBSERVE0 terminal replay",
    )

    _require_once(
        replay,
        "pub(in crate::mir::builder) struct PreparedMapWriteReplayV1",
        "MAP-WRITE-OBSERVE0 replay",
    )
    if "MirBuilder" in replay or "observe_map_write_call" in replay:
        fail("MAP-WRITE-OBSERVE0 replay product must remain Builder-free")
    _check_line_caps(
        root,
        (
            root / "src/mir/builder/calls/unified_emitter.rs",
            root / "src/mir/builder/calls/unified_emitter/post_success.rs",
            root / "src/mir/builder/utils/boxcall_emit.rs",
            root / "src/mir/builder/types/map_value.rs",
            root / "src/mir/builder/types/map_value/post_success.rs",
            root / "src/mir/builder/calls/unified_emitter/map_write_timing_tests.rs",
            Path(__file__),
        ),
        "MAP-WRITE-OBSERVE0",
    )
