---
Status: current design consultation; implementation 0
Date: 2026-08-09
Decision: pending; select the sole same-pass rich body result before H2 connects
Parent: `HAKO-PARSER-TAKE-PARAMETER-CARRIAGE-H2-D0`
Predecessor: `HAKO-PARSER-PARAMETER-LIST-PRODUCT-H2-S1` closed
---

# HAKO-PARSER-RICH-BODY-RESULT-H2-S2-D0

## Problem

H1/H2-S0/H2-S1 now provide source identity, Box-scoped member order, and an
atomic ordinary parameter-list product. The missing boundary is the method
body from the same parse.

Current evidence:

```text
ParserNodeProductV1
  exists in source_carrier_v1
  currently exercised only by disconnected fixtures

live Hako statement/expression parser APIs
  primarily return compatibility JSON strings

ParserProgramBox / ParserDeclarationBox
  do not yet own an ordinary Box direct-method branch

FuncScannerBox
  strips, slices, rescans, injects receiver data, and emits JSON
  compatibility-only; never source authority
```

Connecting H2 without a same-pass rich result would force either a saved-source
rescan or JSON-to-typed reconstruction. Both are forbidden second parsers.

## Required final direction

```text
one ordinary method parse
  -> method source site
  -> ParserParameterListProductV1
  -> one body parse result
       ParserNodeProductV1::Typed
       ParserNodeProductV1::CompatOnly
       ParserNodeProductV1::ParseError
  -> one unpublished method transaction
  -> later sole H3 declaration seal

compatibility ProgramJSON
  = one-way projection from the same result
```

The body product is source syntax authority only. It does not own resolver
bindings, FunctionOwner, body Facts, effects, Home Flow, Recipe, or MIR.

## Decisions to close

1. Which existing live block/statement/expression parser functions are the
   sole grammar owners, and how can they retain a `ParserNodeProductV1`
   alongside their compatibility projection without parsing twice?
2. What is the smallest direct-instance-method body cohort that proves a
   real `Typed` result rather than a fabricated empty shell?
3. Does the existing `source_carrier_v1` node vocabulary cover that cohort
   exactly? If not, which one minimal syntax vocabulary row is missing?
4. Where does the unpublished method transaction own method site, parameter
   product, and body disposition before H3 consumes it?
5. How do `Typed`, `CompatOnly`, and `ParseError` remain exhaustive without a
   silent JSON fallback or partial source seal?
6. Which focused guard proves that no source substring rescan, FuncScanner,
   StageB parser, or JSON decode enters the path?

## Acceptance for the Decision

The consultation closes only when it can state:

```text
This exact direct-method body is parsed once by this existing grammar owner,
produces this complete ParserNodeProductV1, and fails at this boundary before
any method/source-seal publication.
```

The accepted design must name the exact source authority and entry function,
typed body vocabulary and coverage, compatibility projection direction,
unpublished transaction owner, failure boundary, bounded next slice, negative
matrix, and file placement below 800 lines.

## Stop conditions

```text
FuncScannerBox or StageB parser becomes authority
saved method/body source is rescanned
ProgramJSON/MapBox is decoded into typed source truth
ParserNodeProductV1 is fabricated after parse
method body is looked up by name/arity/ordinal after parse
body Typed is defaulted from CompatOnly
second method/declaration sealer is introduced
parser_box.hako grows beyond its current facade boundary
unsupported body silently falls back
```

## Nonclaims

```text
implementation permission
ordinary Box parser connection
Take syntax
parameter/H3 integration
selected build-gate/static/interface/constructor bodies
resolver FunctionOwner/body Facts/conformance
Home capability/Flow
Recipe/CallSlot/Builder/MIR/runtime
production activation or fallback
```

## Stop

No code or fixture is authorized until the existing grammar owner and
same-pass typed result are proven. If the only path is rescan or JSON
reconstruction, record `NoSafeSlice` and select the missing parser substrate.
