#!/usr/bin/env python3
"""Validate the operation-domain-aware FINALIZE0 schema-v2 inventory."""

from __future__ import annotations

import json
import sys
from collections import Counter, defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
FIXTURE = ROOT / "tools/checks/fixtures/mirbuilder_finalize0_pass_inventory_v2.json"

DOMAINS = {"semantic_pass", "lifecycle_transition", "diagnostic_observation"}
SEMANTIC_CLASSES = {
    "VerifyNormalizationPreconditions",
    "NormalizeRepresentation",
    "PublishSealedArtifact",
    "VerifyPublishedDraft",
    "RepairMissingLoweringFact",
    "LegacySemanticInference",
}
PUBLICATION_KINDS = {
    "None",
    "SealedFactSnapshot",
    "PureProjection",
    "RecomputableDerivedArtifact",
    "DiagnosticArtifact",
}
MUTATION_CLASSES = {
    "ReadOnly",
    "CandidateMirMutation",
    "TransientFactMutation",
    "MetadataMutation",
    "LifecycleMutation",
    "DiagnosticOnly",
    "CandidateAndFactMutation",
}
IDENTITY_STABILITY = {"Preserved", "AtomicRewriteRequired", "NotApplicable"}
SESSION_GENERATIONS = {
    "CurrentFunction",
    "CurrentModule",
    "CompilerPostBuild",
    "RepositoryGlobal",
    "None",
}
FAILURE_ATOMICITY = {
    "Infallible",
    "TypedPrePublication",
    "NonAtomicLegacy",
    "DiagnosticOnly",
}
DISPOSITIONS = {
    "retain",
    "split_before_cut",
    "retire_after",
    "external_owner",
    "lifecycle_only",
    "diagnostic_only",
}
CFG_DOMAINS = {"production", "test"}

REQUIRED = {
    "id",
    "operation_domain",
    "owner",
    "production_invocation_count",
    "route_reachability",
    "canonical_repair_reachable",
    "source_sites",
    "input_authority",
    "outputs",
    "publication_kind",
    "first_publication",
    "mutation_class",
    "identity_stability",
    "invalidated_artifacts",
    "session_generation",
    "failure_atomicity",
    "lowering_consumers",
    "downstream_consumers",
    "disposition",
    "retirement_owner",
    "retirement_dependencies",
}
SOURCE_REQUIRED = {
    "path",
    "enclosing_symbol",
    "operation",
    "occurrence_ordinal",
    "cfg_domain",
}


def fail(message: str) -> None:
    raise SystemExit(f"[mirbuilder-finalize0-pass-inventory] {message}")


def require_string(row: dict, key: str) -> None:
    if not isinstance(row.get(key), str) or not row[key]:
        fail(f"{row.get('id')}: {key} must be a non-empty string")


def require_string_list(row: dict, key: str, *, allow_empty: bool = False) -> None:
    value = row.get(key)
    if not isinstance(value, list) or any(not isinstance(item, str) or not item for item in value):
        fail(f"{row.get('id')}: {key} must be a string list")
    if not allow_empty and not value:
        fail(f"{row.get('id')}: {key} must not be empty")


def match_offsets(text: str, needle: str) -> list[int]:
    offsets: list[int] = []
    start = 0
    while True:
        found = text.find(needle, start)
        if found < 0:
            return offsets
        offsets.append(found)
        start = found + len(needle)


def validate_row(row: dict) -> None:
    row_id = row.get("id")
    missing = sorted(REQUIRED - row.keys())
    if missing:
        fail(f"{row_id}: missing fields: {','.join(missing)}")
    extra_class = row.get("semantic_class")
    domain = row.get("operation_domain")
    if domain not in DOMAINS:
        fail(f"{row_id}: invalid operation_domain")
    if domain == "semantic_pass":
        if extra_class not in SEMANTIC_CLASSES:
            fail(f"{row_id}: semantic_pass requires a known semantic_class")
    elif extra_class is not None:
        fail(f"{row_id}: non-semantic operation must omit semantic_class")

    require_string(row, "owner")
    require_string(row, "input_authority")
    require_string(row, "outputs")
    require_string(row, "first_publication")
    require_string(row, "retirement_owner")
    for key in (
        "route_reachability",
        "invalidated_artifacts",
        "lowering_consumers",
        "downstream_consumers",
        "retirement_dependencies",
    ):
        require_string_list(row, key, allow_empty=True)
    if not row["route_reachability"]:
        fail(f"{row_id}: route_reachability must not be empty")

    if (
        not isinstance(row.get("production_invocation_count"), int)
        or row["production_invocation_count"] < 1
    ):
        fail(f"{row_id}: invalid production_invocation_count")
    if not isinstance(row.get("canonical_repair_reachable"), bool):
        fail(f"{row_id}: canonical_repair_reachable must be boolean")
    if row.get("publication_kind") not in PUBLICATION_KINDS:
        fail(f"{row_id}: invalid publication_kind")
    if row.get("mutation_class") not in MUTATION_CLASSES:
        fail(f"{row_id}: invalid mutation_class")
    if row.get("identity_stability") not in IDENTITY_STABILITY:
        fail(f"{row_id}: invalid identity_stability")
    if row.get("session_generation") not in SESSION_GENERATIONS:
        fail(f"{row_id}: invalid session_generation")
    if row.get("failure_atomicity") not in FAILURE_ATOMICITY:
        fail(f"{row_id}: invalid failure_atomicity")
    if row.get("disposition") not in DISPOSITIONS:
        fail(f"{row_id}: invalid disposition")

    sites = row.get("source_sites")
    if not isinstance(sites, list) or not sites:
        fail(f"{row_id}: source_sites must not be empty")
    for site in sites:
        if not isinstance(site, dict) or set(site) != SOURCE_REQUIRED:
            fail(f"{row_id}: source site fields mismatch")
        for key in ("path", "enclosing_symbol", "operation"):
            if not isinstance(site[key], str) or not site[key]:
                fail(f"{row_id}: invalid source site {key}")
        if site["cfg_domain"] not in CFG_DOMAINS:
            fail(f"{row_id}: invalid cfg_domain")
        if not isinstance(site["occurrence_ordinal"], int) or site["occurrence_ordinal"] < 0:
            fail(f"{row_id}: invalid occurrence_ordinal")


def validate_source_bijection(rows: list[dict]) -> int:
    groups: dict[tuple[str, str], list[tuple[int, str]]] = defaultdict(list)
    identities: set[tuple[str, str, int]] = set()
    for row in rows:
        for site in row["source_sites"]:
            key = (site["path"], site["operation"])
            identity = (*key, site["occurrence_ordinal"])
            if identity in identities:
                fail(f"duplicate source identity: {identity}")
            identities.add(identity)
            groups[key].append((site["occurrence_ordinal"], row["id"]))

    for (relative, operation), entries in groups.items():
        source = ROOT / relative
        if not source.is_file() or not source.resolve().is_relative_to(ROOT / "src"):
            fail(f"source path is missing or outside src/: {relative}")
        matches = match_offsets(source.read_text(), operation)
        actual_ordinals = sorted(ordinal for ordinal, _ in entries)
        expected_ordinals = list(range(len(matches)))
        if actual_ordinals != expected_ordinals:
            fail(
                f"source coverage mismatch path={relative} operation={operation!r} "
                f"expected={expected_ordinals} inventory={actual_ordinals}"
            )
    return len(identities)


def main() -> int:
    data = json.loads(FIXTURE.read_text())
    if data.get("schema") != "mirbuilder-finalize0-pass-inventory-v2":
        fail("schema mismatch")
    if data.get("behavior_delta") != 0 or data.get("production_connections") != 0:
        fail("SCHEMA0 must remain behavior-neutral and disconnected")
    if data.get("semantic_lowering_origin_field") != "value_origin_newbox":
        fail("semantic lowering origin field drifted")
    if data.get("diagnostic_origin_field") != "value_origin_callers":
        fail("diagnostic origin field drifted")
    if data.get("post_builder_semantic_origin_field") != "none":
        fail("post-Builder semantic origin publication must remain explicit none")
    rows = data.get("rows")
    if not isinstance(rows, list) or not rows:
        fail("rows must be a non-empty list")
    ids = [row.get("id") for row in rows]
    if any(not row_id for row_id in ids) or len(set(ids)) != len(ids):
        fail("duplicate or empty row id")
    for row in rows:
        validate_row(row)
    source_count = validate_source_bijection(rows)

    domains = Counter(row["operation_domain"] for row in rows)
    classes = Counter(
        row.get("semantic_class")
        for row in rows
        if row["operation_domain"] == "semantic_pass"
    )
    for required in SEMANTIC_CLASSES:
        if classes[required] == 0:
            fail(f"semantic class disappeared: {required}")
    if domains["lifecycle_transition"] == 0 or domains["diagnostic_observation"] == 0:
        fail("lifecycle/diagnostic domain disappeared")
    repair_reachable = sum(row["canonical_repair_reachable"] for row in rows)
    declared_invocations = sum(row["production_invocation_count"] for row in rows)
    print(
        "[mirbuilder-finalize0-pass-inventory] ok "
        f"schema=v2 rows={len(rows)} source_sites={source_count} "
        f"declared_production_invocations={declared_invocations} "
        f"semantic={domains['semantic_pass']} "
        f"lifecycle={domains['lifecycle_transition']} diagnostic={domains['diagnostic_observation']} "
        f"canonical_repair_reachable={repair_reachable} "
        f"behavior_delta={data['behavior_delta']} production_connections={data['production_connections']}"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
