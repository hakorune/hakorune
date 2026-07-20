---
Status: Design consultation stop
Date: 2026-07-20
Scope: classify finalization passes before retiring repair/inference
Parent: docs/development/current/main/investigations/mirbuilder-clean-architecture-consolidation-task-2026-07-19.md
---

# FINALIZE0-CENSUS0: finalization responsibility inventory

## Decision boundary

The current finalization pipeline is not yet safe to cut over. It combines
completed-draft verification, type propagation, call/await type annotation,
PHI return inference, PHI input rematerialization, metadata snapshots, and
derived module refresh. This row records the inventory only; it does not
delete a repair pass or move a producer.

The next code-facing row is not `FINALIZE0-CUT0`. A design decision must first
classify every pass as exactly one of:

```text
VerifyCompletedDraft
NormalizeRepresentation
PublishDerivedArtifact
RepairMissingLoweringFact
LegacySemanticInference
```

The last two classes must be explicit, even when their retirement is parked.

## Observed pass inventory

| Site | Current operation | Initial classification | Decision needed |
| --- | --- | --- | --- |
| `finalize_function_draft` | `TypePropagationPipeline::run` | RepairMissingLoweringFact? | confirm all required facts are producer-owned before deletion |
| `finalize_function_draft` | call/await result annotation | RepairMissingLoweringFact? | separate already-sealed result facts from fallback annotation |
| `finalize_function_draft` | return-type scan/inference | LegacySemanticInference? | retain only sealed return contract projection |
| `finalize_module` | `TypePropagationPipeline::run` | RepairMissingLoweringFact? | prove main/module route parity |
| `finalize_module` | PHI return inference | LegacySemanticInference? | identify exact producer or park separately |
| `finalize_module` | PHI input materialization | VerifyCompletedDraft or repair | distinguish edge validation from rematerialization repair |
| `finalize_module` | metadata `value_types` snapshot | PublishDerivedArtifact | preserve one-way snapshot timing |
| `finalize_module` | NewBox/birth dev scan | VerifyCompletedDraft | diagnostics only; no semantic authority |
| module finalization | record/packed layout refresh | PublishDerivedArtifact | retain only after sealed declarations |
| module finalization | typed-object/direct-state refresh | PublishDerivedArtifact | no lowering fact backfeed |
| all-function close | PHI input materialization | VerifyCompletedDraft or repair | prove no hidden source/type inference |

## Authority rules

Finalization may read and publish already-sealed facts, verify a completed
draft, normalize representation-preserving structure, and build derived
module artifacts. It may not be the first owner of a type, origin, call
disposition, field owner, source identity, or route decision needed earlier by
lowering.

Non-authorities for finalization repair:

```text
emitted MIR spelling/order
finalized metadata used as lowering fallback
runtime class tags
method or field names
environment flags read after session entry
retry/fallback routes
```

## Required census evidence

### `FINALIZE0-CENSUS0-P0` — in progress

The first read-only inventory artifact is
`tools/checks/fixtures/mirbuilder_finalize0_pass_inventory_v1.json`. Its
validator checks 45 explicit pass rows and source anchors, including the
post-module compiler finish schedule and contract-refresh children. It deliberately
records `RepairMissingLoweringFact` and `LegacySemanticInference` as parked
classes rather than silently treating them as verification. No Rust producer,
finalization call, or runtime behavior is changed by this artifact.

Before CUT0, the inventory must record for each call site:

```text
pass owner and invocation count
input fact authority
first publication site for every output fact
whether mutation is representation-preserving
whether failure is typed and pre-publication
downstream consumers that require the result during lowering
```

The census must include both `finalize_function_draft` and `finalize_module`
and the post-module semantic refresh calls. It must not infer ownership from
function names alone.

## Stop conditions

Stop and open a follow-up design row if any pass needs to remain a first
publisher, if MIR scanning is required to recover source semantics, if a
finalization pass must rerun lowering propagation to hide timing drift, or if
removing a pass changes FieldGet/Call/PHI behavior. Do not combine this census
with PHI, Call, FieldGet, Unknown, or metadata-isolation changes.

## Decision lock

> `FINALIZE0-CENSUS0` is a read-only responsibility inventory. It must classify
> every finalization operation and identify the true lowering-time producer
> before any repair or MIR-to-source inference is retired. `FINALIZE0-CUT0`
> remains forbidden until the census and parity matrix are green.
