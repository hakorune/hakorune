---
Status: design_stop__CallableSingleLoopSessionOwnerMismatch
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
The semantic demand and canonical PHI/CFG physicalizer are wired, but this
edge is not yet accepted as a session-entry implementation. The cataloged
method currently calls `capture_resolved_function_pending_session_v1`, while
`callable_lowerer.rs` opens another `CanonicalFunctionLoweringSessionV1`
before producing `ReadyFunctionDraftSealV1`. That leaves the outer session as
a pending wrapper around an inner function session and violates this row's
one-session owner contract, even though existing tests can pass.

`CanonicalSsaFunctionSessionV2` remains a physical SSA/CFG helper inside the
single function session; it is not a second source authority. The legacy
`capture_static_box_method_pending_v1` path stays untouched. The generic Ready
branch in `src/mir/builder/raw_loop_child_port.rs` remains a later consumer and
must not be opened until this owner boundary is closed.

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

1. Keep the landed semantic demand and route selection unchanged. Do not
   reopen source Facts/Recipe issuance or add a plan adapter.
2. Inventory the existing function-session opener, DraftSeal prepare/commit,
   pending restoration, collector admission, and discard terminals. The
   selected entry must have one named owner for all of them.
3. Replace the nested `capture -> lowerer opens another session` shape with a
   private session-scoped lowering API. The selected cataloged entry opens one
   `CanonicalFunctionLoweringSessionV1`; the lowerer borrows that owner for
   Builder effects and returns only a ready DraftSeal product. It must not open,
   retain, or restore a second function session.
4. Add the smallest existing-owner terminal that carries the ready DraftSeal
   through prepare/commit while the same pending parent context remains held
   until collector admission completes. A new semantic receipt or alternate
   publication path is out of scope; if the existing owner cannot express this,
   stop and record the exact API gap before editing callers.
5. Connect header condition, body read/rebind, backedge, and false-edge After
   through the single session's canonical Binding SSA/PHI state. Names and
   composer-local maps remain non-authority.
6. Add one reusable valid fixture with no manual ledger registration. Cover
   zero, one, and multiple iterations only after the real session terminal is
   connected; add mutation-discriminating negatives for foreign owner, stale
   generation, wrong edge/predecessor, and unsealed publication.
7. Run focused positive/negative gates, update the module README/reference
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

## Reopened audit finding (2026-09-11)

The current implementation has two `CanonicalFunctionLoweringSessionV1`
owners for one selected child: the cataloged method's
`capture_resolved_function_pending_session_v1` and the callable lowerer's
`open_resolved_function_draft_seal_session_v1`. The inner owner performs the
DraftSeal transition, while the outer owner only keeps a pending wrapper. This
is not a second PHI issuer, but it makes session ownership and restoration
ambiguous and leaves no direct terminal that combines DraftSeal commit with
collector completion before the parent is restored.

The row is therefore reopened in `design_stop` until the smallest existing
session/pending API is identified. The required design is:

```text
selected cataloged entry
  -> one CanonicalFunctionLoweringSessionV1 owner
  -> borrowed callable physicalizer
  -> ReadyFunctionDraftSealV1
  -> DraftSeal prepare/commit
  -> collector admission while parent remains captured
  -> one restoration/discard terminal
```

`CanonicalSsaFunctionSessionV2` may remain an internal physical helper, but it
must borrow the function-session Builder and cannot become another function
session owner. Until this terminal is available, no 0/1/multiple fixture or
OBJ/EXE claim is valid.
