#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mimalloc-benchmark-lane-lock"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

ROADMAP="$ROOT_DIR/docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md"
README="$ROOT_DIR/docs/development/current/main/phases/phase-296x/README.md"
TASKBOARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CARD_00="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-00-MIMALLOC-BENCHMARK-LANE-LOCK.md"
CARD_01="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-01-MIMALLOC-BENCHMARK-HAKMEM-ASSET-INVENTORY.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
CHECK_INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
EXTERNAL_CORPUS="/home/tomoaki/git/hakmem_20260525_extracted/hakmem"

echo "[$TAG] checking phase-296x benchmark lane lock"

guard_require_files "$TAG" \
  "$ROADMAP" \
  "$README" \
  "$TASKBOARD" \
  "$CARD_00" \
  "$CARD_01" \
  "$CURRENT_STATE" \
  "$CHECK_INDEX"

if [[ ! -d "$EXTERNAL_CORPUS" ]]; then
  guard_fail "$TAG" "external hakmem corpus missing: $EXTERNAL_CORPUS"
fi

guard_expect_fixed_in_file "$TAG" 'active_lane = "phase-296x mimalloc benchmark contract"' "$CURRENT_STATE" "current state must point at phase-296x"
guard_expect_fixed_in_file "$TAG" 'active_phase = "docs/development/current/main/phases/phase-296x/README.md"' "$CURRENT_STATE" "current state active phase must be phase-296x"
guard_expect_fixed_in_file "$TAG" 'taskboard = "docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"' "$CURRENT_STATE" "current state taskboard must be phase-296x taskboard"
guard_expect_fixed_in_file "$TAG" 'method_anchor = "docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md"' "$CURRENT_STATE" "current state method anchor must be benchmark roadmap"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIMALLOC-BENCHMARK-HAKMEM-ASSET-INVENTORY-296X-001"' "$CURRENT_STATE" "current state must expose inventory blocker"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_00" "lane lock card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_01" "inventory card must be current"
guard_expect_fixed_in_file "$TAG" '| 0 | `MIMALLOC-BENCHMARK-LANE-LOCK-296X-001` | Landed |' "$TASKBOARD" "taskboard must mark lane lock landed"
guard_expect_fixed_in_file "$TAG" '| 1 | `MIMALLOC-BENCHMARK-HAKMEM-ASSET-INVENTORY-296X-001` | Current |' "$TASKBOARD" "taskboard must mark inventory current"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-DLL-LOAD-ONLY-SELECTION-296X-001' "$TASKBOARD" "taskboard must keep DLL work as later load-only selection"

for file in "$ROADMAP" "$README" "$TASKBOARD" "$CARD_00" "$CARD_01"; do
  guard_expect_fixed_in_file "$TAG" "$EXTERNAL_CORPUS" "$file" "$(basename "$file") must name external corpus"
done

guard_expect_fixed_in_file "$TAG" 'DLL/provider work starts only after all of the following are true' "$ROADMAP" "roadmap must define DLL timing"
guard_expect_fixed_in_file "$TAG" 'dll_mode=load-only' "$ROADMAP" "roadmap must keep first DLL row load-only"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$ROADMAP" "roadmap must close winner claims"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$ROADMAP" "roadmap must close provider activation"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$ROADMAP" "roadmap must close replacement"
guard_expect_fixed_in_file "$TAG" 'Mini-Agent Restart Queue' "$TASKBOARD" "taskboard must include mini-agent restart queue"
guard_expect_fixed_in_file "$TAG" 'tools/checks/k2_wide_phase296x_mimalloc_benchmark_lane_lock_guard.sh' "$CHECK_INDEX" "check index must list this guard"

echo "[$TAG] ok"
