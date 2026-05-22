#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-page-map"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

PAGE_MAP="lang/src/hako_alloc/memory/page_map_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
APP="apps/mimalloc-page-map-proof/main.hako"
APP_TEST="apps/mimalloc-page-map-proof/test.sh"
APP_README="apps/mimalloc-page-map-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-179-M171-MIMALLOC-PAGE-MAP-MODEL.md"
USIZE_CARD="docs/development/current/main/phases/phase-294x/294x-21-HAKO-ALLOC-USIZE-PAGE-MAP-COUNTERS.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_page_map_guard.sh"
OUT="${TMPDIR:-/tmp}/hakorune_mimalloc_page_map.out"
ERR="${TMPDIR:-/tmp}/hakorune_mimalloc_page_map.err"

echo "[$TAG] checking M171 mimalloc page-map model"

guard_require_files \
  "$TAG" \
  "$PAGE_MAP" \
  "$MODULE" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$CARD" \
  "$USIZE_CARD" \
  "$PLAN" \
  "$INDEX"

guard_expect_in_file "$TAG" 'memory.page_map_box = "memory/page_map_box.hako"' "$MODULE" "hako module must export page_map_box"
guard_expect_in_file "$TAG" 'box HakoAllocPageMapEntry' "$PAGE_MAP" "page-map entry owner must exist"
guard_expect_in_file "$TAG" 'box HakoAllocPageMap' "$PAGE_MAP" "page-map owner must exist"
guard_expect_in_file "$TAG" 'register\(ptr, page_id, block_id\)' "$PAGE_MAP" "page-map must expose register"
guard_expect_in_file "$TAG" 'lookup\(ptr\)' "$PAGE_MAP" "page-map must expose lookup"
guard_expect_in_file "$TAG" 'unregister\(ptr\)' "$PAGE_MAP" "page-map must expose unregister"
guard_expect_in_file "$TAG" 'entry_count: usize = 0' "$PAGE_MAP" "page-map entry counter must be exact usize"
guard_expect_in_file "$TAG" 'live_count: usize = 0' "$PAGE_MAP" "page-map live counter must be exact usize"
guard_expect_in_file "$TAG" 'register_count: usize = 0' "$PAGE_MAP" "page-map register counter must be exact usize"
guard_expect_in_file "$TAG" 'lookup_count: usize = 0' "$PAGE_MAP" "page-map lookup counter must be exact usize"
guard_expect_in_file "$TAG" 'lookup_miss_count: usize = 0' "$PAGE_MAP" "page-map lookup-miss counter must be exact usize"
guard_expect_in_file "$TAG" 'unregister_count: usize = 0' "$PAGE_MAP" "page-map unregister counter must be exact usize"
guard_expect_in_file "$TAG" 'reject_count: usize = 0' "$PAGE_MAP" "page-map reject counter must be exact usize"
guard_expect_in_file "$TAG" 'ptr: i64' "$PAGE_MAP" "page-map entry pointer must stay i64"
guard_expect_in_file "$TAG" 'page_id: i64' "$PAGE_MAP" "page-map entry page id must stay i64"
guard_expect_in_file "$TAG" 'block_id: i64' "$PAGE_MAP" "page-map entry block id must stay i64"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.page_map_box as HakoAllocPageMapBox' "$APP" "proof app must import page_map_box"
guard_expect_in_file "$TAG" 'M171 page-map model' "$PLAN" "plan must retain M171 row"
guard_expect_in_file "$TAG" '293x-179 M171 Mimalloc Page-Map Model' "$CARD" "missing M171 card"
guard_expect_in_file "$TAG" '294x-21 Hako Alloc Usize Page-Map Counters' "$USIZE_CARD" "missing page-map usize counter card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M171 guard"

if rg -n 'init[[:space:]]*\{' "$PAGE_MAP" >/tmp/"$TAG".legacy_init 2>&1; then
  echo "[$TAG] ERROR: M171 page-map must use Unified Members stored fields, not legacy init slots" >&2
  cat /tmp/"$TAG".legacy_init >&2
  rm -f /tmp/"$TAG".legacy_init
  exit 1
fi
rm -f /tmp/"$TAG".legacy_init

if rg -n 'externcall|hako_mem_|OSVM|OsVm|Tls|Atomic|remote_free|RemoteFree|fetch_add|cas_|load_ordered|store_ordered|provider|hook|replacement|realloc|releaseLocal|HakoAllocPageModel|HakoAllocRemoteFreePolicy' \
  "$PAGE_MAP" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: M171 leaked out of pure page-map model scope" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-page-map|HakoAllocPageMap|page_map_box' lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: page-map app/box matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  cargo run -q --bin hakorune -- --backend vm "$ROOT_DIR/$APP" >"$OUT" 2>"$ERR"

grep -q '^mimalloc-page-map-proof$' "$OUT"
grep -q '^register=1,1,0,1,0$' "$OUT"
grep -q '^unregister=1,0$' "$OUT"
grep -q '^shape=5$' "$OUT"
grep -q '^counts=3,2,3,5,2,1,3$' "$OUT"
grep -q '^summary=ok$' "$OUT"

tmp_dir="$(mktemp -d /tmp/hakorune_m171_page_map.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/page_map.mir.json"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"

python3 - "$mir_json" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as fh:
    data = json.load(fh)

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
page_map = plans.get("HakoAllocPageMap")
if page_map is None:
    raise SystemExit("missing typed object plan: HakoAllocPageMap")

fields = {field.get("name"): field for field in page_map.get("fields", [])}
for name in (
    "entry_count",
    "live_count",
    "register_count",
    "lookup_count",
    "lookup_miss_count",
    "unregister_count",
    "reject_count",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"page-map {name} must be exact usize storage: {field}")

entry = plans.get("HakoAllocPageMapEntry")
if entry is None:
    raise SystemExit("missing typed object plan: HakoAllocPageMapEntry")
entry_fields = {field.get("name"): field for field in entry.get("fields", [])}
for name in ("ptr", "page_id", "block_id", "live"):
    field = entry_fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"page-map entry {name} must remain i64 storage: {field}")

print("[m171-page-map-mir-json] ok")
PY

cat "$OUT"

echo "[$TAG] ok"
