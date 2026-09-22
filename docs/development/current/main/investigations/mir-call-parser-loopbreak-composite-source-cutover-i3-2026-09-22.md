---
Status: design_stop__2026-09-22__T4cNoSafeSliceScheduler
Task: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-CUTOVER-I3
Date: 2026-09-22
Parent: mir-call-parser-loopbreak-composite-source-physical-i2-2026-09-22.md
Implementation permission: false; no existing recursive-String result owner; design decision only
NextCard: none__t4c_recursive_string_result_NoSafeSlice__ResultFamilyOwnerAbsent
---

# Parser composite LoopBreak production cutover I3

## Six-line brief

```text
Decision: retain the selected composite LoopBreak handoff at the existing typed
  TargetOnly/RecursiveDependency terminal until a source-backed String result
  owner exists; do not bypass the terminal to force publication.
Source authority + canonical issuer: the existing result solver/catalog and
  VerifiedStaticCallResultPublicationOwnerV1, with the accepted I1 package and
  I2 physical owner remaining the downstream consumer boundary.
Non-authority: VM/compatibility lanes, generic fallback, names, AST rescans,
  and acceptance smoke results from an unselected backend.
Fail-fast boundary: selected caller, source-to-MIR terminal, exact target and
  site relation, one-shot handoff consumption, residual-free finish; caller-zero
  and physical deletion are owned by the successor retirement card.
Smallest next slice: design-only decision on termination, String
  representation, effect/ABI, and a named consumer or pre-effect terminal for
  the two recursive helpers; no implementation is authorized in this card.
Non-claims: no T4c publication acceptance, T5 caller switch, R0 deletion,
  whole-repository migration, backend promotion, or warning cleanup.
```

I1 and I2 are closed at their package/Recipe and focused physical boundaries.
Their design stop was accepted for those bounded slices. The earlier I3
result-ordinal concern is resolved by the real merged-parser probe and the
T4b representation guard; it is retained below as historical evidence only.
A failed or deferred source terminal reopens the owning semantic row; it does
not authorize a fallback or a VM repair. The predecessor existing-owner
pre-front structured-source I0 is closed at rows 1–4 and has handed this card
the named publication frontier. Once T4c and task 5 close, the successor
retirement card owns caller-zero and the exclusive delete set.

## Accepted design decision — 2026-09-22

The read-only publication audit is accepted as the implementation boundary for
this slice. The existing
`VerifiedStaticCallResultPublicationOwnerV1::take_for_source` remains the
canonical issuer and one-shot owner. The existing callable lowering ledger is
the transport, and the existing selected static-result physical bridge is the
only emitter. The composite LoopBreak source site, exact target, `ExactI64`
representation, required argument ordinals, and the handoff are co-sealed at
the selected parser tuple before physical emission.

The bounded implementation must fail fast on missing owner/catalog, foreign or
mismatched site/target, `TargetOnly`/`NoExactStaticTarget`, duplicate take, and
residual handoff. It may reuse the existing plan normalizer and physical
`GlobalCall` bridge, but it must not add a second publication authority, infer
from AST or names, revive VM/compatibility fallback, or alter generic/direct
LoopBreak routes. The acceptance invocation is only
`ParserProgramBox.parse/2 -> ParserStringUtilsBox.starts_with/3`; caller switch
and old-edge deletion remain task 5 and R0 respectively.

## Queue reconciliation — 2026-09-22 (historical, superseded by T4b closeout)

The warning cohort is deliberately paused at I147. `unused_imports=17` is the
remaining mechanical tail; `dead_code` remains owner debt and is not a reason
to keep the semantic lane waiting. No warning cleanup row is selected while
this parser publication boundary is open.

The earlier bounded order was:

1. **I3 task 4 — authority reconciliation, then publication acceptance:**
   resolve the selected `ParserProgramBox.parse/2 ->
   ParserStringUtilsBox.starts_with/3` `[1]` versus `[]` contract first; only
   then connect the source row to the existing one-shot publication owner.
2. **I3 task 5 — caller switch:** switch that selected parser caller to the
   source-backed composite LoopBreak route after the publication guard is
   green. Do not change VM/compatibility or generic fallback routes.
3. **R0 task 6 — retirement:** after the switch, prove caller-zero, delete the
   exclusive old edge and temporary assets, and retain a guard against
   re-entry.

The warning cohort may resume only after this I3/R0 sequence closes, or when
an owner-specific warning becomes a newly selected blocker. This ordering is
the task queue; it does not claim publication, cutover, or retirement yet.

## Queue correction after the real merged-parser probe — 2026-09-22

The real merged-parser probe supersedes the earlier `[1]` premise for this
selected row.  The probe built the merged `parser_program_box.hako` source,
issued the existing target inventory and result catalog, and selected
`ParserProgramBox.parse/2 -> ParserStringUtilsBox.starts_with/3` at
`[Body(22), LoopBody(5), IfThen(0), IfCondition, Lhs]`.  The observed values
were:

- target: `ParserStringUtilsBox.starts_with/3`;
- source witness: `parser_program_box.hako:164` (`starts_with("" + s, 0, ...)`);
- result disposition: `ExactI64 { required_i64_arguments: [] }`;
- call-row result: absent (`None`);
- publication handoff: present only with `required_callee_i64_arguments == []`.

Therefore the repository currently does **not** expose a sealed formal
`[1]` for this row.  The prior claim that the same call row retained formal
`[1]` and call-site `[]` came from a different substitution fixture where the
callee returned its second parameter.  `starts_with/3` returns only literal
`0`/`1`; its `i` parameter is used only in conditions/indexing, so the
existing result proof correctly emits an empty requirement.  The T4b field
rename remains a mechanical clarification, but it does not prove the
selected parser contract.

The read-only audit reconciles the route contract.  The physical publication bridge already lowers every source
argument site with `lower_all`; it does not consume the ordinal list.  No
distinct ABI/type authority for ordinal `1` exists in this owner.  The
selected route therefore needs the existing result owner's `ExactI64`
representation, while preserving whatever sealed formal ordinal list it
issues, including `[]` for this literal-`0` call.  No publication take, caller
switch, VM repair, fallback, or old-edge deletion is part of this slice.

The accepted bounded implementation is representation-only validation in the
existing source-route/facts owners.  It removes the stale fixed `[1]` policy;
it does not rewrite the result proof, mint a new ABI receipt, or discard the
ordinal list.  The next queue is:

1. **T4b representation guard:** replace the fixed ordinal checks in the
   existing LoopCond/LoopTrue/LoopBreak/composite source facts with the sealed
   `ExactI64` representation check; retain focused positive/negative guards.
2. **T4c publication acceptance:** take the real composite handoff and drain
   its residual owner only after T4a/T4b are green.
3. **T5 caller switch**, then **R0 caller-zero and old-edge retirement**.

Warning cleanup remains paused at I147 (`unused_imports=17`); `dead_code` is
owner debt and is not a reason to keep this authority decision unresolved.

## Recheck correction — 2026-09-22

The named no-selected-handoff frontier is earlier than the selected parser
tuple in the full merged package. A read-only owner audit and a temporary
test-only observation (removed after the run) found an armed
`StringHelpers.skip_ws/2 -> StringHelpers.is_space/1` row at
`[Body(4), LoopBody(0), IfCondition]`. The publication owner classifies this
callee as target-only because the current ExactI64 result contract cannot
prove it. The selected `ParserProgramBox.parse/2 -> starts_with/3` rows are
ExactI64, but they are not the first reached row.

This does not authorize filtering the target-only row: the accepted I0
contract requires every resolver item to be `SelectedStatic` or `CoreMethod`,
and the package acceptance D0 forbids target-only lowering or partial package
coverage. I3 remains queued until the static-result authority D1 card names an
existing owner or a typed terminal for this finite family. The earlier
preflight D1 closed with `NoSafeSlice__ResultFamilyOwnerAbsent` after observing
57 target-only rows; D0 closed with the same bounded NoSafeSlice and the new D1
now owns the static-result authority design. No code,
fallback, publication, caller switch, or retirement claim is made here.

## Acceptance frontier recheck — 2026-09-22

The selected merged parser inventory contains one composite LoopBreak candidate
after the earlier callable loop sites. The pre-front structured-source I0 now
consumes those finite rows, and the lifecycle invocation reaches the existing
named publication boundary
`[freeze:contract][callable-loop/static-publication/no-selected-handoff]`.
The selected `parse/2 -> starts_with/3` row is therefore visible to I3 task 4,
but its one-shot publication handoff has not yet been accepted. The old
`callable-loop/route-not-front-selected` / `GenericLoopV1NotSelected` terminal
is no longer the merged-parser frontier.

This is a publication boundary, not permission to add a fallback. No AST
rescan, name-based inference, VM repair, generic fallback, or new publication
authority is authorized.

The ordered queue is consequently:

1. **I3 task 4 — publication acceptance:** connect the selected source row to
   the existing one-shot publication owner and record the named outcome.
2. **I3 task 5 — caller switch:** switch only that selected parser caller after
   the publication guard is green.
3. **R0 task 6 — retirement:** prove caller-zero, remove the exclusive old
   edge and temporary assets, and retain the re-entry guard.

Until item 1 is green, items 2–3 remain queued and no production cutover or
retirement claim is made. The warning cohort remains paused at I147.

## I0 completion handoff — 2026-09-22

The predecessor static-result TargetOnly I0 is closed. Cataloged unavailable
rows now carry an explicit solver reason to the pre-effect terminal and the
merged parser guard observes
`static-result-ingress/target-only/RecursiveDependency` before argument
descent. I3 task 4 remains the selected bounded row, but it is paused at the
required-ordinal authority mismatch described below. Task 5 caller switch and
R0 caller-zero/deletion remain unopened.

## Current design evidence

The merged parser inventory reaches the selected `starts_with/3` rows, and the
pre-front consumer carries the finite preceding rows through the existing
source owner. The lifecycle invocation now stops at the named
`callable-loop/static-publication/no-selected-handoff` boundary. Task 4 must
reuse the existing publication ingress/physical bridge rather than add a
second authority or relax the terminal.

## I3 task 4 preflight evidence — 2026-09-22

The bounded handoff plumbing is present in the existing owners and their
focused guards. The selected composite LoopBreak row must still take the
existing static-result handoff, install it in the callable lowering ledger,
and let the source expression port consume it once. The normalizer and
physical bridge are already the selected `GlobalCall`/publication owners;
missing, foreign, mismatched, duplicate, and residual handoffs remain named
contract errors. Direct and generic LoopBreak routes are unchanged.

Focused evidence is green: source-route 16/16, raw child entry 11/11, raw
child-port 2/2, package 8/8, and publication bridge 2/2. `cargo check
--profile quick --lib` and `cargo fmt --all -- --check` also pass with the
existing warning baseline.

The end-to-end merged parser fixture now stops at the named
`[freeze:contract][callable-loop/static-publication/no-selected-handoff]`
terminal after the pre-front consumer. This evidence does not claim
publication acceptance or caller cutover. Task 5 remains unopened until task
4 proves the selected `parse/2 -> starts_with/3` handoff. R0 still owns
caller-zero and old-edge deletion.

## I0 handoff receipt — 2026-09-22

The predecessor I0 is closed at rows 1–4. Focused evidence is route 35/35,
package 19/19, finite parser source retention 1/1, and the merged parser
frontier guard 1/1. The source bridge reaches this I3 boundary without
changing VM/compatibility, generic fallback, or the direct LoopBreak route.

## Task 4 requirement recheck — 2026-09-22

A temporary focused probe (removed after the run) built the real merged parser
source, issued the existing whole-source target inventory and result catalog,
then issued `VerifiedStaticCallResultPublicationOwnerV1` for the selected
LoopBody row `ParserProgramBox.parse/2 ->
ParserStringUtilsBox.starts_with/3`. The row is `ExactI64` and the owner
provides a one-shot selected handoff, but its observed
`required_i64_arguments()` is `[]`.

The active tuple contract and the source-loop relation require `[1]`. This is
not a wrong-name or wrong-site selection: the row is the resolver-issued
LoopBody target for the selected caller. The existing
`source_proof::loop_invariant_ignores_condition_only_unknown_values` test also
records that the result proof does not add arguments used only in branch or
loop conditions. The parser `starts_with/3` body returns constants while its
index parameter appears in those conditions, so the current owner emits `[]`
under its existing authority.

### Authority split confirmed by read-only audit

The mismatch is a mixed tuple contract at the I3/D0 handoff boundary, not a
wrong target selection. `starts_with(src, i, pat)` has the callee formal i64
requirement `[1]`. The call proof substitutes that requirement through the
actual arguments; the selected parser call passes literal `0` at ordinal `1`,
so its selected call-site requirement is `[]`. The existing call row already
retains both facts: `callee_required_i64_arguments == [1]` and
`required_i64_arguments == []`. The publication handoff currently copies only
the call-site field, while `CallableLoopSourceTargetRequirementV1` and its
`has_exact_i64_requirement(&[1])` guards read that field as if it were the
callee formal requirement.

**Decision:** keep I3 task 4 in `design_stop` until these two meanings are
represented and consumed separately. Do not silently change `[1]` to `[]`,
mint a synthetic handoff, widen the physical relation, or switch the caller.
The next bounded task order is:

1. **T4a — authority decision:** keep the existing call-row evidence as the
   source of both values and decide whether the I3 guard can borrow the formal
   `[1]` there, or whether the existing handoff must expose that already-sealed
   formal evidence as an explicit field.
2. **T4b — focused contract guard:** add one owner-level check for the exact
   caller/site/target that asserts formal `[1]` and selected call-site `[]`
   together, then align the LoopBreak relation checks with their named field.
3. **T4c — publication acceptance:** rerun the selected one-shot owner take
   and residual guard only after T4a/T4b are green.
4. **T5 — caller switch**, then **R0 task 6 — caller-zero and old-edge
   retirement** remain queued behind T4c.

The warning cohort stays paused at I147 (`unused_imports=17`; `dead_code` is
owner debt). It resumes after T4/R0 or only if an owner-specific warning becomes
a newly selected blocker.

## Historical T4a authority decision — superseded by the real merged-parser probe

T4a is accepted. The existing call-row evidence remains the sole authority for
both ordinal meanings: `callee_required_i64_arguments` is the formal parameter
contract, while `required_i64_arguments` is the selected call-site dependency
after actual-argument substitution. The publication handoff is consumed by the
source LoopBreak relation, whose guards require the formal contract, so its
ordinal field is standardized as the already-sealed callee formal requirement.
The call-site dependency remains available only on the call row and is not
reissued through publication.

This is a field-meaning correction in the existing publication owner, not a new
semantic receipt or result authority. `from_exact_i64_requirement` already
copies the formal requirement; `from_general_call_result` must copy the same
formal slice from `same_module_static_evidence`. T4b may rename the existing
handoff/route accessors to make that meaning explicit and must add one focused
owner guard for formal `[1]` alongside call-site `[]`. T4c then accepts the
one-shot publication and residual check. No VM repair, fallback, AST rescan,
caller switch, or old-edge deletion is part of T4b/T4c.

## T4b mechanical field-meaning receipt — semantic acceptance superseded

The existing publication handoff now names and carries
`required_callee_i64_arguments`. Its general-row constructor copies the formal
slice from `same_module_static_evidence`; the call-row
`required_i64_arguments` remains the substituted call-site dependency. The
LoopBreak source relation and all three LoopCond/LoopTrue/Composite guards now
use the explicit formal accessor. No second result authority or new receipt was
introduced.

Focused unit evidence is green:

- `general_owner_handoff_preserves_callee_formal_for_literal_call_site`: 1/1;
  formal `[1]` and call-site `[]` are asserted together.
- `static_call_result_publication_owner`: 10/10.
- `normal_callable_loop_source_route`: 19/19.
- `cargo fmt --all -- --check` and `git diff --check` pass; the quick profile
  retains the existing 545-warning baseline.

This proves only the mechanical field rename and synthetic/literal owner
fixtures. It does not prove that the real merged parser row has formal `[1]`.
The real probe recorded `ExactI64` with `required_i64_arguments == []`, no
call-row result, and a handoff carrying `[]`; therefore T4c is reopened at
the source-result authority boundary. The composite consumer must not take
the row until the corrected T4a/T4b queue at the end of this card is green.

## Current superseding status — 2026-09-22

The source-result audit is complete.  The card is now in `fast` for the
representation-only guard slice above.  Earlier `[1]` acceptance wording is
historical and must not be used as an ABI or production cutover claim.

## T4b representation guard completion — 2026-09-22

The fixed ordinal policy is removed from the existing source-route and
LoopCond/LoopTrue/LoopBreak/composite facts owners.  Their guard now requires
only the sealed `ExactI64` result representation; the publication handoff
continues to carry the result owner's formal ordinal list unchanged.  The
physical bridge remains the sole argument consumer and checks full source
arity before `lower_all`.

Focused evidence is green:

- `normal_callable_loop_source_route`: **19/19**;
- `normal_callable_loop_source_facts::`: **16/16**;
- merged parser inventory/publication probe: **1/1**, observing
  `ParserProgramBox.parse/2 -> ParserStringUtilsBox.starts_with/3` with
  `ExactI64/[]`, one-shot take, and duplicate-take rejection;
- merged parser boundary suite: **3/3**;
- `cargo fmt --all`, `git diff --check`, and the current-state pointer guard
  pass; the existing quick-profile warning baseline remains 545.

This closes only T4b's representation guard.  The actual composite LoopBreak
consumer still must take the selected handoff and drain the residual owner in
T4c.  Caller switch (T5), caller-zero proof, and old-edge deletion (R0) remain
queued; warning cleanup stays paused at I147.

## Current task queue after T4b — 2026-09-22

1. **T4c — publication acceptance (selected now):** exercise the real
   composite LoopBreak consumer for `ParserProgramBox.parse/2 ->
   ParserStringUtilsBox.starts_with/3`; consume the existing one-shot handoff,
   lower through the selected publication bridge, and require a residual-free
   finish. Keep the direct singleton and GenericLoop routes unchanged.
2. **T5 — caller switch:** only after T4c is green, switch that one parser
   caller to the source-backed composite route and retain the named terminal
   guards. VM/compatibility lanes and unrelated callers remain out of scope.
3. **R0 task 6 — retirement:** after T5, prove caller-zero, remove the
   exclusive old edge and temporary assets, and keep a re-entry guard.
4. **Warning cohort:** remain paused at I147 (`unused_imports=17`). The
   remaining `dead_code` warnings stay with their owners and resume only after
   this semantic sequence closes or an owner-specific warning becomes the
   selected blocker.

## T4c physical-consumer guard — 2026-09-22

The existing `CorePlanEffectEmissionPortV1::SourcePublication` consumer now
has a focused positive/negative guard.  The test installs the existing
one-shot handoff in `CallableSemanticLoweringState`, marks the exact source
call consumed, emits the real `GlobalCall` through the selected publication
bridge, rejects a duplicate take, and requires a residual-free `finish`.
The companion negative test leaves the handoff unconsumed and requires the
named `static-publication/residual` terminal.  Both tests pass (2/2), with
the existing 545-warning baseline.

This closes the physical consumer guard only.  It does not claim that the
full merged `ParserProgramBox.parse/2 -> ParserStringUtilsBox.starts_with/3`
source-to-MIR invocation has completed; that canonical acceptance remains
the next T4c evidence step.  T5 caller switch and R0 caller-zero/deletion
remain unopened, and the I147 warning cohort stays paused.

## Canonical merged-parser recheck — 2026-09-22

The existing full-source lifecycle test still passes at its named terminal,
`static-result-ingress/target-only/RecursiveDependency`.  The merged source
inventory separately observes and selects the `starts_with/3` LoopBody row,
but the lifecycle does not reach its physical consumer because an earlier
target-only dependency terminates the package first.  This is dependency
evidence, not T4c publication acceptance.  The target-only row must remain a
typed fail-fast terminal; filtering it, adding a fallback, or switching the
caller would invalidate the accepted package boundary.

T4c therefore remains open for a bounded bridge decision: reconcile the
existing target-only terminal with the selected composite acceptance using
the current result/publication authority, or record a named `NoSafeSlice` if
no existing owner can carry that dependency.  No new result receipt,
synthetic fixture, VM route, T5 switch, or R0 deletion is authorized by this
recheck.

## T4c recursive-result authority design stop — 2026-09-22

The upstream `RecursiveDependency` rows are finite and identified: the
merged parser reaches `ParserStringUtilsBox.i2s/1` and
`StringHelpers.int_to_str/1`, whose bodies recursively call themselves.  The
existing monotone result solver intentionally seals stalled worklist rows as
`Unavailable(RecursiveDependency)`.  The publication owner then preserves
that disposition as `TargetOnly`, and ingress stops before argument descent or
Builder effects.  This is the intended fail-fast boundary, not a missing
`starts_with/3` handoff.

```text
Decision: keep the typed recursive-dependency terminal; do not bypass it to
  reach the selected composite row.
Source authority + canonical issuer: existing result solver/catalog and
  VerifiedStaticCallResultPublicationOwnerV1.
Non-authority: method names, selected-row filtering, VM/compatibility fallback,
  fixture shrinking, synthetic recursion receipts, or a new result ABI.
Fail-fast boundary: every cataloged caller/site is Selected or TargetOnly;
  TargetOnly stops before receiver/argument descent and physical effects.
Smallest next slice: decide whether an existing owner can prove termination,
  representation, effect, and ABI for exactly these two recursive helpers; if
  not, keep the family explicitly parked at NoSafeSlice.
Non-claims: no T4c full acceptance, T5 caller switch, or R0 deletion.
```

Design stop is local to this recursive result family.  The audit produced the
explicit `NoSafeSlice` decision, so no code, fallback, production switch, or
warning cleanup is permitted until a separate owner design is accepted.

## Scheduler premise reset — 2026-09-22

The read-only audit is complete and the missing owner is internal, not an
external wait.  The scheduler checked the already-inventoried alternatives:

- `MIRBUILDER-FINAL-ACCEPTANCE-SCOPE-R0` is already closed with its fixed
  eleven-entry receipt.
- `MIR-CALL-R7-STRINGBOX-LOWER-STRUCTURAL-MEMBERSHIP-I0` is implemented, but
  its dynamic owner evidence stops at the same static-call terminal; bypassing
  that terminal would be a new route.
- `MIR-CALL-STATIC-COMPATIBILITY-CATALOG-TARGET-I0` is parked at the parser
  LoopCond/static-publication boundary and cannot be resumed by filtering the
  recursive rows.
- OwnedText T3 and the remaining M7-S matrix are already
  `ParkedSealed`/`NoSafeSlice` with no exclusive delete-set.

There is therefore no ready Promote, Stop, or Delete row in this family.  The
only unresolved design is a source-backed owner for the two self-recursive
String helpers: bounded termination proof, String result representation,
effect/ABI relation, and a named physical consumer or pre-effect terminal.
The existing catalog/publication owner does not provide that combination, and
the active T4c contract forbids inventing it inside the LoopBreak cutover.

The family remains explicitly parked at
`NoSafeSlice__ResultFamilyOwnerAbsent`.  Reopen only with an accepted owner
design (or a separately selected inventoried family); do not add a synthetic
receipt, widen `ExactI64`, shrink the parser fixture, restore compatibility
fallback, repair VM, or switch T5/R0.  I147 remains the warning boundary and
`dead_code` remains owner debt.

## Task order after the I147 warning boundary — 2026-09-22

The warning cohort is closed for this workstream at I147.  `unused_imports=17`
is the measured mechanical tail; `dead_code` remains owner debt and is not a
shared cleanup task.  The next work therefore stays on the semantic LoopBreak
lane, with no new warning row unless one becomes an owner-specific blocker.

The current recursive-result audit has a finite answer: the two helpers
`ParserStringUtilsBox.i2s/1` and `StringHelpers.int_to_str/1` are self-recursive,
the existing solver has no termination proof for them, and the current result
owner exposes only exact `i64` or exact nominal-Box representations.  There is
no existing String/OwnedText result ABI for this caller family.  The correct
classification is `NoSafeSlice__ResultFamilyOwnerAbsent`; do not filter those
rows, shrink the parser fixture, add a fallback, or mint a synthetic result.

The bounded queue is consequently:

1. **T4c authority closeout — closed here:** keep the typed
   `TargetOnly/RecursiveDependency` terminal under the recorded
   `NoSafeSlice__ResultFamilyOwnerAbsent` decision.  This closes the audit
   boundary but does not claim merged-parser publication.
2. **T4c publication acceptance — parked:** reopen only when an existing owner
   can carry the required termination, representation, effect, and ABI
   contract; then consume the selected composite handoff once and finish
   residual-free.
3. **T5 caller switch:** after T4c is green, switch only the selected parser
   caller to the source-backed LoopBreak route.
4. **R0 task 6 retirement:** after T5, prove caller-zero, delete the selected
   old edge and temporary assets, and retain the re-entry guard.

Until item 2 has an accepted owner, items 3--4 remain unopened.  No VM or
compatibility repair, generic fallback, broad result-ABI expansion, or warning
marathon is part of this queue.
