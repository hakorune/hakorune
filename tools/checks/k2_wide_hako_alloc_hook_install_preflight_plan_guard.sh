#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-hook-install-preflight-plan"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

CARD_422A="docs/development/current/main/phases/phase-293x/293x-1044-MIMAP-422A-HOST-REPLACEMENT-PREFLIGHT-CLOSEOUT.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1045-MIMAP-423A-HOOK-INSTALL-PREFLIGHT-PLAN.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-1046-MIMAP-424A-BACKEND-MATCHER-NO-GROWTH-CLOSEOUT.md"
DESIGN="docs/development/current/main/design/hako-alloc-hook-install-preflight-plan-ssot.md"
PREV_DESIGN="docs/development/current/main/design/hako-alloc-host-replacement-preflight-closeout-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_hook_install_preflight_plan_guard.sh"

printf '[%s] checking MIMAP-423A hook-install preflight plan\n' "$TAG"

guard_require_files "$TAG" "$CARD_422A" "$CARD" "$NEXT_CARD" "$DESIGN" "$PREV_DESIGN" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_422A" "MIMAP-422A must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-423A must be landed"
guard_expect_in_file "$TAG" 'Status: selected current' "$NEXT_CARD" "MIMAP-424A must be selected current"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-423A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$PREV_DESIGN" "MIMAP-422A design must remain accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-423A guard"
guard_expect_in_file "$TAG" 'explicit_hook_install_request_present' "$DESIGN" "hook plan must require explicit hook request"
guard_expect_in_file "$TAG" 'hook_rollback_plan_present' "$DESIGN" "hook plan must require rollback plan"
guard_expect_in_file "$TAG" 'backend_no_growth_evidence_present' "$DESIGN" "hook plan must require backend no-growth evidence"
guard_expect_in_file "$TAG" 'would_install_hook != 0' "$DESIGN" "hook plan must keep install seam rejected"
guard_expect_in_file "$TAG" 'would_replace_host_allocator != 0' "$DESIGN" "hook plan must keep replacement seam rejected"
guard_expect_in_file "$TAG" 'MIMAP-424A should validate backend matcher no-growth' "$DESIGN" "hook plan must select backend no-growth closeout next"

if rg -n 'replace_process_allocator|install_hook[[:space:]]*\(|backendMatcherInstall|pointer_member|dereference[[:space:]]*\(|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|await|sync[[:space:]]+box|context[[:space:]]' "$DESIGN" "$CARD" "$NEXT_CARD" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-423A docs must not open replacement/hook/backend/source-concurrency seams" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'HookInstallPreflight|hook-install-preflight|replace_process_allocator|install_hook|#\[global_allocator\]|BackendMatcherInstaller' lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-423A hook/replacement matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

printf '[%s] ok\n' "$TAG"
