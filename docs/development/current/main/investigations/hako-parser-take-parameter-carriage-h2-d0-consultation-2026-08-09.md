---
Status: current design consultation; implementation 0
Date: 2026-08-09
Parent: `HAKO-PARSER-BOX-DECLARATION-CARRIER-H2/H3`
Blocks: `OWN-HOME-TAKE-DECL-SYNTAX-I0`
---

# HAKO-PARSER-TAKE-PARAMETER-CARRIAGE-H2-D0

## Closed evidence

The Rust parser has one shared parameter-list parser and a broad Clone
compatibility `ParamDecl { name, declared_type_name }`. The parser→resolver
method signature handoff currently carries only the same two fields.

The Hako parser has no connected canonical method-header parser:

- `parser/program` publishes shallow declaration evidence, not Box methods;
- `entry/func_scanner.hako` rescans stripped source text and emits JSON, and is
  explicitly a compatibility surface rather than source authority;
- `parser/source_carrier_v1` H1 has the correct branded transaction/sealer
  substrate but is disconnected;
- its current method draft has name/static/arity/result/source site, but no
  ordered parameter syntax rows;
- H2/H3 already own the future ordinary Box branch and final parser source
  seal, so a new parser or seal would be duplicate authority.

Therefore `take` parser I0 is `NoSafeSlice` until H2/H3 can carry parameters.

## Recommended architecture

```text
one method-header parser
  -> ParserParameterListProductV1
       neutral declarations[]
       exact source rows[]
         parameter ordinal
         name site
         declared TYPE_REF syntax/site
         ParameterTransferSyntaxV1::{Ordinary, Take}
  -> ordinary Box method draft
  -> existing H2/H3 declaration transaction
  -> one final parser source seal

neutral AST projection:
  ParamDecl { name, declared_type_name }

resolver source handoff:
  exact declaration id + parameter ordinal + typed transfer syntax

Home ABI issuer:
  resolved capability + typed transfer syntax
  -> VerifiedHomeAbiV1 parameter demand
```

The parameter-list product is the single parser truth. `ParamDecl` is its
neutral compatibility projection, not a second ownership authority. Hako and
Rust may implement different internals, but normalized source rows must match.

## Decisions requested

1. Should the durable syntax row live in the parser-private parameter-list
   product, with `ParamDecl` kept neutral, or is widening `ParamDecl` justified?
2. Should H2 parse the complete ordinary Box method header (including exact
   parameter/type sites) and H3 seal it atomically with the method source site?
3. Is direct instance method the correct first cohort, with every other shared
   Rust parameter-parser caller rejecting contextual `take` until its own
   parity row exists?
4. Which exact no-line-terminator/token commitment contract should both parsers
   expose without making `take` a global keyword?
5. What minimal negative/parity matrix is required before
   `OWN-HOME-TAKE-DECL-SYNTAX-I0` can open?

## Recommended answer

Use the parser-private atomic product; keep `ParamDecl` neutral. Extend H2/H3,
do not create another Hako parser/seal. Limit I0 to direct instance methods,
while the shared Rust parsing engine uses an explicit policy so unsupported
declaration cohorts reject rather than silently reinterpret `take`. Require
exact same-line `take IDENT : TYPE_REF`, typed source rows, source-order parity,
ordinary `take: T` preservation, foreign/duplicate rejection, and no semantic
Home issuance.

## Nonclaims

```text
Home capability or HomeDemand issuance
Home Flow / availability
call-site take
consuming receiver
share/release semantics
resolver target / Recipe / MIR
FuncScanner or ProgramJSON authority
runtime representation
```

## Stop

No code implementation is authorized by this card. Record the accepted owner,
product shape, cohort, fail-fast boundary, H2/H3 integration, and test matrix
before opening an implementation row.
