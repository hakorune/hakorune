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
COSEAL_ISSUE_SOURCE="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_coseal_issue_source.rs"
SCALAR_EXPR="$ROOT_DIR/src/mir/resolved_semantics/home_new_prefix_scalar_expression.rs"
LOCAL_FIELD_SRC="$ROOT_DIR/src/mir/resolved_semantics/home_new_prefix_field_read.rs"
SCALAR_CLAIM_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/scalar_expression_claim_tests.rs"
SCALAR_EMIT_TESTS="$ROOT_DIR/src/mir/builder/normal_default_root_catalog_scalar_expression_tests.rs"
FIELD_BATCH_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_field_batch_tests.rs"
COSEAL_LEDGER="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_ledger.rs"
COSEAL_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_coseal_tests.rs"
RAW_CLAIM="$ROOT_DIR/src/mir/builder/raw_ordinary_new_claim.rs"
NEW_EXPR="$ROOT_DIR/src/mir/builder/new_expression.rs"
CTOR_SCOPE="$ROOT_DIR/src/mir/builder/normal_instance_constructor_semantic_scope.rs"
rg -q 'birth_site_index' "$COSEAL"
rg -q 'fn take_birth_site_recipe' "$COSEAL_LEDGER"
rg -q 'fn try_take_ordinary_new_birth_recipe' "$RAW_CLAIM"
rg -q 'collect_birth_site_index_v1' "$COSEAL_ISSUE"
rg -q 'fn collect_birth_site_index_v1' "$COSEAL_ISSUE_SOURCE"
rg -q 'fn prove_local_field_read_batch' "$COSEAL_ISSUE_SOURCE"
rg -q 'fn stage_local_field_read_batch' "$COSEAL_ISSUE_SOURCE"
rg -q 'fn observe_scalar_expression' "$SCALAR_EXPR"
rg -q 'scalar_expression_rejects_whole_initializer_without_partial_rows' "$SCALAR_CLAIM_TESTS"
rg -q 'scalar_expression_short_circuit_rhs_read_stays_in_deferred_block' "$SCALAR_EMIT_TESTS"
rg -q 'prepare_local_candidates_by_slot_v1' "$COSEAL_ISSUE"
rg -q 'collect_local_candidates_v1' "$COSEAL_ISSUE_SOURCE"
rg -q 'is_direct_local_initializer' "$COSEAL_ISSUE_SOURCE"
rg -q 'ordinary_birth_recipe' "$NEW_EXPR"
rg -q 'if self\.ordinary_claim\.is_none\(\)' "$NEW_EXPR"
rg -q 'ordinary_new_claim_ledger' "$CTOR_SCOPE"
rg -q 'birth_site_index_covers_field_assign_sites_while_return_position_claims' "$COSEAL_TESTS"
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
# S10: the extractor defers the VariableAccumRecurrence family (exact
# two-assignment accumulator-over-induction recurrence) so LCBC never
# mints a competing fact for the VAR-owned loop.
rg -q 'fn claims_variable_accum_family' "$NO_EXIT"
rg -q 'RejectReason::VariableAccumFamily' "$NO_EXIT"
rg -q 'VariableAccumFamily => "variable_accum_family"' "$REJECT_REASON"
rg -q 'defers_variable_accum_recurrence_family' "$NO_EXIT"
rg -q 'pipeline_defers_variable_accum_family' "$NO_EXIT"

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
NORMALIZER_COMMON="$ROOT_DIR/src/mir/builder/control_flow/plan/normalizer/common.rs"
ITEMS_TESTS="$ROOT_DIR/src/mir/builder/control_flow/plan/parts/associated_source/callable_loop_source_items_tests.rs"
TESTKIT="$ROOT_DIR/src/mir/builder/control_flow/plan/parts/associated_source/callable_loop_source_testkit.rs"
rg -q 'fn exact_source_receiver_value' "$EXPR_PORT"
rg -q 'fn exact_source_receiver_value' "$LOOP_SRC_PORT"
rg -q 'source_read_binding' "$LOOP_SRC_PORT"
rg -q 'me_this_method_call_effect' "$ASSOC_INPUT"
rg -q 'exact_source_receiver_value' "$NORMALIZER_COMMON"
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
STATIC_INGRESS_TESTS="$ROOT_DIR/src/mir/builder/static_result_publication_ingress_tests.rs"
STATIC_OWNER_POLICY="$ROOT_DIR/src/mir/builder/method_call_handlers/static_current_owner_policy.rs"
rg -q 'declaration_for' "$STATIC_INGRESS"
rg -q 'SameModuleCallableNamespaceV1::StaticBoxMethod' "$STATIC_INGRESS"
rg -q 'caller\.namespace\(\) == SameModuleCallableNamespaceV1::StaticBoxMethod' "$STATIC_INGRESS"
rg -q '"<source-owned>"' "$STATIC_OWNER_POLICY"
rg -q 'fn instance_caller_is_admitted_for_declaration_resolved_static_target' "$STATIC_INGRESS_TESTS"
rg -q 'fn instance_caller_declines_non_declaration_targets' "$STATIC_INGRESS_TESTS"
rg -q 'fn me_call_probe_keeps_instance_callers_outside_static_ingress' "$STATIC_INGRESS_TESTS"
rg -q 'fn instance_caller_declines_when_declarations_are_unavailable' "$STATIC_INGRESS_TESTS"

# MIRBUILDER-EXE-ACCEPTANCE-TARGET-ONLY-EMISSION-S0: the member_route
# `StaticReceiver` arm consumes `StaticResultPublicationIngressV1::TargetOnly`
# through the existing `lower_target_only_static_result_publication_v1`
# physical bridge (source-proven exact target -> static global target value
# terminal; no result publication). The me-call probe's TargetOnly arm in
# `static_current_owner_policy.rs` stays a named error (deliberately excluded
# sibling).
MEMBER_ROUTE="$ROOT_DIR/src/mir/builder/calls/member_route.rs"
CALLS_MOD="$ROOT_DIR/src/mir/builder/calls/mod.rs"
PHYSICAL_BRIDGE="$ROOT_DIR/src/mir/builder/calls/static_result_publication_physical_bridge.rs"
rg -q 'lower_target_only_static_result_publication_v1' "$MEMBER_ROUTE"
rg -q 'lower_target_only_static_result_publication_v1' "$CALLS_MOD"
rg -q 'fn lower_target_only_static_result_publication_v1' "$PHYSICAL_BRIDGE"
rg -q 'static-result-ingress/target-only' "$STATIC_OWNER_POLICY"

# MIRBUILDER-EXE-ACCEPTANCE-ORDINARY-NEW-HANDLE-ARG-S0: handle-typed
# ordinary-new arguments are one vocabulary on the selected lane —
# OrdinaryObservation::Handle maps to SelectedNewArgumentKindV1::Handle,
# the fallback coseal keeps the walk's argument observations AND the
# declared parameter contracts, and selected emission materializes
# Handle like Local (exact binding + source-site observation; no name
# lookup). A non-trivial argument still fails ArgumentNotTrivial at the
# issued-claim precheck — no decline-to-raw route.
SEL_ARG="$ROOT_DIR/src/mir/resolved_semantics/selected_new_arguments.rs"
LOCAL_FLOW="$ROOT_DIR/src/mir/resolved_semantics/home_prefix_local_flow.rs"
NEW_PREFIX="$ROOT_DIR/src/mir/resolved_semantics/home_new_prefix.rs"
NEW_PREFIX_ARGS="$ROOT_DIR/src/mir/resolved_semantics/home_new_prefix_arguments.rs"
NEW_PREFIX_SCAN="$ROOT_DIR/src/mir/resolved_semantics/home_new_prefix_scan.rs"
NEW_PREFIX_TERMINAL="$ROOT_DIR/src/mir/resolved_semantics/home_new_prefix_terminal.rs"
ORD_ARGS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_arguments.rs"
COSEAL_HELPERS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_coseal_helpers.rs"
SEL_EMIT="$ROOT_DIR/src/mir/builder/ordinary_new_admission/selected.rs"
SEL_EMIT_ARGS="$ROOT_DIR/src/mir/builder/ordinary_new_admission/selected/arguments.rs"
PHYS_ABI="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/physical_abi.rs"
EMIT_VALID="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/emission_validation.rs"
BRAND_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/brand_catalog_tests.rs"
rg -q 'Handle \{ binding: BindingRefV1 \}' "$SEL_ARG"
rg -q 'Handle \{ binding: BindingRefV1 \}' "$ORD_ARGS"
rg -q 'Self::Handle\(root\) => Some\(SelectedNewArgumentKindV1::Handle \{ binding: root \}\)' "$LOCAL_FLOW"
rg -q 'fn issue_new_home_prefixes_with_arguments_v1' "$NEW_PREFIX_ARGS"
rg -q 'issue_new_home_prefixes_with_arguments_v1' "$NEW_PREFIX"
rg -q 'issue_new_home_prefixes_with_arguments_v1' "$COSEAL_ISSUE"
rg -q 'parameter_contracts\.iter\(\)\.filter\(\|row\| row\.batch_slot == batch_slot\)' "$COSEAL_ISSUE"
rg -q 'OrdinaryNewTrivialArgumentKindV1::Handle \{ binding: \*binding \}' "$COSEAL_HELPERS"
rg -q 'OrdinaryNewTrivialArgumentKindV1::Handle \{ binding \}' "$SEL_EMIT" "$SEL_EMIT_ARGS"
rg -q 'value_for_exact_binding' "$SEL_EMIT" "$SEL_EMIT_ARGS"
rg -q 'observe_variable_site' "$SEL_EMIT" "$SEL_EMIT_ARGS"
rg -q 'Kind::Handle' "$PHYS_ABI"
rg -q 'OrdinaryNewTrivialArgumentKindV1::Handle' "$EMIT_VALID"
rg -q 'ordinary_new_claim_records_parameter_handle_argument' "$BRAND_TESTS"

# MIRBUILDER-GATE1-ORDINARY-NEW-ARGUMENT-SOURCE-S0: a local bound to an
# inventoried call (method_calls()/direct_call_observations() site
# membership) installs StoredLocal::BoundValue, which observes as
# OrdinaryObservation::BoundValue and flows through the same
# SelectedNewArgument -> OrdinaryNewTrivialArgument -> exact-binding
# materialization lane as Local/Handle. Non-inventoried arguments
# still fail ArgumentNotTrivial — no decline-to-raw route.
SEL_ARG_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/brand_catalog_selected_new_argument_tests.rs"
rg -q 'BoundValue \{ binding: BindingRefV1 \}' "$SEL_ARG"
rg -q 'BoundValue \{ binding: BindingRefV1 \}' "$ORD_ARGS"
rg -q 'StoredLocal::BoundValue' "$LOCAL_FLOW"
rg -q 'OrdinaryObservation::BoundValue' "$LOCAL_FLOW"
rg -q 'fn install_bound_value' "$LOCAL_FLOW"
rg -q 'install_inventoried_call_result' "$NEW_PREFIX_SCAN"
rg -q 'OrdinaryNewTrivialArgumentKindV1::BoundValue \{ binding: \*binding \}' "$COSEAL_HELPERS"
rg -q 'OrdinaryNewTrivialArgumentKindV1::BoundValue \{ binding \}' "$SEL_EMIT" "$SEL_EMIT_ARGS"
rg -q 'Kind::BoundValue' "$PHYS_ABI"
rg -q 'OrdinaryNewTrivialArgumentKindV1::BoundValue' "$EMIT_VALID"
rg -q 'selected_new_arguments_admit_inventoried_call_result_local' "$SEL_ARG_TESTS"

# MIRBUILDER-EXE-ACCEPTANCE-ENV-DIRECT-RECEIVER-S0: bare `env.<method>`
# receivers classify into the existing `EnvMethod` route inside
# `plan_member_call_route` BEFORE static-receiver resolution, gated on the
# same `get_env_method_spec` authority the located lanes use. A bound
# `env` local keeps the Standard route (variable_map gate) and unknown
# methods fail with a named error — never a StaticReceiver probe.
ENV_DIRECT_TESTS="$ROOT_DIR/src/mir/builder/calls/member_route_env_direct_tests.rs"
DESCENT_TESTKIT="$ROOT_DIR/src/mir/builder/calls/member_route_descent_testkit.rs"
DESCENT_TESTS="$ROOT_DIR/src/mir/builder/calls/member_route_descent_tests.rs"
rg -q 'name == "env"' "$MEMBER_ROUTE"
rg -q 'variable_map' "$MEMBER_ROUTE"
rg -q 'get_env_method_spec\("env", method\)' "$MEMBER_ROUTE"
rg -q 'env method not supported' "$MEMBER_ROUTE"
rg -q 'env_direct_receiver_uses_env_terminal_and_emits_extern_call' "$ENV_DIRECT_TESTS"
rg -q 'env_direct_receiver_rejects_unknown_method_with_named_error' "$ENV_DIRECT_TESTS"
rg -q 'bound_env_receiver_keeps_standard_route' "$ENV_DIRECT_TESTS"
rg -q 'env_route_keeps_receiver_syntax_only_and_descends_arguments' "$DESCENT_TESTS"

# MIRBUILDER-EXE-ACCEPTANCE-NESTED-PROGRAM-ITEM-SITE-S0: the single
# lane-side item-site producer `body_item_site` collapses a NESTED
# Program body root (`[stmt, ..., ProgramBodyRoot]`) to the rootless
# `[stmt, ..., ProgramBody(i)]` spelling every walk-registered map
# (locals/variables/initializers) keys on. Only the absolute
# `[ProgramBodyRoot]` script root (len == 1) keeps the rootful item
# form — no consumer-side rewrite, no walk change.
ITEM_SITE="$ROOT_DIR/src/mir/builder/raw_invocation_source_item_site.rs"
MAP_LOCAL_TESTS="$ROOT_DIR/src/mir/builder/normal_callable_semantic_lowering_state/map_local_tests.rs"
rg -q 'fn is_rootless_item_site' "$ITEM_SITE"
rg -q 'kind == SourceBodyKindV1::Program && site\.segments\(\)\.len\(\) > 1' "$ITEM_SITE"
rg -q 'nested_program_items_drop_the_program_body_root' "$ITEM_SITE"
rg -q 'program_items_keep_the_explicit_program_root' "$ITEM_SITE"
rg -q 'nested_program_local_registers_rootless_statement_site' "$MAP_LOCAL_TESTS"

# MIRBUILDER-EXE-ACCEPTANCE-COMPOSITE-RETURN-EXIT-S0: the composite
# LoopBreak lane is the sole physical owner for return-in-body
# `loop(cond){…return…}`; `validate_exit_ledger` admits a root carrying a
# root-targeted `Break` OR an `ExplicitReturn`/`Return{target_function}`
# record, and an exit-free root still fails `RootExitMissing` (tolerated
# as "no candidate" by the package issuer — never a fallback).
COMPOSITE_PROJECTION="$ROOT_DIR/src/mir/compiler/loop_break_composite_source_projection.rs"
LOOP_BREAK_FACTS="$ROOT_DIR/src/mir/builder/normal_callable_loop_source_facts/loop_break.rs"
rg -q 'RootExitMissing' "$COMPOSITE_PROJECTION"
if rg -n 'RootBreakMissing' "$COMPOSITE_PROJECTION" "$LOOP_BREAK_FACTS"; then
  echo "[$TAG] composite root-exit reject regressed to break-only" >&2
  exit 1
fi
rg -q 'ResolvedControlTransferV1::Return \{ \.\. \}' "$COMPOSITE_PROJECTION"
rg -q 'ResolvedExitOriginV1::ExplicitReturn' "$COMPOSITE_PROJECTION"
rg -q 'RootExitMissing' "$LOOP_BREAK_FACTS"
rg -q 'return_in_body_projection_is_admitted_by_composite_owner' "$COMPOSITE_PROJECTION"
rg -q 'exit_free_loop_declines_with_root_exit_missing' "$COMPOSITE_PROJECTION"

# MIRBUILDER-EXE-ACCEPTANCE-DEAD-ROUTE-SOLE-FAMILY-S1: `LoopBreakRecipe`
# is TypedDeclined wire vocabulary (retired plan-lane pipeline) — matched
# rows stay provenance-only. Live front-selectable routes must not be
# suppressed by `pred_loop_break_recipe`, and `sole_family()` filters only
# that one owner-less id; every other live route still contends.
ROUTE_PREDICATES="$ROOT_DIR/src/mir/builder/control_flow/joinir/route_entry/registry/predicates.rs"
CALLABLE_ROUTE="$ROOT_DIR/src/mir/builder/normal_callable_loop_source_route.rs"
DEAD_ROUTE_TESTS="$ROOT_DIR/src/mir/builder/normal_callable_loop_source_route_dead_route_tests.rs"
if rg -n '!pred_loop_break_recipe' "$ROUTE_PREDICATES"; then
  echo "[$TAG] dead LoopBreakRecipe route suppresses a live route again" >&2
  exit 1
fi
rg -q 'route != LoopRouteId::LoopBreakRecipe' "$CALLABLE_ROUTE"
rg -q 'sole_family_ignores_owner_less_loop_break_recipe' "$DEAD_ROUTE_TESTS"
rg -q 'loop_simple_while_overlap_keeps_sole_family_none' "$DEAD_ROUTE_TESTS"
rg -q 'trim_header_dead_route_does_not_hide_loop_cond_sole_family' "$DEAD_ROUTE_TESTS"

# MIRBUILDER-EXE-ACCEPTANCE-COND-SUBSTRING-COVERAGE-S2: the source-side
# CoreMethod placement arm is an allowed SET per (op, arity); StringLen/0
# stays Condition-only, StringSubstring/2 admits Body+Condition, and every
# other (op, arity) stays unarmed. The sealed contract keeps the site's
# actual resolved placement, the verifier mirrors the same set, and
# named-array push stays Body-only — no new physical path, no probe
# coverage change, no receiver widening.
CORE_METHOD_SRC="$ROOT_DIR/src/mir/source_call_target/core_method.rs"
NAMED_ARRAY_SRC="$ROOT_DIR/src/mir/source_call_target/named_array_method.rs"
NAMED_ARRAY_METHOD_TESTS="$ROOT_DIR/src/mir/source_call_target/named_array_method_tests.rs"
CONTRACT_ISSUER="$ROOT_DIR/src/mir/resolved_semantics/resolver_core_method_callable_contract.rs"
LEDGER_CONTRACT_TESTS="$ROOT_DIR/src/mir/resolved_semantics/callable_source_ledger_contract_tests.rs"
rg -q 'fn allowed_placements' "$CORE_METHOD_SRC"
rg -q 'allowed\.contains\(placement\)' "$CORE_METHOD_SRC"
rg -q 'manifest_row.op == CoreMethodOp::StringLen' "$CORE_METHOD_SRC"
rg -q 'receiver_has_text_evidence\(ledger, call, &rows\)' "$CORE_METHOD_SRC"
rg -q 'fn allowed_target_placements' "$CONTRACT_ISSUER"
rg -q '\(CoreMethodOp::ArrayPush, 1\) => &\[ResolvedLoopPlacementV1::Body\]' "$CONTRACT_ISSUER"
rg -q 'allowed\.contains\(&placement\)' "$CONTRACT_ISSUER"
rg -q '== Some\(ResolvedLoopPlacementV1::Body\)' "$NAMED_ARRAY_SRC"
rg -q 'fn real_core_method_ledger_with_placements' "$TESTKIT"
rg -q 'condition_position_substring_contracts_and_consumes_exact_row' "$ITEMS_TESTS"
rg -q 'body_position_length_stays_unarmed' "$ITEMS_TESTS"
rg -q 'resolver_callable_contract_co_seals_condition_substring_and_generated_target' "$LEDGER_CONTRACT_TESTS"
rg -q 'resolver_callable_contract_rejects_condition_claimed_body_only_target' "$LEDGER_CONTRACT_TESTS"
rg -q 'condition_position_push_stays_unarmed' "$NAMED_ARRAY_METHOD_TESTS"

# MIRBUILDER-GATE1-CALLABLE-LOOP-ROUTE-FRONT-S0: the named-array ArrayBox
# issuer arm also seals bounded ArrayGet/1 read contracts — LoopBody
# placement only, gated by field-residence receiver evidence (me-field
# alias + declared-or-untyped ArrayBox field + birth `new ArrayBox()`
# provider + no rebind + integer-source index), minting a plain contract
# with no write-requirement product. get/1 stays ambiguous against
# MapBox.get/1 without that evidence; DynamicToCaller is the only new
# result relation and maps to MirType::Unknown — no probe or route
# selection change.
CORE_METHOD_TARGET="$ROOT_DIR/src/mir/resolved_semantics/core_method_instance_target.rs"
CORE_METHOD_CONSUMER="$ROOT_DIR/src/mir/builder/normal_callable_semantic_lowering_state/source_call_publication.rs"
CORE_METHOD_PACKAGE_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/core_method_source_tests.rs"
rg -q 'ArrayDynamicRead' "$CORE_METHOD_TARGET"
rg -q 'fn array_dynamic_read' "$CORE_METHOD_TARGET"
rg -q 'DynamicToCaller' "$CORE_METHOD_TARGET"
rg -q 'fn issue_array_get_read_contract' "$NAMED_ARRAY_SRC"
rg -q 'NamedArrayResultDemandV1::ReadResult' "$NAMED_ARRAY_SRC"
rg -q '\(CoreMethodOp::ArrayGet, 1\) => &\[ResolvedLoopPlacementV1::Body\]' "$CONTRACT_ISSUER"
rg -q 'DynamicToCaller' "$CONTRACT_ISSUER"
rg -q 'DynamicToCaller' "$CORE_METHOD_CONSUMER"
rg -q 'MirType::Unknown' "$CORE_METHOD_CONSUMER"
rg -q 'field_resident_array_get_issues_plain_core_method_contract' "$CORE_METHOD_PACKAGE_TESTS"
rg -q 'field_resident_get_rejects_receiver_rebind_and_non_integer_index' "$CORE_METHOD_PACKAGE_TESTS"
rg -q 'non_resident_get_stays_unarmed' "$CORE_METHOD_PACKAGE_TESTS"

# MIRBUILDER-GATE1-CALLABLE-LOOP-STRING-INDEXOF-S0: the generic StringBox
# issuer arm seals bounded StringIndexOf/1 contracts — LoopBody placement
# only, gated by receiver- and needle-text evidence (string literal or a
# TextToCaller contract minted in the same issuance, single initializer,
# no rebind). indexOf/1 is catalog-unique on StringBox but the runtime
# router also accepts ArrayBox.indexOf/1, so the arm never mints from the
# selector alone; parameter receivers and arity-2 stay unarmed. The
# borrowed needle records the new TextParameter relation; I64ToCaller
# reuses the existing Integer consumer mapping.
SOURCE_ISSUER="$ROOT_DIR/src/mir/source_call_target/core_method.rs"
rg -q 'fn index_of_has_text_evidence' "$SOURCE_ISSUER"
rg -q 'fn text_source_at' "$SOURCE_ISSUER"
rg -q 'TextParameter' "$CORE_METHOD_TARGET"
rg -q '\(CoreMethodOp::StringIndexOf, 1\) => &\[ResolvedLoopPlacementV1::Body\]' "$SOURCE_ISSUER"
rg -q '\(CoreMethodOp::StringIndexOf, 1\) => &\[ResolvedLoopPlacementV1::Body\]' "$CONTRACT_ISSUER"
rg -q 'CoreMethodOp::StringIndexOf => &\[Parameter::TextParameter\]' "$CONTRACT_ISSUER"
rg -q 'literal_receiver_index_of_arms_body_with_text_parameter' "$CORE_METHOD_PACKAGE_TESTS"
rg -q 'substring_contract_supplies_index_of_needle_text' "$CORE_METHOD_PACKAGE_TESTS"
rg -q 'find_alias_follows_the_same_text_evidence_gate' "$CORE_METHOD_PACKAGE_TESTS"
rg -q 'index_of_stays_unarmed_without_text_evidence' "$CORE_METHOD_PACKAGE_TESTS"
rg -q 'index_of_condition_placement_stays_rejected' "$CORE_METHOD_PACKAGE_TESTS"

# MIRBUILDER-EXE-ACCEPTANCE-DECLARED-INSTANCE-LOOP-LOCATOR-S3: callable loop
# source lanes carry the package-issued declared-instance locator so a
# loop-body `me.method` site consumes its exact relation row and emits the
# canonical `Callee::SameModuleInstance` — the same physical meaning the
# body lane produces. The locator scope stays `Copy` via a shared
# `RefCell` consumed set; the port hook `exact_source_declared_instance_call_v1`
# defaults to `Ok(None)` for unarmed ports; statement and value positions
# share one `me_this_method_call_effect` precedence (armed locator ->
# exact receiver -> bound/static fallback); `CoreEffectPlan::DeclaredInstanceCall`
# delegates to the sole canonical emitter; armed lanes never fall back to
# dynamic `MethodCall`.
DECLARED_INSTANCE_LOCATOR="$ROOT_DIR/src/mir/normal_callable_semantic_package/declared_instance_locator.rs"
CORE_EFFECT_PLAN="$ROOT_DIR/src/mir/builder/control_flow/plan/effect.rs"
SOURCE_METHOD_PORT="$ROOT_DIR/src/mir/builder/control_flow/plan/expression_port/source_method.rs"
UNIFIED_EMITTER="$ROOT_DIR/src/mir/builder/calls/unified_emitter.rs"
LOOP_DECLARED_TESTS="$ROOT_DIR/src/mir/builder/control_flow/plan/parts/associated_source/callable_loop_source_declared_instance_tests.rs"
rg -q 'consumed: &.a RefCell<BTreeSet<u32>>' "$DECLARED_INSTANCE_LOCATOR"
rg -q 'declared_instance_locator: Option<DeclaredInstanceCallLocatorScopeV1' "$LOOP_SRC_PORT"
rg -q 'fn exact_source_declared_instance_call_v1' "$EXPR_PORT"
rg -q 'fn exact_source_declared_instance_call_v1' "$LOOP_SRC_PORT"
rg -q 'ExactSourceDeclaredInstanceCallV1' "$SOURCE_METHOD_PORT"
rg -q 'DeclaredInstanceCall \{' "$CORE_EFFECT_PLAN"
rg -q 'fn me_this_method_call_effect' "$NORMALIZER_COMMON"
rg -q 'fn emit_canonical_instance_call_at_v1' "$UNIFIED_EMITTER"
rg -q 'armed_loop_body_me_call_consumes_the_exact_locator_row' "$LOOP_DECLARED_TESTS"
rg -q 'armed_locator_rejects_a_foreign_method_key_without_fallback' "$LOOP_DECLARED_TESTS"
rg -q 'unarmed_port_projects_no_declared_instance_call' "$LOOP_DECLARED_TESTS"

# MIRBUILDER-EXE-ACCEPTANCE-SELECTED-STATIC-CALL-LOOP-PUBLICATION-S4: the
# take->install->consume machinery existed only for composite loops; the
# arming gate widens to every armed loop-source candidate so LoopCond /
# LoopTrue / direct loop_break lanes take their Selected publication rows,
# and all three routes lower through lower_with_source_publication so the
# SourcePublication emission port stays the sole physical consumer. The
# post-lower residual check fails on any taken-but-unemitted handoff —
# no silent GlobalCall fallback on the armed lane.
RAW_LOOP_PORT="$ROOT_DIR/src/mir/builder/raw_loop_child_port.rs"
RAW_LOOP_ENTRY="$ROOT_DIR/src/mir/builder/raw_loop_child_entry.rs"
rg -q 'state\.source_loop_items\(site\)\.is_some\(\)' "$RAW_LOOP_PORT"
rg -q 'has_pending_source_static_result_publications' "$RAW_LOOP_PORT"
rg -q 'residual-after-lower' "$RAW_LOOP_PORT"
test "$(rg -c 'lower_with_source_publication' "$RAW_LOOP_ENTRY")" -ge 4

# MIRBUILDER-EXE-ACCEPTANCE-REVIEW-LINE-BOUNDARY-R0: the S8 loop-carrier
# ledger-publication slice pushed `normal_callable_semantic_lowering_state.rs`
# past the hard 800-line stop, and the backend-view / lifecycle test files
# crossed it too, without guard coverage. The ledger-publication APIs now
# live in a dedicated submodule (same split pattern as
# `source_call_publication` / `map_local_tests`), the backend-view drift
# pins live in a sibling test module, and the main-selection pins live in
# a sibling lifecycle test module — every file is registered here so the
# boundary cannot silently regrow.
CALLABLE_LOWERING_STATE="$ROOT_DIR/src/mir/builder/normal_callable_semantic_lowering_state.rs"
CALLABLE_STATE_SOURCE_PREP="$ROOT_DIR/src/mir/builder/normal_callable_semantic_lowering_state/source_prepare.rs"
LIFECYCLE_PHYSICAL_PROGRAM="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/physical_program.rs"
LIFECYCLE_PHYSICAL_PROJECTION="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/physical_program_projection.rs"
LOOP_VALUE_PUBLICATION="$ROOT_DIR/src/mir/builder/normal_callable_semantic_lowering_state/loop_value_publication.rs"
BACKEND_VIEW_TESTS="$ROOT_DIR/src/mir/function/published_backend_view_tests.rs"
BACKEND_VIEW_DRIFT_TESTS="$ROOT_DIR/src/mir/function/published_backend_view_drift_tests.rs"
BACKEND_VIEW_INTRINSIC_TESTS="$ROOT_DIR/src/mir/function/published_backend_view_intrinsic_array_tests.rs"
ROOT_LIFECYCLE_TESTS="$ROOT_DIR/src/mir/builder/normal_default_root_catalog_lifecycle_tests.rs"
ROOT_MAIN_SELECTION_TESTS="$ROOT_DIR/src/mir/builder/normal_default_root_catalog_main_selection_tests.rs"
LOOP_PIPELINE_TESTS="$ROOT_DIR/src/mir/compiler/normal_default_pipeline_loop_tests.rs"
LOOP_SCOPE_TESTS="$ROOT_DIR/src/mir/builder/normal_default_root_catalog_loop_scope_tests.rs"

# MIRBUILDER-GATE1-CALLFREE-LOOPCOND-S0: a loop whose call inventory is
# empty is admitted only beside a bridge-issued
# VerifiedCallableLoopCallFreeCoverageV1 proof — the bridge resolver-only
# scan seals the `<` condition site plus the flat NoExitBody statement and
# expression rows under the loop (integer/typed literals, lexical locals,
# Add/Multiply, plain BindingRebind writes, no exits, no residual calls).
# The route token co-seals the coverage as CallFree/WithCalls and rejects
# foreign owners, foreign loop sites, residual call evidence, and any
# item beside a CallFree claim; the physical input re-verifies owner,
# parent, condition site, and under-loop containment before the existing
# LoopCond owner lowers. Unproven empty inventories stay
# SourceItemsMissing; a hidden call never mints CallFree.
ROUTE_ITEMS_SRC="$ROOT_DIR/src/mir/builder/normal_callable_loop_source_route_items.rs"
ROUTE_CALL_FREE_TESTS="$ROOT_DIR/src/mir/builder/normal_callable_loop_source_route_call_free_tests.rs"
LEXICAL_INSTANCE_CALL_SRC="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_lexical_instance_call.rs"
LEXICAL_INSTANCE_PROVENANCE_SRC="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_lexical_instance_call_provenance.rs"
LEXICAL_INSTANCE_SOURCE_SRC="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_lexical_instance_call_source.rs"
BORROWED_FORMAL_USE_SRC="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_borrowed_formal_uses.rs"
BORROWED_FORMAL_USE_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_borrowed_formal_use_tests.rs"
BORROWED_FORMAL_SOURCE_SRC="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_borrowed_formal_source.rs"
BORROWED_FORMAL_SOURCE_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_borrowed_formal_source_tests.rs"
BORROWED_FORMAL_ACTUAL_SRC="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_borrowed_formal_actuals.rs"
BORROWED_FORMAL_ACTUAL_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_borrowed_formal_actual_tests.rs"
BORROWED_FORMAL_ENTRY_SRC="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_borrowed_formal_entry.rs"
BORROWED_FORMAL_ENTRY_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_borrowed_formal_entry_tests.rs"
BORROWED_FORMAL_RESULT_SRC="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_borrowed_formal_result.rs"
BORROWED_FORMAL_RESULT_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_borrowed_formal_result_tests.rs"
BORROWED_FORMAL_TERMINAL_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_borrowed_formal_terminal_tests.rs"
BORROWED_ENTRY_VALUES="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_borrowed_formal_entry_values.rs"
BORROWED_ALIAS_MATERIALIZATION="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_borrowed_alias_materialization.rs"
BORROWED_ALIAS_MATERIALIZATION_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_borrowed_alias_materialization_tests.rs"
BORROWED_ALIAS_STATE_TESTS="$ROOT_DIR/src/mir/builder/normal_callable_semantic_lowering_state/borrowed_alias_materialization_tests.rs"
PHYSICAL_COPY_BOUNDARY="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/physical_boundary.rs"
BORROWED_COPY_BOUNDARY="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/physical_boundary_borrowed_copies.rs"
BORROWED_COPY_BOUNDARY_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/physical_boundary_borrowed_copies_tests.rs"
BORROWED_ENTRY_STATE="$ROOT_DIR/src/mir/builder/normal_callable_semantic_lowering_state/borrowed_entry.rs"
BORROWED_ENTRY_STATE_TESTS="$ROOT_DIR/src/mir/builder/normal_callable_semantic_lowering_state/borrowed_entry_tests.rs"
BORROWED_ENTRY_SCOPE_TESTS="$ROOT_DIR/src/mir/builder/normal_callable_semantic_loan_port/borrowed_entry_scope_tests.rs"
BORROWED_ACTUAL_OBSERVER="$ROOT_DIR/src/mir/resolved_semantics/home_local_call_borrowed_actuals.rs"
LEXICAL_INSTANCE_CALL_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_lexical_instance_call_tests.rs"
VALUE_METHOD_CALL_SRC="$ROOT_DIR/src/mir/builder/control_flow/plan/normalizer/helpers_value/method_call.rs"
ROUTE_TEST_SURFACE="$ROOT_DIR/src/mir/builder/normal_callable_loop_source_route_tests.rs"
SOURCE_LOOP_BRIDGE="$ROOT_DIR/src/mir/builder/normal_callable_semantic_lowering_state/source_loop_bridge.rs"
LOOP_COND_FACTS_SRC="$ROOT_DIR/src/mir/builder/normal_callable_loop_source_facts/loop_cond.rs"
GENERIC_FACTS_ISSUER="$ROOT_DIR/src/mir/builder/normal_callable_loop_source_facts/generic/issuer.rs"
rg -q 'VerifiedCallableLoopCallFreeCoverageV1' "$ROUTE_ITEMS_SRC"
rg -q 'CallFree\(VerifiedCallableLoopCallFreeCoverageV1\)' "$ROUTE_ITEMS_SRC"
rg -q 'fn has_residual_evidence' "$ROUTE_ITEMS_SRC"
rg -q 'fn issue_call_free_coverage' "$SOURCE_LOOP_BRIDGE"
rg -q 'Armed \{' "$SOURCE_LOOP_BRIDGE"
rg -q 'SourceCoverageForeign' "$CALLABLE_ROUTE"
rg -q 'SourceCoverageSiteMismatch' "$CALLABLE_ROUTE"
rg -q 'SourceCallResidualEvidence' "$CALLABLE_ROUTE"
rg -q 'fn source_coverage' "$CALLABLE_ROUTE"
rg -q 'call_free' "$GENERIC_FACTS_ISSUER"
rg -q 'source_coverage: CallableLoopSourceCallCoverageV1' "$LOOP_COND_FACTS_SRC"
rg -q 'call-free-coverage-mismatch' "$LOOP_COND_FACTS_SRC"
rg -q 'call_free: Option<VerifiedCallableLoopCallFreeCoverageV1>' "$RAW_LOOP_ENTRY"
rg -q 'source_backed_loop_keeps_invocation_scope_and_ledger_route' "$LOOP_SCOPE_TESTS"
rg -q 'source_backed_call_free_loop_carries_two_bindings' "$LOOP_SCOPE_TESTS"
rg -q 'source_backed_loop_outside_call_free_grammar_still_stops_at_source_items' "$LOOP_SCOPE_TESTS"
rg -q 'source_backed_loop_with_hidden_call_never_mints_call_free' "$LOOP_SCOPE_TESTS"
rg -q 'issue_with_source_relations_admits_verified_call_free_coverage' "$ROUTE_CALL_FREE_TESTS"
rg -q 'issue_with_source_relations_keeps_unproven_empty_inventory_missing' "$ROUTE_CALL_FREE_TESTS"
rg -q 'issue_with_source_relations_rejects_a_foreign_call_free_owner' "$ROUTE_CALL_FREE_TESTS"
rg -q 'issue_with_source_relations_rejects_a_foreign_call_free_site' "$ROUTE_CALL_FREE_TESTS"
rg -q 'issue_with_source_relations_rejects_call_free_with_residual_evidence' "$ROUTE_CALL_FREE_TESTS"
rg -q 'issue_with_source_relations_rejects_call_free_beside_call_items' "$ROUTE_CALL_FREE_TESTS"

# MIRBUILDER-GATE1-PARAM-RECEIVER-CALL-SOURCE-S0: the ordinary_new co-seal
# cohort issues InstanceMethod lexical dispositions — claim-local receivers
# from their sole new initializer, parameter receivers only when every
# caller edge matching the callee selector+arity proves one agreed
# ordinary-new claim class (universal proof; ambiguous/rebound/call-result
# evidence vetoes and stays unarmed). Variable receivers consult the
# lexical map only; `me`/`this` keep the strict declared-instance locator.
rg -q 'InstanceMethod' "$ROUTE_ITEMS_SRC"
rg -q 'fn prepare_lexical_source_targets_v1' "$LEXICAL_INSTANCE_SOURCE_SRC"
rg -q 'prepare_local_candidates_by_slot_v1' "$COSEAL_ISSUE_SOURCE"
rg -q 'lexical_source_targets' "$COSEAL_ISSUE"
rg -q 'fn lexical_instance_call_covered' "$LEXICAL_INSTANCE_CALL_SRC"
rg -q 'fn take_lexical_instance_call' "$LEXICAL_INSTANCE_CALL_SRC"
rg -q 'lexical_instance_call_arms_parameter_receiver_with_claim_edge' "$LEXICAL_INSTANCE_CALL_TESTS"
rg -q 'lexical_instance_call_arms_claim_local_receiver' "$LEXICAL_INSTANCE_CALL_TESTS"
rg -q 'lexical_instance_call_keeps_call_result_argument_unarmed' "$LEXICAL_INSTANCE_CALL_TESTS"
rg -q 'lexical_instance_call_keeps_call_result_receiver_unarmed' "$LEXICAL_INSTANCE_CALL_TESTS"
rg -q 'lexical_instance_call_vetoes_ambiguous_argument_classes' "$LEXICAL_INSTANCE_CALL_TESTS"
rg -q 'lexical_instance_call_keeps_rebound_parameter_unarmed' "$LEXICAL_INSTANCE_CALL_TESTS"
rg -q 'fn lower_method_call_value_input' "$VALUE_METHOD_CALL_SRC"

# MIRBUILDER-GATE1-BODY-LENGTH-TEXT-EVIDENCE-S0: `StringLen/0` admits Body
# placement only beside receiver text evidence (`TextToCaller` initializer
# or string literal — the indexOf proof shape); parameters and non-text
# locals stay unarmed. The verify-side placement mirror moves in lockstep,
# and a lone SelectedStatic beside method buckets keeps coverage proof —
# no silent bucket drop.
rg -q 'fn receiver_has_text_evidence' "$CORE_METHOD_SRC"
rg -q 'StringLen, 0' "$CORE_METHOD_SRC"
rg -q 'ResolvedLoopPlacementV1::Body' "$CORE_METHOD_SRC"
rg -q 'fn allowed_target_placements' "$CONTRACT_ISSUER"
rg -q 'body_position_length_with_text_evidence_arms' "$ITEMS_TESTS"
rg -q 'body_position_length_without_text_evidence_stays_unarmed' "$ITEMS_TESTS"
rg -q 'fn with_covered_call_items' "$ROUTE_ITEMS_SRC"

# MIRBUILDER-GATE1-FIELD-WRITE-CLAIM-EDGE-TRANSPORT-S0: a field claims a
# class only when every package write to that field name is an
# attributed `me.` write storing `new` of one agreed ordinary box —
# non-`new` values, multi-class writers, unattributed receivers, and a
# missing body-shape inventory all veto. The shared `initializer_class`
# transports the claim into both parameter edges and claim-local
# receivers.
FIELD_WRITE_CLAIM_SRC="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_field_write_claim.rs"
rg -q 'struct OrdinaryNewFieldWriteClaimDraftV1' "$FIELD_WRITE_CLAIM_SRC"
rg -q 'unattributed_fields' "$FIELD_WRITE_CLAIM_SRC"
rg -q 'fn field_write_claim' "$COSEAL_LEDGER"
rg -q 'field_write_draft' "$COSEAL_ISSUE_SOURCE"
rg -q 'fn initializer_class' "$LEXICAL_INSTANCE_PROVENANCE_SRC"
rg -q 'lexical_instance_call_arms_parameter_receiver_with_field_read_edge' "$LEXICAL_INSTANCE_CALL_TESTS"
rg -q 'lexical_instance_call_arms_field_read_claim_local_receiver' "$LEXICAL_INSTANCE_CALL_TESTS"
rg -q 'lexical_instance_call_vetoes_multi_class_field_writers' "$LEXICAL_INSTANCE_CALL_TESTS"
rg -q 'lexical_instance_call_vetoes_unattributed_field_write' "$LEXICAL_INSTANCE_CALL_TESTS"
rg -q 'lexical_instance_call_keeps_non_new_field_write_unarmed' "$LEXICAL_INSTANCE_CALL_TESTS"

# MIRBUILDER-GATE1-CALLRESULT-RECEIVER-RESULTCLASS-S0: a selected callable
# claims a result class only when its sealed body ends in a value-bearing
# `return` and every `return` row constructs `new` of one agreed ordinary
# box — missing inventories, value-less returns, non-`new` values and
# mixed classes all veto. The lexical `MethodCall` initializer arm joins
# receiver-class proof + unique selected callee + result-class claim.
RESULT_CLASS_CLAIM_SRC="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_result_class_claim.rs"
rg -q 'struct OrdinaryNewResultClassClaimDraftV1' "$RESULT_CLASS_CLAIM_SRC"
rg -q 'OrdinaryNewResultClassClaimsV1' "$RESULT_CLASS_CLAIM_SRC"
rg -q 'fn callable_result_class' "$COSEAL_LEDGER"
rg -q 'result_class_draft' "$COSEAL_ISSUE_SOURCE"
rg -q 'fn binding_class' "$LEXICAL_INSTANCE_PROVENANCE_SRC"
rg -q 'MAX_PROVENANCE_DEPTH' "$LEXICAL_INSTANCE_PROVENANCE_SRC"
rg -q 'lexical_instance_call_arms_call_result_receiver_with_result_class' "$LEXICAL_INSTANCE_CALL_TESTS"
rg -q 'lexical_instance_call_vetoes_mixed_return_classes' "$LEXICAL_INSTANCE_CALL_TESTS"
rg -q 'lexical_instance_call_keeps_non_new_return_unarmed' "$LEXICAL_INSTANCE_CALL_TESTS"
rg -q 'lexical_instance_call_keeps_fallthrough_result_unarmed' "$LEXICAL_INSTANCE_CALL_TESTS"

# MIRBUILDER-GATE1-INSTANCE-ENTRY-HOME-S0: the sole Home ABI issuer lends
# batch-slot-bound entry demands through a verified instance entry loan; the
# source Home Flow installs receiver/parameter bindings only for the exact
# declaration — foreign or missing evidence keeps EntryDemandMissing. The
# loan carries no result relation and publishes no complete call-site ABI.
ENTRY_HOME="$ROOT_DIR/src/mir/normal_callable_semantic_package/instance_entry_home.rs"
ENTRY_HOME_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/instance_entry_home_tests.rs"
HOME_ABI="$ROOT_DIR/src/mir/resolved_semantics/home_abi.rs"
PKG_ISSUER="$ROOT_DIR/src/mir/normal_callable_semantic_package/issuer.rs"
rg -q 'fn issue_source_entry_home_catalog_v1' "$ENTRY_HOME"
rg -q 'VerifiedInstanceEntryHomeLoanV1' "$HOME_ABI"
rg -q 'fn install_entry_home' "$LOCAL_FLOW"
rg -q 'entry_home' "$NEW_PREFIX"
rg -q 'issue_source_entry_home_catalog_v1' "$PKG_ISSUER"
rg -q 'exact_instance_entry_loan_lends_receiver_and_parameter_demands' "$ENTRY_HOME_TESTS"
rg -q 'foreign_cohort_loan_is_rejected_by_the_exact_declaration' "$ENTRY_HOME_TESTS"
rg -q 'duplicate_and_incomplete_parameter_rows_are_rejected' "$ENTRY_HOME_TESTS"
rg -q 'missing_entry_evidence_keeps_receiver_demand_unavailable' "$ENTRY_HOME_TESTS"

# Files split under this guard (2026-10 hard-stop remediation) are registered
# here so the 800-line boundary cannot silently regrow.
LOCAL_COMMIT="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit.rs"
LOCAL_COMMIT_CALL_RECV="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/call_received.rs"
LOCAL_COMMIT_PREP="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/emission_prepare.rs"
LOCAL_COMMIT_HANDLE="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/handle_call.rs"
COSEAL_ISSUE_LEXICAL="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_coseal_issue_lexical.rs"
DIRECT_CALL_PHYSICAL_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/direct_call_physical_tests.rs"
DIRECT_CALL_HANDLE_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/direct_call_handle_result_tests.rs"
ROOT_CALL_ENTRY="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_call_entry.rs"
ROOT_CALL_ENTRY_VALIDATION="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_call_entry/validation.rs"
ROOT_CALL_ENTRY_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_call_entry/tests.rs"
EXACT_LEXICAL_READ="$ROOT_DIR/src/mir/builder/normal_callable_semantic_receiver_crosswalk.rs"
EXACT_LEXICAL_READ_TESTS="$ROOT_DIR/src/mir/builder/normal_callable_semantic_receiver_crosswalk_tests.rs"
LEXICAL_I64_FIXTURE="$ROOT_DIR/src/mir/builder/lexical_call_projection_test_fixture.rs"
LOCAL_CALL_GROUP="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_call_entry/local_binding_group.rs"
LOCAL_CALL_GROUP_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_call_entry/local_binding_group_tests.rs"
FINISHED_SOURCE_PROJECTION="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/finalized_source_projection.rs"
FINISHED_SOURCE_PROJECTION_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_call_entry/finished_projection_tests.rs"
FINALIZED_CALL_VISITOR="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/finalized_lexical_call_visit.rs"
BORROWED_CALL_INCOMING="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/compiled_entry_contract/borrowed_call_incoming.rs"
BORROWED_CALL_USES="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/compiled_entry_contract/borrowed_call_uses.rs"
BORROWED_CALL_USES_TESTS="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/compiled_entry_contract/borrowed_call_uses_tests.rs"
FINISHED_COPY_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_call_entry/finished_copy_tests.rs"
LIFECYCLE_C_FORMALS="$ROOT_DIR/lang/c-abi/shims/published_mir/hako_llvmc_ffi_published_lifecycle_formal_transport.inc"
LIFECYCLE_C_CALLS="$ROOT_DIR/lang/c-abi/shims/published_mir/hako_llvmc_ffi_published_lifecycle_call_transport.inc"
LIFECYCLE_C_PARSER="$ROOT_DIR/lang/c-abi/shims/published_mir/hako_llvmc_ffi_published_lifecycle_physical_v2.inc"
LIFECYCLE_CALL_JSON="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/physical_program_json_call_transport.rs"
BORROWED_SOURCE_PUBLICATION_TESTS="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/borrowed_source_publication_tests.rs"
BORROWED_WIRE_TESTS="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/physical_program_json_borrowed_transport_tests.rs"
LIFECYCLE_PROGRAM_JSON="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/physical_program_json.rs"
BORROWED_USE_PROJECTION_TESTS="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/borrowed_use_projection_tests.rs"
BORROWED_CARRIER_JSON_TESTS="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/borrowed_carrier_json_tests.rs"
LOCAL_COPY_PROVENANCE="$ROOT_DIR/src/mir/builder/normal_callable_semantic_lowering_state/materialized_values.rs"
LOCAL_COPY_PROVENANCE_TESTS="$ROOT_DIR/src/mir/builder/normal_callable_local_copy_tests.rs"
BORROWED_PHYSICAL_PROJECTION="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_call_entry/borrowed_projection.rs"
BORROWED_PHYSICAL_PROJECTION_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_call_entry/borrowed_projection_tests.rs"
LEXICAL_I64_PROJECTION="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_call_entry/lexical_projection.rs"
LEXICAL_I64_EMIT_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_call_entry/lexical_projection_tests.rs"
LEXICAL_I64_EMIT="$ROOT_DIR/src/mir/builder/ordinary_new_admission/selected/terminal_call/lexical_i64.rs"
RAW_CLAIM_TERMINAL_CALL="$ROOT_DIR/src/mir/builder/raw_ordinary_new_claim/terminal_call.rs"
RECURSIVE_CHILD_LOWERING="$ROOT_DIR/src/mir/builder/recursive_child_lowering.rs"
RECURSIVE_CHILD_DISPOSITION="$ROOT_DIR/src/mir/builder/recursive_child_lowering/direct_call_disposition_port.rs"
ROOT_RESULT_NEW_TESTS="$ROOT_DIR/src/mir/builder/normal_default_root_catalog_result_new_tests.rs"

# MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-NORMAL-INTEGER-S0: a
# borrowed tagged formal may lend a NormalInteger view only to the exact
# Greater compare operand. Source admission requires a direct if condition
# beside an integer-literal or origin-sibling field; the published scan
# accepts one Copy-fed view per admitted use, counts distinct operand
# values (edge-port lowering re-emits the same compare), and rejects view
# escape and coverage drift; the lifecycle C lane admits tagged compare
# operands only beside an emitted kind==1 site check and treats empty
# carriers_only edge args as the null row.
DYNAMIC_OPERATOR_ISSUER="$ROOT_DIR/src/mir/dynamic_operator_contract/issuer.rs"
LIFECYCLE_V4_EMIT="$ROOT_DIR/lang/c-abi/shims/published_mir/hako_llvmc_ffi_lifecycle_v4_emit.inc"
LIFECYCLE_V4_INDEXED="$ROOT_DIR/lang/c-abi/shims/published_mir/hako_llvmc_ffi_lifecycle_v4_indexed_flow.inc"
CHECKED_COMPARE_EXE_TEST="$ROOT_DIR/lang/c-abi/tests/published_lifecycle_v4_checked_compare_execution_test.py"
rg -q 'DynamicOperatorFamilyV1::Greater' "$DYNAMIC_OPERATOR_ISSUER"
rg -q 'DynamicOperatorValueClassV1::NormalInteger' "$DYNAMIC_OPERATOR_ISSUER"
rg -q 'CompareOperand' "$BORROWED_FORMAL_USE_SRC"
rg -q 'checked_compare_admits_direct_if_greater_with_integer_or_origin_sibling' "$BORROWED_FORMAL_USE_TESTS"
rg -q 'checked_compare_rejects_outside_if_wrong_operator_and_unproved_sibling' "$BORROWED_FORMAL_USE_TESTS"
rg -q 'fn scan_views' "$BORROWED_CALL_USES"
rg -q 'compare_admissions' "$BORROWED_CALL_USES"
rg -q 'borrowed_use_checked_compare_view_counts_distinct_operands_once' "$BORROWED_CALL_USES_TESTS"
rg -q 'borrowed_use_rejects_compare_view_escape_and_coverage_drift' "$BORROWED_CALL_USES_TESTS"
rg -q 'borrowed_use_rejects_view_shape_drift' "$BORROWED_CALL_USES_TESTS"
rg -q 'checked_compare_view_publishes_from_original_source' "$BORROWED_SOURCE_PUBLICATION_TESTS"
rg -q 'icmp eq i32' "$LIFECYCLE_V4_EMIT"
rg -q 'LV4_TAGGED' "$LIFECYCLE_V4_INDEXED"
rg -q 'carriers_only' "$LIFECYCLE_V4_INDEXED"
test -f "$CHECKED_COMPARE_EXE_TEST"

# MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-ADD-S0: the lent
# view extends to a dominated Add(NormalInteger, NormalInteger) operand; the
# fresh i64 result writes back through the bare field_set row, whose emitted
# dynamic-integer-range check is the sole usize write-lane authority.
ADD_VIEW_EXE_TEST="$ROOT_DIR/lang/c-abi/tests/published_lifecycle_v4_add_view_execution_test.py"
FIELD_REF_PROJECTION="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/physical_program_field_ref.rs"
EXACT_NUMERIC_LIFECYCLE="$ROOT_DIR/src/mir/exact_numeric_backend_capability/lifecycle.rs"
rg -q 'DynamicOperatorNormalResultV1::NormalInteger' "$DYNAMIC_OPERATOR_ISSUER"
rg -q 'AddOperand' "$BORROWED_FORMAL_USE_SRC"
rg -q 'dominated_add_admits_guarded_integer_sibling_and_aliased_operands' "$BORROWED_FORMAL_USE_TESTS"
rg -q 'add_operand_rejects_unguarded_undominated_and_unproved_sibling_uses' "$BORROWED_FORMAL_USE_TESTS"
rg -q 'borrowed_ordinary_add_uses_v1' "$BORROWED_CALL_USES"
rg -q 'undominated-view' "$BORROWED_CALL_USES"
rg -q 'borrowed_use_dominated_add_view_passes' "$BORROWED_CALL_USES_TESTS"
rg -q 'borrowed_use_rejects_undominated_and_drifting_add_view' "$BORROWED_CALL_USES_TESTS"
rg -q 'dominated_add_view_publishes_from_original_source' "$BORROWED_SOURCE_PUBLICATION_TESTS"
rg -q 'FieldSet' "$FIELD_REF_PROJECTION"
rg -q 'exact_numeric_runtime_check' "$LIFECYCLE_C_PARSER"
rg -q 'NYRT_FAULT_REASON_FIELD_RANGE_V1' "$LIFECYCLE_V4_EMIT"
rg -q 'field_set' "$LIFECYCLE_V4_INDEXED"
test -f "$ADD_VIEW_EXE_TEST"

# MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-ARRAYSET-S0: the
# lent view extends to the dominated `.set(index, value)` element operand on
# a proven `me.<ArrayBox>` receiver; the sole physical authority is the
# route-gated `array_set` row lowered to the checked kernel export.
SET_VIEW_EXE_TEST="$ROOT_DIR/lang/c-abi/tests/published_lifecycle_v4_set_view_execution_test.py"
SET_ELEMENT_DRAFT="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_borrowed_formal_use_array_element.rs"
FAULT_CHECKED_ARRAY="$ROOT_DIR/crates/nyash_kernel/src/exports/fault_checked_array.rs"
rg -q 'ArrayElementValue' "$BORROWED_FORMAL_USE_SRC"
rg -q 'array_element_value_kind' "$SET_ELEMENT_DRAFT"
rg -q 'set_element_value_without_entry_receiver_loan_stays_unresolved' "$BORROWED_FORMAL_USE_TESTS"
rg -q 'borrowed_ordinary_array_element_uses_v1' "$FINISHED_SOURCE_PROJECTION"
rg -q 'set-coverage' "$BORROWED_CALL_USES"
rg -q 'borrowed_use_dominated_set_view_passes' "$BORROWED_CALL_USES_TESTS"
rg -q 'borrowed_use_rejects_undominated_and_drifting_set_view' "$BORROWED_CALL_USES_TESTS"
rg -q 'dominated_set_view_publishes_from_original_source' "$BORROWED_SOURCE_PUBLICATION_TESTS"
rg -q 'slot_load_handle' "$FIELD_REF_PROJECTION"
rg -q 'array_set' "$LIFECYCLE_C_PARSER"
rg -q 'array_set' "$LIFECYCLE_V4_INDEXED"
rg -q 'array_set' "$LIFECYCLE_V4_EMIT"
rg -q 'checked_set_i64_v1' "$FAULT_CHECKED_ARRAY"
test -f "$SET_VIEW_EXE_TEST"

# MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-CTORARG-S0: the
# lent view extends to the dominated `new`-argument ordinal; non-literal i64
# birth actuals (local/bound/entry-receiver field) ride the same kind==1
# lane, the caller re-proves kind==1 at the call edge, and non-borrowed
# callers publish through ordinary scalar validation only.
CTOR_VIEW_EXE_TEST="$ROOT_DIR/lang/c-abi/tests/published_lifecycle_v4_new_argument_execution_test.py"
NEW_ARGUMENT_DRAFT="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_borrowed_formal_use_new_argument.rs"
BORROWED_CALL_USES_CTOR_VIEW_TESTS="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/compiled_entry_contract/borrowed_call_uses_ctor_view_tests.rs"
BORROWED_SOURCE_PUBLICATION_NEW_ARGUMENT_TESTS="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/borrowed_source_publication_new_argument_tests.rs"
rg -q 'NewArgument' "$BORROWED_FORMAL_USE_SRC"
rg -q 'new_argument_kind' "$NEW_ARGUMENT_DRAFT"
rg -q 'dominated_new_argument_admits_exact_site_and_ordinal' "$BORROWED_FORMAL_USE_TESTS"
rg -q 'new_argument_rejects_unguarded_and_inside_arm_uses' "$BORROWED_FORMAL_USE_TESTS"
rg -q 'borrowed_ordinary_new_argument_uses_v1' "$FINISHED_SOURCE_PROJECTION"
rg -q 'has_borrowed_ordinary_entry_v1' "$FINISHED_SOURCE_PROJECTION"
rg -q 'ctor-coverage' "$BORROWED_CALL_USES"
rg -q 'borrowed_use_dominated_ctor_view_passes' "$BORROWED_CALL_USES_CTOR_VIEW_TESTS"
rg -q 'borrowed_use_rejects_undominated_and_drifting_ctor_view' "$BORROWED_CALL_USES_CTOR_VIEW_TESTS"
rg -q 'dominated_new_argument_view_publishes_tagged_birth_actual' "$BORROWED_SOURCE_PUBLICATION_NEW_ARGUMENT_TESTS"
rg -q 'nonliteral_i64_birth_actuals_publish_integer_payload_tag' "$BORROWED_SOURCE_PUBLICATION_NEW_ARGUMENT_TESTS"
rg -q 'scalar_actual_kind' "$PHYS_ABI"
rg -q 'birth-actual-tagged-duplicate' "$PHYS_ABI"
rg -q 'tagged' "$LIFECYCLE_C_CALLS"
rg -q 'lv4_borrowed_tagged_formal' "$LIFECYCLE_V4_INDEXED"
test -f "$CTOR_VIEW_EXE_TEST"

# MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-RESULT-S0: the
# borrowed result class extends to the nullable-handle lane — every explicit
# value-return site is `return null` or `return new ..`, the caller-side
# `local h = recv.m(..)` edge mints `NullableHandle`, and cleanup owes
# exactly one checked release. Child return-type inference iterates blocks
# in sorted order so `null`/`new` multi-exit callees infer deterministically.
HOME_LOCAL_CALL_FLOW="$ROOT_DIR/src/mir/resolved_semantics/home_local_call_flow.rs"
NEW_PREFIX_BRANCH="$ROOT_DIR/src/mir/resolved_semantics/home_new_prefix_branch.rs"
FUNCTION_CONTROL_NEW_HOMES="$ROOT_DIR/src/mir/resolved_control_flow/function_control_new_homes.rs"
LOWERING_CALLS="$ROOT_DIR/src/mir/builder/calls/lowering.rs"
TERMINAL_CALL_ADMISSION="$ROOT_DIR/src/mir/builder/ordinary_new_admission/selected/terminal_call.rs"
LEXICAL_NULLABLE_EMIT="$ROOT_DIR/src/mir/builder/ordinary_new_admission/selected/terminal_call/lexical_nullable.rs"
MAP_VALUE_COMPLETION_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/map_value_completion_tests.rs"
rg -q 'BorrowedResultClassV1' "$BORROWED_FORMAL_RESULT_SRC"
rg -q 'source-class-mixed' "$BORROWED_FORMAL_RESULT_SRC"
rg -q 'NullableObject' "$BORROWED_FORMAL_RESULT_SRC"
rg -q 'issue_lexical_nullable_local_call' "$HOME_LOCAL_CALL_FLOW"
rg -q 'LocalCallResultClassV1::Nullable' "$HOME_LOCAL_CALL_FLOW"
rg -q 'lexical_nullable_result_call' "$COSEAL_ISSUE_LEXICAL"
rg -q 'local_lexical_nullable_call' "$FUNCTION_CONTROL_NEW_HOMES"
rg -q 'emit_local_lexical_nullable' "$TERMINAL_CALL_ADMISSION"
rg -q 'NullableHandle' "$LEXICAL_NULLABLE_EMIT"
rg -q 'ordered.sort_by_key' "$LOWERING_CALLS"
rg -q 'borrowed_nullable_result_lexical_call_publishes_checked_release' "$BORROWED_SOURCE_PUBLICATION_NEW_ARGUMENT_TESTS"
rg -q 'borrowed_nullable_result_rejects_mixed_and_unproved_returns' "$BORROWED_SOURCE_PUBLICATION_NEW_ARGUMENT_TESTS"
rg -q 'borrowed_nullable_result_frontiers_stay_fail_closed' "$BORROWED_SOURCE_PUBLICATION_NEW_ARGUMENT_TESTS"

# MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-FIELDREAD-S0: a
# dominating `h == null` guard whose null arm terminates narrows a
# `ReceivedNullable` local for field reads — the mark lives only on that
# stored class, `store()` drops it on rebinding, and joins intersect marks
# across surviving sides. Admission reuses the field issuer (request
# `nullable`, class from the sealed `NullableObject` claim — never MIR
# types); the physical owner stays `take_*_field_read` and cleanup stays
# one checked release per exit.
TERMINAL_RELATION="$ROOT_DIR/src/mir/resolved_semantics/home_terminal_relation.rs"
FIELD_READS_LEDGER="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_field_reads.rs"
TERMINAL_HOME="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_terminal_home.rs"
rg -q 'nonnull: std::collections::BTreeSet<BindingRefV1>' "$LOCAL_FLOW"
rg -q 'fn mark_nonnull' "$LOCAL_FLOW"
rg -q 'fn field_home' "$LOCAL_FLOW"
rg -q 'FieldReadReceiverV1::ReceivedNullable' "$LOCAL_FLOW" "$LOCAL_FIELD_SRC"
rg -q 'pub\(crate\) nullable: bool' "$LOCAL_FIELD_SRC"
rg -q 'fn null_guarded_local' "$NEW_PREFIX_BRANCH"
rg -q 'join_locals.mark_nonnull' "$NEW_PREFIX_BRANCH"
rg -q 'locals.field_home' "$TERMINAL_RELATION"
rg -q 'fn nullable_received_result_class' "$COSEAL_ISSUE_LEXICAL"
rg -q 'nullable_received_result_class' "$COSEAL_ISSUE"
rg -q 'fn nullable_result_integer_field' "$TERMINAL_HOME"
rg -q 'local\.installs\(row\.home\)' "$FIELD_READS_LEDGER"
rg -q 'nullable_field_read_publishes_guarded_object_field_get' "$BORROWED_SOURCE_PUBLICATION_NEW_ARGUMENT_TESTS"
rg -q 'nullable_field_read_stays_fail_closed' "$BORROWED_SOURCE_PUBLICATION_NEW_ARGUMENT_TESTS"

# MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-I64RESULT-S0: an
# unannotated borrowed callee whose every explicit value-return site is
# source-proven Integer/exact-I64 projects the executable I64 result while
# the declaration stays `Unannotated` — the same Completion/site
# corroboration the explicit `: i64` lane demands; Void, other annotations
# and other domains stay rejected.
BORROWED_SOURCE_PUBLICATION_I64_RESULT_TESTS="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/borrowed_source_publication_i64_result_tests.rs"
rg -q 'DeclaredFunctionResultContractV1::Unannotated' "$BORROWED_FORMAL_RESULT_SRC"
rg -q 'proof.contract_corroborated' "$BORROWED_FORMAL_RESULT_SRC"
rg -q 'borrowed_call_result_accepts_unannotated_complete_i64_source' "$BORROWED_FORMAL_RESULT_TESTS"
rg -q 'borrowed_call_result_keeps_unannotated_i64_bounded' "$BORROWED_FORMAL_RESULT_TESTS"
rg -q 'unannotated_borrowed_i64_result_publishes_call_forms' "$BORROWED_SOURCE_PUBLICATION_I64_RESULT_TESTS"
rg -q 'parameter_field_frontiers_stay_fail_closed' "$BORROWED_SOURCE_PUBLICATION_NEW_ARGUMENT_TESTS"

# MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-ROOTSOURCE-S0:
# the finalized root handoff keeps verified main source for checked birth
# actuals and local-call inventories independently of terminal-map presence
# (union of terminals/actuals/lexical groups, explicit owner field), and a
# proven-i64 bound return mints its own `I64Scalar` source relation so the
# existing result boundary publishes it — an empty terminal map with no ABI
# still fails `retained-root-result-missing`.
BORROWED_SOURCE_PUBLICATION_ROOT_SOURCE_TESTS="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/borrowed_source_publication_root_source_tests.rs"
FINALIZED_ROOT_HANDOFF="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/finalized_root_handoff.rs"
TERMINAL_RESULT_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_terminal_result_tests.rs"
rg -q 'TerminalI64ScalarReturnV1' "$TERMINAL_RELATION"
rg -q 'TerminalI64ScalarReturnV1::issue' "$NEW_PREFIX_TERMINAL"
rg -q 'FinalizedRootResultAbiV1::I64ScalarReturn' "$LOCAL_COMMIT"
rg -q 'has_lexical_local_calls' "$FINALIZED_ROOT_HANDOFF"
rg -q 'bound_i64_scalar_issues_exact_terminal_relation' "$TERMINAL_RESULT_TESTS"
rg -q 'empty_terminal_root_source_keeps_verified_owner_without_abi' "$FINISHED_COPY_TESTS"
rg -q 'bound_i64_returns_publish_through_root_source' "$BORROWED_SOURCE_PUBLICATION_ROOT_SOURCE_TESTS"
rg -q 'unproven_bound_returns_stay_fail_closed' "$BORROWED_SOURCE_PUBLICATION_ROOT_SOURCE_TESTS"

for file in "$SCALAR_EXPR" "$LOCAL_FIELD_SRC" "$SCALAR_CLAIM_TESTS" "$SCALAR_EMIT_TESTS" "$FIELD_BATCH_TESTS" "$COSEAL_ISSUE_SOURCE" "$COSEAL_LEDGER" "$MAIN_ROOT" "$PIN_TEST" "$ARITY_PIN_TEST" "$ENTRY_PORT" "$COSEAL" "$COSEAL_ISSUE" "$COSEAL_TESTS" "$RAW_CLAIM" "$NEW_EXPR" "$CTOR_SCOPE" "$NO_EXIT" "$LOOP_BUILDER" "$LOOP_COND_FACTS" "$REJECT_REASON" "$LOOP_COND_BC" "$BC_ITEM" "$ITEMS_SRC" "$COND_UPDATE_TESTS" "$COND_UPDATE_FACADE" "$HELPERS_LOWER" "$ITEMS_TESTS" "$TESTKIT" "$STATIC_INGRESS" "$STATIC_OWNER_POLICY" "$MEMBER_ROUTE" "$CALLS_MOD" "$PHYSICAL_BRIDGE" "$SEL_ARG" "$LOCAL_FLOW" "$NEW_PREFIX" "$NEW_PREFIX_ARGS" "$NEW_PREFIX_SCAN" "$NEW_PREFIX_TERMINAL" "$ORD_ARGS" "$COSEAL_HELPERS" "$SEL_EMIT" "$SEL_EMIT_ARGS" "$PHYS_ABI" "$EMIT_VALID" "$BRAND_TESTS" "$ENV_DIRECT_TESTS" "$DESCENT_TESTKIT" "$DESCENT_TESTS" "$ITEM_SITE" "$MAP_LOCAL_TESTS" "$COMPOSITE_PROJECTION" "$LOOP_BREAK_FACTS" "$ROUTE_PREDICATES" "$CALLABLE_ROUTE" "$DEAD_ROUTE_TESTS" "$CORE_METHOD_SRC" "$NAMED_ARRAY_SRC" "$NAMED_ARRAY_METHOD_TESTS" "$CONTRACT_ISSUER" "$LEDGER_CONTRACT_TESTS" "$DECLARED_INSTANCE_LOCATOR" "$CORE_EFFECT_PLAN" "$SOURCE_METHOD_PORT" "$UNIFIED_EMITTER" "$LOOP_DECLARED_TESTS" "$ASSOC_INPUT" "$NORMALIZER_COMMON" "$RAW_LOOP_PORT" "$RAW_LOOP_ENTRY" "$CALLABLE_LOWERING_STATE" "$CALLABLE_STATE_SOURCE_PREP" "$LIFECYCLE_PHYSICAL_PROGRAM" "$LIFECYCLE_PHYSICAL_PROJECTION" "$LOOP_VALUE_PUBLICATION" "$BACKEND_VIEW_TESTS" "$BACKEND_VIEW_DRIFT_TESTS" "$BACKEND_VIEW_INTRINSIC_TESTS" "$ROOT_LIFECYCLE_TESTS" "$ROOT_MAIN_SELECTION_TESTS" "$LOOP_PIPELINE_TESTS" "$LOOP_SCOPE_TESTS" "$ROUTE_ITEMS_SRC" "$ROUTE_CALL_FREE_TESTS" "$ROUTE_TEST_SURFACE" "$SOURCE_LOOP_BRIDGE" "$LOOP_COND_FACTS_SRC" "$GENERIC_FACTS_ISSUER" "$LEXICAL_INSTANCE_CALL_SRC" "$LEXICAL_INSTANCE_PROVENANCE_SRC" "$LEXICAL_INSTANCE_SOURCE_SRC" "$BORROWED_FORMAL_USE_SRC" "$BORROWED_FORMAL_USE_TESTS" "$BORROWED_FORMAL_SOURCE_SRC" "$BORROWED_FORMAL_SOURCE_TESTS" "$BORROWED_FORMAL_ACTUAL_SRC" "$BORROWED_FORMAL_ACTUAL_TESTS" "$BORROWED_FORMAL_ENTRY_SRC" "$BORROWED_FORMAL_ENTRY_TESTS" "$BORROWED_FORMAL_RESULT_SRC" "$BORROWED_FORMAL_RESULT_TESTS" "$BORROWED_FORMAL_TERMINAL_TESTS" "$BORROWED_ENTRY_VALUES" "$BORROWED_ALIAS_MATERIALIZATION" "$BORROWED_ALIAS_MATERIALIZATION_TESTS" "$BORROWED_ALIAS_STATE_TESTS" "$PHYSICAL_COPY_BOUNDARY" "$BORROWED_COPY_BOUNDARY" "$BORROWED_COPY_BOUNDARY_TESTS" "$BORROWED_ENTRY_STATE" "$BORROWED_ENTRY_STATE_TESTS" "$BORROWED_ENTRY_SCOPE_TESTS" "$BORROWED_ACTUAL_OBSERVER" "$LEXICAL_INSTANCE_CALL_TESTS" "$VALUE_METHOD_CALL_SRC" "$FIELD_WRITE_CLAIM_SRC" "$RESULT_CLASS_CLAIM_SRC" "$LOCAL_COMMIT" "$LOCAL_COMMIT_CALL_RECV" "$LOCAL_COMMIT_PREP" "$LOCAL_COMMIT_HANDLE" "$COSEAL_ISSUE_LEXICAL" "$DIRECT_CALL_PHYSICAL_TESTS" "$DIRECT_CALL_HANDLE_TESTS" "$ROOT_CALL_ENTRY" "$ROOT_CALL_ENTRY_VALIDATION" "$ROOT_CALL_ENTRY_TESTS" "$RAW_CLAIM_TERMINAL_CALL" "$LEXICAL_I64_EMIT" "$LEXICAL_I64_PROJECTION" "$LOCAL_CALL_GROUP" "$LOCAL_CALL_GROUP_TESTS" "$FINISHED_SOURCE_PROJECTION" "$FINISHED_SOURCE_PROJECTION_TESTS" "$FINALIZED_CALL_VISITOR" "$BORROWED_CALL_INCOMING" "$BORROWED_CALL_USES" "$BORROWED_CALL_USES_TESTS" "$FINISHED_COPY_TESTS" "$BORROWED_CARRIER_JSON_TESTS" "$BORROWED_USE_PROJECTION_TESTS" "$LIFECYCLE_CALL_JSON" "$BORROWED_WIRE_TESTS" "$BORROWED_SOURCE_PUBLICATION_TESTS" "$LIFECYCLE_PROGRAM_JSON" "$LIFECYCLE_C_FORMALS" "$LIFECYCLE_C_CALLS" "$LIFECYCLE_C_PARSER" "$FIELD_REF_PROJECTION" "$EXACT_NUMERIC_LIFECYCLE" "$LOCAL_COPY_PROVENANCE" "$LOCAL_COPY_PROVENANCE_TESTS" "$BORROWED_PHYSICAL_PROJECTION" "$BORROWED_PHYSICAL_PROJECTION_TESTS" "$LEXICAL_I64_FIXTURE" "$LEXICAL_I64_EMIT_TESTS" "$EXACT_LEXICAL_READ" "$EXACT_LEXICAL_READ_TESTS" "$RECURSIVE_CHILD_LOWERING" "$RECURSIVE_CHILD_DISPOSITION" "$ROOT_RESULT_NEW_TESTS" "$ENTRY_HOME" "$ENTRY_HOME_TESTS" "$HOME_ABI" "$PKG_ISSUER" "$SET_ELEMENT_DRAFT" "$FAULT_CHECKED_ARRAY" "$NEW_ARGUMENT_DRAFT" "$BORROWED_CALL_USES_CTOR_VIEW_TESTS" "$BORROWED_SOURCE_PUBLICATION_NEW_ARGUMENT_TESTS" "$HOME_LOCAL_CALL_FLOW" "$NEW_PREFIX_BRANCH" "$FUNCTION_CONTROL_NEW_HOMES" "$LOWERING_CALLS" "$TERMINAL_CALL_ADMISSION" "$LEXICAL_NULLABLE_EMIT" "$MAP_VALUE_COMPLETION_TESTS" "$TERMINAL_RELATION" "$FIELD_READS_LEDGER" "$TERMINAL_HOME" "$BORROWED_SOURCE_PUBLICATION_I64_RESULT_TESTS" "$BORROWED_SOURCE_PUBLICATION_ROOT_SOURCE_TESTS" "$FINALIZED_ROOT_HANDOFF" "$TERMINAL_RESULT_TESTS"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    echo "[$TAG] source reached hard 800-line boundary: ${file#"$ROOT_DIR/"}=$lines" >&2
    exit 1
  fi
done

echo "[$TAG] ok"
