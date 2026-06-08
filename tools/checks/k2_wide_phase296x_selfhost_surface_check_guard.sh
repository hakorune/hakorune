#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-selfhost-surface-check"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="docs/development/current/main/phases/phase-296x/296x-639-MIM-PORT-FMEM-140-SELFHOST-SURFACE-PREFLIGHT-TASK-ORDER.md"
OUTBOX_CARD="docs/development/current/main/phases/phase-296x/296x-640-OUTBOX-0-NARROW-LOWERING-LANDING.md"
WORKSTREAM="docs/development/current/main/workstreams/mimalloc-current.md"
EXPRS="src/mir/builder/exprs.rs"
VAR_STMT="src/mir/builder/stmts/variable_stmt.rs"
META="src/mir/function/metadata.rs"
MIR_TEST="src/tests/mir_outbox_contract.rs"
PARSER_TEST="src/tests/parser_outbox_contract.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_selfhost_surface_check_guard.sh"

echo "[$TAG] checking selfhost surface after outbox landing"

guard_require_files \
  "$TAG" \
  "$CURRENT_STATE" \
  "$TASK_ORDER" \
  "$OUTBOX_CARD" \
  "$WORKSTREAM" \
  "$EXPRS" \
  "$VAR_STMT" \
  "$META" \
  "$MIR_TEST" \
  "$PARSER_TEST" \
  "$INDEX" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-641-SELFHOST-SURFACE-000-SURFACE-CHECK-LANDING"' "$CURRENT_STATE" "current state must point to the selfhost surface landing card"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIM-PORT-FMEM-031 AtomicRemoteHead CAS lowering producer selection"' "$CURRENT_STATE" "current blocker must move back to the mimalloc lane"
guard_expect_fixed_in_file "$TAG" '2026-06-08: 296x-641 landed SELFHOST-SURFACE-000 surface check landing' "$CURRENT_STATE" "current state must record the selfhost surface landing"

guard_expect_in_file "$TAG" '1\. SELFHOST-SURFACE-000' "$TASK_ORDER" "task order must keep selfhost surface check first"
guard_expect_in_file "$TAG" '2\. OUTBOX-0' "$TASK_ORDER" "task order must keep outbox as the second row"
guard_expect_in_file "$TAG" '3\. Post-outbox selfhost gate refresh' "$TASK_ORDER" "task order must keep the post-outbox gate refresh"

guard_expect_fixed_in_file "$TAG" 'Status: Done' "$OUTBOX_CARD" "outbox landing card must be done"
guard_expect_fixed_in_file "$TAG" 'outbox_lowering=1' "$OUTBOX_CARD" "outbox landing card must report lowering"
guard_expect_fixed_in_file "$TAG" 'outbox_binding_count>0' "$OUTBOX_CARD" "outbox landing card must report a binding count"
guard_expect_fixed_in_file "$TAG" 'outbox_transfer_return_metadata=1' "$OUTBOX_CARD" "outbox landing card must report transfer metadata"
guard_expect_fixed_in_file "$TAG" 'outbox_rich_move_checker=0' "$OUTBOX_CARD" "outbox landing card must keep rich move checking closed"

guard_expect_fixed_in_file "$TAG" 'build_outbox_statement' "$EXPRS" "expr lowering must call the outbox helper"
guard_expect_fixed_in_file "$TAG" 'outbox_bindings' "$VAR_STMT" "variable_stmt must record outbox bindings"
guard_expect_fixed_in_file "$TAG" 'outbox_bindings' "$META" "function metadata must carry outbox bindings"
guard_expect_fixed_in_file "$TAG" 'outbox_lowers_as_explicit_contract_binding' "$MIR_TEST" "MIR outbox test must exist"
guard_expect_fixed_in_file "$TAG" 'verification_result.is_ok()' "$MIR_TEST" "MIR outbox test must expect successful verification"
guard_expect_fixed_in_file "$TAG" 'outbox_duplicate_binding_is_fail_fast' "$PARSER_TEST" "parser outbox test must exist"
guard_expect_fixed_in_file "$TAG" '[freeze:contract][moved/outbox_duplicate]' "$PARSER_TEST" "parser outbox duplicate binding contract must stay fail-fast"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list the selfhost surface check"
guard_expect_fixed_in_file "$TAG" 'MIM-PORT-FMEM-031 AtomicRemoteHead CAS lowering producer selection' "$WORKSTREAM" "workstream must move back to the AtomicRemoteHead lane"
guard_expect_fixed_in_file "$TAG" 'SELFHOST-SURFACE-000:' "$WORKSTREAM" "workstream must keep the selfhost surface lane listed"
guard_expect_fixed_in_file "$TAG" 'OUTBOX-0:' "$WORKSTREAM" "workstream must keep the outbox lane listed"
guard_expect_fixed_in_file "$TAG" 'post-outbox selfhost gate refresh:' "$WORKSTREAM" "workstream must record the post-outbox gate refresh"

bash tools/checks/current_state_pointer_guard.sh

cargo test -q parser_outbox_contract --lib
cargo test -q mir_outbox_contract --lib

echo "[$TAG] ok"
