#!/usr/bin/env python3
"""Generate C extern-call route descriptor rows from the Rust route spec."""

from __future__ import annotations

import argparse
import difflib
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
RUST_SPEC = ROOT / "src/mir/extern_call_route_plan/route_spec.rs"
C_OUT = ROOT / "lang/c-abi/shims/hako_llvmc_ffi_extern_call_route_registry.inc"

C_NEED_KIND_BY_CORE_OP = {
    "EnvGet": "HAKO_LLVMC_MIR_CALL_NEED_ENV_GET",
    "EnvSet": "HAKO_LLVMC_MIR_CALL_NEED_ENV_SET",
    "EnvNowMs": "HAKO_LLVMC_MIR_CALL_NEED_ENV_NOW_MS",
    "StringConcat": "HAKO_LLVMC_MIR_CALL_NEED_STRING_CONCAT",
    "StringConcat3": "HAKO_LLVMC_MIR_CALL_NEED_STRING_CONCAT",
    "StringInsert": "HAKO_LLVMC_MIR_CALL_NEED_STRING_CONCAT",
    "StringSubstring": "HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING",
    "StringSubstringConcat": "HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING",
    "StringSubstringConcat3": "HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING",
    "StringSubstringLen": "HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING_LEN",
    "AnyHandleLive": "HAKO_LLVMC_MIR_CALL_NEED_ANY_HANDLE_LIVE",
    "ArraySlotAppendAny": "HAKO_LLVMC_MIR_CALL_NEED_ARRAY_PUSH",
    "ArraySlotLenI64": "HAKO_LLVMC_MIR_CALL_NEED_ARRAY_LEN",
    "ArraySlotLoadI64": "HAKO_LLVMC_MIR_CALL_NEED_ARRAY_GET",
    "ArraySlotStoreI64": "HAKO_LLVMC_MIR_CALL_NEED_ARRAY_SET",
    "HakoAtomicSlotCasI64": "HAKO_LLVMC_MIR_CALL_NEED_HAKO_ATOMIC_SLOT_CAS_I64",
    "HakoAtomicSlotFetchAddI64": "HAKO_LLVMC_MIR_CALL_NEED_HAKO_ATOMIC_SLOT_FETCH_ADD_I64",
    "HakoAtomicSlotLoadI64": "HAKO_LLVMC_MIR_CALL_NEED_HAKO_ATOMIC_SLOT_LOAD_I64",
    "HakoAtomicSlotStoreI64": "HAKO_LLVMC_MIR_CALL_NEED_HAKO_ATOMIC_SLOT_STORE_I64",
    "HakoAtomicPtrCasOrdered": "HAKO_LLVMC_MIR_CALL_NEED_HAKO_ATOMIC_PTR_CAS_ORDERED",
    "HakoAtomicPtrLoadOrdered": "HAKO_LLVMC_MIR_CALL_NEED_HAKO_ATOMIC_PTR_LOAD_ORDERED",
    "HakoAtomicPtrStoreOrdered": "HAKO_LLVMC_MIR_CALL_NEED_HAKO_ATOMIC_PTR_STORE_ORDERED",
    "HakoMemAlloc": "HAKO_LLVMC_MIR_CALL_NEED_HAKO_MEM_ALLOC",
    "HakoMemFree": "HAKO_LLVMC_MIR_CALL_NEED_HAKO_MEM_FREE",
    "HakoOsvmReserveBytesI64": "HAKO_LLVMC_MIR_CALL_NEED_HAKO_OSVM_RESERVE_BYTES_I64",
    "HakoOsvmCommitBytesI64": "HAKO_LLVMC_MIR_CALL_NEED_HAKO_OSVM_COMMIT_BYTES_I64",
    "HakoOsvmDecommitBytesI64": "HAKO_LLVMC_MIR_CALL_NEED_HAKO_OSVM_DECOMMIT_BYTES_I64",
    "HakoOsvmUnreserveBytesI64": "HAKO_LLVMC_MIR_CALL_NEED_HAKO_OSVM_UNRESERVE_BYTES_I64",
    "HakoTlsCacheSlotGetI64": "HAKO_LLVMC_MIR_CALL_NEED_HAKO_TLS_CACHE_SLOT_GET_I64",
    "HakoTlsCacheSlotSetI64": "HAKO_LLVMC_MIR_CALL_NEED_HAKO_TLS_CACHE_SLOT_SET_I64",
    "HakoWorkerCurrentIdI64": "HAKO_LLVMC_MIR_CALL_NEED_HAKO_WORKER_CURRENT_ID_I64",
    "HostBridgeExternInvoke": "HAKO_LLVMC_MIR_CALL_NEED_HOSTBRIDGE_EXTERN_INVOKE_TRAP",
    "Stage1EmitProgramJson": "HAKO_LLVMC_MIR_CALL_NEED_STAGE1_EMIT_PROGRAM_JSON",
    "Stage1EmitMirFromSource": "HAKO_LLVMC_MIR_CALL_NEED_STAGE1_EMIT_MIR_FROM_SOURCE",
    "Stage1EmitMirFromProgramJson": "HAKO_LLVMC_MIR_CALL_NEED_STAGE1_EMIT_MIR_FROM_PROGRAM_JSON",
}


def parse_routes() -> list[dict[str, str]]:
    text = RUST_SPEC.read_text()
    body_match = re.search(
        r"static\s+EXTERN_CALL_ROUTE_SPECS:\s*&\[ExternCallRouteSpec\]\s*=\s*&\[(?P<body>.*)\];",
        text,
        re.S,
    )
    if not body_match:
        raise SystemExit("failed to find EXTERN_CALL_ROUTE_SPECS")
    entries = re.findall(r"ExternCallRouteSpec\s*\{(?P<body>.*?)\n\s*\},", body_match.group("body"), re.S)
    routes: list[dict[str, str]] = []
    seen: set[tuple[str, str]] = set()
    for entry in entries:
        route = {
            "route_id": read_string_field(entry, "route_id"),
            "core_op": read_string_field(entry, "core_op"),
            "route_kind": read_string_field(entry, "route_id"),
            "symbol": read_string_field(entry, "symbol"),
        }
        proof = read_proof_field(entry)
        if proof != "extern_registry":
            raise SystemExit(f"unsupported extern proof for {route['route_id']}: {proof}")
        need = C_NEED_KIND_BY_CORE_OP.get(route["core_op"])
        if need is None:
            raise SystemExit(f"missing C need mapping for extern core_op={route['core_op']}")
        route["need_kind"] = need
        key = (route["route_id"], route["symbol"])
        if key in seen:
            raise SystemExit(f"duplicate extern route descriptor: {key}")
        seen.add(key)
        routes.append(route)
    missing = set(C_NEED_KIND_BY_CORE_OP) - {route["core_op"] for route in routes}
    if missing:
        raise SystemExit(f"C need mapping references missing Rust extern route(s): {sorted(missing)}")
    return routes


def read_string_field(entry: str, field: str) -> str:
    match = re.search(rf"\b{field}:\s*\"([^\"]+)\"", entry)
    if not match:
        raise SystemExit(f"missing string field {field} in extern route entry")
    return match.group(1)


def read_proof_field(entry: str) -> str:
    string_match = re.search(r'\bproof:\s*"([^"]+)"', entry)
    if string_match:
        return string_match.group(1)
    const_match = re.search(r"\bproof:\s*EXTERN_REGISTRY_PROOF\b", entry)
    if const_match:
        return "extern_registry"
    raise SystemExit("missing proof field in extern route entry")


def cq(value: str) -> str:
    return '"' + value.replace("\\", "\\\\").replace('"', '\\"') + '"'


def render_c(routes: list[dict[str, str]]) -> str:
    lines = [
        "/* @generated by tools/extern_call_route_descriptor_codegen.py */",
        "/* source: src/mir/extern_call_route_plan/route_spec.rs */",
        "#ifndef HAKO_LLVMC_EXTERN_CALL_ROUTE_REGISTRY_INC",
        "#define HAKO_LLVMC_EXTERN_CALL_ROUTE_REGISTRY_INC",
        "",
        "      struct HakoLlvmcExternCallRouteRegistryRow {",
        "        const char* route_id;",
        "        const char* core_op;",
        "        const char* route_kind;",
        "        const char* symbol;",
        "        int tier;",
        "        int need_kind;",
        "      };",
        "",
        "      static const struct HakoLlvmcExternCallRouteRegistryRow",
        "          hako_llvmc_extern_call_route_registry_rows[] = {",
    ]
    for route in routes:
        lines.extend(
            [
                "              {",
                f"                  {cq(route['route_id'])},",
                f"                  {cq(route['core_op'])},",
                f"                  {cq(route['route_kind'])},",
                f"                  {cq(route['symbol'])},",
                "                  HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME,",
                f"                  {route['need_kind']},",
                "              },",
            ]
        )
    lines.extend(
        [
            "          };",
            "",
            "      auto const struct HakoLlvmcExternCallRouteRegistryRow*",
            "      hako_llvmc_extern_call_route_registry_find_need_rule(",
            "          const char* route_id,",
            "          const char* core_op,",
            "          const char* route_kind,",
            "          const char* symbol,",
            "          int tier) {",
            "        size_t i = 0;",
            "        for (i = 0;",
            "             i < sizeof(hako_llvmc_extern_call_route_registry_rows) /",
            "                     sizeof(hako_llvmc_extern_call_route_registry_rows[0]);",
            "             i++) {",
            "          const struct HakoLlvmcExternCallRouteRegistryRow* row =",
            "              &hako_llvmc_extern_call_route_registry_rows[i];",
            "          if (row->route_id && row->core_op && row->route_kind && row->symbol &&",
            "              route_id && core_op && route_kind && symbol &&",
            "              !strcmp(row->route_id, route_id) &&",
            "              !strcmp(row->core_op, core_op) &&",
            "              !strcmp(row->route_kind, route_kind) &&",
            "              !strcmp(row->symbol, symbol) &&",
            "              row->tier == tier) {",
            "            return row;",
            "          }",
            "        }",
            "        return NULL;",
            "      }",
            "",
            "#endif /* HAKO_LLVMC_EXTERN_CALL_ROUTE_REGISTRY_INC */",
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
    changed = write_or_check(C_OUT, render_c(parse_routes()), args.check)
    print(f"extern_call_route_descriptors={'changed' if changed else 'unchanged'}")


if __name__ == "__main__":
    main()
