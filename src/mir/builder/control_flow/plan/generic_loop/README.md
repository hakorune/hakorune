# Generic Loop (v0)

Responsibility:
- Recognize a minimal loop body subset (facts)
- Retain one successful GenericLoopV1 step disposition in the same canonical
  extraction transaction that produces those facts
- Normalize to CorePlan using only Loop + leaf effects + ExitIf/IfEffect
- Keep the closed numeric/body-managed carrier-role vocabulary separate from
  Builder-side MIR representation preparation

Non-goals:
- No role inference from AST spelling after facts extraction
- No second step-placement classification after canonical extraction
- No located source/path, Builder, ledger, or CorePlan authority in the
  successful extraction product
- No MIR type storage in facts
- No carrier type default, coercion, or PHI conflict repair
- No nested control-flow or else-branches
- No route-specific semantics beyond the subset

Carrier representation boundary:
- `facts_types::GenericLoopCarrierRoleV1` owns only the semantic role
- `facts_types::GenericLoopV1ExtractionV1` privately co-seals the existing
  facts with one final `GenericLoopV1StepDispositionV1`
- `try_extract_generic_loop_v1` is the primary extraction owner;
  `try_extract_generic_loop_v1_facts` is a consuming thin facade
- `carrier_representation` prepares one exact lowering-time MIR representation
- S0 keeps the preparation product disconnected from production allocation
- Slot publication and V0/V1 producer wiring belong to later TYPE0 rows

SSOT:
- Condition canon (analysis-only view): `plan/canon/generic_loop/condition.rs`
- Update canon (analysis-only view): `plan/canon/generic_loop/update.rs`
- Step canon (extract + placement): `plan/canon/generic_loop/step.rs`
- Facts: `facts.rs`
- Normalizer: `normalizer.rs`
- Reject reasons: `plan/facts/reject_reason.rs` (log format: `[plan/reject]`)

Step extract order (SSOT, no behavior change):
- `extract_loop_increment_plan` (legacy helper fast path)
- `generic_loop_canon/step_extract/var_step.rs` (`i = i + step_var` and related top-level patterns)
- `generic_loop_canon/step_extract/next_step.rs` (`next_i = i + 1; i = next_i` style)
- `generic_loop_canon/step_extract/complex_step.rs` (`i = (i - x) / k` style)
- `facts/canon/generic_loop/step/extract.rs`: compatibility facade only

Step placement split (SSOT, no behavior change):
- `control_flow/generic_loop_canon/step_placement/facts.rs`: increment/conditional step の形マッチだけ担当
- `control_flow/generic_loop_canon/step_placement/plan.rs`: `RejectReason` を含む placement 判定だけ担当
- `facts/canon/generic_loop/step/placement/*`: compatibility facade only

Condition split (SSOT, no behavior change):
- `control_flow/generic_loop_canon/condition/candidates.rs`: loop_var candidate 観測だけ担当
- `control_flow/generic_loop_canon/condition/bound.rs`: BoundExpr 観測だけ担当
- `facts/canon/generic_loop/condition.rs`: compatibility facade only

Update canon split (SSOT, no behavior change):
- `control_flow/generic_loop_canon/update/literal_match.rs`: update 式の shape match だけ担当
- `control_flow/generic_loop_canon/update/literal_step.rs`: `UpdateCanon` の literal step 生成だけ担当
- `facts/canon/generic_loop/update.rs`: compatibility facade only

Type split (SSOT, no behavior change):
- `canon/generic_loop/types.rs`: Condition/Update/Step の観測型定義

Related docs:
- `docs/development/current/main/design/coreloop-generic-loop-v0-ssot.md`
- `docs/development/current/main/design/compiler-expressivity-first-policy.md`

Post-effect stage boundary (M4 test/reference lane):
- `generic_stage_matrix_tests` is an observation-only ledger. It does not own
  Generic winner selection, Recipe/JoinSig/PHI production, physicalization,
  retry, or candidate publication.
- `contract_present = false` is recorded as evidence; it is not an implicit
  pre-effect decline. A missing receipt keeps the row unresolved.
- The first Builder effect is determined from block, ValueId, typed-value, and
  binding snapshots. Restoring a variable map is not candidate rollback.
- Nested lowering observes the depth-1 fastpath first and the
  `nested_loop_recipe_adoption` fallback second. Neither test helper is a
  precedence oracle.
- Legacy receipts remain diagnostic until the parent M4/M10 gates close.
- D2-A3-S1 is a closed test-only census: the Both fixture reaches the depth-1
  fastpath in release/strict/planner-required modes, while Generic fallback is
  not naturally reached. It changes no language grammar or IR semantics.
  The stage-matrix reference page and parent design SSOT are mandatory
  post-implementation closeout surfaces.
- D2-B2 is a test-only overlap-parity stop. Fresh V0/V1 plans and the real
  witness are compared through the shared frame; nested-carrier digest
  mismatch and V0 terminal success keep the semantic result unresolved.
  Planner-required V0 suppression is a separate pre-effect gate, not a winner
  proof. The matrix is closed as deterministic evidence;
  `ParityDispositionV1::UnresolvedStop` is a classification, not a policy
  evaluator. No route, Recipe, JoinSig, PHI, or retry authority changes here.
  M6-B and P1b are closed; D2-B4-S1 is closed as the test-only nested-carrier
  winner-certificate matrix (`JOINIR-GENERIC-NESTED-CARRIER-WINNER0-D2-B4-S1`).
  Release/Strict Both only yields a carrier-projected V1 candidate; the parent
  Generic D2 disposition remains UnresolvedStop because legacy V0 has no debt
  receipt.
  D2-B4-S2 is the next design stop: a test-only BindingRefV1 witness must
  prove that an inner write and post-loop outer read share one strict-ancestor
  binding, with a shadowing negative. This does not select a production
  winner or add a Recipe/JoinSig/PHI/physicalizer caller.

LOOP0-P0b-T0 associated-source boundary:
- T0 is one semantic row implemented as `C0 -> B0 -> R0 -> L0` under
  Refactor Series Mode.
- C0 keeps `lower_loop_header_cond` as the raw `CondBlockView` prelude facade
  and moves tail-expression CFG descent to one associated-input port core.
- B0 threads the same borrowed port through direct body and cleanup lowering.
- R0 adds one neutral Parts entry for O0-verified ExitAllowed recipe items and
  `StmtWrappedJoinIf`; no GenericLoop-local statement/If dispatcher exists.
- L0 consumes the non-Clone O0 representation in one same-call disconnected
  located composer. Production located roots and ledger claims remain zero.
- The located path never reads environment/body policy, `facts.body`,
  `body_no_exit`, or `matches_loop_increment`, and never rebuilds a recipe or
  pairs source through AST equality, spans, names, ValueIds, or side maps.
