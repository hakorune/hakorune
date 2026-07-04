#!/usr/bin/env python3
"""Inventory narrow MirBuilder native-owner parity pilot surfaces.

This is a read-only selector aid. It inventories existing Rust-oracle parity
pilot artifacts and scans Rust source for small pure-looking leaf functions.
It does not select the next owner and does not make adoption claims.
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
PHASE = ROOT / "docs/development/current/main/phases/phase-296x"
HAKO_LIB = ROOT / "lang/src/compiler/lib"
CHECKS = ROOT / "tools/checks"
SRC_MIR = ROOT / "src/mir"
OUTPUT = FIXTURES / "mirbuilder-native-owner-candidate-inventory-v0.json"

TOKEN = "MIRBUILDER-NATIVE-OWNER-CANDIDATE-INVENTORY-001"
CONTRACT = "rust-lifecycle-mirbuilder-native-owner-candidate-inventory-v0"

MIRBUILDER_PREFIX = "mirbuilder-"
RUST_ORACLE_SUFFIX = "-rust-oracle-v0.json"
ADOPTION_SUFFIX = "-hako-adoption-decision-v0.json"
GATE_PREFIX = "rust_lifecycle_mirbuilder_"
GATE_SUFFIX = "_parity_gate.sh"

OWNER_SUFFIXES = (
    "label_formatter",
    "tag_formatter",
    "formatter",
    "classifier",
    "collector",
)

SOURCE_NAME_RE = re.compile(
    r"\b(?:pub\s+)?(?:const\s+)?fn\s+"
    r"(?P<name>as_str|is_[A-Za-z0-9_]+|[A-Za-z0-9_]*(?:label|tag|kind|name|class|format)[A-Za-z0-9_]*)"
    r"\s*\("
)

IMPL_DISPLAY_RE = re.compile(r"\bimpl\s+(?:std::fmt::)?Display\s+for\s+(?P<target>[A-Za-z0-9_:<>]+)")
CARD_ADOPT_RE = re.compile(r"Adopt `([^`]+)`")
CARD_PATH_RE = re.compile(
    r"(docs/development/current/main/design/fixtures/rust-lifecycle/"
    r"mirbuilder-[A-Za-z0-9_-]+-(?:rust-oracle|hako-adoption-decision)-v0\.json|"
    r"lang/src/compiler/lib/[A-Za-z0-9_]+\.hako|"
    r"tools/checks/rust_lifecycle_mirbuilder_[A-Za-z0-9_]+_parity_gate\.sh)"
)

REJECT_TERMS = {
    "builder": "depends_on_builder",
    "context": "depends_on_context",
    "metadata": "depends_on_metadata",
    "lower": "depends_on_lowering",
    "emit": "depends_on_emission",
    "route": "depends_on_route_selection",
    "resolve": "depends_on_resolution",
    "collect": "depends_on_collection",
    "insert": "may_mutate_state",
    "push": "may_mutate_state",
    "remove": "may_mutate_state",
    "retain": "may_mutate_state",
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def owner_from_fixture_name(path: Path, suffix: str) -> str:
    name = path.name
    if not name.startswith(MIRBUILDER_PREFIX) or not name.endswith(suffix):
        raise ValueError(f"unexpected fixture name: {path}")
    core = name[len(MIRBUILDER_PREFIX) : -len(suffix)]
    return core.replace("-", "_")


def owner_from_gate_name(path: Path) -> str | None:
    name = path.name
    if not name.startswith(GATE_PREFIX) or not name.endswith(GATE_SUFFIX):
        return None
    return name[len(GATE_PREFIX) : -len(GATE_SUFFIX)]


def owner_kind(owner: str) -> str:
    for suffix in OWNER_SUFFIXES:
        if owner.endswith(suffix):
            return suffix
    if owner.endswith("vocabulary"):
        return "vocabulary"
    if "_label_" in owner:
        return "label_surface"
    if "_tag_" in owner:
        return "tag_surface"
    return "unknown"


def read_json_or_empty(path: Path) -> dict[str, Any]:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except Exception:
        return {}


def collect_existing_rows() -> list[dict[str, Any]]:
    rows: dict[str, dict[str, Any]] = {}

    def ensure(owner: str) -> dict[str, Any]:
        return rows.setdefault(
            owner,
            {
                "owner": owner,
                "candidate_kind": owner_kind(owner),
                "rust_oracle_fixture": None,
                "hako_source": None,
                "parity_gate": None,
                "adoption_fixture": None,
                "adoption_card": None,
                "status": "unknown",
                "missing": [],
            },
        )

    for path in sorted(FIXTURES.glob(f"{MIRBUILDER_PREFIX}*{RUST_ORACLE_SUFFIX}")):
        owner = owner_from_fixture_name(path, RUST_ORACLE_SUFFIX)
        ensure(owner)["rust_oracle_fixture"] = rel(path)

    for path in sorted(FIXTURES.glob(f"{MIRBUILDER_PREFIX}*{ADOPTION_SUFFIX}")):
        owner = owner_from_fixture_name(path, ADOPTION_SUFFIX)
        row = ensure(owner)
        row["adoption_fixture"] = rel(path)
        fixture = read_json_or_empty(path)
        if fixture.get("owner"):
            row["owner_from_fixture"] = fixture["owner"]
        decision = fixture.get("decision") or {}
        if decision.get("selected_next_card"):
            row["selected_next_card"] = decision["selected_next_card"]

    for path in sorted(HAKO_LIB.glob("*.hako")):
        owner = path.stem
        if owner_kind(owner) == "unknown":
            continue
        ensure(owner)["hako_source"] = rel(path)

    for path in sorted(CHECKS.glob(f"{GATE_PREFIX}*{GATE_SUFFIX}")):
        owner = owner_from_gate_name(path)
        if owner is None:
            continue
        ensure(owner)["parity_gate"] = rel(path)

    for path in sorted(PHASE.glob("*HAKO-ADOPTION-DECISION-001.md")):
        text = path.read_text(encoding="utf-8")
        match = CARD_ADOPT_RE.search(text)
        if not match:
            continue
        logical_owner = match.group(1)
        paths = CARD_PATH_RE.findall(text)
        artifact_owners: list[str] = []
        for raw in paths:
            artifact_path = ROOT / raw
            if raw.endswith(RUST_ORACLE_SUFFIX):
                artifact_owner = owner_from_fixture_name(artifact_path, RUST_ORACLE_SUFFIX)
                row = ensure(artifact_owner)
                row["rust_oracle_fixture"] = raw
                artifact_owners.append(artifact_owner)
            elif raw.endswith(ADOPTION_SUFFIX):
                artifact_owner = owner_from_fixture_name(artifact_path, ADOPTION_SUFFIX)
                row = ensure(artifact_owner)
                row["adoption_fixture"] = raw
                artifact_owners.append(artifact_owner)
            elif raw.endswith(".hako"):
                artifact_owner = Path(raw).stem
                row = ensure(artifact_owner)
                row["hako_source"] = raw
                artifact_owners.append(artifact_owner)
            elif raw.endswith(GATE_SUFFIX):
                artifact_owner = owner_from_gate_name(Path(raw))
                if artifact_owner is not None:
                    row = ensure(artifact_owner)
                    row["parity_gate"] = raw
                    artifact_owners.append(artifact_owner)

        target_owner = artifact_owners[0] if artifact_owners else logical_owner
        if owner_kind(target_owner) == "unknown":
            continue
        row = ensure(target_owner)
        row["adoption_card"] = rel(path)
        row["logical_owner"] = logical_owner
        if logical_owner != target_owner:
            row["artifact_owner"] = target_owner

    required = ("rust_oracle_fixture", "hako_source", "parity_gate", "adoption_fixture", "adoption_card")
    for row in rows.values():
        row["missing"] = [field for field in required if not row.get(field)]
        if row["adoption_fixture"] and not row["missing"]:
            row["status"] = "adopted_complete"
        elif row["adoption_fixture"]:
            row["status"] = "adopted_incomplete_inventory"
        elif row["rust_oracle_fixture"] or row["hako_source"] or row["parity_gate"]:
            row["status"] = "partial_artifact"
        else:
            row["status"] = "unknown"

    return sorted(rows.values(), key=lambda row: row["owner"])


def extract_braced_block(text: str, open_brace: int) -> str:
    depth = 0
    for idx in range(open_brace, len(text)):
        char = text[idx]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return text[open_brace : idx + 1]
    return text[open_brace:]


def line_number(text: str, index: int) -> int:
    return text.count("\n", 0, index) + 1


def classify_source_candidate(name: str, signature: str, body: str, adopted_owners: set[str]) -> tuple[str, list[str]]:
    lowered = (signature + "\n" + body).lower()
    reasons: list[str] = []
    if "&mut self" in lowered or "mut self" in lowered:
        reasons.append("takes_mut_self")
    for needle, reason in REJECT_TERMS.items():
        if needle in lowered:
            reasons.append(reason)
    if "result<" in lowered or "option<" in lowered:
        reasons.append("carrier_surface")
    if "localssa" in lowered or "phi" in lowered:
        reasons.append("phi_or_localssa_surface")
    if "backend" in lowered:
        reasons.append("backend_surface")

    if reasons:
        return "rejected", sorted(set(reasons))
    if name in adopted_owners:
        return "already_adopted_name_match", []
    return "candidate", []


def collect_source_candidates(adopted_owners: set[str]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for path in sorted(SRC_MIR.rglob("*.rs")):
        text = path.read_text(encoding="utf-8", errors="replace")

        for match in SOURCE_NAME_RE.finditer(text):
            brace = text.find("{", match.end())
            if brace == -1:
                continue
            signature = text[match.start() : brace]
            body = extract_braced_block(text, brace)
            fn_name = match.group("name")
            status, reasons = classify_source_candidate(fn_name, signature, body, adopted_owners)
            rows.append(
                {
                    "status": status,
                    "function": fn_name,
                    "path": rel(path),
                    "line": line_number(text, match.start()),
                    "candidate_kind": (
                        "boolean_classifier"
                        if fn_name.startswith("is_")
                        else "label_or_vocabulary_surface"
                    ),
                    "reject_reasons": reasons,
                }
            )

        for match in IMPL_DISPLAY_RE.finditer(text):
            rows.append(
                {
                    "status": "candidate",
                    "function": "Display::fmt",
                    "path": rel(path),
                    "line": line_number(text, match.start()),
                    "candidate_kind": "display_formatter",
                    "display_target": match.group("target"),
                    "reject_reasons": [],
                }
            )

    return sorted(rows, key=lambda row: (row["status"], row["path"], row["line"], row["function"]))


def summarize_existing(rows: list[dict[str, Any]]) -> dict[str, Any]:
    by_status: dict[str, int] = {}
    by_kind: dict[str, int] = {}
    for row in rows:
        by_status[row["status"]] = by_status.get(row["status"], 0) + 1
        kind = row["candidate_kind"]
        by_kind[kind] = by_kind.get(kind, 0) + 1
    return {
        "owner_count": len(rows),
        "by_status": dict(sorted(by_status.items())),
        "by_candidate_kind": dict(sorted(by_kind.items())),
        "incomplete_owner_count": sum(1 for row in rows if row["missing"]),
    }


def summarize_source(rows: list[dict[str, Any]]) -> dict[str, Any]:
    by_status: dict[str, int] = {}
    by_reason: dict[str, int] = {}
    for row in rows:
        by_status[row["status"]] = by_status.get(row["status"], 0) + 1
        for reason in row["reject_reasons"]:
            by_reason[reason] = by_reason.get(reason, 0) + 1
    return {
        "surface_count": len(rows),
        "by_status": dict(sorted(by_status.items())),
        "by_reject_reason": dict(sorted(by_reason.items())),
    }


def build_inventory() -> dict[str, Any]:
    existing = collect_existing_rows()
    adopted_owners = {row["owner"] for row in existing if row["status"].startswith("adopted")}
    source_candidates = collect_source_candidates(adopted_owners)
    return {
        "schema_version": 0,
        "kind": "MirBuilderNativeOwnerCandidateInventoryV1",
        "token": TOKEN,
        "output_contract": CONTRACT,
        "scope": {
            "existing_artifacts": [
                rel(FIXTURES),
                rel(HAKO_LIB),
                rel(CHECKS),
                rel(PHASE),
            ],
            "source_scan_root": rel(SRC_MIR),
            "source_scan_is_advisory": 1,
        },
        "summary": {
            "existing_artifacts": summarize_existing(existing),
            "source_scan": summarize_source(source_candidates),
        },
        "existing_owner_rows": existing,
        "source_scan_rows": source_candidates,
        "claims": {
            "source_selfhost_claim": 0,
            "hako_adopted_decision": 0,
            "next_owner_selected": 0,
            "rust_deletion": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
        },
    }


def print_summary(inventory: dict[str, Any]) -> None:
    summary = inventory["summary"]
    existing = summary["existing_artifacts"]
    source = summary["source_scan"]
    print(f"output_contract={inventory['output_contract']}")
    print(f"existing_owner_count={existing['owner_count']}")
    print(f"existing_by_status={json.dumps(existing['by_status'], sort_keys=True)}")
    print(f"incomplete_owner_count={existing['incomplete_owner_count']}")
    print(f"source_scan_surface_count={source['surface_count']}")
    print(f"source_scan_by_status={json.dumps(source['by_status'], sort_keys=True)}")
    candidates = [row for row in inventory["source_scan_rows"] if row["status"] == "candidate"]
    print(f"source_scan_candidate_count={len(candidates)}")
    for row in candidates[:20]:
        print(f"candidate={row['path']}:{row['line']}::{row['function']} kind={row['candidate_kind']}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in inventory fixture.")
    parser.add_argument("--summary", action="store_true", help="Print a concise text summary.")
    args = parser.parse_args()

    inventory = build_inventory()
    rendered = stable_json(inventory)

    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != rendered:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-native-owner-candidate-inventory unchanged")
        return 0

    changed = write_if_changed(OUTPUT, rendered)
    if args.summary:
        print_summary(inventory)
    else:
        print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
