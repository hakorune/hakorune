#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="k2-wide-phase296x-direct-i64-arrayrepr-materialization-smoke-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_385="docs/development/current/main/phases/phase-296x/296x-385-DIRECTI64-ARRAYREPR-LOWERING-CONSUMER-REBASE.md"
CARD_386="docs/development/current/main/phases/phase-296x/296x-386-DIRECTI64-ARRAYREPR-MATERIALIZATION-SMOKE-REFRESH.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TEST_CONSTRUCTOR="src/llvm_py/tests/test_direct_array_i64_constructor_lowering.py"
TEST_CONSUMER="src/llvm_py/tests/test_collection_method_call.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_direct_i64_arrayrepr_materialization_smoke_refresh_guard.sh"

echo "[$TAG] checking DirectI64 ArrayRepr materialization smoke refresh"

guard_require_files "$TAG" "$CARD_385" "$CARD_386" "$STATE" "$INDEX" "$TEST_CONSTRUCTOR" "$TEST_CONSUMER" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_385" "row385 lowering consumer rebase must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_386" "row386 materialization smoke refresh must be landed"
guard_expect_fixed_in_file "$TAG" 'output_contract=direct-i64-arrayrepr-materialization-smoke-refresh-v0' "$CARD_386" "row386 must define the smoke output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=direct-i64-arrayrepr-lowering-consumer-rebase-v0' "$CARD_386" "row386 must consume row385"
guard_expect_fixed_in_file "$TAG" 'selected_boundary=direct_i64_arrayrepr_materialization_smoke_refresh' "$CARD_386" "row386 must stay on the smoke boundary"
guard_expect_fixed_in_file "$TAG" 'selected_next=direct_i64_arrayrepr_post_rebase_perf_owner_refresh' "$CARD_386" "row386 must point to the post-rebase perf owner refresh"
guard_expect_fixed_in_file "$TAG" 'public_arraybox_birth_smoke=ok' "$CARD_386" "row386 must keep public ArrayBox smoke"
guard_expect_fixed_in_file "$TAG" 'direct_array_birth_smoke=ok' "$CARD_386" "row386 must keep DirectArray birth smoke"
guard_expect_fixed_in_file "$TAG" 'direct_array_materialization_snapshot_smoke=ok' "$CARD_386" "row386 must keep materialization snapshot smoke"
guard_expect_fixed_in_file "$TAG" 'selected_method_direct_lowering_smoke=ok' "$CARD_386" "row386 must keep selected-method direct lowering smoke"
guard_expect_fixed_in_file "$TAG" 'silent_fallback_allowed=0' "$CARD_386" "row386 must forbid silent fallback"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-386-DIRECTI64-ARRAYREPR-MATERIALIZATION-SMOKE-REFRESH"' "$STATE" "current state must keep row386 as the latest landed card"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "DIRECTI64-ARRAYREPR-POST-REBASE-PERF-OWNER-REFRESH-296X-001"' "$STATE" "current state must point to row387"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" 'arrayrepr_facts' "$TEST_CONSTRUCTOR" "constructor tests must assert explicit ArrayRepr facts"
guard_expect_fixed_in_file "$TAG" 'resolver.direct_array_i64_ids' "$TEST_CONSUMER" "consumer tests must still preserve the legacy origin-only negative case"
guard_expect_fixed_in_file "$TAG" 'arrayrepr_facts' "$TEST_CONSUMER" "consumer tests must assert the explicit ArrayRepr fact"

python3 -m unittest \
  "$ROOT_DIR/src/llvm_py/tests/test_direct_array_i64_constructor_lowering.py" \
  "$ROOT_DIR/src/llvm_py/tests/test_collection_method_call.py"

echo "[$TAG] ok"
