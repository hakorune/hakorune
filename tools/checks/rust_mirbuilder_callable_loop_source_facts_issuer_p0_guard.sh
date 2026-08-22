#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-callable-loop-source-facts-issuer-p0"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

ISSUER="$ROOT_DIR/src/mir/builder/normal_callable_loop_source_facts.rs"
TESTS="$ROOT_DIR/src/mir/builder/normal_callable_loop_source_facts_tests.rs"
RAW_ENTRY="$ROOT_DIR/src/mir/builder/raw_loop_child_entry.rs"
FACTS_BUILDER="$ROOT_DIR/src/mir/builder/control_flow/plan/facts/loop_builder.rs"
V0="$ROOT_DIR/src/mir/builder/control_flow/plan/generic_loop/facts/extract/v0.rs"
VALIDATION="$ROOT_DIR/src/mir/builder/control_flow/plan/generic_loop/body_check/validation_v1.rs"
PLANNER="$ROOT_DIR/src/mir/builder/control_flow/plan/single_planner/rules.rs"
PLANNER_MOD="$ROOT_DIR/src/mir/builder/control_flow/plan/single_planner/mod.rs"
PLANNER_INPUT="$ROOT_DIR/src/mir/builder/control_flow/plan/single_planner/input.rs"
STRUCTURAL_PORT="$ROOT_DIR/src/mir/builder/control_flow/joinir/structural_port.rs"
STRUCTURAL_PORT_TESTS="$ROOT_DIR/src/mir/builder/control_flow/joinir/structural_port_tests.rs"
STRUCTURAL_LEASE_TESTS="$ROOT_DIR/src/mir/builder/normal_callable_loop_structural_lease_tests.rs"
GENERIC_LOOP_CONTEXT="$ROOT_DIR/src/mir/builder/control_flow/plan/features/generic_loop_context.rs"
GENERIC_LOOP_PIPELINE="$ROOT_DIR/src/mir/builder/control_flow/plan/features/generic_loop_pipeline.rs"
GENERIC_LOOP_V1="$ROOT_DIR/src/mir/builder/control_flow/plan/features/generic_loop_body/v1.rs"
LOWERING_CONTEXT="$ROOT_DIR/src/mir/builder/control_flow/plan/lowering_context.rs"
COMPOSER="$ROOT_DIR/src/mir/builder/control_flow/plan/recipe_tree/generic_loop_composer.rs"
ADAPTER="$ROOT_DIR/src/mir/builder/normal_callable_loop_physical_adapter.rs"
FEATURES_README="$ROOT_DIR/src/mir/builder/control_flow/plan/features/README.md"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mirbuilder-callable-loop-ready-generic-loop-v1-recipe-authority-d0-2026-08-22.md"
README="$ROOT_DIR/src/mir/builder/README.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/rust_mirbuilder_callable_loop_source_facts_issuer_p0_guard.sh"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$ISSUER" "$TESTS" "$RAW_ENTRY" "$FACTS_BUILDER" \
  "$V0" "$VALIDATION" "$PLANNER" "$PLANNER_MOD" "$PLANNER_INPUT" \
  "$STRUCTURAL_PORT" "$STRUCTURAL_PORT_TESTS" "$STRUCTURAL_LEASE_TESTS" \
  "$GENERIC_LOOP_CONTEXT" "$GENERIC_LOOP_PIPELINE" "$GENERIC_LOOP_V1" \
  "$LOWERING_CONTEXT" "$COMPOSER" "$ADAPTER" \
  "$FEATURES_README" \
  "$CARD" "$README" "$INDEX"

guard_expect_fixed_in_file "$TAG" "CallableGenericLoopSourceFactsIssuerV1" "$ISSUER" \
  "source-aware issuer must remain the named owner"
guard_expect_fixed_in_file "$TAG" "CallableGenericLoopSourceFactsReceiptV1" "$ISSUER" \
  "source-facts claim must retain one private receipt"
guard_expect_fixed_in_file "$TAG" "claim_all" "$ISSUER" \
  "source-facts product must have one one-shot claim operation"
guard_expect_fixed_in_file "$TAG" "PreparedCallableGenericLoopSourceFactsPayloadV1" "$RAW_ENTRY" \
  "raw prepared entry must assemble the opaque move payload"
guard_expect_fixed_in_file "$TAG" "try_build_source_outcome" "$ISSUER" \
  "issuer must use the route-neutral explicit-policy planner seam"
guard_expect_fixed_in_file "$TAG" "CallableLoopFactsPlannerInputV1" "$PLANNER_INPUT" \
  "planner input must be the named route-neutral boundary"
guard_expect_fixed_in_file "$TAG" "CallableLoopStructuralPortV1" "$STRUCTURAL_PORT" \
  "structural handoff must use one opaque port"
guard_expect_fixed_in_file "$TAG" "for<'view> FnOnce" "$STRUCTURAL_PORT" \
  "structural port must be callback-scoped by HRTB"
guard_expect_fixed_in_file "$TAG" "CallableLoopRouteNeutralStructuralSeedV1" "$STRUCTURAL_PORT" \
  "route-neutral structural seed must be owned by the structural module"
guard_expect_fixed_in_file "$TAG" "CallableLoopSourceBoundStructuralPortV1" "$STRUCTURAL_PORT" \
  "source-bound structural port must be opaque and seed-bound"
guard_expect_fixed_in_file "$TAG" "CallableLoopStructuralLeaseIssuerV1" "$ISSUER" \
  "source receipt must have one named structural lease issuer"
guard_expect_fixed_in_file "$TAG" "PreparedCallableLoopStructuralHandoffV1" "$ISSUER" \
  "receipt and structural seed must be co-sealed"
guard_expect_fixed_in_file "$TAG" "CallableLoopReadyStructuralViewV1" "$ISSUER" \
  "ready view must be the named HRTB consumer boundary"
guard_expect_fixed_in_file "$TAG" "GenericLoopV1LoweringContext" "$GENERIC_LOOP_CONTEXT" \
  "GenericLoopV1 must consume a narrow route-neutral context seam"
guard_expect_fixed_in_file "$TAG" "GenericLoopV1SourceLoweringContextV1" "$GENERIC_LOOP_CONTEXT" \
  "source-backed GenericLoopV1 must have a route-neutral context"
guard_expect_fixed_in_file "$TAG" "PlanLoweringContext" "$LOWERING_CONTEXT" \
  "CorePlan lowering must consume a route-neutral diagnostic context"
guard_expect_fixed_in_file "$TAG" "compose_source_generic_loop_v1_recipe" "$COMPOSER" \
  "source-backed lowering must use the route-neutral composer entry"
guard_expect_fixed_in_file "$TAG" "CallableGenericLoopV1SemanticRecipeV1" "$ISSUER" \
  "Ready must move into one semantic Recipe owner"
guard_expect_fixed_in_file "$TAG" "into_semantic_recipe" "$ISSUER" \
  "claimed source Facts must have one semantic Recipe transition"
guard_expect_fixed_in_file "$TAG" "CallableGenericLoopV1PhysicalAdapterV1" "$ADAPTER" \
  "semantic Recipe must have one named physical consumer"
guard_expect_fixed_in_file "$TAG" "with_view" "$ADAPTER" \
  "physical consumer must use the HRTB semantic view"
guard_expect_fixed_in_file "$TAG" "CallableGenericLoopV1PhysicalAdapterV1::lower" "$RAW_ENTRY" \
  "Ready must connect to the named physical adapter"
guard_expect_fixed_in_file "$TAG" "into_semantic_recipe" "$RAW_ENTRY" \
  "Ready must not return to the old lowerer"
guard_expect_fixed_in_file "$TAG" "claim_all()" "$RAW_ENTRY" \
  "Ready must claim the source product exactly once"
guard_expect_fixed_in_file "$TAG" "None => lower_non_callable_loop_legacy_v1" "$RAW_ENTRY" \
  "legacy JoinIR must be named as the non-callable lane"
guard_expect_fixed_in_file "$TAG" "fn lower_non_callable_loop_legacy_v1" "$RAW_ENTRY" \
  "old JoinIR entry must remain outside the Ready branch"
guard_expect_fixed_in_file "$TAG" "&dyn GenericLoopV1LoweringContext" "$GENERIC_LOOP_PIPELINE" \
  "GenericLoopV1 pipeline must accept the narrow context seam"
guard_expect_fixed_in_file "$TAG" "&dyn GenericLoopV1LoweringContext" "$GENERIC_LOOP_V1" \
  "GenericLoopV1 body must accept the narrow context seam"
guard_expect_fixed_in_file "$TAG" "route-neutral context" "$FEATURES_README" \
  "features README must document the route-neutral context boundary"
guard_expect_fixed_in_file "$TAG" "CallableLoopStructuralLeaseIssuerV1::prepare" "$STRUCTURAL_LEASE_TESTS" \
  "focused evidence must consume the source-bound lease"
guard_expect_fixed_in_file "$TAG" "with_existing_structural_port" "$STRUCTURAL_PORT_TESTS" \
  "focused evidence must exercise the structural lease"
guard_expect_fixed_in_file "$TAG" "try_build_source_outcome" "$PLANNER_MOD" \
  "single planner must expose the route-neutral source entry"
guard_expect_fixed_in_file "$TAG" "try_build_outcome_with_policy_parts" "$PLANNER" \
  "Context and source callers must share one planner kernel"
guard_expect_fixed_in_file "$TAG" "try_extract_generic_loop_v0_facts_with_policy" "$FACTS_BUILDER" \
  "Facts builder must pass the captured policy into V0"
guard_expect_fixed_in_file "$TAG" "check_body_generic_v1_with_policy" "$V0" \
  "V0 must pass the captured policy into body validation"
guard_expect_fixed_in_file "$TAG" "GenericLoopFactsPolicyFrameV1" "$VALIDATION" \
  "body validation must have an explicit-policy seam"
guard_expect_fixed_in_file "$TAG" "CallableGenericLoopSourceFactsRouteErrorV1" "$ISSUER" \
  "route rejection must preserve a typed reason"
guard_expect_fixed_in_file "$TAG" "Ready production caller switch = 1" "$CARD" \
  "active card must record the Ready production switch"
guard_expect_fixed_in_file "$TAG" "MIR-CALLABLE-LOOP-READY-GENERIC-LOOP-V1-RECIPE-AUTHORITY-D0" "$CARD" \
  "active card must name the accepted Recipe authority"
guard_expect_fixed_in_file "$TAG" "Callable Loop source-aware Facts issuer I0" "$README" \
  "builder README must document the production Ready seam"
guard_expect_fixed_in_file "$TAG" "CallableLoopStructuralPortV1" "$README" \
  "builder README must document the callback-scoped structural lease"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" \
  "check index must list the reusable issuer guard"

if rg -n -- 'crate::config::env|from_environment\(' "$ISSUER"; then
  guard_fail "$TAG" "source-aware issuer must not reread ambient policy"
fi
if rg -n \
  --glob '!normal_callable_loop_structural_lease_tests.rs' \
  --glob '!normal_callable_loop_source_facts_tests.rs' \
  -- 'CallableLoopStructuralLeaseIssuerV1::prepare' \
  "$ROOT_DIR/src/mir/builder"; then
  guard_fail "$TAG" "structural lease issuer still has a production caller"
fi
if rg -n -- 'LoopRouteContext|choose_route_kind' "$ISSUER"; then
  guard_fail "$TAG" "source-aware issuer must not construct or classify a route context"
fi
if rg -n -- 'LoopRouteContext::new|choose_route_kind|route_loop' "$GENERIC_LOOP_CONTEXT"; then
  guard_fail "$TAG" "route-neutral context must not construct or classify a route"
fi
if rg -n -- 'LoopRouteContext::new|choose_route_kind|route_loop' "$ADAPTER"; then
  guard_fail "$TAG" "source-aware physical adapter must not reconstruct route authority"
fi
if ! rg -n -- 'fn legacy_route_context\(&self\) -> Option' "$GENERIC_LOOP_CONTEXT" >/dev/null; then
  guard_fail "$TAG" "context seam must expose an explicit legacy nested capability"
fi
if ! rg -n -A20 -- 'impl GenericLoopV1LoweringContext for GenericLoopV1SourceLoweringContextV1' "$GENERIC_LOOP_CONTEXT" \
  | rg -- 'None' >/dev/null; then
  guard_fail "$TAG" "source context must not expose legacy nested route capability"
fi
if rg -n -- 'CallableGenericLoopSourceFactsReadyV1|_pre_effect_receipt' "$ISSUER"; then
  guard_fail "$TAG" "source-facts claim must be in-place and retain the receipt"
fi
if rg -n -- 'route_kind|LoopRouteKind|Facts|Recipe|registry|PlanLowerer|ValueId|ASTNode|Deref|route_loop|lower_loop_or_freeze_v1' "$STRUCTURAL_PORT"; then
  guard_fail "$TAG" "structural port must not grow semantic, route, AST, or physical authority"
fi
if rg -n -- 'with_existing_structural_port|route_loop|lower_loop_or_freeze_v1|PlanLowerer' "$ISSUER"; then
  guard_fail "$TAG" "source-aware issuer must not connect the structural lease to the old route"
fi
if rg -n -- 'LoopRouteContext|LoopRouteKind|in_static_box|ValueId|registry' "$PLANNER_INPUT"; then
  guard_fail "$TAG" "route-neutral planner input must not carry structural or physical authority"
fi
if rg -n -- 'try_extract_generic_loop_v0_facts\(' "$FACTS_BUILDER"; then
  guard_fail "$TAG" "explicit Facts builder still calls the ambient V0 facade"
fi
if rg -n -- 'check_body_generic_v1\(' "$V0"; then
  guard_fail "$TAG" "explicit V0 path still calls the ambient body-validation facade"
fi

planner_calls="$(rg -F -o -- 'try_build_source_outcome(' "$ISSUER" | wc -l | tr -d '[:space:]')"
if [[ "$planner_calls" -ne 1 ]]; then
  guard_fail "$TAG" "issuer must have exactly one route-neutral planner call; found $planner_calls"
fi
claim_calls="$(rg -F -o -- 'claim_all()' "$TESTS" | wc -l | tr -d '[:space:]')"
if [[ "$claim_calls" -ne 2 ]]; then
  guard_fail "$TAG" "focused evidence must cover two one-shot source-facts claims; found $claim_calls"
fi
lease_definitions="$(rg -F -o -- 'with_existing_structural_port<R>' "$STRUCTURAL_PORT" | wc -l | tr -d '[:space:]')"
lease_tests="$(rg -F -o -- 'with_existing_structural_port(&context' "$STRUCTURAL_PORT_TESTS" | wc -l | tr -d '[:space:]')"
if [[ "$lease_definitions" -ne 1 || "$lease_tests" -ne 1 ]]; then
  guard_fail "$TAG" "structural lease must have one definition and one focused caller; definitions=$lease_definitions tests=$lease_tests"
fi
seed_definitions="$(rg -F -o -- 'issue_route_neutral_structural_seed(' "$STRUCTURAL_PORT" | wc -l | tr -d '[:space:]')"
if [[ "$seed_definitions" -ne 1 ]]; then
  guard_fail "$TAG" "route-neutral seed must have exactly one issuer definition; found $seed_definitions"
fi
lease_test_cases="$(rg -F -o -- 'CallableLoopStructuralLeaseIssuerV1::prepare' "$STRUCTURAL_LEASE_TESTS" | wc -l | tr -d '[:space:]')"
if [[ "$lease_test_cases" -ne 3 ]]; then
  guard_fail "$TAG" "structural lease focused evidence must cover three cases; found $lease_test_cases"
fi
payload_literals="$(rg -F -o -- 'Ok(PreparedCallableGenericLoopSourceFactsPayloadV1 {' "$RAW_ENTRY" | wc -l | tr -d '[:space:]')"
if [[ "$payload_literals" -ne 1 ]]; then
  guard_fail "$TAG" "prepared payload must have exactly one constructor literal; found $payload_literals"
fi

if rg -n --glob '!normal_callable_loop_source_facts.rs' \
  --glob '!normal_callable_loop_source_facts_tests.rs' \
  --glob '!normal_callable_loop_structural_lease_tests.rs' \
  --glob '!raw_loop_child_entry.rs' \
  -- 'CallableGenericLoopSourceFactsIssuerV1::issue_once' \
  "$ROOT_DIR/src/mir/builder"; then
  guard_fail "$TAG" "Ready issuer has an unexpected production caller"
fi
ready_calls="$(rg -F -o -- 'CallableGenericLoopSourceFactsIssuerV1::issue_once(payload)' "$RAW_ENTRY" | wc -l | tr -d '[:space:]')"
if [[ "$ready_calls" -ne 1 ]]; then
  guard_fail "$TAG" "Ready issuer must have exactly one production caller; found $ready_calls"
fi
if rg -n -- 'from_prepared_parts|CallableGenericLoopSourceFactsInputV1|ParserInvocationWitness' \
  "$ISSUER" "$RAW_ENTRY"; then
  guard_fail "$TAG" "loose input/reconstructed parser identity leaked into P0"
fi
if rg -n -- 'lower_loop_or_freeze_v1|RouteExecutionWitnessV1|PostEffectRetryDebt|ValueId' "$ISSUER"; then
  guard_fail "$TAG" "source issuer grew a lowering/physical/fallback authority"
fi
if rg -n -- 'PreparedCallableGenericLoopSourceFactsPayloadV1::into_parts|fn into_parts' "$RAW_ENTRY"; then
  guard_fail "$TAG" "source-facts payload must not expose an 11-element tuple escape"
fi

for file in "$ISSUER" "$TESTS" "$RAW_ENTRY" "$V0" "$VALIDATION" "$PLANNER" "$STRUCTURAL_PORT" "$STRUCTURAL_PORT_TESTS" "$STRUCTURAL_LEASE_TESTS" "$GENERIC_LOOP_CONTEXT" "$GENERIC_LOOP_PIPELINE" "$GENERIC_LOOP_V1"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "P0 source reached the 800-line hard boundary: ${file#"$ROOT_DIR/"}=$lines"
  fi
done
for file in "$PLANNER_INPUT" "$PLANNER_MOD"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "route-neutral planner source reached the 800-line hard boundary: ${file#"$ROOT_DIR/"}=$lines"
  fi
done

echo "[$TAG] ok (one route-neutral explicit-policy source/Facts issuer, one semantic Recipe/physical consumer, production Ready caller=1)"
