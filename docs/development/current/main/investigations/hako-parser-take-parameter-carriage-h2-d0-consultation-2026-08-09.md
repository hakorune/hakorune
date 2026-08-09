---
Status: accepted Decision; implementation 0
Date: 2026-08-09
Parent: `HAKO-PARSER-BOX-DECLARATION-CARRIER-H2/H3`
Blocks: `OWN-HOME-TAKE-DECL-SYNTAX-I0`
---

# HAKO-PARSER-TAKE-PARAMETER-CARRIAGE-H2-D0

## Decision

The final ownership direction is accepted, but `take` syntax does not open
directly. The Hako parser must first complete one source-authority path:

```text
program-owned parser source session
  -> one Box-scoped member cursor per Box
  -> one same-pass ordinary method transaction
       method source site
       atomic parameter-list product
       rich body disposition
  -> the existing sole H3 declaration seal
  -> neutral compatibility projection
  -> resolver source relation
  -> the existing sole Home ABI issuer
```

The census found three prerequisites before the contextual modifier itself is
safe: H1 currently uses one invocation-wide member cursor, the live Hako
method parser has no atomic parameter-list product, and the live body path does
not yet retain a same-pass `ParserNodeProductV1`. `FuncScannerBox`, saved-source
rescans, and ProgramJSON cannot fill any of these gaps.

## Authority split

```text
AST ParamDecl
  Clone-compatible neutral name/type projection only

ParserParameterListProductV1
  parser-private, non-Clone authority
  ordered neutral declarations + exact syntax rows

ParserParameterSyntaxRowV1
  exact method source site
  parameter ordinal
  name
  declared type syntax/site
  ParameterTransferSyntaxV1::{Ordinary, Take}

resolver parameter relation
  declaration identity + parameter ordinal + resolved semantic type

VerifiedHomeAbiV1 issuer
  sole owner of receiver/parameter/result Home demands
```

`ParamDecl` is not widened. Missing typed rows never default to `Ordinary`, and
the resolver never reconstructs `Take` from a parameter name, raw text, AST,
or physical ABI.

For explicit methods, arity is derived from the atomic parameter product.
The draft must not retain a second independently mutable `arity` truth.
Generated rows remain a separate provenance path and do not acquire source
parameter authority in this cohort.

## Source-session correction

H1's caller-token issuer and invocation-wide `_next_member` are disconnected
proof scaffolding, not the durable production owner. Before H2 connects:

```text
ParserProgramSourceSessionV1
  owns the parser invocation brand
  issues top-level Box statement sites

ParserBoxMemberSourceCursorV1
  is opened from one exact Box site
  starts member ordinal at zero for that Box
  cannot issue a site for another Box
```

Production issuer creation must be reachable only from the program-owned
session. Tests may exercise the owner through a focused fixture, but arbitrary
caller-created brands or cross-Box cursors are not accepted authority.

## H2/H3 integration

The existing `source_carrier_v1` declaration builder/sealer remains the only
final source seal. Do not add a sibling parameter sealer.

H2 eventually parses the complete bounded method exactly once:

```text
method header
  -> method source site
  -> exact simple nominal result/type syntax
  -> atomic ordered parameter product
method body
  -> ParserNodeProductV1::{Typed, CompatOnly, ParseError}
```

H3 consumes the unpublished method/Box transaction and verifies, before any
publication:

- same parser session and exact Box/method site;
- parameter ordinals exactly `0..N`, with no gaps or duplicates;
- source-row count and neutral projection count agree;
- name/type projection agrees row by row;
- every admitted parameter has a complete type and transfer syntax row;
- body disposition is present from the same parse;
- finish occurs exactly once and failure publishes nothing.

Only `Typed` body rows may later enter semantic source authority. `CompatOnly`
remains a one-way compatibility projection, never a semantic substitute.

## Contextual `take` contract

The language target remains a contextual `IDENT`, never a lexer keyword.

```ebnf
take_param
  := IDENT("take") HTRIVIA IDENT HTRIVIA ':' type_ref
```

At parameter-declaration head, same-line `take IDENT` commits to the Take
form. After commitment, the same-line `:` and a supported type reference are
mandatory; there is no fallback to ordinary parameters.

```text
take node: Node   -> Take row
take: Node        -> ordinary parameter named `take`
take              -> ordinary untyped parameter
take(node)        -> ordinary call in expression context
take\nnode: Node   -> not contextual Take
take node         -> parser/take_parameter_type_required
```

Both `take -> name` and `name -> ':'` use horizontal trivia only. Hako's
general `skip_ws` includes line terminators and therefore must not be reused
for this decision. Comment trivia is admitted only when the common
horizontal-trivia owner can prove it contains no line terminator.

The durable grammar is broader, but the first executable cohort is exact
top-level ordinary Box direct instance methods. Free/static/interface/
constructor/generated/selected-gate callers of the shared Rust parser remain
closed. Once contextual `take IDENT` is seen in an unsupported declaration
cohort, fail fast with `parser/take_parameter_cohort_unsupported`; do not
silently reinterpret it.

## Ordered task ladder

```text
H2-S0  parser source-session correction
       program-owned invocation + Box-scoped member cursor

H2-S1  atomic ordinary parameter-list product
       Ordinary rows first; neutral projection; no Home meaning

H2-S2  same-pass rich body result
       Typed/CompatOnly/ParseError; JSON is projection only

H2-I0  bounded ordinary Box direct-method branch
       header + parameters + body parsed once into one transaction

H3-I0  final atomic declaration seal
       sole seal; complete method/parameter/body coverage

RUST-R0 sealed parameter source relation
       stop ownership-path reconstruction from Clone `ParamDecl`

PARITY-I0 Rust/Hako normalized parameter-row parity

OWN-HOME-TAKE-DECL-SYNTAX-I0
       contextual Take row only; Home demand remains closed

RESOLVER-PARAMETER-SOURCE-COSEAL-I0
       exact declaration/ordinal/resolved-type relation

HOME-PARAMETER-DEMAND-I0
       only after nominal capability taxonomy can prove movable Home
```

Each row is separate. In particular, H2-S0/H2-S1/H2-S2 are BoxShape rows and
must not activate a new language form.

## Test and guard matrix

H2-S0:

```text
two Boxes each start member ordinal at zero
one Box cursor cannot issue another Box's site
foreign session/site rejects
arbitrary production issuer calls are absent
double finish/post-seal mutation reject
```

H2-S1/H3:

```text
ordered Ordinary/Take/Ordinary rows are preserved
duplicate, gap, reorder, count mismatch, and foreign method/site reject
row name/type mismatch with neutral projection rejects
missing typed row never becomes Ordinary
empty parameter list is exact zero coverage
```

Take/parity:

```text
take node: Node                 contextual
take: Node / take               ordinary
take(node) / obj.take()         ordinary calls
take\nnode: Node                 not contextual
take node / take node = value   stable committed rejection
unsupported declaration cohort  stable fail-fast
Rust/Hako normalized rows equal in source order
```

The grammar registry row opens only when both parsers expose typed source-row
evidence. The current Hako ProgramJSON grammar adapter is not proof of H2/H3
source authority.

## Stop conditions

Stop as `NoSafeSlice` if any implementation requires:

```text
FuncScannerBox or StageB parser authority
saved source/body rescan or substring parsing
JSON decode to reconstruct typed source rows
caller-supplied production brand
one member cursor shared across Boxes
second parameter/declaration sealer
Clone ParamDecl as ownership authority
default Ordinary row for missing evidence
CompatOnly body as semantic source
unsupported cohort silently accepting take
raw type text from compatibility scanner
```

If the bounded Hako branch lacks a canonical `type_ref` product, admit only an
exact simple nominal type in its first product or stop for a type-reference
Decision. Do not seal unsupported complex syntax partially.

## Nonclaims

```text
Home capability or HomeDemand issuance
Home availability/Flow
call-site take or consuming receiver
share/release semantics
selected build-gate, static/interface/constructor support
resolver target / Recipe / CallSlot
Builder / MIR / CFG / PHI
runtime representation
fallback / retry
```

## Closeout

The consultation is closed as an accepted design. The only executable next
row is `HAKO-PARSER-BOX-SOURCE-SESSION-H2-S0`; later rows remain closed until
their predecessor's receipt is complete.
