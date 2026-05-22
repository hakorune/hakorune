#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-mimalloc-comparison-small-path-slice"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

APP="apps/hako-alloc-mimalloc-comparison-small-path-slice-proof/main.hako"
APP_TEST="apps/hako-alloc-mimalloc-comparison-small-path-slice-proof/test.sh"
APP_README="apps/hako-alloc-mimalloc-comparison-small-path-slice-proof/README.md"
SIZE_CLASS="lang/src/hako_alloc/memory/size_class_box.hako"
PAGE_BOX="lang/src/hako_alloc/memory/page_box.hako"
PAGE_QUEUE="lang/src/hako_alloc/memory/page_queue_box.hako"
PAGE_MAP="lang/src/hako_alloc/memory/page_map_box.hako"
PAGE_RELEASE="lang/src/hako_alloc/memory/page_map_release_box.hako"
TASKBOARD="docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md"
CARD="docs/development/current/main/phases/phase-294x/294x-55-MIMALLOC-COMPARISON-SMALL-PATH-SLICE-PILOT.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_small_path_slice_guard.sh"
OUT="${TMPDIR:-/tmp}/hakorune_mimalloc_comparison_small_path_slice.out"
ERR="${TMPDIR:-/tmp}/hakorune_mimalloc_comparison_small_path_slice.err"
MIR="${TMPDIR:-/tmp}/hakorune_mimalloc_comparison_small_path_slice.mir.json"

echo "[$TAG] checking hako_alloc mimalloc comparison small-path slice"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$SIZE_CLASS" \
  "$PAGE_BOX" \
  "$PAGE_QUEUE" \
  "$PAGE_MAP" \
  "$PAGE_RELEASE" \
  "$TASKBOARD" \
  "$CARD" \
  "$INDEX"

guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.size_class_box as SizeClassBox' "$APP" "small-path slice must consume size-class owner"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.page_box as HakoAllocPageBox' "$APP" "small-path slice must consume page model owner"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.page_queue_box as HakoAllocPageQueueBox' "$APP" "small-path slice must consume page queue owner"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.page_map_box as HakoAllocPageMapBox' "$APP" "small-path slice must consume page map owner"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.page_map_release_box as HakoAllocPageMapReleaseBox' "$APP" "small-path slice must consume page-map release owner"
guard_expect_in_file "$TAG" 'workload=small-path-v1' "$APP" "proof app must expose stable workload id"
guard_expect_in_file "$TAG" 'summary_fields=' "$APP" "proof app must expose stable comparison summary fields"
guard_expect_in_file "$TAG" 'SizeClassBox.size_to_bin_usize' "$APP" "proof app must use exact usize size-class facade"
guard_expect_in_file "$TAG" 'SizeClassBox.good_size_usize' "$APP" "proof app must use exact usize block-size facade"
guard_expect_in_file "$TAG" 'selected_small_a.acquire_usize' "$APP" "proof app must use exact usize page acquire seam"
guard_expect_in_file "$TAG" 'release_seam.releasePtr' "$APP" "proof app must release through page-map-backed release seam"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-VSLICE-003' "$CARD" "card must identify current vertical-slice blocker token"
guard_expect_in_file "$TAG" 'V2' "$CARD" "card must identify V2 small-path slice"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-VSLICE-003' "$TASKBOARD" "taskboard must track V2 blocker"
guard_expect_in_file "$TAG" "$APP" "$INDEX" "check script index must list the small-path slice proof app"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

if rg -n 'remote_free|RemoteFree|Tls|TLS|Atomic|fetch_add|cas_|load_ordered|store_ordered|OSVM|OsVm|provider|hook|replacement|global_allocator|hako_mem_|externcall' \
  "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: small-path comparison slice leaked beyond V2 stop lines" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'hako-alloc-mimalloc-comparison-small-path-slice|small-path-v1|HakoAllocMimallocComparisonSmallPath' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: small-path comparison slice leaked app/owner matcher into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  cargo run -q --bin hakorune -- --backend vm "$ROOT_DIR/$APP" >"$OUT" 2>"$ERR"

grep -q '^hako-alloc-mimalloc-comparison-small-path-slice-proof$' "$OUT"
grep -q '^workload=small-path-v1$' "$OUT"
grep -q '^requests=8,8,24,8$' "$OUT"
grep -q '^bins=1,3$' "$OUT"
grep -q '^block_sizes=8,24$' "$OUT"
grep -q '^queue=3,3,0,0$' "$OUT"
grep -q '^register=1,1,1,1$' "$OUT"
grep -q '^release=1,1,0$' "$OUT"
grep -q '^page_small=3,2,0,0,1,24$' "$OUT"
grep -q '^page_medium=1,1,1,0,24$' "$OUT"
grep -q '^map=4,3,4,1,0$' "$OUT"
grep -q '^summary_fields=4,1,3,48,1$' "$OUT"
grep -q '^summary=ok$' "$OUT"

NYASH_FEATURES="${NYASH_FEATURES:-rune}" \
NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  cargo run -q --bin hakorune -- --emit-mir-json "$MIR" "$ROOT_DIR/$APP" >/tmp/"$TAG".emit.out 2>/tmp/"$TAG".emit.err

python3 - "$MIR" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as fh:
    data = json.load(fh)

functions = {fn.get("name"): fn for fn in data.get("functions", [])}
required = {
    "main",
    "SizeClassBox.size_to_bin_usize/1",
    "SizeClassBox.good_size_usize/1",
    "HakoAllocPageModel.birth/4",
    "HakoAllocPageModel.acquire_usize/1",
    "HakoAllocPageQueue.birth/1",
    "HakoAllocPageQueue.addPage/1",
    "HakoAllocPageQueue.selectPage/0",
    "HakoAllocPageMap.register/3",
    "HakoAllocPageMapReleaseSeam.addPage/1",
    "HakoAllocPageMapReleaseSeam.releasePtr/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

unsupported = []
for fn in functions.values():
    for plan in fn.get("metadata", {}).get("lowering_plan", []):
        if plan.get("emit_kind") == "unsupported":
            unsupported.append((fn.get("name"), plan.get("site"), plan.get("symbol"), plan.get("reason")))
if unsupported:
    raise SystemExit(f"unsupported lowering plans remain: {unsupported[:5]}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for box_name in ("HakoAllocPageModel", "HakoAllocPageQueue", "HakoAllocPageMap", "HakoAllocPageMapReleaseSeam"):
    if plans.get(box_name) is None:
        raise SystemExit(f"missing typed object plan: {box_name}")

page_fields = {field.get("name"): field for field in plans["HakoAllocPageModel"].get("fields", [])}
for name in ("block_size", "capacity", "reserved", "used", "free_top", "local_free_top", "alloc_count", "requested_bytes"):
    field = page_fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"HakoAllocPageModel.{name} must be exact usize storage: {field}")

queue_fields = {field.get("name"): field for field in plans["HakoAllocPageQueue"].get("fields", [])}
for name in ("page_count", "direct_page_index", "add_count", "select_count", "direct_hit_count", "reject_count"):
    field = queue_fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"HakoAllocPageQueue.{name} must be exact usize storage: {field}")

def require_global(owner_name, symbol):
    routes = functions[owner_name].get("metadata", {}).get("lowering_plan", [])
    for route in routes:
        if (
            route.get("route_kind") == "global.user_call"
            and route.get("symbol") == symbol
            and route.get("return_shape") == "ScalarI64"
            and route.get("target_exists") is True
        ):
            return
    raise SystemExit(f"missing global route in {owner_name}: {symbol}")

def require_method(owner_name, box_name, method, return_shape, result_box=None):
    routes = functions[owner_name].get("metadata", {}).get("lowering_plan", [])
    for route in routes:
        if (
            route.get("route_kind") == "user_box.method"
            and route.get("box_name") == box_name
            and route.get("method") == method
            and route.get("target_body_supported") is True
            and route.get("return_shape") == return_shape
            and (result_box is None or route.get("target_result_box_name") == result_box)
        ):
            return
    raise SystemExit(f"missing method route in {owner_name}: {box_name}.{method} -> {return_shape}")

require_global("main", "SizeClassBox.size_to_bin_usize/1")
require_global("main", "SizeClassBox.good_size_usize/1")
require_method("main", "HakoAllocPageQueue", "selectPage", "object_handle", "HakoAllocPageModel")
require_method("main", "HakoAllocPageModel", "acquire_usize", "scalar_i64")
require_method("main", "HakoAllocPageMap", "register", "scalar_i64")
require_method("main", "HakoAllocPageMapReleaseSeam", "releasePtr", "scalar_i64")

print("[small-path-slice-mir-json] ok")
PY

rm -f /tmp/"$TAG".emit.out /tmp/"$TAG".emit.err

cat "$OUT"

echo "[$TAG] ok"
