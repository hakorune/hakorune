#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="pinned-text-selected-preflight-smoke"
TEMP_DIR="$(mktemp -d /tmp/hako-pinned-text-preflight.XXXXXX)"
trap 'rm -rf -- "$TEMP_DIR"' EXIT

bash "$ROOT_DIR/tools/build_hako_llvmc_ffi.sh" >/dev/null
"${CC:-cc}" \
  -I"$ROOT_DIR/plugins/nyash-json-plugin/c/yyjson" \
  -o "$TEMP_DIR/verifier-negative" \
  "$ROOT_DIR/lang/c-abi/tests/pinned_text_selected_verifier_test.c" \
  "$ROOT_DIR/lang/c-abi/shims/hako_aot.c" \
  "$ROOT_DIR/lang/c-abi/shims/hako_json_v1.c" \
  "$ROOT_DIR/plugins/nyash-json-plugin/c/yyjson/yyjson.c"
"$TEMP_DIR/verifier-negative"
if ! HAKO_PINNED_TEXT_REAL_CANDIDATE_JSON_OUT="$TEMP_DIR/real.json" \
     CARGO_BUILD_JOBS=4 \
       cargo test --manifest-path "$ROOT_DIR/Cargo.toml" --profile quick --lib -q \
         mir::builder::resolved_lowering::common_v2_s6c_cursor_cfg_tests::pinned_text_real_candidate_json_preserves_carrier_lineage \
         -- --exact >"$TEMP_DIR/cargo.stdout" 2>"$TEMP_DIR/cargo.stderr"; then
  sed -n '1,240p' "$TEMP_DIR/cargo.stderr" >&2
  exit 1
fi

unset HAKO_BACKEND_COMPAT_REPLAY HAKO_CAPI_PURE
ROOT_DIR="$ROOT_DIR" TEMP_DIR="$TEMP_DIR" python3 - <<'PY'
import copy
import ctypes
import json
import os
import pathlib

root = pathlib.Path(os.environ["ROOT_DIR"])
directory = pathlib.Path(os.environ["TEMP_DIR"])
library = ctypes.CDLL(str(root / "target/release/libhako_llvmc_ffi.so"))
compile_json = library.hako_llvmc_compile_json_pure_first
compile_json.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_void_p)]
compile_json.restype = ctypes.c_int
library.hako_mem_free.argtypes = [ctypes.c_void_p]
base = json.loads((directory / "real.json").read_text())


def invoke(name, value, allow_output=False):
    source = directory / f"{name}.json"
    output = directory / f"{name}.o"
    source.write_text(json.dumps(value, separators=(",", ":")))
    error = ctypes.c_void_p()
    rc = compile_json(str(source).encode(), str(output).encode(), ctypes.byref(error))
    message = ctypes.string_at(error.value).decode(errors="replace") if error.value else ""
    if error.value:
        library.hako_mem_free(error)
    if output.exists() and not allow_output:
        raise SystemExit(f"{name}: preflight path published an object")
    return rc, message


rc, message = invoke("valid", base, allow_output=True)
valid_output = directory / "valid.o"
if rc != 0 or message or not valid_output.is_file() or valid_output.stat().st_size == 0:
    raise SystemExit(f"valid: TargetMachine memory handoff failed: {message}")

unwritable = directory / "missing" / "selected.o"
error = ctypes.c_void_p()
source = directory / "unwritable.json"
source.write_text(json.dumps(base, separators=(",", ":")))
rc = compile_json(str(source).encode(), str(unwritable).encode(), ctypes.byref(error))
message = ctypes.string_at(error.value).decode(errors="replace") if error.value else ""
if error.value:
    library.hako_mem_free(error)
if rc == 0 or unwritable.exists() or not message:
    raise SystemExit(f"unwritable: output failure was not contained: {message}")


def reject(name, mutate, expected="pinned Text selected"):
    value = copy.deepcopy(base)
    mutate(value)
    rc, message = invoke(name, value)
    if rc == 0 or expected not in message:
        raise SystemExit(f"{name}: drift was not rejected by strict preflight: {message}")


def function(value):
    return value["functions"][0]


def move_enter(value):
    blocks = function(value)["blocks"]
    blocks[1]["instructions"].append(blocks[0]["instructions"].pop())


def add_uncovered_return(value):
    function(value)["blocks"].append(
        {"id": 99, "instructions": [{"op": "ret", "value": 4}]}
    )


reject(
    "foreign-enter-owner",
    lambda value: function(value)["blocks"][0]["instructions"][0]["owner"].update(slot=99),
)
reject("moved-enter", move_enter, "Enter site mismatch")
reject(
    "foreign-frame-owner",
    lambda value: function(value)["metadata"]["pinned_text_backend_frame_v1"]["owner"].update(slot=99),
    "carrier owner mismatch",
)
reject(
    "duplicate-parameter",
    lambda value: function(value)["params"].__setitem__(1, function(value)["params"][0]),
)
reject(
    "missing-trap",
    lambda value: function(value)["blocks"][2]["instructions"].clear(),
)
reject(
    "trap-finish",
    lambda value: function(value)["blocks"][2]["instructions"].insert(
        0,
        copy.deepcopy(function(value)["blocks"][5]["instructions"][-2]),
    ),
)
reject(
    "moved-finish",
    lambda value: function(value)["blocks"][5]["instructions"].reverse(),
)
reject(
    "plan-relation-drift",
    lambda value: function(value)["blocks"][4]["instructions"][1]["access"].update(lhs_width=123),
)
reject(
    "plan-root-drift",
    lambda value: function(value)["blocks"][4]["instructions"][1]["access"].update(rhs_root=0),
    "scalar equality plan mismatch",
)
reject(
    "plan-stamp-drift",
    lambda value: function(value)["blocks"][4]["instructions"][0].update(plan_stamp=99),
    "plan header mismatch",
)
reject(
    "duplicate-plan",
    lambda value: function(value)["blocks"][4]["instructions"][1].update(plan=1),
)
reject(
    "unknown-pinned-op",
    lambda value: function(value)["blocks"][1]["instructions"].insert(
        0, {"op": "pinned_text_surprise"}
    ),
)
reject(
    "unknown-ordinary-op-after-preflight",
    lambda value: function(value)["blocks"][1]["instructions"].insert(
        0, {"op": "future_ordinary"}
    ),
    "contract-bound module cannot use compatibility replay",
)
reject(
    "missing-return-value",
    lambda value: function(value)["blocks"][5]["instructions"][-1].pop("value"),
    "Return coverage mismatch",
)
reject("uncovered-return", add_uncovered_return, "Return coverage mismatch")
reject(
    "missing-finish",
    lambda value: function(value)["blocks"][5]["instructions"].pop(-2),
    "Return coverage mismatch",
)
reject(
    "frame-plan-count-drift",
    lambda value: function(value)["metadata"]["pinned_text_backend_frame_v1"].update(plan_count=4),
    "physical shape mismatch",
)
reject(
    "root-lane-drift",
    lambda value: function(value)["metadata"]["pinned_text_residence_carrier_v1"]["roots"][1].update(slot_lane=1),
    "root mapping mismatch",
)
reject(
    "extra-function",
    lambda value: value["functions"].append(copy.deepcopy(function(value))),
    "requires one function",
)
reject(
    "missing-carrier",
    lambda value: function(value)["metadata"].pop("pinned_text_residence_carrier_v1"),
    "carrier metadata is missing",
)

leftovers = list(pathlib.Path("/tmp").glob(f"hako_pure_gen_{os.getpid()}.ll"))
if leftovers:
    raise SystemExit(f"preflight left temporary LLVM: {leftovers}")
object_temps = list(directory.rglob("*.ptfb-tm-*.tmp"))
if object_temps:
    raise SystemExit(f"TargetMachine left temporary objects: {object_temps}")

print(
    "[pinned-text-selected-preflight-smoke] ok "
    "(real carrier reaches TargetMachine from memory; drift/output failures clean up)"
)
PY
