#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-page-model"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

PAGE_BOX="lang/src/hako_alloc/memory/page_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
APP="apps/mimalloc-page-model-proof/main.hako"
APP_TEST="apps/mimalloc-page-model-proof/test.sh"
APP_README="apps/mimalloc-page-model-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-166-M165-MIMALLOC-PAGE-MODEL-SPLIT.md"
USIZE_CARD="docs/development/current/main/phases/phase-294x/294x-29-HAKO-ALLOC-USIZE-PAGE-MODEL-LOCAL-COUNTERS.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_page_model_guard.sh"
OUT="${TMPDIR:-/tmp}/hakorune_mimalloc_page_model.out"
ERR="${TMPDIR:-/tmp}/hakorune_mimalloc_page_model.err"
MIR="${TMPDIR:-/tmp}/hakorune_mimalloc_page_model.mir.json"

echo "[$TAG] checking M165 mimalloc page model split"

guard_require_files \
  "$TAG" \
  "$PAGE_BOX" \
  "$MODULE" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$CARD" \
  "$USIZE_CARD" \
  "$PLAN" \
  "$INDEX" \
  "$ALLOCATOR_GROUP"

guard_expect_in_file "$TAG" 'box HakoAllocPageModel' "$PAGE_BOX" "HakoAllocPageModel must own page-local state"
guard_expect_in_file "$TAG" 'free: ArrayBox = new ArrayBox\(\)' "$PAGE_BOX" "page model must initialize free as a stored member"
guard_expect_in_file "$TAG" 'local_free: ArrayBox = new ArrayBox\(\)' "$PAGE_BOX" "page model must initialize local_free as a stored member"
guard_expect_in_file "$TAG" 'block_used: ArrayBox = new ArrayBox\(\)' "$PAGE_BOX" "page model must initialize block_used"
guard_expect_in_file "$TAG" 'used: i64 = 0' "$PAGE_BOX" "page model must expose used with a scalar field initializer"
guard_expect_in_file "$TAG" 'capacity: i64' "$PAGE_BOX" "page model must expose capacity as a scalar stored member"
guard_expect_in_file "$TAG" 'reserved: i64' "$PAGE_BOX" "page model must expose reserved as a scalar stored member"
guard_expect_in_file "$TAG" 'alloc_count: usize = 0' "$PAGE_BOX" "page alloc counter must be exact usize"
guard_expect_in_file "$TAG" 'local_free_count: usize = 0' "$PAGE_BOX" "page local-free counter must be exact usize"
guard_expect_in_file "$TAG" 'reject_count: usize = 0' "$PAGE_BOX" "page reject counter must be exact usize"
guard_expect_in_file "$TAG" 'local_free_collect_count: usize = 0' "$PAGE_BOX" "local-free collection counter must be exact usize"
guard_expect_in_file "$TAG" 'local_free_collected_blocks: usize = 0' "$PAGE_BOX" "collected-block counter must be exact usize"
guard_expect_in_file "$TAG" 'seedFreeBlocks' "$PAGE_BOX" "page model must seed free blocks locally"
guard_expect_in_file "$TAG" 'releaseLocal' "$PAGE_BOX" "page model must have local release seam"
guard_expect_in_file "$TAG" 'memory.page_box = "memory/page_box.hako"' "$MODULE" "hako module must export page_box"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.page_box as HakoAllocPageBox' "$APP" "proof app must import page_box"
guard_expect_in_file "$TAG" 'local_free' "$APP_README" "proof README must describe local_free"
guard_expect_in_file "$TAG" 'M165 page model split' "$PLAN" "plan must retain M165 row"
guard_expect_in_file "$TAG" '293x-166 M165 Mimalloc Page Model Split' "$CARD" "missing M165 card"
guard_expect_in_file "$TAG" '294x-29 Hako Alloc Usize Page Model Local Counters' "$USIZE_CARD" "missing page model local counter usize card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M165 guard"
guard_expect_in_file "$TAG" 'loop\(i < me\.capacity\)' "$PAGE_BOX" "page seeding must exercise JoinIR field-read loop bound"

if rg -n 'init[[:space:]]*\\{' "$PAGE_BOX" >/tmp/"$TAG".legacy_init 2>&1; then
  echo "[$TAG] ERROR: new page model must use Unified Members stored fields, not legacy init slots" >&2
  cat /tmp/"$TAG".legacy_init >&2
  rm -f /tmp/"$TAG".legacy_init
  exit 1
fi
rm -f /tmp/"$TAG".legacy_init

if rg -n 'page_queue|queue|direct_page|OSVM|OsVm|Tls|Atomic|remote_free|RemoteFree|fetch_add|cas_|load_ordered|store_ordered' "$PAGE_BOX" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: M166+ or substrate ownership leaked into M165 page model" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -F -q "$SELF_SCRIPT" "$ALLOCATOR_GROUP"; then
  guard_fail "$TAG" "M165 focused guard must not be registered as another wide allocator gate step"
fi

if rg -n 'mimalloc-page-model|HakoAllocPageModel|page_box|local_free' lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: page model matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  cargo run -q --bin hakorune -- --backend vm "$ROOT_DIR/$APP" >"$OUT" 2>"$ERR"

grep -q '^mimalloc-page-model-proof$' "$OUT"
grep -q '^blocks=2,1,0,-1,1$' "$OUT"
grep -q '^state=3,3,5,0,0$' "$OUT"
grep -q '^counts=4,1,3,3,56$' "$OUT"
grep -q '^shape=13$' "$OUT"
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
    "HakoAllocPageModel.birth/4",
    "HakoAllocPageModel.acquire/1",
    "HakoAllocPageModel.releaseLocal/1",
    "HakoAllocPageModel.freeCount/0",
    "HakoAllocPageModel.localFreeCount/0",
    "HakoAllocPageModel.availableBlockCount/0",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
page = plans.get("HakoAllocPageModel")
if page is None:
    raise SystemExit("missing typed object plan: HakoAllocPageModel")
fields = {field.get("name"): field for field in page.get("fields", [])}
for name in (
    "alloc_count",
    "local_free_count",
    "local_free_collect_count",
    "local_free_collected_blocks",
    "reject_count",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"page model {name} must be exact usize storage: {field}")
for name in (
    "page_id",
    "block_size",
    "capacity",
    "reserved",
    "used",
    "free_top",
    "local_free_top",
    "retired",
    "decommitted",
    "retire_count",
    "decommit_count",
    "recommit_count",
    "reuse_count",
    "lifecycle_reject_count",
    "reactivate_count",
    "reactivate_reject_count",
    "peak_used",
    "requested_bytes",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"page model {name} must remain i64 storage: {field}")
PY

rm -f /tmp/"$TAG".emit.out /tmp/"$TAG".emit.err

cat "$OUT"

echo "[$TAG] ok"
