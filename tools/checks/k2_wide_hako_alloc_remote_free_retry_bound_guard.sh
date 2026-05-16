#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-remote-free-retry-bound"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

REMOTE_POLICY="lang/src/hako_alloc/memory/remote_free_policy_box.hako"
SSOT="docs/development/current/main/design/hako-alloc-remote-free-retry-bound-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-471-MIMAP-039A-REMOTE-FREE-RETRY-BOUND.md"
INDEX="docs/tools/check-scripts-index.md"
EXISTING_GUARD="tools/checks/k2_wide_hako_alloc_remote_free_policy_exe_guard.sh"

echo "[$TAG] running MIMAP-039A remote-free retry bound guard"

guard_require_files "$TAG" "$REMOTE_POLICY" "$SSOT" "$CARD" "$INDEX" "$EXISTING_GUARD"
guard_require_exec_files "$TAG" "$0" "$EXISTING_GUARD"

guard_expect_in_file "$TAG" "Status: landed" "$CARD" "MIMAP-039A card must be landed after implementation"
guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "retry-bound SSOT must be accepted"
guard_expect_in_file "$TAG" "maxPushRetries\\(\\)" "$REMOTE_POLICY" "remote-free policy must expose named retry bound"
guard_expect_in_file "$TAG" "local max_retries = HakoAllocRemoteFreePolicy\\.maxPushRetries\\(\\)" "$REMOTE_POLICY" "pushRetry must read named retry bound"
guard_expect_in_file "$TAG" "loop \\(done == 0 && retries < max_retries\\)" "$REMOTE_POLICY" "pushRetry loop must use named retry bound"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check script index must list retry-bound guard"

if rg -n 'retries < 5|loop[[:space:]]*\(done == 0 && retries < [0-9]+' "$REMOTE_POLICY" >/tmp/"$TAG".raw_bound 2>&1; then
  cat /tmp/"$TAG".raw_bound >&2
  rm -f /tmp/"$TAG".raw_bound
  guard_fail "$TAG" "pushRetry must not use a raw numeric retry bound"
fi
rm -f /tmp/"$TAG".raw_bound

if rg -n 'hako_atomic_ptr_fetch_add|ptr_fetch_add|provider|global_allocator|install_hook|hook|page_map|PageMap' "$REMOTE_POLICY" >/tmp/"$TAG".forbidden 2>&1; then
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  guard_fail "$TAG" "retry-bound cleanup must not widen remote-free/provider behavior"
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'hako-alloc-remote-free-retry-bound|maxPushRetries' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "retry-bound cleanup matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

bash "$EXISTING_GUARD"

echo "[$TAG] ok"
