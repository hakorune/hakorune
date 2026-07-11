#!/usr/bin/env python3
"""Build and validate the S2 source-site/projection binding inventory.

This is an inventory artifact only. It never changes runtime carriers, routes,
backend lowering, or provider behavior.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
S1_GRAPH = ROOT / "tools/checks/manifests/failure_outcome_semantic_site_graph_v0.json"
OUTPUT = ROOT / "tools/checks/manifests/failure_outcome_projection_binding_v0.json"

PENDING_REASONS = frozenset(
    {
        "SourceOutcomeMissing",
        "ApiContractMissing",
        "ProviderContractMissing",
        "PayloadPolicyMissing",
        "ZeroCollisionUnproven",
    }
)
AUTHORITY_KINDS = frozenset(
    {"LanguageContract", "PublicApiContract", "InternalInvariant", "CompatibilityProfile"}
)
FAMILIES = frozenset(
    {
        "value_conversion",
        "weak_upgrade",
        "null_like_box",
        "provider_status",
        "missing_result_synthesis",
        "ffi_boundary",
    }
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--write", action="store_true")
    mode.add_argument("--check", action="store_true")
    parser.add_argument("--output", type=Path, default=OUTPUT)
    return parser.parse_args()


def read_graph() -> dict[str, Any]:
    return json.loads(S1_GRAPH.read_text(encoding="utf-8"))


def evidence_id(path: str, line: int, token: str) -> str:
    return f"{path}:{line}:{token}"


def anchor(path: str, needle: str, token: str) -> dict[str, Any]:
    source = ROOT / path
    lines = source.read_text(encoding="utf-8", errors="ignore").splitlines()
    for line_number, line in enumerate(lines, start=1):
        if needle in line:
            return {
                "evidence_id": evidence_id(path, line_number, token),
                "source_path": path,
                "line": line_number,
                "token": token,
                "evidence_kind": token,
                "evidence": line.strip(),
            }
    raise RuntimeError(f"missing S2 evidence anchor: {path}:{needle}")


def hako_mem_free_evidence() -> list[dict[str, Any]]:
    return [
        anchor(
            "docs/reference/runtime/substrate-capabilities.md",
            "hako_mem_free(ptr: native_ptr_nullable) -> void",
            "public_api_contract",
        ),
        anchor(
            "docs/reference/runtime/substrate-capabilities.md",
            "hako_mem_free(NULL)",
            "null_policy",
        ),
        anchor(
            "src/mir/extern_call_route_plan/route_spec.rs",
            'route_id: "extern.hako_mem.free"',
            "route_id",
        ),
        anchor(
            "src/mir/extern_call_route_plan/route_spec.rs",
            'return_shape: "void_sentinel_i64_zero"',
            "projection_encoding",
        ),
        anchor(
            "src/mir/extern_call_route_plan/tests/hako_mem.rs",
            'assert_eq!(route.return_shape(), "void_sentinel_i64_zero")',
            "consumer_contract",
        ),
        anchor(
            "crates/nyash_kernel/src/exports/mem.rs",
            'pub extern "C" fn hako_mem_free',
            "producer_implementation",
        ),
    ]


def operation_sites() -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    graph = read_graph()
    classified = [
        {
            **site,
            "authority_kind": "LanguageContract"
            if site.get("layer") == "reference"
            else "InternalInvariant",
            "semantic_owner": site.get("owner", ""),
        }
        for site in graph["semantic_sites"]
        if site.get("site_kind") == "operation_outcome"
        and site.get("review_status") == "classified"
    ]
    free_evidence = hako_mem_free_evidence()
    free_site = {
        "site_id": "runtime_backend.extern.hako_mem_free.success",
        "layer": "runtime_backend",
        "owner_domain": "extern",
        "operation": "hako_mem_free",
        "outcome_branch": "success",
        "authority_kind": "PublicApiContract",
        "semantic_owner": "HakoMemFreeApiOwner",
        "semantic_class": "successful_no_result",
        "target_carrier": "Unit",
        "profile": "canonical",
        "evidence_refs": [row["evidence_id"] for row in free_evidence],
        "review_status": "classified",
    }
    return classified + [free_site], free_evidence


def projection_binding(free_evidence: list[dict[str, Any]]) -> dict[str, Any]:
    return {
        "projection_id": "runtime_backend.extern.hako_mem_free.void_sentinel_i64_zero",
        "projection_owner": "ExternCallRouteOwner",
        "projects_site": "runtime_backend.extern.hako_mem_free.success",
        "backend": "extern_abi",
        "route_id": "extern.hako_mem.free",
        "encoding": "VoidSentinelI64Zero",
        "payload_policy": "NoPayload",
        "collision_policy": "NotAValueLane",
        "observability": "AbiOnlyDiscarded",
        "capability": "extern_registry",
        "profile": "canonical",
        "evidence_refs": [row["evidence_id"] for row in free_evidence],
        "resolution": "BoundInventoryOnly",
    }


def projection_candidates(graph: dict[str, Any]) -> list[dict[str, Any]]:
    candidates: list[dict[str, Any]] = []
    for site in graph["semantic_sites"]:
        if site.get("site_kind") != "boundary_projection":
            continue
        candidates.append(
            {
                "candidate_id": f"pending.{site['site_id']}",
                "observed_site": site["site_id"],
                "observed_carrier": site.get("current_carrier", ""),
                "encoding": (
                    "ConstValueNullOrVoid"
                    if site.get("current_carrier") in {"ConstValue::Null", "ConstValue::Void"}
                    else "UnknownProjectionEncoding"
                ),
                "evidence_refs": site.get("evidence_refs", []),
                "resolution": "Pending",
                "pending_reason": "SourceOutcomeMissing",
            }
        )
    return sorted(candidates, key=lambda row: row["candidate_id"])


def provider_fallback_observations() -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    roots = (ROOT / "src/backend",)
    for root in roots:
        for path in sorted(root.rglob("*.rs")):
            relative = path.relative_to(ROOT).as_posix()
            for line_number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
                if "unwrap_or(Ok(VMValue::Void))" not in line:
                    continue
                rows.append(
                    {
                        "observation_id": evidence_id(
                            relative, line_number, "provider_missing_fallback"
                        ),
                        "boundary_id": "extern_provider_dispatch",
                        "boundary_kind": "provider",
                        "direction": "ingress",
                        "transport_outcome": "ProviderUnavailable",
                        "boundary_carrier": "VMValue::Void",
                        "observation_owner": "ExternProviderBoundaryOwner",
                        "evidence_refs": [
                            evidence_id(relative, line_number, "provider_missing_fallback")
                        ],
                        "resolution": "Pending",
                        "pending_reason": "ProviderContractMissing",
                    }
                )
    return rows


def build_manifest() -> dict[str, Any]:
    graph = read_graph()
    sites, free_evidence = operation_sites()
    fallback_rows = provider_fallback_observations()
    return {
        "schema_version": 0,
        "status": "projection_binding_inventory",
        "semantic_activation": 0,
        "operation_outcome_sites": sites,
        "projection_bindings": [projection_binding(free_evidence)],
        "projection_candidates": projection_candidates(graph),
        "boundary_observations": fallback_rows,
        "boundary_adapters": [],
        "evidence_occurrences": free_evidence,
        "pending_reasons": sorted(PENDING_REASONS),
        "families": sorted(FAMILIES),
    }


def validate(manifest: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if manifest.get("schema_version") != 0:
        errors.append("schema_version must be 0")
    if manifest.get("semantic_activation") != 0:
        errors.append("semantic activation must remain 0")
    sites = {site["site_id"]: site for site in manifest.get("operation_outcome_sites", [])}
    if len(sites) != len(manifest.get("operation_outcome_sites", [])):
        errors.append("duplicate operation outcome site")
    for site in sites.values():
        if site.get("authority_kind") not in AUTHORITY_KINDS:
            errors.append(f"unknown authority kind: {site['site_id']}")
        if "source_observation" in str(site.get("site_id")):
            errors.append(f"synthetic source cannot be authority: {site['site_id']}")
        if site.get("review_status") == "classified" and any(
            not site.get(field)
            for field in ("semantic_class", "target_carrier", "semantic_owner", "authority_kind")
        ):
            errors.append(f"incomplete classified site: {site['site_id']}")
    bindings = manifest.get("projection_bindings", [])
    if len(bindings) != len({row.get("projection_id") for row in bindings}):
        errors.append("duplicate projection binding")
    for binding in bindings:
        source = sites.get(binding.get("projects_site"))
        if not source or source.get("review_status") != "classified":
            errors.append(f"projection source is not classified: {binding.get('projection_id')}")
        if binding.get("resolution") != "BoundInventoryOnly":
            errors.append(f"projection binding is not inventory-bound: {binding.get('projection_id')}")
        if binding.get("encoding") == "VoidSentinelI64Zero" and any(
            binding.get(field) != expected
            for field, expected in (
                ("payload_policy", "NoPayload"),
                ("collision_policy", "NotAValueLane"),
                ("observability", "AbiOnlyDiscarded"),
            )
        ):
            errors.append(f"zero projection policy is incomplete: {binding.get('projection_id')}")
        if not binding.get("payload_policy"):
            errors.append(f"projection payload policy missing: {binding.get('projection_id')}")
    for candidate in manifest.get("projection_candidates", []):
        if candidate.get("resolution") != "Pending":
            errors.append(f"projection candidate must remain pending: {candidate.get('candidate_id')}")
        if candidate.get("pending_reason") not in PENDING_REASONS:
            errors.append(f"unknown projection pending reason: {candidate.get('candidate_id')}")
    observations = manifest.get("boundary_observations", [])
    if len(observations) != len({row.get("observation_id") for row in observations}):
        errors.append("duplicate boundary observation")
    for observation in observations:
        if observation.get("resolution") != "Pending":
            errors.append(f"provider fallback must remain pending: {observation.get('observation_id')}")
        if observation.get("pending_reason") not in PENDING_REASONS:
            errors.append(f"unknown provider pending reason: {observation.get('observation_id')}")
    for adapter in manifest.get("boundary_adapters", []):
        if not adapter.get("consumes_observation") or not adapter.get("maps_to_site"):
            errors.append(f"implicit boundary mapping: {adapter.get('adapter_id')}")
        target = sites.get(adapter.get("maps_to_site"))
        if not target or target.get("review_status") != "classified":
            errors.append(f"boundary adapter target is not classified: {adapter.get('adapter_id')}")
    if len(bindings) != 1 or bindings[0].get("projects_site") != "runtime_backend.extern.hako_mem_free.success":
        errors.append("hako_mem_free positive corridor missing")
    if len(observations) != 6:
        errors.append(f"expected six provider fallback observations, got {len(observations)}")
    if not manifest.get("projection_candidates"):
        errors.append("projection candidate inventory is empty")
    return errors


def main() -> int:
    args = parse_args()
    expected = json.dumps(build_manifest(), ensure_ascii=True, indent=2, sort_keys=True) + "\n"
    if args.write:
        args.output.write_text(expected, encoding="utf-8")
        print(f"[failure-outcome-projection-binding] wrote {args.output}")
        return 0
    actual = args.output.read_text(encoding="utf-8") if args.output.is_file() else ""
    if actual != expected:
        print("[failure-outcome-projection-binding] drift detected")
        return 1
    errors = validate(json.loads(actual))
    if errors:
        for error in errors:
            print(f"[failure-outcome-projection-binding] {error}")
        return 1
    manifest = json.loads(actual)
    print(
        "[failure-outcome-projection-binding] "
        f"sites={len(manifest['operation_outcome_sites'])} "
        f"bindings={len(manifest['projection_bindings'])} "
        f"candidates={len(manifest['projection_candidates'])} "
        f"pending_provider={len(manifest['boundary_observations'])}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
