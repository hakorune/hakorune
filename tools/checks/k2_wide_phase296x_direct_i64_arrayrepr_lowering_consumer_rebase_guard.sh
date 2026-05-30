#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="k2-wide-phase296x-direct-i64-arrayrepr-lowering-consumer-rebase"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_384="docs/development/current/main/phases/phase-296x/296x-384-DIRECTI64-ARRAYREPR-PRODUCER-IMPLEMENTATION.md"
CARD_385="docs/development/current/main/phases/phase-296x/296x-385-DIRECTI64-ARRAYREPR-LOWERING-CONSUMER-REBASE.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
RESOLVER_HELPERS="src/llvm_py/utils/resolver_helpers.py"
CONSUMER="src/llvm_py/instructions/mir_call/collection_method_call.py"
PRODUCER_TEST="src/llvm_py/tests/test_direct_array_i64_constructor_lowering.py"
CONSUMER_TEST="src/llvm_py/tests/test_collection_method_call.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_direct_i64_arrayrepr_lowering_consumer_rebase_guard.sh"

echo "[$TAG] checking DirectI64 ArrayRepr lowering consumer rebase"

guard_require_files "$TAG" "$CARD_384" "$CARD_385" "$STATE" "$INDEX" "$RESOLVER_HELPERS" "$CONSUMER" "$PRODUCER_TEST" "$CONSUMER_TEST" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_384" "row384 producer implementation must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_385" "row385 lowering consumer rebase must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=direct-i64-arrayrepr-lowering-consumer-rebase-v0' "$CARD_385" "row385 must define the consumer rebase output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=direct-i64-arrayrepr-producer-implementation-v0' "$CARD_385" "row385 must consume row384"
guard_expect_fixed_in_file "$TAG" 'selected_boundary=direct_i64_arrayrepr_lowering_consumer_rebase' "$CARD_385" "row385 must stay on the consumer rebase boundary"
guard_expect_fixed_in_file "$TAG" 'selected_next=direct_i64_arrayrepr_materialization_smoke_refresh' "$CARD_385" "row385 must point to the materialization smoke refresh"
guard_expect_fixed_in_file "$TAG" 'consumer_fact_name=ArrayRepr::DirectI64' "$CARD_385" "row385 must name the explicit consumer fact"
guard_expect_fixed_in_file "$TAG" 'consumer_fact_owner=representation_planner' "$CARD_385" "row385 must assign consumer fact ownership"
guard_expect_fixed_in_file "$TAG" 'consumer_fact_store=resolver.arrayrepr_facts' "$CARD_385" "row385 must consume from the explicit fact store"
guard_expect_fixed_in_file "$TAG" 'consumer_selector_must_use_explicit_fact=1' "$CARD_385" "row385 must use the explicit ArrayRepr fact to select the direct path"
guard_expect_fixed_in_file "$TAG" 'legacy_origin_state_must_not_select_direct_path=1' "$CARD_385" "row385 must stop using the legacy origin state as selector"
guard_expect_fixed_in_file "$TAG" 'producer_origin_fact_compat=resolver.direct_array_i64_ids' "$CARD_385" "row385 must keep origin-state compatibility as history"
guard_expect_fixed_in_file "$TAG" 'public_arraybox_birth_unchanged=1' "$CARD_385" "row385 must keep public birth unchanged"
guard_expect_fixed_in_file "$TAG" 'silent_fallback_allowed=0' "$CARD_385" "row385 must forbid silent fallback"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-384-DIRECTI64-ARRAYREPR-PRODUCER-IMPLEMENTATION"' "$STATE" "current state must keep row384 as the latest landed card"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "DIRECTI64-ARRAYREPR-LOWERING-CONSUMER-REBASE-296X-001"' "$STATE" "current state must point to row385"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" 'is_arrayrepr_direct_i64' "$RESOLVER_HELPERS" "resolver helpers must expose the explicit ArrayRepr consumer check"
guard_expect_fixed_in_file "$TAG" 'arrayrepr_facts' "$RESOLVER_HELPERS" "resolver helpers must expose the ArrayRepr fact store"
guard_expect_fixed_in_file "$TAG" 'is_arrayrepr_direct_i64' "$CONSUMER" "consumer lowering must use the explicit ArrayRepr fact"
guard_expect_fixed_in_file "$TAG" 'arrayrepr_facts' "$PRODUCER_TEST" "producer tests must assert the explicit ArrayRepr fact"
guard_expect_fixed_in_file "$TAG" 'arrayrepr_facts' "$CONSUMER_TEST" "consumer tests must assert the explicit ArrayRepr fact"
guard_expect_fixed_in_file "$TAG" 'resolver.direct_array_i64_ids' "$CONSUMER_TEST" "consumer tests must keep the legacy origin-only negative case"

python3 -m unittest \
  "$ROOT_DIR/src/llvm_py/tests/test_direct_array_i64_constructor_lowering.py" \
  "$ROOT_DIR/src/llvm_py/tests/test_collection_method_call.py"

echo "[$TAG] ok"
