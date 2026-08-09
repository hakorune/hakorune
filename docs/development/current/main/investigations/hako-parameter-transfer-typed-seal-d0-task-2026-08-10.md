# HAKO-PARAMETER-TRANSFER-TYPED-SEAL-D0

Status: parked design stop; required before Take source I0
Date: 2026-08-10

## Goal

Replace the disconnected Hako parameter carrier's future raw-string extension
with one closed parser-syntax vocabulary and a real parser-session/method
co-seal before `Take` is implemented.

Exact code owners to audit:

```text
parameter_syntax_records_v1.hako
parameter_list_builder_v1.hako
parameter_list_sealer_v1.hako
parser_source_session_v1.hako
source_declaration_refs_v1.hako
```

## Decision questions

1. How does Hako represent the closed syntax set `Ordinary | Take` without
   allowing arbitrary caller strings?
2. Which canonical issuer owns each tag/capability?
3. How are `ParserProgramSourceSession` brand, exact method site, and parameter
   list issuer sealed without exposing a builder instance?
4. Which limited comparison/co-seal API replaces consumer-visible
   `sealed_token()`?
5. How does Rust/Hako normalized parity encode the typed transfer row?

## Required output

```text
owner/non-owner table
closed constructor/issuer API
same-parser-source co-seal API
typed reject matrix
R0 implementation task
legacy raw-string/token removal condition
focused guard/corpus update list
```

The resulting R0 is a short behavior-neutral Refactor Series:

```text
R0a:
  opaque canonical Ordinary/Take vocabulary
  remove consumer `kind()` / raw-string classification

R0b:
  parser-session + exact-method issuer co-seal
  remove public `sealed_token()` and builder-as-brand
```

Extend `tools/checks/hako_parser_parameter_list_h2_s1_guard.sh` to reject raw
`"Ordinary"` / `"Take"` comparison outside the issuer and any
`sealed_token()` consumer.

## Hard stops

```text
no `_kind = "Take"` authority
no arbitrary public transfer constructor
no builder object as parser provenance/semantic brand
no downstream string reclassification
no Home demand/ABI issuance in parser
no Take grammar activation before this D0 is accepted
```
