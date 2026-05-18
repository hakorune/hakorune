#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
case "$VALIDATION_LEVEL" in
  L0|L1|L2|L3) ;;
  L4)
    echo "[$TAG] ERROR: MIMAP-228A is first-pattern L3; L4 belongs to future batch packs" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-proof/main.hako"
APP_README="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-proof/README.md"
APP_TEST="apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-proof/test.sh"
CARD_226A="docs/development/current/main/phases/phase-293x/293x-749-MIMAP-226A-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-KEYED-RELEASE-SHADOW-CLOSEOUT-PACK.md"
CARD_227A="docs/development/current/main/phases/phase-293x/293x-750-MIMAP-227A-POST-SEGMENT-MAP-LOCAL-FREE-REUSE-LEDGER-LIFECYCLE-KEYED-RELEASE-SHADOW-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-751-MIMAP-228A-SOURCE-RELEASE-LEDGER-LIFECYCLE-KEY-MIGRATION-PILOT.md"
CARD_229A="docs/development/current/main/phases/phase-293x/293x-752-MIMAP-229A-SOURCE-LIFECYCLE-KEYED-RELEASE-LEDGER-DIAGNOSTICS.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-ssot.md"
SHADOW_DESIGN="docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
CADENCE="docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OLD_RELEASE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_release_box.hako"
NEW_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_ledger_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_ledger_guard.sh"

printf '[%s] checking MIMAP-228A lifecycle-keyed source release ledger\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_226A" \
  "$CARD_227A" \
  "$CARD" \
  "$CARD_229A" \
  "$DESIGN" \
  "$SHADOW_DESIGN" \
  "$PLAN" \
  "$JOINT" \
  "$CADENCE" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$OLD_RELEASE_OWNER" \
  "$NEW_OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_226A" "MIMAP-226A closeout must be landed before migration"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_227A" "MIMAP-227A selection must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-228A card must be landed"
guard_expect_in_file "$TAG" 'Status: (landed|selected current)' "$CARD_229A" "MIMAP-229A must be selected current or landed after MIMAP-228A"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-228A design must be accepted"
guard_expect_in_file "$TAG" 'new lifecycle-keyed source release ledger owner instead of' "$DESIGN" "design must introduce a separate migrated owner"
guard_expect_in_file "$TAG" 'mutating the old modeled-reuse-token keyed release owner in place' "$DESIGN" "design must keep source migration reversible"
guard_expect_in_file "$TAG" 'Decision: accepted' "$SHADOW_DESIGN" "MIMAP-224A shadow design must stay accepted"
guard_expect_in_file "$TAG" 'MIMAP-228A' "$PLAN" "granularity SSOT must describe MIMAP-228A"
guard_expect_in_file "$TAG" 'MIMAP-229A' "$PLAN" "granularity SSOT must describe MIMAP-229A"
guard_expect_in_file "$TAG" 'MIMAP-228A source release-ledger lifecycle-key migration pilot' "$JOINT" "joint order must name MIMAP-228A"
guard_expect_in_file "$TAG" 'MIMAP-229A source lifecycle-keyed release ledger diagnostics' "$JOINT" "joint order must name MIMAP-229A"
guard_expect_in_file "$TAG" 'source release-ledger lifecycle-key migration family' "$CADENCE" "cadence SSOT must define migration family"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-228A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-228A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-228A"
guard_expect_in_file "$TAG" 'validation_profile = "first-pattern"' "$PROOF_MANIFEST" "MIMAP-228A must be first-pattern validation"
guard_expect_in_file "$TAG" 'exe = "auto"' "$PROOF_MANIFEST" "MIMAP-228A EXE evidence must be enabled"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_ledger_box' "$MODULE" "module must export lifecycle-keyed source release ledger owner"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_ledger_box.hako' "$MEMORY_README" "memory README must name lifecycle-keyed source release ledger owner"
guard_expect_in_file "$TAG" 'recordLifecycleKeyedRelease' "$NEW_OWNER" "new owner must expose lifecycle-keyed release route"
guard_expect_in_file "$TAG" 'source_release_ledger_key_kind: i64 = 1' "$NEW_OWNER" "new report must declare lifecycle-keyed source key kind"
guard_expect_in_file "$TAG" 'release_key_migrated: i64 = 1' "$NEW_OWNER" "new report must declare migrated key flag"
guard_expect_in_file "$TAG" 'modeled_reuse_token_backref_present: i64 = 1' "$NEW_OWNER" "new report must preserve modeled reuse token backref"
guard_expect_in_file "$TAG" 'check "mimap228a segment map local free reuse ledger lifecycle-keyed release ledger"' "$APP" "MIMAP-228A proof must use labelled check block"

if rg -n 'reuse_lifecycle_token|recordLifecycleKeyedRelease|LifecycleKeyedReleaseLedger|source_release_ledger_key_kind|release_key_migrated' \
  "$OLD_RELEASE_OWNER" >/tmp/"$TAG".old_owner_migration_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-228A must not mutate the old modeled-reuse-token keyed release owner in place" >&2
  cat /tmp/"$TAG".old_owner_migration_leak >&2
  rm -f /tmp/"$TAG".old_owner_migration_leak
  exit 1
fi
rm -f /tmp/"$TAG".old_owner_migration_leak

if rg -n 'realLifecycle|generationToken|migrateReleaseLedger|releaseLedgerKeyMigration' \
  "$APP" "$NEW_OWNER" >/tmp/"$TAG".real_lifecycle_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-228A must not define real lifecycle/generation or broad migration machinery" >&2
  cat /tmp/"$TAG".real_lifecycle_leak >&2
  rm -f /tmp/"$TAG".real_lifecycle_leak
  exit 1
fi
rm -f /tmp/"$TAG".real_lifecycle_leak

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|lookupSegment[[:space:]]*\(|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList[[:space:]]*\.|mutatePageState|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$APP" "$NEW_OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-228A must keep real execution/raw pointer/concurrency/segment-map/atomics/page-source/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-proof|LifecycleKeyedReleaseLedger|lifecycleKeyedReleaseLedger' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-228A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap228_lifecycle_release_ledger.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap228.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-proof' "$vm_log"
rg -F -q 'base=70007004,70007004002,3' "$vm_log"
rg -F -q 'migrated=1,0,0,1,1,70007004,2,70007004002' "$vm_log"
rg -F -q 'rejects=0,1,0,2,0,3,0,4,0,5' "$vm_log"
rg -F -q 'counts=6,1,5,1,1,1,1,1' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'check=1' "$vm_log"
rg -F -q 'summary=ok' "$vm_log"

if ! pure_first_guard_level_allows_mir "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"

python3 - "$mir_json" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as fh:
    data = json.load(fh)

functions = {fn.get("name"): fn for fn in data.get("functions", [])}
required = {
    "main",
    "Main.nextReuseReport/4",
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLifecycleKeyedReleaseLedger.recordLifecycleKeyedRelease/3",
    "HakoAllocSegmentAllocationModeledLocalFreeReuseLifecycleTokenReleaseKeyPrecondition.classify/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentAllocationModeledLocalFreeReuseLifecycleKeyedReleaseLedgerReport")
if report is None:
    raise SystemExit("missing lifecycle-keyed release ledger report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "did_release",
    "reason",
    "reuse_lifecycle_token",
    "modeled_reuse_token",
    "lifecycle_id",
    "source_release_ledger_key_kind",
    "release_key_migrated",
    "modeled_reuse_token_backref_present",
    "lifecycle_keyed_release_ledger_present",
    "would_use_raw_pointer",
    "would_use_segment_map",
    "would_activate_provider",
):
    if name not in fields:
        raise SystemExit(f"missing lifecycle-keyed release ledger report field: {name}")

print("[mimap228a-mir-json] ok")
PY

if ! pure_first_guard_level_allows_exe "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

pure_first_guard_build_toolchain

exe_out="$tmp_dir/mimap228.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-ledger-proof' "$run_log"
rg -F -q 'base=70007004,70007004002,3' "$run_log"
rg -F -q 'migrated=1,0,0,1,1,70007004,2,70007004002' "$run_log"
rg -F -q 'rejects=0,1,0,2,0,3,0,4,0,5' "$run_log"
rg -F -q 'counts=6,1,5,1,1,1,1,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok level=$VALIDATION_LEVEL"
