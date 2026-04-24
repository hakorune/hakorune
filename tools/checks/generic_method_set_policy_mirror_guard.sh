#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="generic-method-set-policy-mirror-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

HAKO_POLICY="$ROOT_DIR/lang/src/runtime/collections/method_policy_box.hako"
C_POLICY="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc"
C_MATCH="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc"
C_LOWERING="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_generic_method_lowering.inc"
C_ROUTE="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$HAKO_POLICY" "$C_POLICY" "$C_MATCH" "$C_LOWERING" "$C_ROUTE"

echo "[$TAG] checking generic-method Set route/demand mirror"

python3 - "$ROOT_DIR" "$HAKO_POLICY" "$C_POLICY" "$C_MATCH" "$C_LOWERING" "$C_ROUTE" <<'PY'
import pathlib
import re
import sys

tag = "generic-method-set-policy-mirror-guard"
root = pathlib.Path(sys.argv[1])
hako_path = pathlib.Path(sys.argv[2])
c_policy_path = pathlib.Path(sys.argv[3])
c_match_path = pathlib.Path(sys.argv[4])
c_lowering_path = pathlib.Path(sys.argv[5])
c_route_path = pathlib.Path(sys.argv[6])

hako = hako_path.read_text()
c_policy = c_policy_path.read_text()
c_match = c_match_path.read_text()
c_lowering = c_lowering_path.read_text()
c_route = c_route_path.read_text()

ROUTES = [
    ("route_none", "None", "HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE"),
    (
        "route_runtime_data_store_any",
        "RuntimeDataStoreAny",
        "HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_RUNTIME_DATA_STORE_ANY",
    ),
    ("route_map_store_any", "MapStoreAny", "HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_MAP_STORE_ANY"),
    ("route_array_store_i64", "ArrayStoreI64", "HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_I64"),
    (
        "route_array_store_string",
        "ArrayStoreString",
        "HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_STRING",
    ),
    ("route_array_store_any", "ArrayStoreAny", "HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_ANY"),
]

SET_ROUTE_METHODS = {method for method, _, _ in ROUTES if method != "route_none"}

DEMANDS = [
    ("array_store_string_source_preserve", 1),
    ("array_store_string_identity_demand", 0),
    ("array_store_string_publication_demand", 1),
]


def fail(msg):
    print(f"[{tag}] ERROR: {msg}", file=sys.stderr)
    sys.exit(1)


def rel(path):
    return path.resolve().relative_to(root.resolve()).as_posix()


def extract_body(text, name):
    idx = text.find(name)
    if idx < 0:
        fail(f"missing function/method {name}")
    open_idx = text.find("{", idx)
    if open_idx < 0:
        fail(f"missing body for {name}")
    depth = 0
    in_string = False
    escape = False
    for pos in range(open_idx, len(text)):
        ch = text[pos]
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
                return text[open_idx + 1 : pos]
    fail(f"unterminated body for {name}")


def require_regex(text, pattern, msg):
    if not re.search(pattern, text, re.S):
        fail(msg)


for method, string_value, _ in ROUTES:
    body = extract_body(hako, method)
    require_regex(
        body,
        r'\breturn\s+"' + re.escape(string_value) + r'"\s*$',
        f"{rel(hako_path)} {method} must return {string_value!r}",
    )

enum = re.search(r"enum\s+GenericMethodSetRouteKind\s*\{(?P<body>.*?)\};", c_policy, re.S)
if not enum:
    fail(f"{rel(c_policy_path)} missing GenericMethodSetRouteKind")
enum_names = re.findall(r"\b(HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_[A-Z0-9_]+)\b", enum.group("body"))
expected_enum_names = [enum_name for _, _, enum_name in ROUTES]
if enum_names != expected_enum_names:
    fail(
        "GenericMethodSetRouteKind drift: "
        f"expected {expected_enum_names}, found {enum_names}"
    )

set_body = extract_body(hako, "set_route")
set_route_calls = set(re.findall(r"\bme\.(route_[A-Za-z0-9_]+)\s*\(", set_body))
unknown_routes = sorted(set_route_calls - SET_ROUTE_METHODS)
missing_routes = sorted(SET_ROUTE_METHODS - set_route_calls)
if unknown_routes:
    fail(f"CollectionMethodPolicyBox.set_route uses unknown Set routes: {unknown_routes}")
if missing_routes:
    fail(f"CollectionMethodPolicyBox.set_route no longer covers expected routes: {missing_routes}")

c_set_body = extract_body(c_policy, "classify_generic_method_set_route")
c_set_legacy_returns = set(
    re.findall(r"\breturn\s+(HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_[A-Z0-9_]+)\s*;", c_set_body)
)
metadata_set_body = extract_body(c_match, "generic_method_set_route_from_metadata_value_shape")
c_set_metadata_returns = set(
    re.findall(
        r"\breturn\s+(HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_[A-Z0-9_]+)\s*;",
        metadata_set_body,
    )
)
c_set_returns = c_set_legacy_returns | c_set_metadata_returns
unknown_c_returns = sorted(c_set_returns - set(expected_enum_names))
missing_c_returns = sorted(set(expected_enum_names) - c_set_returns)
if unknown_c_returns:
    fail(f"C set route consumers return unknown route enums: {unknown_c_returns}")
if missing_c_returns:
    fail(
        "C set route consumers no longer cover expected route enums: "
        f"{missing_c_returns}"
    )

for method, expected in DEMANDS:
    body = extract_body(hako, method)
    require_regex(
        body,
        r"route_array_store_string\s*\(\s*\).*?\breturn\s+" + str(expected) + r"\b",
        f"{rel(hako_path)} {method} must return {expected} for ArrayStoreString",
    )
    require_regex(
        body,
        r"\breturn\s+0\b\s*$",
        f"{rel(hako_path)} {method} fallback must stay 0",
    )

require_regex(
    extract_body(c_policy, "classify_array_store_string_source_preserve"),
    r"return\s+set_route\s*==\s*HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_STRING\s*;",
    "C source-preserve demand must be true only for ARRAY_STORE_STRING",
)
require_regex(
    extract_body(c_policy, "classify_array_store_string_identity_demand_stable_object"),
    r"\(void\)set_route\s*;\s*return\s+0\s*;",
    "C identity demand must stay 0 for ARRAY_STORE_STRING",
)
require_regex(
    extract_body(c_policy, "classify_array_store_string_publication_demand_publish_handle"),
    r"return\s+set_route\s*==\s*HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_STRING\s*;",
    "C publication demand must be true only for ARRAY_STORE_STRING",
)

for helper in (
    "classify_array_store_string_source_preserve",
    "classify_array_store_string_identity_demand_stable_object",
    "classify_array_store_string_publication_demand_publish_handle",
):
    if helper not in c_match:
        fail(f"{rel(c_match_path)} no longer consumes {helper}")

require_regex(
    c_lowering,
    r"case\s+HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_STRING:.*?"
    r"HAKO_LLVMC_ARRAY_SLOT_DEMAND_STORE_TEXT_PUBLIC",
    "ARRAY_STORE_STRING lowering must keep STORE_TEXT_PUBLIC demand",
)
require_regex(
    c_route,
    r"case\s+HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_STRING:.*?"
    r"array_store_string_source_preserve\s*=\s*1\s*;.*?"
    r"array_store_string_identity_demand_stable_object\s*=\s*0\s*;.*?"
    r"array_store_string_publication_demand_publish_handle\s*=\s*1\s*;",
    "runtime array-string route state must keep source=1 identity=0 publication=1",
)

print(f"[{tag}] ok routes={len(ROUTES) - 1} demands={len(DEMANDS)}")
PY
