# Generic Loop V0/V1 Stage Matrix

Status: inspection-only reference
Date: 2026-08-04

This page documents the current test-only evidence boundary for Generic Loop
V0/V1 post-effect debt. It is not a production route policy, Recipe contract,
PHI owner, scheduler, or backend lowering specification.

## Authorities

The design authority is
`docs/development/current/main/design/joinir-generic-post-effect-debt-classification-ssot.md`.
The executable task and acceptance evidence are
`docs/development/current/main/investigations/joinir-generic-post-effect-debt-classification-d0-s1-execution-task-2026-08-04.md`.
The machine-readable test observer is
`src/mir/builder/control_flow/joinir/route_entry/registry/generic_stage_matrix_tests.rs`.

Production selection remains the ordered registry in
`src/mir/builder/control_flow/joinir/route_entry/registry/selection.rs` and
`predicates.rs`. The `loop_route_policy` subtree is test-only evidence and is
not a Generic winner oracle.

## Current source-to-selection evidence

| fixture class | source witness | current generic schedule | status |
| --- | --- | --- | --- |
| V0-only | `v0-additive` | no proven V0-only result | `UnresolvedStop` |
| V1-only | `v1-only` | `GenericLoopV1` | observed |
| Both | `both` | release/strict: `GenericLoopV0, GenericLoopV1`; planner-required: `GenericLoopV1` | observed overlap; precedence unresolved |
| Neither | `neither` | empty | `PreEffectDeclined` before Builder effects |

`contract_present = false` is an ordinary current Generic input for release
and strict modes. It is recorded in the matrix; it is not silently converted
to a Generic pre-effect decline. The pure nested-carrier policy probe may still
return `UnresolvedStop` when that contract receipt is absent.

## Stage and disposition contract

The matrix records these stage arms separately:

```text
facts absent/non-match
composer precondition with no candidate delta
composer first allocation/body/pipeline delta
composer error after candidate delta
strict shadow Some/None/Err
release verifier Ok/Err
release lower Some/Ok(None)/Err
nested fastpath and nested Generic fallback
```

The closed debt vocabulary is:

```text
PreEffectDeclined   facts/policy miss with no Builder effect
PreEffectBlocked    source/policy precondition unavailable before mutation
TerminalFreezeTarget candidate was effected; retry would reuse dirty state
ImpossibleEdge      closed invariant proves the arm cannot occur
UnresolvedStop      evidence is insufficient to choose the above
```

An effectful composer/verifier/lowerer failure is never labelled
`PreEffectDeclined`. Unobserved natural arms are retained as
`NotYetObserved`/`UnresolvedStop` rows; no failure injection is used.

## Snapshot ownership

The matrix compares `before_compose`, `before_lower`, and `after_lower`
snapshots containing block count, next ValueId, typed-value count, and variable
map size. Variable-map restoration is not candidate rollback: the composer can
leave block/value/type counters changed. Therefore `GenericComposer` is the
first effect owner whenever the compose delta changes those counters, even if a
later verifier is pure.

## Non-claims

This reference does not claim:

* V0/V1 semantic precedence or winner equivalence;
* a debt-to-later-winner trace;
* a portable Generic Recipe producer or consumer;
* shared JoinSig/PHI/physicalizer ownership;
* retry/fallback removal or JoinIR deletion;
* any language grammar or source syntax change.

Those claims remain blocked until the parent M4 design stop closes with a
complete matrix, precedence/disjointness proof, and witness equivalence.
