---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: post-M101 allocator provider activation implementation ladder.
Related:
  - docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-consumption-failfast-entry-ssot.md
  - src/runtime/allocator_provider_activation.rs
---

# Allocator Provider Post-M101 Implementation Ladder (SSOT)

## Goal

Prevent the allocator provider lane from returning to diagnostic-only rows after
M101. The next rows are small runtime implementation rows with strict stop
lines.

The behavior owner remains:

```text
src/runtime/allocator_provider_activation.rs
```

Diagnostic report owners remain diagnostic-only. CLI diagnostic surfaces must
not become behavior entries.

## Runtime Ladder

| Row | Task | Output | Must Not Add |
| --- | --- | --- | --- |
| M102 | selected-provider precondition | caller-provided selected provider check against the requested provider/proof report | provider selection, proof consumption |
| M103 | proof validation for selected provider | validate selected provider proof operations/capability facts | proof consumption token, rollback, gate opening |
| M104 | proof bundle consumption token | in-memory token only when selected provider and proof validation pass | rollback execution, gate opening, hook install, replacement |
| M105 | rollback preparation fail-fast entry | block unless a proof token and explicit rollback facts are present | rollback execution, gate opening, hook install, replacement |

## Stop Line

Until the row explicitly changes one item, every row keeps these inactive:

- active provider registry construction;
- provider selection implementation;
- proof consumption for M102-M103;
- rollback preparation for M102-M104;
- rollback execution;
- activation gate opening;
- hook activation or native activation;
- environment toggles and implicit discovery;
- process allocator replacement;
- `#[global_allocator]`;
- `GlobalAlloc`;
- `.inc` hook/provider/facade/policy name matching.

## Row Contracts

M102 checks only a caller-provided selected provider. It does not choose a
provider. A missing selected provider remains blocked. A selected provider that
differs from `requested_provider_id` remains blocked. A matching selected
provider produces a ready precondition report but still keeps
`proof_bundle_consumed=false`.

M103 validates the proof facts for the selected provider. It may report proof
coverage, but it must not create a consumption token.

M104 is the first row allowed to produce an in-memory proof bundle consumption
token. Even then, rollback, gate opening, hook install, native activation, and
replacement remain inactive.

M105 introduces rollback preparation as a fail-fast precondition. It does not
execute rollback and does not open the gate.

## Next Row

The next concrete row after M101 is:

```text
M102 ALLOCATOR-PROVIDER-SELECTED-PROVIDER-PRECONDITION
```
