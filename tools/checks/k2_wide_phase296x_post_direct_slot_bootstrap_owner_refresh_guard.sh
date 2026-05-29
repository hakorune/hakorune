#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-342-POST-DIRECT-SLOT-BOOTSTRAP-OWNER-REFRESH.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-341-DIRECT-SLOT-BOOTSTRAP-MATERIALIZATION-COMPATIBILITY.md"
TOOL="$ROOT_DIR/tools/allocator/direct_slot_post_bootstrap_owner_refresh.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row342_owner_refresh.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

PERF_REPORT="$TMP_DIR/perf.report"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row342-direct-slot-owner-refresh] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq "$pattern" "$file"; then
    echo "[row342-direct-slot-owner-refresh] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-post-bootstrap-owner-refresh-v0"
require_line "$DOC" "input_contract=direct-slot-bootstrap-materialization-compatibility-v0"
require_line "$DOC" "field_helper_pct=65.02"
require_line "$DOC" "exact_slot_helper_pct=0.00"
require_line "$DOC" "array_total_pct=30.27"
require_line "$DOC" "selected_boundary=direct_slot_supported_storage_nativedirect_guard_surface"
require_line "$DOC" "next_diagnostic=direct_slot_supported_storage_nativedirect_guard_surface"
require_line "$DOC" "selected_reason=legacy_field_helpers_dominate_after_direct_slot_bootstrap_compatibility"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

require_pattern "$TOOL" "output_contract=direct-slot-post-bootstrap-owner-refresh-v0"
require_pattern "$TOOL" "nyash.object.field_"
require_pattern "$TOOL" "direct_slot_supported_storage_nativedirect_guard_surface"

cat >"$PERF_REPORT" <<'REPORT'
    29.21%  app.exe  app.exe               [.] nyash.object.field_set_hii
            |
            ---nyash.object.field_set_hii
               |--5.36%--HakoAllocObjectLifecycleAllocResult.recordSuccess/1
               |--4.43%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
               |--4.31%--HakoAllocObjectLifecycleReleaseResult.reset/0
               |--3.84%--HakoAllocObjectLifecyclePageQueue.beginSelection/0
               |--2.97%--HakoAllocObjectLifecycleReleaseResult.recordSuccess/2
               |--2.94%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
               |--2.27%--HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
               |--1.55%--HakoAllocObjectLifecycleAllocResult.reset/0
                --1.54%--HakoAllocObjectLifecycleReleaseResult.recordRequest/2
    15.20%  app.exe  app.exe               [.] nyash.object.field_get_hii
            |
            ---nyash.object.field_get_hii
               |--6.11%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
               |--3.08%--HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0
               |--1.55%--HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
               |--1.55%--HakoAllocObjectLifecycleFacade.resetReleaseResult/0
               |--1.55%--HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2
               |--0.78%--HakoAllocPageModel.isRetired/0
                --0.57%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
    12.61%  app.exe  app.exe               [.] nyash_kernel::plugin::array_slot_backend::single_thread_store_i64
    11.76%  app.exe  app.exe               [.] nyash.object.field_get_u64_hii
            |
            ---nyash.object.field_get_u64_hii
               |--3.10%--HakoAllocObjectLifecycleAllocResult.recordSuccess/1
               |--2.28%--HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0
               |--2.24%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
               |--1.55%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
               |--1.04%--HakoAllocPageModel.releaseLocalKnownLive/1
               |--0.78%--HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
                --0.77%--HakoAllocObjectLifecycleReleaseResult.recordSuccess/2
     8.85%  app.exe  app.exe               [.] nyash.object.field_set_u64_hiu
            |
            ---nyash.object.field_set_u64_hiu
               |--2.93%--HakoAllocObjectLifecycleAllocResult.recordSuccess/1
               |--2.33%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
               |--1.55%--HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0
                --1.55%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
     8.49%  app.exe  app.exe               [.] core::hash::BuildHasher::hash_one
     5.42%  app.exe  app.exe               [.] <core::hash::sip::Hasher<S> as core::hash::Hasher>::write
     3.75%  app.exe  app.exe               [.] nyash_kernel::plugin::array_slot_backend::single_thread_load_encoded_i64
     1.54%  app.exe  app.exe               [.] HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0
     0.78%  app.exe  app.exe               [.] HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2
     0.78%  app.exe  app.exe               [.] HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
     0.77%  app.exe  app.exe               [.] HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
     0.77%  app.exe  app.exe               [.] HakoAllocObjectLifecycleFacade.resetSmallAllocResult/0
REPORT

"$TOOL" --perf-report "$PERF_REPORT" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=direct-slot-post-bootstrap-owner-refresh-v0"
require_line "$REPORT" "input_contract=direct-slot-bootstrap-materialization-compatibility-v0"
require_line "$REPORT" "workload_id=representative-object-lifecycle-small-block-v0"
require_line "$REPORT" "field_helper_pct=65.02"
require_line "$REPORT" "exact_slot_helper_pct=0.00"
require_line "$REPORT" "array_total_pct=30.27"
require_line "$REPORT" "selected_boundary=direct_slot_supported_storage_nativedirect_guard_surface"
require_line "$REPORT" "next_diagnostic=direct_slot_supported_storage_nativedirect_guard_surface"
require_line "$REPORT" "selected_reason=legacy_field_helpers_dominate_after_direct_slot_bootstrap_compatibility"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"
