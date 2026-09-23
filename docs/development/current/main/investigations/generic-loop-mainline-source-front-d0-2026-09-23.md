---
Status: fast__main0_continue_semantic_program_decision_accepted__2026-09-23
Task: GENERIC-LOOP-MAINLINE-SOURCE-FRONT-D0
Date: 2026-09-23
Parent: generic-post-selection-static-call-site-identity-d0-2026-09-23.md
PreviousCard: generic-legacy-observation-front-g0-premise-recheck-2026-09-23.md
NextCard: implement_main0_continue_semantic_program_and_selected_cutover
Implementation permission: true for this card's exact Main0 Continue profile, its canonical Main publication, and removal of that profile's raw body/raw finish edge in the same bounded series.
---

# Main0 Continue source-to-portable migration

## Six-line brief

```
Decision: select the existing Main0 Continue source profile for E; select pre-wrapper admission and one canonical main draft in the existing normal/default invocation. This is BoxCount design, not an already-working refactor.
Source authority + canonical issuer: the same prepared source/parser identity and installed callable package/batch lend Main's resolver input once; compiler source projection owns Facts, the portable producer owns Recipe keys, and existing JoinSig/Completion owners retain control meaning.
Non-authority: fixture/backend names, legacy route IDs, VM output, diagnostic tags, physical MIR types/effects, and another G0 or CallableSingleLoop product cannot issue this profile's meaning.
Fail-fast boundary: missing/foreign/duplicate source, control, ABI or tail/result contracts reject before physical work; actual Completion/DraftSeal is produced and checked at finish. Later failure discards the unpublished invocation, with no retry through raw Main or Compatibility.
Smallest next slice: close this selected Main0 source semantic-program admission: exact membership, initialized inputs, control/result/effect/ABI, one source loan, and finished-root handoff. Do not search for another already-complete tuple.
Non-claims: no runtime admission/first-terminal proof, new Facts/Recipe implementation, common If/Continue physical coverage, production cutover, old-edge deletion, or M8/M9/M10b completion.
```

Boundary: selected MIR source preparation -> same-package Main0 admission ->
whole-function portable lowering -> canonical finished root -> existing module
validation/publication. Includes that profile's raw body and raw finish edges;
excludes other Main profiles, VM restoration, B3/R7 reopening and whole shared
owner deletion. This is a finite design target, not an all-family census.

## Target selection is closed

Use the unchanged source at
`apps/tests/phase29ca_generic_loop_continue_min.hako` as the first acceptance
input. Its declaration is `Main.main/0`; its four root statements are two
initialized locals, one predicate Loop and the carrier's tail Return.
Selection must use the source-owned App Main identity and exact membership,
never the pathname, spelling of local names, or the values `0/4/1`.

The first design domain is a single App Main0 with no helper declarations,
top-level runtime work, parameters, captures or calls: two Integer-literal
initialized locals; `carrier < invariant_bound`; a no-else `carrier == Integer literal` guard
whose body increments that carrier by one and continues to this Loop; a
normal increment by one; and a tail Return of the carrier. Initial values,
bound and comparison literal are source data. Unit steps are an explicit
bounded arithmetic restriction, not a general integer-operator claim.

This selects work even though its new issuer/consumer is not implemented.
The source-backed old path has an unsupported disposition here, so adding
support is BoxCount. Do not describe it as behavior-preserving BoxShape or
repair the old route first and count that as portable migration.

### Static classification, not a runtime receipt

The following evidence is from source at `5b250d051e`. Given this unchanged
Loop AST and successful Facts/planner processing, the legacy selection is
exactly `[LoopCondContinueOnly]`:

| Decision | Evidence under `src/mir/builder/` |
| --- | --- |
| Continue-only conditional shape accepts | `control_flow/facts/loop_cond_continue_only.rs:25-115,171-224`; its gate in `control_flow/plan/loop_cond_unified_helpers.rs:14-31` is enabled. |
| LoopContinueOnly does not accept | `control_flow/plan/facts/loop_continue_only_facts.rs:85-123,206-231` requires a literal bound and another updated carrier. This source has neither. |
| Generic V0/V1 cannot overlap | `control_flow/plan/facts/loop_builder.rs:189-204` suppresses both Facts when ContinueOnly Facts exist. The earlier audit read registry suppression without this producer decision. |
| Other registry routes do not win | Full registry/predicate/suppression review: `control_flow/joinir/route_entry/registry/{mod,predicates,selection}.rs`. Break, Return, nested-loop, explicit-else, literal-true and scan shapes are absent; BreakContinue is absent or suppressed. |
| Source issuer lacks this arm | `normal_callable_loop_source_facts/generic/issuer.rs:54-184` accepts exact GenericV1/V0+V1, BreakContinue and LoopTrue. If reached, this route returns `RouteNotFrontSelected(GenericLoopV1NotSelected)`. |

Earlier Main admission and the first terminal of a real compiler invocation
remain unobserved. `NoExistingObserver` means that missing evidence, not an
absent source owner or an external dependency. The manifest stays unobserved;
the classification above does not rewrite it into a runtime PASS.

## Actual caller and root strategy

The old responsibility is the selected Main profile's dispatch through
`normal_callable_semantic_loan_port/main_root.rs:253-254`:
`inner.lower_body(builder, body)`. The general cataloged-static
`CanonicalCallableRouteV1::Outside` seam is a different caller. The GenericV1
adapter and `route_loop_cond_continue_only` are not established consumers of
this source-backed invocation and are not this row's deletion set.

Accepted structural Decision: keep one existing normal/default invocation,
one physical `main`, and one outer publication. Split module-shell preparation
from legacy root opening; select from the installed source package before
opening a function. The selected profile uses the existing canonical function
session and DraftSeal. It hands one finished root to the existing collector
and validation chain. This is a plan to extend those owners, not a claim that
their current APIs already accept the new input.

| Existing owner | Required change in this migration |
| --- | --- |
| `normal_default_root_catalog_lifecycle.rs` and `module_lifecycle.rs:359-405` | Separate module/catalog preparation from legacy wrapper creation. Preserve catalog clearing/install order; select before entry blocks, hints or Safepoint are emitted. Nonselected inputs retain their existing path. |
| `normal_callable_semantic_package/install/lowering_port.rs:43-121` | Use the same installed batch and parser identity. The Main loan is one-shot: selection and either downstream branch share that one consumption; never probe it and then take it again in the old Main hook. |
| `calls::CanonicalFunctionLoweringSessionV1`, `CanonicalSsaFunctionSessionV2`, DraftSeal | Lower the entire selected Main, including initializers and tail, into one fresh unpublished draft. Settle source header/result/effect/continuation contracts before opening it; produce physical completion and DraftSeal at finish. |
| `module_draft_collector/normal_collector_drain_lifecycle.rs:255-305` | Add exact canonical Main admission with `CanonicalRejectDuplicate` and the same invocation brand. Current normal drain rejects Main as `NonLegacyKey`; do not accept arbitrary keys or overwrite a root. |
| `module_lifecycle.rs:422-619` | Add a finished-canonical-root disposition which consumes the draft without raw Return insertion, return-type inference or PHI repair, including the later all-functions repair loop. Share applicable module metadata/validation, not competing function-finish authority. |
| `normal_default_root_final_validation.rs` | Preserve same-package App Main registration, `RootValidation::OrdinaryNew`, object-definition and static-result residual checks, package `finish()`, and artifact/source validators. No `Absent`/empty default substitutes. |

Paths in this table are under `src/mir/builder/`, except the package port under
`src/mir/`. The existing compiler normal/default finish -> source validation ->
backend view/callback -> invocation external commit remains the publication
owner (`src/mir/compiler/normal_default_pipeline.rs:527-548`).

Rejected alternatives: adopting the supposedly empty old wrapper requires a
new proof for its region/slot/hint/Safepoint state and still leaves raw finish
authority; inserting a fresh function in `main_root` is too late. The separate
canonical Main transaction creates a two-function source+thunk module and
has another publication route. Its failure discipline is a useful precedent,
but neither its extra thunk nor its module publication belongs in this slice.

## Source-to-Recipe obligations for the selected target

| Source role | Required semantic mapping and existing owner |
| --- | --- |
| Two initialized locals | Same resolver `BindingRef`, declaration and initializer sites -> complete initialized-local input relations. Carry the bound unchanged; do not replace its read with a constant inferred from physical MIR. |
| `carrier < bound` | Source Integer representation -> existing V1 I64 reads and Less/Bool predicate. Both operands keep their exact source bindings. |
| Equality and then update | Existing V1 Equal and Add/Write operations, with distinct branch and assignment sites. The producer alone issues their Recipe keys. |
| Same-loop Continue | Resolver exit target -> item-keyed JoinSig Continue carrying the updated carrier to Header. The absent else means logical Fallthrough to the normal update, not a synthetic source node. |
| Normal update | Separate read/Add/Write source sites -> ordinary backedge. It is not executed after the Continue arm. |
| Tail Return | The same Core's JoinSig After binding -> source Return, Integer result/ABI, Completion and DraftSeal. It is outside the Loop and is not `LoopExit::Return`. |
| Function effects | Source coverage proves local writes/control only, with no calls, allocation or external mutation. Do not issue semantic effect from MIR `EffectMask`. |

The V1 algebra and `join_sig_branch.rs` already express these logical control
roles. Their physical coverage is missing: `join_sig/transfer_view_v1.rs`
exposes boundary edges only; `physical_layout.rs:403-407` rejects If/Exit.
Extend the existing JoinSig view with exact branch/exit-item evidence and
the existing common physicalizer; do not duplicate a route-local CFG/PHI
owner or silently coerce through V2. This target needs If and Continue;
Always, Break and in-loop Return are not its entry prerequisites.

`CallableSingleLoop` requires a call prelude; G0 requires its own typed
parameters/result and nested loops. Neither product can be relabelled as
this Main. The missing source projection and one semantic-program issuance
belong to the existing compiler source/normal package and portable producer
owners. The issuer must consume the source context, source-bound Core,
input/item/carrier relations and its own JoinSig continuation together.

## Accepted source result and numeric contract

The source result stays **unannotated**. `Main.main/0` is an ordinary method;
its explicit `return` produces a source value, while omission of `: T` is not
static return-type inference. The selected issuer therefore does not invent
`: i64` or use `MirType::Integer` as proof. It proves the selected tail's
source result representation from the same source-owned value/control graph,
then hands the existing Main entry-result owner `Integer` via its existing
`VerifiedNormalMainThunkResultV1::Integer` case. This is an entry-result
classification, not a new language type or ABI type.

The exact Main0 issuer owns the missing co-seal: both local `BindingRef`s and
their literal initializers; the invariant bound; `Less` and `Equal` operand
relations; both distinct `Add(1)`/local-write sites; the then-branch Continue
target; the no-else fallthrough; the ordinary backedge; and the tail Return of
the same carrier after loop exit. It also proves complete source coverage and
the absence of calls, allocation, field/external mutation, or other effects.
This is a bounded source effect/result contract, not a MIR `EffectMask` or
backend observation.

The selected syntax accepts only plain `Integer` literals and the existing
dynamic `Integer(i64)` value lane. The tokenizer parses integer literals into
`i64`; `Integer + Integer` and same-kind Integer comparison have existing
source semantics. Both updates execute only while `carrier < invariant_bound`.
Since the bound is itself a representable Integer value, each executed
`carrier + 1` is representable (`carrier < bound <= i64::MAX`); no general
overflow, wrapping, termination, or exact-width policy is introduced. The
tail representation is `InlineI64` only as a proved physical carrier. The
existing completion owner separately seals one explicit value Return with
the declaration still `Unannotated`; actual physical Completion and DraftSeal
are produced and checked at function finish.

Reuse the existing Main result/entry contract and `VerifiedFunctionCompletion`
owner, extending their input only to consume this same-source Main0 result
projection. Do not reuse Generic G0's two-parameter/four-literal numeric
product, the CallableSingleLoop product, an annotation-only `ExactTrivialReturnAbi`,
or a second Main thunk/publication route. A missing, conflicting, foreign, or
uncovered source/result/control product rejects before physical opening.

Evidence anchors: plain integer tokens are parsed to `i64` in
`crates/hakorune_frontend_parser/src/tokenizer/lex_number.rs`; the AST stores
`LiteralValue::Integer(i64)` in `crates/hakorune_frontend_ast/src/literal.rs`;
the runtime dynamic Integer lane and same-kind comparisons live in
`docs/reference/language/types.md`; `Main.main/0` explicit-return and
unannotated-result semantics are in
`docs/reference/language/function-exit-and-entry-result.md`; the existing
source-entry `Integer` mapping is
`src/mir/compiler/normal_source_plan/main_thunk_plan.rs` (`seal_result`).
These authorities establish literal/value and entry-result meaning; the new
bounded issuer still has to establish their exact same-source relation.

## Ordered work and exit conditions

1. **Done: select the target and root strategy.** No more candidate search or
   observation-only census is needed for this responsibility.
2. **Done: accept Main0 semantic-program admission.** The exact source issuer
   is the one-shot App Main loan from the installed parser/package batch plus
   its `ResolvedFunctionLoweringInputV1` Core. It issues one complete
   source-indexed program co-seal for membership, value/control/effect/result
   and JoinSig continuation. The source result remains Unannotated; its
   Integer entry-result representation is passed to the existing Main result
   owner. The numeric range proof is local to this exact source profile.
3. **Current: implement the source/Recipe contract and required common control.**
   Reuse the existing semantic-family selector and recursive Loop algebra;
   old route IDs remain migration evidence. Implement exact source Facts,
   same-parent Core/input/control/Completion co-seal and the needed common
   If/Continue coverage. Keep the existing coverage owners and separate
   BoxCount from prerequisite behavior-preserving owner splits.
4. **Switch and retire in the same migration series.** Integrate before the
   legacy wrapper, admit one canonical Main draft, and remove the selected
   profile's raw body and raw finish edges. Do not first wire a new
   ContinueOnly source arm into old CorePlan as a substitute milestone.
5. **Close production acceptance and deletion evidence.** Use the actual
   selected source invocation through publication, not a hand-built AST or
   caller-zero Recipe test. Record selected old-edge zero and retained users.

Entry requires the accepted source/result/effect/control mapping and a
deletion/acceptance plan, now recorded above. The new code, after-change PASS
and resulting caller-zero are outputs, not entry conditions. Implementation
is limited to this selected profile and must preserve the one-shot source
loan, one physical Main and one outer publication. No CI/person wait has been
identified.

Planned admission outcomes must remain separate: complete source membership
is a candidate for canonical selection; an out-of-profile source stays on its
existing path before selection; missing required evidence is unresolved;
foreign/duplicate/conflicting evidence is rejected. None of the latter states
may become an empty candidate. Once selected, failures discard the invocation
and cannot re-enter the nonselected path.

## Acceptance and retained scope

- Unchanged fixture: expected result `4`; a bound `2`, guard `1` variant must
  return `2`, catching an incorrect extra normal update that would yield `3`.
  Result `4` alone does not prove Continue evaluation order.
- Upper-bound cases must exercise both distinct increment sites without
  overflow: with `carrier = i64::MAX - 1`, `bound = i64::MAX`, one case takes
  the Continue branch and one falls through the normal update; both return
  `i64::MAX`.
- Zero-iteration and always-false guard cases; renamed locals preserve the
  same membership. Changed bound writes, another assignment target, else,
  Break, nested Loop, body Return, calls or non-unit steps are outside this
  first profile, with explicit preselection disposition.
- Missing/foreign/duplicate source, control, input, ABI or Completion;
  double consumption and residual sites reject. Later lower/DraftSeal/drain/
  validation failures publish nothing; a fresh invocation remains usable.
- Success contains one physical Main and one outer publication. Selected
  source cannot reach raw body/raw finish. Real source-to-MIR and selected
  executable result evidence are required before production closeout.

Shared raw body dispatch, other Main profiles, GenericV1/LoopCond/LoopTrue
consumers, registry/Composer/PlanLowerer and all nonselected callers remain.
Removing this caller-local responsibility does not authorize whole-symbol
deletion or close the wider M8/M9/M10b requirements.

## In-progress implementation checkpoint — 2026-09-23

The first common physical-control step is implemented:
`join_sig/transfer_view_v1.rs` exposes the exact verified branch and exit-item
relation, and `physical_layout.rs` consumes it for the bounded same-loop
If/Continue shape. Layout now rejects a branch row that no Recipe item
consumes. The focused test proves the Continue arm returns to the loop header,
the fallthrough arm reaches the normal continuation, and an extra branch row
is rejected:

```text
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib if_continue_layout_splits_the_source_block_and_targets_the_loop_header
PASS: 1 passed, 8330 filtered out; quick build/test completed in 4m48s.
```

The command emitted 543 library warnings; this checkpoint does not classify or
suppress that warning cohort. The physicalizer tests were moved to
`physical_layout_tests.rs` so the production module stays below the 760-line
split threshold. This is unit evidence for common physical control only. The
selected source issuer, Main co-seal, canonical-root handoff, production
switch, old-edge retirement, and runtime acceptance remain unfinished. The
physicalizer is committed at `e671cf390e`; Main0 source Facts, mapping, Recipe
co-seal, RecipeDraft, and tests are committed at `4b80242fa7`.

After a restart, read `CURRENT_STATE.toml` and this card, check `git status -sb`,
and inspect running `cargo`/`rustc` processes before starting another Cargo
command. The Main0 modules now compile and their 12 focused tests pass. They use
synthetic AST input plus the embedded resolver; they do not prove parser
admission of the on-disk fixture or production caller selection. Next, connect
the verified product to the canonical-root handoff from the same installed
batch source loan. Keep pre-wrapper selection and one outer publication; do not
repeat the focused tests unless these owners change.

## Producer-architecture feedback tasks — 2026-09-23

User feedback on the existing portable-Recipe producers was checked against
the code and is recorded here as the standing task queue for this family.

Verified findings:

- Fixed-template producers: `variable_accum_break_producer.rs:161-420`
  hand-numbers items 0..19 and values 0..16 and hardcodes deltas 10/1; the
  matching Facts reject `delta() != 10` at
  `loop_structural_facts/variable_accum_break.rs:590`. `generic_g0/recipe.rs`
  and `callable_single_loop_recipe.rs` use the same fixed-key tables. The
  README rule "do not turn 19 legacy routes into 19 recipe variants" is being
  satisfied only in name.
- Duplicated wiring and double verification: `verify(recipe.clone())` then
  `verify_artifact` re-verifies the same recipe in
  `direct_accum_producer.rs:62-80`,
  `variable_accum_break_producer.rs:108-150`, and
  `callable_single_loop_recipe_coseal.rs`. The early `verify` exists only
  because `VerifiedLoopRootSourceV1::into_root_claim` correctly requires a
  verified recipe as the root-key authority.
- Dual bookkeeping: producers know each item's source site while building the
  recipe, discard it, then rebuild the same correspondence by hand in
  `effect_relations()` / `operation_evidence()` tables that Core re-verifies.
- Debt inventory: `loop_recipe_contract` alone is ~25.4k lines; `src/mir` has
  273 `allow(dead_code)` sites and ~205 caller-zero comments. V1/V2 schema,
  recipe_view, and transfer_view families are still duplicated.

Task queue (priority order, from the same feedback):

1. A+B (this slice, bounded): add a `LoopRecipeDraftV1` builder inside
   `loop_recipe_contract` that allocates canonical preorder keys and records
   each pushed item's source anchor once, emitting the recipe plus binding,
   effect, input, and operation-evidence relations as one byproduct. Facts
   keep admission judgment only. The Main0 Continue issuer is the first
   consumer; it must not become another hand-numbered template.
2. C (follow-up): one shared seal entry (verify -> claim -> artifact ->
   JoinSig -> Core -> inputs -> continuation) replacing the per-producer
   wiring, with typestate where cheap. Removing the second recipe
   verification needs a `verify_artifact` path that accepts an already
   verified recipe; do that when the shared seal is introduced, not before.
3. D+E (defer to retirement stage): collapse V1/V2 schema/view duplication
   behind one schema with verifier profiles; consider exposing JoinSig as a
   derived view instead of a stored product.

## Main0 Continue co-seal checkpoint — 2026-09-23

Task-queue item 1 (A+B, bounded) is implemented. The canonical draft builder
`loop_recipe_contract/recipe_draft.rs` allocates all preorder Recipe keys and
records each pushed item's source anchor once, emitting the recipe plus
binding, effect, initialized-input, and operation-evidence relations as one
construction byproduct. `LoopRecipeVerifierV1` remains the sole semantic
authority; the draft only assembles.

The first consumer is the Main0 Continue source front for the selected
`Main.main/0` profile (`apps/tests/phase29ca_generic_loop_continue_min.hako`,
expected result `4`):

- `compiler/main0_continue_syntax_facts.rs` — AST-free admission facts for
  the exact `local i, local n, loop(i<n){ if i==1 {i+=1; continue} i+=1 }
  return i` shape; no Recipe keys, selectors, or physical IDs.
- `compiler/main0_continue_source_map.rs` — joins the verified facts to the
  resolver ledger: declaration bindings, condition/guard/step reads and
  writes, Continue validated via `ResolvedControlTransferV1::Continue`
  against the resolver-issued loop `RegionId`, terminal return validated via
  resolver exits. Residual refs/exits/calls, duplicates, foreign sites, and
  mismatched operators reject.
- `compiler/main0_continue_recipe_coseal.rs` — consumes the map once, builds
  the recipe through `LoopRecipeDraftV1` (no hand-numbered tables; `n` is a
  loop-available carrier because JoinSig requires loop-body reads to be
  carrier-backed), then issues artifact -> verify -> source claim ->
  source-bound Core -> initialized inputs -> JoinSig -> continuation ->
  operation/effect evidence as one semantic-program admission.

Focused evidence:

```text
cargo test --lib main0_continue
PASS: 12 passed, 0 failed, 8331 filtered out.
```

Coverage includes positive co-seal, recipe shape/counts, product ownership
after source-unit drop, and negative facts/map/co-seal rows (else arm,
missing Continue, foreign ledger, non-Less operator, wrong tail read,
missing map row, extra root statement, second loop, non-literal guard).

Non-claims: this is unit/logical evidence only. No production caller switch,
no Main0 runtime or source-to-MIR acceptance, no physical Main handoff, no
outer-publication count proof, no legacy retirement, and no zero-fallback
production proof. The remaining task-queue items (shared seal wiring, V1/V2
collapse) stay open.

## Review and validation

Two read-only workers covered independent uncertainties: complete legacy
classification/source issuer, then Main's fresh-function/publication seam.
The primary checked source/Recipe/input/JoinSig contracts and integrated the
one-target/one-root Decision. No worker edited files or ran compiler probes.
The focused common If/Continue physicalizer test passes. With the Main0
modules and tests registered, `CARGO_BUILD_JOBS=4 cargo test --profile quick
--lib main0_continue_recipe_coseal_tests::` passed 12/12 in a 4m37s quick build
and emitted 543 library warnings. The suite covers the issuer against synthetic
AST plus embedded resolver input; exact parser-to-MIR, runtime, LLVM, and CI
acceptance remain unproven.
