# Private continuation of mirbuilder_qualified_route_scope_guard.sh.
# Its caller supplies the existing variables; this is not a second entry.
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
BORROWED_SOURCE_PUBLICATION_PARAM_FIELD_TESTS="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/borrowed_source_publication_param_field_tests.rs"
rg -q 'DeclaredFunctionResultContractV1::Unannotated' "$BORROWED_FORMAL_RESULT_SRC"
rg -q 'proof.contract_corroborated' "$BORROWED_FORMAL_RESULT_SRC"
rg -q 'borrowed_call_result_accepts_unannotated_complete_i64_source' "$BORROWED_FORMAL_RESULT_TESTS"
rg -q 'borrowed_call_result_keeps_unannotated_i64_bounded' "$BORROWED_FORMAL_RESULT_TESTS"
rg -q 'unannotated_borrowed_i64_result_publishes_call_forms' "$BORROWED_SOURCE_PUBLICATION_I64_RESULT_TESTS"
rg -q 'parameter_field_frontiers_stay_fail_closed' "$BORROWED_SOURCE_PUBLICATION_PARAM_FIELD_TESTS"

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

# MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-USESIZE-T0:
# BoxShape only — the operation-use operand classifiers (checked compare,
# ordered add, Normal-Integer sibling proof, call-argument pre-pass loan and
# dominance helpers) move verbatim into a private `operands` child of the
# same borrowed-use owner; the parent keeps the draft types, incoming/forward
# join and the main use loop, and re-exports the classifiers for its
# existing `array_element`/`new_argument` children.
OPERANDS_DRAFT="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_borrowed_formal_use_operands.rs"
rg -q 'fn compare_operand_kind' "$OPERANDS_DRAFT"
rg -q 'fn normal_integer_operand' "$OPERANDS_DRAFT"
rg -q 'fn add_operand_kind' "$OPERANDS_DRAFT"
rg -q 'fn is_call_argument' "$OPERANDS_DRAFT"
rg -q 'fn use_dominated_by_if' "$OPERANDS_DRAFT"
rg -q 'fn sequence_ordinal' "$OPERANDS_DRAFT"
rg -q 'mod operands' "$BORROWED_FORMAL_USE_SRC"
rg -q 'use operands::' "$BORROWED_FORMAL_USE_SRC"

# MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-NULLCOMPARE-S0:
# the borrowed tagged formal may serve an exact `== null` direct-if
# condition in either operand order. Source admission mints the semantic
# `Equal(Dynamic, Null)` envelope (TrivialBool, NonSuspending), the
# published scan admits the operand only beside the exact ConstValue::Null
# producer, and the sole physical owner spells the row
# `borrowed_null_compare` — the carrier's kind or payload lane answers the
# equality, never the Integer view.
BORROWED_CALL_USES_NULL_COMPARE="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/compiled_entry_contract/borrowed_call_uses_null_compare.rs"
BORROWED_CALL_USES_NULL_COMPARE_TESTS="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/compiled_entry_contract/borrowed_call_uses_null_compare_tests.rs"
BORROWED_SOURCE_PUBLICATION_NULL_COMPARE_TESTS="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/borrowed_source_publication_null_compare_tests.rs"
NULL_COMPARE_EXE_TEST="$ROOT_DIR/lang/c-abi/tests/published_lifecycle_v4_null_compare_execution_test.py"
rg -q 'DynamicOperatorFamilyV1::Equal' "$DYNAMIC_OPERATOR_ISSUER"
rg -q 'DynamicOperatorValueClassV1::Null' "$DYNAMIC_OPERATOR_ISSUER"
rg -q 'DynamicOperatorSuspensionV1::NonSuspending' "$DYNAMIC_OPERATOR_ISSUER"
rg -q 'NullCompareOperand' "$BORROWED_FORMAL_USE_SRC"
rg -q 'fn null_compare_operand_kind' "$OPERANDS_DRAFT"
rg -q 'null_compare_admits_direct_if_equal_with_null_sibling' "$BORROWED_FORMAL_USE_TESTS"
rg -q 'null_compare_rejects_wrong_operator_sibling_and_position' "$BORROWED_FORMAL_USE_TESTS"
rg -q 'borrowed_ordinary_null_compare_uses_v1' "$FINISHED_SOURCE_PROJECTION"
rg -q 'null-coverage' "$BORROWED_CALL_USES_NULL_COMPARE"
rg -q 'borrowed_use_null_compare_view_passes_in_either_operand_order' "$BORROWED_CALL_USES_NULL_COMPARE_TESTS"
rg -q 'borrowed_use_rejects_null_compare_operator_and_sibling_drift' "$BORROWED_CALL_USES_NULL_COMPARE_TESTS"
rg -q 'borrowed_use_null_compare_coverage_is_per_admission' "$BORROWED_CALL_USES_NULL_COMPARE_TESTS"
rg -q 'borrowed_null_compare' "$LIFECYCLE_PROGRAM_JSON" "$LIFECYCLE_C_PARSER" "$LIFECYCLE_V4_INDEXED" "$LIFECYCLE_V4_EMIT"
rg -q 'lv4_null_lane_present' "$LIFECYCLE_V4_INDEXED"
rg -q 'null_compare_publishes_the_dedicated_physical_row' "$BORROWED_SOURCE_PUBLICATION_NULL_COMPARE_TESTS"
rg -q 'non_null_equalities_stay_fail_closed_before_publication' "$BORROWED_SOURCE_PUBLICATION_NULL_COMPARE_TESTS"
test -f "$NULL_COMPARE_EXE_TEST"

# MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-NULLACTUAL-S0:
# borrowed call actuals admit the exact `null` literal — wire tag 0 spelled
# beside its sole `const_null` producer, never a zero payload — and a
# caller-owned ReceivedNullable, a `NullableObject(C)` call result whose
# sealed claim is the sole class authority and which crosses the wire as
# `nullable_typed_object` so the callee's prologue re-proves the live
# tag-3 or void tag-0 pair itself. Aliases, foreign/malformed/consumed
# producers and forged rows stay rejected before publication; the caller
# releases the live result exactly once and the callee End stays zero.
BORROWED_SOURCE_PUBLICATION_NULLABLE_ACTUAL_TESTS="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/borrowed_source_publication_nullable_actual_tests.rs"
NULLABLE_ACTUAL_EXE_TEST="$ROOT_DIR/lang/c-abi/tests/published_lifecycle_v4_nullable_actual_execution_test.py"
rg -q 'BorrowedFormalActualSourceV1::Null' "$BORROWED_FORMAL_ACTUAL_SRC"
rg -q 'BorrowedFormalActualSourceV1::ReceivedNullable' "$BORROWED_FORMAL_ACTUAL_SRC"
rg -q 'nullable-class-unavailable' "$BORROWED_FORMAL_ACTUAL_SRC"
rg -q 'BorrowedCallActualValueV1::Null' "$BORROWED_ACTUAL_OBSERVER"
rg -q 'BorrowedCallActualValueV1::ReceivedNullable' "$BORROWED_ACTUAL_OBSERVER"
rg -q 'is_received_nullable' "$LOCAL_FLOW"
rg -q 'Source::Null' "$LEXICAL_I64_EMIT" "$BORROWED_PHYSICAL_PROJECTION"
rg -q 'nullable_typed_object' "$LIFECYCLE_CALL_JSON" "$LIFECYCLE_C_CALLS" "$LIFECYCLE_V4_INDEXED" "$LIFECYCLE_V4_EMIT"
rg -q 'ordinary_nullable_handle' "$LIFECYCLE_C_FORMALS"
rg -q 'fault-unwind chain' "$EMIT_VALID"
rg -q 'exact_null_actual_selects_the_null_source_not_a_scalar_payload' "$BORROWED_FORMAL_ACTUAL_TESTS"
rg -q 'null_and_nullable_actuals_publish_their_wire_forms' "$BORROWED_SOURCE_PUBLICATION_NULLABLE_ACTUAL_TESTS"
rg -q 'non_null_domains_keep_their_own_tags' "$BORROWED_SOURCE_PUBLICATION_NULLABLE_ACTUAL_TESTS"
test -f "$NULLABLE_ACTUAL_EXE_TEST"

# MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-FIELDSIZE-T0:
# BoxShape only — the field-read batch issuer/stager
# (prove_local_field_read_batch, stage_local_field_read_batch) moves
# verbatim into a private `field_batch` child of the same issue_source
# owner; the parent re-exports both so `source_claims::` callers and the
# staged-read tests resolve unchanged. No predicate, evaluation order,
# error arm or signature changed.
rg -q 'mod field_batch' "$COSEAL_ISSUE_SOURCE"
rg -q 'use field_batch::' "$COSEAL_ISSUE_SOURCE"
rg -q 'source_claims::prove_local_field_read_batch' "$COSEAL_ISSUE"
rg -q 'source_claims::stage_local_field_read_batch' "$COSEAL_ISSUE"

# MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-PARAMFIELD-S0:
# a borrowed `Handle`/`OpaqueHandle` formal admits one dominated
# `formal.field` read after the exact `formal == null` terminating guard.
# The use draft seals `FieldReadOperand` beside a `null_guards` pre-pass;
# the flow marks the formal non-null only on the surviving successor and
# spells it `GuardedFormal`; the co-sealed incoming-call evidence mints one
# `BorrowedFormalObjectViewV1` only when every actual agrees on one class —
# null-only or conflicting callers mint no view and the callee stays on
# the bounded sibling lane. Only a view-carrying formal field-read target
# takes the verified walk so the single completion authority both proves
# the guard and issues the read; the sole physical owner emits exactly one
# `object_field_get` against the formal's `object_view` param key — no
# ownership transfer, no callee `End`.
BORROWED_FORMAL_USE_FIELD_READ="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_borrowed_formal_use_field_read.rs"
BORROWED_CALL_USES_OBJECT_FIELD="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/compiled_entry_contract/borrowed_call_uses_object_field.rs"
BORROWED_CALL_USES_CLOSURE="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/compiled_entry_contract/borrowed_call_uses_closure.rs"
COMPILED_ENTRY_CONTRACT="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/compiled_entry_contract.rs"
rg -q 'FieldReadOperand' "$BORROWED_FORMAL_USE_SRC"
rg -q 'fn field_read_operand_kind' "$BORROWED_FORMAL_USE_FIELD_READ"
rg -q 'FieldReadReceiverV1::GuardedFormal' "$LOCAL_FLOW"
rg -q 'fn mark_nonnull' "$LOCAL_FLOW"
rg -q 'BorrowedFormalObjectViewV1' "$BORROWED_FORMAL_SOURCE_SRC"
rg -q 'fn formal_object_view' "$BORROWED_FORMAL_SOURCE_SRC"
rg -q 'fn formal_field_read_target' "$BORROWED_FORMAL_SOURCE_SRC"
rg -q 'fn path_dominates' "$BORROWED_CALL_USES_OBJECT_FIELD"
rg -q 'fn object_view_values' "$BORROWED_CALL_USES_OBJECT_FIELD"
rg -q 'fn verify_published' "$BORROWED_CALL_USES_CLOSURE"
rg -q 'borrowed_object_views' "$COMPILED_ENTRY_CONTRACT"
rg -q '"object_view"' "$LIFECYCLE_CALL_JSON" "$LIFECYCLE_C_FORMALS"
rg -q 'lv4_borrowed_object_formal' "$LIFECYCLE_V4_INDEXED"
rg -q 'guarded_formal_field_read_publishes_object_view' "$BORROWED_SOURCE_PUBLICATION_PARAM_FIELD_TESTS"
PARAM_FIELD_EXE_TEST="$ROOT_DIR/lang/c-abi/tests/published_lifecycle_v4_param_field_execution_test.py"
test -f "$PARAM_FIELD_EXE_TEST"

# MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-FIELDRESULT-S0:
# the same guarded formal field read may serve a direct `return formal.f`
# I64 result — the flow's `field_home` keeps a Parameter-rooted handle live
# only on the non-null-marked successor, the `field_is_integer` closures in
# both the verified walk and the probe resolve the formal's class through
# the co-sealed `BorrowedFormalObjectViewV1` (same canonical `i64`
# declaration authority), and the result classifier admits the exact
# FieldAccess only beside the draft's `FieldReadOperand` row and the view.
# The physical lane is unchanged: the existing `I64Field` terminal relation
# consumes the exact staged read, now accepting a view-proven formal home.
rg -q '!self.nonnull.contains' "$LOCAL_FLOW"
rg -q 'source.formal_object_view' "$COSEAL_ISSUE" "$COSEAL_ISSUE_SOURCE"
rg -q 'fn guarded_formal_i64_field' "$BORROWED_FORMAL_RESULT_SRC"
rg -q 'nullable_result_integer_field' "$COSEAL"
rg -q 'formal_object_view' "$FIELD_READS_LEDGER"
rg -q 'fr-object-field' "$BORROWED_SOURCE_PUBLICATION_PARAM_FIELD_TESTS"

# MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-PARAMFIELD-ACCEPTANCE-R0:
# the selected frontier shapes ride the real Rust -> physical JSON -> C ->
# OBJ/link/EXE path: unused formal/I64 result, local-new TypedHome call,
# exact null guard (literal-null and object actuals), guarded field
# initializer/terminal and literal-null/received-nullable actuals, each in
# its normal, null-state or injected-fault lane while forged rows reject.
BORROWED_SOURCE_PUBLICATION_PARAM_ACCEPTANCE_TESTS="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/borrowed_source_publication_param_field_acceptance_tests.rs"
PARAM_ACCEPTANCE_EXE_TEST="$ROOT_DIR/lang/c-abi/tests/published_lifecycle_v4_param_field_acceptance_execution_test.py"
rg -q 'param_field_acceptance_shapes_publish_their_issued_inputs' "$BORROWED_SOURCE_PUBLICATION_PARAM_ACCEPTANCE_TESTS"
rg -q 'mod param_field_acceptance_tests' "$BORROWED_SOURCE_PUBLICATION_TESTS"
test -f "$PARAM_ACCEPTANCE_EXE_TEST"

# MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-FIELDOPERAND-S0:
# order comparisons `<`/`>`/`<=`/`>=` join the scalar vocabulary as
# integer-operand -> Bool expressions, but profile selection stays narrow:
# only a GuardedFormal field leaf selects the staged field-read root,
# `me.` receivers are expressible operands without triggering selection,
# and every other provenance returns `None` so the whole root falls back
# to the original compare lane instead of partially selecting. The staged
# reads still ride the same FieldReadOperand draft, the same batch
# prover/stager and the same `take_local_field_read` physical owner —
# `handle.page_id < 0` publishes one `object_field_get` on the sealed
# object view plus `slt`, and `handle.page_id >= me.limit` carries both
# reads on one issued root plus `sge`.
rg -q 'Op::Less | Op::Greater | Op::LessEqual | Op::GreaterEqual' "$SCALAR_EXPR"
rg -q 'local_flow::FieldReadReceiverV1::GuardedFormal' "$SCALAR_EXPR"
rg -q 'scalar_expression_claims_order_compare_field_operands' "$SCALAR_CLAIM_TESTS"
rg -q 'scalar_expression_rejects_order_compare_on_object_typed_field' "$SCALAR_CLAIM_TESTS"
rg -q '"cmp-ge"' "$BORROWED_SOURCE_PUBLICATION_PARAM_FIELD_TESTS"
rg -q '"pa-guarded-nullarg-cmp"' "$BORROWED_SOURCE_PUBLICATION_PARAM_FIELD_TESTS"
rg -q "'cmp-ge': 'param-field-cmp-ge'" "$PARAM_ACCEPTANCE_EXE_TEST"

# MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-FINISHBIND-S0:
# the physical projection walks the whole captured draft graph
# (`walk_graph`), not only recorded-binding blocks, because finishing may
# merge a recorded block into an uncaptured trampoline, fold an
# equal-arm `Branch` to a `Jump`, or dead-prune a block. Destinations map
# every walked block, embedded `BasicBlockId`s in recorded instructions
# rewrite through the same map, and the expected sequence is keyed by the
# surviving walk start whenever the walk carried a binding — an absorbed
# node's content is never waived. Fail-closed tokens stay: unmapped
# binding block, contraction predecessor mismatch, foreign target,
# incoming-edge drift, forged sequence. Dead pure definitions may be
# omitted only when unrecorded, unused and uniquely defined.
PHYSICAL_BOUNDARY_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/physical_boundary_tests.rs"
ROOT_CLEANUP_GRAPH_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_cleanup_graph_tests.rs"
rg -q 'walk_graph: BTreeMap<BasicBlockId, Node>' "$PHYSICAL_COPY_BOUNDARY"
rg -q 'recorded_dsts: BTreeSet<ValueId>' "$PHYSICAL_COPY_BOUNDARY"
rg -q 'single_definitions: BTreeSet<ValueId>' "$PHYSICAL_COPY_BOUNDARY"
rg -q 'carries_binding' "$PHYSICAL_COPY_BOUNDARY"
rg -q 'jump_discriminant' "$PHYSICAL_COPY_BOUNDARY"
rg -q 'unmapped-block' "$PHYSICAL_COPY_BOUNDARY"
rg -q 'contraction-predecessor' "$PHYSICAL_COPY_BOUNDARY"
rg -q 'foreign-target' "$PHYSICAL_COPY_BOUNDARY"
rg -q 'incoming-drift' "$PHYSICAL_COPY_BOUNDARY"
rg -q 'finished-sequence' "$PHYSICAL_COPY_BOUNDARY"
rg -q 'duplicate-or-cycle' "$PHYSICAL_COPY_BOUNDARY"
rg -q 'only_original_unrecorded_unused_constants_may_disappear' "$PHYSICAL_BOUNDARY_TESTS"
rg -q 'finished_cleanup_rejects_operation_path_and_prefix_mutations' "$ROOT_CLEANUP_GRAPH_TESTS"
rg -q 'exact_original_and_three_contractions_preserve_full_release' "$ROOT_CLEANUP_GRAPH_TESTS"

# MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-NUMFIELD-S0:
# the verifier's `ObjectFieldGet` field-type arm admits any non-weak
# declared type recognized by `is_numeric_integer_type_name` — numeric
# fields share the i64 physical representation and every read reaching
# the arm is backed by a staged ledger claim, so the earlier
# `BorrowedTaggedValue` corridor carve-out is subsumed. Weak fields,
# non-numeric declared types and missing declarations stay
# `object-field-read-definition-invalid`.
VERIFIER_INVOKE="$ROOT_DIR/src/mir/verification/invoke.rs"
VERIFIER_INVOKE_TESTS="$ROOT_DIR/src/mir/verification/invoke_tests.rs"
rg -q 'crate::mir::numeric_substrate::is_numeric_integer_type_name' "$VERIFIER_INVOKE"
rg -q 'object-field-read-definition-invalid' "$VERIFIER_INVOKE"
rg -q 'definition.is_weak' "$VERIFIER_INVOKE"
rg -q 'object_field_read_admits_numeric_fields_and_rejects_other_types' "$VERIFIER_INVOKE_TESTS"

# MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-TASK4-ARTIFACT-S0:
# the `.set(index, value)` draft admits an index proven as the sole
# initializer of a `me.<ArrayBox>.get`/1 result local via the same
# `receiver_array_field` + `CORE_METHOD_CONTRACT_ROWS_V2` authority —
# `normal_integer_operand` is not widened and rebound or plain-copy
# locals stay rejected. Prefix scanning consults
# `dominated_view_sites`, the dominated-view use sites recorded at
# draft-classification time (classification output, not borrowed
# transport membership: `me.<field>` receiver callees never enter the
# lexical incoming-call map). Verified walk and probe share
# `dominated_view_use_consult_v1`; bounded sibling lanes keep the
# fail-closed `Ok(false)` stub.
NEW_PREFIX_FIELD_CALL="$ROOT_DIR/src/mir/resolved_semantics/home_new_prefix_field_call.rs"
NEW_PREFIX_FIELD_WRITE="$ROOT_DIR/src/mir/resolved_semantics/home_new_prefix_field_write.rs"
rg -q 'fn get_result_index' "$SET_ELEMENT_DRAFT"
rg -q 'receiver_array_field' "$SET_ELEMENT_DRAFT"
rg -q 'CORE_METHOD_CONTRACT_ROWS_V2' "$SET_ELEMENT_DRAFT"
rg -q 'BindingRebind' "$SET_ELEMENT_DRAFT"
rg -q 'dominated_view_sites' "$BORROWED_FORMAL_SOURCE_SRC"
rg -q 'fn dominated_view_use_at' "$BORROWED_FORMAL_SOURCE_SRC"
rg -q 'fn dominated_view_use_consult_v1' "$COSEAL_ISSUE_SOURCE"
rg -q 'view_use: &mut impl FnMut\(&OwnedExprSiteV1\) -> Result<bool, E>' "$NEW_PREFIX_FIELD_CALL"
rg -q 'view_use: &mut impl FnMut\(&OwnedExprSiteV1\) -> Result<bool, E>' "$NEW_PREFIX_FIELD_WRITE"
rg -q 'field_receiver_callee_keeps_dominated_view_sites_outside_transport' "$BORROWED_FORMAL_SOURCE_TESTS"
rg -q 'get_result_index_rejects_plain_copy_of_get_result' "$BORROWED_FORMAL_SOURCE_TESTS"

# MIRBUILDER-APP-MIMALLOC-LITE-OBJECT-FORMAL-FIELD-STORE-S0: the
# `Some(<user-class>)` field arm of `issue_construction_plan` admits a
# birth `Parameter` RHS when the declared field class resolves uniquely
# to a non-self `PlainI64NoHook` user class, and defaulted scalar
# (`i64`/`usize`) fields admit the generated-initializer overwrite —
# the dead literal releases trivially while object-field re-stores stay
# `BodyCoverageUnsupported`. `OwnedFieldResidenceV1::Provided` records
# the sole birth-attributed formal store without minting a stored-`new`
# class claim; the physical install contract accepts `Handle` demand
# only on ordinals carrying a `Parameter` store (provider ordinals keep
# the strict `Handle` requirement).
INSTANCE_CONSTRUCTION_SRC="$ROOT_DIR/src/mir/normal_callable_semantic_package/instance_construction.rs"
CONSTRUCTION_STATE_SRC="$ROOT_DIR/src/mir/builder/normal_callable_construction_state.rs"
COSEAL_ISSUE_CHILDREN="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_coseal_issue_source_owned_children.rs"
RESULT_CLAIM_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_result_claim_tests.rs"
rg -q 'has_stored_field_initializer' "$INSTANCE_CONSTRUCTION_SRC"
rg -q 'OwnedFieldResidenceV1::Provided' "$FIELD_WRITE_CLAIM_SRC"
rg -q 'OwnedFieldResidenceV1::Provided' "$COSEAL_ISSUE_CHILDREN"
rg -q 'let provided = plan.stores' "$CONSTRUCTION_STATE_SRC"
rg -q 'provided_parameter_object_store_seals_and_prepares' "$RESULT_CLAIM_TESTS"
rg -q 'provided_parameter_store_rejects_builtin_and_non_plain_fields' "$RESULT_CLAIM_TESTS"
rg -q 'provided_parameter_store_rejects_copied_local' "$RESULT_CLAIM_TESTS"
rg -q 'defaulted_scalar_field_accepts_birth_overwrite' "$RESULT_CLAIM_TESTS"
rg -q 'defaulted_object_field_rejects_birth_overwrite' "$RESULT_CLAIM_TESTS"
rg -q 'page_heap_fixture_result_claim_census' "$RESULT_CLAIM_TESTS"

# MIRBUILDER-APP-MIMALLOC-LITE-FORMAL-FORWARD-RESULT-S0: an explicit
# `formal: <ordinary-box>` declaration resolves to the existing ordinary
# box identity in the callable parameter contract issuer — `DeclaredObject`
# is a kind arm in the existing catalog with the same Handle demand/carrier
# as `OpaqueHandle`; unresolvable or unadmitted names stay
# `UnsupportedDeclaredType`. `return <opaque formal>` mints the class-free
# `NullableForwarded{ordinal}` identity claim, as does an ordinary declared
# formal. Caller-side substitution preserves the original borrowed ordinal,
# never an owned class; a class-free claim mints no receiver
# observation. The nullable emission lane admits the `Handle` argument
# kind under that issuer proof, and the physical boundary derives expected
# incoming edges from all draft edges under the destination projection so
# contracted bridge blocks cannot drop edges.
PARAM_CONTRACT_MODEL="$ROOT_DIR/src/mir/callable_parameter_contract/model.rs"
PARAM_CONTRACT_ISSUER="$ROOT_DIR/src/mir/callable_parameter_contract/issuer.rs"
PARAM_CONTRACT_TESTS="$ROOT_DIR/src/mir/callable_parameter_contract/tests.rs"
RECEIVER_OBSERVATION_SRC="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_receiver_call_observation.rs"
PHYSICAL_BOUNDARY_SRC="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/physical_boundary.rs"
SEL_TERMINAL_CALL="$ROOT_DIR/src/mir/builder/ordinary_new_admission/selected/terminal_call.rs"
rg -q 'DeclaredObject' "$PARAM_CONTRACT_MODEL" "$PARAM_CONTRACT_ISSUER" "$RECEIVER_OBSERVATION_SRC"
rg -Fq 'class equality cannot create a received_nullable Home' "$RESULT_CLAIM_TESTS"
rg -q 'NullableForwarded' "$RESULT_CLASS_CLAIM_SRC" "$RESULT_CLAIM_TESTS"
rg -q 'ForwardFormal' "$RESULT_CLASS_CLAIM_SRC"
rg -q 'SelectedNewArgumentKindV1::Handle \{ binding \}' "$SEL_TERMINAL_CALL"
rg -q 'all_edges' "$PHYSICAL_BOUNDARY_SRC"
rg -q 'opaque_formal_return_mints_nullable_forwarded_claim' "$RESULT_CLAIM_TESTS"
rg -q 'declared_formal_return_preserves_borrowed_nullable_identity' "$RESULT_CLAIM_TESTS"
rg -q 'forwarded_formal_preserves_caller_ordinal_without_owned_result' "$RESULT_CLAIM_TESTS"
rg -q 'rebound_formal_never_claims_incoming_result_identity' "$RESULT_CLAIM_TESTS"
rg -q 'binding_is_unrebound' "$RESULT_CLASS_CLAIM_SRC"
rg -q 'page_heap_fixture_forwarded_formal_claims' "$RESULT_CLAIM_TESTS"
rg -q 'page_heap_fixture_composes_allocate_and_realloc' "$RESULT_CLAIM_TESTS"
rg -q 'page_heap_fixture_observes_me_allocate_and_realloc' "$RESULT_CLAIM_TESTS"
rg -q 'ordinary_box_declaration_projects_declared_object' "$PARAM_CONTRACT_TESTS"
rg -q 'unsupported_explicit_type_rejects_without_opaque_fallback' "$PARAM_CONTRACT_TESTS"
rg -q 'unresolved_declared_box_name_still_rejects' "$PARAM_CONTRACT_TESTS"
rg -q 'unadmitted_declared_box_name_still_rejects' "$PARAM_CONTRACT_TESTS"

# MIRBUILDER-DECLARED-OBJECT-TAGGED-BORROW-SOURCE-S0: source class/null
# constraints share the original borrow graph, never numeric or Home authority.
DECLARED_BORROW_SOURCE_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_declared_borrow_source_tests.rs"
rg -q 'is_ordinary_borrowed_handle' "$PARAM_CONTRACT_MODEL" "$BORROWED_FORMAL_USE_SRC"
rg -q 'numeric_origins' "$BORROWED_FORMAL_USE_SRC"
rg -q 'declared_formal_and_copy_share_existing_forwarded_source_and_class' "$DECLARED_BORROW_SOURCE_TESTS"
rg -q 'ignored_declared_formal_rejects_scalar_and_foreign_class_incoming' "$DECLARED_BORROW_SOURCE_TESTS"
rg -q 'array_index_requires_numeric_origin_even_with_an_exact_receiver_loan' "$DECLARED_BORROW_SOURCE_TESTS"
rg -q 'declared_rebind_and_escape_stay_outside_the_closed_source_profile' "$DECLARED_BORROW_SOURCE_TESTS"

# MIRBUILDER-APP-MIMALLOC-LITE-HEAP-CONSTRUCTION-SIZE-T0: the 971-line
# construction-state owner is split into private responsibility children —
# `emission` owns store/provider-new invoke emission and `validation` owns
# the retained transport + emitted-shape checks; the parent keeps the
# source-plan install, the take/complete lifecycle and the shared fault
# tag. Pure code motion: no predicate, order, landing or visibility change.
CONSTRUCTION_EMISSION_SRC="$ROOT_DIR/src/mir/builder/normal_callable_construction_state/emission.rs"
CONSTRUCTION_VALIDATION_SRC="$ROOT_DIR/src/mir/builder/normal_callable_construction_state/validation.rs"
rg -q 'fn emit_construction_store' "$CONSTRUCTION_EMISSION_SRC"
rg -q 'fn jump_landing' "$CONSTRUCTION_EMISSION_SRC"
rg -q 'fn validate_artifact_after_compiler_finishing' "$CONSTRUCTION_VALIDATION_SRC"
rg -q 'fn validate_bindings' "$CONSTRUCTION_VALIDATION_SRC"
rg -q 'fn lands_on' "$CONSTRUCTION_VALIDATION_SRC"
rg -q 'fn install_construction' "$CONSTRUCTION_STATE_SRC"
rg -q 'fn take_construction_store' "$CONSTRUCTION_STATE_SRC"
rg -q 'fn take_finalized_construction_validation' "$CONSTRUCTION_STATE_SRC"

# MIRBUILDER-APP-MIMALLOC-LITE-HEAP-OWNED-PROVIDER-S0: a provider `new`
# whose canonical child is `OwnedArrayFieldsNoHook` admits one bounded
# nesting level — the plan store records the child's declaration-order
# `ArrayBox` ordinals in `owned_fields`, and the co-seal pass seals the
# inventory into the existing `owned_field_children` ledger because a
# provider site mints no `local`-bound claim. In-flight cleanup is the
# MIR discharge chain (`OwnedFieldResidenceRelease` newest-first, then
# `ReclaimUnpublished`/`HomeRelease`); installed teardown stays with the
# physical `object_field_release` consumer, which walks the child layout
# row's `owned_residences` marks — `object_field_set` fault no longer
# releases the child inline. Missing, unproven, deeper, self-referential
# or drifted inventories stay unadmitted; consumers never release the
# child as plain.
NAMED_ARRAY_SOURCE_TESTS="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/named_array_source_tests.rs"
PHYS_PROGRAM_JSON_TESTS="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/physical_program_json_owned_fields_tests.rs"
rg -q 'fn owned_field_children_of' "$COSEAL_ISSUE_CHILDREN"
rg -q 'fn seal_provider_owned_children_v1' "$COSEAL_ISSUE_CHILDREN"
rg -q 'OwnedArrayFieldsNoHook' "$INSTANCE_CONSTRUCTION_SRC"
rg -q 'owned_fields' "$INSTANCE_CONSTRUCTION_SRC"
rg -q 'provider-children-missing' "$CONSTRUCTION_EMISSION_SRC"
rg -q 'provider-children-drift' "$CONSTRUCTION_EMISSION_SRC"
rg -q 'fn residence_chain' "$CONSTRUCTION_VALIDATION_SRC"
rg -q 'owned_residences' "$PHYS_ABI" "$LIFECYCLE_PROGRAM_JSON" "$LIFECYCLE_C_PARSER" "$LIFECYCLE_V4_EMIT"
rg -q 'lv4_layout_slot' "$ROOT_DIR/lang/c-abi/shims/published_mir/hako_llvmc_ffi_lifecycle_v4_index.inc"
rg -q 'ordinary_new_owned_nested_array_child_seals' "$ROOT_DIR/src/mir/normal_callable_semantic_package/brand_catalog_owned_children_tests.rs"
rg -q 'provider_owned_array_child_reaches_artifact_lane' "$NAMED_ARRAY_SOURCE_TESTS"
rg -q 'installed_owned_array_child_publishes_residence_marks' "$PHYS_PROGRAM_JSON_TESTS"

# MIRBUILDER-APP-MIMALLOC-LITE-HEAP-PROVIDER-CALL-ARG-S0: a provider `new`
# argument admits a proven `Alias.m(..)` qualified static call. The claim
# index pairs each `(caller, site)` row with its sealed `StaticBoxMethod`
# target from one seal; the Birth construction plan re-corroborates the
# resolver's `QualifiedUnbound` receiver, exact arity, Integer/Bool literal
# actuals and i64 evidence at required ordinals, sealing a
# `QualifiedStaticCall` argument row under the birth caller key. Emission
# takes the staged `(caller, site)` publication handoff through the sole
# ledger boundary, emits exactly one `Call{Global, I64}` invoke per row in
# source order faulting onto the shared reclaim head, and projects the i64
# actual through `InvokeNormalResult` for the provider `birth_call`. The
# selected `local x = new` lane refuses the row outright; physical census
# admits birth callers only for i64 results and V4 keeps map/handle Birth
# calls rejected.
QUALIFIED_STATIC_CLAIM_SRC="$ROOT_DIR/src/mir/normal_callable_semantic_package/qualified_static_call_claim.rs"
INSTANCE_CTOR_SEMANTIC="$ROOT_DIR/src/mir/normal_callable_semantic_package/instance_constructor_semantic.rs"
CHILD_LOWERING_IMPL="$ROOT_DIR/src/mir/builder/raw_invocation_source_transport/child_lowering_impl.rs"
PROVIDER_CALL_ARG_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/provider_static_call_argument_tests.rs"
rg -q 'QualifiedStaticCall' "$ORD_ARGS"
rg -q 'QualifiedStaticCallArgumentKindV1' "$ORD_ARGS"
rg -q 'fn claim_target' "$QUALIFIED_STATIC_CLAIM_SRC"
rg -q 'provider_static_claims' "$INSTANCE_CONSTRUCTION_SRC"
rg -q 'QualifiedUnbound' "$INSTANCE_CONSTRUCTION_SRC"
rg -q 'claim_target' "$INSTANCE_CONSTRUCTION_SRC"
rg -q 'birth_constructor' "$INSTANCE_CTOR_SEMANTIC"
rg -q 'static_claim_index' "$PKG_ISSUER"
rg -q 'fn take_provider_static_result_publication' "$CORE_METHOD_CONSUMER"
rg -q 'provider-static-publication-missing' "$CORE_METHOD_CONSUMER"
rg -q 'provider-static-publication-target' "$CORE_METHOD_CONSUMER"
rg -q 'ProviderCallArgEmission' "$CONSTRUCTION_STATE_SRC"
rg -q 'call_args' "$CONSTRUCTION_STATE_SRC"
rg -q 'provider-argument-handoff' "$CONSTRUCTION_EMISSION_SRC"
rg -q 'provider-argument-drift' "$CONSTRUCTION_VALIDATION_SRC"
rg -q 'provider-argument-chain' "$CONSTRUCTION_VALIDATION_SRC"
rg -q 'argument-kind-provider-only' "$SEL_EMIT_ARGS"
rg -q 'argument-call-drift' "$EMIT_VALID"
rg -q 'take_static_result_publication_handoff' "$CHILD_LOWERING_IMPL"
rg -q 'publication-relation-drift' "$CHILD_LOWERING_IMPL"
rg -q 'compiled-entry-birth-call-result' "$COMPILED_ENTRY_CONTRACT"
rg -Fq 'call_sets.get(symbol)' "$LIFECYCLE_PHYSICAL_PROJECTION"
rg -q 'QualifiedStaticCall' "$PHYS_ABI"
rg -q 'birth_i64' "$LIFECYCLE_V4_INDEXED"
rg -Fq 'ordinary || birth' "$LIFECYCLE_V4_EMIT"
rg -q 'provider_static_call_argument_seals_target_and_literal_actuals' "$PROVIDER_CALL_ARG_TESTS"
rg -q 'provider_static_call_arguments_seal_in_source_order' "$PROVIDER_CALL_ARG_TESTS"
rg -q 'provider_static_call_argument_stays_fail_closed' "$PROVIDER_CALL_ARG_TESTS"
rg -q 'provider_static_call_argument_serializes_inside_birth_unit' "$PHYS_PROGRAM_JSON_TESTS"

# Exact ordinary-candidate compatibility remains in the private source owner.
rg -Fq 'ordinary_candidate_compatible_v1' "$COSEAL_ISSUE_SOURCE"
rg -Fq 'source_claims::ordinary_candidate_compatible_v1' "$COSEAL_ISSUE"

# MIRBUILDER-EXACT-USIZE-ENTRY-PROJECTION-S1: original parameter rows only.
EXACT_USIZE_ENTRY="$ROOT_DIR/src/mir/builder/normal_callable_semantic_lowering_state/exact_parameter_entry.rs"
EXACT_USIZE_SCOPE="$ROOT_DIR/src/mir/builder/normal_callable_semantic_loan_port/source_scope.rs"
EXACT_USIZE_TESTS="$ROOT_DIR/src/mir/builder/normal_callable_semantic_loan_port/exact_usize_entry_scope_tests.rs"
rg -Fq '.parameter_contracts()' "$EXACT_USIZE_SCOPE"
rg -Fq 'ExactTrivialParameterAbiV1::USIZE' "$EXACT_USIZE_SCOPE"
rg -Fq 'exact-usize-entry/source-binding-drift' "$EXACT_USIZE_ENTRY"
rg -Fq 'exact-usize-entry/carrier-formal-drift' "$EXACT_USIZE_ENTRY"
rg -Fq 'exact-usize-entry/declaration-or-value-drift' "$EXACT_USIZE_ENTRY"
rg -Fq 'selected_exact_usize_wrapper_survives_real_source_finalization_with_exact_facts' "$EXACT_USIZE_TESTS"
rg -Fq 'selected_exact_usize_source_entry_rejects_foreign_type_and_carriers_before_projection' "$EXACT_USIZE_TESTS"
for file in "$EXACT_USIZE_ENTRY" "$EXACT_USIZE_SCOPE" "$EXACT_USIZE_TESTS"; do
  if (( $(wc -l < "$file") >= 800 )); then
    echo "[$TAG] exact usize entry owner reached hard 800-line boundary" >&2
    exit 1
  fi
done

# Declared-object tagged borrow: original ledger loan, mandatory view wire,
# and source-issued both-opt/runtime class checks stay with their existing owners.
DECLARED_BORROW_ENTRY_TESTS="$ROOT_DIR/src/mir/builder/normal_callable_semantic_lowering_state/borrowed_declared_entry_tests.rs"
DECLARED_BORROW_SOURCE_TESTS="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/borrowed_source_publication_declared_tests.rs"
DECLARED_BORROW_EXEC_TEST="$ROOT_DIR/lang/c-abi/tests/published_lifecycle_v4_declared_borrow_execution_test.py"
rg -q 'declared_header_and_value_drift_reject_before_any_entry_effect' "$DECLARED_BORROW_ENTRY_TESTS"
rg -q 'inferred_opaque_class_never_authorizes_a_declared_box_header' "$DECLARED_BORROW_ENTRY_TESTS"
rg -q 'declared_and_forwarding_only_domains_publish_exact_class_layouts' "$DECLARED_BORROW_SOURCE_TESTS"
rg -q 'borrowed_object_view' "$PHYS_ABI"
rg -Fq 'nyash.object.type_id_h' "$LIFECYCLE_V4_EMIT"
rg -q 'known-foreign-class-through-nullable-spelling' "$DECLARED_BORROW_EXEC_TEST"
rg -q 'borrowed-formal-cannot-be-released' "$DECLARED_BORROW_EXEC_TEST"
for file in "$DECLARED_BORROW_ENTRY_TESTS" "$DECLARED_BORROW_SOURCE_TESTS"; do
  if (( $(wc -l < "$file") >= 800 )); then
    echo "[$TAG] declared borrow test owner reached hard 800-line boundary" >&2
    exit 1
  fi
done

CALL_RESULT_OWNER="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_borrowed_formal_result_composition.rs"
CALL_RESULT_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_borrowed_formal_result_composition_tests.rs"
rg -q 'ground_source_results' "$CALL_RESULT_OWNER"
rg -q 'composed_result_corroboration_rejects_foreign_dependencies' "$CALL_RESULT_TESTS"
# Direct field result finishing uses the original captured boundary projection.
FIELD_RETURN_PROJECTION_SRC="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_terminal_field_return.rs"
FIELD_RETURN_PROJECTION_TESTS="$ROOT_DIR/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/terminal_field_projection_tests.rs"
FIELD_RETURN_SOURCE_TESTS="$ROOT_DIR/src/mir/compiler/normal_default_pipeline/published_backend_view/borrowed_source_publication_field_return_projection_tests.rs"
rg -q 'check_read_binding' "$FIELD_RETURN_PROJECTION_SRC"
rg -q 'full_child_finishing_rejects_foreign_field_positions_and_masked_exits' "$FIELD_RETURN_PROJECTION_TESTS"
rg -q 'root_and_borrowed_child_field_returns_survive_canonical_finishing' "$FIELD_RETURN_SOURCE_TESTS"
rg -q 'field_return_publication_rejects_swapped_null_guard_with_original_handoff' "$FIELD_RETURN_SOURCE_TESTS"
test -f "$ROOT_DIR/lang/c-abi/tests/published_lifecycle_v4_field_return_projection_execution_test.py"
for file in "$FIELD_RETURN_PROJECTION_SRC" "$FIELD_RETURN_PROJECTION_TESTS" "$FIELD_RETURN_SOURCE_TESTS" "$CALL_RESULT_OWNER" "$CALL_RESULT_TESTS"; do
  if (( $(wc -l < "$file") >= 800 )); then
    echo "[$TAG] terminal field projection owner reached hard 800-line boundary" >&2
    exit 1
  fi
done

for file in "$SCALAR_EXPR" "$LOCAL_FIELD_SRC" "$SCALAR_CLAIM_TESTS" "$SCALAR_EMIT_TESTS" "$FIELD_BATCH_TESTS" "$COSEAL_ISSUE_SOURCE" "$COSEAL_LEDGER" "$MAIN_ROOT" "$PIN_TEST" "$ARITY_PIN_TEST" "$ENTRY_PORT" "$COSEAL" "$COSEAL_ISSUE" "$COSEAL_TESTS" "$RAW_CLAIM" "$NEW_EXPR" "$CTOR_SCOPE" "$NO_EXIT" "$LOOP_BUILDER" "$LOOP_COND_FACTS" "$REJECT_REASON" "$LOOP_COND_BC" "$BC_ITEM" "$ITEMS_SRC" "$COND_UPDATE_TESTS" "$COND_UPDATE_FACADE" "$HELPERS_LOWER" "$ITEMS_TESTS" "$TESTKIT" "$STATIC_INGRESS" "$STATIC_OWNER_POLICY" "$MEMBER_ROUTE" "$CALLS_MOD" "$PHYSICAL_BRIDGE" "$SEL_ARG" "$LOCAL_FLOW" "$NEW_PREFIX" "$NEW_PREFIX_ARGS" "$NEW_PREFIX_SCAN" "$NEW_PREFIX_TERMINAL" "$ORD_ARGS" "$COSEAL_HELPERS" "$SEL_EMIT" "$SEL_EMIT_ARGS" "$PHYS_ABI" "$EMIT_VALID" "$BRAND_TESTS" "$ENV_DIRECT_TESTS" "$DESCENT_TESTKIT" "$DESCENT_TESTS" "$ITEM_SITE" "$MAP_LOCAL_TESTS" "$COMPOSITE_PROJECTION" "$LOOP_BREAK_FACTS" "$ROUTE_PREDICATES" "$CALLABLE_ROUTE" "$DEAD_ROUTE_TESTS" "$CORE_METHOD_SRC" "$NAMED_ARRAY_SRC" "$NAMED_ARRAY_METHOD_TESTS" "$CONTRACT_ISSUER" "$LEDGER_CONTRACT_TESTS" "$DECLARED_INSTANCE_LOCATOR" "$CORE_EFFECT_PLAN" "$SOURCE_METHOD_PORT" "$UNIFIED_EMITTER" "$LOOP_DECLARED_TESTS" "$ASSOC_INPUT" "$NORMALIZER_COMMON" "$RAW_LOOP_PORT" "$RAW_LOOP_ENTRY" "$CALLABLE_LOWERING_STATE" "$CALLABLE_STATE_SOURCE_PREP" "$LIFECYCLE_PHYSICAL_PROGRAM" "$LIFECYCLE_PHYSICAL_PROJECTION" "$LOOP_VALUE_PUBLICATION" "$BACKEND_VIEW_TESTS" "$BACKEND_VIEW_DRIFT_TESTS" "$BACKEND_VIEW_INTRINSIC_TESTS" "$ROOT_LIFECYCLE_TESTS" "$ROOT_MAIN_SELECTION_TESTS" "$LOOP_PIPELINE_TESTS" "$LOOP_SCOPE_TESTS" "$ROUTE_ITEMS_SRC" "$ROUTE_CALL_FREE_TESTS" "$ROUTE_TEST_SURFACE" "$SOURCE_LOOP_BRIDGE" "$LOOP_COND_FACTS_SRC" "$GENERIC_FACTS_ISSUER" "$LEXICAL_INSTANCE_CALL_SRC" "$LEXICAL_INSTANCE_PROVENANCE_SRC" "$LEXICAL_INSTANCE_SOURCE_SRC" "$BORROWED_FORMAL_USE_SRC" "$BORROWED_FORMAL_USE_TESTS" "$BORROWED_FORMAL_SOURCE_SRC" "$BORROWED_FORMAL_SOURCE_TESTS" "$BORROWED_FORMAL_ACTUAL_SRC" "$BORROWED_FORMAL_ACTUAL_TESTS" "$BORROWED_FORMAL_ENTRY_SRC" "$BORROWED_FORMAL_ENTRY_TESTS" "$BORROWED_FORMAL_RESULT_SRC" "$BORROWED_FORMAL_RESULT_TESTS" "$BORROWED_FORMAL_TERMINAL_TESTS" "$BORROWED_ENTRY_VALUES" "$BORROWED_ALIAS_MATERIALIZATION" "$BORROWED_ALIAS_MATERIALIZATION_TESTS" "$BORROWED_ALIAS_STATE_TESTS" "$PHYSICAL_COPY_BOUNDARY" "$BORROWED_COPY_BOUNDARY" "$BORROWED_COPY_BOUNDARY_TESTS" "$BORROWED_ENTRY_STATE" "$BORROWED_ENTRY_STATE_TESTS" "$BORROWED_ENTRY_SCOPE_TESTS" "$BORROWED_ACTUAL_OBSERVER" "$LEXICAL_INSTANCE_CALL_TESTS" "$VALUE_METHOD_CALL_SRC" "$FIELD_WRITE_CLAIM_SRC" "$RESULT_CLASS_CLAIM_SRC" "$LOCAL_COMMIT" "$LOCAL_COMMIT_CALL_RECV" "$LOCAL_COMMIT_PREP" "$LOCAL_COMMIT_HANDLE" "$COSEAL_ISSUE_LEXICAL" "$DIRECT_CALL_PHYSICAL_TESTS" "$DIRECT_CALL_HANDLE_TESTS" "$ROOT_CALL_ENTRY" "$ROOT_CALL_ENTRY_VALIDATION" "$ROOT_CALL_ENTRY_TESTS" "$RAW_CLAIM_TERMINAL_CALL" "$LEXICAL_I64_EMIT" "$LEXICAL_I64_PROJECTION" "$LOCAL_CALL_GROUP" "$LOCAL_CALL_GROUP_TESTS" "$FINISHED_SOURCE_PROJECTION" "$FINISHED_SOURCE_PROJECTION_TESTS" "$FINALIZED_CALL_VISITOR" "$BORROWED_CALL_INCOMING" "$BORROWED_CALL_USES" "$BORROWED_CALL_USES_TESTS" "$FINISHED_COPY_TESTS" "$BORROWED_CARRIER_JSON_TESTS" "$BORROWED_USE_PROJECTION_TESTS" "$LIFECYCLE_CALL_JSON" "$BORROWED_WIRE_TESTS" "$BORROWED_SOURCE_PUBLICATION_TESTS" "$LIFECYCLE_PROGRAM_JSON" "$LIFECYCLE_C_FORMALS" "$LIFECYCLE_C_CALLS" "$LIFECYCLE_C_PARSER" "$FIELD_REF_PROJECTION" "$EXACT_NUMERIC_LIFECYCLE" "$LOCAL_COPY_PROVENANCE" "$LOCAL_COPY_PROVENANCE_TESTS" "$BORROWED_PHYSICAL_PROJECTION" "$BORROWED_PHYSICAL_PROJECTION_TESTS" "$LEXICAL_I64_FIXTURE" "$LEXICAL_I64_EMIT_TESTS" "$EXACT_LEXICAL_READ" "$EXACT_LEXICAL_READ_TESTS" "$RECURSIVE_CHILD_LOWERING" "$RECURSIVE_CHILD_DISPOSITION" "$ROOT_RESULT_NEW_TESTS" "$ENTRY_HOME" "$ENTRY_HOME_TESTS" "$HOME_ABI" "$PKG_ISSUER" "$SET_ELEMENT_DRAFT" "$FAULT_CHECKED_ARRAY" "$NEW_ARGUMENT_DRAFT" "$BORROWED_CALL_USES_CTOR_VIEW_TESTS" "$BORROWED_SOURCE_PUBLICATION_NEW_ARGUMENT_TESTS" "$HOME_LOCAL_CALL_FLOW" "$NEW_PREFIX_BRANCH" "$FUNCTION_CONTROL_NEW_HOMES" "$LOWERING_CALLS" "$TERMINAL_CALL_ADMISSION" "$LEXICAL_NULLABLE_EMIT" "$MAP_VALUE_COMPLETION_TESTS" "$TERMINAL_RELATION" "$FIELD_READS_LEDGER" "$TERMINAL_HOME" "$BORROWED_SOURCE_PUBLICATION_I64_RESULT_TESTS" "$BORROWED_SOURCE_PUBLICATION_ROOT_SOURCE_TESTS" "$FINALIZED_ROOT_HANDOFF" "$TERMINAL_RESULT_TESTS" "$OPERANDS_DRAFT" "$BORROWED_CALL_USES_NULL_COMPARE" "$BORROWED_CALL_USES_NULL_COMPARE_TESTS" "$BORROWED_SOURCE_PUBLICATION_NULL_COMPARE_TESTS" "$BORROWED_SOURCE_PUBLICATION_NULLABLE_ACTUAL_TESTS" "$FIELD_BATCH_DRAFT" "$BORROWED_FORMAL_USE_FIELD_READ" "$BORROWED_CALL_USES_OBJECT_FIELD" "$BORROWED_CALL_USES_CLOSURE" "$COMPILED_ENTRY_CONTRACT" "$BORROWED_SOURCE_PUBLICATION_PARAM_FIELD_TESTS" "$BORROWED_SOURCE_PUBLICATION_PARAM_ACCEPTANCE_TESTS" "$VERIFIER_INVOKE" "$VERIFIER_INVOKE_TESTS" "$NEW_PREFIX_FIELD_CALL" "$NEW_PREFIX_FIELD_WRITE" "$INSTANCE_CONSTRUCTION_SRC" "$RESULT_CLAIM_TESTS" "$PARAM_CONTRACT_MODEL" "$PARAM_CONTRACT_ISSUER" "$PARAM_CONTRACT_TESTS" "$RECEIVER_OBSERVATION_SRC" "$PHYSICAL_BOUNDARY_SRC" "$SEL_TERMINAL_CALL" "$CONSTRUCTION_STATE_SRC" "$CONSTRUCTION_EMISSION_SRC" "$CONSTRUCTION_VALIDATION_SRC" "$COSEAL_ISSUE_CHILDREN" "$QUALIFIED_STATIC_CLAIM_SRC" "$INSTANCE_CTOR_SEMANTIC" "$CHILD_LOWERING_IMPL" "$PROVIDER_CALL_ARG_TESTS" "$CORE_METHOD_CONSUMER"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    echo "[$TAG] source reached hard 800-line boundary: ${file#"$ROOT_DIR/"}=$lines" >&2
    exit 1
  fi
done

echo "[$TAG] ok"
