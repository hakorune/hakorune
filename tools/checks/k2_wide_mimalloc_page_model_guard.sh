#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-page-model"
cd "$ROOT_DIR"
source tools/checks/lib/pure_first_exe_guard.sh

PAGE_BOX="lang/src/hako_alloc/memory/page_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
APP="apps/mimalloc-page-model-proof/main.hako"
APP_TEST="apps/mimalloc-page-model-proof/test.sh"
APP_README="apps/mimalloc-page-model-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-166-M165-MIMALLOC-PAGE-MODEL-SPLIT.md"
USIZE_CARD="docs/development/current/main/phases/phase-294x/294x-31-HAKO-ALLOC-USIZE-PAGE-MODEL-LIFECYCLE-COUNTERS.md"
USIZE_STACK_CARD="docs/development/current/main/phases/phase-294x/294x-43-HAKO-ALLOC-USIZE-PAGE-MODEL-STACK-OCCUPANCY.md"
USIZE_CAPACITY_CARD="docs/development/current/main/phases/phase-294x/294x-45-HAKO-ALLOC-USIZE-PAGE-MODEL-CAPACITY.md"
USIZE_SIZE_CARD="docs/development/current/main/phases/phase-294x/294x-47-HAKO-ALLOC-USIZE-PAGE-MODEL-SIZE-BYTES.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_page_model_guard.sh"
DIRECT_EXACT_ENV="tools/allocator/mimalloc_direct_exact_env.sh"
OUT="${TMPDIR:-/tmp}/hakorune_mimalloc_page_model.out"
ERR="${TMPDIR:-/tmp}/hakorune_mimalloc_page_model.err"
MIR="${TMPDIR:-/tmp}/hakorune_mimalloc_page_model.mir.json"
EXE_DIR="$ROOT_DIR/target/checks/$TAG/tmp"
EXE_OUT="$EXE_DIR/app.exe"
BUILD_LOG="$EXE_DIR/build.log"
RUN_LOG="$EXE_DIR/exe.out"

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
  "$USIZE_STACK_CARD" \
  "$USIZE_CAPACITY_CARD" \
  "$USIZE_SIZE_CARD" \
  "$PLAN" \
  "$INDEX" \
  "$ALLOCATOR_GROUP" \
  "$DIRECT_EXACT_ENV"

guard_expect_in_file "$TAG" 'box HakoAllocPageModel' "$PAGE_BOX" "HakoAllocPageModel must own page-local state"
guard_expect_in_file "$TAG" 'free: DirectArrayI64 = new DirectArrayI64\(\)' "$PAGE_BOX" "page model must initialize free as direct i64 storage"
guard_expect_in_file "$TAG" 'local_free: DirectArrayI64 = new DirectArrayI64\(\)' "$PAGE_BOX" "page model must initialize local_free as direct i64 storage"
guard_expect_in_file "$TAG" 'block_used: DirectArrayI64 = new DirectArrayI64\(\)' "$PAGE_BOX" "page model must initialize block_used as direct i64 storage"
guard_expect_in_file "$TAG" 'used: i64 = 0' "$PAGE_BOX" "page model used remains signed for sentinel-compatible accounting"
guard_expect_in_file "$TAG" 'free_top: i64 = 0' "$PAGE_BOX" "page model free_top remains signed for stack sentinel-compatible paths"
guard_expect_in_file "$TAG" 'local_free_top: i64 = 0' "$PAGE_BOX" "page model local_free_top remains signed for stack sentinel-compatible paths"
guard_expect_in_file "$TAG" 'capacity: usize' "$PAGE_BOX" "page model must expose capacity as exact usize storage"
guard_expect_in_file "$TAG" 'reserved: usize' "$PAGE_BOX" "page model must expose reserved as exact usize storage"
guard_expect_in_file "$TAG" 'block_size: usize' "$PAGE_BOX" "page model must expose block_size as exact usize storage"
guard_expect_in_file "$TAG" 'requested_bytes: usize = 0' "$PAGE_BOX" "page model requested bytes must be exact usize storage"
guard_expect_in_file "$TAG" 'alloc_count: i64 = 0' "$PAGE_BOX" "page alloc counter must remain signed"
guard_expect_in_file "$TAG" 'local_free_count: i64 = 0' "$PAGE_BOX" "page local-free counter must remain signed"
guard_expect_in_file "$TAG" 'reject_count: usize = 0' "$PAGE_BOX" "page reject counter must be exact usize"
guard_expect_in_file "$TAG" 'local_free_collect_count: usize = 0' "$PAGE_BOX" "local-free collection counter must be exact usize"
guard_expect_in_file "$TAG" 'local_free_collected_blocks: usize = 0' "$PAGE_BOX" "collected-block counter must be exact usize"
guard_expect_in_file "$TAG" 'retire_count: i64 = 0' "$PAGE_BOX" "retire counter must remain signed"
guard_expect_in_file "$TAG" 'decommit_count: usize = 0' "$PAGE_BOX" "decommit counter must be exact usize"
guard_expect_in_file "$TAG" 'recommit_count: usize = 0' "$PAGE_BOX" "recommit counter must be exact usize"
guard_expect_in_file "$TAG" 'reuse_count: usize = 0' "$PAGE_BOX" "reuse counter must be exact usize"
guard_expect_in_file "$TAG" 'lifecycle_reject_count: usize = 0' "$PAGE_BOX" "lifecycle reject counter must be exact usize"
guard_expect_in_file "$TAG" 'reactivate_count: usize = 0' "$PAGE_BOX" "reactivate counter must be exact usize"
guard_expect_in_file "$TAG" 'reactivate_reject_count: usize = 0' "$PAGE_BOX" "reactivate reject counter must be exact usize"
guard_expect_in_file "$TAG" 'peak_used: i64 = 0' "$PAGE_BOX" "page model peak_used must remain signed"
guard_expect_in_file "$TAG" 'seedFreeBlocks' "$PAGE_BOX" "page model must seed free blocks locally"
guard_expect_in_file "$TAG" 'releaseLocal' "$PAGE_BOX" "page model must have local release seam"
guard_expect_in_file "$TAG" 'memory.page_box = "memory/page_box.hako"' "$MODULE" "hako module must export page_box"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.page_box as HakoAllocPageBox' "$APP" "proof app must import page_box"
guard_expect_in_file "$TAG" 'local_free' "$APP_README" "proof README must describe local_free"
guard_expect_in_file "$TAG" 'M165 page model split' "$PLAN" "plan must retain M165 row"
guard_expect_in_file "$TAG" '293x-166 M165 Mimalloc Page Model Split' "$CARD" "missing M165 card"
guard_expect_in_file "$TAG" '294x-31 Hako Alloc Usize Page Model Lifecycle Counters' "$USIZE_CARD" "missing page model lifecycle counter usize card"
guard_expect_in_file "$TAG" '294x-43 Hako Alloc Usize Page Model Stack Occupancy' "$USIZE_STACK_CARD" "missing page model stack occupancy usize card"
guard_expect_in_file "$TAG" '294x-45 Hako Alloc Usize Page Model Capacity' "$USIZE_CAPACITY_CARD" "missing page model capacity usize card"
guard_expect_in_file "$TAG" '294x-47 Hako Alloc Usize Page Model Size Bytes' "$USIZE_SIZE_CARD" "missing page model size/bytes usize card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M165 guard"
guard_expect_in_file "$TAG" 'loop\(i < me\.capacity\)' "$PAGE_BOX" "page seeding must exercise JoinIR field-read loop bound"
guard_expect_in_file "$TAG" "$DIRECT_EXACT_ENV" "$SELF_SCRIPT" "LLVM EXE page model proof must source the direct-exact env preset"

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

if rg -n 'mimalloc-page-model|HakoAllocPageModel|page_box|local_free' lang/c-abi/shims \
  | rg -v 'HakoAllocPageModel\.acquire_usize/1' >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: unexpected page model matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  cargo run -q --bin hakorune -- --backend vm "$ROOT_DIR/$APP" >"$OUT" 2>"$ERR"

proof_output_assert_fixed_lines "$TAG" "$OUT" \
  'mimalloc-page-model-proof' \
  'blocks=2,1,0,-1,1' \
  'state=3,3,5,0,0' \
  'counts=4,1,3,3,56' \
  'shape=14' \
  'summary=ok'

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
    "HakoAllocPageModel.acquire_usize/1",
    "HakoAllocPageModel.acquireFreshSmall/1",
    "HakoAllocPageModel.requestedBytesAccumulatorLimit/0",
    "HakoAllocPageModel.canAccumulateRequestedBytes/1",
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
    "local_free_collect_count",
    "local_free_collected_blocks",
    "reject_count",
    "decommit_count",
    "recommit_count",
    "reuse_count",
    "lifecycle_reject_count",
    "reactivate_count",
    "reactivate_reject_count",
    "block_size",
    "requested_bytes",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"page model {name} must be exact usize storage: {field}")
for name in (
    "page_id",
    "used",
    "free_top",
    "local_free_top",
    "alloc_count",
    "local_free_count",
    "retired",
    "decommitted",
    "retire_count",
    "peak_used",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"page model {name} must remain i64 storage: {field}")
for name in (
    "capacity",
    "reserved",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"page model {name} must be exact usize storage: {field}")
for name in (
    "free",
    "local_free",
    "block_used",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "DirectArrayI64" or field.get("storage") != "handle":
        raise SystemExit(f"page model {name} must be DirectArrayI64 handle storage: {field}")
PY

rm -f /tmp/"$TAG".emit.out /tmp/"$TAG".emit.err

mkdir -p "$EXE_DIR"
# shellcheck source=tools/allocator/mimalloc_direct_exact_env.sh
source "$DIRECT_EXACT_ENV"
mimalloc_direct_exact_env_check
pure_first_guard_build_toolchain
pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$MIR" "$EXE_OUT" "$BUILD_LOG"
pure_first_guard_assert_clean_build_log "$TAG" "$BUILD_LOG"
pure_first_guard_run_exe "$TAG" "$EXE_OUT" "$RUN_LOG"

proof_output_assert_fixed_lines "$TAG" "$RUN_LOG" \
  'mimalloc-page-model-proof' \
  'blocks=2,1,0,-1,1' \
  'state=3,3,5,0,0' \
  'counts=4,1,3,3,56' \
  'shape=14' \
  'summary=ok'

cat "$OUT"

echo "[$TAG] ok"
