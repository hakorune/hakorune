#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="generic-method-set-policy-mirror-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

HAKO_POLICY="$ROOT_DIR/lang/src/runtime/collections/method_policy_box.hako"
SPEC="$ROOT_DIR/spec/mir/generic_method_routes.toml"
C_POLICY="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc"
C_MATCH="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc"
C_LOWERING="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_generic_method_lowering.inc"
C_ROUTE="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc"
C_REGISTRY="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_generic_method_route_registry.inc"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$HAKO_POLICY" "$SPEC" "$C_POLICY" "$C_MATCH" "$C_LOWERING" "$C_ROUTE" "$C_REGISTRY"

echo "[$TAG] checking generic-method Set route/demand mirror"

python3 - "$ROOT_DIR" "$HAKO_POLICY" "$SPEC" "$C_POLICY" "$C_MATCH" "$C_LOWERING" "$C_ROUTE" "$C_REGISTRY" <<'PY'
import pathlib
import re
import sys
import tomllib

tag = "generic-method-set-policy-mirror-guard"
root = pathlib.Path(sys.argv[1])
hako_path = pathlib.Path(sys.argv[2])
spec_path = pathlib.Path(sys.argv[3])
c_policy_path = pathlib.Path(sys.argv[4])
c_match_path = pathlib.Path(sys.argv[5])
c_lowering_path = pathlib.Path(sys.argv[6])
c_route_path = pathlib.Path(sys.argv[7])
c_registry_path = pathlib.Path(sys.argv[8])

hako = hako_path.read_text()
with spec_path.open("rb") as fh:
    spec = tomllib.load(fh)
c_policy = c_policy_path.read_text()
c_match = c_match_path.read_text()
c_lowering = c_lowering_path.read_text()
c_route = c_route_path.read_text()
c_registry = c_registry_path.read_text()

ROUTES = [
    ("route_none", "None", "HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE"),
    ("route_map_store_any", "MapStoreAny", "HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_MAP_STORE_ANY"),
    ("route_array_store_i64", "ArrayStoreI64", "HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_I64"),
    (
        "route_array_store_string",
        "ArrayStoreString",
        "HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_STRING",
    ),
    ("route_array_store_any", "ArrayStoreAny", "HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_ANY"),
]

C_ENUM_NAMES = [
    "HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE",
    "HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_MAP_STORE_I64",
    "HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_MAP_STORE_ANY",
    "HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_I64",
    "HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_STRING",
    "HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_ANY",
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
if enum_names != C_ENUM_NAMES:
    fail(
        "GenericMethodSetRouteKind drift: "
        f"expected {C_ENUM_NAMES}, found {enum_names}"
    )

set_body = extract_body(hako, "set_route")
set_route_calls = set(re.findall(r"\bme\.(route_[A-Za-z0-9_]+)\s*\(", set_body))
unknown_routes = sorted(set_route_calls - SET_ROUTE_METHODS)
missing_routes = sorted(SET_ROUTE_METHODS - set_route_calls)
if unknown_routes:
    fail(f"CollectionMethodPolicyBox.set_route uses unknown Set routes: {unknown_routes}")
if missing_routes:
    fail(f"CollectionMethodPolicyBox.set_route no longer covers expected routes: {missing_routes}")

for path, text in (
    (c_policy_path, c_policy),
    (c_match_path, c_match),
    (c_lowering_path, c_lowering),
):
    if re.search(r"\bclassify_generic_method_set_route\s*\(", text):
        fail(f"{rel(path)} still contains handwritten Set route fallback")

expected_set_routes = {
    ("map_store_i64", "any", "map_store_i64"),
    ("map_store_any", "any", "map_store_any"),
    ("array_store_any", "i64", "array_store_i64"),
    ("array_store_any", "non_string_handle", "array_store_any"),
    ("array_store_any", "string_handle", "array_store_string"),
}
actual_set_routes = set()
for route in spec.get("routes", []):
    route_kind = route.get("kind")
    for item in route.get("c_set_routes", []):
        actual_set_routes.add(
            (route_kind, item.get("value_shape"), item.get("result"))
        )
if actual_set_routes != expected_set_routes:
    fail(
        "generic-method Set c_set_routes drift: "
        f"expected {sorted(expected_set_routes)}, found {sorted(actual_set_routes)}"
    )

require_regex(
    c_registry,
    r"\bint\s+set_value_shape\s*;\s*\bint\s+set_route_result\s*;",
    "generated C route registry must expose Set value-shape/result fields",
)
if "hako_llvmc_generic_method_route_registry_find_set_rule" not in c_registry:
    fail("generated C route registry must expose Set rule lookup helper")

set_row_patterns = [
    (
        "MapSet/map_store_i64",
        r'"generic_method\.set"\s*,\s*"MapSet"\s*,\s*"map_store_i64"\s*,\s*'
        r'"nyash\.map\.slot_store_hih"\s*,\s*"set_surface_policy"\s*,\s*'
        r"3\s*,\s*0\s*,\s*1\s*,\s*2\s*,\s*1\s*,\s*1\s*,",
    ),
    (
        "MapSet/map_store_any",
        r'"generic_method\.set"\s*,\s*"MapSet"\s*,\s*"map_store_any"\s*,\s*'
        r'"nyash\.map\.slot_store_hhh"\s*,\s*"set_surface_policy"\s*,\s*'
        r"3\s*,\s*0\s*,\s*1\s*,\s*2\s*,\s*1\s*,\s*2\s*,",
    ),
    (
        "ArraySet/i64",
        r'"generic_method\.set"\s*,\s*"ArraySet"\s*,\s*"array_store_any"\s*,\s*'
        r'"nyash\.array\.slot_store_hii"\s*,\s*"set_surface_policy"\s*,\s*'
        r"3\s*,\s*0\s*,\s*1\s*,\s*12\s*,\s*2\s*,\s*3\s*,",
    ),
    (
        "ArraySet/non_string_handle",
        r'"generic_method\.set"\s*,\s*"ArraySet"\s*,\s*"array_store_any"\s*,\s*'
        r'"nyash\.array\.slot_store_hih"\s*,\s*"set_surface_policy"\s*,\s*'
        r"3\s*,\s*0\s*,\s*1\s*,\s*12\s*,\s*3\s*,\s*5\s*,",
    ),
    (
        "ArraySet/string_handle",
        r'"generic_method\.set"\s*,\s*"ArraySet"\s*,\s*"array_store_any"\s*,\s*'
        r'"nyash\.array\.set_his"\s*,\s*"set_surface_policy"\s*,\s*'
        r"3\s*,\s*0\s*,\s*1\s*,\s*12\s*,\s*4\s*,\s*4\s*,",
    ),
]
for label, pattern in set_row_patterns:
    require_regex(c_registry, pattern, f"generated C registry missing Set row {label}")

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
