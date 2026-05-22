#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-page-queue"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

QUEUE_BOX="lang/src/hako_alloc/memory/page_queue_box.hako"
PAGE_BOX="lang/src/hako_alloc/memory/page_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
APP="apps/mimalloc-page-queue-proof/main.hako"
APP_TEST="apps/mimalloc-page-queue-proof/test.sh"
APP_README="apps/mimalloc-page-queue-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-167-M166-MIMALLOC-PAGE-QUEUE-DIRECT-CACHE.md"
USIZE_CARD="docs/development/current/main/phases/phase-294x/294x-28-HAKO-ALLOC-USIZE-PAGE-QUEUE-COUNTERS.md"
USIZE_COUNT_CARD="docs/development/current/main/phases/phase-294x/294x-48-HAKO-ALLOC-USIZE-PAGE-QUEUE-PAGE-COUNT.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_page_queue_guard.sh"
OUT="${TMPDIR:-/tmp}/hakorune_mimalloc_page_queue.out"
ERR="${TMPDIR:-/tmp}/hakorune_mimalloc_page_queue.err"
MIR="${TMPDIR:-/tmp}/hakorune_mimalloc_page_queue.mir.json"

echo "[$TAG] checking M166 mimalloc page queue/direct-cache"

guard_require_files \
  "$TAG" \
  "$QUEUE_BOX" \
  "$PAGE_BOX" \
  "$MODULE" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$CARD" \
  "$USIZE_CARD" \
  "$USIZE_COUNT_CARD" \
  "$PLAN" \
  "$INDEX" \
  "$ALLOCATOR_GROUP"

guard_expect_in_file "$TAG" 'box HakoAllocPageQueue' "$QUEUE_BOX" "HakoAllocPageQueue must own page selection"
guard_expect_in_file "$TAG" 'pages: ArrayBox = new ArrayBox\(\)' "$QUEUE_BOX" "page queue must initialize pages as a stored member"
guard_expect_in_file "$TAG" 'page_count: usize = 0' "$QUEUE_BOX" "page queue length must be exact usize storage"
guard_expect_in_file "$TAG" 'has_direct_page: i64 = 0' "$QUEUE_BOX" "page queue must initialize direct-page presence state"
guard_expect_in_file "$TAG" 'direct_page_index: i64 = 0' "$QUEUE_BOX" "page queue must keep direct-page index non-negative"
guard_expect_in_file "$TAG" 'add_count: usize = 0' "$QUEUE_BOX" "page queue add counter must be exact usize"
guard_expect_in_file "$TAG" 'select_count: usize = 0' "$QUEUE_BOX" "page queue select counter must be exact usize"
guard_expect_in_file "$TAG" 'direct_hit_count: usize = 0' "$QUEUE_BOX" "page queue direct-hit counter must be exact usize"
guard_expect_in_file "$TAG" 'refresh_count: usize = 0' "$QUEUE_BOX" "page queue refresh counter must be exact usize"
guard_expect_in_file "$TAG" 'reject_count: usize = 0' "$QUEUE_BOX" "page queue reject counter must be exact usize"
guard_expect_in_file "$TAG" 'selectPage' "$QUEUE_BOX" "page queue must expose selectPage"
guard_expect_in_file "$TAG" 'refreshDirectPage' "$QUEUE_BOX" "page queue must expose refreshDirectPage"
guard_expect_in_file "$TAG" 'freeCount' "$QUEUE_BOX" "page queue must observe page availability only"
guard_expect_in_file "$TAG" 'memory.page_queue_box = "memory/page_queue_box.hako"' "$MODULE" "hako module must export page_queue_box"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.page_queue_box as HakoAllocPageQueueBox' "$APP" "proof app must import page_queue_box"
guard_expect_in_file "$TAG" 'M166 page queue and direct-page cache' "$PLAN" "plan must retain M166 row"
guard_expect_in_file "$TAG" '293x-167 M166 Mimalloc Page Queue Direct Cache' "$CARD" "missing M166 card"
guard_expect_in_file "$TAG" '294x-28 Hako Alloc Usize Page Queue Counters' "$USIZE_CARD" "missing page queue counter usize card"
guard_expect_in_file "$TAG" '294x-48 Hako Alloc Usize Page Queue Page Count' "$USIZE_COUNT_CARD" "missing page queue page-count usize card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M166 guard"

if rg -n 'init[[:space:]]*\\{' "$QUEUE_BOX" >/tmp/"$TAG".legacy_init 2>&1; then
  echo "[$TAG] ERROR: new page queue must use Unified Members stored fields, not legacy init slots" >&2
  cat /tmp/"$TAG".legacy_init >&2
  rm -f /tmp/"$TAG".legacy_init
  exit 1
fi
rm -f /tmp/"$TAG".legacy_init

if rg -n '\.acquire\(' "$QUEUE_BOX" >/tmp/"$TAG".acquire 2>&1; then
  echo "[$TAG] ERROR: M166 queue must choose pages, not pop allocation blocks" >&2
  cat /tmp/"$TAG".acquire >&2
  rm -f /tmp/"$TAG".acquire
  exit 1
fi
rm -f /tmp/"$TAG".acquire

if rg -n 'direct_page_index: i64 = -1|direct_page_index[[:space:]]*<[[:space:]]*0|direct_page_index[[:space:]]*=[[:space:]]*-1|found_index[[:space:]]*=[[:space:]]*-1' "$QUEUE_BOX" >/tmp/"$TAG".sentinel 2>&1; then
  echo "[$TAG] ERROR: direct-page cache must use explicit presence state, not -1 sentinel storage" >&2
  cat /tmp/"$TAG".sentinel >&2
  rm -f /tmp/"$TAG".sentinel
  exit 1
fi
rm -f /tmp/"$TAG".sentinel

if rg -n 'OSVM|OsVm|Tls|Atomic|remote_free|RemoteFree|fetch_add|cas_|load_ordered|store_ordered|page_map|replacement|hook' "$QUEUE_BOX" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: M166+ or substrate ownership leaked into page queue" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -F -q "$SELF_SCRIPT" "$ALLOCATOR_GROUP"; then
  guard_fail "$TAG" "M166 focused guard must not be registered as another wide allocator gate step"
fi

if rg -n 'mimalloc-page-queue|HakoAllocPageQueue|page_queue_box|direct_page_index' lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: page queue matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

if ! guard_timeout_run "$TAG" "${MIMAP_VM_TIMEOUT:-25s}" "$OUT" "$ERR" env NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" cargo run -q --bin hakorune -- --backend vm "$ROOT_DIR/$APP"; then
  cat "$OUT" >&2 || true
  cat "$ERR" >&2 || true
  guard_fail "$TAG" "VM page queue proof failed or timed out"
fi

grep -q '^mimalloc-page-queue-proof$' "$OUT"
grep -q '^entries=0,1,2$' "$OUT"
grep -q '^ids=10,11,-1,12$' "$OUT"
grep -q '^direct=1,2,12$' "$OUT"
grep -q '^counts=3,4,2,2,1$' "$OUT"
grep -q '^shape=10$' "$OUT"
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
    "HakoAllocPageQueue.birth/1",
    "HakoAllocPageQueue.addPage/1",
    "HakoAllocPageQueue.selectPage/0",
    "HakoAllocPageQueue.refreshDirectPage/0",
    "HakoAllocPageQueue.directPageId/0",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
queue = plans.get("HakoAllocPageQueue")
if queue is None:
    raise SystemExit("missing typed object plan: HakoAllocPageQueue")
fields = {field.get("name"): field for field in queue.get("fields", [])}
for name in ("add_count", "select_count", "direct_hit_count", "refresh_count", "reject_count", "page_count"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"page queue {name} must be exact usize storage: {field}")
for name in ("bin", "has_direct_page", "direct_page_index"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"page queue {name} must remain i64 storage: {field}")
PY

rm -f /tmp/"$TAG".emit.out /tmp/"$TAG".emit.err

cat "$OUT"

echo "[$TAG] ok"
