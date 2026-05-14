#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-page-free-list-pilot"
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

APP="apps/mimalloc-page-free-list-pilot-proof/main.hako"
README="apps/mimalloc-page-free-list-pilot-proof/README.md"
PAGE="lang/src/hako_alloc/memory/page_box.hako"
CARD="docs/development/current/main/phases/phase-293x/293x-341-MIMAP-008-PAGE-FREE-LIST-PILOT.md"
SSOT="docs/development/current/main/design/mimalloc-page-free-list-pilot-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_page_free_list_pilot_guard.sh"

for path in "$APP" "$README" "$PAGE" "$CARD" "$SSOT" "$INDEX"; do
  require_file "$path"
done

expect_in_file 'box HakoAllocPageModel' "$PAGE" "page/free-list model owner must be HakoAllocPageModel"
expect_in_file 'releaseLocal\(block_id\)' "$PAGE" "page model must expose local-free release"
expect_in_file 'reactivate\(\)' "$PAGE" "page model must expose reactivation"
expect_in_file 'blockIsLive\(block_id\)' "$PAGE" "page model must expose block liveness probe"
expect_in_file 'using selfhost\.hako_alloc\.memory\.page_box as HakoAllocPageBox' "$APP" "proof app must import page model"
expect_in_file 'new HakoAllocPageModel' "$APP" "proof app must construct the direct page model"
expect_in_file 'summary=ok' "$APP" "proof app must emit summary=ok on success"
expect_in_file 'Decision: accepted' "$SSOT" "SSOT must record accepted decision"
expect_in_file 'MIMAP-009 lifecycle integration pilot' "$CARD" "card must point to next lifecycle row"
expect_in_file "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-008 guard"

if rg -n 'OSVM|OsVm|externcall|atomic|RawBuf|provider|global_allocator|install_hook|hook' "$APP" >/tmp/${TAG}.forbidden 2>/dev/null; then
  cat /tmp/${TAG}.forbidden >&2
  fail "MIMAP-008 proof must not activate substrate/provider/hook behavior"
fi

out="$(mktemp)"
err="$(mktemp)"
trap 'rm -f "$out" "$err" /tmp/${TAG}.forbidden' EXIT

if ! guard_timeout_run "$TAG" "${MIMAP_VM_TIMEOUT:-20s}" "$out" "$err" env NYASH_DISABLE_PLUGINS=1 cargo run -q --bin hakorune -- --backend vm "$APP"; then
  cat "$out" >&2 || true
  cat "$err" >&2 || true
  fail "proof app failed"
fi

rg -F -q 'mimalloc-page-free-list-pilot-proof' "$out" || fail "missing proof banner"
rg -F -q 'summary=ok' "$out" || fail "proof app did not report summary=ok"
rg -F -q 'blocks=3,2,1,-1,3' "$out" || fail "unexpected block sequence"
rg -F -q 'counts=4,3,3,3,3' "$out" || fail "unexpected page counters"
rg -F -q 'state=1,3,0,3,0' "$out" || fail "unexpected final page state"
rg -F -q 'shape=15' "$out" || fail "unexpected shape count"

cat "$out"
echo "[$TAG] ok"
