#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-page-queue-lifecycle-selection"
cd "$ROOT_DIR"

source tools/checks/lib/guard_common.sh

fail() {
  echo "[$TAG] ERROR: $*" >&2
  exit 1
}

require_file() {
  local path="$1"
  [[ -f "$path" ]] || fail "missing required file: $path"
}

expect_in_file() {
  local pattern="$1"
  local path="$2"
  local message="$3"
  rg -q "$pattern" "$path" || fail "$message"
}

APP="apps/mimalloc-page-queue-lifecycle-selection-proof/main.hako"
QUEUE="lang/src/hako_alloc/memory/page_queue_lifecycle_box.hako"
PAGE="lang/src/hako_alloc/memory/page_box.hako"
CARD="docs/development/current/main/phases/phase-293x/293x-343-MIMAP-010-PAGE-QUEUE-LIFECYCLE-SELECTION-PILOT.md"
SSOT="docs/development/current/main/design/mimalloc-page-queue-lifecycle-selection-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_page_queue_lifecycle_selection_guard.sh"

for path in "$APP" "$QUEUE" "$PAGE" "$CARD" "$SSOT" "$INDEX"; do
  require_file "$path"
done

expect_in_file 'box HakoAllocLifecyclePageQueue' "$QUEUE" "queue lifecycle owner must exist"
expect_in_file 'beginSelection\(\)' "$QUEUE" "queue must expose lifecycle selection reset"
expect_in_file 'considerPage\(page_id, decommitted, retired, reusable, available\)' "$QUEUE" "queue must expose scalar page consideration"
expect_in_file 'finishSelection\(\)' "$QUEUE" "queue must expose selection finish"
expect_in_file 'decommitted != 0' "$QUEUE" "queue must skip decommitted pages"
expect_in_file 'reusable != 0' "$QUEUE" "queue must select reusable retired pages"
expect_in_file 'last_selected_kind = 1' "$QUEUE" "queue must tag reusable selections"
expect_in_file 'Decision: accepted' "$SSOT" "SSOT must record accepted decision"
expect_in_file 'MIMAP-011 allocator facade lifecycle route pilot' "$CARD" "card must point to next facade row"
expect_in_file "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-010 guard"

if rg -n 'OSVM|OsVm|externcall|atomic|RawBuf|provider|global_allocator|install_hook|hook' "$APP" "$QUEUE" >/tmp/${TAG}.forbidden 2>/dev/null; then
  cat /tmp/${TAG}.forbidden >&2
  fail "MIMAP-010 proof must not activate substrate/provider/hook behavior"
fi

out="$(mktemp)"
err="$(mktemp)"
trap 'rm -f "$out" "$err" /tmp/${TAG}.forbidden' EXIT

if ! guard_timeout_run "$TAG" "${MIMAP_VM_TIMEOUT:-20s}" "$out" "$err" env NYASH_DISABLE_PLUGINS=1 cargo run -q --bin hakorune -- --backend vm "$APP"; then
  cat "$out" >&2 || true
  cat "$err" >&2 || true
  fail "proof app failed"
fi

rg -F -q 'mimalloc-page-queue-lifecycle-selection-proof' "$out" || fail "missing proof banner"
rg -F -q 'summary=ok' "$out" || fail "proof app did not report summary=ok"
rg -F -q 'pages=20,30,-1' "$out" || fail "unexpected selected page sequence"
rg -F -q 'kinds=1,2,0' "$out" || fail "unexpected selected kind sequence"
rg -F -q 'queue=2,1,1,3,0,1' "$out" || fail "unexpected queue counters"
rg -F -q 'shape=12' "$out" || fail "unexpected shape count"

cat "$out"
echo "[$TAG] ok"
