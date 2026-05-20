#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-optional-process-allocator-replacement-proposal"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

CARD_424A="docs/development/current/main/phases/phase-293x/293x-1046-MIMAP-424A-BACKEND-MATCHER-NO-GROWTH-CLOSEOUT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1047-MIMAP-425A-OPTIONAL-PROCESS-ALLOCATOR-REPLACEMENT-PROPOSAL.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1048-MIMAP-426A-POST-HOST-REPLACEMENT-OPTIONAL-LADDER-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-optional-process-allocator-replacement-proposal-ssot.md"
PREV_DESIGN="docs/development/current/main/design/hako-alloc-host-replacement-backend-matcher-no-growth-closeout-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_optional_process_allocator_replacement_proposal_guard.sh"

printf '[%s] checking MIMAP-425A optional process allocator replacement proposal\n' "$TAG"

guard_require_files "$TAG" "$CARD_424A" "$CARD" "$NEXT_CARD" "$DESIGN" "$PREV_DESIGN" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_424A" "MIMAP-424A must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-425A must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-426A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-425A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$PREV_DESIGN" "MIMAP-424A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-425A guard"
guard_expect_in_file "$TAG" 'performance and memory usage can be compared against C mimalloc' "$DESIGN" "proposal must keep comparison goal explicit"
guard_expect_in_file "$TAG" 'replacement_execution = closed' "$DESIGN" "proposal must keep replacement execution closed"
guard_expect_in_file "$TAG" 'hook_installation = closed' "$DESIGN" "proposal must keep hook install closed"
guard_expect_in_file "$TAG" 'backend_matcher_additions = closed' "$DESIGN" "proposal must keep backend matcher additions closed"
guard_expect_in_file "$TAG" 'global_allocator_install = closed' "$DESIGN" "proposal must keep global allocator install closed"
guard_expect_in_file "$TAG" 'optional replacement execution remains parked' "$NEXT_CARD" "next row must keep replacement execution parked"

if rg -n 'replace_process_allocator[[:space:]]*\(|install_hook[[:space:]]*\(|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$DESIGN" "$CARD" "$NEXT_CARD" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-425A docs must not open replacement/hook/backend/source-concurrency seams" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'OptionalProcessAllocatorReplacement|optional-process-allocator-replacement|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-425A replacement matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

printf '[%s] ok\n' "$TAG"
