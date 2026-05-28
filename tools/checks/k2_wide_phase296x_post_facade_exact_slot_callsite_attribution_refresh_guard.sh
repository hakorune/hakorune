#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-233-POST-FACADE-EXACT-SLOT-CALLSITE-ATTRIBUTION-REFRESH.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-232-POST-SELECTED-FACADE-GET-SET-OWNER-REFRESH.md"
TOOL="$ROOT_DIR/tools/allocator/typed_object_exact_slot_callsite_attribution.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row233_callsite_attr.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

PERF_REPORT="$TMP_DIR/perf.report"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row233-callsite-attr] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=typed-object-exact-slot-callsite-attribution-v0"
require_line "$DOC" "input_contract=post-selected-facade-get-set-owner-refresh-v0"
require_line "$DOC" "attribution_source=perf_callgraph"
require_line "$DOC" "callgraph_attribution_available=1"
require_line "$DOC" "exact_slot_get_set_pct=56.37"
require_line "$DOC" "attributed_callsite_count=29"
require_line "$DOC" "top_callsite_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "$DOC" "dominant_family=object_lifecycle_facade"
require_line "$DOC" "dominant_family_pct=17.36"
require_line "$DOC" "selected_boundary=exact_slot_callsite_owner_selection"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cat >"$PERF_REPORT" <<'REPORT'
    19.44%  app.exe  app.exe               [.] nyash.object.exact_slot_set_i64_hii
            |
            ---nyash.object.exact_slot_set_i64_hii
               |--4.14%--HakoAllocObjectLifecycleReleaseResult.reset/0
               |--3.54%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
               |--3.50%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
               |--3.48%--HakoAllocObjectLifecycleAllocResult.reset/0
               |--2.06%--HakoAllocObjectLifecyclePageQueue.beginSelection/0
               |--1.37%--HakoAllocObjectLifecycleAllocResult.recordSuccess/1
               |--0.69%--HakoAllocObjectLifecycleReleaseResult.recordSuccess/2
                --0.67%--HakoAllocObjectLifecycleReleaseResult.recordRequest/2
    10.38%  app.exe  app.exe               [.] nyash.object.exact_slot_get_i64_hii
            |
            ---nyash.object.exact_slot_get_i64_hii
               |--4.15%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
               |--2.77%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
               |--2.07%--HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
               |--0.70%--HakoAllocPageModel.acquire_usize/1
                --0.69%--HakoAllocPageModel.isRetired/0
     9.92%  app.exe  app.exe               [.] nyash.object.exact_slot_get_handle_hii
            |
            ---nyash.object.exact_slot_get_handle_hii
               |--2.79%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
               |--2.25%--HakoAllocPageModel.acquire_usize/1
               |--1.40%--HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2
               |--1.39%--HakoAllocObjectLifecycleFacade.recordReleaseSuccess/2
               |--0.69%--HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
               |--0.69%--HakoAllocPageModel.releaseLocalKnownLive/1
                --0.69%--HakoAllocObjectLifecycleFacade.resetSmallAllocResult/0
     8.96%  app.exe  app.exe               [.] nyash.object.exact_slot_get_u64_hii
            |
            ---nyash.object.exact_slot_get_u64_hii
               |--3.46%--HakoAllocPageModel.acquire_usize/1
               |--1.41%--HakoAllocObjectLifecycleAllocResult.recordSuccess/1
               |--1.39%--HakoAllocPageModel.releaseLocalKnownLive/1
               |--1.34%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
               |--0.69%--HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0
                --0.68%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
     4.89%  app.exe  app.exe               [.] nyash.object.exact_slot_set_u64_hiu
            |
            ---nyash.object.exact_slot_set_u64_hiu
               |--2.79%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
               |--1.39%--HakoAllocPageModel.acquire_usize/1
                --0.71%--HakoAllocObjectLifecycleAllocResult.recordSuccess/1
     2.78%  app.exe  app.exe               [.] nyash.object.exact_slot_set_handle_hii
            |
            ---nyash.object.exact_slot_set_handle_hii
               HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
REPORT

python3 "$TOOL" \
  --perf-report "$PERF_REPORT" \
  --input-contract post-selected-facade-get-set-owner-refresh-v0 \
  --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=typed-object-exact-slot-callsite-attribution-v0"
require_line "$REPORT" "input_contract=post-selected-facade-get-set-owner-refresh-v0"
require_line "$REPORT" "workload_id=representative-object-lifecycle-small-block-v0"
require_line "$REPORT" "attribution_source=perf_callgraph"
require_line "$REPORT" "callgraph_attribution_available=1"
require_line "$REPORT" "exact_slot_get_set_pct=56.37"
require_line "$REPORT" "attributed_callsite_count=29"
require_line "$REPORT" "top_callsite_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "$REPORT" "dominant_family=object_lifecycle_facade"
require_line "$REPORT" "selected_boundary=exact_slot_callsite_owner_selection"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"
