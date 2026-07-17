#!/usr/bin/env python3
"""Validate the behavior-neutral TYPE-PUBLISH0-M0 producer/timing inventory."""

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
        if relative.startswith("src/mir/builder/phi_type_publication/"):
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
            "src/mir/builder/resolved_lowering/if_materialization.rs": 1,
        },
        "define_phi_final_with_type_hint(": {
            "src/mir/builder/phi.rs": 1,
            "src/mir/builder/emission/phi_lifecycle.rs": 2,
        },
        "define_phi_batch_prepend(": {
            "src/mir/builder/emission/phi_lifecycle.rs": 1,
            "src/mir/builder/control_flow/joinir/merge/loop_header_phi_builder.rs": 1,
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
            fail(f"M0 production consumer count for {symbol!r} must be 0, got {count}")
    for label, text in (
        ("raw", builder_emit),
        ("lifecycle", lifecycle),
        ("origin", origin),
    ):
        if "phi_type_publication" in code_only(text):
            fail(f"M0 connected the decision owner to {label}")

    # Raw currently rematerializes, writes combined type+origin, then appends.
    require_order(
        builder_emit,
        [
            "phi_input_materializer::for_pred",
            "origin::phi::propagate_phi_meta",
            "block.add_instruction_with_span(",
        ],
        "raw pre-I0 timing",
    )
    require(origin, "value_types.insert(dst, ct)", "raw type publication")
    require(origin, "value_origin_newbox.insert(dst, cc)", "raw origin publication")

    # Complete and batch rematerialize logical rows. Patch is sorted identity today.
    require_order(
        lifecycle,
        [
            "fn define_phi_final_with_type_hint(",
            "phi_input_materializer::for_pred",
            "insert_phi_at_head_spanned_with_type_hint(",
        ],
        "complete lifecycle",
    )
    require_order(
        lifecycle,
        [
            "fn define_phi_batch_prepend(",
            "phi_input_materializer::for_pred",
            "insert_phi_batch_prepend_spanned_with_type_hint(",
        ],
        "batch lifecycle",
    )
    patch_body = lifecycle.rsplit("pub(in crate::mir::builder) fn patch_phi_inputs(", 1)[1].split(
        "fn rollback_provisional_phi(", 1
    )[0]
    require(patch_body, "inputs.sort_by_key", "patch normalization")
    require(patch_body, ".update_phi_instruction", "patch mutation")
    if "phi_input_materializer::for_pred" in patch_body:
        fail("patch unexpectedly gained rematerialization during M0")

    # One existing post-patch Unknown writer is the explicit I0 retirement seam.
    binding_patch = binding.split("fn patch_phi_inputs(", 1)[1].split(
        "fn verify_phi_input(", 1
    )[0]
    require_order(
        binding_patch,
        [".patch_phi_inputs(", ".value_types", ".insert(token.dst(), MirType::Unknown)"],
        "Binding SSA post-patch Unknown writer",
    )
    if binding_patch.count("MirType::Unknown") != 1:
        fail("Binding SSA post-patch Unknown writer count must remain exactly 1 in M0")

    # The selected structural timing is Phi(param0,param0) -> LocalSSA Copy.
    require_order(
        local_ssa,
        [
            "builder.emit_instruction(MirInstruction::Copy { dst: loc, src: v })",
            "builder.type_ctx.value_types.get(&v).cloned()",
            "builder.type_ctx.value_types.insert(loc, t)",
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

    # Report the fixed M0 matrix without creating a second checked-in authority.
    report = {
        "schema": "phi-type-publication-m0-inventory-v1",
        "authorized_entries": [row[0] for row in authorized],
        "explicit_nonconsumers": [row[0] for row in nonconsumers],
        "caller_families": {
            needle: counts_by_path(root, production, needle)
            for needle in sorted(expected_calls)
        },
        "production_decision_consumers": 0,
        "raw_direct_phi_emit_call_sites": occurrence_count(
            production, "emit_instruction(MirInstruction::Phi"
        ),
        "post_patch_unknown_writers": 1,
        "raw_pre_append_combined_writers": 1,
        "late_copy_publisher": "user_box_route_fixpoint_copy_phi_fixed_point",
        "patch_physical_carrier": "sorted_logical_identity",
    }
    if report["raw_direct_phi_emit_call_sites"] != 0:
        fail("raw direct Phi emit call sites must remain 0 in M0")
    artifact = root / "target/checks/phi-type-publication-inventory/report.json"
    artifact.parent.mkdir(parents=True, exist_ok=True)
    artifact.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(
        "[phi-type-publication-inventory] ok "
        "authorized=4 consumers=0 post_patch_unknown=1 "
        "late_publisher=user_box_route_fixpoint_copy_phi_fixed_point"
    )


if __name__ == "__main__":
    main()
