#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-cleanup-reason-ssot"
cd "$ROOT_DIR"

FACADE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
REASON="lang/src/hako_alloc/memory/object_lifecycle_facade_reason_box.hako"
RESULT="lang/src/hako_alloc/memory/object_lifecycle_facade_result_box.hako"
MODULES="lang/src/hako_alloc/hako_module.toml"
README="lang/src/hako_alloc/memory/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-366-MIMAP-FACADE-CLEAN-001-RESULT-SSOT-TODO.md"
INDEX="docs/tools/check-scripts-index.md"

for path in "$FACADE" "$REASON" "$RESULT" "$MODULES" "$README" "$CARD" "$INDEX"; do
  [[ -f "$path" ]] || { echo "[$TAG] ERROR: missing required file: $path" >&2; exit 1; }
done

rg -F -q 'using selfhost.hako_alloc.memory.object_lifecycle_facade_reason_box as HakoAllocObjectLifecycleFacadeReason' "$FACADE"
rg -F -q 'using selfhost.hako_alloc.memory.object_lifecycle_facade_result_box as HakoAllocObjectLifecycleFacadeResultBox' "$FACADE"
rg -F -q 'memory.object_lifecycle_facade_reason_box = "memory/object_lifecycle_facade_reason_box.hako"' "$MODULES"
rg -F -q 'memory.object_lifecycle_facade_result_box = "memory/object_lifecycle_facade_result_box.hako"' "$MODULES"
rg -F -q 'objectLifecycleKnownPageIndexById(page_id)' "$FACADE"
rg -F -q 'HakoAllocObjectLifecycleFacadeReason' "$REASON"
rg -F -q 'HakoAllocObjectLifecycleAllocResult' "$RESULT"
rg -F -q 'HakoAllocObjectLifecycleReleaseResult' "$RESULT"
rg -F -q 'HakoAllocObjectLifecycleAlignmentResult' "$RESULT"
rg -F -q 'HakoAllocObjectLifecycleReallocResult' "$RESULT"
rg -F -q 'object_lifecycle_facade_reason_box.hako' "$README"
rg -F -q 'object_lifecycle_facade_result_box.hako' "$README"
rg -F -q 'Reason-code SSOT' "$README"
rg -F -q 'k2_wide_mimalloc_facade_cleanup_reason_ssot_guard.sh' "$INDEX"

for method in \
  small_no_page \
  small_bad_selected_kind \
  small_reuse_failed \
  small_acquire_failed \
  small_alignment_unsupported \
  release_no_page \
  release_bad_block \
  release_page_reject \
  alignment_unsupported \
  realloc_no_page \
  realloc_bad_block \
  realloc_bad_size \
  realloc_direction_unsupported \
  realloc_stale_block \
  realloc_alloc_failed \
  realloc_release_failed
do
  rg -F -q "${method}()" "$REASON"
  rg -F -q "HakoAllocObjectLifecycleFacadeReason.${method}()" "$FACADE"
done

for scan in \
  'local page0 = pages.get(0)' \
  'local page1 = pages.get(1)' \
  'local page2 = pages.get(2)'
do
  count="$(rg -F -c "$scan" "$FACADE" || true)"
  if [[ "$count" != "1" ]]; then
    echo "[$TAG] ERROR: known-page scan anchor '$scan' must appear exactly once in the facade helper" >&2
    exit 1
  fi
done

if rg -n 'record(SmallAlloc|Release|Alignment|Realloc)Failure\([0-9]' "$FACADE" >/tmp/"$TAG".literal 2>&1; then
  echo "[$TAG] ERROR: facade failure reason literals must route through object_lifecycle_facade_reason_box.hako" >&2
  cat /tmp/"$TAG".literal >&2
  rm -f /tmp/"$TAG".literal
  exit 1
fi
rm -f /tmp/"$TAG".literal

if rg -n 'last_alloc_|small_alloc_|last_release_|last_alignment_|last_realloc_' "$FACADE" >/tmp/"$TAG".facade_state 2>&1; then
  echo "[$TAG] ERROR: facade result observer state must live in object_lifecycle_facade_result_box.hako" >&2
  cat /tmp/"$TAG".facade_state >&2
  rm -f /tmp/"$TAG".facade_state
  exit 1
fi
rm -f /tmp/"$TAG".facade_state

if rg -n 'PageMap|page_map|lookup\(|copy[A-Za-z0-9_]*\(|byte[A-Za-z0-9_]*\(|memcpy|OSVM|OsVm|externcall|atomic[A-Za-z0-9_]*\(|RawBuf|provider[A-Za-z0-9_]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*\(|pageSource|remote[A-Za-z0-9_]*\(' \
  "$FACADE" "$REASON" "$RESULT" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: facade cleanup must not add page-map/substrate/provider/backend behavior" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

echo "[$TAG] ok"
