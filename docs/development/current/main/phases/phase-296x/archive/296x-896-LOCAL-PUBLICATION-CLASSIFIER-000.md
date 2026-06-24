# 296x-896 LOCAL-PUBLICATION-CLASSIFIER-000

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-publication-classifier-v0
source_evidence=296x-895
row_kind=passive_vocabulary

publication_state_vocabulary_defined=1
publication_state_unpublished_fastpath_allowed=1
publication_state_published_fastpath_allowed=0
publication_state_maybe_published_fastpath_allowed=0
local_fastpath_fallback_reason_vocabulary_defined=1
local_fastpath_fact_vocabulary_defined=1
local_fastpath_fact_backend_consumable=1
fallback_evidence_backend_consumable=0
fallback_fact_enabled=0

full_escape_engine_required_for_v0=0
interprocedural_fixedpoint_required_for_v0=0
object_storage_plan_execution_enabled=0
object_plan_execution_enabled=0
backend_new_lowering_enabled=0
next_task=LOCAL-ALIAS-CLASS-MVP-001
summary=ok
```

## Implementation

`src/object_storage_plan.rs` now contains passive local-first fast-path
vocabulary:

```text
PublicationState:
  Unpublished
  Published
  MaybePublished

LocalFastPathFallbackReason:
  OpenWorld
  AliasUnknown
  PublishedBeforeSite
  MaybePublishedBeforeSite
  DynamicRoute
  GenericStorage
  BackendMissing
  UnknownCall

LocalFastPathFact:
  backend-consumable positive fast-path permission
```

No lowering or backend execution is enabled by this row.

## Decision

`Unpublished` is the only publication state that permits a local fast path.
`Published` and `MaybePublished` produce fallback reasons. Unknown analysis
must use fallback instead of manufacturing a backend-readable fact.

## Tests

```bash
cargo test --lib object_storage_plan -- --nocapture
```

## Stop Lines

- no backend lowering enablement
- no full escape engine
- no interprocedural fixed-point
- no fallback facts
- no HostHandle bypass
- no direct storage and direct call combined pilot
- no MIRBuilder representation ownership
