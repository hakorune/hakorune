#!/usr/bin/env python3
"""Generate the normalized TextScan ProviderSlot manifest.

The Hako box is the semantic grouping owner.  CoreMethodContractBox remains
the sole result/effect owner; this manifest intentionally contains neither
field.  The generated JSON is a checked projection, not a provider registry.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_SOURCE = ROOT / "lang/src/runtime/meta/provider_slot_contract_box.hako"
DEFAULT_OUTPUT = ROOT / "lang/src/runtime/meta/generated/provider_slot_contract_manifest.json"

ROLE_FIELDS = ["role", "core_op", "dispatch", "arity", "lifecycle", "policy"]
SCHEMA_FIELDS = ["contract_id", "profile", "receiver", "suspension", "roles"]
EXPECTED_ROLES = {
    "TextSliceRange": {
        "core_op": "StringSubstring",
        "dispatch": "substring",
        "arity": "2",
        "lifecycle": "end_authorized",
        "policy": "codepoint_half_open_clamped",
    },
    "TextFindNeedle": {
        "core_op": "StringIndexOf",
        "dispatch": "indexOf",
        "arity": "1",
        "lifecycle": "none",
        "policy": "codepoint_first_index_or_minus_one",
    },
}


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
    for index in range(brace, len(text)):
        char = text[index]
        if in_string:
            if escape:
                escape = False
            elif char == "\\":
                escape = True
            elif char == '"':
                in_string = False
            continue
        if char == '"':
            in_string = True
        elif char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return text[brace + 1 : index]
    raise ValueError(f"unterminated block: {marker}")


def extract_call_args(body: str, call: str) -> list[str]:
    start = body.find(call)
    if start < 0:
        raise ValueError(f"missing call: {call}")
    opening = body.find("(", start)
    if opening < 0:
        raise ValueError(f"missing call paren: {call}")
    depth = 0
    in_string = False
    escape = False
    arg_start = opening + 1
    args: list[str] = []
    for index in range(opening, len(body)):
        char = body[index]
        if in_string:
            if escape:
                escape = False
            elif char == "\\":
                escape = True
            elif char == '"':
                in_string = False
            continue
        if char == '"':
            in_string = True
        elif char == "(":
            depth += 1
        elif char == ")":
            depth -= 1
            if depth == 0:
                args.append(body[arg_start:index].strip())
                return args
        elif char == "," and depth == 1:
            args.append(body[arg_start:index].strip())
            arg_start = index + 1
    raise ValueError(f"unterminated call: {call}")


def parse_returns(text: str) -> dict[str, str]:
    pattern = re.compile(
        r"^\s*([A-Za-z0-9_]+)\(\)\s*\{\s*return\s+\"([^\"]*)\"\s*\}",
        re.MULTILINE,
    )
    return {name: value for name, value in pattern.findall(text)}


def parse_pushed_strings(text: str, marker: str) -> list[str]:
    body = extract_block(text, marker)
    return re.findall(r'out\.push\("([^"]+)"\)', body)


def parse_value(expr: str, returns: dict[str, str]) -> str:
    expr = expr.strip()
    if expr.startswith('"') and expr.endswith('"'):
        return expr[1:-1]
    match = re.fullmatch(r"me\.([A-Za-z0-9_]+)\(\)", expr)
    if match and match.group(1) in returns:
        return returns[match.group(1)]
    raise ValueError(f"unsupported contract expression: {expr}")


def parse_contract(source: Path) -> tuple[dict[str, str], list[dict[str, str]]]:
    text = source.read_text(encoding="utf-8")
    returns = parse_returns(text)
    fields = parse_pushed_strings(text, "contract_fields()")
    if fields != SCHEMA_FIELDS:
        raise ValueError(f"schema fields drift: expected={SCHEMA_FIELDS!r} actual={fields!r}")

    metadata = {
        field: returns.get(field)
        for field in ("contract_id", "profile", "receiver", "suspension")
    }
    if any(value is None for value in metadata.values()):
        raise ValueError("contract metadata must use literal zero-argument returns")

    roles_body = extract_block(text, "roles()")
    methods = re.findall(r"out\.push\(me\.([A-Za-z0-9_]+)\(\)\)", roles_body)
    roles: list[dict[str, str]] = []
    for method in methods:
        body = extract_block(text, f"{method}()")
        args = extract_call_args(body, "me._role")
        if len(args) != len(ROLE_FIELDS):
            raise ValueError(f"{method} role arity mismatch")
        roles.append({field: parse_value(expr, returns) for field, expr in zip(ROLE_FIELDS, args)})
    validate_contract(metadata, roles)
    return metadata, roles


def validate_contract(metadata: dict[str, str], roles: list[dict[str, str]]) -> None:
    expected_metadata = {
        "contract_id": "hako.text.scan@1",
        "profile": "utf8-codepoint-clamped-v1",
        "receiver": "Text",
        "suspension": "non_suspending",
    }
    if metadata != expected_metadata:
        raise ValueError(f"contract metadata drift: expected={expected_metadata!r} actual={metadata!r}")
    if len(roles) != len(EXPECTED_ROLES):
        raise ValueError(f"TextScan role count must be {len(EXPECTED_ROLES)}")
    seen_roles: set[str] = set()
    seen_ops: set[tuple[str, str]] = set()
    seen_dispatch: set[tuple[str, str]] = set()
    for role in roles:
        missing = [field for field in ROLE_FIELDS if field not in role]
        if missing:
            raise ValueError(f"role missing fields: {missing!r}")
        name = role["role"]
        if name in seen_roles:
            raise ValueError(f"duplicate TextScan role: {name}")
        seen_roles.add(name)
        expected = EXPECTED_ROLES.get(name)
        if expected is None or {key: role[key] for key in ROLE_FIELDS[1:]} != expected:
            raise ValueError(f"unexpected TextScan role contract: {role!r}")
        op_key = (role["core_op"], role["arity"])
        if op_key in seen_ops:
            raise ValueError(f"duplicate TextScan CoreMethod operation: {op_key!r}")
        seen_ops.add(op_key)
        dispatch_key = (role["dispatch"], role["arity"])
        if dispatch_key in seen_dispatch:
            raise ValueError(f"duplicate TextScan dispatch: {dispatch_key!r}")
        seen_dispatch.add(dispatch_key)


def generate(source: Path) -> str:
    metadata, roles = parse_contract(source)
    manifest = {
        "schema": "provider_slot_contract_manifest/v1",
        "source": str(source.relative_to(ROOT)),
        "fields": SCHEMA_FIELDS,
        **metadata,
        "role_count": len(roles),
        "roles": roles,
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
        print(f"[provider-slot-contract-manifest] wrote {output.relative_to(ROOT)}")
        return 0

    current = output.read_text(encoding="utf-8") if output.exists() else ""
    if current != rendered:
        print(
            "[provider-slot-contract-manifest] generated artifact is stale; "
            "run: python3 tools/provider_slot_contract_manifest_codegen.py --write",
            file=sys.stderr,
        )
        return 1
    print("[provider-slot-contract-manifest] ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
