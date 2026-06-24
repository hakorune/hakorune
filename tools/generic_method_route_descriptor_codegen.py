#!/usr/bin/env python3
"""Generate generic-method route descriptor tables from one manifest."""

from __future__ import annotations

import argparse
import difflib
import sys
import tomllib
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
SPEC = ROOT / "spec/mir/generic_method_routes.toml"
RUST_OUT = ROOT / "src/mir/generated/generic_method_route_descriptors.rs"
C_OUT = ROOT / "lang/c-abi/shims/hako_llvmc_ffi_generic_method_route_registry.inc"
PY_OUT = ROOT / "src/llvm_py/generated/generic_method_route_registry.py"

ROUTE_KIND_VARIANTS = {
    "runtime_data_load_any": "RuntimeDataLoadAny",
    "runtime_data_contains_any": "RuntimeDataContainsAny",
    "map_load_scalar_i64": "MapLoadScalarI64",
    "map_load_i64_any": "MapLoadI64Any",
    "map_load_any": "MapLoadAny",
    "map_entry_count": "MapEntryCount",
    "map_keys_array": "MapKeysArray",
    "any_length": "AnyLength",
    "array_slot_load_any": "ArraySlotLoadAny",
    "array_contains_any": "ArrayContainsAny",
    "array_slot_len": "ArraySlotLen",
    "array_append_any": "ArrayAppendAny",
    "array_store_any": "ArrayStoreAny",
    "map_store_i64": "MapStoreI64",
    "map_store_any": "MapStoreAny",
    "map_delete_any": "MapDeleteAny",
    "string_len": "StringLen",
    "string_substring": "StringSubstring",
    "string_indexof": "StringIndexOf",
    "string_last_indexof": "StringLastIndexOf",
    "string_contains": "StringContains",
    "map_contains_any": "MapContainsAny",
    "map_contains_i64": "MapContainsI64",
}

C_EMIT_KIND_VALUES = {
    "none": 0,
    "set": 1,
    "get": 2,
    "len": 3,
    "push": 4,
    "substring": 5,
    "has": 6,
    "keys": 7,
    "delete": 0,
    "indexOf": 0,
    "lastIndexOf": 0,
    "contains": 0,
}

C_NEED_KIND_VALUES = {
    "none": 0,
    "map_set": 2,
    "map_size": 3,
    "map_get": 4,
    "map_has": 5,
    "map_delete": 6,
    "map_keys": 7,
    "runtime_data_has": 8,
    "array_push": 10,
    "array_len": 11,
    "array_set": 12,
    "array_get": 13,
    "array_has": 14,
    "string_len": 16,
    "string_substring": 17,
    "string_indexof": 19,
    "string_contains": 20,
    "any_length": 22,
}


def load_spec() -> dict[str, Any]:
    with SPEC.open("rb") as fh:
        data = tomllib.load(fh)
    if data.get("schema_version") != 0:
        raise SystemExit("unsupported generic method route schema_version")
    routes = data.get("routes")
    rows = data.get("c_registry_rows")
    if not isinstance(routes, list) or not isinstance(rows, list):
        raise SystemExit("manifest must define routes and c_registry_rows arrays")
    seen: set[str] = set()
    for route in routes:
        kind = str(route.get("kind", ""))
        if kind not in ROUTE_KIND_VARIANTS:
            raise SystemExit(f"unknown route kind in manifest: {kind}")
        if kind in seen:
            raise SystemExit(f"duplicate route kind in manifest: {kind}")
        seen.add(kind)
    missing = set(ROUTE_KIND_VARIANTS) - seen
    if missing:
        raise SystemExit(f"missing route descriptors: {sorted(missing)}")
    route_by_kind = {str(route["kind"]): route for route in routes}
    for row in rows:
        route_kind = str(row.get("route_kind", ""))
        route = route_by_kind.get(route_kind)
        if route is None:
            raise SystemExit(f"c_registry_row references unknown route_kind: {route_kind}")
        normalized = normalize_c_registry_row(row, route)
        row.clear()
        row.update(normalized)
    return data


def check_if_present(row: dict[str, Any], field: str, expected: Any) -> None:
    if field in row and row[field] != expected:
        raise SystemExit(
            f"c_registry_row {row.get('core_op')}/{row.get('route_kind')} "
            f"has {field}={row[field]!r}, expected {expected!r}"
        )


def route_emit_kind_value(route: dict[str, Any]) -> int:
    emit_kind = str(route.get("emit_kind", ""))
    if emit_kind not in C_EMIT_KIND_VALUES:
        raise SystemExit(f"route {route.get('kind')} has no C emit mapping: {emit_kind}")
    return C_EMIT_KIND_VALUES[emit_kind]


def route_need_kind_value(route: dict[str, Any]) -> int:
    need_kind = str(route.get("c_need_kind", ""))
    if need_kind not in C_NEED_KIND_VALUES:
        raise SystemExit(f"route {route.get('kind')} has no C need mapping: {need_kind}")
    return C_NEED_KIND_VALUES[need_kind]


def normalize_c_registry_row(row: dict[str, Any], route: dict[str, Any]) -> dict[str, Any]:
    route_id = str(route["route_id"])
    route_helper_symbol = str(route["helper_symbol"])
    helper_symbol = resolve_c_row_helper_symbol(row, route)
    tier = int(route["tier"])
    emit_kind = route_emit_kind_value(route)
    need_kind = route_need_kind_value(route)
    check_if_present(row, "route_id", route_id)
    if helper_symbol is not None and "*" not in route_helper_symbol:
        check_if_present(row, "helper_symbol", route_helper_symbol)
    check_if_present(row, "tier", tier)
    check_if_present(row, "emit_kind", emit_kind)
    check_if_present(row, "need_kind", need_kind)
    return {
        "route_id": route_id,
        "core_op": row["core_op"],
        "route_kind": route["kind"],
        "helper_symbol": helper_symbol,
        "route_proof": row.get("route_proof"),
        "tier": tier,
        "route_result": row["route_result"],
        "emit_kind": emit_kind,
        "need_kind": need_kind,
    }


def resolve_c_row_helper_symbol(row: dict[str, Any], route: dict[str, Any]) -> str | None:
    variant_key = row.get("c_helper_variant")
    if variant_key is not None:
        variants = route.get("c_helper_variants")
        if not isinstance(variants, list):
            raise SystemExit(
                f"c_registry_row {row.get('core_op')}/{row.get('route_kind')} "
                "uses c_helper_variant but route has no c_helper_variants"
            )
        for variant in variants:
            if variant.get("key") == variant_key:
                return str(variant["helper_symbol"])
        raise SystemExit(
            f"c_registry_row {row.get('core_op')}/{row.get('route_kind')} "
            f"has unknown c_helper_variant={variant_key!r}"
        )
    helper_symbol = row.get("helper_symbol")
    if helper_symbol is None:
        return None
    route_helper_symbol = str(route["helper_symbol"])
    if "*" not in route_helper_symbol:
        check_if_present(row, "helper_symbol", route_helper_symbol)
        return str(helper_symbol)
    for variant in route.get("c_helper_variants", []):
        if variant.get("helper_symbol") == helper_symbol:
            return str(helper_symbol)
    raise SystemExit(
        f"c_registry_row {row.get('core_op')}/{row.get('route_kind')} "
        f"uses helper_symbol={helper_symbol!r} not listed in c_helper_variants"
    )


def q(value: str | None) -> str:
    if value is None:
        return "None"
    return f'Some("{value}")'


def c_q(value: str | None) -> str:
    if value is None:
        return "NULL"
    escaped = value.replace("\\", "\\\\").replace('"', '\\"')
    return f'"{escaped}"'


def py_q(value: str | None) -> str:
    return repr(value)


def render_rust(data: dict[str, Any]) -> str:
    lines = [
        "// @generated by tools/generic_method_route_descriptor_codegen.py",
        "// source: spec/mir/generic_method_routes.toml",
        "",
        "use crate::mir::generic_method_route_plan::GenericMethodRouteKind;",
        "",
        "#[allow(dead_code)]",
        "#[derive(Debug, Clone, Copy)]",
        "pub(crate) struct GenericMethodRouteDescriptor {",
        "    pub(crate) kind: GenericMethodRouteKind,",
        "    pub(crate) tag: &'static str,",
        "    pub(crate) route_id: &'static str,",
        "    pub(crate) emit_kind: &'static str,",
        "    pub(crate) helper_symbol: &'static str,",
        "    pub(crate) tier: i32,",
        "    pub(crate) return_shape: Option<&'static str>,",
        "    pub(crate) value_demand: &'static str,",
        "    pub(crate) publication_policy: Option<&'static str>,",
        "    pub(crate) effects: &'static [&'static str],",
        "}",
        "",
        "pub(crate) fn descriptor_for_route_kind(",
        "    kind: GenericMethodRouteKind,",
        ") -> GenericMethodRouteDescriptor {",
        "    match kind {",
    ]
    for route in data["routes"]:
        kind = str(route["kind"])
        variant = ROUTE_KIND_VARIANTS[kind]
        effects = ", ".join(f'"{effect}"' for effect in route.get("effects", []))
        lines.extend(
            [
                f"        GenericMethodRouteKind::{variant} => GenericMethodRouteDescriptor {{",
                f"            kind: GenericMethodRouteKind::{variant},",
                f'            tag: "{kind}",',
                f'            route_id: "{route["route_id"]}",',
                f'            emit_kind: "{route["emit_kind"]}",',
                f'            helper_symbol: "{route["helper_symbol"]}",',
                f"            tier: {int(route['tier'])},",
                f"            return_shape: {q(route.get('return_shape'))},",
                f'            value_demand: "{route["value_demand"]}",',
                f"            publication_policy: {q(route.get('publication_policy'))},",
                f"            effects: &[{effects}],",
                "        },",
            ]
        )
    lines.extend(["    }", "}", ""])
    return "\n".join(lines)


def render_c(data: dict[str, Any]) -> str:
    lines = [
        "/* @generated by tools/generic_method_route_descriptor_codegen.py */",
        "/* source: spec/mir/generic_method_routes.toml */",
        "#ifndef HAKO_LLVMC_GENERIC_METHOD_ROUTE_REGISTRY_INC",
        "#define HAKO_LLVMC_GENERIC_METHOD_ROUTE_REGISTRY_INC",
        "",
        "      struct HakoLlvmcGenericMethodRouteRegistryRow {",
        "        const char* route_id;",
        "        const char* core_op;",
        "        const char* route_kind;",
        "        const char* helper_symbol;",
        "        const char* route_proof;",
        "        int tier;",
        "        int route_result;",
        "        int emit_kind;",
        "        int need_kind;",
        "      };",
        "",
        "      static const struct HakoLlvmcGenericMethodRouteRegistryRow",
        "          hako_llvmc_generic_method_route_registry_rows[] = {",
    ]
    for row in data["c_registry_rows"]:
        lines.extend(
            [
                "              {",
                f"                  {c_q(row.get('route_id'))},",
                f"                  {c_q(row.get('core_op'))},",
                f"                  {c_q(row.get('route_kind'))},",
                f"                  {c_q(row.get('helper_symbol'))},",
                f"                  {c_q(row.get('route_proof'))},",
                f"                  {int(row['tier'])},",
                f"                  {int(row['route_result'])},",
                f"                  {int(row['emit_kind'])},",
                f"                  {int(row['need_kind'])},",
                "              },",
            ]
        )
    lines.extend(
        [
            "          };",
            "",
            "      static const struct HakoLlvmcGenericMethodRouteRegistryRow*",
            "      hako_llvmc_generic_method_route_registry_find_by_tuple(",
            "          const char* route_id,",
            "          const char* core_op,",
            "          const char* route_kind,",
            "          int tier) {",
            "        size_t i = 0;",
            "        for (i = 0;",
            "             i < sizeof(hako_llvmc_generic_method_route_registry_rows) /",
            "                     sizeof(hako_llvmc_generic_method_route_registry_rows[0]);",
            "             i++) {",
            "          const struct HakoLlvmcGenericMethodRouteRegistryRow* row =",
            "              &hako_llvmc_generic_method_route_registry_rows[i];",
            "          if (row->route_id && row->core_op && row->route_kind &&",
            "              route_id && core_op && route_kind &&",
            "              !strcmp(row->route_id, route_id) &&",
            "              !strcmp(row->core_op, core_op) &&",
            "              !strcmp(row->route_kind, route_kind) &&",
            "              row->tier == tier) {",
            "            return row;",
            "          }",
            "        }",
            "        return NULL;",
            "      }",
            "",
            "      static const struct HakoLlvmcGenericMethodRouteRegistryRow*",
            "      hako_llvmc_generic_method_route_registry_find_by_route_tokens(",
            "          const char* route_id,",
            "          const char* route_kind,",
            "          const char* helper_symbol,",
            "          const char* route_proof) {",
            "        size_t i = 0;",
            "        for (i = 0;",
            "             i < sizeof(hako_llvmc_generic_method_route_registry_rows) /",
            "                     sizeof(hako_llvmc_generic_method_route_registry_rows[0]);",
            "             i++) {",
            "          const struct HakoLlvmcGenericMethodRouteRegistryRow* row =",
            "              &hako_llvmc_generic_method_route_registry_rows[i];",
            "          if (row->route_id && row->route_kind && row->helper_symbol &&",
            "              row->route_proof && route_id && route_kind && helper_symbol &&",
            "              route_proof &&",
            "              !strcmp(row->route_id, route_id) &&",
            "              !strcmp(row->route_kind, route_kind) &&",
            "              !strcmp(row->helper_symbol, helper_symbol) &&",
            "              !strcmp(row->route_proof, route_proof)) {",
            "            return row;",
            "          }",
            "        }",
            "        return NULL;",
            "      }",
            "",
            "      static const char*",
            "      hako_llvmc_generic_method_route_registry_first_tuple_mismatch(",
            "          const char* route_id,",
            "          const char* core_op,",
            "          const char* route_kind,",
            "          int tier,",
            "          const char* helper_symbol,",
            "          const char* route_proof) {",
            "        size_t i = 0;",
            "        int saw_route_id = 0;",
            "        int saw_core_op = 0;",
            "        int saw_route_kind = 0;",
            "        int saw_tier = 0;",
            "        if (!route_id) return \"route_id\";",
            "        if (!core_op) return \"core_op\";",
            "        if (!route_kind) return \"route_kind\";",
            "        for (i = 0;",
            "             i < sizeof(hako_llvmc_generic_method_route_registry_rows) /",
            "                     sizeof(hako_llvmc_generic_method_route_registry_rows[0]);",
            "             i++) {",
            "          const struct HakoLlvmcGenericMethodRouteRegistryRow* row =",
            "              &hako_llvmc_generic_method_route_registry_rows[i];",
            "          if (!(row->route_id && !strcmp(row->route_id, route_id))) continue;",
            "          saw_route_id = 1;",
            "          if (!(row->core_op && !strcmp(row->core_op, core_op))) continue;",
            "          saw_core_op = 1;",
            "          if (!(row->route_kind && !strcmp(row->route_kind, route_kind))) continue;",
            "          saw_route_kind = 1;",
            "          if (row->tier != tier) continue;",
            "          saw_tier = 1;",
            "          if (helper_symbol) {",
            "            if (!(row->helper_symbol &&",
            "                  !strcmp(row->helper_symbol, helper_symbol))) {",
            "              return \"helper_symbol\";",
            "            }",
            "          }",
            "          if (route_proof) {",
            "            if (!(row->route_proof && !strcmp(row->route_proof, route_proof))) {",
            "              return \"route_proof\";",
            "            }",
            "          }",
            "          return NULL;",
            "        }",
            "        if (!saw_route_id) return \"route_id\";",
            "        if (!saw_core_op) return \"core_op\";",
            "        if (!saw_route_kind) return \"route_kind\";",
            "        if (!saw_tier) return \"tier\";",
            "        return \"unknown\";",
            "      }",
            "#endif /* HAKO_LLVMC_GENERIC_METHOD_ROUTE_REGISTRY_INC */",
            "",
        ]
    )
    return "\n".join(lines)


def render_python(data: dict[str, Any]) -> str:
    lines = [
        "# @generated by tools/generic_method_route_descriptor_codegen.py",
        "# source: spec/mir/generic_method_routes.toml",
        "",
        "GENERIC_METHOD_ROUTE_DESCRIPTORS = {",
    ]
    for route in data["routes"]:
        lines.append(f"    {py_q(route['kind'])}: {{")
        for key in (
            "route_id",
            "emit_kind",
            "helper_symbol",
            "tier",
            "return_shape",
            "value_demand",
            "publication_policy",
            "effects",
        ):
            lines.append(f"        {key!r}: {repr(route.get(key))},")
        lines.append("    },")
    lines.extend(["}", "", "GENERIC_METHOD_ROUTE_REGISTRY_ROWS = ("])
    for row in data["c_registry_rows"]:
        py_row = {key: value for key, value in dict(row).items() if value is not None}
        lines.append(f"    {repr(py_row)},")
    lines.extend([")", ""])
    return "\n".join(lines)


def write_or_check(path: Path, content: str, check: bool) -> bool:
    if path.exists():
        old = path.read_text()
    else:
        old = None
    if old == content:
        return False
    if check:
        old_lines = [] if old is None else old.splitlines(keepends=True)
        diff = difflib.unified_diff(
            old_lines,
            content.splitlines(keepends=True),
            fromfile=str(path),
            tofile=f"{path} (generated)",
        )
        sys.stderr.writelines(diff)
        raise SystemExit(f"generated output differs: {path.relative_to(ROOT)}")
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content)
    return True


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    data = load_spec()
    changed = [
        write_or_check(RUST_OUT, render_rust(data), args.check),
        write_or_check(C_OUT, render_c(data), args.check),
        write_or_check(PY_OUT, render_python(data), args.check),
    ]
    if args.check:
        print("generic_method_route_descriptors=unchanged")
    elif any(changed):
        print("generic_method_route_descriptors=updated")
    else:
        print("generic_method_route_descriptors=unchanged")


if __name__ == "__main__":
    main()
