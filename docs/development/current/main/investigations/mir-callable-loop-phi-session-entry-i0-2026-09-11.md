---
Status: fast__SourceIndexHeaderPreludeHandoffI0
Date: 2026-09-11
Decision: MIR-CALLABLE-LOOP-PHI-SESSION-ENTRY-I0
Parent: mir-callable-loop-phi-canonical-session-bridge-d0-2026-09-11
---

# MIR-CALLABLE-LOOP-PHI-SESSION-ENTRY-I0

Active execution row: `MIR-CALLABLE-LOOP-PHI-SOURCE-INDEX-HEADER-PRELUDE-HANDOFF-I0`.

## Boundary

```text
starts: selected static-callable entry after a source-bound Loop capability is issued
ends: one unpublished function session capability consumed by the selected
      canonical Loop consumer
includes: session ownership handoff, logical Recipe consumption, canonical
          BindingRef -> block/value reads, CFG edges, PHI seal, and discard;
          short reborrow of the raw child port
excludes: new source Facts/Recipe issuer, plan-level PHI adapter, local
          completion handoff, generic fallback/retry, backend/OBJ/EXE, and R7;
          the generic RawInvocationChildPortV1::lower_loop Ready branch remains
          a follow-up slice
```

The selected production edge is the
`CallableSingleLoop` branch in
`NormalCallableSemanticPackagePortAdapterV1::lower_cataloged_static_box_method`.
The semantic demand and canonical PHI/CFG physicalizer are wired. The
session-entry implementation now opens one
`CanonicalFunctionLoweringSessionV1` at that selected edge, lends it to
`callable_lowerer.rs`, and moves the prepared DraftSeal into the existing
pending restoration terminal before collector admission. The session bridge
is focused-tested, but this row is not fully accepted until the real selected
module publication path proves zero, one, and multiple iterations without
manual ledger setup.

The PHI value-flow contract is already recorded in
`docs/reference/mir/loop-recipe-contract.md:1227` and is not missing a second
SSOT. It names the source `BindingRef` owner, the canonical physical
Binding SSA/PHI issuer, the `h_n -> s_n -> h_(n+1)` edge relation, the false
edge `After` rule, and the named pre-effect rejects. No additional PHI
receipt, issuer, or design layer is opened by this review.

The first real package-to-production probe exposed a separate authority gap
before PHI emission. `SelectedCallableLoweringInputRefV1` is intentionally
created by the semantic batch through
`ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable`, so a
selected non-AppMain child has no attached `VerifiedCallableIndexV1` or
`VerifiedCallableHeaderV1`. The physicalizer currently requires both and
stops at the named `[freeze:contract][callable-loop/missing-index]` boundary.
The existing `ResolvedCallablePhysicalSignatureLoanV1` and package physical
header are projections and cannot substitute for the resolver index/header.
The same boundary also matters for the prefix call contract. The source map
can carry a target only when the selected input's resolver ledger owns an
exact `ResolvedDirectCallTargetV1`; the observer-only child batch does not
issue that target, while `VerifiedResolvedCallableModuleV1::function_input()`
does so only on a separate module product. The current package fixture uses a
`SourceCallKindV1::Method` prefix, for which this ledger has no direct target,
but `VerifiedCallablePreludeCapabilityV1::issue` currently requires a target
for every call kind. Therefore an index/header handoff alone would only move
the failure to `[freeze:contract][callable-loop/prelude]
MissingPreludeTarget`. The target must not be rebuilt from the source name,
method selector, physical signature, or catalog key. This is evidence that the
session bridge is structurally landed, not evidence that the selected
production edge is executable.

`CanonicalSsaFunctionSessionV2` remains a physical SSA/CFG helper inside the
single function session; it is not a second source authority. The legacy
`capture_static_box_method_pending_v1` path stays untouched. The generic Ready
branch in `src/mir/builder/raw_loop_child_port.rs` remains a later consumer and
must not be opened until this owner boundary is closed.

## Session-entry implementation receipt (2026-09-11)

The selected cataloged CallableSingleLoop edge now owns the complete function
session lifecycle. `lower_callable_single_loop_function_draft_v1` borrows the
already-open `CanonicalFunctionLoweringSessionV1` and returns only a ready
`ReadyFunctionDraftSealV1`; it does not open or restore another session. The
private DraftSeal bridge prepares and commits that product into
`PendingFunctionSessionCloseV1` without restoring the captured parent. The
existing `complete_before_restore` terminal then performs collector admission
and restores the parent once. Lowerer and DraftSeal failures discard the same
unpublished owner.

The focused pending-close test proves that the parent remains captured until
completion and is restored after the terminal runs. Route selection,
callable production canary, and the exact pending-close test are green. This
receipt does not claim the full selected-module fixture, zero/one/multiple
iteration matrix, module publication, or OBJ/EXE execution; those remain the
next bounded acceptance work.

## Six-line brief

```text
Decision: use one CanonicalSsaFunctionSessionV2 for the selected callable Loop
  and one source-unit resolver index for eligible selected roots.
Source authority + canonical issuer: the resolver session issues the exact
  index/header/FreeStatic target; the batch lends those products by owner.
Non-authority: Method-name lookup, catalog-key repair, physical signatures,
  variable_map, composer-local phi_bindings, and latest-value ordering.
Fail-fast boundary: before physical Builder effects, reject missing or foreign
  index/header/target, Method prefix outside the profile, stale generation,
  wrong edge/predecessor, dominance drift, and unsealed publication.
Smallest next slice: attach the existing resolver index/header to eligible
  selected roots, keep nested owners unindexed, and prove one FreeStatic
  production caller without manual ledger setup.
Non-claims: no declared-instance Method target, local-completion closure,
  backend/OBJ/EXE, legacy retirement, or performance result.
```

## Ownership contract

`CanonicalSsaFunctionSessionV2` is the sole physical owner for this row. The
DirectAccum first edge uses the existing
`CanonicalDirectAccumSsaLowererV1` session owner, which contains
`ResolvedSsaIdentityStateV2`, `BindingSsaBuilderV1`, `CanonicalCfgSessionV1`,
and one `PhiTxn`. The generic Ready follow-up will use a private
`CallableCanonicalFunctionScopeV1` that owns the unpublished session for one
function call and lends a short mutable capability to the recursive body. The
session is not a field on `RawInvocationChildPortV1`, is not put behind
`Rc<RefCell<_>>`, and is not visible to compatibility callers. The source port
must not return a physical `ValueId`; it exposes only the exact source-bound
relation already issued by the Recipe.

The handoff is one-shot. The selected consumer creates or receives the
unpublished session, consumes the Recipe once, and closes it through the
existing function finish path. Any failure discards the unpublished session
through its existing cleanup path; no partially published PHI or CFG may
escape. Nested callable functions open an independent session and never borrow
the parent's.

## Source-index handoff implementation receipt (2026-09-11)

The selected source-unit batch now reuses the existing
`FunctionSemanticResolverSessionV1` callable index as its sole issuer. Exact
top-level and static roots receive the shared index and an owner-matched
`VerifiedCallableHeaderV1` through `ResolvedFunctionLoweringInputV1`; nested
owners stay unindexed. The observer-only batch entry remains unchanged, and
the index is not copied into each row. The focused positive
`freestatic_target_batch_lends_one_owner_matched_index_and_header` proves that
both source owners receive only their own header from the same resolver-issued
index. This receipt does not claim Method-prefix admission, the real package
production fixture, PHI emission, module publication, or OBJ/EXE execution.

## Package direct-call target validation receipt (2026-09-11)

The cataloged package gate now validates each direct-call observation against
the same resolver-issued source-unit index already lent by the batch. It
requires an exact target and an owner-known callable header, while preserving
the existing source-site and owned-site checks. An observation with no exact
target or no matching index is rejected at the package gate; it is never
repaired from a name, selector, catalog key, or physical signature.

The focused positive
`cataloged_typed_static_direct_call_uses_source_index_target` proves a typed
cataloged FreeStatic call is admitted without manual ledger installation. The
existing package rejection/acceptance tests remain green, including unissued,
foreign, nested, and wrong-arity observations. This closes the package's
source-index admission check only; it does not claim the selected Loop
consumer, PHI emission, module publication, or OBJ/EXE execution.

## Implementation order

1. **Accepted design (2026-09-11):** reuse the existing
   `FunctionSemanticResolverSessionV1` source-unit callable index as the sole
   issuer. Eligible selected top-level/static roots receive the exact index
   and their owner-matched header through the batch handoff; nested owners
   remain unindexed. `VerifiedResolvedCallableModuleV1::function_input()` is
   evidence of the same owner pattern, not a product to mix into this batch.
   The CallableSingleLoop profile accepts a `FreeStatic` prefix only until an
   existing declared-instance target owner is available; `Method` is an
   explicit outside-shape, not a fallback. Preserve
   `VerifiedCallableFunctionLoweringInputV1::issue` and
   `MissingPreludeTarget`; do not repair by name, selector, catalog key, or
   physical signature, attach a main-only index to unrelated children, or add
   a second semantic receipt.
2. Keep the landed semantic demand and route selection unchanged. Do not
   reopen source Facts/Recipe issuance or add a plan adapter.
3. [implemented locally; focused test green 2026-09-11] Add the bounded
   resolver-index/header handoff and exact owner/header lookup without
   duplicating the index per row. Keep observer-only callers and nested owner
   policy unchanged. The implementation remains uncommitted until the
   natural code boundary is reviewed.
4. [implemented locally; FreeStatic route and package target gate green
   2026-09-11] Mark Method-prefix CallableSingleLoop shapes outside so they
   return to the ordinary method path; the Method target path remains a later
   declared-instance row. The cataloged package gate now consumes the
   resolver-issued index to validate an exact FreeStatic target, and the
   focused typed positive has no manual ledger setup. This still does not
   claim the selected Loop production consumer or its PHI execution.
5. Inventory the existing function-session opener, DraftSeal prepare/commit,
   pending restoration, collector admission, and discard terminals. The
   selected entry must have one named owner for all of them.
6. [landed at `6c41d0925b`] Replace the nested `capture -> lowerer opens another session` shape with a
   private session-scoped lowering API. The selected cataloged entry opens one
   `CanonicalFunctionLoweringSessionV1`; the lowerer borrows that owner for
   Builder effects and returns only a ready DraftSeal product. It must not open,
   retain, or restore a second function session.
7. [landed at `6c41d0925b`] Add the smallest existing-owner terminal that carries the ready DraftSeal
   through prepare/commit while the same pending parent context remains held
   until collector admission completes. The selected entry may use the
   existing `PendingFunctionSessionCloseV1`; only its private DraftSeal-to-
   pending bridge is missing. A new semantic receipt or alternate publication
   path is out of scope.
8. Connect header condition, body read/rebind, backedge, and false-edge After
   through the single session's canonical Binding SSA/PHI state. Names and
   composer-local maps remain non-authority.
9. Add one reusable valid fixture with no manual ledger registration. Cover
   zero, one, and multiple iterations only after the real session terminal is
   connected; add mutation-discriminating negatives for foreign owner, stale
   generation, wrong edge/predecessor, and unsealed publication.
10. Run focused positive/negative gates, update the module README/reference
   receipt, and only then move to module publication/OBJ/EXE. The generic Ready
   consumer, local-completion handoff, and legacy retirement remain later rows.

## Acceptance

The final positive graph must reach a named session terminal without manual
`install_entry_values` or `install_single_local_for_test` calls. The wrapper
must return with its short borrow ended and the session fully sealed or
explicitly discarded. Header reads
use generation `h_n`, body reads use `h_n`, a rebind defines `s_n`, the
backedge carries `s_n`, and the false edge exposes the canonical After value
(`h_0` for zero iterations, or the next-header `h_(n+1)` after a backedge).
The later `s_n` value is never selected for After merely by creation order.
The session is fully sealed or explicitly discarded before the handoff
returns.

Each negative starts from that valid graph and mutates exactly one relation.
The resulting named reject must occur before a new physical Builder effect;
an arbitrary nonzero return or a fixture that already fails liveness is not
evidence. A successful test of this row does not claim Loop OBJ/EXE or process
exit behavior.

The first DirectAccum edge is currently evidenced by the route-selection test
`direct_accum_selection_uses_the_canonical_route` and the existing DirectAccum
session/lowering tests. This evidence does not close the generic Ready bridge
or the row's full 0/1/multiple-iteration acceptance matrix.

The physicalizer audit confirms the edge meaning for the eventual consumer:
`After` reaches the initial `h_0` when the header false edge is taken without a
backedge, and reaches the next-header generation after a preceding backedge.
No explicit, named zero-iteration production fixture was found in the current
test inventory. The generic Ready implementation must therefore add that
fixture from the same valid graph as the one- and multiple-iteration cases;
physicalizer behavior alone is supporting evidence, not acceptance evidence.

## Exclusive delete-set and non-claims

After production cutover, this row may delete only the selected source Loop
consumer's physical PHI generation, its composer-local physical value maps,
and manual-ledger setup helpers used solely by that consumer. It must not
delete `CorePhiInfo`, common PlanLowerer code, dynamic/non-callable Loop
routes, or the legacy compatibility family.

Option B (a plan-level mechanical adapter) remains parked. Reopen it only if
the existing session cannot consume the Recipe and a new owner contract proves
that every PHI token carries BindingRef, exact block/predecessor witnesses,
dominance, and seal completion without creating a second issuer.

## Reopened audit finding and accepted resolution (2026-09-11)

The current implementation has two `CanonicalFunctionLoweringSessionV1`
owners for one selected child: the cataloged method's
`capture_resolved_function_pending_session_v1` and the callable lowerer's
`open_resolved_function_draft_seal_session_v1`. The inner owner performs the
DraftSeal transition, while the outer owner only keeps a pending wrapper. This
is not a second PHI issuer, but it makes session ownership and restoration
ambiguous and leaves no direct terminal that combines DraftSeal commit with
collector completion before the parent is restored.

The row was reopened in `design_stop` until the smallest existing
session/pending API was identified. The accepted design is:

```text
selected cataloged entry
  -> one CanonicalFunctionLoweringSessionV1 owner
  -> borrowed callable physicalizer
  -> ReadyFunctionDraftSealV1
  -> DraftSeal prepare
  -> pending DraftSeal commit (without parent restore)
  -> collector admission while parent remains captured
  -> one restoration/discard terminal
```

`CanonicalSsaFunctionSessionV2` may remain an internal physical helper, but it
must borrow the function-session Builder and cannot become another function
session owner. The selected cataloged entry opens the one
`CanonicalFunctionLoweringSessionV1`. The callable lowerer receives a mutable
borrow of that owner and returns `ReadyFunctionDraftSealV1`; it never opens or
restores a function session. A private DraftSeal terminal moves the prepared
projected draft into the existing `PendingFunctionSessionCloseV1` without
restoring the parent. `complete_before_restore` then performs collector
admission and restores the parent exactly once. Any lowerer or DraftSeal
failure discards the same owner. No semantic receipt, source adapter, or second
publication path is introduced.

This Decision closes the design stop. Implementation may begin at the selected
CallableSingleLoop entry; 0/1/multiple fixtures and OBJ/EXE remain acceptance
work, not pre-existing evidence.

## MIR-CALLABLE-LOOP-PHI-SOURCE-INDEX-HEADER-HANDOFF-D0

### Reopened design finding: selected child has no resolver index/header or prelude target (2026-09-11)

The first package-owned production-entry fixture intentionally avoided manual
ledger registration and entered through the selected cataloged adapter. It
reached the existing lowerer and returned
`[freeze:contract][callable-loop/missing-index]`. The source batch's
`with_lowering_input` path confirms that only the App Main slot receives the
co-issued callable index; other selected rows use
`from_exact_parts_without_callable` by design. The resolver comments also
state that nested owners do not inherit the index.

The design stop is resolved by reusing one existing source-unit resolver index
for each eligible selected root. The batch may lend that index and the header
matched to the selected owner; nested owners remain unindexed. This is a
policy/handoff change inside the existing resolver owner, not a second issuer.
The CallableSingleLoop profile is explicitly FreeStatic-only for this row, so
the current Method fixture remains outside until a declared-instance target
owner is opened separately. The next implementation task must not weaken
`VerifiedCallableFunctionLoweringInputV1::issue` or
`MissingPreludeTarget`, nor use `CallablePhysicalHeaderRefV1`/
`ResolvedCallablePhysicalSignatureLoanV1` as a look-alike.

The failed probe is retained as design evidence only; its uncommitted test
fixture was removed and no production-success claim is made. The PHI
value-flow SSOT remains valid and unchanged.

### Direct-call target scope (same design stop)

The selected CallableSingleLoop source map reads a prefix target from
`ResolvedFunctionLoweringInputV1::function().direct_call_targets()`. The
specialized package resolver intentionally gives only the App Main row a
callable index and leaves other selected rows observer-only; an unissued
observation is rejected by the package gate rather than repaired. The complete
`VerifiedResolvedCallableModuleV1` path resolves every top-level module
function with the same catalog index, but it is not retained by the selected
package and its forest/owner products are not interchangeable with the package
batch. The current package fixture's `Method` prefix also has no resolver
direct target, while the physicalizer requires one. Therefore the D0 must
decide whether an already-existing resolver/module owner can lend the selected
child an exact index/header/target set in one scoped handoff, or whether the
profile must explicitly reject Method prefixes and select a valid FreeStatic
caller. If neither is available, record `NoSafeSlice` and explicitly redesign
the resolver/prefix policy before any production switch. No source-name or
method-selector lookup, physical-signature substitute, or optional target
default is allowed.

## CallableSingleLoop AppMain consumer receipt (2026-09-11)

The selected AppMain static-child adapter now reuses the installed owner,
resolver index/header, and physical signature. It opens the existing
CallableSingleLoop consumer only when the existing source-bound
`try_prepare_callable_single_loop_program_v1` produces a program; other static
child shapes retain their previous source lowering until their own consumer is
selected. The physical direct-call emitter receives the admission-selected
physical symbol explicitly, while the resolver header remains the logical
callable identity. Cataloged children are admitted with the existing
`CatalogedBoxMethod` key, so no second semantic issuer or name repair is added.

The production fixture
`normal_ingress_routes_app_main_static_loop_child_through_callable_consumer`
compiles without manual ledger installation and observes a PHI. The existing
AppMain free-static publication test and CallableSingleLoop positive and late
failure canaries also pass. This receipt proves only the selected
CallableSingleLoop AppMain child edge; zero/one/multiple iteration acceptance,
generic Ready/DirectAccum consumers, module publication, OBJ/EXE, and process
exit remain open.

### Condition-bound extension receipt (2026-09-11)

The selected CallableSingleLoop profile now carries its source-issued
`ConditionBound` literal through the existing source-map to Recipe and relation
co-seal. The profile keeps the initial carrier at `0`, the step delta at `1`,
and the strict-less-than condition; only a non-negative plain integer bound is
accepted. Negative, typed, symbolic, and otherwise unsupported bounds remain
named source-map rejects. No new issuer, receipt, fallback, or physicalizer
path was added.

The focused source-map test accepts bounds `0` and `3` and checks that the
co-sealed row retains the exact literal. The production AppMain consumer test
compiles the same selected edge for bounds `0`, `1`, and `3`, with no manual
ledger installation, and observes the published PHI plus the bound constant in
the emitted helper. This provides compile-time coverage for zero, one, and
multiple iteration shapes within the selected profile.

The receipt does not claim that generic Loop Composer value-flow, body-local
ledger completion, mutation-discriminating negatives, module/OBJ/EXE, or Pair
exit behavior is complete; those remain queued acceptance work.

### Mutation-discriminating receiver rejection receipt (2026-09-11)

The valid declaration-backed FreeStatic fixture is now reused for a one-point
receiver-shape mutation. The only changed input is the expected receiver
(`FreeStatic` -> `Other`); the source owner, resolver index/header, callable
target, completion, Recipe, and cleanup-capable graph remain valid. The
physical prepare boundary rejects this mutation with the named
`PreludeReceiverMismatch` terminal before a physical effect. This closes the
receiver side of the negative-proof requirement for the selected entry.

Wrong edge/predecessor, stale generation, and unsealed publication mutations
still need dedicated selected-session helpers; the existing low-level verifier
tests are not promoted as production-fixture evidence. Module publication,
OBJ/EXE, and Pair exit remain unclaimed.

### Selected production value-flow receipt (2026-09-11)

The selected AppMain production test now inspects the emitted helper's actual
CFG rather than asserting only that a PHI exists. For bounds `0`, `1`, and `3`,
it locates the source-bound header branch, checks that the condition PHI has
both preheader and backedge inputs, verifies that the body PHI consumes the
header generation before the increment, and verifies that the exit return is
the header PHI's preheader/backedge value. This is compile-time evidence for
the `h_n -> s_n -> h_(n+1)` shape and the false-edge After value within the
selected profile, with no manual ledger setup.

The structural assertion was moved to
`normal_default_pipeline_loop_tests.rs` so the production-shaped test facade
stays below the 760-line design target. The receipt still does not claim
generic Composer value-flow, body-local ledger completion, stale/wrong-edge/
unsealed selected-session mutations, module/OBJ/EXE, or Pair exit.

### Profile coverage observation receipt (2026-09-11)

The selected callable lowerer no longer supplies a hard-coded profile tuple to
the existing profile-close owner. It derives operation, pure, read, and write
counts from the completed dispatch before that product is moved into After
completion. The canary uses the same helper, so the production and canary
paths share one observation owner. A focused test with a three-operation
dispatch confirms that the observed count is returned rather than the old
`(7, 4, 2, 1)` literal.

This closes the tautological production coverage check only. It does not claim
that the selected profile accepts a changed operation family, nor does it
close stale/wrong-edge/unsealed mutation helpers, module/OBJ/EXE publication,
Pair exit, generic Loop, or LocalSSA follow-up rows.

### Selected module artifact probe receipt (2026-09-11)

The selected module artifact probe was attempted after the resolver-index,
session-entry, condition-bound, receiver, and compile-time PHI value-flow
receipts were green. A source with a Pair root allocation, a CallableSingleLoop
child, and a direct child call reached the existing published lifecycle
boundary but stopped at the named
`[freeze:contract][ordinary-new/local-commit/emission-local-copy-drift]`.
An earlier source without a root allocation stopped at
`artifact-root-completion-unavailable`; that fixture was discarded because it
did not satisfy the root completion contract.

This is a prerequisite finding, not an OBJ/EXE result. The next bounded owner
is the existing `ordinary_new_local_commit` emission validation and finalized
root handoff. It must preserve the source-issued local Copy relation, let the
valid Pair-plus-Loop graph reach `issue_lifecycle_physical_abi_input()`, and
keep a one-point Copy source/value mutation as the named drift rejection before
any artifact effect. No name repair, latest-value substitution, or physical MIR
inference is allowed; Loop execution and Pair exit30 remain unclaimed.
