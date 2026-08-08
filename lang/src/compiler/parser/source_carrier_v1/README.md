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
branch-local member ordinals, and the declaration builder/sealer issues the
selected method ordinal. Caller-supplied arbitrary ref constructors are not
an authority. Diagnostic offsets are carried separately and are never used
as identity.

```text
ParserProgramBox cursor
  -> ordinary Box parser branch
  -> branded ordered declaration drafts
  -> one declaration sealer
  -> sealed parser product
```

The ordinary branch is the only future source authority. `ParserBox`,
`FuncScannerBox`, `StageBRuneBox`, `tools/hako_parser`, ProgramJSON, and
MapBox remain non-authoritative compatibility surfaces. The branch must parse
the Box body once and carry the resulting `ParserNodeProductV1` disposition;
it must never save a source slice for a later rescan.

H1 is intentionally disconnected and proves only branded refs/sites,
ordered drafts, duplicate-without-mutation, one-Box seal, foreign-brand/site
rejection, and double-finish rejection. It does not wire the program parser,
build-gate selection, runes, resolver semantics, Recipe, or publication. The
D0/task card is the authority for the H1 connection and removal conditions.
