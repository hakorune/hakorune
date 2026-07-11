#!/usr/bin/env python3
"""Build and validate the Failure/Outcome semantic-site graph.

The source scan remains an evidence queue. This module adds only stable
operation/outcome grouping; it does not infer runtime meaning for unresolved
sites or activate a new carrier.
"""

from __future__ import annotations

import argparse
import json
import re
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from failure_outcome_site_inventory import evidence_rows


ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / "tools/checks/manifests/failure_outcome_semantic_site_graph_v0.json"

# Lower this only in the same accepted classification slice that reduces the
# pending queue. Increasing the baseline would hide inventory regression.
PENDING_BASELINE_COUNTS = {"missing_argument_zero": 0}

IDENTIFIER = re.compile(r"^[a-z][a-z0-9_]*$")
SITE_SEGMENTS = ("layer", "owner_domain", "operation", "outcome_branch")
ALLOWED_LAYERS = frozenset(
    {"reference", "test", "runtime_backend", "mir", "parser", "implementation"}
)
ALLOWED_OWNER_DOMAINS = frozenset(
    {
        "backend",
        "carrier",
        "catch",
        "compatibility",
        "constant",
        "evidence",
        "extern",
        "option",
        "parser",
        "register",
        "result",
        "return",
        "weak",
    }
)
ALLOWED_OPERATIONS = frozenset(
    {
        "constant_bridge",
        "env_file_read",
        "env_get",
        "env_now_ms",
        "hako_mem_free",
        "missing_box",
        "option_value",
        "postfix_catch",
        "result_value",
        "token_observation",
        "vmvalue_void",
        "weak_upgrade",
    }
)
ALLOWED_OUTCOME_BRANCHES = frozenset(
    {
        "backend_null_projection",
        "backend_zero_null_projection",
        "current_carrier_observation",
        "dead_freed_absence",
        "equality_boxing",
        "missing_argument_zero",
        "missing_box_observation",
        "ordinary_absence",
        "profile_surface",
        "provider_failure",
        "provider_missing",
        "provider_route",
        "recoverable_failure",
        "success",
        "source_observation",
        "successful_no_result",
        "unresolved",
        "unit_projection",
        "undefined_register_fallback",
        "upgrade_observation",
        "weak_upgrade_projection",
    }
)
SITE_KINDS = frozenset(
    {"operation_outcome", "boundary_projection", "compatibility_adapter", "internal_sentinel"}
)
SEMANTIC_CLASSES = frozenset(
    {
        "",
        "optional_absence",
        "successful_no_result",
        "recoverable_failure",
        "contract_fault",
        "parser_or_builder_sentinel",
        "foreign_null",
        "compatibility_only",
    }
)


@dataclass(frozen=True)
class SiteDescriptor:
    layer: str
    owner_domain: str
    operation: str
    outcome_branch: str
    site_kind: str = "operation_outcome"

    @property
    def site_id(self) -> str:
        return ".".join(
            (self.layer, self.owner_domain, self.operation, self.outcome_branch)
        )


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--write", action="store_true")
    mode.add_argument("--check", action="store_true")
    parser.add_argument("--output", type=Path, default=OUTPUT)
    return parser.parse_args()


def contains_any(text: str, *needles: str) -> bool:
    return any(needle in text for needle in needles)


def is_missing_argument_zero(text: str, token: str = "") -> bool:
    if token in {"Option::None", "Result::Err"}:
        return False
    return bool(
        re.search(r"(?:arg_count|default[^\n]*argument|argument[^\n]*default)[^\n]*\b0\b", text)
    )


def descriptor_for(row: dict[str, Any]) -> SiteDescriptor:
    layer = str(row["layer"])
    token = str(row["surface_or_symbol"])
    evidence = str(row["evidence"])
    path = str(row["source_path"])

    if token == "env.get":
        branch = "missing_argument_zero" if is_missing_argument_zero(evidence, token) else "provider_route"
        if contains_any(evidence, "None =>", "provider", "missing"):
            branch = "provider_missing"
        return SiteDescriptor(layer, "extern", "env_get", branch)
    if token == "env.file.read":
        branch = "missing_argument_zero" if is_missing_argument_zero(evidence, token) else "provider_route"
        if contains_any(evidence, "Err", "error", "failure"):
            branch = "provider_failure"
        return SiteDescriptor(layer, "extern", "env_file_read", branch)
    if token == "env.now_ms":
        return SiteDescriptor(layer, "extern", "env_now_ms", "provider_failure")
    if token == "weak_to_strong":
        branch = "dead_freed_absence" if contains_any(
            evidence, "Dead", "Freed", "null", "none", "fail"
        ) else "upgrade_observation"
        return SiteDescriptor(layer, "weak", "weak_upgrade", branch)
    if token == "MissingBox":
        branch = "equality_boxing" if contains_any(
            evidence, "equal", "equality", "compat", "backward"
        ) else "missing_box_observation"
        kind = "compatibility_adapter" if branch == "equality_boxing" else "internal_sentinel"
        return SiteDescriptor(layer, "compatibility", "missing_box", branch, kind)
    if token == "postfix_catch":
        return SiteDescriptor(layer, "catch", "postfix_catch", "profile_surface")
    if token == "Option::None":
        return SiteDescriptor(layer, "option", "option_value", "ordinary_absence")
    if token == "Result::Err":
        return SiteDescriptor(layer, "result", "result_value", "recoverable_failure")
    if token == "ConstValue::Null":
        return SiteDescriptor(
            layer, "constant", "constant_bridge", "backend_null_projection", "boundary_projection"
        )
    if token == "ConstValue::Void":
        return SiteDescriptor(
            layer, "constant", "constant_bridge", "unit_projection", "boundary_projection"
        )
    if token == "VMValue::Void":
        if contains_any(evidence, "upgrade_weak", "weak_to_strong", "WeakBox"):
            branch = "weak_upgrade_projection"
            domain = "weak"
        elif contains_any(evidence, "unwrap_or", "reg_load", "args.get", "take_reg", "mem.get"):
            branch = "undefined_register_fallback"
            domain = "register"
        elif contains_any(evidence, "Null", "null", "Wasm", "LLVM", "zero"):
            branch = "backend_zero_null_projection"
            domain = "backend"
        elif contains_any(evidence, "MissingBox", "VoidBox", "compatibility"):
            branch = "equality_boxing"
            domain = "compatibility"
        elif contains_any(evidence, "Ok(VMValue::Void)", "return Ok(VMValue::Void)"):
            branch = "successful_no_result"
            domain = "return"
        else:
            branch = "current_carrier_observation"
            domain = "carrier"
        kind = "boundary_projection" if domain == "backend" else "internal_sentinel"
        return SiteDescriptor(layer, domain, "vmvalue_void", branch, kind)

    return SiteDescriptor(layer, "evidence", "token_observation", "unresolved")


def source_descriptor_for_projection(descriptor: SiteDescriptor) -> SiteDescriptor:
    return SiteDescriptor(
        descriptor.layer,
        descriptor.owner_domain,
        descriptor.operation,
        "source_observation",
        "operation_outcome",
    )


def evidence_occurrence(row: dict[str, Any]) -> dict[str, Any]:
    return {
        "evidence_id": row["site_id"],
        "source_path": row["source_path"],
        "line": row["line"],
        "token": row["surface_or_symbol"],
        "evidence_kind": row["evidence_kind"],
        "evidence": row["evidence"],
    }


def consensus(rows: list[dict[str, Any]], field: str) -> str:
    values = {str(row.get(field, "")) for row in rows}
    return next(iter(values)) if len(values) == 1 else ""


def semantic_site(descriptor: SiteDescriptor, rows: list[dict[str, Any]]) -> dict[str, Any]:
    classes = {str(row.get("semantic_class", "")) for row in rows}
    owners = {str(row.get("owner", "")) for row in rows}
    targets = {str(row.get("target_carrier", "")) for row in rows}
    classified = len(classes) == 1 and len(owners) == 1 and len(targets) == 1 and "" not in classes | owners | targets
    pending_reason = ""
    if any(
        is_missing_argument_zero(str(row["evidence"]), str(row["surface_or_symbol"]))
        for row in rows
    ):
        pending_reason = "missing_argument_zero"
        classified = False
    site: dict[str, Any] = {
        "site_id": descriptor.site_id,
        "site_kind": descriptor.site_kind,
        "layer": descriptor.layer,
        "owner_domain": descriptor.owner_domain,
        "operation": descriptor.operation,
        "outcome_branch": descriptor.outcome_branch,
        "semantic_class": consensus(rows, "semantic_class") if classified else "",
        "target_carrier": consensus(rows, "target_carrier") if classified else "",
        "owner": consensus(rows, "owner") if classified else "",
        "profile": consensus(rows, "profile"),
        "migration_action": consensus(rows, "migration_action") if classified else "",
        "backend_policy": consensus(rows, "backend_policy") if classified else "",
        "current_carrier": consensus(rows, "current_carrier") or "mixed",
        "evidence_refs": sorted(str(row["site_id"]) for row in rows),
        "review_status": "classified" if classified else "pending",
    }
    if pending_reason:
        site["pending_reason"] = pending_reason
    return site


def build_graph() -> dict[str, Any]:
    rows = evidence_rows()
    grouped: dict[SiteDescriptor, list[dict[str, Any]]] = {}
    projection_sources: dict[SiteDescriptor, list[dict[str, Any]]] = {}
    for row in rows:
        descriptor = descriptor_for(row)
        grouped.setdefault(descriptor, []).append(row)
        if descriptor.site_kind == "boundary_projection":
            projection_sources.setdefault(source_descriptor_for_projection(descriptor), []).append(row)

    sites = [semantic_site(descriptor, site_rows) for descriptor, site_rows in grouped.items()]
    for descriptor, site_rows in projection_sources.items():
        source = semantic_site(descriptor, site_rows)
        sites.append(source)
    for site in sites:
        if site["site_kind"] == "boundary_projection":
            source_id = source_descriptor_for_projection(
                SiteDescriptor(
                    site["layer"], site["owner_domain"], site["operation"], site["outcome_branch"]
                )
            ).site_id
            site["projects_site"] = source_id

    pending_counts = {
        "missing_argument_zero": sum(
            1 for site in sites if site.get("pending_reason") == "missing_argument_zero"
        )
    }
    return {
        "schema_version": 0,
        "status": "semantic_site_graph",
        "semantic_activation": 0,
        "site_id_grammar": {
            "segments": list(SITE_SEGMENTS),
            "separator": ".",
            "style": "lower_snake_case",
        },
        "pending_counts": pending_counts,
        "pending_baseline_counts": PENDING_BASELINE_COUNTS.copy(),
        "previous_pending_counts": PENDING_BASELINE_COUNTS.copy(),
        "evidence_occurrences": [evidence_occurrence(row) for row in rows],
        "semantic_sites": sorted(sites, key=lambda site: str(site["site_id"])),
    }


def validate_site_id(site_id: Any) -> str | None:
    if not isinstance(site_id, str):
        return "site_id must be a string"
    parts = site_id.split(".")
    if len(parts) != 4:
        return "semantic site id must have exactly four segments"
    if any(not IDENTIFIER.fullmatch(part) for part in parts):
        return "semantic site id contains a non-lower-snake-case segment"
    if parts[0] not in ALLOWED_LAYERS:
        return f"unknown site layer: {parts[0]}"
    if parts[1] not in ALLOWED_OWNER_DOMAINS:
        return f"unknown site owner domain: {parts[1]}"
    if parts[2] not in ALLOWED_OPERATIONS:
        return f"unknown site operation: {parts[2]}"
    if parts[3] not in ALLOWED_OUTCOME_BRANCHES:
        return f"unknown site outcome branch: {parts[3]}"
    return None


def validate(manifest: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if manifest.get("schema_version") != 0:
        errors.append("schema_version must be 0")
    if manifest.get("semantic_activation") != 0:
        errors.append("semantic activation must remain 0")
    occurrences = manifest.get("evidence_occurrences", [])
    evidence_ids = [row.get("evidence_id") for row in occurrences]
    if len(evidence_ids) != len(set(evidence_ids)):
        errors.append("duplicate evidence_id")
    sites = manifest.get("semantic_sites", [])
    site_ids = [site.get("site_id") for site in sites]
    if len(site_ids) != len(set(site_ids)):
        errors.append("duplicate semantic site id")
    site_index = set(site_ids)
    occurrence_index = set(evidence_ids)
    referenced_evidence: set[Any] = set()
    for site in sites:
        site_id = site.get("site_id")
        error = validate_site_id(site_id)
        if error:
            errors.append(f"{site_id}: {error}")
        if site.get("site_kind") not in SITE_KINDS:
            errors.append(f"{site_id}: unknown site kind")
        if site.get("semantic_class", "") not in SEMANTIC_CLASSES:
            errors.append(f"{site_id}: unknown semantic class")
        refs = site.get("evidence_refs", [])
        referenced_evidence.update(refs)
        if not refs:
            errors.append(f"{site_id}: semantic site has no evidence")
        if any(ref not in occurrence_index for ref in refs):
            errors.append(f"{site_id}: unknown evidence reference")
        if site.get("semantic_class") == "compatibility_only" and not site.get("profile"):
            errors.append(f"{site_id}: compatibility_only requires profile")
        if site.get("review_status") == "classified" and any(
            not site.get(field) for field in ("semantic_class", "owner", "target_carrier")
        ):
            errors.append(f"{site_id}: classified site is incomplete")
        if site.get("site_kind") == "boundary_projection":
            source = site.get("projects_site")
            if not source or source not in site_index:
                errors.append(f"{site_id}: projection missing projects_site")
            elif site.get("semantic_class") != next(
                candidate.get("semantic_class") for candidate in sites if candidate.get("site_id") == source
            ):
                errors.append(f"{site_id}: projection semantic-class drift")
    if referenced_evidence != occurrence_index:
        errors.append("evidence occurrence without semantic-site disposition")
    current = manifest.get("pending_counts", {}).get("missing_argument_zero")
    baseline = manifest.get("pending_baseline_counts", {}).get("missing_argument_zero")
    previous = manifest.get("previous_pending_counts", {}).get("missing_argument_zero")
    if not all(isinstance(value, int) for value in (current, baseline, previous)):
        errors.append("missing_argument_zero pending count missing")
    elif current > baseline:
        errors.append("missing_argument_zero pending count increased from baseline")
    elif current > previous:
        errors.append("missing_argument_zero pending count increased from previous manifest")
    return errors


def main() -> int:
    args = parse_args()
    expected = json.dumps(build_graph(), ensure_ascii=True, indent=2, sort_keys=True) + "\n"
    if args.write:
        args.output.write_text(expected, encoding="utf-8")
        print(f"[failure-outcome-site-graph] wrote {args.output}")
        return 0
    actual = args.output.read_text(encoding="utf-8") if args.output.is_file() else ""
    if actual != expected:
        print("[failure-outcome-site-graph] drift detected")
        return 1
    errors = validate(json.loads(actual))
    if errors:
        for error in errors:
            print(f"[failure-outcome-site-graph] {error}")
        return 1
    manifest = json.loads(actual)
    print(
        "[failure-outcome-site-graph] "
        f"evidence={len(manifest['evidence_occurrences'])} "
        f"sites={len(manifest['semantic_sites'])}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
