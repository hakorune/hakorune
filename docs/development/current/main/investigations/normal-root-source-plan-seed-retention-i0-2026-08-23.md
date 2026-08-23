# Normal root source-plan seed retention I0

Status: active fast — one parser postpass ownership edge
Date: 2026-08-23
Decision: NORMAL-ROOT-SOURCE-PLAN-SEED-RETENTION-I0
Owner: parser postpass finalizer -> completed parser product

## Six-line brief

Decision:
  Preserve one parser-owned source-plan seed across the ordinary postpass
  finalizer and completed parser product. The seed is transport only; the
  later `new` consume and normal source-plan bound are separate cells.
Source authority + canonical issuer:
  `OpenParserPostpassProductV1::finalize` is the sole seed issuer for the
  ordinary parser-backed path. It co-seals the existing projected program
  slots and the full prepared static-parent/member rows already owned by the
  parser postpass transaction.
Non-authority:
  AST/name/ordinal scans, `ParserStaticBoxSourceSealV1` as full-row owner,
  `ParsedProgramWithCallableParameterSourceV1::new`, normal source-plan policy,
  Builder, MIR, compatibility fallback, and Raw.
Fail-fast boundary:
  Missing projected slots, foreign relation, duplicate relation, or incomplete
  static-member coverage rejects before initial-source issuance; no sibling
  rows are silently dropped.
Smallest next slice:
  Issue one non-`Clone` seed, borrow its projected slots only while producing
  owned `InitialCallableFinalSlotV1` rows, and move the seed into the existing
  completed source product.
Non-claims:
  No `Ready -> Consumed` product boundary yet, no `ParserBackedNormalSourcePlanBoundV1`,
  no policy/transform/root cutover, no Builder effect, no compatibility change.

Census boundary: ordinary `OpenParserPostpassProductV1::finalize` from the
same parser invocation through `ParsedProgramWithSourceV1` and
`CompletedParserPostpassV1::from_source_product`; compatibility/static total
postpass remains an explicit non-participating arm for this cell.

## Ownership contract

```text
prepared source session
  -> one ParserNormalSourcePlanSeedV1
  -> seed.projected_program_slots() borrowed by initial source issuer
  -> owned InitialCallableFinalSlotV1 values
  -> ParsedProgramWithSourceV1 owns seed
  -> CompletedParserPostpassV1 owns seed
```

The seed owns the full prepared static-parent/member relation. The existing
narrow static seal is not widened and is not introduced as a second owner in
this cell. No AST reference or borrow is stored in the seed or initial source.

The current `Option<ProjectedProgramItemSlotSetV1>` is normalized at the
finalizer boundary: `Some` is the accepted complete relation, while `None`
is a typed missing-slot rejection for this ordinary path. An empty complete
set remains a real parser-issued empty set; it is not represented by
`None`, `default`, or an empty fallback.

## Acceptance

```text
seed issuer definition = 1
ordinary seed assignment = 1
prepared static rows are not discarded = 0
initial issuer receives &ProjectedProgramItemSlotSetV1 = 1
initial source stores seed/slot-set borrow = 0
seed Clone/Copy = 0
AST source-plan role/member classifier added = 0
compatibility fallback or Raw retry added = 0
all touched source/test files < 760 lines
```

Focused evidence must cover an ordinary source with a complete slot set and
an explicit missing-slot rejection. The seed may be inspected only inside
the parser module; no production getter returns parallel rows.

## Next cell, not this cell

The next bounded cell will move the retained seed through
`ParsedProgramWithCallableParameterSourceV1::new` exactly once:

```text
CompletedParserPostpass::Ready(seed)
  -> typed consume
  -> existing source-backed product
```

That cell must first decide the compatibility terminal and the full-row versus
narrow Main projection. It may then open `ParserBackedNormalSourcePlanBoundV1`.
This I0 does not pre-authorize either change.

## Stop conditions

Return to `design_stop` immediately if this slice requires:

- cloning/reissuing projected slots or static rows;
- a seed borrow stored in `VerifiedInitialCallableProgramSourceV1`;
- moving a borrow into `ParsedProgramWithSourceV1` or `CompletedParserPostpassV1`;
- a second static source issuer or an AST source-plan scan;
- changing compatibility behavior or adding a fallback;
- a new semantic `Verified*`/`Prepared*` product;
- a touched source/test file reaching 760 lines.

