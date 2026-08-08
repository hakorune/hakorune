---
Status: closed — accepted design; I0 opened
Date: 2026-08-09
Decision: one parser-seal-consuming Box source handoff; no second source authority
Parent: `loop-resolver-canonical-callable-contract-d0-design-task-2026-08-08.md`
Reference: `docs/reference/language/callable-contracts.md`
---

# RESOLVER-BOX-SOURCE-HANDOFF-D0

## Decision

The rich Rust parser already issues one non-Clone
`ParserBoxSourceSealV1` for the bounded ordinary top-level Box cohort. The
next boundary is not another inventory or a resolver rescan. It is one
parser-owned handoff that consumes that seal and exposes an opaque, AST-free
source capability to the resolver.

```text
ParserBoxSourceSealV1 (non-Clone, parser authority)
  -> one consuming parser→resolver handoff
  -> resolver-owned source declaration capability
  -> declaration/signature issuer
```

The handoff is a transfer receipt, not a new semantic authority. It preserves
the parser seal's exact source identity and typed syntax while preventing
resolver code from recovering facts from AST, names, JSON, `HashMap`, or
selected inventory order.

## What the handoff owns

The handoff may carry only parser-source facts needed by the next resolver
row:

```text
same parser invocation/source brand
exact Box declaration site
exact as-written method source site
method declaration header data needed for semantic signature resolution
typed CallableContractSyntaxV1 row, when present
inventory ordinal only as diagnostic placement
```

The handoff does not issue or own:

```text
NominalBoxTypeId
semantic parameter/result types
VerifiedHomeAbi
Query behavior obligation
instance target
source-bound call relation
Recipe/CallSlot
MIR/FunctionSignature/EffectMask
provider/runtime route
fallback or retry
```

The resolver declaration row will issue nominal identity and semantic
signature from this exact source capability. The canonical callable issuer
will later co-seal Query and the same-declaration `VerifiedHomeAbi`; neither
axis is fabricated by this handoff.

## Ownership and API shape

`ParserBoxSourceSealV1` remains the sole parser authority and remains
non-Clone. A future consuming parser entry should be the only way to issue the
handoff. The handoff itself is also non-Clone until the resolver declaration
issuer has consumed it.

```text
parser-private seal
  -> consuming parser handoff entry
  -> opaque resolver ingress product
```

No public constructor may accept arbitrary source rows. In particular, the
following are forbidden:

```text
clone seal relations and pass them independently
rebuild a seal from BoxMethodInventoryV1
sort names or inventory ordinals to infer source order
read raw `ExplicitSource` as resolver authority
reparse AST or JSON in resolver
construct a Verified* product in tests without an issuer
```

The exact module/file split is intentionally left to the implementation slice.
The parser source-authority and source-seal files are already near their
800-line cleanliness boundary; the handoff must use a small dedicated module
or a narrow parser entry rather than growing either file into a second
orchestrator.

## Source identity versus placement

The handoff uses `SourceBoxMethodSiteV1` (as-written member identity and
selected-gate path) as resolver identity. `BoxMethodInventoryOrdinalV1` is
placement/diagnostic data only. Generated property/delegate rows have no
explicit source method site and cannot enter the instance-method declaration
row through this handoff.

The parser seal's bounded cohort is explicit:

```text
accepted now:
  ordinary top-level Rust Box, complete rich-parser final seal

outside this row:
  Hako parser source seal
  interface/static/record/mixed cohorts
  AST-only postpass compatibility products
  generated-only methods without an explicit source site
```

If the required source relation is unavailable, the implementation stops at
`NoSafeSlice`; it does not downgrade to a compatibility map or invent a
resolver-side relation.

## Typed syntax and conflict boundary

`CallableContractSyntaxV1::Query` is parser-normalized syntax carriage. Raw
rune strings do not cross this boundary. Parser syntax errors remain parser
errors; semantic conflicts with `Profile`, `Ownership`, `ReturnsOwned`,
`CallConv`, or legacy `Contract(pure|readonly)` are checked by the canonical
semantic issuer, not duplicated in this handoff.

The current typed syntax uses a declaration-local rune ordinal as its syntax
coordinate. It is diagnostic/source-local data, not semantic identity and not
a replacement for `SourceBoxMethodSiteV1`.

## Disposition and fail-fast matrix

This row has a development state and source dispositions; they must not be
collapsed:

```text
seal/handoff issuer not implemented          -> NoSafeSlice
missing CallableContract(query)              -> Declined (after issuer exists)
missing source/site/nominal inventory        -> Unresolved
foreign brand/site, duplicate, forged row    -> Rejected
one exact same-brand handoff                 -> Candidate
```

Precedence after an issuer exists is:

```text
Rejected > Unresolved > Declined > Candidate
```

`NoSafeSlice` is never a source disposition and never authorizes fallback.

## Minimal implementation slice after this D0

The next I0 may do only the following:

1. consume one parser-private non-Clone seal;
2. issue one opaque AST-free handoff for the bounded ordinary Rust Box cohort;
3. prove exact source-site/placement separation and same-brand ownership;
4. preserve typed Query carriage without raw string matching;
5. reject foreign, duplicate, generated-only, missing, or post-consume reuse;
6. update the parser/resolver owner README, language reference receipt, task
   card, and focused guards in the same commit.

The I0 must not issue semantic signature, Home ABI, target, call relation,
Recipe, CallSlot, body conformance, Builder/MIR, provider/runtime, or
publication products. Those remain later ordered rows.

## Acceptance gates

```text
positive: one ordinary Box method with Query typed carriage survives handoff
negative: foreign brand/site, duplicate row, generated-only row, reuse
negative: inventory ordinal changed while source site stays stable
negative: raw JSON/HashMap/name lookup cannot issue the handoff
structural: handoff module remains below 800 lines
structural: no MIR/Builder/Recipe imports
docs: owner README + docs/reference + active task receipt same commit
```

No production resolver activation is claimed by this design. The next design
stop remains here until the handoff and its issuer have a focused acceptance
card; only then may semantic declaration/signature and Home ABI rows open.
