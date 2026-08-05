---
Status: active design stop — source-shape and disposition boundary
Date: 2026-08-05
Parent: joinir-generic-resolved-carrier-selection-boundary-d3-design-2026-08-05.md
Predecessor: joinir-generic-resolved-carrier-source-matrix-toplevel-compound-premise-d2-s4-task-2026-08-05.md
Decision: provisional — no cfg(test) NoRecursive row until this boundary is sealed
Task: `JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-NORECURSIVE-DISPOSITION0-D2-S5-D0`
---

# NoRecursive source shape and disposition boundary

## Why this design stop exists

The facts enum has one observation label,
`CompleteNoRecursiveCarrier`, but source shapes that reach it do not share
one downstream meaning. A dedicated simple-while route can coexist with a
Generic V0 attempt; a local/effect body can expose a V1-only shape; and an
unsupported body can emit no facts and no raw schedule. These must not be
collapsed into one source-backed “NoRecursive” winner or target capability.

D2-S4 separately closed top-level `CompoundAssignment` as typed
`NoStandaloneRow`. This task must not mix that facts-absent compound boundary
with the plain Assignment shape below.

## Candidate source shape

The smallest candidate for this design is one parsed flat loop:

```hako
function generic_both_no_recursive(j, m, n) {
  loop(j + m < n) {
    j = j + 1
  }
  return j
}
```

The candidate is intentionally plain Assignment, not CompoundAssignment,
ScopeBox, nested IfThen, or a dedicated simple-while fixture. The eventual
source row must use the real parser and resolver, one-member loop forest,
resolver-issued write/read `BindingRefV1`s, actual Generic facts, and the
same-invocation Release/Strict preflight schedule.

## Design decisions to seal

### Source-shape owner

The selected row, if admitted, owns exactly one one-loop flat Assignment
shape. The existing two-member strict-ancestor projector used by D3 eligibility
is not reused as if it covered this shape. A one-member forest cannot issue the
current recursive-carrier eligibility capability.

The following shapes stay outside this task:

```text
simple-while dedicated-route inputs
local/effect bodies that produce V1-only schedules
nested IfThen or nested Loop carriers
CompoundAssignment / ScopeBox / unsupported containers
```

### Disposition

`CompleteNoRecursiveCarrier` is an observed facts label only. For the
one-member flat shape it maps to typed pre-effect
`UnresolvedStop(NonRecursiveOutOfTarget)`, not `Eligible`, `Legacy`,
`ProvenOutsideTarget`, or a V0 winner. If facts are absent, the raw schedule is
empty, or the shape is not the exact flat candidate, the row closes as typed
`NoStandaloneRow` instead of widening the extractor.

The implementation must measure the schedule; the expected natural-Both
shape is `[GenericLoopV0, GenericLoopV1]`, but that value is not assumed. A
different schedule is recorded as evidence and returns to this design stop.

## Authority

```text
NyashParser
  -> VerifiedResolvedSourceUnitV1 / FunctionSourceViewV1
  -> resolver loop forest and BindingRefV1
  -> existing Generic facts extractor
  -> same-invocation LivePreflightFrameV1 raw schedule
```

Only these products are semantic evidence. Synthetic AST helpers,
`FixtureClassV1` labels, route names, digests, legacy receipts, runtime
traces, and post-effect stage observations are non-authority.

## Fail-fast boundary

The future cfg(test) row must stop before Builder effects when any of these
occur:

```text
parse/resolve failure
forest is not exactly one root with no children
write/read BindingRef or owner/frame identity mismatch
facts absent or carrier != CompleteNoRecursiveCarrier
mode/schedule mismatch between Release and Strict
fresh-repeat drift
simple-route or V1-only shape is observed instead
```

Facts absence or empty raw schedule is typed `NoStandaloneRow`; it is not a
retry, fallback, Legacy receipt, or selector decline.

## Non-claims

This design and its later test row do not authorize:

```text
VerifiedResolvedCarrierEligibilityV1
Legacy packaging or ProvenOutsideTarget
V0/V1 precedence, suppression, or winner equivalence
neutral issuer, RecipeFirstSelectionInputV1, selector/router changes
Recipe/JoinSig/PHI/physicalizer/Builder/MIR/backend callers
Retry deletion, fallback removal, runtime changes, or production handoff
```

## Implementation gate after design seal

Only after this card is accepted may one sibling `cfg(test)` source witness be
added. It must co-seal source/forest/BindingRef/frame/facts/mode/raw schedule
in a private non-`Clone` receipt, use one shape and one commit, keep all source
and check files below 800 lines, and update the parent card, Generic SSOT,
`docs/reference/mir/generic-loop-stage-matrix.md`, both Generic READMEs,
current mirrors, and the artifact manifest in the same closeout commit.

The next implementation task must return to this design stop on any premise
drift. No production code or selector policy is permitted from this card.
