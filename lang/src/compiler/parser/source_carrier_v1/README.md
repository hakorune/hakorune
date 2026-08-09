# Parser-private Source Carrier V1

This directory owns the construction substrate for the Hako parser's sealed,
source-preserving tree.

P0 rules:

- authoritative parser branches do not import this directory yet;
- records are constructed through factories/builder only;
- raw MapBox, JSON, token text, ProgramV0, MIR, planner, route, backend, and
  runtime meaning are forbidden;
- children are built before parents (`child_id < parent_id`);
- a builder publishes zero or one defensively reconstructed tree;
- `CompatOnly` is a parser product for future ProgramV0 migration and is never
  a source-tree node;
- every source file stays below 800 lines.

Mutation and publication are physically separate: `source_carrier_builder_v1`
owns drafts/state, while `source_carrier_sealer_v1` owns reachability,
bottom-up defensive validation, and immutable reconstruction.

The language cannot enforce module-private constructors. Repository guards
therefore enforce factory-only construction and parser nonconnection during
P0.

## Box declaration carrier D0

The next Hako parser row reuses this lifecycle and sealer, but does not
promote the current index-only `SourceNodeRefV1`/`SourceNodeListRefV1` to
declaration identity. The declaration carrier must add parser-invocation
branded refs/sites and keep its product parser-private until a later resolver
co-seal.

The brand and structural ordinals are issued by their owners: the program
cursor issues the top-level statement ordinal, the ordinary Box branch issues
exact member paths, and the declaration sealer issues independent selected
inventory ordinals. An explicit method source site is never the same thing as
an all-row inventory position; generated property/delegate rows have only a
source-member origin and generated role. Caller-supplied arbitrary ref
constructors are not an authority. Diagnostic offsets are carried separately
and are never used as identity.

```text
ParserProgramBox cursor
  -> ordinary Box parser branch
  -> branded unpublished member transaction
  -> one declaration sealer
  -> ordered inventory + non-Clone parser source seal
```

Here `non-Clone` is a repository-enforced capability boundary, not an affine
type-system claim. The seal has no public copy/clone/reconstruction API and
can be finished only once; H1 guards enforce that shape.

The ordinary branch is the only future source authority. `ParserBox`,
`FuncScannerBox`, `StageBRuneBox`, `tools/hako_parser`, ProgramJSON, and
MapBox remain non-authoritative compatibility surfaces. The branch must parse
the Box body once and carry the resulting `ParserNodeProductV1` disposition;
it must never save a source slice for a later rescan.

H1 is intentionally disconnected and proves only branded refs/sites, exact
source-member paths, separate inventory ordinals, ordered drafts,
duplicate-without-mutation, one-Box seal, foreign-brand/site rejection, and
double-finish rejection. It does not wire the program parser, build-gate
selection, delegate postpass, typed CallableContract carriage, resolver
semantics, Recipe, or publication. The D0/task card is the authority for the
H1 connection and removal conditions.

The focused receipt is `tools/checks/hako_parser_box_declaration_h1_guard.sh`.
The P0 guard keeps an explicit seven-module manifest, so adding these H1
modules cannot silently change the P0 scope.

## H2/H3 parameter-carriage Decision

The accepted next sequence is intentionally narrower than connecting the
ordinary parser in one step:

```text
H2-S0  program-owned parser source session
       + one member cursor per exact Box
H2-S1  atomic ordered parameter-list product
H2-S2  same-pass rich body result
H2-I0  bounded ordinary Box direct-method transaction
H3-I0  sole final declaration seal
```

H2-S0 must remove the invocation-wide member-ordinal authority: Box statement
ordinals belong to the program session, while member ordinals restart at zero
inside each Box cursor. H2-S1 keeps Clone `ParamDecl` neutral and carries exact
parameter source rows in a parser-private non-Clone product. H2-S2 must retain
`ParserNodeProductV1` from the same parse; ProgramJSON is only a projection.

`FuncScannerBox`, saved-source rescans, JSON reconstruction, caller-supplied
production brands, a second declaration/parameter sealer, and defaulting a
missing parameter row to `Ordinary` are forbidden. The owning Decision is
`docs/development/current/main/investigations/hako-parser-take-parameter-carriage-h2-d0-consultation-2026-08-09.md`.
