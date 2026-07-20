#!/usr/bin/env python3
"""Scoped FINALIZE0 function-draft typed-value split source guard.

This checker intentionally proves only the selected
``MirBuilder::finalize_function_draft`` lifecycle seam.  Module finalization
and loop lowering remain named legacy consumers until their own post-mutation
boundaries are selected; this tool never treats their continued use as a
successful conversion.
"""

from __future__ import annotations

import argparse
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
LOWERING = ROOT / "src/mir/builder/calls/lowering.rs"
DEFINITION = ROOT / "src/mir/builder/emission/value_lifecycle_definition.rs"
LEGACY = ROOT / "src/mir/builder/emission/value_lifecycle.rs"
MODULE = ROOT / "src/mir/builder/module_lifecycle.rs"
LOOP = ROOT / "src/mir/builder/control_flow/plan/lowerer/loop_lowering.rs"


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(f"[finalize0/verify-split-function-guard] {message}")


def source(path: Path) -> str:
    return path.read_text()


def function_body(text: str, marker: str) -> str:
    start = text.find(marker)
    require(start >= 0, f"missing function marker: {marker}")
    open_brace = text.find("{", start)
    require(open_brace >= 0, f"missing body brace: {marker}")

    depth = 0
    index = open_brace
    block_comment_depth = 0
    while index < len(text):
        char = text[index]
        next_char = text[index + 1] if index + 1 < len(text) else ""

        if block_comment_depth:
            if char == "/" and next_char == "*":
                block_comment_depth += 1
                index += 2
                continue
            if char == "*" and next_char == "/":
                block_comment_depth -= 1
                index += 2
                continue
            index += 1
            continue

        if char == "/" and next_char == "/":
            newline = text.find("\n", index + 2)
            index = len(text) if newline < 0 else newline + 1
            continue
        if char == "/" and next_char == "*":
            block_comment_depth = 1
            index += 2
            continue
        if char == '"':
            index = skip_quoted(text, index, '"')
            continue
        if char == "'" and not (
            next_char.isascii()
            and (next_char.isalpha() or next_char == "_")
            and (index + 2 >= len(text) or text[index + 2] != "'")
        ):
            index = skip_quoted(text, index, "'")
            continue
        if char == "r" and next_char in ('"', "#"):
            raw_end = skip_raw_string(text, index)
            if raw_end is not None:
                index = raw_end
                continue
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return text[open_brace + 1 : index]
        index += 1
    raise SystemExit(f"[finalize0/verify-split-function-guard] unterminated body: {marker}")


def skip_quoted(text: str, start: int, quote: str) -> int:
    index = start + 1
    while index < len(text):
        if text[index] == "\\":
            index += 2
            continue
        if text[index] == quote:
            return index + 1
        index += 1
    raise SystemExit("[finalize0/verify-split-function-guard] unterminated quoted literal")


def skip_raw_string(text: str, start: int) -> int | None:
    index = start + 1
    hashes = 0
    while index < len(text) and text[index] == "#":
        hashes += 1
        index += 1
    if index >= len(text) or text[index] != '"':
        return None
    terminator = '"' + ('#' * hashes)
    end = text.find(terminator, index + 1)
    require(end >= 0, "unterminated raw string")
    return end + len(terminator)


def ordered_positions(body: str, markers: list[str]) -> None:
    cursor = -1
    for marker in markers:
        position = body.find(marker, cursor + 1)
        require(position >= 0, f"missing or out-of-order selected-finalizer marker: {marker}")
        cursor = position


def validate_sources(
    lowering: str,
    definition: str,
    legacy: str,
    module: str,
    loop: str,
) -> None:
    finalize = function_body(
        lowering,
        "pub(in crate::mir::builder) fn finalize_function_draft",
    )

    require(
        definition.count("fn prepare_transient_stale_value_facts_v1(") == 1,
        "prepare definition count must be 1",
    )
    require(
        definition.count("fn verify_completed_draft_typed_value_definitions_v1(") == 1,
        "completed verifier definition count must be 1",
    )
    require(
        finalize.count("prepare_transient_stale_value_facts_v1(") == 1,
        "selected finalizer prepare consumer count must be 1",
    )
    require(
        finalize.count("verify_completed_draft_typed_value_definitions_v1(") == 1,
        "selected finalizer verifier consumer count must be 1",
    )
    require(
        finalize.count("prepared.commit(&mut self.function_state.type_ctx)") == 1,
        "selected finalizer prepared commit count must be 1",
    )
    ordered_positions(
        finalize,
        [
            "TypePropagationPipeline::run",
            "annotate_missing_result_types_from_calls_and_await",
            "prepare_transient_stale_value_facts_v1(",
            "prepared.commit(&mut self.function_state.type_ctx)",
            "f.metadata.value_types =",
            "f.metadata.value_origin_callers =",
            "verify_completed_draft_typed_value_definitions_v1(",
            "self.function_state.current_function.take()",
        ],
    )
    for forbidden in [
        "strict_or_dev_planner_required",
        "strict_enabled",
        "joinir_dev_enabled",
        "planner_required_enabled",
        "value_lifecycle::verify_typed_values_are_defined",
    ]:
        require(forbidden not in finalize, f"selected finalizer must not contain: {forbidden}")

    for lane in [
        "type_ctx.value_types.remove(&value)",
        "type_ctx.value_kinds.remove(&value)",
        "type_ctx.value_origin_newbox.remove(&value)",
    ]:
        require(definition.count(lane) == 1, f"prepared commit lane count must be 1: {lane}")

    legacy_marker = "value_lifecycle::verify_typed_values_are_defined("
    require(
        legacy.count("fn verify_typed_values_are_defined(") == 1,
        "legacy mixed-helper definition count must be 1",
    )
    require(module.count(legacy_marker) == 1, "module legacy consumer count must be 1")
    require(loop.count(legacy_marker) == 1, "loop legacy consumer count must be 1")
    require(lowering.count(legacy_marker) == 0, "selected finalizer legacy consumer count must be 0")


def run_drift_probes(
    lowering: str,
    definition: str,
    legacy: str,
    module: str,
    loop: str,
) -> None:
    probes = [
        (
            "selected commit removed",
            lowering.replace(
                "prepared.commit(&mut self.function_state.type_ctx)",
                "prepared.commit_was_removed(&mut self.function_state.type_ctx)",
                1,
            ),
            definition,
            legacy,
            module,
            loop,
        ),
        (
            "kind removal removed",
            lowering,
            definition.replace("type_ctx.value_kinds.remove(&value)", "// kind removal absent", 1),
            legacy,
            module,
            loop,
        ),
        (
            "module legacy partition removed",
            lowering,
            definition,
            legacy,
            module.replace("value_lifecycle::verify_typed_values_are_defined(", "legacy_call_removed(", 1),
            loop,
        ),
        (
            "build-mode gate added",
            lowering.replace(
                "// Void return追加（必要な場合）",
                "strict_enabled(); // injected drift\n        // Void return追加（必要な場合）",
                1,
            ),
            definition,
            legacy,
            module,
            loop,
        ),
    ]
    for label, *candidate in probes:
        try:
            validate_sources(*candidate)
        except SystemExit:
            continue
        raise SystemExit(
            f"[finalize0/verify-split-function-guard] drift probe was accepted: {label}"
        )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--drift-probes", action="store_true")
    args = parser.parse_args()
    lowering = source(LOWERING)
    definition = source(DEFINITION)
    legacy = source(LEGACY)
    module = source(MODULE)
    loop = source(LOOP)
    validate_sources(lowering, definition, legacy, module, loop)
    if args.drift_probes:
        run_drift_probes(lowering, definition, legacy, module, loop)

    print("output_contract=finalize0-verify-split0-function-draft-guard-v0")
    print("selected_function_finalizer_normalizer_consumers=1")
    print("selected_function_finalizer_verifier_consumers=1")
    print("legacy_mixed_helper_remaining_consumers=2")
    print("module_legacy_conversion_claim=0")
    print("loop_legacy_conversion_claim=0")
    print("mixed_helper_retirement_claim=0")
    print("all_build_gate_tokens_in_selected_finalizer=0")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
