#!/usr/bin/env python3
"""Resolve EmissionSsaPhi contract/lifecycle projection policy."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
PRIORITY = FIXTURES / "mirbuilder-projection-policy-cluster-priority-resolution-v0.json"
REPORT = FIXTURES / "mirbuilder-crate-wide-unconverted-surface-report-v0.json"
OUTPUT = FIXTURES / "mirbuilder-emission-ssa-phi-projection-policy-v0.json"
TOKEN = "MIRBUILDER-EMISSION-SSA-PHI-PROJECTION-POLICY-001"
SELECTED_CLUSTER_IDS = [
    (
        "projection_policy::UnsupportedDirectShape::shape.emission_ssa_phi::"
        "FixtureMapped::EmissionSsaPhiCluster::borrow=NoBorrow::"
        "control=PhiRequired::type=Known::call=AllKnown::verifier=Present"
    ),
    (
        "projection_policy::UnsupportedDirectShape::shape.emission_ssa_phi::"
        "FixtureMapped::EmissionSsaPhiCluster::borrow=NoReturnedBorrow::"
        "control=PhiRequired::type=Known::call=AllKnown::verifier=Present"
    ),
]

EXPECTED_SURFACES = {
    "src/mir/builder/ssa/analysis.rs::strict_planner_required:L5": {
        "role": "analysis_predicate",
        "marker": "planner_required_enabled()",
    },
    "src/mir/builder/ssa/analysis.rs::value_defined_in_current_function:L10": {
        "role": "analysis_predicate",
        "marker": "func.params.iter().any",
    },
    "src/mir/builder/ssa/analysis.rs::format_value_ids:L36": {
        "role": "diagnostic_formatter",
        "marker": 'out.push_str(&v.0.to_string())',
    },
    "src/mir/builder/ssa/analysis.rs::has_dominated_same_field_set_after_root:L201": {
        "role": "analysis_predicate",
        "marker": "compute_dominators(func)",
    },
    "src/mir/builder/ssa/phi_input_contract.rs::check_phi_input_contract:L7": {
        "role": "contract_validator",
        "marker": "compute_predecessors(func)",
    },
    "src/mir/builder/emission/phi_lifecycle.rs::patch_phi_inputs:L81": {
        "role": "lifecycle_scope_patch",
        "marker": "self.pending",
    },
    "src/mir/builder/emission/phi_lifecycle.rs::define_phi_final:L246": {
        "role": "builder_phi_definition",
        "marker": "define_phi_final_with_type_hint(builder, block, dst, inputs, None, tag)",
    },
    "src/mir/builder/emission/phi_lifecycle.rs::define_phi_final_with_type_hint:L258": {
        "role": "builder_phi_definition",
        "marker": "phi_input_materializer::for_pred",
    },
    "src/mir/builder/emission/phi_lifecycle.rs::define_phi_final_fn:L423": {
        "role": "function_phi_definition",
        "marker": '"edgecfg_block_params"',
    },
    "src/mir/builder/emission/phi_lifecycle.rs::define_phi_final_fn_with_type_hint_and_tag:L443": {
        "role": "function_phi_definition",
        "marker": "insert_phi_at_head_spanned_with_type_hint",
    },
    "src/mir/builder/emission/phi_lifecycle.rs::patch_phi_inputs:L518": {
        "role": "builder_phi_patch",
        "marker": "builder.update_phi_instruction(block, dst, inputs)",
    },
    "src/mir/builder/phi.rs::define_current_block_phi_final:L9": {
        "role": "current_block_phi_definition",
        "marker": "define_phi_final(",
    },
    "src/mir/builder/phi.rs::define_current_block_phi_final_with_type_hint:L25": {
        "role": "current_block_phi_definition",
        "marker": "define_phi_final_with_type_hint(",
    },
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def read_source(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def cluster_axis(item: dict[str, Any]) -> str:
    for key in [
        "loop_cond_co_statement_lowering_subcluster",
        "loop_cond_co_helper_subcluster",
        "loop_cond_co_group_if_subcluster",
        "loop_cond_co_continue_if_subcluster",
        "loop_cond_co_subcluster",
        "loop_cond_bc_pipeline_subcluster",
        "loop_cond_bc_item_lowering_subcluster",
        "loop_cond_bc_cleanup_subcluster",
        "loop_cond_bc_else_pattern_subcluster",
        "loop_cond_bc_subcluster",
        "loop_cond_feature_subcluster",
        "plan_feature_subcluster",
        "joinir_plan_subcluster",
        "likely_owner_cluster",
    ]:
        value = item.get(key)
        if value:
            return value
    return "Unclustered"


def borrow_axis(item: dict[str, Any]) -> str:
    ret = item.get("return_type") or ""
    params = item.get("params") or ""
    if "&mut" in ret:
        return "ReturnedMutableAliasUnknown"
    if "&" in ret:
        return "BorrowPolicyNeeded"
    if "&mut" in params or "&self" in params:
        return "NoReturnedBorrow"
    return "NoBorrow"


def type_transport_axis(item: dict[str, Any]) -> str:
    ret = item.get("return_type") or ""
    if "unsafe" in (item.get("source_path") or ""):
        return "UnsafeOrFFI"
    if ret in {"", "bool", "usize", "i64", "String"}:
        return "Known"
    if "&" in ret:
        return "Missing"
    return "Missing"


def verifier_or_oracle_state(item: dict[str, Any]) -> str:
    if item.get("evidence_refs"):
        return "Present"
    return "MissingVerifier"


def selected_report_items(report: dict[str, Any]) -> list[dict[str, Any]]:
    items = [
        item
        for item in report.get("items", [])
        if item.get("classification") == "MissingProjectionPolicy"
        and cluster_axis(item) == "EmissionSsaPhiCluster"
        and item.get("owner_edge_confidence") == "FixtureMapped"
        and item.get("stable_deny_reason") == "UnsupportedDirectShape"
        and item.get("shape_signature") == "shape.emission_ssa_phi"
        and borrow_axis(item) in {"NoBorrow", "NoReturnedBorrow"}
        and type_transport_axis(item) == "Known"
        and verifier_or_oracle_state(item) == "Present"
    ]
    found = {item["source_id"] for item in items}
    expected = set(EXPECTED_SURFACES)
    if found != expected:
        missing = sorted(expected - found)
        extra = sorted(found - expected)
        raise SystemExit(f"EmissionSsaPhi selected surface drift: missing={missing} extra={extra}")
    return sorted(items, key=lambda item: item["source_id"])


def require_source_markers(items: list[dict[str, Any]]) -> list[dict[str, str]]:
    markers: list[dict[str, str]] = []
    for item in items:
        source_id = item["source_id"]
        marker = EXPECTED_SURFACES[source_id]["marker"]
        source_text = read_source(item["source_path"])
        if marker not in source_text:
            raise SystemExit(f"source marker drift for {source_id}: {marker!r}")
        markers.append({
            "source_id": source_id,
            "marker": marker,
        })
    return markers


def build_policy() -> dict[str, Any]:
    priority = read_json(PRIORITY)
    report = read_json(REPORT)
    priority_selected = (
        priority["decision"]["selected_cluster_id"] in set(SELECTED_CLUSTER_IDS)
        and priority["decision"]["selected_next_card"] == TOKEN
    )
    priority_excluded = any(
        item.get("cluster_id") in set(SELECTED_CLUSTER_IDS)
        for item in priority.get("excluded_existing_decision_clusters", [])
    )
    if not (priority_selected or priority_excluded):
        raise SystemExit("priority resolver neither selects nor excludes EmissionSsaPhi cluster")

    items = selected_report_items(report)
    source_markers = require_source_markers(items)
    role_counts: dict[str, int] = {}
    for item in items:
        role = EXPECTED_SURFACES[item["source_id"]]["role"]
        role_counts[role] = role_counts.get(role, 0) + 1

    return {
        "schema_version": 0,
        "kind": "MirBuilderEmissionSsaPhiProjectionPolicyV1",
        "token": TOKEN,
        "input_state": {
            "priority_resolution": rel(PRIORITY),
            "unconverted_surface_report": rel(REPORT),
            "selected_cluster_id": SELECTED_CLUSTER_IDS[0],
            "selected_cluster_ids": SELECTED_CLUSTER_IDS,
            "source_count": len(items),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "selection_axes": {
            "owner_edge_confidence": "FixtureMapped",
            "stable_deny_reason": "UnsupportedDirectShape",
            "shape_signature": "shape.emission_ssa_phi",
            "borrow_axes": ["NoBorrow", "NoReturnedBorrow"],
            "control_flow_axis": "PhiRequired",
            "type_transport_axis": "Known",
            "verifier_or_oracle_state": "Present",
        },
        "source_surfaces": [
            {
                "source_id": item["source_id"],
                "symbol": item["symbol"],
                "source_path": item["source_path"],
                "line": item["line"],
                "visibility": item["visibility"],
                "receiver": item["receiver"],
                "params": item["params"],
                "return_type": item["return_type"],
                "known_owner_edge": item["known_owner_edge"],
                "owner_edge_confidence": item["owner_edge_confidence"],
                "role": EXPECTED_SURFACES[item["source_id"]]["role"],
                "borrow_axis": borrow_axis(item),
            }
            for item in items
        ],
        "emission_ssa_phi_descriptor": {
            "descriptor_id": "emission_ssa_phi_contract_lifecycle_v1",
            "source_extraction": "rust_emission_ssa_phi_contract_and_lifecycle_helpers",
            "role_counts": dict(sorted(role_counts.items())),
            "analysis_predicates": [
                "strict_planner_required",
                "value_defined_in_current_function",
                "has_dominated_same_field_set_after_root",
            ],
            "contract_validators": [
                "check_phi_input_contract",
            ],
            "diagnostic_formatters": [
                "format_value_ids",
            ],
            "mutation_entrypoints": [
                "PhiLifecycle::patch_phi_inputs",
                "define_phi_final",
                "define_phi_final_with_type_hint",
                "define_phi_final_fn",
                "define_phi_final_fn_with_type_hint_and_tag",
                "patch_phi_inputs",
                "MirBuilder::define_current_block_phi_final",
                "MirBuilder::define_current_block_phi_final_with_type_hint",
            ],
            "mutation_frame": [
                "PHI inputs are sorted/materialized before insertion or patch",
                "builder/function PHI instruction state may be inserted or updated",
                "debug metadata may record value origin callers when debug is enabled",
            ],
            "return_contract": "Result<(), String> or predicate/diagnostic scalar",
            "returned_borrow": 0,
            "source_markers": source_markers,
        },
        "selected_policy": {
            "policy": "EmissionSsaPhiContractLifecycleDescriptor",
            "owner_edge": "mirbuilder::emission_ssa_phi",
            "descriptor_selected": True,
            "hako_projection_selected": False,
            "reason_token": "EmissionSsaPhiContractLifecycleDescriptorRequiredBeforeHakoProjection",
        },
        "decision": {
            "kind": "SelectProjectionPolicyDescriptor",
            "selected_next_card": "MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001",
            "reason_token": "EmissionSsaPhiContractLifecycleDescriptorMaterialized",
        },
        "claims": {
            "manual_family_selection": 0,
            "descriptor_selected": 1,
            "hako_projection_selected": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "native_seed_materialization": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
        },
        "provenance": {
            "tool_role": "FactsAdapterGuardOrchestrator",
            "semantic_projection_inference": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in policy fixture.")
    args = parser.parse_args()

    output = stable_json(build_policy())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-emission-ssa-phi-projection-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
