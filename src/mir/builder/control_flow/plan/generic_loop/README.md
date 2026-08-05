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
  D2-B4-S2 is a closed test-only BindingRefV1 witness: an inner write and
  post-loop outer read share one strict-ancestor binding, with a shadowing
  negative. The scoped D3 design stop follows; neither selects a production
  winner or adds a Recipe/JoinSig/PHI/physicalizer caller.

The D2-B4-S2 cfg(test) witness is green with:

```bash
env -u HAKO_JOINIR_STRICT -u HAKO_JOINIR_PLANNER_REQUIRED \
  RUSTFLAGS='-Awarnings' cargo test --lib generic_d2_b4_s2 -- --nocapture
```

The positive row uses parsed source/canonical facts and resolver-issued
`BindingRefV1`s; the shadowing row remains unresolved. In strict
planner-required mode the V0 facts contract is intentionally suppressed, so
the test records `SuppressedByPlannerRequired`, captures the `[GenericLoopV1]`
schedule in the same mode scope, and never calls the V0 composer. Legacy
carrier labels/tags are corroborating only. This remains test-only evidence;
no Generic route, Recipe, JoinSig, PHI, physicalizer, Builder, MIR, or runtime
authority is changed.

S2, the scoped D3 matrix, the source-backed bridge, the planner-suppression
row, the Index/Ambiguous row, and
`JOINIR-GENERIC-RESOLVED-CARRIER-ELIGIBILITY-PROTOCOL0-D3-S0` are closed as
cfg(test)-only tests. D3-S0 co-seals actual natural-Both resolver/facts/frame
evidence into a private non-`Clone` eligibility witness and retains typed
mismatch negatives. The Index/Ambiguous row remains a negative source witness;
the bounded
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-COMPOUND-UNAVAILABLE0-D2-S3`.
is now closed as cfg(test)-only: parsed nested `CompoundAssignment` under
scoped basic sugar co-seals actual resolver/source/frame/BindingRef/facts
evidence and retains exact `Unavailable("CompoundAssignment")` as typed
pre-effect `UnresolvedStop(CompoundUnavailableCarrier)`. Release/Strict raw
schedule is measured `[V0,V1]`. Execution returns to the parent D3 design
stop; no production selector, neutral issuer, or route handoff is allowed.

The bounded premise
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-TOPLEVEL-COMPOUND-PREMISE0-D2-S4`.
It must observe a parsed top-level `CompoundAssignment` through the existing
resolver and facts owners before any `CompleteNoRecursiveCarrier` source row
is selected. It is now closed as typed `NoStandaloneRow`: the parsed
resolver/source/frame witness is stable, but facts are absent and the measured
Release/Strict raw schedule is `[]`. No collector widening, selector,
Legacy/winner, Recipe, PHI, Builder, MIR, Retry, fallback, or production
handoff is allowed; the Generic reference page and current mirrors were
updated in the same closeout commit.

The accepted implementation child follows the design child
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-NORECURSIVE-DISPOSITION0-D2-S5-D0`.
It must choose one parsed flat Assignment shape, distinguish it from
simple-while/V1-only/facts-absent shapes, and define the one-loop projector
boundary before a cfg(test) source row is added. Exact
`CompleteNoRecursiveCarrier` is provisionally an out-of-target unresolved
disposition; absent facts or empty raw schedule is `NoStandaloneRow`. The
implementation child is
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-BOTH-NORECURSIVE0-D2-S5-S1`.
It remains cfg(test)-only and must return to D2-S5-D0 on shape, facts, mode,
schedule, or identity drift.

The S1 source witness is closed as cfg(test)-only: exact flat Assignment facts
`CompleteNoRecursiveCarrier` and measured Release/Strict `[V0,V1]` yield only
typed `UnresolvedStop(NonRecursiveOutOfTarget)`. The one-member shape is not
the recursive eligibility capability and adds no production route.

The accepted D3-S1 policy boundary is
`JOINIR-GENERIC-RESOLVED-CARRIER-SELECTION-DISPOSITION-MATRIX0-D3-S1-D0`.
It owns the two-column evidence/selection partition and the
winner/correctness/disjointness proof for natural recursive Both. Its selected
cfg(test)-only child is now closed as evidence:
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-V1ONLY-LOCAL0-D3-S1-S1`, which
co-seals V0=false, V1=true, `CompleteNoRecursiveCarrier`,
`has_body_local=false`, actual frame flags, no recipe contract, and raw `[V1]`,
with typed `UnresolvedStop(V1OnlyNonRecursive)`; no facts snapshot issuer,
selector arm, or production handoff may be added.

The scoped D3 matrix is one cfg(test) test over four typed rows. It separates
pre-effect resolver eligibility from post-effect V1 corroboration. The next
design stop is the co-sealed source-to-selection handoff card in
`docs/development/current/main/investigations/joinir-generic-resolved-carrier-selection-boundary-d3-design-2026-08-05.md`;
the current facts-only selector remains unchanged.

The nested-`IfThen` carrier coverage row is now closed as test-only evidence:
`generic_d2_b4_s2_nested_if` parses one outer/inner loop source, keeps the
canonical inner `j` step separate, and resolves the nested write plus post-loop
read to the same resolver-issued `BindingRefV1`. Release/Strict raw schedules
remain `[V0, V1]`; fresh direct V0/V1 observations reach `LowerSome` with
`GenericComposer` as first effect owner and stable distinct semantic digests.
The V1 witness records `CompleteRecursiveCarrier(["j"])`; the legacy witness
still terminates at V0. This row adds no production route, selector, Recipe,
PHI, Builder, MIR, Retry, fallback, or runtime authority and does not close
parent Generic D2. The next blocker is the co-sealed source-to-selection
design stop named above.

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

## D4-WITNESS0 closeout

The shared source-window witness is closed as a private `#[cfg(test)]`
transport product. It is not a Generic-loop planner input and does not
authorize this plan family, a selector, Recipe, Builder/MIR caller, retry,
fallback, or runtime route. D4-S2 family-boundary authority is accepted, and
the existing raw Generic route remains unchanged until a future selector
decision and atomic cutover.

D4-S1-S0 covers exact DirectAccum admission, foreign/non-loop source-window
rejects, and a shape-negative terminal probe reject. It remains test-only and
does not make this Generic-loop planner a selector or production consumer.

D4-S2-S0 is now closed outside this planner as a private six-row legacy
same-source census. The rows consume one resolver-owned receipt per fixture /
mode pair and retain only `legacy_*` facts, carrier, raw schedule, and existing
resolved-preflight observations: nested-predicate is
`CompleteRecursive(["j", "sum"])` with `[NestedLoopMinimal, GenericLoopV1]`,
while direct-accum is `CompleteNoRecursive` with `[AccumConstLoop]`, across
Release/Strict/planner-required. This README remains a Generic facts/recipe
boundary: the census does not add a planner selector, Recipe/key, Builder/MIR
caller, retry, fallback, or canonical policy. D4-S3-D0 is closed; the private
D4-S3-S0 witness is complete, and D4-S3-S1 remains the next matrix-only row.

D4-S3-D0 is now closed as design-only. This Generic facts/planner boundary
does not become the future family selector: its raw V0/V1 carrier and schedule
remain legacy-labelled observations. The future resolver-branded observation
set is separate, and its family-level selector must reject incomplete or
ambiguous evidence rather than use a legacy suffix/fallback. D4-S3-S0 is a closed
private witness only; no Generic Recipe, Builder/MIR caller, or retry removal
is authorized.

D4-S3-S0 is now closed outside this planner as six private resolver-branded
observation sets. D4-S3-S1 is also closed outside this planner as nine private
source-backed fixture/mode matrix sets. It keeps `NoStandaloneRow`, planner
freeze, V0Only, and parsed Neither distinct; all family rows remain unresolved
and carry no winner, Recipe/key, Builder/MIR effect, retry, fallback, or
production caller. D4-S3-S2 is now closed outside this planner as a separate
test-only neutral selector; all nine rows remain unresolved and this planner
does not become the family selector. D4-S4-D0 further rejects current
GenericLoop facts/RecipeBody as portable Recipe inputs: a resolver-issued
AST-free Generic demand and real Selected(Generic) proof are required before
any producer witness. D4-S4-S0 is now closed as `NoSafeSlice`; no selected
callsite or candidate envelope exists. D4-S4-S0-D0 closes the future
move-only source lease -> AST-free shape/candidate -> observation -> selector
-> demand chain, with this Builder planner remaining outside it. The next
frontier is the gated demand witness; no synthetic winner or production caller
is allowed.
