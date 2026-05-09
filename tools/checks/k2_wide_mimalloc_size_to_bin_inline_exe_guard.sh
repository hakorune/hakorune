#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-size-to-bin-inline-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-size-to-bin-inline-proof/main.hako"
APP_README="apps/mimalloc-size-to-bin-inline-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-076-M24-MIMALLOC-SIZE-TO-BIN-INLINE-EXE-PROOF.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"

echo "[$TAG] running M24 mimalloc size_to_bin inline EXE guard"

guard_require_files "$TAG" "$APP" "$APP_README" "$CARD" "$TASKBOARD"

if rg -n 'MI_SIZE_CLASS|MI_CLASS_CAP|MiBinSelector|mimalloc-size-to-bin-inline' lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  echo "[$TAG] ERROR: app-specific size_to_bin matcher leaked into .inc" >&2
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  exit 1
fi
rm -f /tmp/"$TAG".app_specific.inc

cargo test -q mir_optimizer_consumes_verified_profile_allocator_fast_required_inline -- --nocapture
cargo test -q static_const_table_load
pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m24_size_bin.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m24.mir.json"
exe_out="$tmp_dir/m24.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"

python3 - "$mir_json" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as fh:
    data = json.load(fh)

functions = {fn.get("name"): fn for fn in data.get("functions", [])}
helper = functions.get("MiBinSelector.size_to_bin/1")
main = functions.get("main")
if helper is None:
    raise SystemExit("missing MiBinSelector.size_to_bin/1")
if main is None:
    raise SystemExit("missing main")

inline_plans = helper.get("metadata", {}).get("inline_plans", [])
if not any(
    plan.get("request") == "required"
    and plan.get("verified") is True
    and plan.get("source") == "rune_profile:allocator.fast"
    for plan in inline_plans
):
    raise SystemExit(f"missing verified allocator.fast InlinePlan: {inline_plans}")

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

const_regs = set()
static_loads = []
for block in main.get("blocks", []):
    for inst in block.get("instructions", []):
        if inst.get("op") == "const":
            const_regs.add(inst.get("dst"))
        if inst.get("op") == "static_data_load":
            static_loads.append(inst)
        call = inst.get("mir_call") or {}
        callee = call.get("callee") or {}
        if callee.get("type") == "Global" and callee.get("name") == "MiBinSelector.size_to_bin/1":
            raise SystemExit("main still calls MiBinSelector.size_to_bin/1")

for route_key in ("global_call_routes", "lowering_plan"):
    for route in main.get("metadata", {}).get(route_key, []) or []:
        if (
            route.get("symbol") == "MiBinSelector.size_to_bin/1"
            or route.get("target_symbol") == "MiBinSelector.size_to_bin/1"
        ):
            raise SystemExit(f"main still has {route_key} for size_to_bin")

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

print("[m24-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

if rg -F -q 'MiBinSelector.size_to_bin/1' "$build_log"; then
  echo "[$TAG] ERROR: size_to_bin helper must be inlined before backend route trace" >&2
  sed -n '1,200p' "$build_log" >&2
  exit 1
fi

rg -F -q 'mir_call_global_generic_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_alloc_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_free_emit' "$build_log"
rg -F -q 'mir_call_array_slot_append_any_emit' "$build_log"
rg -F -q 'mir_call_array_slot_load_i64_emit' "$build_log"
rg -F -q 'mir_call_array_slot_store_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-size-to-bin-inline-proof' "$run_log"
rg -F -q 'bin=1 table=64/2' "$run_log"
rg -F -q 'allocs=3 frees=1 reused=1 peak=2 free=0' "$run_log"
rg -F -q 'rejects=2' "$run_log"
rg -F -q 'summary=ok' "$run_log"

rg -F -q '`M24 mimalloc size_to_bin inline EXE proof`' "$TASKBOARD"
rg -F -q 'M24 mimalloc size_to_bin inline EXE proof' "$CARD"
rg -F -q '@rune Profile(allocator.fast)' "$APP"
rg -F -q 'k2_wide_mimalloc_size_to_bin_inline_exe_guard.sh' docs/tools/check-scripts-index.md

if rg -F -q 'Profile(' lang/c-abi/shims -g '*.inc'; then
  echo "[$TAG] ERROR: .inc must not consume Profile syntax" >&2
  exit 1
fi

if rg -F -q 'allocator.fast' lang/c-abi/shims lang/src/shared/backend -g '*.inc' -g '*.hako' -g '*.rs'; then
  echo "[$TAG] ERROR: backend/.inc must not branch on profile names" >&2
  exit 1
fi

echo "[$TAG] ok"
