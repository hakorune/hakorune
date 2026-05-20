#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-real-external-provider-api-call-first-pattern-plan"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

DESIGN="docs/development/current/main/design/hako-alloc-real-external-provider-api-call-first-pattern-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
CARD_410A="docs/development/current/main/phases/phase-293x/293x-1032-MIMAP-410A-REAL-EXTERNAL-PROVIDER-API-ADAPTER-EXECUTION-PREFLIGHT.md"
CARD_412A="docs/development/current/main/phases/phase-293x/293x-1034-MIMAP-412A-REAL-EXTERNAL-PROVIDER-API-ADAPTER-EXECUTION-PREFLIGHT-CLOSEOUT.md"
CARD_413A="docs/development/current/main/phases/phase-293x/293x-1035-MIMAP-413A-POST-REAL-EXTERNAL-PROVIDER-API-ADAPTER-EXECUTION-PREFLIGHT-CLOSEOUT-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1036-MIMAP-414A-REAL-EXTERNAL-PROVIDER-API-CALL-FIRST-PATTERN-PLAN.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1037-MIMAP-415A-REAL-EXTERNAL-PROVIDER-API-CALL-FIRST-PATTERN-PILOT.md"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_real_external_provider_api_call_first_pattern_plan_guard.sh"

printf '[%s] checking MIMAP-414A real external provider API call first-pattern plan\n' "$TAG"

guard_require_files "$TAG" "$DESIGN" "$INDEX" "$CARD_410A" "$CARD_412A" "$CARD_413A" "$CARD" "$NEXT_CARD" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

for card in "$CARD_410A" "$CARD_412A" "$CARD_413A" "$CARD"; do
  guard_expect_in_file "$TAG" 'Status: landed' "$card" "$card must be landed"
done
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-415A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-414A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-414A guard"
guard_expect_in_file "$TAG" 'HakoAllocRealExternalProviderApiAdapterExecutionPreflightReport' "$DESIGN" "plan must consume the MIMAP-410A preflight report"
guard_expect_in_file "$TAG" 'real_external_provider_api_call_executed' "$DESIGN" "plan must define real-call execution evidence"
guard_expect_in_file "$TAG" 'host allocator replacement' "$DESIGN" "plan must keep host replacement closed"

if rg -n 'callProvider|actual_external_provider_api_call[[:space:]]*\(|replace_process_allocator|install_hook[[:space:]]*\(|global_allocator|backendMatcherInstall|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$DESIGN" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-414A plan must not open execution/replacement/hook/backend/source-concurrency seams" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

printf '[%s] ok\n' "$TAG"
