---
Status: design_stop__SourceIndexHeaderHandoffOpen
Date: 2026-09-11
Decision: MIR-CALLABLE-LOOP-PHI-SESSION-ENTRY-I0
Parent: mir-callable-loop-phi-canonical-session-bridge-d0-2026-09-11
---

# MIR-CALLABLE-LOOP-PHI-SESSION-ENTRY-I0

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
The same boundary also matters for a prefix free-static call: the source map
can carry a target only when the selected input's resolver ledger owns an
exact `ResolvedDirectCallTargetV1`; the observer-only child batch does not
issue that target, while `VerifiedResolvedCallableModuleV1::function_input()`
does so only on a separate module product. The target must not be rebuilt from
the source name, physical signature, or catalog key. This is evidence that
the session bridge is structurally landed, not evidence that the selected
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
Decision: use one CanonicalSsaFunctionSessionV2 for the selected callable Loop.
Source authority + canonical issuer: CallableSemanticLoweringState/verified
  Recipe owns BindingRef, role, and source site; session identity plus
  BindingSsaBuilderV1 owns physical ValueId/PHI issuance.
Non-authority: variable_map, composer-local phi_bindings, carrier_step_phis,
  CorePhiInfo tags, names, latest values, and ValueId ordering.
Fail-fast boundary: before a new CFG/PHI or other Builder effect, reject
  foreign owner/site, stale generation, wrong predecessor/edge, dominance
  drift, and unsealed publication with a named terminal.
Smallest next slice: open the generic Ready consumer through a private
  function-scope wrapper after the DirectAccum edge is stable; keep the Recipe
  logical and move-only.
Non-claims: no local-completion closure, backend/OBJ/EXE, legacy retirement,
  performance result, or plan-level adapter.
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

## Implementation order

1. **Design stop (open):** choose an existing source owner that can lend the
   exact resolver-issued index/header pair, plus the exact direct-call target
   relation when the selected prefix is a free-static call, to the selected
   non-AppMain child. The resolver-owned
   `VerifiedCallableIndexV1`/`VerifiedCallableHeaderV1` and
   `VerifiedResolvedCallableModuleV1::function_input()` are the only
   complete candidate authorities found so far, but the latter belongs to a
   separate module product and cannot be mixed with the package's observer
   forest. Preserve the current observer-only
   `from_exact_parts_without_callable` contract for roots/generic callers;
   do not repair by name, rebuild a header or target from a physical
   signature/catalog key, attach the main-only index to unrelated children, or
   add a second semantic receipt. Close this item with an owner, caller,
   pre-effect reject, and exact positive/negative acceptance before resuming
   code.
2. Keep the landed semantic demand and route selection unchanged. Do not
   reopen source Facts/Recipe issuance or add a plan adapter.
3. Inventory the existing function-session opener, DraftSeal prepare/commit,
   pending restoration, collector admission, and discard terminals. The
   selected entry must have one named owner for all of them.
4. [landed at `6c41d0925b`] Replace the nested `capture -> lowerer opens another session` shape with a
   private session-scoped lowering API. The selected cataloged entry opens one
   `CanonicalFunctionLoweringSessionV1`; the lowerer borrows that owner for
   Builder effects and returns only a ready DraftSeal product. It must not open,
   retain, or restore a second function session.
5. [landed at `6c41d0925b`] Add the smallest existing-owner terminal that carries the ready DraftSeal
   through prepare/commit while the same pending parent context remains held
   until collector admission completes. The selected entry may use the
   existing `PendingFunctionSessionCloseV1`; only its private DraftSeal-to-
   pending bridge is missing. A new semantic receipt or alternate publication
   path is out of scope.
6. Connect header condition, body read/rebind, backedge, and false-edge After
   through the single session's canonical Binding SSA/PHI state. Names and
   composer-local maps remain non-authority.
7. Add one reusable valid fixture with no manual ledger registration. Cover
   zero, one, and multiple iterations only after the real session terminal is
   connected; add mutation-discriminating negatives for foreign owner, stale
   generation, wrong edge/predecessor, and unsealed publication.
8. Run focused positive/negative gates, update the module README/reference
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

### Reopened design finding: selected child has no resolver index/header (2026-09-11)

The first package-owned production-entry fixture intentionally avoided manual
ledger registration and entered through the selected cataloged adapter. It
reached the existing lowerer and returned
`[freeze:contract][callable-loop/missing-index]`. The source batch's
`with_lowering_input` path confirms that only the App Main slot receives the
co-issued callable index; other selected rows use
`from_exact_parts_without_callable` by design. The resolver comments also
state that nested owners do not inherit the index.

This reopens the row in `design_stop`. The next task is not to weaken
`VerifiedCallableFunctionLoweringInputV1::issue`, nor to use
`CallablePhysicalHeaderRefV1`/`ResolvedCallablePhysicalSignatureLoanV1` as a
look-alike. It is to connect one already-issued resolver index/header owner to
the selected child boundary without changing observer-only callers. If no
existing owner can supply that pair for the child, the row remains
`NoSafeSlice` until the resolver policy is explicitly redesigned; no guessed
`Verified*`/`Prepared*` product may be added.

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
batch. Therefore the D0 must decide whether an already-existing resolver/module
owner can lend the selected child an exact index/header/target set in one
scoped handoff. If not, record `NoSafeSlice` and explicitly redesign the
resolver policy before any production switch. No source-name lookup, physical
signature substitute, or optional target default is allowed.
