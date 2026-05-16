#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-huge-backing-set-helper"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/mimalloc-facade-huge-backing-set-helper-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-467-MIMAP-037A-FACADE-HUGE-BACKING-SET-HELPER.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
ROOT_README="lang/src/hako_alloc/README.md"
HELPER="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_backing_set_box.hako"
ROUTE="lang/src/hako_alloc/memory/object_lifecycle_facade_huge_unreserve_failfast_box.hako"
MIMAP035A_GUARD="tools/checks/k2_wide_mimalloc_facade_huge_unreserve_failfast_exe_guard.sh"

echo "[$TAG] checking MIMAP-037A facade huge backing-set helper"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$CARD" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$INDEX" \
  "$MODULE" \
  "$MEMORY_README" \
  "$ROOT_README" \
  "$HELPER" \
  "$ROUTE" \
  "$MIMAP035A_GUARD"
guard_require_exec_files "$TAG" "$0" "$MIMAP035A_GUARD"

guard_expect_in_file "$TAG" "Status: landed" "$CARD" "MIMAP-037A card must be landed after implementation"
guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-037A helper SSOT must be accepted"
guard_expect_in_file "$TAG" 'box HakoAllocObjectLifecycleFacadeHugeBackingSet' "$HELPER" "helper box missing"
guard_expect_in_file "$TAG" 'bases: ArrayBox' "$HELPER" "helper must own backing bases"
guard_expect_in_file "$TAG" 'bytes_values: ArrayBox' "$HELPER" "helper must own backing byte lengths"
guard_expect_in_file "$TAG" 'find\(base, bytes\)' "$HELPER" "helper must own exact pair lookup"
guard_expect_in_file "$TAG" 'has\(base, bytes\)' "$HELPER" "helper must own membership check"
guard_expect_in_file "$TAG" 'mark\(base, bytes\)' "$HELPER" "helper must own mark operation"
guard_expect_in_file "$TAG" 'return 5' "$HELPER" "helper must preserve duplicate status vocabulary"

guard_expect_in_file "$TAG" 'using selfhost\.hako_alloc\.memory\.object_lifecycle_facade_huge_backing_set_box' "$ROUTE" "failfast route must import helper"
guard_expect_in_file "$TAG" 'unreserved_backings: HakoAllocObjectLifecycleFacadeHugeBackingSet' "$ROUTE" "failfast route must own helper field"
guard_expect_in_file "$TAG" 'return me\.unreserved_backings\.find\(base, bytes\)' "$ROUTE" "route lookup must delegate to helper"
guard_expect_in_file "$TAG" 'return me\.unreserved_backings\.has\(base, bytes\)' "$ROUTE" "route membership must delegate to helper"
guard_expect_in_file "$TAG" 'return me\.unreserved_backings\.mark\(base, bytes\)' "$ROUTE" "route mark must delegate to helper"
guard_expect_in_file "$TAG" 'result\.marked_count = me\.unreserved_backings\.length\(\)' "$ROUTE" "route counters must read helper length"

guard_expect_in_file "$TAG" 'memory.object_lifecycle_facade_huge_backing_set_box = "memory/object_lifecycle_facade_huge_backing_set_box.hako"' "$MODULE" "module must export helper"
guard_expect_in_file "$TAG" 'object_lifecycle_facade_huge_backing_set_box.hako' "$MEMORY_README" "memory README must name helper"
guard_expect_in_file "$TAG" 'HakoAllocObjectLifecycleFacadeHugeBackingSet' "$ROOT_README" "root README must name helper"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check index must list MIMAP-037A guard"
guard_expect_in_file "$TAG" "$MIMAP035A_GUARD" "$INDEX" "check index must retain MIMAP-035A proof guard"
guard_expect_in_file "$TAG" '| `MIMAP-037A` | facade huge backing-set helper cleanup | landed after MIMAP-036B |' "$GRANULARITY" "granularity SSOT must list MIMAP-037A"
guard_expect_in_file "$TAG" '| `MIMAP-037A` | landed | Facade huge backing-set helper cleanup. | after MIMAP-036B |' "$TASKBOARD" "taskboard must mark MIMAP-037A landed"

if rg -n 'unreserved_bases|unreserved_bytes' "$ROUTE" >/tmp/"$TAG".parallel_arrays 2>&1; then
  cat /tmp/"$TAG".parallel_arrays >&2
  rm -f /tmp/"$TAG".parallel_arrays
  guard_fail "$TAG" "failfast route must not own parallel unreserved base/bytes arrays"
fi
rm -f /tmp/"$TAG".parallel_arrays

if rg -n 'HakoAllocPageSourcePolicy\.|OsVmCoreBox\.|\.unreservePage[[:space:]]*\(|\.decommitPage[[:space:]]*\(|\.recommitPage[[:space:]]*\(' \
  "$HELPER" >/tmp/"$TAG".substrate_leak 2>&1; then
  cat /tmp/"$TAG".substrate_leak >&2
  rm -f /tmp/"$TAG".substrate_leak
  guard_fail "$TAG" "helper must not call page-source or OSVM APIs"
fi
rm -f /tmp/"$TAG".substrate_leak

if rg -n 'provider[A-Za-z0-9_]*[[:space:]]*\(|global_allocator|GlobalAlloc|replace_allocator|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(' \
  "$HELPER" "$ROUTE" >/tmp/"$TAG".provider_leak 2>&1; then
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  guard_fail "$TAG" "MIMAP-037A must not open provider or host allocator replacement"
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'mimalloc-facade-huge-backing-set|HakoAllocObjectLifecycleFacadeHugeBackingSet|object_lifecycle_facade_huge_backing_set' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "MIMAP-037A matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

echo "[$TAG] ok"
