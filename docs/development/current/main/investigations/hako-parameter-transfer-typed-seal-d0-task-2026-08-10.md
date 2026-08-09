# HAKO-PARAMETER-TRANSFER-TYPED-SEAL-D0

Status: accepted Decision; R0a selected
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

## Decision

Use one parser-private closed enum plus an opaque wrapper bound to an exact
parameter-list issuer seal:

```text
ParserParameterTransferKindV1::{Ordinary, Take}

ParserParameterTransferSyntaxV1
  private kind
  exact parameter-list issuer seal
  optional modifier site
```

The wrapper has no raw kind getter. R0 exposes only the Ordinary issuer. Take
is reserved vocabulary with no issuing API until Take I0. Any normalized
`"Ordinary"` text is serializer output only, never input or evidence.

`ParserProgramSourceSessionV1` issues one
`ParserParameterListIssuerSealV1` for an exact owned method site. Duplicate or
foreign issuance rejects. The builder retains that seal but is not itself a
brand. The sealer consumes `(method_site, rows, list_seal)`, and the finished
product keeps the seal private. Allowed comparisons are limited to
`same_parser_source(...)`, `same_parameter_list_source(...)`, and bounded
`parameter_is_ordinary(index)`.

Ordinary parameters may have no declared type. The source row represents
`None | Some(type token)` explicitly; empty string is not a type and absence
does not reject. Take remains typed when its grammar opens.

## Implementation series

```text
owner/non-owner table
closed constructor/issuer API
same-parser-source co-seal API
typed reject matrix
R0 implementation task
legacy raw-string/token removal condition
focused guard/corpus update list
```

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
