#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-mimalloc-comparison-realloc-aligned-slice"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

APP="apps/hako-alloc-mimalloc-comparison-realloc-aligned-slice-proof/main.hako"
APP_TEST="apps/hako-alloc-mimalloc-comparison-realloc-aligned-slice-proof/test.sh"
APP_README="apps/hako-alloc-mimalloc-comparison-realloc-aligned-slice-proof/README.md"
FACADE="lang/src/hako_alloc/memory/allocator_facade_box.hako"
ALIGNED_PATH="lang/src/hako_alloc/memory/page_map_aligned_small_path_box.hako"
PAGE_BOX="lang/src/hako_alloc/memory/page_box.hako"
PAGE_MAP="lang/src/hako_alloc/memory/page_map_box.hako"
PAGE_RELEASE="lang/src/hako_alloc/memory/page_map_release_box.hako"
TASKBOARD="docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md"
CARD="docs/development/current/main/phases/phase-294x/294x-57-MIMALLOC-COMPARISON-REALLOC-ALIGNED-SLICE-PILOT.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_realloc_aligned_slice_guard.sh"
OUT="${TMPDIR:-/tmp}/hakorune_mimalloc_comparison_realloc_aligned_slice.out"
ERR="${TMPDIR:-/tmp}/hakorune_mimalloc_comparison_realloc_aligned_slice.err"
MIR="${TMPDIR:-/tmp}/hakorune_mimalloc_comparison_realloc_aligned_slice.mir.json"

echo "[$TAG] checking hako_alloc mimalloc comparison realloc/aligned slice"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$FACADE" \
  "$ALIGNED_PATH" \
  "$PAGE_BOX" \
  "$PAGE_MAP" \
  "$PAGE_RELEASE" \
  "$TASKBOARD" \
  "$CARD" \
  "$INDEX"

guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.allocator_facade_box as HakoAllocFacade' "$APP" "V3 proof must consume production facade realloc owner"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.page_map_aligned_small_path_box as HakoAllocPageMapAlignedSmallPathBox' "$APP" "V3 proof must keep aligned small-path owner"
guard_expect_in_file "$TAG" 'workload=realloc-aligned-v1' "$APP" "proof app must expose stable workload id"
guard_expect_in_file "$TAG" 'summary_fields=' "$APP" "proof app must expose stable comparison summary fields"
guard_expect_in_file "$TAG" 'reallocResult' "$APP" "proof app must route realloc through production facade"
guard_expect_in_file "$TAG" 'isLiveHandle' "$APP" "proof app must observe handle liveness through production facade"
guard_expect_in_file "$TAG" 'allocateAlignedSmallUsize' "$APP" "proof app must use exact usize aligned-small facade"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-VSLICE-005' "$CARD" "card must identify current vertical-slice blocker token"
guard_expect_in_file "$TAG" 'V3' "$CARD" "card must identify V3 realloc/aligned slice"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-VSLICE-005' "$TASKBOARD" "taskboard must track V3 blocker"
guard_expect_in_file "$TAG" "$APP" "$INDEX" "check script index must list the V3 proof app"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

if rg -n 'remote_free|RemoteFree|Tls|TLS|Atomic|fetch_add|cas_|load_ordered|store_ordered|OSVM|OsVm|provider|hook|replacement|global_allocator|hako_mem_|externcall' \
  "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: realloc/aligned comparison slice leaked beyond V3 stop lines" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'hako-alloc-mimalloc-comparison-realloc-aligned-slice|realloc-aligned-v1|HakoAllocMimallocComparisonReallocAligned' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: realloc/aligned comparison slice leaked app/owner matcher into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  cargo run -q --bin hakorune -- --backend vm "$ROOT_DIR/$APP" >"$OUT" 2>"$ERR"

grep -q '^hako-alloc-mimalloc-comparison-realloc-aligned-slice-proof$' "$OUT"
grep -q '^workload=realloc-aligned-v1$' "$OUT"
grep -q '^same=1,1001,0,1,1$' "$OUT"
grep -q '^grow=1,9000,0,1,1,0,0$' "$OUT"
grep -q '^aligned=1,12000,8,31,1,12001,64,111,0$' "$OUT"
grep -q '^requested_bytes=216$' "$OUT"
grep -q '^copied_bytes_model=16$' "$OUT"
grep -q '^live_handles=4$' "$OUT"
grep -q '^rejects=4$' "$OUT"
grep -q '^release_count=1$' "$OUT"
grep -q '^alignment_meta=2$' "$OUT"
grep -q '^summary_fields=216,16,4,4,1,2$' "$OUT"
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
    "HakoAllocProductionFacade.allocate/1",
    "HakoAllocProductionFacade.reallocResult/2",
    "HakoAllocProductionFacade.isLiveHandle/1",
    "HakoAllocPageMapAlignedSmallPath.allocateAlignedSmallUsize/2",
    "HakoAllocPageMapAlignedSmallPath.alignmentFor/1",
    "HakoAllocPageMapAlignedSmallPath.paddedSizeFor/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

unsupported = []
inspect_prefixes = (
    "main",
    "HakoAllocProductionFacade.",
    "HakoAllocPageMapAlignedSmallPath.",
    "HakoAllocPageMapReleaseSeam.",
    "HakoAllocPageModel.",
)
for fn in functions.values():
    name = fn.get("name") or ""
    if not any(name == prefix or name.startswith(prefix) for prefix in inspect_prefixes):
        continue
    for plan in fn.get("metadata", {}).get("lowering_plan", []):
        if plan.get("emit_kind") == "unsupported":
            unsupported.append((name, plan.get("site"), plan.get("symbol"), plan.get("reason")))
if unsupported:
    raise SystemExit(f"unsupported lowering plans remain: {unsupported[:5]}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}

def require_usize_fields(box_name, names):
    plan = plans.get(box_name)
    if plan is None:
        raise SystemExit(f"missing typed object plan: {box_name}")
    fields = {field.get("name"): field for field in plan.get("fields", [])}
    for name in names:
        field = fields.get(name)
        if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
            raise SystemExit(f"{box_name}.{name} must be exact usize storage: {field}")

require_usize_fields(
    "HakoAllocProductionFacade",
    ("alloc_count", "free_count", "reject_count", "realloc_success_count", "realloc_reject_count"),
)
require_usize_fields(
    "HakoAllocPageMapAlignedSmallPath",
    ("meta_count", "alloc_count", "invalid_alignment_count", "oversized_count", "alloc_fail_count", "register_fail_count", "reject_count"),
)

def require_method(owner_name, box_name, method):
    routes = functions[owner_name].get("metadata", {}).get("lowering_plan", [])
    for route in routes:
        if (
            route.get("route_kind") == "user_box.method"
            and route.get("box_name") == box_name
            and route.get("method") == method
            and route.get("target_body_supported") is True
        ):
            return
    raise SystemExit(f"missing method route in {owner_name}: {box_name}.{method}")

require_method("main", "HakoAllocProductionFacade", "allocate")
require_method("main", "HakoAllocProductionFacade", "reallocResult")
require_method("main", "HakoAllocProductionFacade", "isLiveHandle")
require_method("main", "HakoAllocPageMapAlignedSmallPath", "allocateAlignedSmallUsize")
require_method("main", "HakoAllocPageMapAlignedSmallPath", "alignmentFor")
require_method("main", "HakoAllocPageMapAlignedSmallPath", "paddedSizeFor")

print("[realloc-aligned-slice-mir-json] ok")
PY

rm -f /tmp/"$TAG".emit.out /tmp/"$TAG".emit.err

cat "$OUT"

echo "[$TAG] ok"
