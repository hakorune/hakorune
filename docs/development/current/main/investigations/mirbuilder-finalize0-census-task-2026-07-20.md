---
Status: Implementation task selected
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

Candidate A-prime is accepted in the linked boundary decision. The next
code-facing row is `FINALIZE0-CENSUS0-SCHEMA0`, not `FINALIZE0-CUT0`.

Semantic operations classify as exactly one of:

```text
VerifyNormalizationPreconditions
NormalizeRepresentation
PublishSealedArtifact
VerifyPublishedDraft
RepairMissingLoweringFact
LegacySemanticInference
```

The last two classes must be explicit, even when their retirement is parked.
Lifecycle transitions and diagnostic observations are separate operation
domains and do not receive a semantic class.

The completed source census found that the v1 five-class vocabulary cannot be
applied to facade function names: several current facades contain two or more
semantic operations. The consultation and recommended decomposition are now
fixed in
[`mirbuilder-finalize0-boundary-consultation-2026-07-20.md`](mirbuilder-finalize0-boundary-consultation-2026-07-20.md).

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
validator checks 51 explicit pass rows and source anchors, including the
post-module compiler finish schedule, semantic-refresh stages, and
contract-refresh children. It deliberately
records `RepairMissingLoweringFact` and `LegacySemanticInference` as parked
classes rather than silently treating them as verification. No Rust producer,
finalization call, or runtime behavior is changed by this artifact.

Schema v2 must record for each operation:

```text
operation_domain
semantic_class when domain=semantic_pass
pass owner and production invocation count
route/profile reachability
source sites: path, enclosing symbol, operation, ordinal, cfg domain
input fact authority
outputs and publication_kind
first publication site
mutation and identity-stability law
invalidated artifacts
session generation
failure timing and atomicity
lowering/downstream consumers
disposition and retirement owner/dependency
```

The validator must prove both directions:

```text
every production source match -> exactly one inventory row
every inventory source site -> exactly one production source match
```

The census must include both `finalize_function_draft` and `finalize_module`
and the post-module semantic refresh calls. It must not infer ownership from
function names alone.

### Census findings — design stop confirmed

Three independent read-only audits found that the current v1 artifact is not
a completion proof.

```text
missing operations:
  function.metadata_type_snapshot
  module.call_await_annotation
  module.metadata_origin_snapshot

TypePropagationPipeline production sites:
  3

materialize_all_phi_inputs production sites:
  3
```

The current validator checks row count, enums, and anchor substrings only. Its
`production_consumers=0` output is a fixed literal. It does not yet validate
inputs, outputs, first publication, invocation count, mutation, failure
atomicity, consumers, or retirement dependencies.

The following entries are composite and must be split before they can receive
one exact classification:

```text
verify_typed_values_are_defined
annotate_missing_result_types_from_calls_and_await
materialize_all_phi_inputs
optimizer
contract refresh
semantic refresh
callsite canonicalization
extern route refresh
```

`materialize_all_phi_inputs` is a legacy repair rather than verification. It
can delete PHIs, add missing rows, allocate Values, rematerialize instructions,
and rewrite inputs before all later failures are known. Metadata snapshots can
therefore occur before the final structural shape exists.

The external decision selected producer-first pure finalization with temporary
repair quarantine and five clarifications: post-publication final verification,
lifecycle/diagnostic domains, fresh fact generations, all-exit return sealing,
and identity/freshness-preserving normalization.

The sole next code-facing row is:

```text
FINALIZE0-CENSUS0-SCHEMA0
```

It upgrades the machine-readable inventory and validator. It makes no compiler
behavior change and is the required executable/artifact step after this
decision. No second docs-only FINALIZE0 row is permitted.

Required SCHEMA0 output includes explicit rows for semantic lowering origin
`value_origin_newbox`, diagnostic origin `value_origin_callers`, and an exact
post-Builder semantic-origin field or explicit `none`. A no-consumer origin
write is a retirement candidate, not an automatic producer-migration target.

The next implementation sequence after schema/parity is:

```text
VERIFY-SPLIT0
-> FACTSESSION0
-> TYPEPIPE-SPLIT0 / CALLAWAIT-SPLIT0 / PHI-SPLIT0
-> FIELD / CALLAWAIT / COPY / BINOP / PHI producer closures
-> RETURN0 all-exit contract
-> DERIVED0 publication/freshness verification
-> CONDITIONFN-RET0
-> family CUT0
-> Builder-local FINALIZE0-G0
```

## Stop conditions

Stop the active implementation row if any pass needs to remain a first
publisher, if MIR scanning is required to recover source semantics, if a
finalization pass must rerun lowering propagation to hide timing drift, or if
removing a pass changes FieldGet/Call/PHI behavior. These conditions are now
observed, so no CUT0 implementation is authorized. Do not combine this census
with PHI, Call, FieldGet, Unknown, or metadata-isolation changes.

## Decision lock

> `FINALIZE0-CENSUS0-SCHEMA0` upgrades the read-only inventory to operation-
> domain-aware schema v2 with bidirectional production-site coverage, route
> reachability, publication/freshness, generation, atomicity, consumers, and
> retirement ownership. It changes no compiler behavior. `FINALIZE0-CUT0`
> remains forbidden until schema v2, its parity guard, producer closures, and
> the all-exit return/freshness proofs are green.
