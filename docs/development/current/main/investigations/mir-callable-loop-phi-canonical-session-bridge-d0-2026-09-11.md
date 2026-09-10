---
Status: accepted design; implementation not opened
Date: 2026-09-11
Decision: MIR-CALLABLE-LOOP-PHI-CANONICAL-SESSION-BRIDGE-D0
Parent: mir-callable-loop-phi-value-binding-r0-2026-09-11
---

# MIR-CALLABLE-LOOP-PHI-CANONICAL-SESSION-BRIDGE-D0

## Six-line brief

```text
Decision: choose the only bridge that makes the source-backed callable Loop use the existing function-owned Binding SSA/PHI authority exactly once.
Source authority + canonical issuer: CallableSemanticLoweringState/verified Loop schedule own BindingRef/role/site; CanonicalSsaFunctionSessionV2 owns CFG, BindingSsaBuilderV1, and PhiTxn.
Non-authority: Recipe composer name maps, CorePhiInfo tags, variable_map, source latest values, raw ValueId order, and any fallback/retry.
Fail-fast boundary: the bridge must co-seal owner, source role, block placement, predecessor witness, generation, dominance, and terminal result before physical effects are published.
Smallest next slice: inspect the existing session entry and the selected Ready caller, then define one move-only logical-to-session handoff; no source or backend change yet.
Non-claims: no second SSA/PHI adapter, no new semantic receipt, no local-completion handoff, no production cutover, no OBJ/EXE, and no R7 retirement.
```

## Why the previous R0 is stopped

The existing Ready consumer composes a `CoreLoopPlan` before the
`PlanLowerer` emits a real CFG. Its source port reads the request-local
`CallableSemanticLoweringState`, while `generic_loop_composer.rs` allocates
carrier values and records `CorePhiInfo` by name. The established canonical
Binding SSA owner is used by `CanonicalSsaFunctionSessionV2` while blocks,
terminators, and seals are being emitted. It cannot be inserted into that
composer by merely passing a ledger or replacing one map: doing so would leave
two places deciding the reaching value or would require a new plan-level
adapter whose seal and dominance contract is not yet defined.

The PHI value-flow contract itself is already SSOTed in
`docs/reference/mir/loop-recipe-contract.md:1227`. This card only decides the
physical bridge that consumes that contract.

## Bounded alternatives

### A — session-level consumer

Move the source-backed Ready consumer from the current CorePlan composer into
the existing `CanonicalSsaFunctionSessionV2` path. The Recipe remains the
logical input; the session creates the canonical blocks, reads the unsealed
Header through its `BindingSsaBuilderV1`, defines the body rebind, seals the
actual predecessor set, and lets the one `PhiTxn` patch the Header PHI. The
source port becomes an exact claim/role capability and never supplies a
physical ValueId.

Required census before implementation:

```text
selected Ready caller
-> existing canonical session opener
-> block/terminator owners
-> identity declaration/assignment APIs
-> completion and unpublished-session discard
```

### B — plan-level mechanical adapter

Keep the existing CorePlan composer and wrap the generic `BindingSsaBuilderV1`
in a private adapter that emits only `CorePhiInfo` and ValueIds. This is
permitted only if the adapter is proven mechanical: it may not issue
`BindingRefV1`, infer source roles, or choose a value from a name. Its token
must carry the exact block and predecessor witness, and the final plan must be
rejected unless every provisional PHI is sealed and every input dominates its
predecessor before `PlanLowerer` consumes it.

This option needs a new accepted owner contract for deferred PHI publication;
the current `BindingSsaIrV1`/`MirBindingSsaAdapterV1` is tied to a real MIR
`PhiTxn`, so it cannot be assumed to satisfy this option without a design
decision.

## Selection criteria and task order

Choose A unless the existing canonical session cannot accept the current
Recipe without reopening source discovery. Choose B only after an owner audit
proves that its adapter is mechanical and that no second PHI lifecycle exists.

```text
1. inventory the existing canonical session entry and the Ready caller
2. verify the Recipe can be consumed as logical input without reclassification
3. select A or B with owner, terminal, caller, exclusive delete-set, and
   named rejects
4. implement the selected bridge for one valid 0/1/multiple-iteration graph
5. add mutation-discriminating generation/edge/publication negatives
6. only then reopen local completion and guard-cleanup rows

## Worker audit and accepted decision (2026-09-11)

The read-only owner audit confirms **A — session-level consumption**. The
existing `CanonicalSsaFunctionSessionV2` is the only safe physical bridge:
it already owns `ResolvedSsaIdentityStateV2`, `BindingSsaBuilderV1`,
`CanonicalCfgSessionV1`, and the single `PhiTxn`. The direct-accum and dynamic
loop consumers provide the existing precedent for this ownership shape.

The current composer cannot be made safe by passing its `variable_map`,
`phi_bindings`, or latest values into a wrapper. `CorePhiInfo` has no
`BindingRef`, predecessor witness, or seal/dominance proof, so a plan-level
adapter would become a second physical PHI lifecycle. Option B is therefore
parked until a separate owner contract proves it mechanical; it is not part of
the next implementation.

The selected bounded implementation slice is a capability handoff at the
callable function entry:

```text
callable function entry
  -> one CanonicalSsaFunctionSessionV2
  -> RawInvocationChildPortV1::lower_loop Ready consumer
  -> existing BindingSsaBuilderV1 / CanonicalCfgSessionV1 / PhiTxn
```

The Recipe remains a move-only logical product. It supplies the already-issued
`BindingRefV1`/role/site rows; the session supplies block-scoped `ValueId`s,
PHI creation, predecessor edges, and seals. The composer-local name maps,
`CorePhiInfo`, and `latest` values are not allowed to issue or select physical
SSA values.

The implementation card is
[`mir-callable-loop-phi-session-entry-i0-2026-09-11.md`](./mir-callable-loop-phi-session-entry-i0-2026-09-11.md).
It is queued at this taskization boundary and is not opened by this design
commit. Its first acceptance is a valid source-backed graph without manual
ledger registration, followed by 0/1/multiple-iteration coverage and
single-relation mutation negatives before any published physical effect.
```

## Acceptance and non-claims

The selected bridge must prove condition reads use `h_n`, body reads use
`h_n`, a rebind produces `s_n`, the backedge carries `s_n`, and After/tail use
the canonical false-edge value. A valid source graph must run without manual
ledger injection. Each negative starts from that graph and mutates exactly one
owner, BindingRef, generation, predecessor, edge, dominance, or seal relation;
the named rejection must occur before publication of a new physical effect.

This design does not authorize a fallback to `variable_map`, a source-name
lookup, a second BindingSsaBuilder/PhiTxn, a new semantic `Verified*` or
`Prepared*` receipt, local handoff work, backend/OBJ/EXE work, or aggregate R7
LegacyCallV0 retirement.
