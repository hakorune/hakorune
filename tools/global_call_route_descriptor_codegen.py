#!/usr/bin/env python3
"""Generate C global-call route descriptor rows from Rust route proof metadata."""

from __future__ import annotations

import argparse
import difflib
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
RUST_MODEL = ROOT / "src/mir/global_call_route_plan/model.rs"
RUST_ROUTE = ROOT / "src/mir/global_call_route_plan/route.rs"
C_OUT = ROOT / "lang/c-abi/shims/hako_llvmc_ffi_global_call_route_registry.inc"

GLOBAL_CALL_ORIGIN_KIND_BY_RESULT_ORIGIN = {
    "none": "ORG_NONE",
    "string": "ORG_STRING",
    "array_string_birth": "ORG_ARRAY_STRING_BIRTH",
    "map_birth": "ORG_MAP_BIRTH",
}

LOWERING_TIER_JSON_BY_RUST_VARIANT = {
    "DirectAbi": "DirectAbi",
}


def parse_global_call_proofs() -> list[str]:
    text = RUST_MODEL.read_text()
    proof_map = parse_global_call_proof_map(text)
    variants = parse_enum_variants(text, "GlobalCallProof")
    return [proof_map[variant] for variant in variants]


def parse_global_call_proof_rows() -> list[dict[str, str]]:
    text = RUST_MODEL.read_text()
    variants = parse_enum_variants(text, "GlobalCallProof")
    proof_map = parse_global_call_proof_map(text)
    result_origin_map = parse_self_string_match(
        find_rust_fn_body(text, "result_origin")
    )
    validate_complete_mapping("GlobalCallProof::result_origin", variants, result_origin_map)
    definition_owner_variant_map = parse_self_enum_match(
        find_rust_fn_body(text, "definition_owner"),
        "GlobalCallDefinitionOwner",
    )
    validate_complete_mapping(
        "GlobalCallProof::definition_owner",
        variants,
        definition_owner_variant_map,
    )
    definition_owner_map = parse_definition_owner_names(text)
    rows: list[dict[str, str]] = []
    for variant in variants:
        result_origin = result_origin_map[variant]
        origin_kind = GLOBAL_CALL_ORIGIN_KIND_BY_RESULT_ORIGIN.get(result_origin)
        if origin_kind is None:
            raise SystemExit(
                f"missing C origin kind for GlobalCallProof::{variant} "
                f"result_origin={result_origin!r}"
            )
        rows.append(
            {
                "variant": variant,
                "proof": proof_map[variant],
                "result_origin": result_origin,
                "origin_kind": origin_kind,
                "definition_owner": definition_owner_map[
                    definition_owner_variant_map[variant]
                ],
            }
        )
    return rows


def parse_definition_owner_names(text: str) -> dict[str, str]:
    variants = parse_enum_variants(text, "GlobalCallDefinitionOwner")
    impl_body = find_rust_impl_body(text, "GlobalCallDefinitionOwner")
    owner_map = parse_self_string_match(find_rust_fn_body(impl_body, "as_json_name"))
    validate_complete_mapping("GlobalCallDefinitionOwner::as_json_name", variants, owner_map)
    return owner_map


def parse_global_call_proof_map(text: str) -> dict[str, str]:
    variants = parse_enum_variants(text, "GlobalCallProof")
    body = find_rust_fn_body(text, "as_json_name")
    proof_map = parse_self_string_match(body)
    validate_complete_mapping("GlobalCallProof::as_json_name", variants, proof_map)
    for variant, proof in proof_map.items():
        if not proof.startswith("typed_global_call_"):
            raise SystemExit(
                f"unexpected GlobalCallProof JSON name for {variant}: {proof}"
            )
    if len(set(proof_map.values())) != len(proof_map):
        raise SystemExit("duplicate GlobalCallProof JSON name")
    return proof_map


def parse_global_call_return_contracts() -> dict[str, dict[str, str]]:
    text = RUST_MODEL.read_text()
    variants = parse_enum_variants(text, "GlobalCallReturnContract")
    impl_body = find_rust_impl_body(text, "GlobalCallReturnContract")
    shape_map = parse_self_string_match(find_rust_fn_body(impl_body, "as_json_name"))
    value_demand_map = parse_self_string_match(find_rust_fn_body(impl_body, "value_demand"))
    validate_complete_mapping("GlobalCallReturnContract::as_json_name", variants, shape_map)
    validate_complete_mapping("GlobalCallReturnContract::value_demand", variants, value_demand_map)
    return {
        variant: {
            "return_shape": shape_map[variant],
            "value_demand": value_demand_map[variant],
        }
        for variant in variants
    }


def parse_global_call_runtime_routes() -> list[dict[str, str]]:
    text = RUST_ROUTE.read_text()
    variants = parse_enum_variants(text, "GlobalCallLoweringOverride")
    route_kind_map = parse_self_string_match(find_rust_fn_body(text, "route_kind"))
    symbol_map = parse_self_string_match(find_rust_fn_body(text, "target_symbol"))
    proof_variant_map = parse_self_enum_match(
        find_rust_fn_body(text, "proof"),
        "GlobalCallProof",
    )
    contract_variant_map = parse_self_enum_match(
        find_rust_fn_body(text, "return_contract"),
        "GlobalCallReturnContract",
    )
    validate_complete_mapping("GlobalCallLoweringOverride::route_kind", variants, route_kind_map)
    validate_complete_mapping("GlobalCallLoweringOverride::target_symbol", variants, symbol_map)
    validate_complete_mapping("GlobalCallLoweringOverride::proof", variants, proof_variant_map)
    validate_complete_mapping(
        "GlobalCallLoweringOverride::return_contract",
        variants,
        contract_variant_map,
    )
    proof_map = parse_global_call_proof_map(RUST_MODEL.read_text())
    contract_map = parse_global_call_return_contracts()
    routes: list[dict[str, str]] = []
    for variant in variants:
        contract = contract_map[contract_variant_map[variant]]
        routes.append(
            {
                "variant": variant,
                "route_kind": route_kind_map[variant],
                "symbol": symbol_map[variant],
                "proof": proof_map[proof_variant_map[variant]],
                "return_shape": contract["return_shape"],
                "value_demand": contract["value_demand"],
            }
        )
    return routes


def parse_global_call_direct_contract() -> dict[str, str]:
    text = RUST_ROUTE.read_text()
    impl_body = find_rust_impl_body(text, "GlobalCallRoute")
    route_id = parse_literal_return(impl_body, "route_id")
    core_op = parse_literal_return(impl_body, "core_op")
    route_kind = require_string(find_rust_fn_body(impl_body, "route_kind"), route_id)
    tier_variant = require_enum_variant(
        find_rust_fn_body(impl_body, "lowering_tier"),
        "LoweringPlanTier",
        "DirectAbi",
    )
    emit_kind_variant = require_enum_variant(
        find_rust_fn_body(impl_body, "lowering_emit_kind"),
        "LoweringPlanEmitKind",
        "DirectFunctionCall",
    )
    tier = LOWERING_TIER_JSON_BY_RUST_VARIANT.get(tier_variant)
    if tier is None:
        raise SystemExit(f"missing C lowering tier for {tier_variant}")
    return {
        "route_id": route_id,
        "core_op": core_op,
        "route_kind": route_kind,
        "tier": tier,
        "emit_kind": lowering_emit_kind_json_name(emit_kind_variant),
    }


def parse_literal_return(text: str, fn_name: str) -> str:
    body = find_rust_fn_body(text, fn_name).strip()
    match = re.search(r"^\s*\"(?P<value>[^\"]+)\"\s*$", body, re.S)
    if not match:
        raise SystemExit(f"expected {fn_name} to return one string literal")
    return match.group("value")


def require_string(text: str, value: str) -> str:
    if f'"{value}"' not in text:
        raise SystemExit(f"missing expected Rust route string {value!r}")
    return value


def require_enum_variant(text: str, enum_name: str, variant: str) -> str:
    needle = f"{enum_name}::{variant}"
    if needle not in text:
        raise SystemExit(f"missing expected Rust enum variant {needle}")
    return variant


def lowering_emit_kind_json_name(variant: str) -> str:
    if variant == "DirectFunctionCall":
        return "direct_function_call"
    raise SystemExit(f"missing JSON lowering emit kind for {variant}")


def parse_enum_variants(text: str, enum_name: str) -> list[str]:
    enum_match = re.search(
        rf"enum\s+{re.escape(enum_name)}\s*\{{(?P<body>.*?)\n\}}",
        text,
        re.S,
    )
    if not enum_match:
        raise SystemExit(f"failed to find {enum_name} enum")
    variants = [
        item.strip()
        for item in enum_match.group("body").split(",")
        if item.strip() and not item.strip().startswith("//")
    ]
    variants = [re.sub(r"\s.*$", "", item) for item in variants]
    if not variants:
        raise SystemExit(f"{enum_name} enum has no variants")
    return variants


def find_rust_fn_body(text: str, fn_name: str) -> str:
    match = re.search(
        rf"\bfn\s+{re.escape(fn_name)}\s*\([^)]*\)\s*(?:->\s*[^\{{]+)?\{{",
        text,
        re.S,
    )
    if not match:
        raise SystemExit(f"failed to find Rust function {fn_name}")
    return balanced_body_after_open_brace(text, match.end() - 1)


def find_rust_impl_body(text: str, impl_name: str) -> str:
    match = re.search(rf"\bimpl\s+{re.escape(impl_name)}\s*\{{", text, re.S)
    if not match:
        raise SystemExit(f"failed to find Rust impl {impl_name}")
    return balanced_body_after_open_brace(text, match.end() - 1)


def parse_self_string_match(body: str) -> dict[str, str]:
    result: dict[str, str] = {}
    for match in re.finditer(
        r"(?P<lhs>Self::[A-Za-z0-9_]+(?:\s*\|\s*Self::[A-Za-z0-9_]+)*)"
        r"\s*=>\s*(?:\{\s*)?\"(?P<value>[^\"]+)\"",
        body,
        re.S,
    ):
        for variant in re.findall(r"Self::([A-Za-z0-9_]+)", match.group("lhs")):
            if variant in result:
                raise SystemExit(f"duplicate string match arm for Self::{variant}")
            result[variant] = match.group("value")
    return result


def parse_self_enum_match(body: str, enum_name: str) -> dict[str, str]:
    result: dict[str, str] = {}
    for match in re.finditer(
        r"(?P<lhs>Self::[A-Za-z0-9_]+(?:\s*\|\s*Self::[A-Za-z0-9_]+)*)"
        rf"\s*=>\s*{re.escape(enum_name)}::(?P<value>[A-Za-z0-9_]+)",
        body,
        re.S,
    ):
        for variant in re.findall(r"Self::([A-Za-z0-9_]+)", match.group("lhs")):
            if variant in result:
                raise SystemExit(f"duplicate enum match arm for Self::{variant}")
            result[variant] = match.group("value")
    return result


def validate_complete_mapping(name: str, variants: list[str], mapping: dict[str, str]) -> None:
    missing = set(variants) - set(mapping)
    extra = set(mapping) - set(variants)
    if missing or extra:
        raise SystemExit(f"{name} mismatch missing={sorted(missing)} extra={sorted(extra)}")


def balanced_body_after_open_brace(text: str, open_brace_index: int) -> str:
    depth = 0
    for index in range(open_brace_index, len(text)):
        char = text[index]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return text[open_brace_index + 1 : index]
    raise SystemExit("unterminated Rust function body")


def cq(value: str) -> str:
    return '"' + value.replace("\\", "\\\\").replace('"', '\\"') + '"'


def render_c(
    proof_rows: list[dict[str, str]],
    runtime_routes: list[dict[str, str]],
    direct_contract: dict[str, str],
) -> str:
    lines = [
        "/* @generated by tools/global_call_route_descriptor_codegen.py */",
        "/* source: src/mir/global_call_route_plan/model.rs */",
        "#ifndef HAKO_LLVMC_GLOBAL_CALL_ROUTE_REGISTRY_INC",
        "#define HAKO_LLVMC_GLOBAL_CALL_ROUTE_REGISTRY_INC",
        "",
        "      struct HakoLlvmcGlobalCallProofRegistryRow {",
        "        const char* proof;",
        "        const char* result_origin;",
        "        int origin_kind;",
        "        const char* definition_owner;",
        "      };",
        "",
        "      struct HakoLlvmcGlobalCallRuntimeRouteRegistryRow {",
        "        const char* route_kind;",
        "        const char* proof;",
        "        const char* symbol;",
        "        const char* return_shape;",
        "        const char* value_demand;",
        "        const char* emit_kind;",
        "      };",
        "",
        "      struct HakoLlvmcGlobalCallDirectContractRow {",
        "        const char* route_id;",
        "        const char* core_op;",
        "        const char* route_kind;",
        "        const char* tier;",
        "        const char* emit_kind;",
        "      };",
        "",
        "      static const struct HakoLlvmcGlobalCallProofRegistryRow",
        "          hako_llvmc_global_call_proof_registry_rows[] = {",
    ]
    for row in proof_rows:
        lines.append(
            f"              {{{cq(row['proof'])}, {cq(row['result_origin'])}, "
            f"{row['origin_kind']}, {cq(row['definition_owner'])}}},"
        )
    lines.extend(
        [
            "          };",
            "",
            "      auto const struct HakoLlvmcGlobalCallProofRegistryRow*",
            "      hako_llvmc_global_call_proof_registry_find(const char* proof) {",
            "        size_t i = 0;",
            "        if (!proof) return NULL;",
            "        for (i = 0;",
            "             i < sizeof(hako_llvmc_global_call_proof_registry_rows) /",
            "                     sizeof(hako_llvmc_global_call_proof_registry_rows[0]);",
            "             i++) {",
            "          const struct HakoLlvmcGlobalCallProofRegistryRow* row =",
            "              &hako_llvmc_global_call_proof_registry_rows[i];",
            "          if (row->proof && !strcmp(row->proof, proof)) return row;",
            "        }",
            "        return NULL;",
            "      }",
            "",
            "      auto int hako_llvmc_global_call_proof_registry_contains(",
            "          const char* proof) {",
            "        return hako_llvmc_global_call_proof_registry_find(proof) != NULL;",
            "      }",
            "",
            "      auto int hako_llvmc_global_call_proof_registry_result_origin_kind(",
            "          const char* proof,",
            "          const char* result_origin) {",
            "        const struct HakoLlvmcGlobalCallProofRegistryRow* row =",
            "            hako_llvmc_global_call_proof_registry_find(proof);",
            "        if (!(row && row->result_origin && result_origin)) return ORG_NONE;",
            "        if (strcmp(row->result_origin, result_origin)) return ORG_NONE;",
            "        return row->origin_kind;",
            "      }",
            "",
            "      auto int hako_llvmc_global_call_proof_registry_definition_owner_is(",
            "          const char* proof,",
            "          const char* definition_owner,",
            "          const char* expected_owner) {",
            "        const struct HakoLlvmcGlobalCallProofRegistryRow* row =",
            "            hako_llvmc_global_call_proof_registry_find(proof);",
            "        if (!(row && row->definition_owner && definition_owner && expected_owner)) {",
            "          return 0;",
            "        }",
            "        return !strcmp(row->definition_owner, definition_owner) &&",
            "               !strcmp(row->definition_owner, expected_owner);",
            "      }",
            "",
            "      static const struct HakoLlvmcGlobalCallDirectContractRow",
            "          hako_llvmc_global_call_direct_contract_row = {",
            f"              {cq(direct_contract['route_id'])},",
            f"              {cq(direct_contract['core_op'])},",
            f"              {cq(direct_contract['route_kind'])},",
            f"              {cq(direct_contract['tier'])},",
            f"              {cq(direct_contract['emit_kind'])},",
            "          };",
            "",
            "      auto int hako_llvmc_global_call_route_registry_matches_direct_contract(",
            "          const char* route_id,",
            "          const char* core_op,",
            "          const char* route_kind,",
            "          const char* tier,",
            "          const char* emit_kind) {",
            "        const struct HakoLlvmcGlobalCallDirectContractRow* row =",
            "            &hako_llvmc_global_call_direct_contract_row;",
            "        return row->route_id && row->core_op && row->route_kind &&",
            "               row->tier && row->emit_kind && route_id && core_op && route_kind &&",
            "               tier && emit_kind &&",
            "               !strcmp(row->route_id, route_id) &&",
            "               !strcmp(row->core_op, core_op) &&",
            "               !strcmp(row->route_kind, route_kind) &&",
            "               !strcmp(row->tier, tier) &&",
            "               !strcmp(row->emit_kind, emit_kind);",
            "      }",
            "",
            "      static const struct HakoLlvmcGlobalCallRuntimeRouteRegistryRow",
            "          hako_llvmc_global_call_runtime_route_registry_rows[] = {",
        ]
    )
    for route in runtime_routes:
        lines.extend(
            [
                "              {",
                f"                  {cq(route['route_kind'])},",
                f"                  {cq(route['proof'])},",
                f"                  {cq(route['symbol'])},",
                f"                  {cq(route['return_shape'])},",
                f"                  {cq(route['value_demand'])},",
                "                  \"runtime_call\",",
                "              },",
            ]
        )
    lines.extend(
        [
            "          };",
            "",
            "      auto const struct HakoLlvmcGlobalCallRuntimeRouteRegistryRow*",
            "      hako_llvmc_global_call_runtime_route_registry_find(",
            "          const char* route_kind,",
            "          const char* proof,",
            "          const char* symbol,",
            "          const char* return_shape,",
            "          const char* value_demand,",
            "          const char* emit_kind) {",
            "        size_t i = 0;",
            "        for (i = 0;",
            "             i < sizeof(hako_llvmc_global_call_runtime_route_registry_rows) /",
            "                     sizeof(hako_llvmc_global_call_runtime_route_registry_rows[0]);",
            "             i++) {",
            "          const struct HakoLlvmcGlobalCallRuntimeRouteRegistryRow* row =",
            "              &hako_llvmc_global_call_runtime_route_registry_rows[i];",
            "          if (row->route_kind && row->proof && row->symbol &&",
            "              row->return_shape && row->value_demand && row->emit_kind &&",
            "              route_kind && proof && symbol && return_shape && value_demand && emit_kind &&",
            "              !strcmp(row->route_kind, route_kind) &&",
            "              !strcmp(row->proof, proof) &&",
            "              !strcmp(row->symbol, symbol) &&",
            "              !strcmp(row->return_shape, return_shape) &&",
            "              !strcmp(row->value_demand, value_demand) &&",
            "              !strcmp(row->emit_kind, emit_kind)) {",
            "            return row;",
            "          }",
            "        }",
            "        return NULL;",
            "      }",
            "",
            "      auto int hako_llvmc_global_call_runtime_route_registry_matches_stage1_emit_program_json(",
            "          const char* route_kind,",
            "          const char* proof,",
            "          const char* symbol,",
            "          const char* return_shape,",
            "          const char* value_demand,",
            "          const char* emit_kind) {",
            "        const struct HakoLlvmcGlobalCallRuntimeRouteRegistryRow* row =",
            "            hako_llvmc_global_call_runtime_route_registry_find(",
            "                route_kind,",
            "                proof,",
            "                symbol,",
            "                return_shape,",
            "                value_demand,",
            "                emit_kind);",
            "        return row && row->route_kind &&",
            "               !strcmp(row->route_kind, \"stage1.emit_program_json_v0\");",
            "      }",
            "",
            "#endif /* HAKO_LLVMC_GLOBAL_CALL_ROUTE_REGISTRY_INC */",
            "",
        ]
    )
    return "\n".join(lines)


def write_or_check(path: Path, content: str, check: bool) -> bool:
    old = path.read_text() if path.exists() else ""
    if old == content:
        return False
    if check:
        diff = "\n".join(
            difflib.unified_diff(
                old.splitlines(),
                content.splitlines(),
                fromfile=str(path),
                tofile=str(path) + " (generated)",
                lineterm="",
            )
        )
        raise SystemExit(diff)
    path.write_text(content)
    return True


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    changed = write_or_check(
        C_OUT,
        render_c(
            parse_global_call_proof_rows(),
            parse_global_call_runtime_routes(),
            parse_global_call_direct_contract(),
        ),
        args.check,
    )
    print(f"global_call_route_descriptors={'changed' if changed else 'unchanged'}")


if __name__ == "__main__":
    main()
