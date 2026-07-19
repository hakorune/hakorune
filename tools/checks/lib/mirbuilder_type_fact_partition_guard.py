#!/usr/bin/env python3
"""Freeze the approved FACT0-P1 semantic partition before PHI0 migration."""

from __future__ import annotations

import hashlib
import json
import sys
from collections import Counter
from pathlib import Path

from mirbuilder_type_fact_producer_inventory import check as check_inventory
from mirbuilder_type_fact_producer_inventory import fail, load_fixture


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


def check(root: Path) -> None:
    check_inventory(root)
    validate_p1_g0_profile_freeze_v1(load_fixture(root))
    print(
        "[mirbuilder-type-fact-partition-guard] ok "
        "writer_paths=47 writer_occurrences=99 slices=58 profiles=38 shared_slices=2"
    )


def main(argv: list[str]) -> None:
    if len(argv) != 2:
        fail("usage: mirbuilder_type_fact_partition_guard.py ROOT")
    check(Path(argv[1]).resolve())


if __name__ == "__main__":
    main(sys.argv)
