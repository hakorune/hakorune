---
Status: active design stop
Date: 2026-08-05
Parent: ../design/joinir-generic-post-effect-debt-classification-ssot.md
Decision: provisional — `JOINIR-GENERIC-NESTED-CARRIER-WINNER0-D2-B4-D0`
---

# Generic nested-carrier winner certificate — D2-B4

## Boundary

D2-B2/B3 established a real Generic `Both` overlap whose V0 and V1 plans
have different nested-carrier meaning. The legacy witness still terminates at
V0, while the test-only observation sees the recursive V1 carrier target. This
row narrows the unresolved question to one bounded pre-effect certificate; it
does not promote the observation into production policy.

## Source authority

The row consumes only products already owned by the current pipeline:

```text
shared LivePreflightFrameV1
  -> resolved GenericLoopV1 facts observation
  -> frozen raw schedule / mode snapshot
  -> natural fresh-candidate V1 stage result
```

The recursive-carrier observation is the only source-side input. Its target
identity follows the existing resolved binding/enclosing-binding contract. The
V1 stage result must come from the real facts → selection → composer → verify /
lower path on a fresh candidate; no malformed plan or failure injection is
allowed.

## Non-authority

This row must not read AST nodes, route names, `diagnostic_effective`, CorePlan
digests as policy, legacy receipts, or post-effect success to select a winner.
It must not add a selector arm, scheduler, retry/fallback, Recipe/JoinSig/PHI
producer, physicalizer, candidate publication, or production caller. The
legacy scheduler and receipts remain execution authority.

## Candidate disposition

Only this bounded candidate may be issued by a test-only neutral probe:

```text
CompleteRecursiveCarrier(targets)
  + raw Both overlap
  + natural V1 stage success on a fresh candidate
  -> V1 pre-effect winner certificate candidate
```

All other cases remain unresolved:

```text
CompleteNoRecursiveCarrier -> UnresolvedStop
Unavailable                -> UnresolvedStop
Ambiguous                  -> UnresolvedStop
missing/failed V1 stage   -> UnresolvedStop
planner-required V0 gate  -> separate pre-effect observation
```

The certificate is valid only if the observed recursive targets equal the V1
outer carrier/final-value targets, the frame and source row match, and a fresh
repeat is stable. A legacy V0 success with no debt receipt is recorded as a
semantic mismatch, not as certificate evidence.

## Acceptance evidence

The test-only matrix must cover the real `Both` nested-carrier source under
release and strict modes, plus planner-required as a separate gate. Each row
records the frame, raw schedule, recursive targets, V1 stage/first-effect
owner, outer PHI/final-value targets, legacy prefix/receipt/terminal, and the
comparison disposition. The matrix must show either exact target equality and
stable fresh repeat, or retain `UnresolvedStop` with the concrete mismatch.

No production selection changes are permitted while any claimed row lacks a
natural V1 stage, source-derived target equality, or candidate isolation.

## Closeout and next boundary

Implementation of this design row, if authorized later, must update the parent
Generic SSOT, this task card, `docs/reference/mir/generic-loop-stage-matrix.md`,
`src/mir/builder/control_flow/plan/generic_loop/README.md`,
`CURRENT_STATE.toml`, `10-Now.md`, and the active workstream. The closeout must
state exact commands, line budgets, and explicit non-claims. Reference updates
are part of implementation acceptance, not deferred cleanup.

Even a green certificate here does not authorize a production V1 selector,
Generic Recipe, Retry removal, PHI ownership, M7-S4, M10a, or M10b. Those need
the parent M4/D2 decision and later pipeline gates.
