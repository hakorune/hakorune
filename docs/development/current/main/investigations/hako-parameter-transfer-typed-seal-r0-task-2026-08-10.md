# HAKO-PARAMETER-TRANSFER-TYPED-SEAL-R0

Status: R0a closed; R0b selected
Date: 2026-08-10
Depends on: `HAKO-PARAMETER-TRANSFER-TYPED-SEAL-D0`

## Goal

Replace the disconnected Hako parameter carrier's raw String authority and
consumer-visible builder token with the closed transfer syntax and exact
parser-session/method co-seal selected by D0. This row does not activate Take.

## Required change

```text
canonical parser issuer
  -> Ordinary | Take typed syntax capability

ParserProgramSourceSession brand
+ exact method source site
+ parameter-list issuer seal
  -> sealed parameter-list product
```

Consumers compare source membership through limited APIs such as
`same_parser_source(...)` or an atomic co-seal. They do not read or compare a
builder object, raw tag String, or public token.

Use a two-commit behavior-neutral Refactor Series:

```text
R0a:
  issue one opaque closed Ordinary/Take vocabulary
  remove consumer `kind()` and raw-string classification

R0b:
  co-seal parser session + exact method + parameter-list issuer
  remove public `sealed_token()` and builder-as-brand
```

The exact owners are `parameter_syntax_records_v1.hako`,
`parameter_list_builder_v1.hako`, `parameter_list_sealer_v1.hako`,
`parser_source_session_v1.hako`, and `source_declaration_refs_v1.hako`.

## Acceptance

- arbitrary `"Ordinary"` / `"Take"` construction cannot issue authority;
- typo/foreign issuer/foreign method/duplicate transfer rows reject;
- existing Ordinary-only normalized output is equality-identical;
- Rust/Hako parity has one typed transfer vocabulary;
- `sealed_token()` has no semantic consumer and is removed or private;
- owner README, focused tests, task receipt, and language/parser reference are
  updated in the same commit;
- all touched source files remain below 800 lines.
- an Ordinary row represents absent or present declared type explicitly;
  untyped `skip_while` parameters never require an empty-string fake type;

`tools/checks/hako_parser_parameter_list_h2_s1_guard.sh` must prove that no
raw `"Ordinary"` / `"Take"` comparison exists outside the issuer and no
consumer uses `sealed_token()`.

## Nonclaims

```text
Take grammar activation
Home demand / Home ABI
Share / Release meaning
Resolver target / Recipe / Builder / MIR
```

## R0a closeout

R0a is closed behavior-neutrally:

- `ParserParameterTransferKindV1` is one closed parser-private
  `Ordinary | Take` vocabulary backed by an opaque issuer seal;
- only `Ordinary` has an issuing factory, and consumers use the limited
  `accepts_ordinary()` capability instead of reading a raw kind;
- declared parameter type syntax is explicit `Absent | Explicit`, so an
  untyped ordinary parameter is admitted without treating an empty String as
  a malformed or inferred type;
- the guard rejects raw `"Ordinary"` / `"Take"` comparison, escaped enum
  construction, a Take issuer, and files at or above 800 lines;
- the focused fixture covers ordered typed rows, one untyped row, duplicate
  and empty-name rejection, foreign site/ordinal rejection, and closed-state
  rejection.

Evidence:

```text
bash tools/checks/hako_parser_parameter_list_h2_s1_guard.sh
  -> summary=ok
  -> raw_transfer_string_authority=0
  -> untyped_parameter_explicit_absence=1
  -> take_syntax_construction=0
```

R0a does not make the builder a parser provenance authority. R0b remains the
only next row and must bind the list seal to the exact parser session and
method while removing `sealed_token()` and builder-as-brand.
