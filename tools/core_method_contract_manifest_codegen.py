#!/usr/bin/env python3
"""Generate the CoreMethodContract manifest from the .hako owner box.

The .hako file remains the contract owner. This script only reads the narrow
seed-row shape used by CoreMethodContractBox and emits a derived JSON manifest.
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
        return split_alias_args(expr)
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
    fields = [
        "box",
        "canonical",
        "aliases",
        "arity",
        "effect",
        "core_op",
        "lowering_tier",
        "cold_lowering",
        "runtime_owner",
    ]
    rows: list[dict[str, object]] = []
    for method in row_methods:
        body = extract_block(text, f"{method}()")
        args = extract_call_args(body, "me._row")
        if len(args) != len(fields):
            raise ValueError(f"{method} row arity mismatch: expected {len(fields)}, got {len(args)}")
        row = {field: eval_expr(expr, returns) for field, expr in zip(fields, args)}
        row["status"] = returns.get("status_seed", "seed")
        row["guards"] = []
        row["id"] = f'{row["box"]}.{row["canonical"]}/{row["arity"]}'
        rows.append(row)
    return rows


def generate(source: Path) -> str:
    text = source.read_text(encoding="utf-8")
    fields = parse_schema_fields(text)
    rows = parse_rows(text)
    manifest = {
        "schema": "core_method_contract_manifest/v0",
        "source": str(source.relative_to(ROOT)),
        "fields": fields,
        "row_count": len(rows),
        "rows": rows,
    }
    return json.dumps(manifest, ensure_ascii=False, indent=2) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--source", type=Path, default=DEFAULT_SOURCE)
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--write", action="store_true")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()

    if args.write == args.check:
        parser.error("choose exactly one of --write or --check")

    source = args.source.resolve()
    output = args.output.resolve()
    rendered = generate(source)

    if args.write:
        output.parent.mkdir(parents=True, exist_ok=True)
        output.write_text(rendered, encoding="utf-8")
        print(f"[core-method-contract-manifest] wrote {output.relative_to(ROOT)}")
        return 0

    current = output.read_text(encoding="utf-8") if output.exists() else ""
    if current != rendered:
        print(
            "[core-method-contract-manifest] generated manifest is stale; "
            "run: python3 tools/core_method_contract_manifest_codegen.py --write",
            file=sys.stderr,
        )
        return 1
    print("[core-method-contract-manifest] ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
