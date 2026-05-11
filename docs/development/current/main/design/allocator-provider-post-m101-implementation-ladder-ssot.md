---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: post-M101 allocator provider activation implementation ladder.
Related:
  - docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-consumption-failfast-entry-ssot.md
  - docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md
  - docs/tools/check-scripts-index.md
  - src/runtime/allocator_provider_activation.rs
  - tools/checks/k2_wide_allocator_gate.sh
---

# Allocator Provider Post-M101 Implementation Ladder (SSOT)

## Goal

Prevent the allocator provider lane from returning to diagnostic-only rows after
M101. The next rows are small runtime implementation rows with strict stop
lines.

This ladder is optional future host-replacement support. It is not the default
implementation path for the current mimalloc port, which belongs in `.hako` /
`hako_alloc` unless host allocator replacement is explicitly reopened.

The behavior owner remains:

```text
src/runtime/allocator_provider_activation.rs
```

Diagnostic report owners remain diagnostic-only. CLI diagnostic surfaces must
not become behavior entries.

## Growth Control

Post-M101 rows must keep the implementation ladder small. These are stop-line
signals, not cleanup suggestions:

1. `tools/checks/k2_wide_allocator_gate.sh` keeps growing per row.
2. `src/runtime/allocator_provider_activation.rs` stops being an orchestration
   entry and starts owning detailed proof/rollback/gate logic.
3. Provider names, mimalloc names, hook names, or replacement names appear in
   `.inc` route/matcher logic.

When signal 1 appears, prefer one post-M101 allocator-provider guard that calls
focused unit tests and shared forbidden-pattern checks. Do not add a full new
allocator gate step by default.

M103 must not add a new per-row `k2_wide_allocator_provider_*` step to
`tools/checks/k2_wide_allocator_gate.sh`. If M103 needs a new guard, make it a
focused proof-validation guard and run it from a consolidated post-M101
allocator-provider guard or directly during the row proof bundle. The guard
should prove it is not individually registered in the wide allocator gate.

When signal 2 appears, keep the public entry in
`allocator_provider_activation.rs` and move internals into a narrower runtime
module. Candidate internal owners:

```text
src/runtime/allocator_provider_proof_validation.rs
src/runtime/allocator_provider_proof_consumption_token.rs
src/runtime/allocator_provider_rollback_preflight.rs
```

When signal 3 appears, stop immediately. Stage0 / `.inc` must not learn
allocator provider semantics.

## Runtime Ladder

| Row | Task | Output | Must Not Add |
| --- | --- | --- | --- |
| M102 | selected-provider precondition | caller-provided selected provider check against the requested provider/proof report | provider selection, proof consumption |
| M103 | proof validation for selected provider | validate selected provider proof operations/capability facts | proof consumption token, rollback, gate opening |
| M104 | proof bundle consumption token | in-memory token only when selected provider and proof validation pass | rollback execution, gate opening, hook install, replacement |
| M105 | rollback preparation fail-fast entry | block unless a proof token and explicit rollback facts are present | rollback execution, gate opening, hook install, replacement |

## Default Priority After M103

After M103, the default repository task is not M104. The default is to resume
`.hako` mimalloc / `hako_alloc` completeness work on top of the capability
substrate.

M104 remains the next concrete row only inside this optional provider ladder.
If it is resumed, its token is proof custody/readiness only and must keep
activation closed:

```text
activation_allowed=false
rollback_prepared=false
gate_open=false
hook_installed=false
process_allocator_replaced=false
```

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
coverage, but it must not create a consumption token. If the validation logic
needs more than a small helper, create a proof-validation internal module and
keep `allocator_provider_activation.rs` as orchestration only.

M103 implementation shape:

```text
src/runtime/allocator_provider_activation.rs
  public entry / report assembly only

src/runtime/allocator_provider_proof_validation.rs
  selected provider proof fact validation
  requested operation coverage checks
  capability/readiness checks
  focused unit tests
```

The validation module API must stay `pub(crate)` and must not select providers,
consume proofs, prepare rollback, open gates, install hooks, or replace the
process allocator.

M103 is landed when
`allocator_provider_selected_provider_proof_validation_attempt` returns
`ReadySelectedProviderProofValidated` only after the selected provider
precondition and proof operation coverage facts pass. It still returns
`proof_bundle_consumed=false`.

M104 is the first row in this optional ladder allowed to produce an in-memory
proof bundle consumption token. Even then, rollback, gate opening, hook
install, native activation, and replacement remain inactive.

M105 introduces rollback preparation as a fail-fast precondition. It does not
execute rollback and does not open the gate.

## Next Row

The next concrete row after M103 in this optional host-replacement ladder is:

```text
M104 ALLOCATOR-PROVIDER-PROOF-BUNDLE-CONSUMPTION-TOKEN
```
