Status: Design stop — NamedConsumerMissing
Date: 2026-08-23
Decision: NORMAL-MAIN-APP-CONSUMER-D0
ParentCurrentCard: docs/development/current/main/investigations/normal-main-app-entry-transport-d0-2026-08-23.md
PrerequisiteExecutionRow: NORMAL-GENERAL-PROGRAM-PARSER-MAIN-APP-ENTRY-TRANSPORT-I0
ProductionCaller: 0; no Main/App consumer is selected
ProductionEdit: none; read-only authority census only
CeremonyTier: D0 — named consumer design
---

# NORMAL-MAIN-APP-CONSUMER-D0

CurrentExecutionRow: NORMAL-MAIN-APP-CONSUMER-D0

## Six-line brief

```text
Decision:
  keep the lane at design_stop until one existing root owner is proven able
  to consume the parser-issued Main/App disposition without re-observation.
Source authority + canonical issuer:
  issue_parser_main_app_entry_v1 is the sole source issuer; the canonical
  Main/App consumer issuer is not selected yet and must not be invented.
Non-authority:
  AST/name/ordinal rescans, root_is_app_mode, NormalCompileRequest, Builder
  state, raw root expansion, compatibility routes, and parser transport owners.
Fail-fast boundary:
  every non-ready parser disposition remains a typed no-effect terminal; a
  Ready disposition cannot enter root/Builder work until a named consumer is
  accepted in this card or a successor card.
Smallest next slice:
  perform a read-only top-down census of existing root admission, final source,
  and compile-request owners, then select one existing consumer or record
  NoSafeSlice; do not add code, fixtures, receipts, or fallback.
Non-claims:
  App semantic meaning, root selection, ABI/result validation, Builder/MIR
  effects, NormalCompileRequest changes, compatibility policy, production
  switch, old-route retirement, and performance.
```

## Current boundary

The parser/source transport I0 is closed. One parser invocation issues the
non-`Clone` `ParserMainAppEntryDispositionV1`, and the existing source-backed
Prepared, VerifiedFinal, and retained products carry it by move. That transport
does not choose a root or authorize Builder work.

The next design question is therefore not how to re-observe Main/App syntax.
It is whether an existing root owner already has a named, source-backed
consumer boundary that can consume the disposition exactly once. A raw
`bool`, `root_is_app_mode`, a request constructor, or a compatibility route is
not sufficient evidence of that consumer.

## Authority map

| Owner / candidate | May own | Must not be treated as |
| --- | --- | --- |
| `issue_parser_main_app_entry_v1` | one parser Main/App disposition | semantic root selection or ABI |
| source-backed callable products | move-only transport of that disposition | a second Main/App issuer |
| existing root admission owners | candidate consumer only after census | authority merely because they hold a `bool` |
| `NormalCompileRequestV1` | future transport if explicitly selected | Main/App semantic issuer |
| Builder/raw root state | physical realization after admission | parser/source authority |
| compatibility route | explicit separate lane | synthetic Main/App fallback |
| future named Main/App consumer | one exact consume and terminal mapping | AST re-observation or retry |

The future consumer issuer is intentionally unassigned. Until its owner and
input relation are proven, this row remains `NamedConsumerMissing` rather than
inventing a `Verified*` or `Prepared*` semantic product.

## Finite state table

| State | Issuer / owner | Pre-effect behavior | Allowed terminal | Fallback |
| --- | --- | --- | --- | --- |
| `AppMainReady` | parser issuer; consumer unselected | remain transport-only | design stop | none |
| `Outside(reason)` | parser issuer | no root/Builder effect | typed outside terminal | none |
| `SourceAuthorityUnavailable(reason)` | parser issuer | no effect | typed unavailable terminal | none |
| `Incomplete(reason)` | parser issuer | no effect | typed incomplete terminal | none |
| `IntegrityInvalid(reason)` | parser issuer | no effect | typed integrity terminal | none |
| `Compatibility` | existing compatibility owner | preserve separate route | compatibility terminal | no synthetic Main/App state |
| `NamedConsumerMissing` | design process / SSOT | no implementation | NoSafeSlice/design stop | none |

No `Option::None`, empty row, default bool, or compatibility label may merge
`AppMainReady` with any non-ready state. `NamedConsumerMissing` is a
development state, not a runtime disposition.

## Bounded census task

The only authorized next work is a read-only audit of the existing chain:

```text
VerifiedFinalCallableProgramSourceV1
  -> existing final-source/root admission owner candidates
  -> NormalCompileRequestV1 transport boundary
  -> ModuleBuilderInvocationSessionV1 / root admission
```

For each candidate, record:

```text
owner and constructor
input relation to the parser disposition
whether it can consume by move exactly once
whether it can reject non-ready states before effects
whether it re-observes AST/name/ordinal or uses raw bool state
whether a legacy fallback/retry edge exists
```

The census may select one existing owner for a successor I0. If no existing
owner satisfies all rows, record `NoSafeSlice` and design the missing consumer
authority; do not add an adapter or a guessed receipt.

## Acceptance / stop conditions

This D0 is complete only when one of these is true:

```text
SafeSlice:
  exactly one named existing consumer, exact move relation, exhaustive typed
  terminal mapping, no second issuer, no re-observation, and no fallback.

NoSafeSlice:
  the existing root/request/Builder owners cannot consume the parser product
  without a new authority, a raw-bool reconstruction, or an effectful repair.
```

Until then:

```text
new Main/App semantic Verified*/Prepared* product = 0
root_is_app_mode production write = 0
NormalCompileRequest field/constructor change = 0
Builder/MIR effect = 0
fallback/retry/reselection = 0
production caller = 0
```

