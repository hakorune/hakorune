---
Status: accepted bounded test-only execution brief
Date: 2026-08-05
Parent: ../design/joinir-generic-post-effect-debt-classification-ssot.md
Decision: accepted — `JOINIR-GENERIC-NESTED-CARRIER-WINNER0-D2-B4-D0`
Next implementation row: `JOINIR-GENERIC-NESTED-CARRIER-WINNER0-D2-B4-S1`
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

The recursive-carrier observation is the only source-side input. The current
facts/plan witness exposes deterministic source-binding labels (`Vec<String>`),
not shadowing-safe BindingRef identities; therefore this row claims label
equality only. The V1 stage result must come from the real facts → selection →
composer → verify / lower path on a fresh candidate; no malformed plan or
failure injection is allowed.

The existing test-only `evaluate_nested_carrier_policy_probe` requires
`contract_present`. The real `Both` frame does not carry a Generic Recipe
contract, and this row must not create one. D2-B4 therefore supersedes that
condition only inside the new `cfg(test)` certificate evaluator: record the
false contract bit as evidence, never use it as production authority, and keep
the parent disposition `UnresolvedStop` against the legacy V0 terminal.

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

The certificate is valid only if the observed recursive labels equal the V1
outer final-value labels, the required `loop_carrier_<label>` and
`loop_step_in_<label>` tags are present, the frame and `both` source row match,
and a fresh repeat is stable. This is not a shadowing-safe binding identity
claim. A legacy V0 success with no debt receipt is recorded as a semantic
mismatch, not as certificate evidence.

## Acceptance evidence

The test-only matrix must cover the real `Both` nested-carrier source under
release and strict modes, plus planner-required as a separate gate. Each row
records the frame, raw schedule, recursive targets, V1 stage/first-effect
owner, outer PHI/final-value targets, legacy prefix/receipt/terminal, and the
comparison disposition. The matrix must show either exact target equality and
stable fresh repeat, or retain `UnresolvedStop` with the concrete mismatch.

No production selection changes are permitted while any claimed row lacks a
natural V1 stage, source-derived label equality, or candidate isolation. The
implementation slice is one new `#[cfg(test)]` sibling module,
`registry/generic_nested_carrier_winner_tests.rs`, with only `pub(super)` test
projections exposed from the two existing observer modules; those modules are
already near the source-file line budget and must not receive the matrix body.
The product/issuer/consumer remain test-only: a neutral candidate certificate
is issued by the new matrix, consumed only by its assertions, and never enters
selection, scheduling, Recipe, JoinSig, PHI, physicalization, Retry, or a
production caller.

## S1 execution contract

**Change**: add only the test-only certificate matrix and the smallest
`pub(super)` projections needed from the existing observer tests. Reuse the
real `Both` fixture, release/strict raw schedule, natural fresh V1 stage, and
the planner-required gate as a separate non-overlap row.

**Product / issuer / consumer**:

```text
V1PreEffectWinnerCertificateCandidate (test-only DTO)
  issuer: generic_nested_carrier_winner_tests.rs
  consumer: the same matrix assertions
  production callers: 0
```

**Done**: `Both` release and strict rows record exact recursive-label equality,
required V1 carrier/step-in tags, `LowerSome`, `GenericComposer` first effect,
frame/source-row identity, legacy prefix/receipt/terminal, and fresh-repeat
determinism. Planner-required records raw `[V1]` separately and emits no overlap
certificate. Negative rows remain `UnresolvedStop`.

**Stop**: do not add a Generic Recipe contract, resolved BindingRef identity,
selector arm, scheduler/retry, Recipe/JoinSig/PHI/physicalizer caller, or
production policy. If the existing `contract_present=false` bit is used to
qualify a row, if labels/tags do not match, or if the stage is not natural and
fresh, stop with `UnresolvedStop`.

## Closeout and next boundary

Implementation of this design row must update the parent
Generic SSOT, this task card, `docs/reference/mir/generic-loop-stage-matrix.md`,
`src/mir/builder/control_flow/plan/generic_loop/README.md`,
`CURRENT_STATE.toml`, `10-Now.md`, and the active workstream. The closeout must
state exact commands, line budgets, and explicit non-claims. Reference updates
are part of implementation acceptance, not deferred cleanup; the implementation
is incomplete until the reference page and Generic README describe the landed
test-only certificate and its non-claims.

Even a green certificate here does not authorize a production V1 selector,
Generic Recipe, Retry removal, PHI ownership, M7-S4, M10a, or M10b. Those need
the parent M4/D2 decision and later pipeline gates.
