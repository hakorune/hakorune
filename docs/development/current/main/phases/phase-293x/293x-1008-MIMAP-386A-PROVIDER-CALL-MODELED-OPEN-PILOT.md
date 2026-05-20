# 293x-1008 MIMAP-386A Provider Call Modeled Open Pilot

Status: landed
Date: 2026-05-21

## Purpose

Open provider-call readiness in model space after the explicit provider-call
capability gate and unsupported dry-run outcome rows. This row records
provider-call-open evidence without executing provider APIs or process allocator
replacement.

## Scope

- Add `provider_call_modeled_open_pilot_box.hako`.
- Consume `HakoAllocProviderCallDryRunUnsupportedBehaviorReport`.
- Accept only explicit, accepted provider-call dry-run unsupported outcomes.
- Record:
  - `provider_call_modeled_open = 1`
  - `provider_call_model_active = 1`
  - `provider_call_inactive = 0`
  - `would_call_provider = 1`
- Keep provider API calls, host replacement, hooks, backend matcher additions,
  worker/thread execution, and global allocator install closed.

## Stop Lines

- No provider API calls.
- No hidden env, implicit discovery, or process-global activation config.
- No host allocator replacement, hooks, or `#[global_allocator]`.
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

L3/L4 evidence is deferred until a provider-call modeled-open closeout or the
first row that actually opens provider API call execution.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_provider_call_modeled_open_pilot_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed. MIMAP-387A is selected as the next row-selection card.
