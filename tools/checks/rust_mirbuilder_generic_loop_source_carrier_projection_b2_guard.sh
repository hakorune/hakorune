#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-generic-loop-source-carrier-projection-b2"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

PORT="$ROOT_DIR/src/mir/builder/normal_callable_loop_source_port.rs"
ADAPTER="$ROOT_DIR/src/mir/builder/normal_callable_loop_physical_adapter.rs"
STATE="$ROOT_DIR/src/mir/builder/normal_callable_semantic_lowering_state.rs"
RELATION="$ROOT_DIR/src/mir/builder/normal_callable_loop_source_facts/generic/carrier_relation.rs"
RELATION_TESTS="$ROOT_DIR/src/mir/builder/normal_callable_loop_source_facts/generic/carrier_relation_tests.rs"
STATE_TESTS="$ROOT_DIR/src/mir/builder/normal_callable_semantic_lowering_state/map_local_tests.rs"
STATE_NAMED_TESTS="$ROOT_DIR/src/mir/builder/normal_callable_semantic_lowering_state/named_array_tests.rs"
TARGET_TESTS="$ROOT_DIR/src/mir/source_call_target/named_array_method_tests.rs"
EMISSION_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/named_array_emission_tests.rs"
PUBLISHED_TESTS="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/named_array_source_tests.rs"
TRAIT="$ROOT_DIR/src/mir/builder/control_flow/plan/expression_port.rs"
NORMALIZER="$ROOT_DIR/src/mir/builder/control_flow/plan/normalizer/helpers_value/lower.rs"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mir-call-parser-array-push-b2-i0-2026-09-23.md"
README="$ROOT_DIR/src/mir/builder/README.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/rust_mirbuilder_generic_loop_source_carrier_projection_b2_guard.sh"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$PORT" "$ADAPTER" "$STATE" "$RELATION" "$RELATION_TESTS" \
  "$STATE_TESTS" "$STATE_NAMED_TESTS" "$TARGET_TESTS" "$EMISSION_TESTS" "$PUBLISHED_TESTS" "$TRAIT" \
  "$NORMALIZER" "$CARD" "$README" "$INDEX"

guard_expect_fixed_in_file "$TAG" "CallableLoopSourceExpressionPortWithRelationV1" "$PORT" \
  "the source port must retain the relation-aware type"
guard_expect_fixed_in_file "$TAG" "with_carrier_bindings" "$PORT" \
  "carrier bindings must enter through the existing relation"
guard_expect_fixed_in_file "$TAG" "source_read_binding" "$PORT" \
  "source reads must consume resolver BindingRef identity"
guard_expect_fixed_in_file "$TAG" "physical_value_for_binding" "$PORT" \
  "physical projection must use the relation, not a name lookup"
guard_expect_fixed_in_file "$TAG" "take_source_array_push" "$PORT" \
  "ArrayPush must be consumed from the exact source row"
guard_expect_fixed_in_file "$TAG" "source-evidence-session-mismatch" "$ADAPTER" \
  "the adapter must reject a mismatched source evidence session"
guard_expect_fixed_in_file "$TAG" "publish_source_loop_final_value" "$ADAPTER" \
  "final induction publication must use the existing callable ledger"
guard_expect_fixed_in_file "$TAG" "physical_bindings" "$TRAIT" \
  "the plan port must receive the existing physical binding map"
guard_expect_fixed_in_file "$TAG" "phi_bindings" "$NORMALIZER" \
  "the normalizer must forward its physical binding map"
guard_expect_fixed_in_file "$TAG" "source-carrier-physical-missing" "$RELATION" \
  "missing physical carrier labels must fail closed"
guard_expect_fixed_in_file "$TAG" "source_carrier_projection_rejects_missing_physical_label" "$RELATION_TESTS" \
  "missing physical carrier evidence must be executable"
guard_expect_fixed_in_file "$TAG" "source_carrier_projection_keeps_non_carrier_binding_unmapped" "$RELATION_TESTS" \
  "a non-carrier BindingRef must not acquire a physical value"
guard_expect_fixed_in_file "$TAG" "named_array_source_reaches_retained_typed_write_and_c_frame" "$PUBLISHED_TESTS" \
  "the positive source-to-physical ArrayPush path must remain covered"
guard_expect_fixed_in_file "$TAG" "named_array_value_demand_rejects_before_published_consumer" "$PUBLISHED_TESTS" \
  "value-demanded ArrayPush must remain a terminal negative"
guard_expect_fixed_in_file "$TAG" "physical_adapter_rejects_relation_owner_mismatch_before_builder_effect" \
  "$ROOT_DIR/src/mir/builder/normal_callable_loop_source_facts_tests.rs" \
  "foreign source relation ownership must remain rejected"
guard_expect_fixed_in_file "$TAG" "physical_adapter_rejects_source_evidence_session_drift_before_builder_effect" \
  "$ROOT_DIR/src/mir/builder/normal_callable_loop_source_facts_tests.rs" \
  "source evidence session drift must remain rejected before physical effects"
guard_expect_fixed_in_file "$TAG" "source_core_method_take_rejects_duplicate_exact_site" "$STATE_TESTS" \
  "the existing exact-site owner must retain duplicate/missing source-row evidence"
guard_expect_fixed_in_file "$TAG" "named_array_state_rejects_call_shape_drift" "$STATE_NAMED_TESTS" \
  "the named ArrayPush state must reject call-shape drift"
guard_expect_fixed_in_file "$TAG" "named_array_state_rejects_duplicate_exact_site_consumption" "$STATE_NAMED_TESTS" \
  "the named ArrayPush state must reject duplicate exact-site consumption"
guard_expect_fixed_in_file "$TAG" "named_array_state_finish_rejects_missing_source_row" "$STATE_NAMED_TESTS" \
  "the required caller must reject an unconsumed named source row"
guard_expect_fixed_in_file "$TAG" "named_array_state_finish_rejects_write_without_physical_emission" "$STATE_NAMED_TESTS" \
  "the named ArrayPush state must reject a residual physical write"
guard_expect_fixed_in_file "$TAG" "selected_array_contract_rejects_reassignment_value_demand_and_non_text" "$TARGET_TESTS" \
  "the source target owner must retain receiver/value/text negative evidence"
guard_expect_fixed_in_file "$TAG" "package_inventory_rejects_dropped_draft_and_allocation_without_write" "$EMISSION_TESTS" \
  "the existing emission owner must retain residual completion evidence"
guard_expect_fixed_in_file "$TAG" "MIR-CALL-PARSER-ARRAY-PUSH-B2-I0" "$CARD" \
  "the active card must name this bounded slice"
guard_expect_fixed_in_file "$TAG" "one physical port" "$CARD" \
  "the card must keep the single physical-port authority"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" \
  "the check index must list this reusable guard"

for file in "$PORT" "$ADAPTER" "$STATE" "$RELATION" "$TRAIT" "$NORMALIZER"; do
    lines="$(wc -l <"$file" | tr -d '[:space:]')"
    if [[ "$lines" -ge 800 ]]; then
        guard_fail "$TAG" "source file reached the 800-line hard stop: ${file#$ROOT_DIR/} (${lines})"
    fi
done

printf '[%s] PASS: relation-aware carrier projection and retained ArrayPush seam are pinned\n' "$TAG"
