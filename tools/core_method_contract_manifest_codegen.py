#!/usr/bin/env python3
"""Generate CoreMethodContract JSON and Rust views from the .hako owner box.

The .hako file remains the contract owner. This script only reads the narrow
seed-row shape used by CoreMethodContractBox and emits derived artifacts.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_SOURCE = ROOT / "lang/src/runtime/meta/core_method_contract_box.hako"
DEFAULT_OUTPUT = ROOT / "lang/src/runtime/meta/generated/core_method_contract_manifest.json"
DEFAULT_RUST_OUTPUT = ROOT / "src/mir/generated/core_method_contract_rows.rs"
MANIFEST_SCHEMA = "core_method_contract_manifest/v2"

ROW_FIELDS = [
    "box",
    "canonical",
    "aliases",
    "arity",
    "semantic_law",
    "effect",
    "core_op",
    "result_kind",
    "lowering_tier",
    "cold_lowering",
    "runtime_owner",
]
EXPECTED_SCHEMA_FIELDS = [*ROW_FIELDS, "status", "guards"]
RESULT_KINDS = {
    "I64Value",
    "BoolValue",
    "StringValue",
    "NoValue",
    "Dynamic",
}
EFFECTS = {
    "pure_read",
    "mutates_slot",
    "mutates_shape",
}
CORE_OPS = {
    "ArrayLen",
    "ArrayGet",
    "ArrayHas",
    "ArraySet",
    "ArrayPush",
    "AnyGet",
    "AnyHas",
    "MapGet",
    "MapSet",
    "MapHas",
    "MapDelete",
    "MapLen",
    "MapKeys",
    "AnyLen",
    "StringLen",
    "StringSubstring",
    "StringIndexOf",
    "StringLastIndexOf",
    "StringContains",
    "StringEquals",
}
EFFECT_RUST_VARIANTS = {
    "pure_read": "PureRead",
    "mutates_slot": "MutatesSlot",
    "mutates_shape": "MutatesShape",
}
LOWERING_TIER_RUST_VARIANTS = {
    "design_only": "DesignOnly",
    "hot_inline": "HotInline",
    "warm_direct_abi": "WarmDirectAbi",
    "cold_fallback": "ColdFallback",
}
SEMANTIC_LAW_RUST_VARIANTS = {
    "Unprojected": "Unprojected",
    "CodePointCount": "CodePointCount",
    "CodePointHalfOpenClamped": "CodePointHalfOpenClamped",
}
SEMANTIC_LAWS = set(SEMANTIC_LAW_RUST_VARIANTS)


def extract_block(text: str, marker: str) -> str:
    start = text.find(marker)
    if start < 0:
        raise ValueError(f"missing block marker: {marker}")
    brace = text.find("{", start)
    if brace < 0:
        raise ValueError(f"missing block body for marker: {marker}")
    depth = 0
    in_string = False
    escape = False
    for idx in range(brace, len(text)):
        ch = text[idx]
        if in_string:
            if escape:
                escape = False
            elif ch == "\\":
                escape = True
            elif ch == '"':
                in_string = False
            continue
        if ch == '"':
            in_string = True
            continue
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                return text[brace + 1 : idx]
    raise ValueError(f"unterminated block: {marker}")


def extract_call_args(body: str, call: str) -> list[str]:
    start = body.find(call)
    if start < 0:
        raise ValueError(f"missing call: {call}")
    open_paren = body.find("(", start)
    if open_paren < 0:
        raise ValueError(f"missing call paren: {call}")
    depth = 0
    in_string = False
    escape = False
    arg_start = open_paren + 1
    args: list[str] = []
    for idx in range(open_paren, len(body)):
        ch = body[idx]
        if in_string:
            if escape:
                escape = False
            elif ch == "\\":
                escape = True
            elif ch == '"':
                in_string = False
            continue
        if ch == '"':
            in_string = True
            continue
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
            if depth == 0:
                args.append(body[arg_start:idx].strip())
                return args
        elif ch == "," and depth == 1:
            args.append(body[arg_start:idx].strip())
            arg_start = idx + 1
    raise ValueError(f"unterminated call: {call}")


def split_alias_args(expr: str) -> list[str]:
    return [part.strip().strip('"') for part in extract_call_args(expr, "me._aliases")]


def parse_zero_arg_returns(text: str) -> dict[str, str]:
    returns: dict[str, str] = {}
    pattern = re.compile(r"^\s*([A-Za-z0-9_]+)\(\)\s*\{\s*return\s+\"([^\"]*)\"\s*\}", re.MULTILINE)
    for name, value in pattern.findall(text):
        returns[name] = value
    return returns


def eval_expr(expr: str, returns: dict[str, str]) -> object:
    expr = expr.strip()
    if expr.startswith('"') and expr.endswith('"'):
        return expr[1:-1]
    alias_match = re.fullmatch(r"me\._aliases([0-9]+)\((.*)\)", expr, re.DOTALL)
    if alias_match:
        count = int(alias_match.group(1))
        if count == 0:
            return []
        aliases = split_alias_args(expr)
        if len(aliases) != count:
            raise ValueError(
                f"alias helper count mismatch: helper={count} actual={len(aliases)}"
            )
        return aliases
    law_match = re.fullmatch(r"me\._law([0-9_]+)\((.*)\)", expr, re.DOTALL)
    if law_match:
        arities = tuple(int(part) for part in law_match.group(1).split("_"))
        args = extract_call_args(expr, "me._law")
        if len(args) != len(arities):
            raise ValueError(
                f"semantic law helper count mismatch: helper={len(arities)} actual={len(args)}"
            )
        laws = [eval_expr(arg, returns) for arg in args]
        if any(not isinstance(law, str) for law in laws):
            raise ValueError(f"semantic law helper requires string laws: {expr}")
        return "|".join(f"{arity}={law}" for arity, law in zip(arities, laws))
    call_match = re.fullmatch(r"me\.([A-Za-z0-9_]+)\(\)", expr)
    if call_match:
        name = call_match.group(1)
        if name not in returns:
            raise ValueError(f"unknown zero-arg return method: {name}")
        return returns[name]
    raise ValueError(f"unsupported expression: {expr}")


def parse_schema_fields(text: str) -> list[str]:
    body = extract_block(text, "schema_fields()")
    return re.findall(r'out\.push\("([^"]+)"\)', body)


def parse_rows(text: str) -> list[dict[str, object]]:
    returns = parse_zero_arg_returns(text)
    rows_body = extract_block(text, "rows()")
    row_methods = re.findall(r"out\.push\(me\.([A-Za-z0-9_]+)\(\)\)", rows_body)
    rows: list[dict[str, object]] = []
    for method in row_methods:
        body = extract_block(text, f"{method}()")
        args = extract_call_args(body, "me._row")
        if len(args) != len(ROW_FIELDS):
            raise ValueError(
                f"{method} row arity mismatch: expected {len(ROW_FIELDS)}, got {len(args)}"
            )
        row = {}
        for field, expr in zip(ROW_FIELDS, args):
            value = eval_expr(expr, returns)
            row[field] = parse_semantic_law(value) if field == "semantic_law" else value
        row["status"] = returns.get("status_seed", "seed")
        row["guards"] = []
        row["id"] = f'{row["box"]}.{row["canonical"]}/{row["arity"]}'
        rows.append(row)
    return rows


def parse_semantic_law(value: object) -> dict[str, str]:
    if not isinstance(value, str) or not value:
        raise ValueError(f"invalid semantic_law: {value!r}")
    entries: dict[str, str] = {}
    for raw_entry in value.split("|"):
        parts = raw_entry.split("=", 1)
        if len(parts) != 2 or not parts[0].isdigit() or not parts[1]:
            raise ValueError(f"invalid semantic_law entry: {raw_entry!r}")
        arity, law = parts
        if arity in entries:
            raise ValueError(f"duplicate semantic_law arity: {arity}")
        entries[arity] = law
    return entries


def parse_arities(pattern: object) -> tuple[int, ...]:
    if not isinstance(pattern, str) or not pattern:
        raise ValueError(f"invalid arity pattern: {pattern!r}")
    parts = pattern.split("|")
    if any(not part.isdigit() for part in parts):
        raise ValueError(f"invalid arity pattern: {pattern!r}")
    arities = tuple(int(part) for part in parts)
    if len(set(arities)) != len(arities):
        raise ValueError(f"duplicate arity in pattern: {pattern!r}")
    if arities != tuple(sorted(arities)):
        raise ValueError(f"arity pattern must be sorted: {pattern!r}")
    return arities


def validate_rows(rows: list[dict[str, object]]) -> None:
    seen_ids: set[str] = set()
    selected: dict[tuple[str, str, int], str] = {}
    selected_by_op: dict[tuple[str, str, int], str] = {}
    for row in rows:
        missing = [field for field in [*ROW_FIELDS, "id"] if field not in row]
        if missing:
            raise ValueError(f"CoreMethodContract row is missing fields: {missing!r}")
        row_id = str(row["id"])
        if row_id in seen_ids:
            raise ValueError(f"duplicate row id: {row_id}")
        seen_ids.add(row_id)

        result_kind = row["result_kind"]
        if result_kind not in RESULT_KINDS:
            raise ValueError(f"{row_id} has unknown result_kind: {result_kind!r}")

        effect = row["effect"]
        if effect not in EFFECTS:
            raise ValueError(f"{row_id} has unknown effect: {effect!r}")

        core_op = row["core_op"]
        if core_op not in CORE_OPS:
            raise ValueError(f"{row_id} has unknown core_op: {core_op!r}")

        lowering_tier = row["lowering_tier"]
        if lowering_tier not in LOWERING_TIER_RUST_VARIANTS:
            raise ValueError(f"{row_id} has unknown lowering_tier: {lowering_tier!r}")
        if lowering_tier == "design_only" and row["cold_lowering"] != "none":
            raise ValueError(
                f"{row_id} design_only rows must use cold_lowering=none"
            )

        arities = parse_arities(row["arity"])
        semantic_law = row["semantic_law"]
        if not isinstance(semantic_law, dict):
            raise ValueError(f"{row_id} semantic_law must be an arity map")
        expected_law_arities = [str(arity) for arity in arities]
        if list(semantic_law) != expected_law_arities:
            raise ValueError(
                f"{row_id} semantic_law must exactly cover sorted arities: "
                f"expected={expected_law_arities!r} actual={list(semantic_law)!r}"
            )
        for law_arity, law in semantic_law.items():
            if law not in SEMANTIC_LAWS:
                raise ValueError(
                    f"{row_id} has unknown semantic_law for arity {law_arity}: {law!r}"
                )

        receiver = str(row["box"])
        canonical = str(row["canonical"])
        if not receiver or not canonical:
            raise ValueError(f"{row_id} receiver and canonical spelling must be non-empty")
        aliases = row["aliases"]
        if not isinstance(aliases, list):
            raise ValueError(f"{row_id} aliases must be a list")
        spellings = [canonical, *(str(alias) for alias in aliases)]
        if any(not spelling for spelling in spellings):
            raise ValueError(f"{row_id} contains an empty method spelling")
        for arity in arities:
            op_key = (receiver, str(row["core_op"]), arity)
            previous_by_op = selected_by_op.get(op_key)
            if previous_by_op is not None:
                raise ValueError(
                    "CoreMethodContract operation collision: "
                    f"receiver={receiver} core_op={row['core_op']} arity={arity} "
                    f"rows={previous_by_op},{row_id}"
                )
            selected_by_op[op_key] = row_id

            for spelling in spellings:
                key = (receiver, spelling, arity)
                previous = selected.get(key)
                if previous is not None:
                    raise ValueError(
                        "CoreMethodContract spelling collision: "
                        f"receiver={receiver} spelling={spelling} arity={arity} "
                        f"rows={previous},{row_id}"
                    )
                selected[key] = row_id


def load_contract(source: Path) -> tuple[list[str], list[dict[str, object]]]:
    text = source.read_text(encoding="utf-8")
    fields = parse_schema_fields(text)
    if fields != EXPECTED_SCHEMA_FIELDS:
        raise ValueError(
            "CoreMethodContract schema fields drift: "
            f"expected={EXPECTED_SCHEMA_FIELDS!r} actual={fields!r}"
        )
    rows = parse_rows(text)
    validate_rows(rows)
    return fields, rows


def generate_json(source: Path, fields: list[str], rows: list[dict[str, object]]) -> str:
    manifest = {
        "schema": MANIFEST_SCHEMA,
        "source": str(source.relative_to(ROOT)),
        "fields": fields,
        "row_count": len(rows),
        "rows": rows,
    }
    return json.dumps(manifest, ensure_ascii=False, indent=2) + "\n"


def rust_string(value: object) -> str:
    return json.dumps(str(value), ensure_ascii=False)


def generate_rust(source: Path, rows: list[dict[str, object]]) -> str:
    lines = [
        "// @generated by tools/core_method_contract_manifest_codegen.py",
        f"// source: {source.relative_to(ROOT)}",
        "",
        "use crate::mir::core_method_op::{CoreMethodLoweringTier, CoreMethodOp};",
        "use crate::mir::core_method_result_kind::{",
        "    CoreMethodContractRowV2, CoreMethodEffectV1, CoreMethodResultKindV1, CoreMethodSemanticLawV2,",
        "};",
        "",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]",
        "pub(crate) struct CoreMethodManifestBrandV2 {",
        "    schema: &'static str,",
        "}",
        "",
        "impl CoreMethodManifestBrandV2 {",
        "    pub(crate) const fn schema(self) -> &'static str {",
        "        self.schema",
        "    }",
        "}",
        "",
        "pub(crate) const CORE_METHOD_MANIFEST_BRAND_V2: CoreMethodManifestBrandV2 =",
        "    CoreMethodManifestBrandV2 {",
        f"        schema: {rust_string(MANIFEST_SCHEMA)},",
        "    };",
        "",
        "#[cfg(test)]",
        "pub(crate) const CORE_METHOD_MANIFEST_FOREIGN_BRAND_FOR_TEST: CoreMethodManifestBrandV2 =",
        "    CoreMethodManifestBrandV2 {",
        '        schema: "foreign/core_method_contract_manifest",',
        "    };",
        "",
        "pub(crate) const CORE_METHOD_CONTRACT_ROWS_V2: &[CoreMethodContractRowV2] = &[",
    ]
    for row in rows:
        aliases = ", ".join(rust_string(alias) for alias in row["aliases"])
        arities = ", ".join(str(arity) for arity in parse_arities(row["arity"]))
        law_entries = [
            f"({arity}, CoreMethodSemanticLawV2::{SEMANTIC_LAW_RUST_VARIANTS[law]})"
            for arity, law in row["semantic_law"].items()
        ]
        semantic_law_lines = (
            [f"        semantic_law: &[{law_entries[0]}],"]
            if len(law_entries) == 1
            else [
                "        semantic_law: &[",
                *(f"            {entry}," for entry in law_entries),
                "        ],",
            ]
        )
        lines.extend(
            [
                "    CoreMethodContractRowV2 {",
                f"        receiver_box: {rust_string(row['box'])},",
                f"        canonical: {rust_string(row['canonical'])},",
                f"        aliases: &[{aliases}],",
                f"        arities: &[{arities}],",
                *semantic_law_lines,
                f"        op: CoreMethodOp::{row['core_op']},",
                f"        result_kind: CoreMethodResultKindV1::{row['result_kind']},",
                f"        effect: CoreMethodEffectV1::{EFFECT_RUST_VARIANTS[row['effect']]},",
                f"        lowering_tier: CoreMethodLoweringTier::{LOWERING_TIER_RUST_VARIANTS[row['lowering_tier']]},",
                "    },",
            ]
        )
    lines.extend(["];", ""])
    return "\n".join(lines)


def generate(source: Path) -> tuple[str, str]:
    fields, rows = load_contract(source)
    return generate_json(source, fields, rows), generate_rust(source, rows)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--source", type=Path, default=DEFAULT_SOURCE)
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--rust-output", type=Path, default=DEFAULT_RUST_OUTPUT)
    parser.add_argument("--write", action="store_true")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()

    if args.write == args.check:
        parser.error("choose exactly one of --write or --check")

    source = args.source.resolve()
    output = args.output.resolve()
    rust_output = args.rust_output.resolve()
    rendered, rendered_rust = generate(source)

    if args.write:
        output.parent.mkdir(parents=True, exist_ok=True)
        output.write_text(rendered, encoding="utf-8")
        rust_output.parent.mkdir(parents=True, exist_ok=True)
        rust_output.write_text(rendered_rust, encoding="utf-8")
        print(
            "[core-method-contract-manifest] wrote "
            f"{output.relative_to(ROOT)} and {rust_output.relative_to(ROOT)}"
        )
        return 0

    current = output.read_text(encoding="utf-8") if output.exists() else ""
    current_rust = rust_output.read_text(encoding="utf-8") if rust_output.exists() else ""
    if current != rendered or current_rust != rendered_rust:
        print(
            "[core-method-contract-manifest] generated artifacts are stale; "
            "run: python3 tools/core_method_contract_manifest_codegen.py --write",
            file=sys.stderr,
        )
        return 1
    print("[core-method-contract-manifest] ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
