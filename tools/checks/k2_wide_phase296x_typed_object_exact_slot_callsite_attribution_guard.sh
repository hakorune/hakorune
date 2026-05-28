#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-225-TYPED-OBJECT-EXACT-SLOT-CALLSITE-ATTRIBUTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-224-POST-RMW-FUSION-OWNER-REFRESH.md"
TOOL="$ROOT_DIR/tools/allocator/typed_object_exact_slot_callsite_attribution.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row225_callsite_attr.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

PERF_REPORT="$TMP_DIR/perf.report"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row225-callsite-attr] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=typed-object-exact-slot-callsite-attribution-v0"
require_line "$DOC" "input_contract=typed-object-post-rmw-fusion-owner-refresh-v0"
require_line "$DOC" "exact_slot_get_set_pct=59.97"
require_line "$DOC" "attributed_callsite_count=30"
require_line "$DOC" "top_callsite_symbol=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "dominant_family=object_lifecycle_facade"
require_line "$DOC" "selected_boundary=exact_slot_callsite_owner_selection"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cat >"$PERF_REPORT" <<'REPORT'
    16.00%  app.exe  app.exe               [.] nyash.object.exact_slot_set_i64_hii
               |--3.09%--HakoAllocObjectLifecycleReleaseResult.recordSuccess/2
               |--3.08%--HakoAllocObjectLifecycleAllocResult.reset/0
               |--3.05%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
               |--2.34%--HakoAllocObjectLifecycleReleaseResult.recordRequest/2
               |--2.30%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
               |--1.40%--HakoAllocObjectLifecycleAllocResult.recordSuccess/1
                --0.74%--HakoAllocObjectLifecycleReleaseResult.reset/0
    11.42%  app.exe  app.exe               [.] nyash.object.exact_slot_get_u64_hii
               |--4.54%--HakoAllocPageModel.acquire_usize/1
               |--2.33%--HakoAllocObjectLifecycleAllocResult.recordSuccess/1
               |--1.53%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
               |--1.49%--HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0
               |--0.78%--HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
                --0.75%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
     9.53%  app.exe  app.exe               [.] nyash.object.exact_slot_get_handle_hii
               |--2.34%--HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0
               |--1.70%--HakoAllocPageModel.releaseLocalKnownLive/1
               |--1.67%--HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
               |--1.53%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
               |--1.53%--HakoAllocObjectLifecycleFacade.resetSmallAllocResult/0
                --0.75%--HakoAllocPageModel.acquire_usize/1
     9.20%  app.exe  app.exe               [.] nyash.object.exact_slot_set_u64_hiu
               |--3.84%--HakoAllocPageModel.acquire_usize/1
               |--2.30%--HakoAllocObjectLifecycleAllocResult.recordSuccess/1
               |--1.55%--HakoAllocPageModel.releaseLocalKnownLive/1
               |--0.77%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
                --0.75%--HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0
     6.94%  app.exe  app.exe               [.] nyash.object.exact_slot_get_i64_hii
               |--2.34%--HakoAllocPageModel.acquire_usize/1
               |--2.33%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
                --2.28%--HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
     6.88%  app.exe  app.exe               [.] nyash.object.exact_slot_set_handle_hii
               |--3.81%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
               |--1.54%--HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
                --1.53%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
REPORT

python3 "$TOOL" --perf-report "$PERF_REPORT" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=typed-object-exact-slot-callsite-attribution-v0"
require_line "$REPORT" "input_contract=typed-object-post-rmw-fusion-owner-refresh-v0"
require_line "$REPORT" "workload_id=representative-object-lifecycle-small-block-v0"
require_line "$REPORT" "exact_slot_get_set_pct=59.97"
require_line "$REPORT" "attributed_callsite_count=30"
require_line "$REPORT" "top_callsite_symbol=HakoAllocPageModel.acquire_usize/1"
require_line "$REPORT" "top_callsite_helper=nyash.object.exact_slot_get_u64_hii"
require_line "$REPORT" "dominant_family=object_lifecycle_facade"
require_line "$REPORT" "selected_boundary=exact_slot_callsite_owner_selection"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"
