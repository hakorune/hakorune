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

## Parked Front

```text
status = parked_by_mirbuilder_inplace_replacement
resume = MIRBUILDER-MAPSTORE-ROUTE-POLICY-KEY-VALUE-DOMAIN-BOXSHAPE-001
resume_after = explicit CURRENT_STATE.toml lane selection
```

This `.hako` converter order is independent from the active Rust MirBuilder
in-place replacement. Do not translate or widen a Rust responsibility while
its production owner is being exchanged.

The active token, latest card, and phase pointer are authoritative only in:

```text
docs/development/current/main/CURRENT_STATE.toml
```

## Current Evidence

```text
MapLoad caller-orientation authority pilot = complete
String caller-orientation authority pilot = complete
Collection caller-orientation authority pilot = complete
MapStoreI64 caller-orientation pilot = selected, implementation pending

caller input = PolicyRowIdOnly
caller return = Unit
Rust oracle / compatibility veto = retained
mismatch = fail-fast
```

Collection stays an explicitly enumerated four-row boundary. Its receiver
domains remain generated policy metadata, not caller inputs. `AnyLength -> Box`
is one explicit row, not wildcard or global Box authority.

## Parked BoxShape Prerequisite

The 3454 selection merged key and stored-value domains. The corrected policy is:

```text
MapStoreI64: key_domain = I64, stored_value_domain = Any
MapStoreAny: key_domain = Any, stored_value_domain = Any
```

When resumed, 3456 is one BoxShape-only task: introduce a typed
`RoutePolicyRow` SSOT,
generate the decision payload and caller projection from one Hako-owned row,
retain independent Rust route matching/oracle behavior, centralize caller and
shadow validation, and test key/value axes independently.

After 3456 is green, resume 3454. After a green 3454 fixture-backed rerun, enter
3455, park caller orientation, and return to focused Fact/Plan/Boundary
inventory for the smallest Fact-owner or REGISTRY-rule hard-authority slice.

## Authority Owners

```text
route matching = Rust write_routes.rs
policy row edit source = hand-authored Hako
decision payload = Rust artifact generated from Hako
compatibility veto = independent Rust validator / oracle
mutation and backend = downstream Rust
caller orientation = policy-row contract acceptance or rejection only
```

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

parked_resume_task = MIRBUILDER-MAPSTORE-ROUTE-POLICY-KEY-VALUE-DOMAIN-BOXSHAPE-001
