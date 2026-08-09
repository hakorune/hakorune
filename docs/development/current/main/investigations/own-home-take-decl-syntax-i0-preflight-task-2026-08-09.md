---
Status: census closed as NoSafeSlice; Hako H2/H3 parameter authority required
Date: 2026-08-09
Parent: `OWN-HOME-SYNTAX-D0`
Ceremony: T2 declaration/parser/Home authority boundary
---

# OWN-HOME-TAKE-DECL-SYNTAX-I0-PREFLIGHT

## Goal

Select the smallest exact source carrier for:

```hako
adopt(take node: Node) { }
```

without putting Home meaning into parser data or creating a second callable
Home ABI authority.

## Landed owner census

```text
Rust parameter syntax:
  src/parser/common/params.rs::parse_param_decl_list
  -> shared by functions, Box methods, static methods, interfaces,
     constructors

AST compatibility carrier:
  crates/hakorune_frontend_ast/src/decls.rs::ParamDecl
  -> name + optional declared_type_name
  -> Clone and widely constructed by compatibility/tests

parser -> resolver declaration handoff:
  ResolverMethodParameterSyntaxV1
  -> name + optional declared type only

semantic Home ABI authority:
  VerifiedHomeAbiV1
  -> sole receiver/parameter/result Home-demand aggregate
  -> current bounded schema classifies only I64/Unit as Trivial and hardcodes
     instance receiver Handle

Hako canonical parser owner:
  absent today

Hako compatibility scanner:
  lang/src/compiler/entry/func_scanner.hako
  -> strips/comments and rescans method text
  -> emits params/param_decls JSON
  -> explicitly non-authoritative for source identity

Hako future source authority:
  lang/src/compiler/parser/source_carrier_v1
  -> H1 disconnected substrate is landed
  -> method draft currently carries name/static/arity/result/source site only
  -> H2/H3 ordinary Box branch + final source seal are not connected
  -> ordered parameter syntax rows do not yet exist
```

## Census conclusion

`OWN-HOME-TAKE-DECL-SYNTAX-I0` cannot open safely yet. Rust could parse the
surface, but Hako has no canonical method-header/parameter authority capable of
issuing the matching row. `FuncScannerBox`, ProgramJSON, or raw method text
cannot fill that gap. This is a development `NoSafeSlice`, not a source
`Unresolved` outcome.

The next design card is:

```text
HAKO-PARSER-TAKE-PARAMETER-CARRIAGE-H2-D0
```

It refines the already planned
`HAKO-PARSER-BOX-DECLARATION-CARRIER-H2/H3`; it does not create a parallel
parser or source seal.

## Required design result after the prerequisite

Choose one typed syntax vocabulary such as:

```text
ParameterTransferSyntaxV1::Ordinary
ParameterTransferSyntaxV1::Take { exact modifier site }
```

and decide where it lives so that:

- ordinary compatibility `ParamDecl` constructors do not silently acquire
  ownership semantics;
- `take` stays declaration-local, same-line, contextual `IDENT` syntax;
- exact `take IDENT : TYPE_REF` is required and `take: T` remains an ordinary
  parameter named `take`;
- the parser/handoff carries syntax only;
- the resolver resolves the exact declared type/capability;
- only the Home ABI issuer may project a selected parameter to
  `HomeDemandV1::Home`;
- source syntax and Home ABI are co-sealed by declaration identity and
  parameter ordinal, never by name/string repair.

## Questions the census must close

1. Prefer a parser-private atomic parameter-list product whose neutral AST
   projection remains `ParamDecl`, while exact source rows retain parameter
   ordinal, declared type syntax, and `ParameterTransferSyntaxV1`. Confirm this
   instead of widening every Clone compatibility `ParamDecl` constructor.
2. Extend the canonical Hako H2 ordinary Box branch to parse method headers
   once and place ordered parameter syntax rows in the H3 source seal. Do not
   reuse `FuncScannerBox` or introduce a sibling final seal.
3. Which declaration cohorts enter I0: direct instance methods only, or every
   caller of the shared Rust parameter parser? The durable grammar is general,
   but one-sided activation is forbidden.
4. What exact resolved Box capability allows `HomeDemandV1::Home`? The existing
   `I64UnitTrivial` schema must not be widened by spelling alone.
5. How are duplicate/conflicting modifiers rejected without making `take` a
   lexer keyword or overloading `CallableContract(query)`?

## Mandatory tests for the selected I0

```text
take node: Node     contextual typed parameter
take: Node          ordinary parameter named take
take                ordinary untyped parameter named take
take(node)          ordinary call
obj.take()          ordinary method call
take\nnode: Node     not contextual take
take node            stable missing-type rejection after commitment
take node = value    stable rejection
```

Rust/Hako normalized parity, source-site/parameter-ordinal identity, foreign
declaration rejection, and absence of arbitrary `Verified*` constructors are
required in the same implementation slice.

## Nonclaims

```text
Home capability inference
Home availability/flow
call-site take
consuming receiver
field/place take
share
release semantics
target / Recipe / CallSlot
Builder / MIR / CFG / PHI
runtime representation
fallback / retry
```

## Stop

Stop for design consultation if the implementation would require changing all
legacy `ParamDecl` constructors at once, inferring Home from a type/name string,
duplicating parameter demand outside `VerifiedHomeAbiV1`, guessing the Hako
owner, activating only one parser, or adding a global `TAKE` token.

The present row stops here because the canonical Hako owner is absent.
