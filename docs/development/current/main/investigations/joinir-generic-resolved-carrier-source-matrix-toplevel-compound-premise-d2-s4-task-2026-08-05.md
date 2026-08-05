---
Status: selected bounded premise audit — cfg(test)-only
Date: 2026-08-05
Parent: joinir-generic-resolved-carrier-selection-boundary-d3-design-2026-08-05.md
Decision: provisional — classify the observed source path before any NoRecursive row
Task: `JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-TOPLEVEL-COMPOUND-PREMISE0-D2-S4`
---

# Top-level CompoundAssignment premise audit

## Purpose

Close one source-backed premise only. The current carrier collector marks a
nested `CompoundAssignment` as `Unavailable("CompoundAssignment")`, while a
top-level compound can fall through the non-nested path. The same AST kind
therefore does not yet have one settled facts owner. A source-backed
`CompleteNoRecursive` row must not be selected until this asymmetry is
observed and named.

This task is not a selector change, a Generic winner decision, or a widening
of the collector. It is a bounded source observation that returns to the
parent D3 design stop.

## Source witness

Use a real parsed `.hako` function under one scoped
`NYASH_SYNTAX_SUGAR_LEVEL=basic` guard. The smallest accepted witness is:

```hako
function generic_top_level_compound(i, j) {
  loop(i < 3) {
    j += 1
    i = i + 1
  }
  return j
}
```

The test must assert the actual `ASTNode::CompoundAssignment` operator and
must not replace it with a synthetic AST helper. The resolver forest, target
binding, post-loop read, function/source/frame identity, and invocation owner
must be retained in one private test witness.

## Observation contract

For Release and Strict, with planner-required disabled, record the actual
facts result and raw route schedule from the same invocation. The result is
intentionally open and must be one of:

```text
CompleteNoRecursiveCarrier
Unavailable("CompoundAssignment")
Ambiguous(...)
typed NoStandaloneRow / pre-effect reject
```

The test must not predeclare which arm wins. A second fresh invocation must
preserve source/forest/frame identity shape, facts label, mode, and raw
schedule while receiving a distinct invocation owner.

The observation is sealed only if:

- parser and resolver produce the real source witness;
- the expected one-member forest/BindingRef relation is present;
- facts are observed before any Builder effect;
- Release and Strict schedules are measured, not inferred from route names;
- repeat drift, missing facts, missing source/frame identity, or an unexpected
  shape returns a typed unresolved result.

## Authority and non-authority

Authority belongs to the parser, `VerifiedResolvedSourceUnitV1`, the resolver
forest/BindingRef product, and the existing Generic facts extractor. The test
router may transport mode and raw schedule only.

The following are explicitly non-authority:

```text
synthetic matrix fixtures
route names or digests
registry selection and handlers
Legacy receipts or V0/V1 precedence
Recipe/JoinSig/PHI/Builder/MIR/backend output
runtime traces, fallback, and Retry
```

No neutral eligibility issuer, selector arm, source-to-selection capability,
production caller, or compatibility fallback may be added.

## Closeout and reference obligation

The implementation must stay in `cfg(test)` and use a sibling test module
under the existing registry test boundary. Keep every touched source/check
file below 800 lines. The same implementation commit must update:

```text
this task card with measured outcome
parent D3 design card
Generic post-effect debt SSOT
docs/reference/mir/generic-loop-stage-matrix.md
generic-loop and resolved-semantics READMEs
CURRENT_STATE.toml, 10-Now.md, and the active workstream mirror
affected docs/reference navigation/status indexes
artifact manifest and pointer guards
```

This reference closeout is mandatory after implementation; it must not be
deferred to a later language or production cutover task.

## Stop and follow-up

If the facts path is absent or the result is not a standalone observation,
close this task as typed `NoStandaloneRow` and return to the D3 design stop.
Do not widen `collect_recursive_carrier_targets`, body/recipe assignment
owners, or selector policy in this row.

Only after this premise is classified may a separate parsed
`Both/NoRecursive` source row be selected. Neither row authorizes V0-only
winner claims, Legacy packaging, precedence, production handoff, Recipe,
PHI, MIR, Retry deletion, or fallback removal.

## Implementation closeout — 2026-08-05

The cfg(test)-only sibling
`generic_resolved_carrier_toplevel_compound_premise_tests.rs` now records one
real parsed top-level compound witness. The resolver produces one loop member,
the compound target and post-loop read share the same function-owned
`BindingRefV1`, and the source/frame/owner identity remains stable across
Release and Strict and across fresh repeats.

The observed facts result is **typed `NoStandaloneRow`**: the current facts
product is absent for this top-level compound shape, and the measured raw
schedule is `[]` in both modes. The test does not reinterpret this as
`CompleteNoRecursiveCarrier`, `Unavailable`, or a V0-only result. No facts
label, eligibility, Legacy, winner, selector, Builder, MIR, Recipe, PHI,
Retry, fallback, or production handoff was added.

Focused evidence is green:

```bash
env -u HAKO_JOINIR_STRICT -u HAKO_JOINIR_PLANNER_REQUIRED \
  RUSTFLAGS='-Awarnings' cargo test --lib top_level_compound -- --nocapture
```

The implementation closeout also updates the parent D3 card, Generic SSOT,
Generic stage-matrix reference, both Generic READMEs, current mirrors, and
the artifact manifest in this same commit. The next parsed `Both/NoRecursive`
row is a separate design decision; this result does not authorize it.
