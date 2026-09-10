---
Status: accepted design; implementation not opened
Date: 2026-09-11
Decision: MIR-CALLABLE-LOOP-PHI-SESSION-ENTRY-I0
Parent: mir-callable-loop-phi-canonical-session-bridge-d0-2026-09-11
---

# MIR-CALLABLE-LOOP-PHI-SESSION-ENTRY-I0

## Boundary

```text
starts: callable function entry after the source-bound Loop Recipe is issued
ends: one unpublished function session capability consumed by the selected
      RawInvocationChildPortV1::lower_loop Ready branch through a scoped
      function wrapper
includes: session ownership handoff, logical Recipe consumption, canonical
          BindingRef -> block/value reads, CFG edges, PHI seal, and discard;
          short reborrow of the raw child port
excludes: new source Facts/Recipe issuer, plan-level PHI adapter, local
          completion handoff, generic fallback/retry, backend/OBJ/EXE, and R7
```

The production caller is the existing Ready branch in
`src/mir/builder/raw_loop_child_port.rs`. This row does not add a second
selector or a second Loop consumer. It moves the selected source-backed
consumer to the existing function-owned session path; it does not redesign
`MirBuilder`.

The entry-side owner is
`src/mir/builder/resolved_lowering/canonical_callable_session_scope.rs`,
constructed only from the selected static-callable branch of
`NormalCallableSemanticPackagePortAdapterV1::lower_cataloged_static_box_method`.
The legacy `capture_static_box_method_pending_v1` path stays untouched. The
scope's body driver must pass a short-lived canonical body port into the
located invocation driver; calling the raw port's ordinary `lower_body` from
inside the scope would drop the session capability at nested descent.

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
Smallest next slice: establish the session capability at callable entry and
  lend it once through a private function-scope wrapper to the Ready
  consumer; keep the Recipe logical and move-only.
Non-claims: no local-completion closure, backend/OBJ/EXE, legacy retirement,
  performance result, or plan-level adapter.
```

## Ownership contract

`CanonicalSsaFunctionSessionV2` is the sole physical owner for this row. It
owns the existing `ResolvedSsaIdentityStateV2`, `BindingSsaBuilderV1`,
`CanonicalCfgSessionV1`, and one `PhiTxn`. A private
`CallableCanonicalFunctionScopeV1` owns that unpublished session for the
function call and lends a short mutable capability to the recursive body. The
session is not a field on `RawInvocationChildPortV1`, is not put behind
`Rc<RefCell<_>>`, and is not visible to compatibility callers. The source port
must not return a physical `ValueId`; it exposes only the exact source-bound
relation already issued by the Recipe.

The handoff is one-shot. The entry creates or receives the unpublished
session, the wrapper lends it for one scoped Ready-consumer call, and the
Recipe is consumed once. Success closes the session through the existing
function finish path. Any failure discards the unpublished session through its
existing cleanup path; no partially published PHI or CFG may escape. Nested
callable functions open an independent session and never borrow the parent's.

## Implementation order

1. Inventory the existing callable-function session opener, block/terminator
   owners, identity declaration/assignment APIs, completion owner, and
   unpublished-session discard path. Do not add a local map or adapter.
2. Split the selected static-callable canonical entry's body preparation from
   legacy capture, then add the private function-scope wrapper in
   `canonical_callable_session_scope.rs`. It owns one session and lends a
   short capability; do not add a session field or new lifetime parameter to
   the raw port. Keep source relation rows unchanged and consume the Recipe
   once.
3. Pass the scoped body port to the located invocation body driver and connect
   header condition, body read/rebind, backedge, and false-edge After to the
   session's canonical block-scoped reads and seals.
4. Remove only the selected source consumer's name-keyed physical PHI path
   after the new consumer is live; keep common generic PlanLowerer and
   non-callable Loop routes.
5. Add a reusable valid fixture with no manual ledger registration. Cover
   zero, one, and multiple iterations, including a body local initialization
   only after its real completion publisher is connected by the next row.
6. Add mutation-discriminating negatives for one foreign owner/BindingRef,
   stale generation, wrong edge or predecessor, and unsealed publication.
7. Run the focused gate and update the module README/reference receipt. Only
   after this row closes may `MIR-CALLABLE-LOOP-LOCAL-COMPLETION-HANDOFF-R0`
   become selectable.

## Acceptance

The positive graph must reach a named session terminal without manual
`install_entry_values` or `install_single_local_for_test` calls. The wrapper
must return with its short borrow ended and the session fully sealed or
explicitly discarded. Header reads
use generation `h_n`, body reads use `h_n`, a rebind defines `s_n`, the
backedge carries `s_n`, and the false edge exposes the canonical After value.
The session is fully sealed or explicitly discarded before the handoff
returns.

Each negative starts from that valid graph and mutates exactly one relation.
The resulting named reject must occur before a new physical Builder effect;
an arbitrary nonzero return or a fixture that already fails liveness is not
evidence. A successful test of this row does not claim Loop OBJ/EXE or process
exit behavior.

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
