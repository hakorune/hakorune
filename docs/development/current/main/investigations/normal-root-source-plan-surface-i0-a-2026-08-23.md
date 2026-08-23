# Normal root parser-backed source-plan surface I0-A

Status: fast — parser surface bound only
Date: 2026-08-23
Decision: NORMAL-ROOT-SOURCE-PLAN-SURFACE-I0-A
Parent: NORMAL-ROOT-SOURCE-PLAN-SURFACE-D0
Owner: parser finalizer -> parser callable-source product

## Six-line brief

Decision:
  Consume the retained parser seed once and issue one private, non-Clone
  source-plan surface bound containing nested top-level and Main/member rows.
Source authority + canonical issuer:
  The parser source relation already held by the postpass; the sole bound
  issuer is `ParsedProgramWithCallableParameterSourceV1::new`.
Non-authority:
  MIR/Builder, `NormalSourceSurfaceInventoryV1`, `VerifiedRawRootExpansionV1`,
  names/ordinals as join keys, compatibility retry, and the AST after this
  parser handoff.
Fail-fast boundary:
  Missing/foreign/duplicate slot, member, callable, or parser relation, and
  any incomplete seed terminal, become a typed source disposition before the
  bound is exposed.
Smallest next slice:
  Parser source relation and bound transport only; preserve the existing
  `SealedNormal*` policy and root lifecycle unchanged.
Non-claims:
  No policy selection, transform transport, root work-plan change, Builder
  effect, fallback, fixture expansion, or production root switch.

Census boundary: postpass finalizer after selected BuildGate projection and
compatibility/source-backed program construction ->
`ParsedProgramWithCallableParameterSourceV1::new`; includes ordinary and
source-backed initial-compatibility routes plus seed terminals; excludes
normal source-plan policy, final transform, root lifecycle, and publication.

## Change

1. Replace the seed's downstream use of independently pairable
   `projected_program_slots` and `static_parent_sources` with one private
   nested surface relation. A top-level row owns its parser source relation,
   final placement witness, and exactly one observation. A Main-box row owns
   the complete ordered member relation as a child product.
2. Keep the full prepared static-parent/member rows in the parser seed until
   this consumer. Reduce `ParserStaticBoxSourceSealV1` to an owned narrow
   projection (parent relation, Main method relation, and member-coverage
   witness); it must not retain the full prepared rows or borrow the seed.
3. Make `ParsedProgramWithCallableParameterSourceV1::new` consume the seed
   disposition exactly once and retain one required private
   `ParserBackedNormalSourcePlanBoundV1`/typed terminal field. The ordinary
   source-backed and initial source-backed compatibility paths must use the
   same issuer; non-source-backed compatibility remains an explicit terminal.
4. The parser issuer may inspect the final parser AST once while co-sealing
   the parser path and slot. It must emit owned source observations and must
   not leak an AST view or perform policy classification. No compiler module
   may call `from_program`, `expected_callable_slots`, or the AST inventory
   for this surface.

## Contract

```text
parser finalizer / selected compatibility finalizer
  -> ParserNormalSourcePlanSeedDispositionV1
  -> ParsedProgramWithCallableParameterSourceV1::new
  -> one ParserBackedNormalSourcePlanBoundV1
       CompleteEmpty | CompleteRows
       top-level row = relation + slot + observation
       MainBox row = relation + full ordered member rows
```

The bound is source-only. It contains no Recipe key, selector, root-role
`bool`, ValueId, MIR type, BasicBlockId, physical receipt, or fallback route.
`CompleteEmpty` is issued only from a parser-proven zero-row surface; missing
or unavailable seed data is a typed terminal, never an empty/default product.

The existing narrow Main admission consumes the narrow projection. It does
not decide the full source-plan surface and does not copy the full member
rows. The future policy cell will consume the bound and evolve the existing
`SealedNormal*` family; this cell does not call that policy.

## Done

Positive evidence:

- empty Program produces an explicit complete-empty source surface;
- executable-only Script has one complete top-level observation;
- exact static `Main.main/0` retains the full nested member relation;
- Main helper and top-level callable rows are retained without re-pairing;
- executable sibling and unsupported rows remain explicit observations;
- ordinary and source-backed initial-compatibility routes use the same seed
  issuer or an explicit typed outside terminal.

Negative evidence:

- foreign parser brand, duplicate path/slot, missing member, duplicate Main,
  missing Main method, wrong staticness/arity, and incomplete seed reject
  before the bound is exposed;
- compatibility-only postpass cannot silently enter the source-backed bound;
- no AST/name/ordinal-only reconstruction can compile as the bound consumer.

Structural guards:

```text
bound issuer definition = 1
bound production caller = 1
seed Ready -> Consumed = 1 on each admitted source route
full member rows retained by narrow Main seal = 0
bound contains AST reference / raw pointer / Recipe or physical ID = 0
compiler source-plan AST inventory caller = 0
compiler from_program/expected_callable_slots caller = 0
parallel source-row arrays exposed to policy = 0
fallback/retry = 0
touched production/test source < 760 lines; hard stop at 800
```

## Stop

Return to `design_stop` without adding a compatibility adapter if any of the
following appears:

- the two seed arrays must be joined outside the parser issuer;
- the narrow static seal must retain or borrow the full rows;
- ordinary and source-backed initial-compatibility routes require different
  source issuers without a named typed terminal;
- `new` cannot consume every seed disposition without `Option`/default
  merging;
- the bound needs AST/name/ordinal authority, policy meaning, or physical IDs;
- a touched source/test file reaches 800 lines or the cell needs policy,
  transform, root, fallback, or Builder changes.

## Non-claims

`NormalSourcePlanClassifierV1::seal_parser_bound`, transform preservation,
`SealedNormal*` evolution, retained-source production use, raw-root classifier
retirement, root work-plan typing, and the C0 production switch are separate
cards. This cell may land only the parser product and its focused evidence.
