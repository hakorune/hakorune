#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-post-huge-unreserve-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/mimalloc-post-huge-unreserve-closeout-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
CARD_034A="docs/development/current/main/phases/phase-293x/293x-461-MIMAP-034A-FACADE-HUGE-UNRESERVE-ROUTE.md"
CARD_035A="docs/development/current/main/phases/phase-293x/293x-463-MIMAP-035A-FACADE-HUGE-UNRESERVE-FAILFAST.md"
CARD_036A="docs/development/current/main/phases/phase-293x/293x-465-MIMAP-036A-POST-HUGE-UNRESERVE-CLOSEOUT-GUARD.md"
CARD_036B="docs/development/current/main/phases/phase-293x/293x-466-MIMAP-036B-POST-HUGE-UNRESERVE-CLOSEOUT-ROW-SELECTION.md"
INDEX="docs/tools/check-scripts-index.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
ROOT_README="lang/src/hako_alloc/README.md"
ROUTE_034A="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_unreserve_box.hako"
ROUTE_035A="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_unreserve_failfast_box.hako"
APP_034A="apps/mimalloc-facade-huge-unreserve-proof/main.hako"
APP_035A="apps/mimalloc-facade-huge-unreserve-failfast-proof/main.hako"
GUARD_034A="tools/checks/k2_wide_mimalloc_facade_huge_unreserve_exe_guard.sh"
GUARD_035A="tools/checks/k2_wide_mimalloc_facade_huge_unreserve_failfast_exe_guard.sh"

echo "[$TAG] checking MIMAP-036A post-huge-unreserve closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$CARD_034A" \
  "$CARD_035A" \
  "$CARD_036A" \
  "$CARD_036B" \
  "$INDEX" \
  "$MODULE" \
  "$MEMORY_README" \
  "$ROOT_README" \
  "$ROUTE_034A" \
  "$ROUTE_035A" \
  "$APP_034A" \
  "$APP_035A" \
  "$GUARD_034A" \
  "$GUARD_035A"
guard_require_exec_files "$TAG" "$GUARD_034A" "$GUARD_035A" "$0"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_034A" "MIMAP-034A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_035A" "MIMAP-035A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_036A" "MIMAP-036A must be landed after closeout"
guard_expect_in_file "$TAG" "MIMAP-035B selected MIMAP-036A" "$CARD_036A" "MIMAP-036A card must name its selection source"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_036B" "MIMAP-036B must remain landed after closeout selection"

guard_expect_in_file "$TAG" "MIMAP-032A" "$SSOT" "closeout SSOT must include substrate unreserve row"
guard_expect_in_file "$TAG" "MIMAP-033A" "$SSOT" "closeout SSOT must include page-source unreserve adapter row"
guard_expect_in_file "$TAG" "MIMAP-034A" "$SSOT" "closeout SSOT must include facade success row"
guard_expect_in_file "$TAG" "MIMAP-035A" "$SSOT" "closeout SSOT must include facade fail-fast row"
guard_expect_in_file "$TAG" "provider activation" "$SSOT" "closeout SSOT must keep provider activation visible as inactive"
guard_expect_in_file "$TAG" "process allocator replacement" "$SSOT" "closeout SSOT must keep process replacement inactive"
guard_expect_in_file "$TAG" "MIMAP-036B post-huge-unreserve-closeout row selection" "$SSOT" "closeout SSOT must name the next planning row"

guard_expect_in_file "$TAG" '| `MIMAP-036A` | post-huge-unreserve closeout guard | landed after MIMAP-035B |' "$GRANULARITY" "granularity SSOT must mark MIMAP-036A landed"
guard_expect_in_file "$TAG" '| `MIMAP-036B` | post-huge-unreserve-closeout row selection | landed; selected MIMAP-037A |' "$GRANULARITY" "granularity SSOT must keep MIMAP-036B closeout selection"
guard_expect_in_file "$TAG" '| `MIMAP-036A` | landed | Post-huge-unreserve closeout guard. | after MIMAP-035B |' "$TASKBOARD" "taskboard must mark MIMAP-036A landed"
guard_expect_in_file "$TAG" '| `MIMAP-036B` | landed | Post-huge-unreserve-closeout row selection. | selected MIMAP-037A |' "$TASKBOARD" "taskboard must keep MIMAP-036B closeout selection"

guard_expect_in_file "$TAG" 'memory.object_lifecycle_facade_huge_unreserve_box = "memory/object_lifecycle_facade_huge_unreserve_box.hako"' "$MODULE" "MIMAP-034A owner must stay exported"
guard_expect_in_file "$TAG" 'memory.object_lifecycle_facade_huge_unreserve_failfast_box = "memory/object_lifecycle_facade_huge_unreserve_failfast_box.hako"' "$MODULE" "MIMAP-035A owner must stay exported"
guard_expect_in_file "$TAG" 'HakoAllocObjectLifecycleFacadeHugeUnreserveRoute' "$ROOT_README" "root README must name MIMAP-034A owner"
guard_expect_in_file "$TAG" 'HakoAllocObjectLifecycleFacadeHugeUnreserveFailfastRoute' "$ROOT_README" "root README must name MIMAP-035A owner"
guard_expect_in_file "$TAG" 'object_lifecycle_facade_huge_unreserve_box.hako' "$MEMORY_README" "memory README must name MIMAP-034A owner"
guard_expect_in_file "$TAG" 'object_lifecycle_facade_huge_unreserve_failfast_box.hako' "$MEMORY_README" "memory README must name MIMAP-035A owner"

guard_expect_in_file "$TAG" "$GUARD_034A" "$INDEX" "check index must list MIMAP-034A guard"
guard_expect_in_file "$TAG" "$GUARD_035A" "$INDEX" "check index must list MIMAP-035A guard"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check index must list MIMAP-036A closeout guard"

if rg -n 'mimalloc-facade-huge-unreserve|HakoAllocObjectLifecycleFacadeHugeUnreserve|object_lifecycle_facade_huge_unreserve' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "facade huge unreserve matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n '#\[global_allocator\]|GlobalAlloc|replace_allocator\(' \
  src lang/c-abi/shims crates/nyash_kernel lang/src >/tmp/"$TAG".replacement_leak 2>&1; then
  cat /tmp/"$TAG".replacement_leak >&2
  rm -f /tmp/"$TAG".replacement_leak
  guard_fail "$TAG" "host allocator replacement must remain inactive"
fi
rm -f /tmp/"$TAG".replacement_leak

echo "[$TAG] ok"
