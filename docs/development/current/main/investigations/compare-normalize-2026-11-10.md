# Comparison normalization: source meaning, one private plan, independent verification

Date: 2026-11-10. Base: `988a95b790`. Task: `$S/codex-compare-normalize-2026-11-10.md`.
Evidence: `$L=/mnt/workdisk/claude-scratch/compare-normalize`; immutable baseline `$SC`.

## Rules fixed before implementation

N1. The canonical issuer owns source domain, law, required evidence, result, Fault/suspension/order/borrow; it contains no JSON or C selection.
N2. A single private source-to-execution conversion retains original loan/site and operand indices, choosing Scalar, NullObservation, Text or Checked; physical binding never reclassifies MIR types.
N3. Frame decision, the existing writer and source correspondence consume that plan; independent validation inspects actual MIR and never treats the plan as a waiver.
N4. Exact I64/I64 six comparisons and Bool/Bool Eq/Ne explicitly issue NoSourceFault and NonSuspending; NormalInteger/NormalInteger checked view remains TypeError/MaySuspend.
N5. EQUAL_DYNAMIC_NULL explicitly issues NoSourceFault, retaining its existing NonSuspending and source capability restrictions.
N6. Nullable field, nullable result, borrowed Dynamic guard, Dynamic result and nullable field-alias initializer keep their distinct original laws; none is forged as another law's Dynamic/null envelope.
N7. The existing field-alias initializer Eq/Ne law is retained through the existing HomeExpressionPlan/source-operation owner before raw descent; no parallel comparison ledger is created.
N8. NullCompare is a nonterminator READ with dst:ValueId, op:CompareOp, lhs:ValueId, rhs:ValueId; op is Eq/Ne, result Bool, source operand order unchanged, no frame/edges/ownership transition.
N9. Exactly one NullCompare operand has a unique direct ConstValue::Null definition; 0, false and Copy(null) cannot replace that sentinel position.
N10. The other operand must be an available, dominating permitted live carrier; integer/Bool carriers remain distinct from null and Text-only residences are not null carriers.
N11. Independent checks preserve actual definition uniqueness/order/dominance, Copy/PHI origin and kind, borrower/holder/opaque carrier lifetime and faulted-state restrictions.
N12. Canonical Compare with a direct null operand is rejected by name before publication; compatibility producers do not gain canonical admission.
N13. Serializer maps NullCompare to unchanged borrowed_null_compare five keys and Compare to unchanged compare; it does not search definitions or choose a representation.
N14. C, runtime and JSON spellings/predicates remain unchanged; no schema version, alias or new descriptor is introduced.
N15. Preserve current source Eq-only capabilities and existing nullable-alias Eq/Ne; do not broaden Dynamic/null Ne, Bool ordering, object identity or basic-expr acceptance.
N16. NullCompare may be reused in the same block or have IDs remapped, but is never added to the Compare SSA cloning/rematerialization arms.
N17. If an existing null observation is actually cloned in the fixed baseline, stop that migration stage and report evidence rather than replaying a borrowed observation.
N18. Source evidence mismatch refuses by an existing named boundary; no checked fallback, AST/source workaround, backend repair or integer-zero identity guess.
N19. Evaluate original left then right once; preserve executable Normal and surviving Fault acquisition/transfer/End responsibility and order, without requiring equal static End counts.
N20. Stage A contracts and stage B plan connection preserve admission/MIR; later changes are lighter or equal-weight Compare→NullCompare spelling only, with every changed input recorded.
N21. Functionality and cleanup have separate normalized production/test counts; cleanup includes all shared destinations and closes net negative without movement/formatting credit.

## Private plan and NullCompare decision

Frame selection precedes ValueId materialization, so the source plan holds indices/op and execution kind; the existing scalar physical plan binds already evaluated values to it.
Reuse `home_expression_comparison_nodes`, `CheckedScalarConditionOperationV1` and the normal read plan rather than adding a second comparison compiler.
The source operation loan retains the original proof; the existing field-alias preflight currently returns Bool without saving one, so its accepted law must be retained at that owner before emission.
The raw recursive descent borrows a plan by original source site before evaluating children, passes it through the existing comparison append and retains original SSA correspondence.
The default approved NullCompare shape fits the existing wire: direct null operand and ordered carrier are already what borrowed_null_compare admits. No alternative public shape is needed.
Rust result-origin checks share a small scalar result-kind query, not the Text-operand query, because NullCompare borrows a different carrier domain.

## 1. Meaning issuance and selection (baseline positions)

| Owner | Responsibility |
|---|---|
| `dynamic_operator_contract/issuer.rs:82-114,265-340,369-386,393-533` | String/Bool, checked Integer, Dynamic/null and domain issuance |
| `resolved_semantics/home_expression_operations.rs:30-58` | Exact source I64 currently asks NormalInteger; split to I64 without changing checked consumers |
| `resolved_semantics/home_bool_condition.rs:305-414,445-480` | Four null authorities and exact scalar/Text comparisons |
| `resolved_semantics/home_scalar_operation_preflight.rs:57-70` | Fifth authority: field-alias local Eq/Ne with source-null, both orders; currently early return without loan |
| `resolved_semantics/home_expression_operator_loan.rs:70-109` | Original nullable field/result proofs, not tagged envelopes |
| `resolved_semantics/home_expression_comparison_nodes.rs:12,64,142` | Existing separate Text/checked Integer/exact Integer selection to consolidate |
| `resolved_semantics/home_expression_evaluation.rs:51-55` | Separate frame exclusions to replace by one plan query |
| `ordinary_new_borrowed_formal_use_operands.rs:91-98,230-280` | Keep checked NormalInteger and Eq-only borrowed/null capability |
| `ordinary_new_borrowed_compare_materialization.rs:183-238` | Original checked carrier/source evidence; not exact I64 |
| `loop_recipe_contract/source_loop_bool.rs:203-220` | Arithmetic keeps NormalInteger; exact compare follows I64 domain, logical may_fault stays conservative |

## 2. First MIR writers

| Owner | Responsibility |
|---|---|
| `builder/raw_invocation_source_transport/bool_condition.rs:201-225` | Source-scoped Binary observation, not a separate semantic decision |
| `builder/ops/binary_expression_descent.rs:248-267` | Source plan before left/right one-time evaluation; existing append flow |
| `builder/ops/comparison.rs:36-88` | Operand SSA finalization, existing comparison writer |
| `builder/emission/compare.rs:48-63` | Sole raw comparison append; plan selects the approved opcode |
| `builder/builder_emit_core.rs:92-105` | Canonical exact integer append, not a null producer |
| `selected/expression_value/invoke.rs:75,147-160` | Common Text read/scalar plan writer |
| `root_call_entry/scalar_condition_operation.rs` | Existing physical plan and expected-row source matcher |
| `resolved_lowering/trivial_ssa/operation.rs:12-56`, `control_flow/plan/lowerer/effect_emission.rs:356-366` | Existing compatibility/lowerer emission; do not infer new canonical source meaning |

## 3. SSA transformation and regeneration

| Owner | Responsibility |
|---|---|
| `builder/ssa/local/materialize.rs:224-235` | Reuse same-block condition value, preserve for NullCompare |
| same `:556-568` | Existing Compare cloning; do not add NullCompare |
| `builder/ssa/local/checked_compare.rs:305-365` | Reuse checked result, cross-block Bool Copy rather than replay |
| `builder/ssa/phi_input_materializer/edge_rematerialization.rs:170-174,198-203` | Dominating reuse versus existing Compare regeneration; no new NullCompare clone |
| `builder/joinir_id_remapper.rs:130`, `joinir_id_remapper_values.rs:26` | Preserve opcode while remapping IDs |
| `passes/simplify_cfg/value_uses.rs:56`, `passes/inline_soft_leaf.rs:302` | Operand substitution/inline ID remap, not semantic issuance |

Existing 1788 dumps contain 54 inputs/functions with one direct-null Compare each (53 Eq, 1 Ne), all RHS direct null; no double-direct-null or duplicated observation group was found.
Code-chain inspection confirms same-block/dominating reuse and checked-result Copy before clone arms; repeated null constants/operand Copies are not observation replay.
The single Ne and Copy(null) carrier examples require canonical-versus-compatibility classification; do not erase their existing acceptance based on the four Eq-only guard laws.

## 4. Verification, transport and readers

| Owner | Responsibility |
|---|---|
| `verification/invoke.rs:324-346` | Activate independent checks even for NullCompare-only and canonical old null Compare |
| `verification/invoke_string_read.rs:130,178,229` | Shared availability/scalar-origin components; keep Text borrowing separate |
| `verification/value_origin.rs:118`, `ssa.rs`, `dom.rs` | Actual Copy/complete-PHI origin, definition/SSA/dominance, not overwritten def-map as proof |
| `compiled_entry_contract/borrowed_call_uses_null_compare.rs` | Existing null use/coverage and direct sentinel checking |
| `compiled_entry_contract/borrowed_call_uses.rs:154,232,522` | Scalar-authority fence, view pickup and null admission |
| `compiled_entry_contract/borrowed_call_uses_object_field.rs:43-87` | Existing Eq false-edge nonnull guard |
| `verification/invoke_owned_array.rs:245` | Holder/slot/carrier lifetime at actual use |
| `published_backend_view/physical_program_json.rs:276-301,463-480` | Replace selection and delete null_producer; direct transport only |
| `published_mir/*indexed_rows.inc:133-160`, `*emit.inc:290-322` | Existing carrier kinds/liveness and wire execution, unchanged |
| `src/mir/*_micro_seed_plan.rs`, instruction queries/printers/vocabulary | Inventory/readback of existing MIR, not source authority |

## All Fault/suspension readers before changes

| Reader | Effect of contract-only stage |
|---|---|
| `dynamic_operator_contract/model.rs` accessors | Definition only; sealed defaults remain unchanged |
| `dynamic_operator_contract/tests.rs:35,48,82,100,108` | Other Dynamic/mixed rows unchanged |
| same `:141,149` | Dynamic/null Fault expectation changes only |
| same `:268,277` | Integer arithmetic unchanged |
| same `:333,348` | Bool exact expectations change; checked NormalInteger remains unchanged; add exact I64 coverage |
| `home_expression_comparison_nodes.rs:49-50` | Only production DynamicOperator axis reader, guarded by exact String domain; unchanged for other rows in stage A |
| `dynamic_invocation_contract/tests.rs:101`, field-append/local-field `.fault()` | Different contract types, unchanged |

Evidence families: scalar six ops/Bool EqNe use N1-4; distinct null source capabilities N5-7/N15; direct-null and live carrier N8-12; stable wire N13-14; Copy/PHI and SSA N11/N16-17; short-circuit/Fault/Text End N18-20.
