#!/usr/bin/env python3
"""Report ObjectStoragePlan vocabulary groups and merge candidates.

This audit is deliberately read-only. It records which vocabulary groups are
real semantic boundaries and which names are future merge/retire candidates.
"""

from __future__ import annotations

import argparse
import json
from dataclasses import asdict, dataclass
from pathlib import Path


@dataclass(frozen=True)
class AuditRow:
    name: str
    kind: str
    status: str
    action: str
    reason: str


ROWS: tuple[AuditRow, ...] = (
    AuditRow(
        name="ids",
        kind="keep_separate",
        status="strong_newtype_boundary",
        action="keep",
        reason="semantic_id_boundaries_prevent_raw_u32_confusion",
    ),
    AuditRow(
        name="storage",
        kind="keep_separate",
        status="representation_truth",
        action="keep",
        reason="ObjectStoragePlan_is_representation_not_execution",
    ),
    AuditRow(
        name="publication",
        kind="keep_separate",
        status="publication_state_truth",
        action="keep",
        reason="publication_is_escape_boundary_not_backend_fact",
    ),
    AuditRow(
        name="local_fastpath_fact",
        kind="keep_separate",
        status="positive_backend_fact",
        action="keep",
        reason="Fact_fallback_separation_is_core_invariant",
    ),
    AuditRow(
        name="alias",
        kind="keep_separate",
        status="passive_observation",
        action="keep",
        reason="alias_observation_is_not_publication_or_fact",
    ),
    AuditRow(
        name="inventory_shadow",
        kind="keep_separate",
        status="report_only_surface",
        action="keep",
        reason="inventory_and_shadow_rows_are_not_backend_consumable",
    ),
)


def _read_rs_files(root: Path) -> list[tuple[Path, str]]:
    src = root / "src"
    if not src.exists():
        return []
    rows: list[tuple[Path, str]] = []
    for path in sorted(src.rglob("*.rs")):
        if "target" in path.parts:
            continue
        rows.append((path.relative_to(root), path.read_text(encoding="utf-8")))
    return rows


def _count_token(
    files: list[tuple[Path, str]],
    token: str,
    *,
    exclude_prefixes: tuple[str, ...] = (),
    exclude_suffixes: tuple[str, ...] = (),
) -> int:
    count = 0
    for rel, text in files:
        rel_text = str(rel).replace("\\", "/")
        if any(rel_text.startswith(prefix) for prefix in exclude_prefixes):
            continue
        if any(rel_text.endswith(suffix) for suffix in exclude_suffixes):
            continue
        count += text.count(token)
    return count


def usage_inventory(root: Path) -> dict[str, str]:
    files = _read_rs_files(root)
    exact_stack_source_presence = _count_token(
        files,
        "ExactStackObject",
        exclude_prefixes=(),
    )
    fastpath_decision_consumers = _count_token(
        files,
        "FastPathDecision",
        exclude_suffixes=("src/object_storage_plan/decision.rs", "src/object_storage_plan/tests.rs"),
    )
    fastpath_reachability_consumers = _count_token(
        files,
        "FastPathReachability",
        exclude_suffixes=("src/object_storage_plan/reachability.rs", "src/object_storage_plan/tests.rs"),
    )
    fastpath_deny_owner_presence = _count_token(
        files,
        "FastPathDenyOwner",
        exclude_suffixes=("src/object_storage_plan/tests.rs",),
    )
    return {
        "exact_stack_object_retired": "1",
        "exact_stack_object_source_presence_count": str(exact_stack_source_presence),
        "active_exact_storage_forms": "ExactNativeStruct,Scalarized,FlattenedNestedFields",
        "stack_allocation_support_claimed": "0",
        "reason_enum_merge_enabled": "0",
        "reason_domain_report_enabled": "1",
        "reason_domain_count": "3",
        "reason_domain_storage_enums_kept": "3",
        "reason_domain_publication_enum_kept": "1",
        "reason_domain_fastpath_enum_kept": "1",
        "object_site_location_field_migration_complete": "1",
        "site_location_fields_candidate_retired": "1",
        "scalar_field_descriptor_merge_enabled": "0",
        "field_scalar_plan_kept": "1",
        "flattened_nested_field_plan_kept": "1",
        "scalar_field_descriptor_candidate_closed": "1",
        "fastpath_reachability_rust_vocab_retired": "1",
        "fastpath_reachability_tooling_owner": "hako_check",
        "fastpath_decision_non_test_consumer_count": str(fastpath_decision_consumers),
        "fastpath_reachability_non_test_consumer_count": str(fastpath_reachability_consumers),
        "fastpath_deny_owner_code_retired": "1",
        "fastpath_deny_owner_source_presence_count": str(fastpath_deny_owner_presence),
        "fastpath_deny_owner_mapping_owner": "docs_report",
        "passive_vocab_execution_enabled": "0",
        "vocab_retire_allowed": "0",
    }


def build_report(repo_root: Path | None = None) -> dict[str, object]:
    keep_count = sum(row.kind == "keep_separate" for row in ROWS)
    merge_count = sum(row.kind == "merge_candidate" for row in ROWS)
    report: dict[str, object] = {
        "output_contract": "hako-object-storage-plan-vocab-audit-v0",
        "source_evidence": "296x-1055,object-storage-plan-storage-rs",
        "row_kind": "inventory",
        "keep_separate_count": str(keep_count),
        "merge_candidate_count": str(merge_count),
        "immediate_merge_allowed": "0",
        "vocabulary_merge_count": "0",
        "fact_fallback_separation_preserved": "1",
        "public_api_reexport_preserved": "1",
        "guard_path_compat_landed": "1",
        "first_safe_followup": "none",
        "summary": "ok",
        "rows": [asdict(row) for row in ROWS],
    }
    if repo_root is not None:
        report.update(usage_inventory(repo_root))
    return report


def emit_kv(report: dict[str, object]) -> None:
    for key, value in report.items():
        if key == "rows":
            continue
        print(f"{key}={value}")
    rows = report.get("rows")
    if isinstance(rows, list):
        for idx, row in enumerate(rows):
            if not isinstance(row, dict):
                continue
            for key, value in row.items():
                print(f"row_{idx}_{key}={value}")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", type=Path, default=Path.cwd())
    parser.add_argument("--format", choices=("kv", "json"), default="kv")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    report = build_report(args.repo_root.resolve())
    if args.format == "json":
        print(json.dumps(report, indent=2, sort_keys=True))
    else:
        emit_kv(report)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
