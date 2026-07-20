#!/usr/bin/env python3
"""Freeze the approved FACT0-P1 semantic partition before PHI0 migration."""

from __future__ import annotations

import hashlib
import json
import sys
from collections import Counter
from pathlib import Path

from mirbuilder_type_fact_producer_inventory import (
    code_only,
    fail,
    load_fixture,
    read,
    require_anchor,
    strip_cfg_test_modules,
    writer_counts,
)


EXPECTED_COUNTS = {
    "writer_paths": 47,
    "writer_occurrences": 99,
    "partitions": 47,
    "slices": 58,
    "profiles": 38,
    "profile_linked_occurrences": 101,
}
EXPECTED_STATUS_COUNTS = {
    "scoped_cutover": 1,
    "candidate": 6,
    "legacy": 14,
    "unsafe": 9,
    "retire": 8,
}
EXPECTED_PROFILE_FREEZE = {
    "receiver_param0_rcv0": ("parameter", "scoped_cutover", "RCV0", 1, 1),
    "explicit_static_legacy": ("parameter", "legacy", "PARAMETER-UNKNOWN0-D0", 1, 1),
    "phi_carrier_precompletion_unsafe": ("phi", "unsafe", "PHI0-CUT0", 3, 1),
    "phi_provisional_unknown_legacy": ("phi", "retire", "PHI0-CUT0", 1, 1),
    "phi_rollback_teardown": ("phi", "retire", "PHI0-CUT0", 1, 1),
    "phi_postcompletion_bool_legacy": ("phi", "legacy", "PHI0-CUT0", 1, 1),
    "finalization_stale_cleanup": ("finalization", "retire", "FINALIZE0-CUT0", 1, 1),
    "finalization_result_repair": ("finalization", "retire", "FINALIZE0-CUT0", 2, 1),
    "session_teardown_nonpublication": ("session", "retire", "FSESSION0-CUT0", 1, 1),
    "plan_preemit_legacy": ("plan", "legacy", "PLAN0", 14, 3),
    "plan_selected_call": ("call", "unsafe", "FACT0-TX0-D0", 1, 1),
    "signature_annotation_mixed_legacy": ("call", "legacy", "CALL-ANNOTATION0-D0", 1, 1),
    "compat_name_heuristic": ("compatibility", "retire", "RAWADAPT0", 6, 3),
    "copy_exact": ("copy", "candidate", "FACT0-I1-COPY0", 1, 1),
    "copy_origin_legacy": ("copy", "legacy", "ORIGIN0-D0", 1, 1),
    "simple_exact": ("expression", "candidate", "FACT0-I1-EXACT0-D0", 10, 5),
    "literal_postemit_exact": ("expression", "candidate", "FACT0-I1-EXACT0-D0", 3, 3),
    "operator_mode_legacy": ("expression", "legacy", "OPERATOR0-D0", 15, 2),
    "newbox_lifecycle": ("allocation", "legacy", "NEWBOX-LIFECYCLE0-D0", 7, 6),
    "extern_return_table_legacy": ("call", "legacy", "EXTERN-CALL0-D0", 2, 2),
    "call_legacy": ("call", "legacy", "RAWADAPT0", 3, 2),
    "field_collection_unsafe": ("field", "unsafe", "FACT0-I1-FIELDGET0", 2, 2),
    "fastmem_legacy": ("fastmem", "legacy", "FASTMEM0-D0", 1, 1),
    "fastmem_field_fallback_unsafe": ("field", "unsafe", "FASTMEM0-D0", 1, 1),
    "static_data_load_exact": ("field", "candidate", "FACT0-I1-EXACT0-D0", 1, 1),
    "array_element_result_candidate": ("field", "candidate", "ARRAY-FACT0-D0", 1, 1),
    "array_receiver_chain_backfill_unsafe": ("field", "unsafe", "ARRAY-FACT0-D0", 1, 1),
    "record_field_late_backfill_unsafe": ("field", "unsafe", "RECORD-FACT0-D0", 1, 1),
    "origin_heuristic": ("origin", "retire", "COMPCTX0", 1, 1),
    "resolved_exact": ("resolved lowering", "candidate", "FACT0-I1-RESOLVED0-D0", 4, 4),
    "resolved_representation_legacy": (
        "resolved lowering",
        "legacy",
        "FACT0-I1-RESOLVED0-D0",
        1,
        1,
    ),
    "metadata_copy_propagation_legacy": ("metadata", "legacy", "METADATA0-CUT0", 1, 1),
    "metadata_override_unsafe": ("metadata", "unsafe", "METADATA0-CUT0", 1, 1),
    "async_legacy": ("async", "legacy", "RAWADAPT0", 2, 1),
    "allocation_preinstruction_unsafe": ("allocation", "unsafe", "FACT0-TX0-D0", 1, 1),
    "generic_type_override_unsafe": ("compatibility", "unsafe", "FACT0-TX0-D0", 1, 1),
    "weak_ref_legacy": ("weak reference", "retire", "RAWADAPT0", 1, 1),
    "enum_match_mixed": ("enum match", "legacy", "ENUM-REP0-D0", 4, 1),
}
EXPECTED_SHARED_PROFILE_SETS = {
    ("compat_name_heuristic", "signature_annotation_mixed_legacy"),
    ("explicit_static_legacy", "receiver_param0_rcv0"),
}
EXPECTED_PARTITION_DIGEST = "8f85e1ee5db91b5b6f58f5a6d69ee37382722ac8e8e41d8153be245e45f80cde"

# The P1 fixture remains the immutable pre-cutover census. These are the only
# approved current source-level replacements landed by later independent rows.
# A new FACT0 row must extend this map deliberately; it may not rewrite the
# historical partition to make a direct writer disappearance look invisible.
ACTIVE_CUTOVER_WRITER_REPLACEMENTS = {
    "src/mir/builder/ssa/local.rs": None,
    "src/mir/builder/ssa/local/copy_type.rs": 1,
    "src/mir/builder/ssa/local/post_success.rs": 3,
    "src/mir/builder/emission/constant.rs": None,
    "src/mir/builder/emission/constant_type.rs": 1,
    "src/mir/builder/indexing.rs": None,
    "src/mir/builder/indexing/static_load_type.rs": 1,
    "src/mir/builder/exprs_check.rs": None,
    "src/mir/builder/exprs_check/select_type.rs": 1,
    "src/mir/builder/builder_build.rs": 3,
    "src/mir/builder/resolved_lowering/lowerer.rs": 1,
    "src/mir/builder/ops/unary.rs": 5,
    "src/mir/builder/resolved_lowering/trivial_ssa/operation.rs": None,
    "src/mir/builder/resolved_lowering/trivial_ssa/operation_type.rs": 1,
    "src/mir/builder/resolved_lowering/trivial_ssa/direct_call.rs": None,
    "src/mir/builder/resolved_lowering/trivial_ssa/direct_call_type.rs": 1,
    "src/mir/builder/emission/compare.rs": None,
    "src/mir/builder/emission/compare_type.rs": 1,
    "src/mir/builder/fields/post_success.rs": 1,
}


def partition_projection_v1(fixture: dict[str, object]) -> dict[str, object]:
    profiles = fixture.get("partition_profiles")
    partitions = fixture.get("writer_partitions")
    if not isinstance(profiles, dict) or not isinstance(partitions, list):
        fail("P1-G0 fixture lacks profiles or partitions")
    projection_profiles = {
        profile_id: {
            field: profile[field]
            for field in ("family", "status", "retirement_prerequisite")
        }
        for profile_id, profile in sorted(profiles.items())
        if isinstance(profile, dict)
    }
    projection_partitions = [
        {
            "source_file": partition["source_file"],
            "slices": [
                {
                    "first_ordinal": slice_row["first_ordinal"],
                    "last_ordinal": slice_row["last_ordinal"],
                    "producer_profiles": sorted(slice_row["producer_profiles"]),
                }
                for slice_row in sorted(
                    partition["slices"],
                    key=lambda row: (row["first_ordinal"], row["last_ordinal"]),
                )
            ],
        }
        for partition in sorted(partitions, key=lambda row: row["source_file"])
        if isinstance(partition, dict)
    ]
    return {"profiles": projection_profiles, "partitions": projection_partitions}


def partition_digest_v1(fixture: dict[str, object]) -> str:
    payload = json.dumps(
        partition_projection_v1(fixture),
        sort_keys=True,
        separators=(",", ":"),
    ).encode("utf-8")
    return hashlib.sha256(payload).hexdigest()


def validate_p1_g0_profile_freeze_v1(fixture: dict[str, object]) -> None:
    if fixture.get("schema_version") != 2:
        fail("P1-G0 requires fixture schema_version=2")
    inventory = fixture.get("write_inventory")
    profiles = fixture.get("partition_profiles")
    partitions = fixture.get("writer_partitions")
    if not isinstance(inventory, dict) or not isinstance(profiles, dict) or not isinstance(partitions, list):
        fail("P1-G0 fixture shape is invalid")

    occurrences = Counter()
    slices = Counter()
    shared_sets = set()
    for partition in partitions:
        if not isinstance(partition, dict):
            fail("P1-G0 partition entry is invalid")
        for slice_row in partition["slices"]:
            if not isinstance(slice_row, dict):
                fail("P1-G0 slice entry is invalid")
            profile_ids = slice_row["producer_profiles"]
            width = slice_row["last_ordinal"] - slice_row["first_ordinal"] + 1
            for profile_id in profile_ids:
                occurrences[profile_id] += width
                slices[profile_id] += 1
            if len(profile_ids) > 1:
                shared_sets.add(tuple(sorted(profile_ids)))

    actual_counts = {
        "writer_paths": len(inventory),
        "writer_occurrences": sum(inventory.values()),
        "partitions": len(partitions),
        "slices": sum(slices.values()) - len(shared_sets),
        "profiles": len(profiles),
        "profile_linked_occurrences": sum(occurrences.values()),
    }
    if actual_counts != EXPECTED_COUNTS:
        fail(f"P1-G0 count drift: expected={EXPECTED_COUNTS} actual={actual_counts}")

    actual_freeze = {
        profile_id: (
            profile.get("family"),
            profile.get("status"),
            profile.get("retirement_prerequisite"),
            occurrences[profile_id],
            slices[profile_id],
        )
        for profile_id, profile in profiles.items()
        if isinstance(profile, dict)
    }
    if actual_freeze != EXPECTED_PROFILE_FREEZE:
        fail("P1-G0 profile/prerequisite freeze drift")
    if dict(Counter(profile[1] for profile in actual_freeze.values())) != EXPECTED_STATUS_COUNTS:
        fail("P1-G0 profile status count drift")
    if shared_sets != EXPECTED_SHARED_PROFILE_SETS:
        fail("P1-G0 shared-profile set drift")
    if partition_digest_v1(fixture) != EXPECTED_PARTITION_DIGEST:
        fail("P1-G0 partition projection digest drift")
    if "FACT0-G0" in (profile[2] for profile in actual_freeze.values()):
        fail("P1-G0 cannot name FACT0-G0 as a retirement prerequisite")


def validate_active_cutover_writer_inventory_v1(root: Path, fixture: dict[str, object]) -> None:
    baseline = fixture.get("write_inventory")
    if not isinstance(baseline, dict):
        fail("active FACT0 cutover requires write_inventory")
    expected = dict(baseline)
    for path, count in ACTIVE_CUTOVER_WRITER_REPLACEMENTS.items():
        if count is None:
            expected.pop(path, None)
        else:
            expected[path] = count
    actual = writer_counts(root)
    if actual != expected:
        fail(
            "active FACT0 direct-writer inventory drift: "
            f"expected={expected} actual={actual}"
        )


def validate_const0_authority_v1(root: Path) -> None:
    constant = code_only(read(root / "src/mir/builder/emission/constant.rs"))
    owner = code_only(read(root / "src/mir/builder/emission/constant_type.rs"))

    if "value_types.insert" in constant or "value_origin_newbox" in constant:
        fail("CONST0 direct type/origin writer survived in constant.rs")
    if constant.count("PreparedCanonicalConstTypeV1::prepare") != 1:
        fail("CONST0 requires one shared preparation consumer")
    if constant.count("prepared.commit(") != 1:
        fail("CONST0 requires one shared post-emission commit consumer")
    if constant.count("emit_exact_const(") != 7:
        fail("CONST0 requires one helper plus six canonical public delegates")
    if constant.count("string_literals.insert") != 1:
        fail("CONST0 String companion publication drift")
    if constant.find("prepared.commit(") > constant.find("string_literals.insert"):
        fail("CONST0 String companion must follow the shared type commit")
    if owner.count("TypeFactDecisionV1::prepare") != 1 or owner.count("type_ctx.set_type") != 1:
        fail("CONST0 decision/commit owner drift")
    for path in (
        root / "src/mir/builder/emission/constant.rs",
        root / "src/mir/builder/emission/constant_type.rs",
        Path(__file__),
    ):
        if len(read(path).splitlines()) >= 800:
            fail(f"CONST0 source/check file reached 800 lines: {path}")


def validate_staticload0_authority_v1(root: Path) -> None:
    indexing = code_only(read(root / "src/mir/builder/indexing.rs"))
    owner = code_only(read(root / "src/mir/builder/indexing/static_load_type.rs"))

    if "metadata.value_types.insert" in indexing or "type_ctx.value_types.insert" in indexing:
        fail("STATICLOAD0 direct type writer survived in indexing.rs")
    if indexing.count("PreparedStaticU16LoadTypeV1::prepare") != 1:
        fail("STATICLOAD0 requires one pre-emission preparation consumer")
    if indexing.count("prepared.commit(") != 1:
        fail("STATICLOAD0 requires one post-emission transient commit consumer")
    if indexing.find("prepared.commit(") < indexing.find("StaticDataLoad {"):
        fail("STATICLOAD0 commit must follow StaticDataLoad emission")
    if owner.count("TypeFactDecisionV1::prepare") != 1 or owner.count("type_ctx.set_type") != 1:
        fail("STATICLOAD0 decision/commit owner drift")
    for path in (
        root / "src/mir/builder/indexing.rs",
        root / "src/mir/builder/indexing/static_load_type.rs",
        Path(__file__),
    ):
        if len(read(path).splitlines()) >= 800:
            fail(f"STATICLOAD0 source/check file reached 800 lines: {path}")


def validate_checkselect0_authority_v1(root: Path) -> None:
    check_expr = strip_cfg_test_modules(
        code_only(read(root / "src/mir/builder/exprs_check.rs"))
    )
    owner = code_only(read(root / "src/mir/builder/exprs_check/select_type.rs"))

    if "value_types.insert" in check_expr or "metadata.value_types" in check_expr:
        fail("CHECKSELECT0 direct type/metadata writer survived in exprs_check.rs")
    if check_expr.count("PreparedCheckSelectIntegerTypeV1::prepare") != 1:
        fail("CHECKSELECT0 requires one pre-emission preparation consumer")
    if check_expr.count("prepared.commit(") != 1:
        fail("CHECKSELECT0 requires one post-emission transient commit consumer")
    if check_expr.find("prepared.commit(") < check_expr.find("MirInstruction::Select {"):
        fail("CHECKSELECT0 commit must follow Select emission")
    if owner.count("TypeFactDecisionV1::prepare") != 1 or owner.count("type_ctx.set_type") != 1:
        fail("CHECKSELECT0 decision/commit owner drift")
    for path in (
        root / "src/mir/builder/exprs_check.rs",
        root / "src/mir/builder/exprs_check/select_type.rs",
        Path(__file__),
    ):
        if len(read(path).splitlines()) >= 800:
            fail(f"CHECKSELECT0 source/check file reached 800 lines: {path}")


def validate_literal_postemit_retirement_v1(root: Path) -> None:
    literal_builder = read(root / "src/mir/builder/builder_build.rs")
    resolved_lowerer = read(root / "src/mir/builder/resolved_lowering/lowerer.rs")
    unary = read(root / "src/mir/builder/ops/unary.rs")

    literal_dispatch = literal_builder.split("pub(super) fn build_literal", 1)[1].split(
        "pub(in crate::mir::builder) fn emit_typed_integer_literal", 1
    )[0]
    resolved_literal = resolved_lowerer.split("fn lower_literal", 1)[1]
    folded_negative = unary.split('if operator == "-"', 1)[1].split("let operand_val", 1)[0]

    if "value_types.insert" in literal_dispatch:
        fail("LITERAL-POSTEMIT-RET0 literal dispatch direct type writer survived")
    if "value_types.insert" in resolved_literal:
        fail("LITERAL-POSTEMIT-RET0 resolved Null/Void direct type writer survived")
    if "value_types.insert" in folded_negative:
        fail("LITERAL-POSTEMIT-RET0 folded negative direct type writer survived")
    if literal_dispatch.count("emission::constant::emit_") != 6:
        fail("LITERAL-POSTEMIT-RET0 literal dispatch must retain six canonical Const delegates")
    if "emit_typed_integer_literal" not in literal_dispatch:
        fail("LITERAL-POSTEMIT-RET0 TypedInteger canonical delegate missing")
    if "build_literal(literal.clone())" not in resolved_literal:
        fail("LITERAL-POSTEMIT-RET0 resolved literal must delegate to canonical literal lowering")
    if "emission::constant::emit_integer(builder, negated)" not in folded_negative:
        fail("LITERAL-POSTEMIT-RET0 folded negative must delegate to canonical Const")
    for path in (
        root / "src/mir/builder/builder_build.rs",
        root / "src/mir/builder/resolved_lowering/lowerer.rs",
        root / "src/mir/builder/ops/unary.rs",
        Path(__file__),
    ):
        if len(read(path).splitlines()) >= 800:
            fail(f"LITERAL-POSTEMIT-RET0 source/check file reached 800 lines: {path}")


def validate_resolved_trivial_operation_authority_v1(root: Path) -> None:
    operation = code_only(
        read(root / "src/mir/builder/resolved_lowering/trivial_ssa/operation.rs")
    )
    owner = code_only(
        read(root / "src/mir/builder/resolved_lowering/trivial_ssa/operation_type.rs")
    )

    if "value_types.insert" in operation or "type_ctx.set_type" in operation:
        fail("RESOLVED-TRIVIAL-OP0 direct type writer survived in operation.rs")
    if operation.count("PreparedResolvedTrivialOperationTypeV1::prepare") != 1:
        fail("RESOLVED-TRIVIAL-OP0 requires one pre-emission preparation consumer")
    if operation.count("prepared.commit(") != 1:
        fail("RESOLVED-TRIVIAL-OP0 requires one post-emission commit consumer")
    if operation.find("prepared.commit(") < operation.find("builder.emit_instruction(instruction)?"):
        fail("RESOLVED-TRIVIAL-OP0 commit must follow BinOp/Compare emission")
    if owner.count("TypeFactDecisionV1::prepare") != 1 or owner.count("type_ctx.set_type") != 1:
        fail("RESOLVED-TRIVIAL-OP0 decision/commit owner drift")
    for path in (
        root / "src/mir/builder/resolved_lowering/trivial_ssa/operation.rs",
        root / "src/mir/builder/resolved_lowering/trivial_ssa/operation_type.rs",
        Path(__file__),
    ):
        if len(read(path).splitlines()) >= 800:
            fail(f"RESOLVED-TRIVIAL-OP0 source/check file reached 800 lines: {path}")


def validate_resolved_direct_call_authority_v1(root: Path) -> None:
    direct_call = code_only(
        read(root / "src/mir/builder/resolved_lowering/trivial_ssa/direct_call.rs")
    )
    owner = code_only(
        read(root / "src/mir/builder/resolved_lowering/trivial_ssa/direct_call_type.rs")
    )

    if "value_types.insert" in direct_call or "type_ctx.set_type" in direct_call:
        fail("RESOLVED-DIRECT-CALL0 direct type writer survived in direct_call.rs")
    if direct_call.count("PreparedResolvedDirectCallIntegerTypeV1::prepare") != 1:
        fail("RESOLVED-DIRECT-CALL0 requires one pre-emission preparation consumer")
    if direct_call.count("prepared.commit(") != 1:
        fail("RESOLVED-DIRECT-CALL0 requires one post-emission commit consumer")
    if direct_call.find("prepared.commit(") < direct_call.find(
        "builder.emit_instruction(instruction)?"
    ):
        fail("RESOLVED-DIRECT-CALL0 commit must follow Call emission")
    if owner.count("TypeFactDecisionV1::prepare") != 1 or owner.count("type_ctx.set_type") != 1:
        fail("RESOLVED-DIRECT-CALL0 decision/commit owner drift")
    for path in (
        root / "src/mir/builder/resolved_lowering/trivial_ssa/direct_call.rs",
        root / "src/mir/builder/resolved_lowering/trivial_ssa/direct_call_type.rs",
        Path(__file__),
    ):
        if len(read(path).splitlines()) >= 800:
            fail(f"RESOLVED-DIRECT-CALL0 source/check file reached 800 lines: {path}")


def validate_compareemit0_authority_v1(root: Path) -> None:
    compare = code_only(read(root / "src/mir/builder/emission/compare.rs"))
    owner = code_only(read(root / "src/mir/builder/emission/compare_type.rs"))

    if "value_types.insert" in compare or "type_ctx.set_type" in compare:
        fail("COMPAREEMIT0 direct Bool writer survived in compare.rs")
    if "cf_common::emit_compare_func" in compare:
        fail("COMPAREEMIT0 Builder helper must not use unit-return cf_common emission")
    if compare.count("require_existing_current_compare_block(") != 2:
        fail("COMPAREEMIT0 requires one strict receipt preflight definition and consumer")
    if compare.count("PreparedCanonicalCompareBoolTypeV1::prepare") != 1:
        fail("COMPAREEMIT0 requires one pre-emission Bool preparation consumer")
    if compare.count("prepared.commit(") != 1:
        fail("COMPAREEMIT0 requires one post-emission Bool commit consumer")
    if compare.find("prepared.commit(") < compare.find("b.emit_instruction(MirInstruction::Compare"):
        fail("COMPAREEMIT0 Bool commit must follow checked Compare emission")
    if owner.count("TypeFactDecisionV1::prepare") != 1 or owner.count("type_ctx.set_type") != 1:
        fail("COMPAREEMIT0 Bool decision/commit owner drift")
    for path in (
        root / "src/mir/builder/emission/compare.rs",
        root / "src/mir/builder/emission/compare_type.rs",
        Path(__file__),
    ):
        if len(read(path).splitlines()) >= 800:
            fail(f"COMPAREEMIT0 source/check file reached 800 lines: {path}")


def validate_call_receipt0_authority_v1(root: Path) -> None:
    emitter = strip_cfg_test_modules(
        code_only(read(root / "src/mir/builder/calls/unified_emitter.rs"))
    )
    receipt = strip_cfg_test_modules(
        code_only(read(root / "src/mir/builder/calls/unified_emitter/post_success.rs"))
    )

    canonical = emitter.split("fn emit_unified_call_impl", 1)[1].split(
        "pub fn emit_global_unified", 1
    )[0]
    if canonical.count("PreparedUnifiedCallPostSuccessV1::prepare") != 1:
        fail("CALL-RECEIPT0 requires one canonical payload preparation consumer")
    if canonical.count("prepared_post_success.commit_after_success(builder)") != 1:
        fail("CALL-RECEIPT0 requires one canonical post-success payload consumer")
    if canonical.find("prepared_post_success.commit_after_success(builder)") < canonical.find(
        "builder.emit_instruction(call_inst)?"
    ):
        fail("CALL-RECEIPT0 payload consumption must follow successful Call emission")
    for forbidden in (
        "annotate_call_result_from_func_name",
        "annotate_array_element_result",
        "annotate_map_get_result",
        "verify_after_call",
    ):
        if forbidden in canonical:
            fail(f"CALL-RECEIPT0 direct post-success effect survived in emitter: {forbidden}")

    if receipt.count("fn commit_after_success") != 1:
        fail("CALL-RECEIPT0 requires one post-success commit owner")
    for required in (
        "annotate_call_result_from_func_name",
        "annotate_array_element_result",
        "annotate_map_get_result",
        "verify_after_call",
    ):
        if receipt.count(required) != 1:
            fail(f"CALL-RECEIPT0 post-success owner drift: {required}")
    for path in (
        root / "src/mir/builder/calls/unified_emitter.rs",
        root / "src/mir/builder/calls/unified_emitter/post_success.rs",
        root / "src/mir/builder/calls/unified_emitter/temporal_witness_tests.rs",
        Path(__file__),
    ):
        if len(read(path).splitlines()) >= 800:
            fail(f"CALL-RECEIPT0 source/check file reached 800 lines: {path}")


def validate_fieldget_receipt0_authority_v1(root: Path) -> None:
    fields = code_only(read(root / "src/mir/builder/fields.rs"))
    receipt = code_only(read(root / "src/mir/builder/fields/post_success.rs"))

    ordinary = fields.split("pub(super) fn build_field_access_from_value", 1)[1].split(
        "pub(super) fn build_field_assignment_from_value", 1
    )[0]
    ordinary_commit = ordinary.split("let field_result_origin", 1)[1]
    if ordinary.count("PreparedOrdinaryFieldGetPostSuccessV1::prepare") != 1:
        fail("FIELDGET-RECEIPT0 requires one ordinary payload preparation consumer")
    if ordinary.count("post_success.commit(self, field_val, object_value)") != 1:
        fail("FIELDGET-RECEIPT0 requires one ordinary post-success payload consumer")
    if ordinary_commit.find("post_success.commit(self, field_val, object_value)") < ordinary_commit.find(
        "self.emit_instruction(crate::mir::MirInstruction::FieldGet"
    ):
        fail("FIELDGET-RECEIPT0 ordinary payload consumption must follow FieldGet emission")
    for forbidden in (
        "alloc_typed(",
        "value_types.insert",
        "record_field_access_site(",
        "publish_field_result_origin(",
    ):
        if forbidden in ordinary_commit:
            fail(f"FIELDGET-RECEIPT0 direct ordinary effect survived in fields.rs: {forbidden}")

    if receipt.count("fn commit(") != 1:
        fail("FIELDGET-RECEIPT0 requires one post-success commit owner")
    if receipt.count("TypeFactDecisionV1::prepare") != 1 or receipt.count("type_ctx.set_type") != 1:
        fail("FIELDGET-RECEIPT0 exact type decision/commit owner drift")
    if receipt.count("record_field_access_site(") != 1 or receipt.count("set_origin_box") != 1:
        fail("FIELDGET-RECEIPT0 site/origin commit owner drift")
    if receipt.find("type_ctx.set_type") > receipt.find("record_field_access_site("):
        fail("FIELDGET-RECEIPT0 type commit must precede ordinary site commit")
    if receipt.find("record_field_access_site(") > receipt.find("set_origin_box"):
        fail("FIELDGET-RECEIPT0 ordinary site commit must precede origin commit")
    if "metadata::propagate" in receipt:
        fail("FIELDGET-RECEIPT0 must not reuse metadata propagation")
    for path in (
        root / "src/mir/builder/fields.rs",
        root / "src/mir/builder/fields/post_success.rs",
        root / "src/mir/builder/calls/unified_emitter/temporal_witness_tests.rs",
        Path(__file__),
    ):
        if len(read(path).splitlines()) >= 800:
            fail(f"FIELDGET-RECEIPT0 source/check file reached 800 lines: {path}")


def check(root: Path) -> None:
    fixture = load_fixture(root)
    validate_p1_g0_profile_freeze_v1(fixture)
    validate_active_cutover_writer_inventory_v1(root, fixture)
    validate_const0_authority_v1(root)
    validate_staticload0_authority_v1(root)
    validate_checkselect0_authority_v1(root)
    validate_literal_postemit_retirement_v1(root)
    validate_resolved_trivial_operation_authority_v1(root)
    validate_resolved_direct_call_authority_v1(root)
    validate_compareemit0_authority_v1(root)
    validate_call_receipt0_authority_v1(root)
    validate_fieldget_receipt0_authority_v1(root)
    matrix = fixture.get("primary_matrix")
    if not isinstance(matrix, list):
        fail("FACT0 fixture primary matrix is invalid")
    for row in matrix:
        if not isinstance(row, dict):
            fail("FACT0 fixture primary matrix row is invalid")
        require_anchor(root, row)
    print(
        "[mirbuilder-type-fact-partition-guard] ok "
        "baseline_writer_paths=47 baseline_writer_occurrences=99 "
        "active_writer_paths=49 active_writer_occurrences=94 slices=58 profiles=38 "
        "shared_slices=2 const0=closed staticload0=closed checkselect0=closed "
        "literal_postemit_ret0=closed resolved_trivial_op0=closed "
        "resolved_direct_call0=closed compareemit0=closed call_receipt0=closed "
        "fieldget_receipt0=closed"
    )


def main(argv: list[str]) -> None:
    if len(argv) != 2:
        fail("usage: mirbuilder_type_fact_partition_guard.py ROOT")
    check(Path(argv[1]).resolve())


if __name__ == "__main__":
    main(sys.argv)
