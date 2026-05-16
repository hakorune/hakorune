#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-osvm-fast-path-route-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/mimalloc-osvm-fast-path-route-closeout-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
CARD_042A="docs/development/current/main/phases/phase-293x/293x-516-MIMAP-042A-OSVM-FAST-PATH-BOUNDED-PURGE-ROUTE.md"
CARD_043A="docs/development/current/main/phases/phase-293x/293x-518-MIMAP-043A-OSVM-FAST-PATH-RECOMMIT-REUSE-ROUTE.md"
CARD_044A="docs/development/current/main/phases/phase-293x/293x-520-MIMAP-044A-OSVM-FAST-PATH-ROUTE-CLOSEOUT-GUARD.md"
CARD_044B="docs/development/current/main/phases/phase-293x/293x-521-MIMAP-044B-POST-FAST-PATH-CLOSEOUT-ROW-SELECTION.md"
INDEX="docs/tools/check-scripts-index.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
ROOT_README="lang/src/hako_alloc/README.md"
ROUTE_042A="lang/src/hako_alloc/memory/osvm_fast_path_purge_route_box.hako"
ROUTE_043A="lang/src/hako_alloc/memory/osvm_fast_path_reuse_route_box.hako"
APP_042A="apps/hako-alloc-osvm-fast-path-purge-route-proof/main.hako"
APP_043A="apps/hako-alloc-osvm-fast-path-reuse-route-proof/main.hako"
GUARD_042A="tools/checks/k2_wide_hako_alloc_osvm_fast_path_purge_route_guard.sh"
GUARD_043A="tools/checks/k2_wide_hako_alloc_osvm_fast_path_reuse_route_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_osvm_fast_path_route_closeout_guard.sh"

echo "[$TAG] checking MIMAP-044A OSVM-backed fast-path route closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$CARD_042A" \
  "$CARD_043A" \
  "$CARD_044A" \
  "$CARD_044B" \
  "$INDEX" \
  "$MODULE" \
  "$MEMORY_README" \
  "$ROOT_README" \
  "$ROUTE_042A" \
  "$ROUTE_043A" \
  "$APP_042A" \
  "$APP_043A" \
  "$GUARD_042A" \
  "$GUARD_043A" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$GUARD_042A" "$GUARD_043A" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_042A" "MIMAP-042A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_043A" "MIMAP-043A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_044A" "MIMAP-044A card must be landed after closeout"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_044B" "MIMAP-044B must be selected after closeout"

guard_expect_in_file "$TAG" "MIMAP-042A" "$SSOT" "closeout SSOT must include bounded purge row"
guard_expect_in_file "$TAG" "MIMAP-043A" "$SSOT" "closeout SSOT must include recommit/reuse row"
guard_expect_in_file "$TAG" "provider activation" "$SSOT" "closeout SSOT must keep provider activation inactive"
guard_expect_in_file "$TAG" "process allocator replacement" "$SSOT" "closeout SSOT must keep process replacement inactive"
guard_expect_in_file "$TAG" "MIMAP-044B post-fast-path-closeout row selection" "$SSOT" "closeout SSOT must name the next planning row"

guard_expect_in_file "$TAG" '| `MIMAP-044A` | OSVM-backed fast-path route closeout guard | landed after MIMAP-043B |' "$GRANULARITY" "granularity SSOT must mark MIMAP-044A landed"
guard_expect_in_file "$TAG" '| `MIMAP-044B` | post-fast-path-closeout row selection | selected current |' "$GRANULARITY" "granularity SSOT must mark MIMAP-044B current"
guard_expect_in_file "$TAG" '| `MIMAP-044A` | landed | OSVM-backed fast-path route closeout guard. | after MIMAP-043B |' "$TASKBOARD" "taskboard must mark MIMAP-044A landed"
guard_expect_in_file "$TAG" '| `MIMAP-044B` | selected current | Post-fast-path-closeout row selection. | selected after MIMAP-044A |' "$TASKBOARD" "taskboard must mark MIMAP-044B current"

guard_expect_in_file "$TAG" 'memory.osvm_fast_path_purge_route_box = "memory/osvm_fast_path_purge_route_box.hako"' "$MODULE" "MIMAP-042A owner must stay exported"
guard_expect_in_file "$TAG" 'memory.osvm_fast_path_reuse_route_box = "memory/osvm_fast_path_reuse_route_box.hako"' "$MODULE" "MIMAP-043A owner must stay exported"
guard_expect_in_file "$TAG" 'HakoAllocOsVmFastPathPurgeRoute' "$ROOT_README" "root README must name MIMAP-042A owner"
guard_expect_in_file "$TAG" 'HakoAllocOsVmFastPathReuseRoute' "$ROOT_README" "root README must name MIMAP-043A owner"
guard_expect_in_file "$TAG" 'osvm_fast_path_purge_route_box.hako' "$MEMORY_README" "memory README must name MIMAP-042A owner"
guard_expect_in_file "$TAG" 'osvm_fast_path_reuse_route_box.hako' "$MEMORY_README" "memory README must name MIMAP-043A owner"

guard_expect_in_file "$TAG" "$GUARD_042A" "$INDEX" "check index must list MIMAP-042A guard"
guard_expect_in_file "$TAG" "$GUARD_043A" "$INDEX" "check index must list MIMAP-043A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-044A closeout guard"

if rg -n 'HakoAllocPageSourcePolicy|OsVmCoreBox|reservePage[[:space:]]*\(|commitPage[[:space:]]*\(|decommitPage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(' \
  "$ROUTE_042A" "$ROUTE_043A" >/tmp/"$TAG".direct_execution_leak 2>&1; then
  cat /tmp/"$TAG".direct_execution_leak >&2
  rm -f /tmp/"$TAG".direct_execution_leak
  guard_fail "$TAG" "route owners must not call page-source/OSVM/unreserve/OS-release seams directly"
fi
rm -f /tmp/"$TAG".direct_execution_leak

if rg -n 'hako-alloc-osvm-fast-path-(purge|reuse)-route-proof|HakoAllocOsVmFastPath(Purge|Reuse)Route|osvm_fast_path_(purge|reuse)_route' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "OSVM fast-path route matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n '#\[global_allocator\]|GlobalAlloc|replace_allocator\(' \
  src lang/c-abi/shims crates/nyash_kernel lang/src >/tmp/"$TAG".replacement_leak 2>&1; then
  cat /tmp/"$TAG".replacement_leak >&2
  rm -f /tmp/"$TAG".replacement_leak
  guard_fail "$TAG" "provider / host allocator replacement must remain inactive"
fi
rm -f /tmp/"$TAG".replacement_leak

if rg -n 'AtomicCoreBox|TlsCoreBox|worker_local|spawn[[:space:]]*\(|thread::|RemoteFree|remote_free' \
  "$ROUTE_042A" "$ROUTE_043A" "$APP_042A" "$APP_043A" >/tmp/"$TAG".concurrency_leak 2>&1; then
  cat /tmp/"$TAG".concurrency_leak >&2
  rm -f /tmp/"$TAG".concurrency_leak
  guard_fail "$TAG" "OSVM fast-path route closeout must not open concurrency execution"
fi
rm -f /tmp/"$TAG".concurrency_leak

echo "[$TAG] ok"
