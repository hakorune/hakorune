---
Status: selected cfg(test)-only implementation child
Date: 2026-08-05
Parent: joinir-generic-resolved-carrier-source-matrix-norecursive-disposition-d2-s5-d0-design-2026-08-05.md
Decision: accepted — one source witness, no production consumer
Task: `JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-BOTH-NORECURSIVE0-D2-S5-S1`
---

# Parsed flat NoRecursive source witness

## Scope

Implement exactly one sibling `cfg(test)` source row. Do not add a neutral
issuer, selector arm, eligibility capability, Legacy receipt, Builder/MIR
caller, Recipe/JoinSig/PHI owner, Retry deletion, fallback removal, or
production handoff.

## Exact source contract

```hako
function generic_both_no_recursive(j, m, n) {
  loop(j + m < n) {
    j = j + 1
  }
  return j
}
```

The parsed function body is exactly `[Loop, Return]`; the loop body is exactly
`[Assignment]`. The condition, target, RHS, operator, and return value must be
the exact source roles documented in the D2-S5-D0 design card. Nested/extra
statements, `If`, `Local`, `ScopeBox`, compound assignment, different target,
operator, condition, or return shape is a typed reject.

## Evidence receipt

The private non-`Clone` test receipt must co-seal:

```text
parsed source and FunctionSourceViewV1 sites
one-member resolved loop forest
same-owner write/read BindingRefV1s
source kind and frame key
actual Generic facts observation
Release/Strict frame environment
actual raw route schedule
```

Release and Strict must each be run with planner-required disabled. A fresh
repeat must use a distinct `FunctionOwnerId` while preserving origin,
source/frame/loop-site identity, BindingRef slot shape, facts identity, and
raw schedule. Mode flags come from the returned frame, never route labels.

## Acceptance and disposition

The only accepted positive observation is exact
`CompleteNoRecursiveCarrier` with exact raw schedule `[V0,V1]` in both
Release and Strict. Its typed pre-effect result is
`UnresolvedStop(NonRecursiveOutOfTarget)`. This is not
`ProvenOutsideTarget`, `Legacy`, a winner, or a selector decline.

If facts are absent or raw schedule is `[]`, record typed `NoStandaloneRow`.
If the carrier, schedule, mode, source shape, forest, identity, or repeat
drifts, record a typed premise reject and return to D2-S5-D0. Never widen the
collector or reinterpret a dedicated `LoopSimpleWhile`/V1-only result as Both.

## Closeout

The implementation commit must update the D2-S5-D0 card, parent D3 design,
Generic SSOT, `docs/reference/mir/generic-loop-stage-matrix.md`, both Generic
READMEs, `CURRENT_STATE.toml`, `10-Now.md`, the active workstream, and the
artifact manifest. Focused and adjacent generic-resolved-carrier tests plus
pointer/artifact/line guards are required. All touched source/check files stay
below 800 lines.

## Implementation closeout — 2026-08-05

The sibling cfg(test) module
`generic_resolved_carrier_both_norecursive_tests.rs` implements the exact
source contract. It verifies the parsed function body `[Loop, Return]`, loop
body `[Assignment]`, `j + m < n`, `j = j + 1`, and post-loop `j` sites before
resolving the one-member forest.

Release and Strict, with planner-required disabled, both observe exact
`CompleteNoRecursiveCarrier` and measured raw schedule
`[GenericLoopV0, GenericLoopV1]`. The same-owner write/read BindingRefs,
source kind, frame key, function origin, and facts identity are co-sealed. A
fresh repeat gets a distinct `FunctionOwnerId` while preserving the semantic
shape and schedule. The typed disposition is
`UnresolvedStop(NonRecursiveOutOfTarget)`; it is not ProvenOutsideTarget,
Legacy, a winner, or an eligibility capability.

Focused evidence is green:

```bash
env -u HAKO_JOINIR_STRICT -u HAKO_JOINIR_PLANNER_REQUIRED \
  RUSTFLAGS='-Awarnings' cargo test --lib flat_norecursive -- --nocapture
```

The implementation closeout updates the D2-S5-D0 design card, parent D3
design, Generic SSOT, stage-matrix reference, both Generic READMEs, current
mirrors, and artifact manifest in this same commit. No production caller or
selector/eligibility authority moved.
