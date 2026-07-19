#!/usr/bin/env python3
"""Validate the TYPE-PUBLISH0-I0 publication and PRED0 readiness boundaries."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path


def fail(message: str) -> None:
    raise SystemExit(f"[phi-type-publication-inventory] {message}")


def read(root: Path, relative: str) -> str:
    path = root / relative
    if not path.is_file():
        fail(f"missing required file: {relative}")
    return path.read_text(encoding="utf-8")


def code_only(text: str) -> str:
    text = re.sub(r"/\*.*?\*/", "", text, flags=re.S)
    return re.sub(r"//.*", "", text)


def require(text: str, snippet: str, label: str) -> None:
    if snippet not in text:
        fail(f"missing {label}: {snippet!r}")


def require_pattern(text: str, pattern: str, label: str) -> None:
    if re.search(pattern, text, flags=re.S) is None:
        fail(f"missing {label}: {pattern!r}")


def require_order(text: str, snippets: list[str], label: str) -> None:
    cursor = -1
    for snippet in snippets:
        position = text.find(snippet, cursor + 1)
        if position < 0:
            fail(f"missing {label} order anchor: {snippet!r}")
        if position <= cursor:
            fail(f"drifted {label} order at: {snippet!r}")
        cursor = position


def production_rust(root: Path) -> list[Path]:
    paths = []
    for path in (root / "src").rglob("*.rs"):
        relative = path.relative_to(root).as_posix()
        if "/tests/" in relative or relative.endswith("_tests.rs") or path.name == "tests.rs":
            continue
        if relative.startswith(
            (
                "src/mir/builder/phi_type_publication/",
                "src/mir/builder/phi_completion/",
            )
        ):
            continue
        paths.append(path)
    return paths


def occurrence_count(paths: list[Path], needle: str) -> int:
    return sum(code_only(path.read_text(encoding="utf-8")).count(needle) for path in paths)


def counts_by_path(root: Path, paths: list[Path], needle: str) -> dict[str, int]:
    counts = {}
    for path in paths:
        count = code_only(path.read_text(encoding="utf-8")).count(needle)
        if count:
            counts[path.relative_to(root).as_posix()] = count
    return counts


def main() -> None:
    if len(sys.argv) != 2:
        fail("usage: phi_type_publication_inventory.py ROOT")
    root = Path(sys.argv[1]).resolve()

    builder_emit = read(root, "src/mir/builder/builder_emit.rs")
    lifecycle = read(root, "src/mir/builder/emission/phi_lifecycle.rs")
    batch = read(
        root,
        "src/mir/builder/emission/phi_lifecycle/batch_type_publication.rs",
    )
    phi_completion = read(root, "src/mir/builder/phi_completion/mod.rs")
    resolved_if_connection = read(
        root, "src/mir/builder/phi_completion/resolved_if_connection.rs"
    )
    resolved_if = read(root, "src/mir/builder/resolved_lowering/if_materialization.rs")
    resolved_if_bridge = read(
        root, "src/mir/builder/resolved_lowering/if_cfg_ready_bridge.rs"
    )
    origin = read(root, "src/mir/builder/origin/phi.rs")
    binding = read(root, "src/mir/builder/ssa/binding/mir_adapter.rs")
    local_ssa = read(root, "src/mir/builder/ssa/local.rs")
    pipeline = read(root, "src/mir/type_propagation/pipeline.rs")
    route_publish = read(root, "src/mir/user_box_method_route_plan/value_type_publish.rs")
    route_convergence = read(root, "src/mir/user_box_method_route_plan/convergence.rs")
    function_repair = read(
        root, "src/mir/builder/ssa/phi_input_materializer/function_repair.rs"
    )
    remapper = read(root, "src/mir/builder/joinir_id_remapper.rs")

    authorized = [
        ("raw_emit", builder_emit, "fn emit_instruction("),
        ("complete_final", lifecycle, "fn define_phi_final_with_type_hint("),
        ("patch", lifecycle, "fn patch_phi_inputs("),
        ("batch", lifecycle, "fn define_phi_batch_prepend("),
    ]
    for entry_id, text, anchor in authorized:
        require(text, anchor, f"authorized entry {entry_id}")

    nonconsumers = [
        ("provisional_define", lifecycle, "fn define_provisional_phi("),
        (
            "function_level_final",
            lifecycle,
            "fn define_phi_final_fn_with_type_hint_and_tag(",
        ),
        ("thin_final", lifecycle, "fn define_phi_final("),
        ("transaction_facade", lifecycle, "struct PhiTxn"),
        ("function_repair", function_repair, "fn materialize_all_phi_inputs("),
        ("joinir_id_remap", remapper, "fn remap_instruction("),
    ]
    for entry_id, text, anchor in nonconsumers:
        require(text, anchor, f"explicit nonconsumer {entry_id}")

    production = production_rust(root)
    expected_calls = {
        "define_phi_final(": {
            "src/mir/builder/phi.rs": 2,
            "src/mir/builder/loop_api_impl.rs": 1,
            "src/mir/builder/if_form.rs": 1,
            "src/mir/builder/exprs_peek.rs": 1,
            "src/mir/builder/emission/phi_lifecycle.rs": 1,
        },
        "define_phi_final_with_type_hint(": {
            "src/mir/builder/phi.rs": 1,
            "src/mir/builder/emission/phi_lifecycle.rs": 2,
        },
        "define_phi_batch_prepend(": {
            "src/mir/builder/emission/phi_lifecycle.rs": 2,
            "src/mir/builder/control_flow/joinir/merge/loop_header_phi_builder.rs": 1,
            "src/mir/builder/emission/phi_lifecycle/batch_type_publication.rs": 1,
        },
        "define_provisional_phi(": {
            "src/mir/builder/emission/phi_lifecycle.rs": 3,
            "src/mir/builder/control_flow/plan/lowerer/loop_preparation.rs": 1,
            "src/mir/builder/control_flow/joinir/merge/exit_phi_builder.rs": 2,
            "src/mir/builder/ssa/binding/adapter.rs": 1,
            "src/mir/builder/ssa/binding/mod.rs": 2,
            "src/mir/builder/ssa/binding/mir_adapter.rs": 2,
        },
        "patch_phi_inputs(": {
            "src/mir/builder/emission/phi_lifecycle.rs": 3,
            "src/mir/builder/control_flow/plan/lowerer/phi_processing.rs": 1,
            "src/mir/builder/control_flow/joinir/merge/exit_phi_builder.rs": 2,
            "src/mir/builder/ssa/binding/adapter.rs": 1,
            "src/mir/builder/ssa/binding/mod.rs": 2,
            "src/mir/builder/ssa/binding/mir_adapter.rs": 2,
        },
        "define_phi_final_fn_with_type_hint_and_tag(": {
            "src/mir/join_ir_vm_bridge/joinir_block_converter/handlers.rs": 1,
            "src/mir/join_ir_vm_bridge/handlers/conditional_method_call.rs": 1,
            "src/mir/builder/emission/phi_lifecycle.rs": 2,
        },
        "define_phi_final_fn(": {
            "src/mir/builder/emission/phi_lifecycle.rs": 1,
            "src/mir/builder/control_flow/edgecfg/api/emit.rs": 1,
        },
    }
    for needle, expected in expected_calls.items():
        actual = counts_by_path(root, production, needle)
        if actual != expected:
            fail(f"caller inventory drift for {needle!r}: expected={expected} actual={actual}")

    for symbol in (
        "PhiTransientTypeDecisionV1::prepare",
        "commit_prepared_phi_type(",
        "PreparedPhiTypePublicationV1",
    ):
        count = occurrence_count(production, symbol)
        if count != 0:
            fail(f"direct production consumer count for {symbol!r} must be 0, got {count}")
    if occurrence_count(production, "phi_type_publication::prepare_for_builder(") != 0:
        fail("I0 must retire direct type-publication preparation consumers")
    for symbol in (
        "phi_completion::prepare_for_builder(",
        "phi_completion::commit_for_builder(",
    ):
        count = occurrence_count(production, symbol)
        if count != 4:
            fail(f"I0 completion consumer count for {symbol!r} must be 4, got {count}")

    # PRED0 keeps route-owned CFG readiness out of every generic lifecycle.
    # CFGREADY0 activates exactly the canonical resolved-If sidecar; raw rows
    # never become a generic expected-predecessor constructor.
    if code_only(phi_completion).count("prepare_cfg_ready(") != 1:
        fail("PRED0 private CFG-ready preparation must have exactly one definition")
    require(
        phi_completion,
        "pub(super) fn prepare_cfg_ready(",
        "CFG-ready preparation visibility",
    )
    require(
        phi_completion,
        "    fn verify(\n        expected_predecessors:",
        "private CFG-ready row constructor",
    )
    if code_only(resolved_if_connection).count("CfgReadyPhiRowsV1::verify(") != 1:
        fail("CFGREADY0 must have exactly one resolved-If CFG row constructor")
    if code_only(resolved_if_connection).count(".prepare_cfg_ready(") != 1:
        fail("CFGREADY0 must have exactly one resolved-If CFG-ready preparation")
    for forbidden in (
        "compute_predecessors",
        "compute_reachable_blocks",
        "compute_dominators",
        "insert_phi",
        "value_types.insert",
        "value_origin_newbox.insert",
    ):
        if forbidden in code_only(resolved_if_connection):
            fail(f"resolved-If CFG sidecar unexpectedly owns {forbidden!r}")

    require_order(
        resolved_if,
        [
            "fn define_join_phis(",
            "VerifiedResolvedIfCfgReadyJoinRowsV1::verify(",
            "phi_completion::prepare_for_resolved_if(",
            "phi_lifecycle::define_final_from_prepared_completion(",
        ],
        "resolved-If CFGREADY0 timing",
    )
    resolved_if_join = resolved_if.split("pub(super) fn define_join_phis(", 1)[1].split(
        "impl DefinedIfJoinSetV1", 1
    )[0]
    for forbidden in (
        "phi_lifecycle::define_phi_final(",
        "insert_phi",
        "MirInstruction::Phi",
        "value_types.insert",
        "value_origin_newbox.insert",
        "compute_predecessors",
    ):
        if forbidden in code_only(resolved_if_join):
            fail(f"resolved-If join path unexpectedly owns {forbidden!r}")
    for forbidden in (
        "phi_completion",
        "phi_lifecycle",
        "insert_phi",
        "value_types",
        "value_origin_newbox",
        "compute_predecessors",
    ):
        if forbidden in code_only(resolved_if_bridge):
            fail(f"resolved-If CFG witness unexpectedly owns {forbidden!r}")

    # The shared final physical continuation is not another completion decision
    # owner. It may be reached only by the generic final facade and the one
    # selected resolved-If route; origin publication remains raw-only.
    if occurrence_count(production, "define_final_from_prepared_completion(") != 3:
        fail("shared final-PHI continuation must have one definition and two callers")
    if code_only(resolved_if_connection).count("PhiDraftV1::new(") != 1:
        fail("resolved-If sidecar must construct exactly one CFG-ready draft")
    if code_only(read(root, "src/mir/builder/phi_completion/connection.rs")).count(
        "PhiDraftV1::new("
    ) != 1:
        fail("generic connection must construct exactly one input-only draft")
    if occurrence_count(production, "origin::phi::commit_unanimous_origin(") != 1:
        fail("raw emit must remain the sole builder-session PHI origin committer")

    # Raw prepares from logical inputs, rematerializes, appends, then commits
    # type and the independently prepared legacy origin fact.
    require_order(
        builder_emit,
        [
            "phi_completion::prepare_for_builder",
            "phi_input_materializer::for_pred",
            "origin::phi::prepare_unanimous_origin",
            "block.add_instruction_with_span(",
            "phi_completion::commit_for_builder",
            "origin::phi::commit_unanimous_origin",
        ],
        "raw I0 timing",
    )
    if "value_types" in code_only(origin):
        fail("raw origin owner still writes type facts")
    require_pattern(
        code_only(origin),
        r"value_origin_newbox\s*\.insert\(\s*dst,\s*origin\s*\)",
        "raw origin publication",
    )

    # Complete and batch decide from logical rows before rematerialization and
    # commit only after successful PHI mutation. Patch remains sorted identity.
    require_order(
        lifecycle,
        [
            "fn define_phi_final_with_type_hint(",
            "phi_completion::prepare_for_builder",
            "phi_input_materializer::for_pred",
            "insert_phi_at_head_spanned_with_type_hint(",
            "phi_completion::commit_for_builder",
        ],
        "complete lifecycle",
    )
    require_order(
        batch,
        [
            "fn define_phi_batch_prepend(",
            "preflight(builder, block, &items, tag)",
            "phi_completion::prepare_for_builder",
            ".clone()",
            "phi_input_materializer::for_pred",
            "insert_phi_batch_prepend_spanned_with_type_hint(",
            "= candidate",
            "phi_completion::commit_for_builder",
        ],
        "atomic batch lifecycle",
    )
    patch_body = lifecycle.rsplit("pub(in crate::mir::builder) fn patch_phi_inputs(", 1)[1].split(
        "fn rollback_provisional_phi(", 1
    )[0]
    require(patch_body, "inputs.sort_by_key", "patch normalization")
    require_order(
        patch_body,
        [
            "phi_completion::prepare_for_builder",
            ".update_phi_instruction",
            "phi_completion::commit_for_builder",
        ],
        "patch lifecycle",
    )
    require(patch_body, ".update_phi_instruction", "patch mutation")
    if "phi_input_materializer::for_pred" in patch_body:
        fail("patch unexpectedly gained rematerialization during I0")
    if "compute_predecessors" in patch_body:
        fail("generic patch must not acquire CFG predecessor authority during PRED0")
    if "compute_predecessors" in code_only(batch):
        fail("generic batch must not acquire CFG predecessor authority during PRED0")

    # I0 retires the post-patch Unknown overwrite; the provisional seed remains
    # the only Binding SSA Unknown publication before completion.
    binding_patch = binding.split("fn patch_phi_inputs(", 1)[1].split(
        "fn verify_phi_input(", 1
    )[0]
    if "MirType::Unknown" in binding_patch:
        fail("Binding SSA post-patch Unknown writer must be retired in I0")

    # The selected structural timing is Phi(param0,param0) -> LocalSSA Copy.
    require_order(
        local_ssa,
        [
            "builder.emit_instruction(MirInstruction::Copy { dst: loc, src: v })",
            "builder.function_state.type_ctx.value_types.get(&v).cloned()",
            "builder.function_state.type_ctx.value_types.insert(loc, t)",
        ],
        "LocalSSA success publication",
    )
    require_order(
        pipeline,
        [
            "Self::step1_copy_propagation",
            "Self::step2_binop_repropagation",
            "Self::step3_copy_propagation",
            "Self::step4_phi_type_inference",
        ],
        "final type pipeline",
    )
    require(
        route_publish,
        "fn propagate_user_box_box_value_types(",
        "exact late Copy/Phi publisher",
    )
    require(
        route_convergence,
        "propagate_user_box_box_value_types(module)",
        "late publisher convergence caller",
    )
    require(
        route_publish,
        "type_publish0_m0_identifies_route_fixpoint_as_late_phi_copy_publisher",
        "late publisher timing test",
    )

    # Report the fixed I0 matrix without creating a second checked-in authority.
    report = {
        "schema": "phi-type-publication-pred0-inventory-v1",
        "authorized_entries": [row[0] for row in authorized],
        "explicit_nonconsumers": [row[0] for row in nonconsumers],
        "caller_families": {
            needle: counts_by_path(root, production, needle)
            for needle in sorted(expected_calls)
        },
        "production_decision_consumers": 4,
        "raw_direct_phi_emit_call_sites": occurrence_count(
            production, "emit_instruction(MirInstruction::Phi"
        ),
        "post_patch_unknown_writers": 0,
        "raw_pre_append_combined_writers": 0,
        "raw_success_committed_origin_writers": 1,
        "late_copy_publisher": "user_box_route_fixpoint_copy_phi_fixed_point",
        "patch_physical_carrier": "sorted_logical_identity",
        "cfg_ready_preparation_definitions": 1,
        "cfg_ready_production_consumers": 1,
        "cfg_ready_resolved_if_consumers": 1,
        "shared_final_physical_continuation_occurrences": 3,
        "raw_origin_committers": 1,
        "function_level_phi_scope": "excluded_no_builder_transient_facts",
        "generic_patch_cfg_predecessor_reads": 0,
        "generic_batch_cfg_predecessor_reads": 0,
    }
    if report["raw_direct_phi_emit_call_sites"] != 0:
        fail("raw direct Phi emit call sites must remain 0 in I0")
    artifact = root / "target/checks/phi-type-publication-inventory/report.json"
    artifact.parent.mkdir(parents=True, exist_ok=True)
    artifact.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(
        "[phi-type-publication-inventory] ok "
        "authorized=4 consumers=4 cfg_ready_consumers=1 "
        "late_publisher=user_box_route_fixpoint_copy_phi_fixed_point"
    )


if __name__ == "__main__":
    main()
