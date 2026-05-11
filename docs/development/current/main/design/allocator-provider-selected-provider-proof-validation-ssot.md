---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: M103 allocator provider selected-provider proof validation runtime row.
Related:
  - docs/development/current/main/design/allocator-provider-post-m101-implementation-ladder-ssot.md
  - docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md
  - docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md
  - src/runtime/allocator_provider_activation.rs
  - src/runtime/allocator_provider_proof_validation.rs
---

# Allocator Provider Selected-Provider Proof Validation (SSOT)

## Goal

M103 moves one step past the M102 selected-provider precondition without
creating a proof consumption token. The activation owner may now validate that a
caller-provided selected provider has proof coverage for the requested provider
report.

## Owner Split

```text
src/runtime/allocator_provider_activation.rs
  public orchestration entry
  attempt report assembly
  inactive action output

src/runtime/allocator_provider_proof_validation.rs
  selected provider proof facts
  requested operation coverage facts
  focused unit tests
```

The proof validation module is `pub(crate)`. It must not become a provider
selection, proof consumption, rollback, gate, hook, native activation, or
process allocator replacement owner.

## Runtime Entry

```text
allocator_provider_selected_provider_proof_validation_attempt(
  proof_bundle_report,
  selected_provider_id,
) -> AllocatorProviderProofBundleConsumptionAttemptReport
```

The entry first reuses the M102 caller-provided selected-provider precondition.
If that precondition is blocked, M103 returns the blocked M102 report. If the
precondition is ready, M103 validates the selected-provider proof facts.

## Status Contract

```text
BlockedSelectedProviderProofMissing
  selected provider has no proof facts

BlockedSelectedProviderProofIncomplete
  selected provider proof facts do not cover the requested operations

ReadySelectedProviderProofValidated
  selected provider proof facts cover the requested operations
  proof_bundle_consumed=false
```

Stable diagnostics:

```text
[allocator-provider/proof-bundle-consumption-selected-provider-proof-missing]
[allocator-provider/proof-bundle-consumption-selected-provider-proof-incomplete]
[allocator-provider/proof-bundle-consumption-selected-provider-proof-ready]
```

## Stop Line

M103 must keep all activation behavior inactive:

- no provider selection;
- no proof consumption token;
- no rollback preparation;
- no activation gate opening;
- no hook install or native activation;
- no process allocator replacement;
- no new environment toggle or implicit discovery;
- no `.inc` provider or hook matcher.

## Guard Policy

M103 uses a focused row guard:

```text
tools/checks/k2_wide_allocator_provider_proof_validation_guard.sh
```

That guard is intentionally not registered as another per-row step in
`tools/checks/k2_wide_allocator_gate.sh`. It must prove that absence so the
post-M101 allocator gate does not keep growing per row.

## Next Row

M104 may create an in-memory proof bundle consumption token inside the optional
host-replacement ladder, but only after the selected-provider precondition and
M103 proof validation both pass. It is not the default next implementation task
for the current mimalloc port. M104 still must not prepare rollback, open the
gate, install a hook, activate native allocator behavior, or replace the
process allocator.
