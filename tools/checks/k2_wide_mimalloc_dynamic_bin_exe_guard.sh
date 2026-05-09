#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-dynamic-bin-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-dynamic-bin-proof/main.hako"
APP_README="apps/mimalloc-dynamic-bin-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-075-M23-MIMALLOC-DYNAMIC-BIN-EXE-PROOF.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"

echo "[$TAG] running M23 mimalloc dynamic bin EXE guard"

guard_require_files "$TAG" "$APP" "$APP_README" "$CARD" "$TASKBOARD"

if rg -n 'MI_SIZE_CLASS|MI_CLASS_CAP|mimalloc-dynamic-bin' lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  echo "[$TAG] ERROR: app-specific dynamic-bin matcher leaked into .inc" >&2
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  exit 1
fi
rm -f /tmp/"$TAG".app_specific.inc

cargo test -q static_const_table_load
pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m23_dyn_bin.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m23.mir.json"
exe_out="$tmp_dir/m23.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"

python3 - "$mir_json" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as fh:
    data = json.load(fh)

plans = {
    plan.get("source_name"): plan
    for plan in data.get("static_data_plans", [])
}
expected = {
    "MI_SIZE_CLASS": [32, 64],
    "MI_CLASS_CAP": [4, 2],
}
for name, values in expected.items():
    plan = plans.get(name)
    if plan is None:
        raise SystemExit(f"missing static data plan: {name}")
    if plan.get("element") != "u16" or plan.get("values") != values:
        raise SystemExit(f"unexpected static data plan {name}: {plan}")

functions = {fn.get("name"): fn for fn in data.get("functions", [])}
main = functions.get("main")
if main is None:
    raise SystemExit("missing main")

const_regs = set()
static_loads = []
for block in main.get("blocks", []):
    for inst in block.get("instructions", []):
        if inst.get("op") == "const":
            const_regs.add(inst.get("dst"))
        if inst.get("op") == "static_data_load":
            static_loads.append(inst)

for name in expected:
    if name not in [inst.get("source_name") for inst in static_loads]:
        raise SystemExit(f"missing static_data_load for {name}: {static_loads}")

dynamic_sources = {
    inst.get("source_name")
    for inst in static_loads
    if inst.get("index") not in const_regs
}
for name in expected:
    if name not in dynamic_sources:
        raise SystemExit(
            f"missing dynamic-index static_data_load for {name}: {static_loads}"
        )

routes = main.get("metadata", {}).get("global_call_routes", [])
symbols = {
    "RawBufCoreBox.alloc_bytes_i64/1",
    "RawBufCoreBox.free_bytes_i64/1",
    "RawArrayCoreBox.slot_append_any/2",
    "RawArrayCoreBox.slot_load_i64/2",
    "RawArrayCoreBox.slot_store_i64/3",
}
for symbol in symbols:
    for route in routes:
        if (
            route.get("symbol") == symbol
            and route.get("target_shape") == "generic_i64_body"
            and route.get("proof") == "typed_global_call_generic_i64"
        ):
            break
    else:
        raise SystemExit(f"missing generic-i64 route main -> {symbol}: {routes}")

print("[m23-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_global_generic_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_alloc_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_free_emit' "$build_log"
rg -F -q 'mir_call_array_slot_append_any_emit' "$build_log"
rg -F -q 'mir_call_array_slot_load_i64_emit' "$build_log"
rg -F -q 'mir_call_array_slot_store_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-dynamic-bin-proof' "$run_log"
rg -F -q 'class=1 table=32,64/2' "$run_log"
rg -F -q 'allocs=3 frees=1 reused=1 peak=2 free=0' "$run_log"
rg -F -q 'rejects=2' "$run_log"
rg -F -q 'summary=ok' "$run_log"

rg -F -q '`M23 mimalloc dynamic bin EXE proof`' "$TASKBOARD"
rg -F -q 'M23 mimalloc dynamic bin EXE proof' "$CARD"
rg -F -q 'non-constant static table indices' "$APP_README"
rg -F -q 'k2_wide_mimalloc_dynamic_bin_exe_guard.sh' docs/tools/check-scripts-index.md

echo "[$TAG] ok"
