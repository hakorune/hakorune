#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-backing-modeled-residence-arena-binding"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

if [ "$#" -eq 0 ]; then
  VALIDATION_LEVEL="L2"
else
  VALIDATION_LEVEL="$(pure_first_guard_parse_level "$TAG" "$@")"
fi
case "$VALIDATION_LEVEL" in
  L0|L1|L2) ;;
  L3|L4)
    echo "[$TAG] ERROR: MIMAP-252A defers L3/L4 evidence to a future closeout pack" >&2
    exit 2
    ;;
esac

APP="apps/hako-alloc-segment-arena-backing-modeled-residence-arena-binding-proof/main.hako"
APP_README="apps/hako-alloc-segment-arena-backing-modeled-residence-arena-binding-proof/README.md"
APP_TEST="apps/hako-alloc-segment-arena-backing-modeled-residence-arena-binding-proof/test.sh"
CARD_250A="docs/development/current/main/phases/phase-293x/293x-773-MIMAP-250A-SEGMENT-ARENA-BACKING-MODELED-NO-ESCAPE-ADDRESS-RESIDENCE-CLOSEOUT-PACK.md"
CARD_251A="docs/development/current/main/phases/phase-293x/293x-774-MIMAP-251A-POST-SEGMENT-ARENA-BACKING-MODELED-NO-ESCAPE-ADDRESS-RESIDENCE-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-775-MIMAP-252A-SEGMENT-ARENA-BACKING-MODELED-RESIDENCE-ARENA-BINDING-INVENTORY.md"
CARD_253A="docs/development/current/main/phases/phase-293x/293x-776-MIMAP-253A-SEGMENT-ARENA-BACKING-MODELED-RESIDENCE-ARENA-BINDING-DIAGNOSTICS.md"
RESIDENCE_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-no-escape-address-residence-ssot.md"
MATRIX_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-backing-requirement-matrix-ssot.md"
DESIGN="docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-residence-arena-binding-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MATRIX_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_requirement_matrix_box.hako"
RESIDENCE_OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_no_escape_address_residence_box.hako"
OWNER="lang/src/hako_alloc/memory/segment_arena_backing_modeled_residence_arena_binding_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_residence_arena_binding_guard.sh"

printf '[%s] checking MIMAP-252A segment arena backing modeled residence arena-binding\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD_250A" \
  "$CARD_251A" \
  "$CARD" \
  "$CARD_253A" \
  "$RESIDENCE_SSOT" \
  "$MATRIX_SSOT" \
  "$DESIGN" \
  "$PLAN" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$MATRIX_OWNER" \
  "$RESIDENCE_OWNER" \
  "$OWNER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_250A" "MIMAP-250A closeout must be landed before arena-binding"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_251A" "MIMAP-251A selection must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-252A card must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_253A" "MIMAP-253A diagnostics must be landed after MIMAP-252A"
guard_expect_in_file "$TAG" 'Decision: accepted' "$RESIDENCE_SSOT" "MIMAP-248A residence design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$MATRIX_SSOT" "MIMAP-240A matrix design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-252A arena-binding design must be accepted"
guard_expect_in_file "$TAG" 'modeled residence arena-binding accepted' "$DESIGN" "MIMAP-252A reason vocabulary must be present"
guard_expect_in_file "$TAG" 'MIMAP-252A segment arena backing modeled residence arena-binding inventory' "$PLAN" "granularity SSOT must describe MIMAP-252A"
guard_expect_in_file "$TAG" 'MIMAP-252A segment arena backing modeled residence arena-binding inventory' "$JOINT" "joint order must name MIMAP-252A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-252A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-252A"' "$PROOF_MANIFEST" "proof manifest must list MIMAP-252A"
guard_expect_in_file "$TAG" 'validation_profile = "scalar-mir"' "$PROOF_MANIFEST" "MIMAP-252A must use scalar-mir validation"
guard_expect_in_file "$TAG" 'exe = "deferred-to-closeout"' "$PROOF_MANIFEST" "MIMAP-252A EXE evidence must be deferred"
guard_expect_in_file "$TAG" 'memory.segment_arena_backing_modeled_residence_arena_binding_box' "$MODULE" "module must export arena-binding owner"
guard_expect_in_file "$TAG" 'segment_arena_backing_modeled_residence_arena_binding_box.hako' "$MEMORY_README" "memory README must name arena-binding owner"
guard_expect_in_file "$TAG" 'recordBinding' "$OWNER" "arena-binding owner must expose record route"
guard_expect_in_file "$TAG" 'check "mimap252a segment arena backing modeled residence arena binding"' "$APP" "proof must use labelled check block"

if rg -n 'lookupByPointer|lookupPointer|pointer_member|dereference[[:space:]]*\(|mutateSegmentMap|claimBitmap|unclaimBitmap|AtomicCoreBox|hako_atomic|cas_i64|fetch_add|hako_osvm|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|providerActivate|global_allocator' \
  "$APP" "$OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-252A must keep pointer lookup/arena/segment-map/atomic/OSVM/thread/provider seams inactive" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-segment-arena-backing-modeled-residence-arena-binding-proof|ModeledResidenceArenaBinding|modeledResidenceArenaBinding' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-252A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if ! pure_first_guard_level_allows_vm "$VALIDATION_LEVEL"; then
  printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
  exit 0
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimap252_residence_binding.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap252.mir.json"
vm_log="$tmp_dir/vm.log"

if ! pure_first_guard_run_vm "$TAG" "$ROOT_DIR" "$APP" "$vm_log"; then
  exit 1
fi

rg -F -q 'hako-alloc-segment-arena-backing-modeled-residence-arena-binding-proof' "$vm_log"
rg -F -q 'binding=1,0,1,70,7,3,70007004002,70007004002,16,8,4,16,4096,1' "$vm_log"
rg -F -q 'rejects=1,2,3,4,5,6,7,8,9' "$vm_log"
rg -F -q 'counts=10,1,9,1,1,1,1,1,1,1,1,1,9,0' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0,0' "$vm_log"
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
    "Main.makeMatrix/2",
    "Main.makeResidence/2",
    "HakoAllocSegmentArenaBackingModeledResidenceArenaBindingInventory.recordBinding/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
report = plans.get("HakoAllocSegmentArenaBackingModeledResidenceArenaBindingReport")
if report is None:
    raise SystemExit("missing modeled residence arena-binding report typed object plan")

fields = {field.get("name"): field for field in report.get("fields", [])}
for name in (
    "accepted",
    "reason",
    "binding_present",
    "modeled_residence_arena_binding_present",
    "row_index",
    "segment_id",
    "arena_id",
    "residence_token",
    "binding_token",
    "geometry_valid",
    "slice_count",
    "closed_substrate_blocker_count",
    "would_add_backend_matcher",
):
    if name not in fields:
        raise SystemExit(f"missing modeled residence arena-binding field: {name}")

print("[mimap252a-mir-json] ok")
PY

printf '[%s] ok level=%s\n' "$TAG" "$VALIDATION_LEVEL"
