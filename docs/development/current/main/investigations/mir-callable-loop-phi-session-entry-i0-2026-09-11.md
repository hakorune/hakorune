---
Status: accepted design; implementation held by generic physical-demand D0 (DirectAccum first edge connected)
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

The first production edge is the selected static-callable branch in
`NormalCallableSemanticPackagePortAdapterV1::lower_cataloged_static_box_method`.
It routes the existing DirectAccum capability to
`CanonicalDirectAccumSsaLowererV1`, whose function-owned session already owns
Binding SSA, CFG, and the PHI transaction. This row does not add a second
selector or a second Loop consumer. The generic Ready branch in
`src/mir/builder/raw_loop_child_port.rs` remains the next bounded consumer,
but its builder-free physical-demand handoff is first specified by
`mir-callable-loop-phi-generic-physical-demand-d0-2026-09-11.md`.

The legacy `capture_static_box_method_pending_v1` path stays untouched. A
private function-scope wrapper is still required when the generic Ready
consumer is opened; after the physical-demand handoff is accepted, it must
lend a short-lived canonical body capability
without adding a session field to `RawInvocationChildPortV1`. The selected
static-callable entry is the only session opener and owner for that follow-up;
`RawInvocationChildPortV1::lower_loop` receives the scoped capability as a
consumer and must never construct, retain, or finish a session by itself.

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

1. (Connected for DirectAccum.) Reuse the existing DirectAccum capability
   probe and `CanonicalDirectAccumSsaLowererV1` session; pass the catalog
   physical symbol only as the already-validated function name.
2. Inventory the existing callable-function session opener, block/terminator
   owners, identity declaration/assignment APIs, completion owner, and
   unpublished-session discard path. Do not add a local map or adapter.
3. Split the selected static-callable canonical entry's body preparation from
   legacy capture, then add the private function-scope wrapper in
   `canonical_callable_session_scope.rs`. The outer entry owns one session
   and lends a short capability to `lower_loop`; do not add a session field or
   new lifetime parameter to the raw port. Keep source relation rows
   unchanged and consume the Recipe once.
4. Pass the scoped body port to the located invocation body driver and connect
   header condition, body read/rebind, backedge, and false-edge After to the
   session's canonical block-scoped reads and seals.
5. Remove only the selected source consumer's name-keyed physical PHI path
   after the new consumer is live; keep common generic PlanLowerer and
   non-callable Loop routes.
6. Add a reusable valid fixture with no manual ledger registration. Cover
   zero, one, and multiple iterations, including a body local initialization
   only after its real completion publisher is connected by the next row.
7. Add mutation-discriminating negatives for one foreign owner/BindingRef,
   stale generation, wrong edge or predecessor, and unsealed publication.
8. Run the focused gate and update the module README/reference receipt. Only
   after this row closes may `MIR-CALLABLE-LOOP-LOCAL-COMPLETION-HANDOFF-R0`
   become selectable.

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
