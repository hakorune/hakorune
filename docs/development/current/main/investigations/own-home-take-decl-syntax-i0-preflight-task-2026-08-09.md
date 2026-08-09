---
Status: current census/design stop; implementation 0
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

Hako parity owner:
  not yet identified by the bounded census; do not infer it from body JSON or
  add a one-sided Rust production
```

## Required design result

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

1. Should the typed transfer marker extend `ParamDecl`, or should explicit
   method source inventory carry a sibling parameter-transfer row so legacy
   AST constructors stay neutral?
2. Which Hako parser module owns method/function parameter declarations, and
   can it preserve the same no-line-terminator contextual decision?
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
