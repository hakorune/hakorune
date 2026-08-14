#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="dynamic-v2-checked-callout-physicalizer"
BASE="$ROOT_DIR/apps/tests/mir_shape_guard/method_call_only_small.prebuilt.mir.json"
FFI="$ROOT_DIR/target/release/libhako_llvmc_ffi.so"
TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/$TAG.XXXXXX")"
trap 'rm -rf "$TMP_DIR"' EXIT

if [[ ! -f "$BASE" ]]; then
  echo "[$TAG] missing fixture: ${BASE#$ROOT_DIR/}" >&2
  exit 1
fi

bash "$ROOT_DIR/tools/build_hako_llvmc_ffi.sh" >/dev/null

python3 - "$BASE" "$TMP_DIR/valid.json" "$TMP_DIR/invalid.json" "$TMP_DIR/invalid_launch.json" "$TMP_DIR/invalid_arity.json" "$TMP_DIR/invalid_missing_helper.json" "$TMP_DIR/invalid_duplicate_helper.json" "$TMP_DIR/invalid_duplicate_launch.json" "$TMP_DIR/invalid_missing_launch.json" "$TMP_DIR/invalid_nonzero_launch.json" "$TMP_DIR/invalid_physical.json" "$TMP_DIR/ordinary.json" <<'PY'
import copy
import json
import sys

(
    base_path, valid_path, invalid_path, invalid_launch_path, invalid_arity_path,
    invalid_missing_helper_path, invalid_duplicate_helper_path,
    invalid_duplicate_launch_path, invalid_missing_launch_path,
    invalid_nonzero_launch_path, invalid_physical_path, ordinary_path,
) = sys.argv[1:]
data = json.load(open(base_path, encoding="utf-8"))
function = data["functions"][0]
launch = copy.deepcopy(function)
launch["name"] = "main"
launch["params"] = []
launch["metadata"] = {}
launch["blocks"] = [{"id": 0, "instructions": [{"op": "ret", "value": None}]}]
function["name"] = "ParserScanLoopBox.skip_while/4"
function["params"] = [0, 1, 2, 3]
admission = {
    "schema_version": 2,
    "contract_id": "hako.text.scan@1",
    "profile": 1,
    "abi_revision": 1,
    "wire_revision": 2,
    "registry_generation": 18446744073709551613,
    "plan_stamp": {
        "compiler_domain": 18446744073709551615,
        "invocation_ordinal": 18446744073709551614,
    },
    "return_type": "i64",
    "return_lane": "immediate_i64",
    "function_effects": 16,
    "formal_parameters": [
        {"role": "src", "value_id": 0, "lane": "opaque_handle"},
        {"role": "pos", "value_id": 1, "lane": "immediate_i64"},
        {"role": "end", "value_id": 2, "lane": "immediate_i64"},
        {"role": "pred_chars", "value_id": 3, "lane": "opaque_handle"},
    ],
    "calls": [
        {
            "role": "substring", "site_id": 0, "entry_id": 1,
            "symbol": "hako.text.scan.substring.v1", "abi_revision": 1,
            "wire_revision": 2, "receiver_lane": "opaque_handle",
            "argument_lanes": ["immediate_i64", "immediate_i64"],
            "result_lane": "opaque_handle", "lease": "end_authorized",
            "normal_shape": "end_authorized_handle", "outcome_slot": 0,
            "normal_result_dst": 20, "effects": 16,
            "source_block": 0, "receiver": 0, "arguments": [1, 2],
            "normal_landing": 2, "fault_landing": 3,
            "fault_terminal_block": 3, "normal_result_block": 2,
            "normal_result_index": 0,
        },
        {
            "role": "index_of", "site_id": 1, "entry_id": 2,
            "symbol": "hako.text.scan.index_of.v1", "abi_revision": 1,
            "wire_revision": 2, "receiver_lane": "opaque_handle",
            "argument_lanes": ["opaque_handle"],
            "result_lane": "immediate_i64", "lease": "none",
            "normal_shape": "immediate_i64", "outcome_slot": 1,
            "normal_result_dst": 21, "effects": 16,
            "source_block": 2, "receiver": 3, "arguments": [20],
            "normal_landing": 4, "fault_landing": 5,
            "fault_terminal_block": 5, "normal_result_block": 4,
            "normal_result_index": 0,
        },
    ],
    "end_facts": [
        {"site_id": 0, "lease_slot": 0, "block": 4, "instruction_index": 1},
        {"site_id": 0, "lease_slot": 0, "block": 5, "instruction_index": 0},
        {"site_id": 0, "lease_slot": 0, "block": 6, "instruction_index": 0},
    ],
}
function["metadata"] = {
    "a_prime_i64_physical_receipt": {"schema_version": 1},
    "dynamic_v2_aot_call_admission_v2": admission,
}
for block in function["blocks"]:
    block["instructions"] = []
function["blocks"][0]["instructions"] = [
    {"op": "checked_callout", "site_id": 0, "receiver": 0,
     "args": [1, 2], "normal": 2, "fault": 3, "effects": 16},
]
function["blocks"][1]["instructions"] = [{"op": "ret", "value": None}]
function["blocks"][2]["instructions"] = [
    {"op": "checked_callout_normal_result", "site_id": 0, "dst": 20},
    {"op": "checked_callout", "site_id": 1, "receiver": 3,
     "args": [20], "normal": 4, "fault": 5, "effects": 16},
]
function["blocks"][3]["instructions"] = [
    {"op": "checked_callout_fault", "site_id": 0}
]
function["blocks"][4]["instructions"] = [
    {"op": "checked_callout_normal_result", "site_id": 1, "dst": 21},
    {"op": "checked_callout_end", "site_id": 0, "lease_slot": 0},
    {"op": "ret", "value": 21},
]
function["blocks"][5]["instructions"] = [
    {"op": "checked_callout_end", "site_id": 0, "lease_slot": 0},
    {"op": "checked_callout_fault", "site_id": 1},
]
function["blocks"].append({"id": 6, "instructions": [
    {"op": "checked_callout_end", "site_id": 0, "lease_slot": 0},
    {"op": "ret", "value": None},
]})
data["functions"] = [launch, function]
json.dump(data, open(valid_path, "w", encoding="utf-8"))
invalid = copy.deepcopy(data)
invalid["functions"][1]["metadata"]["dynamic_v2_aot_call_admission_v2"]["calls"][1]["wire_revision"] = 99
json.dump(invalid, open(invalid_path, "w", encoding="utf-8"))
invalid_launch = copy.deepcopy(data)
invalid_launch["functions"][0]["metadata"] = copy.deepcopy(invalid_launch["functions"][1]["metadata"])
invalid_launch["functions"][1]["metadata"] = {}
json.dump(invalid_launch, open(invalid_launch_path, "w", encoding="utf-8"))
invalid_arity = copy.deepcopy(data)
invalid_arity["functions"][1]["params"] = [0, 1, 2]
json.dump(invalid_arity, open(invalid_arity_path, "w", encoding="utf-8"))
invalid_missing_helper = copy.deepcopy(data)
invalid_missing_helper["functions"][1]["name"] = "ParserScanLoopBox.not_selected/4"
json.dump(invalid_missing_helper, open(invalid_missing_helper_path, "w", encoding="utf-8"))
invalid_duplicate_helper = copy.deepcopy(data)
invalid_duplicate_helper["functions"].append(copy.deepcopy(function))
json.dump(invalid_duplicate_helper, open(invalid_duplicate_helper_path, "w", encoding="utf-8"))
invalid_duplicate_launch = copy.deepcopy(data)
duplicate_launch = copy.deepcopy(launch)
duplicate_launch["name"] = "ny_main"
invalid_duplicate_launch["functions"].append(duplicate_launch)
json.dump(invalid_duplicate_launch, open(invalid_duplicate_launch_path, "w", encoding="utf-8"))
invalid_missing_launch = copy.deepcopy(data)
invalid_missing_launch["functions"] = [copy.deepcopy(function)]
json.dump(invalid_missing_launch, open(invalid_missing_launch_path, "w", encoding="utf-8"))
invalid_nonzero_launch = copy.deepcopy(data)
invalid_nonzero_launch["functions"][0]["params"] = [0]
json.dump(invalid_nonzero_launch, open(invalid_nonzero_launch_path, "w", encoding="utf-8"))
invalid_physical = copy.deepcopy(data)
invalid_physical["functions"][1]["metadata"]["dynamic_v2_aot_call_admission_v2"]["calls"][0]["source_block"] = 9
json.dump(invalid_physical, open(invalid_physical_path, "w", encoding="utf-8"))
json.dump({
    "kind": "MIR",
    "schema_version": "1.0",
    "metadata": {"extern_c": []},
    "functions": [{
        "name": "ny_main",
        "blocks": [{"id": 0, "instructions": [
            {"op": "const", "dst": 1, "value": {"type": "i64", "value": 0}},
            {"op": "ret", "value": 1},
        ]}],
    }],
}, open(ordinary_path, "w", encoding="utf-8"))
PY

python3 - "$FFI" "$TMP_DIR/valid.json" "$TMP_DIR/valid.o" <<'PY'
import ctypes
import os
import sys

ffi_path, json_path, obj_path = sys.argv[1:]
lib = ctypes.CDLL(ffi_path)
compile_fn = lib.hako_llvmc_compile_json_pure_first
compile_fn.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_void_p)]
compile_fn.restype = ctypes.c_int
error = ctypes.c_void_p()
result = compile_fn(json_path.encode(), obj_path.encode(), ctypes.byref(error))
if result != 0 or not os.path.exists(obj_path):
    message = ctypes.cast(error, ctypes.c_char_p).value.decode() if error.value else "(no error)"
    raise SystemExit(f"positive C1 physicalization failed: rc={result}: {message}")
PY

if ! nm -u "$TMP_DIR/valid.o" | grep -Fq 'hako.text.scan.substring.v1'; then
  echo "[$TAG] positive object is missing substring entry" >&2
  exit 1
fi
if ! nm -u "$TMP_DIR/valid.o" | grep -Fq 'hako.text.scan.index_of.v1'; then
  echo "[$TAG] positive object is missing indexOf entry" >&2
  exit 1
fi
if ! nm -u "$TMP_DIR/valid.o" | grep -Fq 'nyrt_dynamic_v2_lease_consume_end_authorized_v1'; then
  echo "[$TAG] positive object is missing the sole lease C ABI consumer" >&2
  exit 1
fi
if [[ "$(nm -g --defined-only "$TMP_DIR/valid.o" | grep -Fc 'hako_dynamic_v2_static_artifact_descriptor_v1')" -ne 1 ]]; then
  echo "[$TAG] positive object must define one artifact descriptor" >&2
  exit 1
fi
if [[ "$(readelf -W -S "$TMP_DIR/valid.o" | grep -Fc '.hako_dynamic_v2_descriptor')" -ne 1 ]]; then
  echo "[$TAG] positive object must retain one artifact descriptor section" >&2
  exit 1
fi
if ! nm -g --defined-only "$TMP_DIR/valid.o" | grep -Fq 'ParserScanLoopBox.skip_while/4'; then
  echo "[$TAG] positive object must define the selected helper symbol" >&2
  exit 1
fi
if [[ "$(nm -g --defined-only "$TMP_DIR/valid.o" | grep -Fc 'ny_main')" -ne 1 ]]; then
  echo "[$TAG] selected object must define exactly one runtime ny_main launch" >&2
  exit 1
fi
if [[ "$(nm -g --defined-only "$TMP_DIR/valid.o" | grep -Fc 'ParserScanLoopBox.skip_while/4')" -ne 1 ]]; then
  echo "[$TAG] selected object must define exactly one selected helper" >&2
  exit 1
fi

python3 - "$FFI" "$TMP_DIR/ordinary.json" "$TMP_DIR/ordinary.o" <<'PY'
import ctypes
import os
import sys

ffi_path, json_path, obj_path = sys.argv[1:]
lib = ctypes.CDLL(ffi_path)
compile_fn = lib.hako_llvmc_compile_json_pure_first
compile_fn.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_void_p)]
compile_fn.restype = ctypes.c_int
error = ctypes.c_void_p()
result = compile_fn(json_path.encode(), obj_path.encode(), ctypes.byref(error))
if result != 0 or not os.path.exists(obj_path):
    message = ctypes.cast(error, ctypes.c_char_p).value.decode() if error.value else "(no error)"
    raise SystemExit(f"ordinary C1 physicalization failed: rc={result}: {message}")
PY
if nm -g --defined-only "$TMP_DIR/ordinary.o" | grep -Fq 'hako_dynamic_v2_static_artifact_descriptor_v1'; then
  echo "[$TAG] ordinary object unexpectedly defines a Dynamic V2 descriptor" >&2
  exit 1
fi
if readelf -W -S "$TMP_DIR/ordinary.o" | grep -Fq '.hako_dynamic_v2_descriptor'; then
  echo "[$TAG] ordinary object unexpectedly retains a Dynamic V2 descriptor section" >&2
  exit 1
fi

if ! python3 - "$FFI" "$TMP_DIR/invalid.json" "$TMP_DIR/invalid.o" <<'PY'
import ctypes
import os
import sys

ffi_path, json_path, obj_path = sys.argv[1:]
lib = ctypes.CDLL(ffi_path)
compile_fn = lib.hako_llvmc_compile_json_pure_first
compile_fn.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_void_p)]
compile_fn.restype = ctypes.c_int
error = ctypes.c_void_p()
result = compile_fn(json_path.encode(), obj_path.encode(), ctypes.byref(error))
if result == 0 or os.path.exists(obj_path):
    raise SystemExit("negative C1 physicalization unexpectedly succeeded")
PY
then
  echo "[$TAG] negative metadata drift was accepted" >&2
  exit 1
fi

for negative in \
  invalid_launch invalid_arity invalid_missing_helper invalid_duplicate_helper \
  invalid_duplicate_launch invalid_missing_launch invalid_nonzero_launch invalid_physical; do
  if ! python3 - "$FFI" "$TMP_DIR/${negative}.json" "$TMP_DIR/${negative}.o" <<'PY'
import ctypes
import os
import sys

ffi_path, json_path, obj_path = sys.argv[1:]
lib = ctypes.CDLL(ffi_path)
compile_fn = lib.hako_llvmc_compile_json_pure_first
compile_fn.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_void_p)]
compile_fn.restype = ctypes.c_int
error = ctypes.c_void_p()
result = compile_fn(json_path.encode(), obj_path.encode(), ctypes.byref(error))
if result == 0 or os.path.exists(obj_path):
    raise SystemExit("negative dual-identity fixture unexpectedly succeeded")
PY
  then
    echo "[$TAG] negative dual-identity fixture was accepted: ${negative}" >&2
    exit 1
  fi
done

echo "[$TAG] PASS"
