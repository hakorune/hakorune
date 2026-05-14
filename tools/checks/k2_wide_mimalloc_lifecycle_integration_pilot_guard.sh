#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-lifecycle-integration-pilot"
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

APP="apps/mimalloc-lifecycle-integration-pilot-proof/main.hako"
README="apps/mimalloc-lifecycle-integration-pilot-proof/README.md"
PAGE="lang/src/hako_alloc/memory/page_box.hako"
CARD="docs/development/current/main/phases/phase-293x/293x-342-MIMAP-009-LIFECYCLE-INTEGRATION-PILOT.md"
SSOT="docs/development/current/main/design/mimalloc-lifecycle-integration-pilot-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_lifecycle_integration_pilot_guard.sh"

for path in "$APP" "$README" "$PAGE" "$CARD" "$SSOT" "$INDEX"; do
  require_file "$path"
done

expect_in_file 'decommitted: i64 = 0' "$PAGE" "page model must expose decommitted state"
expect_in_file 'decommit_count: i64 = 0' "$PAGE" "page model must count accepted decommit transitions"
expect_in_file 'recommit_count: i64 = 0' "$PAGE" "page model must count accepted recommit transitions"
expect_in_file 'reuse_count: i64 = 0' "$PAGE" "page model must count accepted reuse transitions"
expect_in_file 'lifecycle_reject_count: i64 = 0' "$PAGE" "page model must count rejected lifecycle transitions"
expect_in_file 'decommit\(\)' "$PAGE" "page model must expose decommit"
expect_in_file 'recommit\(\)' "$PAGE" "page model must expose recommit"
expect_in_file 'canReuse\(\)' "$PAGE" "page model must expose canReuse"
expect_in_file 'reuse\(\)' "$PAGE" "page model must expose reuse"
expect_in_file 'new HakoAllocPageModel' "$APP" "proof app must construct direct page model"
expect_in_file 'summary=ok' "$APP" "proof app must emit summary=ok on success"
expect_in_file 'Decision: accepted' "$SSOT" "SSOT must record accepted decision"
expect_in_file 'MIMAP-010 page queue lifecycle selection pilot' "$CARD" "card must point to next lifecycle queue row"
expect_in_file "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-009 guard"

if rg -n 'OSVM|OsVm|externcall|atomic|RawBuf|provider|global_allocator|install_hook|hook' "$APP" >/tmp/${TAG}.forbidden 2>/dev/null; then
  cat /tmp/${TAG}.forbidden >&2
  fail "MIMAP-009 proof must not activate substrate/provider/hook behavior"
fi

out="$(mktemp)"
err="$(mktemp)"
trap 'rm -f "$out" "$err" /tmp/${TAG}.forbidden' EXIT

if ! guard_timeout_run "$TAG" "${MIMAP_VM_TIMEOUT:-20s}" "$out" "$err" env NYASH_DISABLE_PLUGINS=1 cargo run -q --bin hakorune -- --backend vm "$APP"; then
  cat "$out" >&2 || true
  cat "$err" >&2 || true
  fail "proof app failed"
fi

rg -F -q 'mimalloc-lifecycle-integration-pilot-proof' "$out" || fail "missing proof banner"
rg -F -q 'summary=ok' "$out" || fail "proof app did not report summary=ok"
rg -F -q 'blocks=2,1,2,-1' "$out" || fail "unexpected lifecycle block sequence"
rg -F -q 'lifecycle=0,0,1,1,1,4' "$out" || fail "unexpected lifecycle counters"
rg -F -q 'counts=3,2,2,2,2,1' "$out" || fail "unexpected page counters"
rg -F -q 'state=1,2,0,2' "$out" || fail "unexpected final lifecycle state"
rg -F -q 'shape=17' "$out" || fail "unexpected lifecycle shape count"

cat "$out"
echo "[$TAG] ok"
