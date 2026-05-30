#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="k2-wide-phase296x-direct-i64-arrayrepr-producer-implementation"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_383="docs/development/current/main/phases/phase-296x/296x-383-DIRECTI64-ARRAYREPR-PRODUCER-CONTRACT.md"
CARD_384="docs/development/current/main/phases/phase-296x/296x-384-DIRECTI64-ARRAYREPR-PRODUCER-IMPLEMENTATION.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
RESOLVER_HELPERS="src/llvm_py/utils/resolver_helpers.py"
NEWBOX="src/llvm_py/instructions/newbox.py"
CONSTRUCTOR="src/llvm_py/instructions/mir_call/constructor_call.py"
CONSUMER="src/llvm_py/instructions/mir_call/collection_method_call.py"
TEST="src/llvm_py/tests/test_direct_array_i64_constructor_lowering.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_direct_i64_arrayrepr_producer_implementation_guard.sh"

echo "[$TAG] checking DirectI64 ArrayRepr producer implementation"

guard_require_files "$TAG" "$CARD_383" "$CARD_384" "$STATE" "$INDEX" "$RESOLVER_HELPERS" "$NEWBOX" "$CONSTRUCTOR" "$CONSUMER" "$TEST" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_383" "row383 producer contract must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_384" "row384 producer implementation must be landed"
guard_expect_fixed_in_file "$TAG" 'output_contract=direct-i64-arrayrepr-producer-implementation-v0' "$CARD_384" "row384 must define the implementation output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=direct-i64-arrayrepr-producer-contract-v0' "$CARD_384" "row384 must consume row383"
guard_expect_fixed_in_file "$TAG" 'selected_boundary=direct_i64_arrayrepr_producer_implementation' "$CARD_384" "row384 must stay on the producer implementation boundary"
guard_expect_fixed_in_file "$TAG" 'selected_next=direct_i64_arrayrepr_lowering_consumer_rebase' "$CARD_384" "row384 must point to the lowering consumer rebase"
guard_expect_fixed_in_file "$TAG" 'producer_fact_name=ArrayRepr::DirectI64' "$CARD_384" "row384 must name the explicit ArrayRepr fact"
guard_expect_fixed_in_file "$TAG" 'producer_fact_owner=representation_planner' "$CARD_384" "row384 must assign fact ownership"
guard_expect_fixed_in_file "$TAG" 'producer_fact_store=resolver.arrayrepr_facts' "$CARD_384" "row384 must define the fact store"
guard_expect_fixed_in_file "$TAG" 'producer_origin_fact_compat=resolver.direct_array_i64_ids' "$CARD_384" "row384 must keep compatibility with the origin fact"
guard_expect_fixed_in_file "$TAG" 'producer_fact_recorded_for_direct_birth=1' "$CARD_384" "row384 must record the fact for direct birth"
guard_expect_fixed_in_file "$TAG" 'public_arraybox_birth_unchanged=1' "$CARD_384" "row384 must keep public birth unchanged"
guard_expect_fixed_in_file "$TAG" 'silent_fallback_allowed=0' "$CARD_384" "row384 must forbid silent fallback"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-384-DIRECTI64-ARRAYREPR-PRODUCER-IMPLEMENTATION"' "$STATE" "current state must keep row384 as the latest landed card"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "DIRECTI64-ARRAYREPR-LOWERING-CONSUMER-REBASE-296X-001"' "$STATE" "current state must point to the consumer rebase"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" 'mark_arrayrepr_direct_i64' "$RESOLVER_HELPERS" "resolver helpers must expose the explicit ArrayRepr fact marker"
guard_expect_fixed_in_file "$TAG" 'arrayrepr_facts' "$RESOLVER_HELPERS" "resolver helpers must expose the ArrayRepr fact store"
guard_expect_fixed_in_file "$TAG" 'mark_arrayrepr_direct_i64' "$NEWBOX" "newbox must record the explicit ArrayRepr fact"
guard_expect_fixed_in_file "$TAG" 'mark_arrayrepr_direct_i64' "$CONSTRUCTOR" "constructor lowering must record the explicit ArrayRepr fact"
guard_expect_fixed_in_file "$TAG" 'arrayrepr_facts' "$TEST" "constructor tests must assert the explicit ArrayRepr fact"

PYTHONPATH="$ROOT_DIR/src/llvm_py:$ROOT_DIR" \
  python3 -m unittest "$ROOT_DIR/src/llvm_py/tests/test_direct_array_i64_constructor_lowering.py"

echo "[$TAG] ok"
