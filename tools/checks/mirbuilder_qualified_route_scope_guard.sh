#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-qualified-route-scope"
MAIN_ROOT="$ROOT_DIR/src/mir/builder/normal_callable_semantic_loan_port/main_root.rs"
PIN_TEST="$ROOT_DIR/src/mir/compiler/normal_default_pipeline_tests.rs"

command -v rg >/dev/null
command -v wc >/dev/null

# The canonical qualified-methods diversion must key on QualifiedUnbound
# receivers only (MIRBUILDER-EXE-ACCEPTANCE-QUALIFIED-PREFLIGHT-ROUTE-SCOPE-S0).
rg -q 'ResolvedMethodCallReceiverSourceV1::QualifiedUnbound' "$MAIN_ROOT"
rg -q 'method_calls\(\)\.any' "$MAIN_ROOT"
if rg -n 'method_calls\(\)\.next\(\)\.is_some\(\)' "$MAIN_ROOT"; then
  echo "[$TAG] diversion regressed to any-method-call predicate" >&2
  exit 1
fi

# MAIN-IMPORT-ARITY-SCOPE-S0: the relation issue and the route diversion are
# both arity-0-scoped. An arity-bearing app main can never consume a row.
MODEL="$ROOT_DIR/src/mir/normal_callable_semantic_package/model.rs"
ARITY_PIN_TEST="$ROOT_DIR/src/mir/compiler/normal_default_pipeline_arity_scope_tests.rs"
ENTRY_PORT="$ROOT_DIR/src/mir/builder/normal_callable_binding_materialization_port.rs"
rg -q 'caller\.arity\(\) != 0' "$MODEL"

# MAIN-WRAPPER-PARAM-ENTRY-S0: declared source parameters adopt the
# injector's published `variable_map` locals via StaticInjectedLocals, and
# the diversion gate checks the DECLARED arity (wrapper physical formals
# are structurally 0, so a physical-formal check would be vacuous).
rg -q 'StaticInjectedLocals' "$ENTRY_PORT"
rg -q 'callable-entry/injected-local-missing' "$ENTRY_PORT"
rg -q 'declared_parameter_names_v1' "$MAIN_ROOT"
rg -q 'if arity0' "$MAIN_ROOT"

# Lexical-receiver and arity regression pins must stay in place.
rg -q 'lexical_receiver_method_call_main_stays_off_qualified_route' "$PIN_TEST"
rg -q 'arity_bearing_main_with_qualified_call_stays_off_canonical_route' "$ARITY_PIN_TEST"
rg -q 'injected_locals_snapshot_follows_declared_name_order' "$ENTRY_PORT"
rg -q 'injected_locals_missing_name_fails_named_boundary' "$ENTRY_PORT"

# MIRBUILDER-EXE-ACCEPTANCE-ORDINARY-NEW-BIRTH-SITE-INDEX-S0: the
# destination-less birth-recipe index on OrdinaryNewClaimLedgerV1 admits
# verified `Birth` recipes for non-`[Body, Initializer]` `new` sites only;
# the raw physical owner consumes `constructor` dispositions, never a whole
# claim.
COSEAL="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_coseal.rs"
COSEAL_ISSUE="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_coseal_issue.rs"
COSEAL_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_coseal_tests.rs"
RAW_CLAIM="$ROOT_DIR/src/mir/builder/raw_ordinary_new_claim.rs"
NEW_EXPR="$ROOT_DIR/src/mir/builder/new_expression.rs"
CTOR_SCOPE="$ROOT_DIR/src/mir/builder/normal_instance_constructor_semantic_scope.rs"
rg -q 'birth_site_index' "$COSEAL"
rg -q 'fn take_birth_site_recipe' "$COSEAL"
rg -q 'fn try_take_ordinary_new_birth_recipe' "$RAW_CLAIM"
rg -q 'collect_birth_site_index_v1' "$COSEAL_ISSUE"
rg -q 'is_direct_local_initializer' "$COSEAL_ISSUE"
rg -q 'ordinary_birth_recipe' "$NEW_EXPR"
rg -q 'if self\.ordinary_claim\.is_none\(\)' "$NEW_EXPR"
rg -q 'ordinary_new_claim_ledger' "$CTOR_SCOPE"
rg -q 'birth_site_index_covers_field_assign_and_return_position_sites' "$COSEAL_TESTS"
rg -q 'birth_site_index_skips_builtin_and_missing_birth_classes' "$COSEAL_TESTS"
rg -q 'birth_site_take_enforces_class_arity_and_is_affine' "$COSEAL_TESTS"

# MIRBUILDER-EXE-ACCEPTANCE-LOOP-COND-NO-EXIT-S0: the no-exit extractor is a
# disjoint sibling of loop_cond_break_continue. It feeds the existing
# LoopCondBreakContinueFacts with NoExitBody; the canonical issuer, route
# match, and located-source lowerer stay the sole owner chain.
NO_EXIT="$ROOT_DIR/src/mir/builder/control_flow/plan/facts/loop_cond_no_exit_facts.rs"
LOOP_BUILDER="$ROOT_DIR/src/mir/builder/control_flow/plan/facts/loop_builder.rs"
LOOP_COND_FACTS="$ROOT_DIR/src/mir/builder/control_flow/facts/loop_cond_break_continue.rs"
REJECT_REASON="$ROOT_DIR/src/mir/builder/control_flow/plan/facts/reject_reason.rs"
LOOP_COND_BC="$ROOT_DIR/src/mir/builder/control_flow/plan/features/loop_cond_bc.rs"
BC_ITEM="$ROOT_DIR/src/mir/builder/control_flow/plan/loop_cond/break_continue_item.rs"
rg -q 'fn try_extract_loop_cond_no_exit_facts' "$NO_EXIT"
rg -q 'LoopCondBreakAcceptKind::NoExitBody' "$NO_EXIT"
rg -q 'continue_branches: Vec::new()' "$NO_EXIT"
rg -q 'body_exit_allowed: None' "$NO_EXIT"
rg -q 'BodyLoweringPolicy::RecipeOnly' "$NO_EXIT"
# Bounded vocabulary only: Stmt / ProgramBlock / GeneralIf.
rg -q 'LoopCondBreakContinueItem::ProgramBlock' "$NO_EXIT"
rg -q 'LoopCondBreakContinueItem::GeneralIf' "$NO_EXIT"
# Disjoint wiring: runs only when the exit-bearing extractor rejected.
rg -q 'None => try_extract_loop_cond_no_exit_facts' "$LOOP_BUILDER"
# No second issuer, no fallback to the legacy route.
if rg -n 'lower_loop_or_freeze_v1|LoopRouteContext' "$NO_EXIT"; then
  echo "[$TAG] no-exit extractor reached into a fallback route" >&2
  exit 1
fi
rg -q 'NoExitBody' "$LOOP_COND_FACTS"
rg -q 'ExitSignalPresent => "exit_signal_present"' "$REJECT_REASON"
rg -q 'fn for_loop_cond_no_exit' "$REJECT_REASON"
rg -q 'LoopCondBreakAcceptKind::NoExitBody => \(\)' "$LOOP_COND_BC"
rg -q 'fn build_loop_cond_break_continue_recipe' "$BC_ITEM"
rg -q 'pipeline_pins_no_exit_body_kind' "$NO_EXIT"
rg -q 'pipeline_keeps_exit_bearing_body_on_sibling' "$NO_EXIT"
rg -q 'rejects_conditional_update_if' "$NO_EXIT"

# MIRBUILDER-EXE-ACCEPTANCE-LOOP-COND-COND-UPDATE-S0: the located-source
# parts driver owns ConditionalUpdateIf. The arm seals the issued recipe
# against the located carriers (else parity keyed on body-or-exit presence,
# CondBlockView match, branch body/exit shape) and reuses the existing
# port-aware conditional-update/Select owner — no facade changes, no new
# recipe variant, no fallback arm.
ITEMS_SRC="$ROOT_DIR/src/mir/builder/control_flow/plan/parts/associated_source/callable_loop_source_items.rs"
COND_UPDATE_TESTS="$ROOT_DIR/src/mir/builder/control_flow/plan/parts/associated_source/callable_loop_source_items_cond_update_tests.rs"
COND_UPDATE_FACADE="$ROOT_DIR/src/mir/builder/control_flow/plan/parts/conditional_update.rs"
rg -q 'LoopCondBreakContinueItem::ConditionalUpdateIf' "$ITEMS_SRC"
rg -q 'else_body.is_some() || else_exit.is_some()' "$ITEMS_SRC"
rg -q 'require_condition_view_match\(cond_view' "$ITEMS_SRC"
rg -q 'fn verify_cond_update_branch_recipe' "$ITEMS_SRC"
rg -q 'try_lower_conditional_update_if_input' "$ITEMS_SRC"
rg -q 'loop-cond-item-conditional-update-unsupported' "$ITEMS_SRC"
rg -q 'fn try_lower_conditional_update_if_input' "$COND_UPDATE_FACADE"
rg -q 'source_item_lowers_conditional_update_if_through_located_branches' "$COND_UPDATE_TESTS"
rg -q 'source_item_lowers_conditional_update_if_with_tail_break' "$COND_UPDATE_TESTS"
rg -q 'source_item_lowers_conditional_update_if_with_tail_continue' "$COND_UPDATE_TESTS"
rg -q 'source_item_lowers_conditional_update_if_with_exit_only_else' "$COND_UPDATE_TESTS"
rg -q 'source_item_rejects_conditional_update_if_else_parity_drift' "$COND_UPDATE_TESTS"
rg -q 'source_item_rejects_conditional_update_if_exit_parity_drift' "$COND_UPDATE_TESTS"
rg -q 'source_item_rejects_conditional_update_if_unsupported_shape' "$COND_UPDATE_TESTS"

# MIRBUILDER-EXE-ACCEPTANCE-ME-RECEIVER-SITE-S0: located `Me`/`This`
# MethodCall receivers consume their registered `Receiver` site through the
# source port (`exact_source_receiver_value`); unregistered receivers keep
# the existing `lower_me_this_method_effect` path — site registration is
# the check, never a name fallback.
EXPR_PORT="$ROOT_DIR/src/mir/builder/control_flow/plan/expression_port.rs"
LOOP_SRC_PORT="$ROOT_DIR/src/mir/builder/normal_callable_loop_source_port.rs"
ASSOC_INPUT="$ROOT_DIR/src/mir/builder/control_flow/plan/normalizer/loop_body_lowering_associated_input.rs"
HELPERS_LOWER="$ROOT_DIR/src/mir/builder/control_flow/plan/normalizer/helpers_value/lower.rs"
ITEMS_TESTS="$ROOT_DIR/src/mir/builder/control_flow/plan/parts/associated_source/callable_loop_source_items_tests.rs"
TESTKIT="$ROOT_DIR/src/mir/builder/control_flow/plan/parts/associated_source/callable_loop_source_testkit.rs"
rg -q 'fn exact_source_receiver_value' "$EXPR_PORT"
rg -q 'fn exact_source_receiver_value' "$LOOP_SRC_PORT"
rg -q 'source_read_binding' "$LOOP_SRC_PORT"
rg -q 'exact_source_receiver_value' "$ASSOC_INPUT"
rg -q 'exact_source_receiver_value' "$HELPERS_LOWER"
rg -q 'fn cataloged_body_source' "$TESTKIT"
rg -q 'source_item_consumes_me_receiver_site_for_method_call' "$ITEMS_TESTS"
rg -q 'source_item_consumes_me_receiver_site_for_value_call' "$ITEMS_TESTS"
rg -q 'source_item_consumes_this_receiver_site_for_method_call' "$ITEMS_TESTS"
rg -q 'port_declines_receiver_value_for_non_receiver_expression' "$ITEMS_TESTS"
rg -q 'source_item_rejects_duplicate_me_receiver_site_consumption' "$ITEMS_TESTS"

# MIRBUILDER-EXE-ACCEPTANCE-INSTANCE-STATIC-INGRESS-S0: the static-result
# publication ingress admits a `Cataloged` non-StaticBoxMethod caller only
# when the probed (owner, method, arity) resolves through the declaration
# catalog (`declaration_for` -> StaticBoxMethod). The me-call probe keeps
# `"<source-owned>"` (never resolves -> DeclaredInstance sibling preserved)
# and Math/builtin owners keep `Unavailable` -> compatibility.
STATIC_INGRESS="$ROOT_DIR/src/mir/builder/static_result_publication_ingress.rs"
STATIC_OWNER_POLICY="$ROOT_DIR/src/mir/builder/method_call_handlers/static_current_owner_policy.rs"
rg -q 'declaration_for' "$STATIC_INGRESS"
rg -q 'SameModuleCallableNamespaceV1::StaticBoxMethod' "$STATIC_INGRESS"
rg -q 'caller\.namespace\(\) == SameModuleCallableNamespaceV1::StaticBoxMethod' "$STATIC_INGRESS"
rg -q '"<source-owned>"' "$STATIC_OWNER_POLICY"
rg -q 'fn instance_caller_is_admitted_for_declaration_resolved_static_target' "$STATIC_INGRESS"
rg -q 'fn instance_caller_declines_non_declaration_targets' "$STATIC_INGRESS"
rg -q 'fn me_call_probe_keeps_instance_callers_outside_static_ingress' "$STATIC_INGRESS"
rg -q 'fn instance_caller_declines_when_declarations_are_unavailable' "$STATIC_INGRESS"

for file in "$MAIN_ROOT" "$PIN_TEST" "$ARITY_PIN_TEST" "$ENTRY_PORT" "$COSEAL" "$COSEAL_ISSUE" "$COSEAL_TESTS" "$RAW_CLAIM" "$NEW_EXPR" "$CTOR_SCOPE" "$NO_EXIT" "$LOOP_BUILDER" "$LOOP_COND_FACTS" "$REJECT_REASON" "$LOOP_COND_BC" "$BC_ITEM" "$ITEMS_SRC" "$COND_UPDATE_TESTS" "$COND_UPDATE_FACADE" "$HELPERS_LOWER" "$ITEMS_TESTS" "$TESTKIT" "$STATIC_INGRESS" "$STATIC_OWNER_POLICY"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    echo "[$TAG] source reached hard 800-line boundary: ${file#"$ROOT_DIR/"}=$lines" >&2
    exit 1
  fi
done

echo "[$TAG] ok"
