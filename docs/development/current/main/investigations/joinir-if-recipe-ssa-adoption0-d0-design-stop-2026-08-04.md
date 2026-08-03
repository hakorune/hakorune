# JOINIR-IF-RECIPE-SSA-ADOPTION0-D0

Status: D0-A census closed; D0-B0 same-pass facts projection implemented and
tested; D0-B1 portable schema/source-claim/structural verification and
normalization implemented and tested. The mapper, JoinSig, and production If
consumer remain unstarted — do not wire production If yet.
Date: 2026-08-04

This card records the next cleanup target after the Loop cutover lane. It is
not a claim that If already has two, or only two, equivalent production
implementations. The first step is an authority census.

## Verified audit facts

The repository currently contains several If-producing surfaces:

```text
raw/descent path:
  src/mir/builder/if_form.rs
  src/mir/builder/stmts/if_statement_descent.rs
  block_stmt -> drive_raw_if_statement_with_port_v1

CorePlan/JoinIR path:
  control_flow/plan/lowerer/plan_lowering.rs
  control_flow/plan/features/if_join.rs
  control_flow/plan/parts/dispatch/if_join.rs
  if_branch_lowering.rs / if_general.rs / if_exit.rs

resolved source-bound path:
  resolved_lowering/located_if.rs
  resolved_lowering/if_materialization.rs (IfCfgSessionV1)
```

`if_statement_parity_tests.rs` exercises the raw IfForm boundary and is useful
parity evidence, but its existence alone does not prove that the two paths
have identical authority or that one is test-only. The resolved path adds a
third surface that must be classified separately.

The current canonical lifecycle is:

```text
CanonicalSsaFunctionSessionV2
  = Binding SSA + CanonicalCfgSessionV1 + one PhiTxn
```

It is the SSOT for the canonical resolved lane. It is not yet the sole writer
for every If/Loop/JoinIR production edge. `IfCfgSessionV1`, the plan If join
materializers, raw `IfForm`, legacy PHI repair, and any JoinIR inline writer
remain execution surfaces until their callers are retired.

## Non-claims and boundary

Do not claim any of the following before the census is closed:

- that `if_form.rs` and CorePlan are the only If authorities;
- that `located_legacy_*` is wholly dead or safe to delete;
- that all PHI/CFG writers already use the canonical session;
- that a portable `IfRecipeV1` is already consumed by production;
- that the feedback's line-count reduction is a safe deletion estimate.

`LocatedLegacyLoweringSessionV1::verify` currently appears test-only, while
related located/raw carriers still have production-facing references. The
cleanup task is therefore scoped to a caller census and test migration proof,
not a blanket `located_legacy_*` deletion.

## Located legacy S0 census (2026-08-04)

The dedicated guard
`tools/checks/joinir_located_legacy_retire_guard.sh` is green and fixes the
retirement boundary:

```text
LocatedLegacyLoweringSessionV1::verify calls = 48, all test-only
production-root constructors                        = 0
non-test module/re-export roots                     = builder.rs only
internal adapter selector/lower pairs               = 6, session-local only
source/claim carriers                               = retained and still separate
test oracles                                        = 2 callable-result modules present
all located components                              < 800 lines
```

This census has now been followed by the bounded S1 retirement. The carrier
`callable_result_representation::located_legacy` and its caller-ledger/
loop-claim consumers remain in scope. No normal/raw ingress, IfCfgSession, or
resolved A+ owner was changed.

## Located legacy S1 closeout (2026-08-04)

The disconnected session and its test-only adapters were removed after the
caller-zero proof; no production caller was introduced to preserve the old
oracles. Deleted scope is limited to:

```text
builder/located_legacy_lowering.rs
builder/located_legacy_{assignment,if,return}.rs
builder/located_legacy_*_tests.rs
callable_result_representation/tests/{located_legacy_lowering,located_short_circuit_lowering}.rs
builder.rs module/re-export and callable-result test-module registrations
```

The live located carrier/ledger/claim files remain. The post-retirement guard
and focused compile/test checks are green:

```text
bash tools/checks/joinir_located_legacy_retire_guard.sh
RUSTFLAGS='-Awarnings' cargo check -q
RUSTFLAGS='-Awarnings' cargo test -q callable_result_representation --lib
RUSTFLAGS='-Awarnings' cargo test -q nested_predicate_profile --lib
```

Mixed raw/descent guards are intentionally retained for the later raw-parity
retirement lane; they are not evidence that the deleted located session still
exists.

## D0-A preliminary authority census (2026-08-04)

The first PHI/CFG audit fixes the boundary without claiming a single writer:

| Surface | Current production owner | Candidate reachability | Disposition |
| --- | --- | --- | --- |
| resolved trivial If | profile/IfControl supplies semantics; `CanonicalCfgSessionV1` + `BindingSsaBuilderV1`/`PhiTxn` physicalize | existing resolved candidate ingress | first pilot candidate when the fixture includes an explicit else and post-merge read |
| resolved located If | pre-SSA compatibility owner: `located_if.rs` + `IfCfgSessionV1` + `define_join_phis` | resolved source-bound candidate | parity oracle; private CFG/PHI session is not canonical |
| raw/descent If | `if_form.rs::lower_if_form_with_condition...`, `normalize_if_else_phi` / `merge_modified_vars`, and logical short-circuit/conditional-expression joins | raw/default ingress | remains raw authority until its own cutover |
| CorePlan/JoinIR If | `plan/lowerer/plan_lowering.rs` + `plan/features/if_join.rs::apply_if_joins` | plan/JoinIR route and loop features | direct production If-PHI writer; first retirement target only if the selected shape reaches this owner |
| JoinIR inline / route-local | merge coordinator, exit-PHI builder, rewriter stages, loop-cond features | loop/JoinIR routes | separate Loop/JoinIR adoption lane |
| JoinIR-to-MIR converter | `join_ir_to_mir/joinir_block_converter/handlers.rs` direct PHI writers | JoinIR conversion callers | downstream physicalizer; separate caller-zero row |
| JSON-v0 bridge | `json_v0_bridge` `if_else`/`merge`/`ternary`/`match_expr` writers and Stage1 Program-JSON producer | bootstrap/compat callers | separate phase-29ci retirement lane |

The semantic SSA owner is already named (`BindingSsaBuilderV1`/`PhiTxn` and
`CanonicalCfgSessionV1`); exclusive physicalization is not complete. The first
If shape is a resolved-trivial explicit-else, fallthrough-only join with one
outer `BindingRef` assignment per branch and a post-merge read (exact i64/Bool
condition, no nested control, return, short-circuit, call, record, or match).
The minimal fixture is equivalent to
`local x=0; if (x<1) { x=1 } else { x=2 }; return x`. This exercises two real
predecessors and the existing canonical session without touching the A+ pre-SSA
`IfCfgSessionV1` lane. An implicit-else is a later, smaller follow-up shape.

Candidate classification is explicit: resolved-trivial and resolved A+ roots
are inside the unpublished resolved-module candidate; raw IfForm/CorePlan
roots require per-entry proof and are not assumed candidate-scoped; JSON-v0
bridge writers are standalone bootstrap/compat builders (`NoCandidate`) and
cannot be used as evidence for canonical candidate isolation. `cf_common`,
`emission::branch`, and `phi_lifecycle` are shared physical sinks, not semantic
authorities; their sink caller counts are separate from owner retirement.

The census is not closed until each surface has an exact caller set and the
selected old writer has a proven candidate-scoped retirement edge. In
particular, do not retire both raw `IfForm` and CorePlan `apply_if_joins` from
the first pilot, and do not use a carrier-free trivial fixture without a
post-merge read as PHI proof.

### D0-A caller-set confirmation

The bounded production roots are now identified:

```text
raw IfForm:
  normal_script_direct_statement_owner::lower_direct_if_statement_v1
    -> block_stmt/if_statement_descent -> IfForm
  control_flow::cf_if_with_port_v1 -> IfForm

CorePlan If:
  PlanLowerer::lower_if -> plan/features/if_join::apply_if_joins
  (one non-test apply_if_joins caller; loop features can construct CoreIfPlan)

resolved A+ If:
  source_bound_package::consume -> lower_resolved_function_draft
    -> located_if.rs -> IfCfgSessionV1/define_join_phis

resolved trivial If:
  CanonicalTrivialSsaLowererV1::lower_if -> session.cfg
  (post-merge BindingSSA read creates the provisional/patch PhiTxn row)
```

The selected first pilot is the resolved-trivial path with an explicit-else,
fallthrough-only join, one outer `BindingRef` assignment per branch, and a
post-merge read. The existing
`CanonicalTrivialSsaLowererV1::lower_if` is already the canonical physicalizer;
the new gate is a single verified-Recipe producer/adapter feeding that same
session, not a second SSA owner. A+ located
`IfCfgSessionV1`, raw `IfForm`, CorePlan `apply_if_joins`, JoinIR inline writers,
and JSON-v0 remain separate owners until their own caller-zero rows. The
existing lowerer must stop re-reading source to make route decisions once the
Recipe adapter is promoted; otherwise the Recipe remains only a parity oracle.

## Ordered task sequence

### D0-A — authority and caller census (closed)

Inventory every production and test caller for:

```text
IfForm / if_statement_descent
CorePlan::If / plan If join helpers
IfCfgSessionV1 / resolved located_if
PhiMergeHelper / emission::phi / phi_input_materializer
route-local If/Loop PHI materializers
JoinIR inline PHI/CFG writers
json_v0 bridge writers
located legacy sessions and raw child carriers
```

For every surface record owner, input contract, mutation boundary, and
whether it can be reached from an unpublished compile candidate. This is a
BoxShape task: no new accepted source shape and no production wiring.

The matrix is now closed by the caller-set confirmation and independent worker
review. The selected first owner is the resolved-trivial canonical session; its
first old edge is shape-scoped to `CanonicalTrivialSsaLowererV1::lower_if`.
Other writers remain explicit non-selected owners.

### D0-A closeout / D0-B handoff

```text
source authority:
  resolved-trivial profile + canonical BindingSSA/CFG/PHI session
non-authority:
  raw IfForm, A+ IfCfgSession, CorePlan apply_if_joins, JoinIR converter,
  JSON-v0 writers, RecipeTree/StmtRef
fail-fast boundary:
  unsupported branch effects or non-trivial transfers reject before Builder
  effects; no old writer is retried after a selected recipe
selected next slice:
  design the AST/Builder/physical-ID-free IfRecipeV1 for the one-binding
  explicit-else shape, then prove its verified recipe can feed the existing
  canonical trivial session
non-claims:
  no whole-repository PHI writer unification, no A+/raw/CorePlan/JSON-v0
  retirement, and no production Recipe consumer yet
```

The next row is D0-B. Do not begin production wiring until the portable
contract, exact pre-effect rejection boundary, and a shape-scoped parity gate
are written.

### D0-B — portable IfRecipeV1 contract

Durable contract boundary: `docs/development/current/main/design/joinir-if-recipe-contract-ssot.md`.

Design one recursive semantic product, analogous to the Loop recipe, with the
following selected-shape contract:

```text
IfRecipeArtifactV1:
  schema version + owned source provenance/binding + one IfRecipeV1

IfRecipeV1:
  condition value (Bool, produced by the admitted i64 comparison profile)
  then block + explicit else block (both fallthrough-only)
  one outer BindingRef assignment in each branch
  post-merge read obligation for that BindingRef
  ElseDisposition::Explicit(block) now; ImplicitFallthrough later and distinct
  branch-transfer obligations and JoinSig merge/predecessor obligations
  source provenance without AST, Builder, ValueId, or BasicBlockId
```

The recipe is the semantic boundary; existing raw/plan/resolved structures are
parity oracles until a named producer and consumer are proven. The contract
must expose enough operation/BindingRef facts for the canonical lowerer adapter
to consume the recipe without re-reading AST or `LocatedStmt` to make a route
decision. Control flow remains in the recursive block algebra; leaf operations
do not contain a nested If or Loop. Do not introduce a second universal control
algebra while the Loop contract is still the active portable owner: first prove
the shape-scoped If artifact, then decide any shared control vocabulary in a
separate design row.

The implementation order is:

```text
D0-B0  same-pass VerifiedTrivialIfRecipeFactsV1 projection
       (condition/branch operation/write cardinality/continuation facts)
D0-B1  IfRecipeArtifactV1 schema + recipe-local IDs + source claim
       structural verifier + deterministic normalizer (landed)
D0-B2  same-pass facts -> recipe mapper and source-witness design stop
D0-B3  non-Clone IfJoinSig elaborator and typed physical-input seal
```

`VerifiedResolvedIfFlowV1` and `VerifiedTrivialCanonicalOwnerV1` remain their
own authorities; the new facts projection fills only the missing expression
and branch-operation contract. `BindingRefV1` is retained only in the
producer/adapter correspondence, never in the portable artifact. If/Loop
control shells stay separate until both have independent production consumers
and caller-zero evidence; only pure validation helpers may later be shared.

Verifier fail-fast rules for this row are: exactly two branch predecessors;
explicit else is not implicit fallthrough; exactly one write to the same outer
BindingRef per branch; both branch writes have the same admitted value class;
the post-merge read names that binding; no nested control, return/throw,
short-circuit, call, record, match, BlockExpr, or hidden fallback; canonical
recipe-local key order and no physical IDs. JoinSig elaboration, not the
schema verifier, owns predecessor/value-edge proof. The physicalizer may only
consume the verified product and return success or Freeze.

D0-B stops before production connection. `VerifiedResolvedIfFlowV1` alone is
not a recipe producer because it does not carry the condition type, branch
assignment cardinality, or post-merge read. A shape-specific builder-free
projection (or an exact preflight fact product) must be named before D0-C.
Schema/verify/normalize tests must prove deterministic semantic output and
reject every omitted obligation without touching a Builder.

### D0-B0 — same-pass facts projection (implemented witness)

Change:
: Add an owner-branded `VerifiedTrivialIfRecipeFactsV1` emitted by the existing
  trivial analyzer traversal. It records only the selected explicit-else
  shape's condition/value operations, branch writes, merge binding, and
  post-merge read witness; the portable mapper will later convert these to
  recipe-local keys.

Contract:
: `IfControl`, `ResolvedTrivialCanonicalOwnerV1`, and their verifiers retain
  their existing authorities. The new facts box is a same-pass witness, not a
  second policy analyzer, and it has no Builder, MIR IDs, route retry, or
  portable `BindingRefV1` ownership.

Done:
: The golden explicit-else fixture emits one owner-branded facts witness, while
  implicit-else and other non-selected shapes emit no facts. Existing analyzer
  admission/rejection remains the authority; this row adds no new accepted
  source shape and no Builder effect. Focused profile tests are green, the
  facts are collected during the existing traversal (no post-hoc source pass),
  and touched Rust/test files stay below 800 lines.

Stop:
: If the existing analyzer cannot emit the facts without a second source pass
or duplicated acceptance policy, stop and reopen D0-B design rather than
adding a second semantic owner.

### D0-B1 — portable schema, source claim, and structural verification

Change:
: Add an If-specific `IfRecipeArtifactV1` contract under
  `src/mir/if_recipe_contract/`. Use recipe-local binding/value/block/item keys
  and a fixed four-block shell (`condition`, `then`, `else`, `continuation`)
  for the selected explicit-else shape. Include `Explicit` and
  `ImplicitFallthrough` in the schema, but admit only `Explicit` in this row.

Contract:
: The artifact has no `BindingRefV1`, `FunctionOwnerIdV1`, AST/Located sites,
  `MirBuilder`, `ValueId`, `BasicBlockId`, route retry, or environment state.
  Source provenance is an ordinal owner plus a fail-closed structural path
  claim; it proves wire coverage/order only, not AST existence. The semantic
  verifier owns canonical keys, defined-before-use, value classes, explicit
  else, one same-binding write per branch, and the continuation-read
  obligation. Join predecessor/value-edge proof remains the later non-Clone
  `IfJoinSig` row. Do not reuse Loop-specific nodes, carriers, exits, or route
  IDs, and do not extract a shared control algebra in this row.

Done:
: A golden explicit-else artifact verifies and round-trips through deterministic
  semantic/source-bound normalization. Typed rejects cover wrong schema,
  unknown fields, key/order/duplicate/dangling rows, non-Bool condition,
  missing or mismatched branch writes, implicit-else, unsupported nested/exit
  operations, and missing/wrong continuation read. Repeated normalization is
  byte-stable, route/source receipts do not alter semantic normalization, and
  no production If caller or PHI/SSA writer changes.

Closeout:
: `src/mir/if_recipe_contract/` now owns the fixed-shell schema, recipe-local
  IDs, structural source-claim verifier, typed structural verifier, and
  semantic/source-bound normalizers. Five focused tests and the library check
  are green; every touched Rust file remains below 800 lines. The module has a
  README that fixes its authority and forbidden-dependency boundary. Commit
  `8999950faf` is pushed. The next row must design the facts-to-recipe mapper
  and source correspondence before any JoinSig or production wiring.

Stop:
: If facts cannot be mapped without reopening AST/source traversal, or if the
  schema requires Loop carrier/exit semantics or owner-branded identity, stop
  and return to this design boundary. The artifact verifier is now green; do
  not add the mapper or JoinSig in this closeout commit. They require the next
  source-witness design row.

### D0-C — one canonical production consumer

After D0-A/B, select one exact If shape and connect it through the existing
resolved source-bound candidate chain:

```text
one selection/preflight
  -> sealed IfRecipeV1
  -> CanonicalSsaFunctionSessionV2
  -> CanonicalCfgSessionV1 + PhiTxn
  -> one If merge physicalizer
  -> unpublished compile candidate
```

Do not create a second SSA/PHI transaction, do not connect directly to a raw
route registry, and do not preserve post-effect route retry at this seam.
Unsupported branch-transfer shapes must be typed rejects until their JoinSig
obligations are closed; the physicalizer must not repair missing predecessors
or invent PHI inputs.

### D0-D — canonical PHI/CFG adoption

For the selected If shape, make the canonical session the only production
writer for its branch blocks, merge block, predecessor seals, and PHI commit.
Classify and retire only the selected old writer in the same cutover. Then
repeat for remaining loop-variant and JoinIR writers. This is adoption of the
existing owner, not a new SSA design.

### D0-E — cheap cleanup, independently gated

S0 and the bounded S1 deletion are complete. The disconnected session,
adapters, test-only module registrations, and dedicated test modules are
gone; the carrier and its semantic evidence remain. S1 did not modify
normal/raw ingress, `located_if.rs`, `IfCfgSessionV1`, or the portable IfRecipe
design. Any future located lowering requires a new candidate-scoped caller
and a new design row.

This cleanup must not be mixed with If BoxCount or PHI owner adoption. The
raw/descent/parity trio for local/return/assignment/binary/short-circuit is a
later retirement lane after If parity is green.

## Acceptance gates

```text
D0-A: every If/PHI/CFG writer has one classified owner and caller set
D0-B: IfRecipeV1 has no AST/Builder/ValueId/BasicBlockId ownership
D0-B: Explicit(block) vs ImplicitFallthrough is represented distinctly
D0-B: selected explicit-else shape has two branch writes + post-merge read
D0-B: same-pass `VerifiedTrivialIfRecipeFactsV1` supplies all verifier inputs
D0-B1: schema/source-claim/verify/normalize rejection and deterministic
       semantic parity green
D0-B2: facts-to-recipe mapping has one same-pass source witness and no second
       acceptance policy (not started)
D0-C: selected recipe producer = exactly 1; physicalizer = exactly 1
D0-C: selected caller is inside an unpublished compile candidate
D0-D: selected old If/PHI writer caller = 0 after cutover
D0-D: selected shape has no post-effect Option/Retry/reselection
D0-D: legacy/new semantic digest, MIR/CFG, PHI, diagnostics, and reuse parity green
D0-D: injected late failure leaves live Builder/candidate owner unchanged
D0-E/S0: 48 test-only verifies, zero production roots, and retained carrier
D0-E/S1: only the proven disconnected session/adapters/tests are deleted; the
         post-retirement guard and focused checks are green
all touched Rust/test files < 800 lines
```

## Relationship to the Loop lane

This is queued after the active Nested D5-I0 consultation and the remembered
Loop convergence task:

```text
JOINIR-LOOP-RECURSIVE-FRAME-CONVERGENCE0-M12
```

If adoption may share the canonical `JoinSig` branch-transfer/merge owner,
but it must not reopen Generic post-effect debt or silently broaden Nested I0.
The Loop lane's bounded Nested work and the Located S0/S1 retirement are now
closed. This card is the next design target, not an implementation
authorization.
