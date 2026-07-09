---
Status: SSOT
Date: 2026-07-10
Scope: MirBuilder-only Rust-to-Hako converter current task order.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/design/mirbuilder-authority-based-hako-migration-ssot.md
  - docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md
  - docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-active-v1.json
  - docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-history-v1.jsonl
---

# MirBuilder Rust-to-Hako Converter Task Order

This is a current-only restart entry. It must not accumulate landed cards,
inventories, transcripts, or historical next chains.

## Current Blocker

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-POST-COLLECTION-CALLER-ORIENTATION-PILOT-DESIGN-CONSULTATION-001
```

The active token, latest card, and phase pointer are authoritative only in:

```text
docs/development/current/main/CURRENT_STATE.toml
```

## Current Evidence

```text
MapLoad caller-orientation authority pilot = complete
String caller-orientation authority pilot = complete
Collection caller-orientation authority pilot = complete

caller input = PolicyRowIdOnly
caller return = Unit
Rust oracle / compatibility veto = retained
mismatch = fail-fast
```

Collection stays an explicitly enumerated four-row boundary. Its receiver
domains remain generated policy metadata, not caller inputs. `AnyLength -> Box`
is one explicit row, not wildcard or global Box authority.

## Consultation Frontier

Do not implement the next authority slice until 3452 selects one mutation-
bearing non-Delete Write surface.

```text
A: MapStoreI64-only caller contract authority pilot
B: ArrayAppendAny-only caller contract authority pilot
C: MapStoreAny-only Any-write pilot
D: full non-Delete Write three-row authority island
E: park caller orientation and return to Source Selfhost route selection
```

The decision must define source authority, non-authority, exhaustive scope,
mutation versus metadata boundary, fail-fast behavior, fallback prohibition,
and promotion conditions.

## Invariants

```text
caller_orientation_runtime_path = 0
hako_runtime_route_authority = 0
scalar_known_hako_runtime_route_authority = 0
route_selection_authority_switch = 0
backend_lowering_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
delete_hako_route_decision_authority_pilot = 0
scalar_known_wide_authority = 0
runtime_fallback = 0
source_selfhost_claim = 0
```

Do not infer authority from row count, route count, owner name, source path,
surface membership, or coverage percentage.

## Historical Pointers

Use these sources for landed history rather than restoring it here:

```text
phase cards:
  docs/development/current/main/phases/phase-296x/

family history ledger:
  docs/development/current/main/design/fixtures/rust-lifecycle/
  source-selfhost-family-guard-history-v1.jsonl

frozen compatibility snapshot:
  docs/development/current/main/design/fixtures/rust-lifecycle/
  source-selfhost-family-guard-manifest-v0.json

durable policy and migration maps:
  docs/development/current/main/design/
  mirbuilder-authority-based-hako-migration-ssot.md
  mirbuilder-selfhost-checkpoint-roadmap-ssot.md

exact old task-order text:
  git log -- docs/development/current/main/design/
  mirbuilder-rust-to-hako-converter-task-order-ssot.md
```

## Maintenance Contract

Keep this document below 400 lines and each line at or below 500 characters.
When a change needs detailed evidence, add it to the active card, a fixture,
the family history JSONL, an investigation, or git history. Do not create a
numbered card for inventory-only bookkeeping.

next_documented_task = MIRBUILDER-SCALAR-KNOWN-FASTPATH-POST-COLLECTION-CALLER-ORIENTATION-PILOT-DESIGN-CONSULTATION-001
