Status: Done
Date: 2026-06-17
Scope: add report-only alias/publication inputs for the recursive fastpath
resolver.
Related:
  - docs/development/current/main/design/fastpath-eligibility-resolver-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1036-FASTPATH-DENY-OWNER-MAPPING-001.md

# FASTPATH-ALIAS-PUBLICATION-MVP-001

## Purpose

Provide the minimal resolver inputs needed before shadowing
`KnownReceiverDirectCall`.

This row does not execute the resolver and does not let the backend consume new
facts. It only proves that a linear alias chain can feed existing publication
inventory vocabulary.

## Change

Added:

```text
LocalAliasLink
linear_alias_chain_observations(...)
```

The helper produces passive `LocalAliasClassObservation` rows for a root value
and each link destination. It does not run a heap graph, field-sensitive
points-to analysis, or interprocedural analysis.

## Fixture

Added a 5-hop alias-chain unit fixture:

```text
v1 --SsaCopy--> v2
v2 --SimpleReceiverAlias--> v3
v3 --Phi--> v4
v4 --Select--> v5
v5 --SsaCopy--> v6
```

All values receive the same alias class, and the final value can feed
`LocalPublicationInventoryRow` with `PublicationState::Unpublished`.

## Validation

```text
cargo test -q five_hop_alias_chain_feeds_publication_inventory_without_backend_consumption --lib
cargo test -q object_storage_plan --lib
```

Both passed.

## Contract

```text
output_contract=fastpath-alias-publication-mvp-v0
local_alias_class_mvp_linear_chain_supported=1
local_alias_class_mvp_five_hop_fixture=1
local_publication_inventory_report_only=1
resolver_execution_enabled=0
backend_behavior_changed=0
heap_graph_enabled=0
field_sensitive_points_to_enabled=0
interprocedural_fixedpoint_enabled=0
next_task=FASTPATH-KNOWN-RECEIVER-DIRECT-CALL-SHADOW-001
summary=ok
```

## Stop Lines

```text
do not build a general escape-analysis engine
do not make alias observations backend-consumable
do not make publication inventory backend-consumable
do not execute the recursive resolver in this row
do not expand beyond linear MVP alias inputs
```
