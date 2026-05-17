#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-osvm-fast-path-unreserve-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/mimalloc-osvm-fast-path-unreserve-closeout-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
CARD_045A="docs/development/current/main/phases/phase-293x/293x-522-MIMAP-045A-OSVM-FAST-PATH-UNRESERVE-ROUTE.md"
CARD_046A="docs/development/current/main/phases/phase-293x/293x-524-MIMAP-046A-OSVM-FAST-PATH-UNRESERVE-FAILFAST.md"
CARD_047A="docs/development/current/main/phases/phase-293x/293x-526-MIMAP-047A-OSVM-FAST-PATH-UNRESERVE-CLOSEOUT-GUARD.md"
CARD_047B="docs/development/current/main/phases/phase-293x/293x-527-MIMAP-047B-POST-FAST-PATH-UNRESERVE-CLOSEOUT-ROW-SELECTION.md"
INDEX="docs/tools/check-scripts-index.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
ROOT_README="lang/src/hako_alloc/README.md"
ROUTE_045A="lang/src/hako_alloc/memory/osvm_fast_path_unreserve_route_box.hako"
ROUTE_046A="lang/src/hako_alloc/memory/osvm_fast_path_unreserve_failfast_box.hako"
APP_045A="apps/hako-alloc-osvm-fast-path-unreserve-route-proof/main.hako"
APP_046A="apps/hako-alloc-osvm-fast-path-unreserve-failfast-proof/main.hako"
GUARD_045A="tools/checks/k2_wide_hako_alloc_osvm_fast_path_unreserve_route_guard.sh"
GUARD_046A="tools/checks/k2_wide_hako_alloc_osvm_fast_path_unreserve_failfast_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_osvm_fast_path_unreserve_closeout_guard.sh"

echo "[$TAG] checking MIMAP-047A OSVM-backed fast-path unreserve closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$CARD_045A" \
  "$CARD_046A" \
  "$CARD_047A" \
  "$CARD_047B" \
  "$INDEX" \
  "$MODULE" \
  "$MEMORY_README" \
  "$ROOT_README" \
  "$ROUTE_045A" \
  "$ROUTE_046A" \
  "$APP_045A" \
  "$APP_046A" \
  "$GUARD_045A" \
  "$GUARD_046A" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$GUARD_045A" "$GUARD_046A" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_045A" "MIMAP-045A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_046A" "MIMAP-046A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_047A" "MIMAP-047A card must be landed after closeout"
guard_expect_in_file "$TAG" "Status:" "$CARD_047B" "MIMAP-047B selection card must have status"
guard_expect_in_file "$TAG" "MIMAP-047B" "$CARD_047B" "MIMAP-047B selection card must stay present after closeout"

guard_expect_in_file "$TAG" "MIMAP-045A" "$SSOT" "closeout SSOT must include success row"
guard_expect_in_file "$TAG" "MIMAP-046A" "$SSOT" "closeout SSOT must include fail-fast row"
guard_expect_in_file "$TAG" "provider activation" "$SSOT" "closeout SSOT must keep provider activation inactive"
guard_expect_in_file "$TAG" "process allocator replacement" "$SSOT" "closeout SSOT must keep process replacement inactive"
guard_expect_in_file "$TAG" "MIMAP-047B post-fast-path-unreserve-closeout row selection" "$SSOT" "closeout SSOT must name the next planning row"

guard_expect_in_file "$TAG" '| `MIMAP-047A` | OSVM-backed fast-path unreserve closeout guard | landed after MIMAP-046B |' "$GRANULARITY" "granularity SSOT must mark MIMAP-047A landed"
guard_expect_in_file "$TAG" '| `MIMAP-047B` | post-fast-path-unreserve-closeout row selection | selected current |' "$GRANULARITY" "granularity SSOT must mark MIMAP-047B current"
guard_expect_in_file "$TAG" '| `MIMAP-047A` | landed | OSVM-backed fast-path unreserve closeout guard. | after MIMAP-046B |' "$TASKBOARD" "taskboard must mark MIMAP-047A landed"
guard_expect_in_file "$TAG" '| `MIMAP-047B` | selected current | Post-fast-path-unreserve-closeout row selection. | selected after MIMAP-047A |' "$TASKBOARD" "taskboard must mark MIMAP-047B current"

guard_expect_in_file "$TAG" 'memory.osvm_fast_path_unreserve_route_box = "memory/osvm_fast_path_unreserve_route_box.hako"' "$MODULE" "MIMAP-045A owner must stay exported"
guard_expect_in_file "$TAG" 'memory.osvm_fast_path_unreserve_failfast_box = "memory/osvm_fast_path_unreserve_failfast_box.hako"' "$MODULE" "MIMAP-046A owner must stay exported"
guard_expect_in_file "$TAG" 'HakoAllocOsVmFastPathUnreserveRoute' "$ROOT_README" "root README must name MIMAP-045A owner"
guard_expect_in_file "$TAG" 'HakoAllocOsVmFastPathUnreserveFailFastRoute' "$ROOT_README" "root README must name MIMAP-046A owner"
guard_expect_in_file "$TAG" 'osvm_fast_path_unreserve_route_box.hako' "$MEMORY_README" "memory README must name MIMAP-045A owner"
guard_expect_in_file "$TAG" 'osvm_fast_path_unreserve_failfast_box.hako' "$MEMORY_README" "memory README must name MIMAP-046A owner"

guard_expect_in_file "$TAG" "$GUARD_045A" "$INDEX" "check index must list MIMAP-045A guard"
guard_expect_in_file "$TAG" "$GUARD_046A" "$INDEX" "check index must list MIMAP-046A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-047A closeout guard"

if rg -n 'HakoAllocPageSourcePolicy|OsVmCoreBox|(^|[^A-Za-z0-9_])(reservePage|commitPage|decommitPage|releasePage)[[:space:]]*\(|unreserve_bytes_i64' \
  "$ROUTE_045A" "$ROUTE_046A" >/tmp/"$TAG".direct_execution_leak 2>&1; then
  cat /tmp/"$TAG".direct_execution_leak >&2
  rm -f /tmp/"$TAG".direct_execution_leak
  guard_fail "$TAG" "route owners must not call page-source/OSVM/OS-release seams directly"
fi
rm -f /tmp/"$TAG".direct_execution_leak

if rg -n 'hako-alloc-osvm-fast-path-unreserve-(route|failfast)-proof|HakoAllocOsVmFastPathUnreserve(Route|FailFastRoute)|osvm_fast_path_unreserve_(route|failfast)' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "OSVM fast-path unreserve matcher leaked into .inc"
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
  "$ROUTE_045A" "$ROUTE_046A" "$APP_045A" "$APP_046A" >/tmp/"$TAG".concurrency_leak 2>&1; then
  cat /tmp/"$TAG".concurrency_leak >&2
  rm -f /tmp/"$TAG".concurrency_leak
  guard_fail "$TAG" "OSVM fast-path unreserve closeout must not open concurrency execution"
fi
rm -f /tmp/"$TAG".concurrency_leak

echo "[$TAG] ok"
