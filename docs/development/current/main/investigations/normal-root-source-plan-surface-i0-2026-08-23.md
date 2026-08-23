# Normal root parser-backed source-plan surface I0

Status: parked draft; `NoSafeSlice` until D0-E seed ownership is accepted
Date: 2026-08-23
Decision: NORMAL-ROOT-SOURCE-PLAN-SURFACE-I0
Owner: parser invocation -> one bound source-plan surface

## Six-line brief

Decision:
  Add the first production parser edge for one opaque,
  non-`Clone` full root source-plan surface. Preserve the existing narrow
  App admission as a projection; do not widen it into a classifier.
Source authority + canonical issuer:
  `ParsedProgramWithCallableParameterSourceV1::new` consumes one postpass
  source-plan seed together with the already completed callable anchors,
  body/source rows, and parameter catalog. One parser surface issuer
  co-seals them.
Non-authority:
  Builder, `NormalSourceSurfaceInventoryV1`, raw AST/name/ordinal scans,
  `NormalSourcePlanClassifierV1`, MIR, compatibility, and independent site
  arrays.
Fail-fast boundary:
  Foreign/duplicate/missing member or statement relation, incomplete coverage,
  or parser witness mismatch returns a typed terminal before policy or Builder.
Smallest next slice:
  Move one postpass seed through the parser product, issue and store the
  parser-bound surface exactly once, with focused positive/negative parser
  tests; no normal source-plan policy or root switch.
Non-claims:
  No `SealedNormal*` policy cutover, AST-free lowerer, Builder lifecycle
  change, fallback removal, compatibility expansion, or physical MIR change.

Census boundary: parser postpass completion ->
`ParsedProgramWithCallableParameterSourceV1::new` -> parser-bound surface
terminal; includes Script top-level rows, static Main/member rows, callable
anchors, empty/missing/unsupported/foreign outcomes; excludes policy,
transform, root work-plan, and post-terminal lowering.

## Preconditions

This card is executable only after `CURRENT_STATE.toml` changes to `fast`, the
D0 card records the accepted surface/transform/output decision, and D0-E
proves the one-shot seed move from postpass finalizer to `new`. A worker report
is premise evidence, not implementation permission. If the full relation
cannot be issued from one parser invocation, return to `NoSafeSlice` instead
of adding an adapter.

## Required product shape

The production edge must retain one opaque aggregate:

```text
ParserBackedNormalSourcePlanBoundV1 {
  parser_invocation_witness,
  complete_surface,
  source_loan_relation,
  private seal,
}
```

The bound is issued from one `ParserNormalSourcePlanSeedV1`; the seed is
consumed exactly once and reaches an explicit `Consumed` terminal. The seed,
not an AST clone or a second static seal, owns the full member relation while
the parameter catalog is joined at the final parser-product boundary.

The surface is a closed state, not parallel arrays:

```text
CompleteEmpty(ExactEmptyWitness)
CompleteRows(NonEmptyRows)
SourceAuthorityUnavailable(reason)
Incomplete(reason)
IntegrityInvalid(reason)
```

Each row co-seals its parser statement/member relation with exactly one
observation:

```text
Executable
TopLevelCallable(exact parser callable relation)
MainBox(exact static-parent relation + every ordered member row)
Unsupported(parser-owned syntax kind)
```

Names, arity, staticness, and ordinal are syntax/coverage evidence only. They
cannot be used as cross-product keys. The bound is non-`Clone`, its constructor
is private, and the parser product holds it as a required source-backed field.

## Implementation cells

### I0-A — parser relation model and issuer

1. Add the parser-owned source-plan surface module under
   `src/parser/callable_parameter_source/`.
2. Reuse existing parser-issued body rows, callable anchors, parser witness,
   and static-parent/member source relations. If an existing seal hides the
   complete member rows, expose a parser-private relation view; do not scan the
   AST again and do not alter the narrow App admission's cohort.
3. Invoke the sole issuer from the existing parser product constructor exactly
   once and store the resulting disposition.
4. Keep all constructors/seals private to the parser module. No public getter
   may return independently pairable rows or an AST reference.

### I0-B — focused evidence

Positive cases:

- empty Program -> `CompleteEmpty`;
- executable-only Script -> `CompleteRows(Executable)`;
- exact static `Main.main/0` -> one complete Main relation;
- Main helper -> complete ordered member relation;
- top-level callable plus Main -> both rows in one surface;
- executable sibling plus Main -> both rows in one surface.

Negative cases:

- non-Main static parent -> explicit unsupported/outside terminal;
- missing or duplicate callable/member relation -> typed incomplete/integrity;
- foreign parser witness -> typed integrity reject;
- empty coverage versus missing coverage -> distinct states;
- unsupported member -> row-level unsupported, never default/empty;
- second issue/loan or `Clone` attempt -> compile/visibility guard.

Each reject must prove no Builder or policy effect occurred. Parser tests may
inspect the disposition inside the parser module; they must not construct a
fake production receipt from an AST.

### I0-C — reusable guard and closeout

Add or extend one parser source-plan guard only after the production caller is
connected. It must prove:

```text
surface issuer definition = 1
surface issuer production call = 1
bound constructor outside issuer = 0
AST/name/ordinal source-plan scan in this route = 0
parallel relation transport = 0
Clone/Copy bound = 0
Builder/policy caller from parser surface = 0 for this I0
all touched source/test files < 760 lines
```

Run the exact focused parser test path, the reusable guard, pointer guard, and
`git diff --check`. Classify any pre-existing red separately from an I0
regression; do not waive an unclassified failure.

## Stop conditions

Return to `NoSafeSlice` before editing if any item appears:

1. Full Main/member coverage requires a second AST scan or a name/ordinal join.
2. Existing parser products cannot provide the relation without a new parser
   authority or a parallel source family.
3. The bound must be `Clone`, `Copy`, `Arc<AST>`, or an independently returned
   row array to cross the constructor.
4. Issuing the bound changes App/Script policy or requires a Builder call.
5. A focused positive requires a synthetic/default row instead of parser
   evidence.
6. A touched source/test file would reach 760 lines; split by owner before
   continuing, and stop at 800.

## Done

The I0 row is complete only when the parser product owns one issuer result,
positive/negative evidence covers the declared census boundary, the guard
proves one issuer/one call/no re-scan, and the change is committed and pushed.
This row does not claim the normal source-plan policy or root lifecycle
cutover; those are the next explicitly selected cells.
