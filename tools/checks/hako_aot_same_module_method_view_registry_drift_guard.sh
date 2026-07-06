#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
STATIC_REGISTRY="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_same_module_method_view_registry.inc"
GENERATED_REGISTRY="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_generic_method_route_registry.inc"

python3 - "$STATIC_REGISTRY" "$GENERATED_REGISTRY" <<'PY'
import pathlib
import re
import sys

static_path = pathlib.Path(sys.argv[1])
generated_path = pathlib.Path(sys.argv[2])
static_src = static_path.read_text()
generated_src = generated_path.read_text()

tier_values = {
    "HAKO_LLVMC_LOWERING_PLAN_TIER_NONE": 0,
    "HAKO_LLVMC_LOWERING_PLAN_TIER_HOT_INLINE": 1,
    "HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI": 2,
    "HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME": 3,
    "HAKO_LLVMC_LOWERING_PLAN_TIER_UNSUPPORTED": 4,
}


def clean_field(line):
    line = line.strip()
    if line.endswith(","):
        line = line[:-1].strip()
    return line


def field_value(token):
    if token == "NULL":
        return None
    if token.startswith('"') and token.endswith('"'):
        return token[1:-1]
    return token


def parse_static_rows(src):
    rows = []
    pattern = re.compile(
        r"\{\s*(SAME_MODULE_METHOD_VIEW_STATIC_[A-Z0-9_]+),\s*\{\s*(.*?)\n\s*\},\s*\},",
        re.S,
    )
    for match in pattern.finditer(src):
        row_id = match.group(1)
        fields = [
            field_value(clean_field(line))
            for line in match.group(2).splitlines()
            if clean_field(line)
        ]
        if len(fields) != 14:
            raise SystemExit(f"{row_id}: expected 14 SameModuleMethodViewRouteRule fields, got {len(fields)}")
        tier_name = fields[13]
        if tier_name not in tier_values:
            raise SystemExit(f"{row_id}: unknown tier token {tier_name!r}")
        rows.append((row_id, tuple(fields[:5] + [tier_values[tier_name]])))
    if not rows:
        raise SystemExit("same-module static registry rows not found")
    return rows


def parse_generated_rows(src):
    rows_start = src.find("hako_llvmc_generic_method_route_registry_rows[]")
    if rows_start < 0:
        raise SystemExit("generated generic method registry array not found")
    array_src = src[rows_start:]
    rows = set()
    for block in re.finditer(r"\{\s*(.*?)\n\s*\},", array_src, re.S):
        fields = [
            field_value(clean_field(line))
            for line in block.group(1).splitlines()
            if clean_field(line) and clean_field(line) != "{"
        ]
        if len(fields) != 11:
            continue
        try:
            tier = int(fields[5])
        except (TypeError, ValueError):
            continue
        rows.add(tuple(fields[:5] + [tier]))
    if not rows:
        raise SystemExit("generated generic method registry rows not parsed")
    return rows


static_rows = parse_static_rows(static_src)
generated_rows = parse_generated_rows(generated_src)
missing = [
    (row_id, key)
    for row_id, key in static_rows
    if key not in generated_rows
]
if missing:
    details = "\n".join(f"  {row_id}: {key}" for row_id, key in missing)
    raise SystemExit(f"same-module method view registry drift:\n{details}")

print("output_contract=hako-aot-same-module-method-view-registry-drift-guard-v0")
print(f"static_rows_checked={len(static_rows)}")
print("generated_route_tuple_source=generic_method_route_registry")
print("route_id_core_op_route_kind_helper_proof_tier=green")
print("route_family_unification_claim=0")
print("summary=ok")
PY
