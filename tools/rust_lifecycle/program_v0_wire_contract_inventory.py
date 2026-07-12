#!/usr/bin/env python3
"""Generate the ProgramV0 producer/consumer wire contract inventory."""

from __future__ import annotations

import argparse
import re
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed

ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/program-v0-wire-contract-inventory-v0.json"
CARD = ROOT / "docs/development/current/main/investigations/mirbuilder-hako-bounded-body-analysis-snapshot-v0-2026-07-12.md"
CONSUMER = ROOT / "src/runner/json_v0_bridge/ast.rs"
CONSUMER_ENTRY = ROOT / "src/runner/json_v0_bridge/core.rs"
INT_CONSUMER = ROOT / "src/runner/json_v0_bridge/lowering/expr.rs"
PRODUCERS = [
    ROOT / "src/stage1/program_json_v0/lowering.rs",
    ROOT / "src/stage1/program_json_v0/lowering/statements.rs",
    ROOT / "src/stage1/program_json_v0/lowering/expr_support.rs",
    ROOT / "src/stage1/program_json_v0/lowering/typed_array.rs",
]

TOKEN = "MIRBUILDER-PROGRAMV0-WIRE-CONTRACT-INVENTORY-V0-001"

STMT_ROWS = {
    "Return": (1, 1, "Accepted"),
    "Extern": (0, 1, "KnownUnsupported"),
    "Expr": (1, 1, "Accepted"),
    "Local": (1, 1, "Accepted"),
    "If": (1, 1, "Accepted"),
    "Loop": (1, 1, "Accepted"),
    "LoopRange": (1, 1, "Accepted"),
    "TaskScope": (1, 1, "KnownUnsupported"),
    "FastMemRegion": (1, 0, "SchemaMismatchStop"),
    "Throw": (1, 1, "KnownUnsupported"),
    "Break": (1, 1, "Accepted"),
    "Continue": (1, 1, "Accepted"),
    "Try": (1, 1, "KnownUnsupported"),
    "FiniReg": (0, 1, "KnownUnsupported"),
}

EXPR_ROWS = {
    "Int": (1, 1, "Accepted"),
    "Float": (1, 0, "SchemaMismatchStop"),
    "Str": (1, 1, "Accepted"),
    "Bool": (1, 1, "Accepted"),
    "Null": (1, 1, "Accepted"),
    "Binary": (1, 1, "Accepted"),
    "Extern": (0, 1, "KnownUnsupported"),
    "Compare": (1, 1, "Accepted"),
    "Logical": (1, 1, "Accepted"),
    "Call": (1, 1, "Accepted"),
    "ArrayLiteral": (1, 1, "KnownUnsupported"),
    "Method": (1, 1, "Accepted"),
    "Field": (1, 1, "Accepted"),
    "New": (1, 1, "KnownUnsupported"),
    "Var": (1, 1, "Accepted"),
    "Throw": (0, 1, "KnownUnsupported"),
    "BlockExpr": (1, 1, "KnownUnsupported"),
    "Ternary": (0, 1, "KnownUnsupported"),
    "Match": (1, 1, "KnownUnsupported"),
    "EnumCtor": (1, 1, "KnownUnsupported"),
    "EnumMatch": (1, 1, "KnownUnsupported"),
    "BrandConstruct": (1, 0, "SchemaMismatchStop"),
    "BrandUnwrap": (1, 0, "SchemaMismatchStop"),
    "RecordField": (1, 0, "SchemaMismatchStop"),
    "RecordLiteral": (1, 0, "SchemaMismatchStop"),
    "RecordUpdate": (1, 0, "SchemaMismatchStop"),
}


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def evidence(path: Path) -> dict[str, str]:
    return {"path": rel(path), "sha256": sha256_file(path)}


def enum_variants(source: str, enum_name: str) -> set[str]:
    marker = f"enum {enum_name} {{"
    start = source.index(marker) + len(marker)
    depth = 1
    variants: set[str] = set()
    for line in source[start:].splitlines():
        stripped = line.strip()
        if depth == 1:
            match = re.match(r"([A-Z][A-Za-z0-9_]*)\s*(?:\{|,)", stripped)
            if match:
                variants.add(match.group(1))
        depth += line.count("{") - line.count("}")
        if depth == 0:
            break
    return variants


def producer_tags() -> set[str]:
    tags: set[str] = set()
    pattern = re.compile(r'"type"\s*:\s*"([A-Za-z][A-Za-z0-9_]*)"')
    for path in PRODUCERS:
        tags.update(pattern.findall(path.read_text(encoding="utf-8")))
    return tags


def validate_inventory() -> None:
    consumer_source = CONSUMER.read_text(encoding="utf-8")
    actual_stmt = enum_variants(consumer_source, "StmtV0")
    actual_expr = enum_variants(consumer_source, "ExprV0")
    expected_stmt = {tag for tag, (_, consumer, _) in STMT_ROWS.items() if consumer}
    expected_expr = {tag for tag, (_, consumer, _) in EXPR_ROWS.items() if consumer}
    if actual_stmt != expected_stmt:
        raise SystemExit(f"StmtV0 inventory drift: actual={sorted(actual_stmt)} expected={sorted(expected_stmt)}")
    if actual_expr != expected_expr:
        raise SystemExit(f"ExprV0 inventory drift: actual={sorted(actual_expr)} expected={sorted(expected_expr)}")
    expected_producer = {
        tag
        for rows in (STMT_ROWS, EXPR_ROWS)
        for tag, (producer, _, _) in rows.items()
        if producer
    }
    actual_producer = producer_tags()
    if actual_producer != expected_producer:
        raise SystemExit(
            f"producer tag inventory drift: actual={sorted(actual_producer)} expected={sorted(expected_producer)}"
        )


def rows(domain: str, inventory: dict[str, tuple[int, int, str]]) -> list[dict[str, Any]]:
    return [
        {
            "domain": domain,
            "tag": tag,
            "producer_emittable": producer,
            "consumer_decodable": consumer,
            "classification": classification,
        }
        for tag, (producer, consumer, classification) in sorted(inventory.items())
    ]


def build_fixture() -> dict[str, Any]:
    validate_inventory()
    return {
        "schema_version": 0,
        "kind": "ProgramV0WireContractInventoryV0",
        "token": TOKEN,
        "rows": rows("stmt", STMT_ROWS) + rows("expr", EXPR_ROWS),
        "field_seams": [
            {"path": "$.body[].declared_type", "owner": "Local", "classification": "known-but-unobserved"},
            {"path": "$.body[].expr.declared_type", "owner": "Int", "classification": "known-but-unobserved"},
            {"path": "$.body[].expr.field_initializers", "owner": "New", "classification": "known-but-unobserved"},
            {"path": "$.brand_decls", "owner": "ProgramV0", "classification": "producer-only-extra"},
            {"path": "$.type_alias_decls", "owner": "ProgramV0", "classification": "producer-only-extra"},
            {"path": "$.defs[].uses", "owner": "FuncDefV0", "classification": "producer-only-extra"},
            {"path": "$.defs[].contracts", "owner": "FuncDefV0", "classification": "producer-only-extra"},
        ],
        "parser_seams": {
            "json_syntax": "strict",
            "full_input_consumption": "strict_non_whitespace",
            "known_duplicate_fields": "rejected",
            "unknown_duplicate_fields": "unproven",
            "extra_fields": "tolerated_and_discarded",
            "deny_unknown_fields": 0,
            "int_value_decode": "serde_json_value_then_i64_number_or_decimal_string",
            "known_unsupported_vs_malformed": "not_distinguished",
        },
        "operators": {
            "Binary": ["+", "-", "*", "/", "%", "&", "|", "^", "<<", ">>"],
            "Compare": ["==", "!=", "<", ">", "<=", ">="],
            "Logical": ["&&", "||"],
        },
        "claims": {
            "snapshot_implementation_started": 0,
            "program_json_schema_changed": 0,
            "source_kind_recovery": 0,
            "mir_or_id_allocation": 0,
            "planner_or_route_authority": 0,
            "all_variants_classified": 1,
            "schema_mismatch_stops": 7,
        },
        "provenance": {
            "card": evidence(CARD),
            "consumer": evidence(CONSUMER),
            "consumer_entry": evidence(CONSUMER_ENTRY),
            "int_consumer": evidence(INT_CONSUMER),
            "producers": [evidence(path) for path in PRODUCERS],
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("program-v0 wire contract inventory unchanged")
        return 0
    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
