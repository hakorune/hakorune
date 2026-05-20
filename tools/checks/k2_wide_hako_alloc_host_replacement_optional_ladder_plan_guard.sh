#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-host-replacement-optional-ladder-plan"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

DESIGN="docs/development/current/main/design/hako-alloc-host-replacement-optional-ladder-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
CARD_415A="docs/development/current/main/phases/phase-293x/293x-1037-MIMAP-415A-REAL-EXTERNAL-PROVIDER-API-CALL-FIRST-PATTERN-PILOT.md"
CARD_417A="docs/development/current/main/phases/phase-293x/293x-1039-MIMAP-417A-REAL-EXTERNAL-PROVIDER-API-CALL-FIRST-PATTERN-CLOSEOUT.md"
CARD_418A="docs/development/current/main/phases/phase-293x/293x-1040-MIMAP-418A-POST-REAL-EXTERNAL-PROVIDER-API-CALL-FIRST-PATTERN-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1041-MIMAP-419A-HOST-REPLACEMENT-OPTIONAL-LADDER-PLAN.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1042-MIMAP-420A-HOST-REPLACEMENT-EXPLICIT-PREFLIGHT-INVENTORY.md"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_host_replacement_optional_ladder_plan_guard.sh"

printf '[%s] checking MIMAP-419A host replacement optional ladder plan\n' "$TAG"

guard_require_files "$TAG" "$DESIGN" "$INDEX" "$CARD_415A" "$CARD_417A" "$CARD_418A" "$CARD" "$NEXT_CARD" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

for card in "$CARD_415A" "$CARD_417A" "$CARD_418A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-420A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-419A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-419A guard"
guard_expect_in_file "$TAG" 'MIMAP-420A host replacement explicit preflight inventory' "$DESIGN" "plan must select explicit preflight inventory next"
guard_expect_in_file "$TAG" 'not the default process allocator' "$DESIGN" "plan must keep hako_alloc as optional/comparable allocator"
guard_expect_in_file "$TAG" 'process allocator replacement' "$DESIGN" "plan must name process replacement as still closed"
guard_expect_in_file "$TAG" '#\[global_allocator\]' "$DESIGN" "plan must keep global allocator install closed"

if rg -n 'replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|backendMatcherInstall[[:space:]]*\(|global_allocator[[:space:]]*=|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$DESIGN" "$CARD" "$NEXT_CARD" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-419A plan must not open replacement/hook/backend/source-concurrency seams" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

printf '[%s] ok\n' "$TAG"
