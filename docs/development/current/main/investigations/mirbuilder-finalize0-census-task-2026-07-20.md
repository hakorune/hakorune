---
Status: SCHEMA0 closed; P0 production-census design stop
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

### `FINALIZE0-CENSUS0-SCHEMA0` — closed

The first read-only inventory artifact is
`tools/checks/fixtures/mirbuilder_finalize0_pass_inventory_v2.json`. Its
validator records 66 semantic child-operation rows, 68 explicit source-site
anchors, and 93 declared production invocations. The measured domain split is:

```text
semantic_pass = 49
lifecycle_transition = 14
diagnostic_observation = 3
canonical_repair_reachable = 33
```

The validator rejects an unclassified semantic operation, a semantic class on
a lifecycle/diagnostic operation, an invalid publication kind, a missing
route/generation/atomicity/retirement field, a duplicate source identity, a
missing source occurrence, and an uncovered occurrence of any registered
operation anchor. It deliberately records `RepairMissingLoweringFact` and
`LegacySemanticInference` as parked classes rather than silently treating them
as verification. No Rust producer, finalization call, or runtime behavior is
changed by this artifact.

Schema v2 records for each operation:

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

The validator proves both directions for every registered operation anchor:

```text
every production source match -> exactly one inventory row
every inventory source site -> exactly one production source match
```

The census includes both `finalize_function_draft` and `finalize_module`, the
post-module compiler schedule, semantic-refresh stages, and contract-refresh
children. It does not infer ownership from function names alone.

Origin surfaces are explicit rather than grouped under one ambiguous
"type/origin snapshot" label:

```text
semantic lowering origin = value_origin_newbox
diagnostic origin = value_origin_callers
post-Builder semantic origin publication = none
```

Focused acceptance:

```text
python3 tools/checks/lib/mirbuilder_finalize0_pass_inventory.py
bash tools/checks/run_row_guard.sh --only mirbuilder-finalize0-pass-inventory
```

Both commands are green with `behavior_delta=0` and
`production_connections=0`.

### `FINALIZE0-CENSUS0-P0` — design stop

P0 now consumes schema v2 and proves the repository-wide production call-site
and route-reachability counts. In particular, it must replace manually entered
`production_invocation_count` and `canonical_repair_reachable` assertions with
measured source/call-graph evidence. It must also prove that every production
finalization/refresh child belongs to one inventory row, rather than only
proving the reverse coverage of already registered operation anchors.

The first independent P0 audit found that the three remaining fields do not
yet share one measurable meaning. Inline operation counts, facade direct-call
counts, and boundary ingress counts are mixed. It also found that authority
routes and execution boundaries occupy one array, while static code
reachability is being asked to stand in for actual repair mutation.

Implementation is paused at
[`mirbuilder-finalize0-p0-production-census-consultation-2026-07-20.md`](mirbuilder-finalize0-p0-production-census-consultation-2026-07-20.md).
No scanner, route fixture, counter, or schema correction is authorized until
that decision selects the count law and proof architecture.

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

SCHEMA0 is closed. The sole next code-facing row is:

```text
FINALIZE0-CENSUS0-P0
```

It upgrades invocation/reachability claims from explicit inventory assertions
to measured repository evidence. It makes no compiler behavior change. No
second docs-only FINALIZE0 row is permitted.

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
