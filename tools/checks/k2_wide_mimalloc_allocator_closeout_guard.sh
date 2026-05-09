#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-allocator-closeout"
cd "$ROOT_DIR"

TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-090-M38-MIMALLOC-ALLOCATOR-APP-CLOSEOUT-GUARD.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"

echo "[$TAG] checking M38 mimalloc allocator app closeout coverage"

fail() {
  echo "[$TAG] ERROR: $*" >&2
  exit 1
}

require_file() {
  local path="$1"
  [[ -f "$path" ]] || fail "missing file: $path"
}

require_dir() {
  local path="$1"
  [[ -d "$path" ]] || fail "missing directory: $path"
}

require_text() {
  local file="$1"
  local needle="$2"
  rg -F -q "$needle" "$file" || fail "missing text in $file: $needle"
}

require_file "$TASKBOARD"
require_file "$CARD"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"

required_apps=(
  "apps/mimalloc-raw-page-proof"
  "apps/mimalloc-size-class-table-proof"
  "apps/mimalloc-two-class-page-proof"
  "apps/mimalloc-dynamic-bin-proof"
  "apps/mimalloc-size-to-bin-inline-proof"
  "apps/mimalloc-osvm-page-proof"
  "apps/mimalloc-tls-cache-slot-proof"
  "apps/mimalloc-atomic-cas-proof"
  "apps/mimalloc-atomic-load-proof"
  "apps/mimalloc-atomic-store-proof"
  "apps/mimalloc-atomic-fetch-add-proof"
  "apps/mimalloc-remote-free-i64-proof"
  "apps/mimalloc-ptr-atomic-store-proof"
  "apps/mimalloc-tls-ptr-remote-free-proof"
  "apps/mimalloc-remote-free-policy-proof"
)

required_guards=(
  "tools/checks/k2_wide_mimalloc_raw_page_exe_guard.sh"
  "tools/checks/k2_wide_mimalloc_size_class_table_exe_guard.sh"
  "tools/checks/k2_wide_mimalloc_two_class_page_exe_guard.sh"
  "tools/checks/k2_wide_mimalloc_dynamic_bin_exe_guard.sh"
  "tools/checks/k2_wide_mimalloc_size_to_bin_inline_exe_guard.sh"
  "tools/checks/k2_wide_mimalloc_osvm_page_exe_guard.sh"
  "tools/checks/k2_wide_mimalloc_tls_cache_slot_exe_guard.sh"
  "tools/checks/k2_wide_mimalloc_atomic_cas_exe_guard.sh"
  "tools/checks/k2_wide_mimalloc_atomic_load_exe_guard.sh"
  "tools/checks/k2_wide_mimalloc_atomic_store_exe_guard.sh"
  "tools/checks/k2_wide_mimalloc_atomic_fetch_add_exe_guard.sh"
  "tools/checks/k2_wide_mimalloc_remote_free_i64_exe_guard.sh"
  "tools/checks/k2_wide_mimalloc_ptr_atomic_store_exe_guard.sh"
  "tools/checks/k2_wide_mimalloc_tls_ptr_remote_free_exe_guard.sh"
  "tools/checks/k2_wide_mimalloc_remote_free_policy_exe_guard.sh"
)

for app in "${required_apps[@]}"; do
  require_dir "$app"
  require_file "$app/main.hako"
  require_file "$app/README.md"
  require_file "$app/test.sh"
done

for guard in "${required_guards[@]}"; do
  require_file "$guard"
  [[ -x "$guard" ]] || fail "guard is not executable: $guard"
  require_text "$INDEX" "$guard"
  require_text "$DEV_GATE" "$guard"
done

require_text "$TASKBOARD" '| `M38 mimalloc allocator app closeout guard` | `live-narrow` |'
require_text "$CARD" "M38 Mimalloc Allocator App Closeout Guard"
require_text "$PHASE_README" '`293x-090`'
require_text "$REAL_APP_TASKBOARD" '`293x-090` M38 mimalloc allocator app closeout guard'
require_text "$INDEX" "tools/checks/k2_wide_mimalloc_allocator_closeout_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_mimalloc_allocator_closeout_guard.sh"

if rg -n 'mimalloc-(raw-page|size-class-table|two-class-page|dynamic-bin|size-to-bin-inline|osvm-page|tls-cache-slot|atomic-cas|atomic-load|atomic-store|atomic-fetch-add|remote-free-i64|ptr-atomic-store|tls-ptr-remote-free|remote-free-policy)-proof|AllocatorRemoteFreePolicy' \
  lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  fail "mimalloc app-specific matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".app_specific.inc

if rg -n 'hako_atomic_ptr_(load|cas)_ordered|HakoAtomicPtr(Load|Cas)Ordered|extern\\.hako_atomic\\.ptr_(load|cas)_ordered|ptr_fetch_add' \
  src lang/c-abi/shims crates/nyash_kernel >/tmp/"$TAG".inactive_pointer_rows 2>&1; then
  cat /tmp/"$TAG".inactive_pointer_rows >&2
  rm -f /tmp/"$TAG".inactive_pointer_rows
  fail "pointer atomic load/CAS/fetch_add rows must stay inactive at M38 closeout"
fi
rm -f /tmp/"$TAG".inactive_pointer_rows

echo "[$TAG] ok"
