# 293x-1004 MIMAP-382A Provider Call Capability Gate Inventory

Status: selected current
Date: 2026-05-21

## Purpose

Record the provider-call capability gate required after the provider activation
modeled-open pilot. Activation is open only in model space; this row prepares
the next boundary by proving which preconditions must be present before any
provider API call can be attempted.

## Scope

- Add a narrow provider-call capability gate inventory owner.
- Consume the MIMAP-380A modeled-open report shape.
- Record whether the modeled activation is open, provider-call capability is
  present, and provider-call execution remains inactive.
- Publish reason/counter fields for missing modeled-open evidence, inactive
  modeled activation, missing call capability, and closed execution seams.

## Stop Lines

- No provider API calls.
- No host allocator replacement, hooks, or `#[global_allocator]`.
- No hidden env, implicit discovery, or process-global activation config.
- No backend `.inc` matcher by app, box, owner, or row name.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

Daily validation is L2:

```text
VM proof
MIR JSON emit
route preflight
```

L3/L4 evidence is deferred until a provider-call gate closeout or the first row
that actually opens a provider-call execution seam.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_provider_call_capability_gate_inventory_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
