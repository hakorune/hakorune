# 293x-411 DOCS-SLIM-004 Activation Closeout Resolver

Status: landed
Date: 2026-05-15

## Decision

Adopt the phase-card resolver helper in one high-value closeout cluster.

`DOCS-SLIM-003` added the resolver helper but intentionally did not mass-edit
the 270 direct card-reference guard files. This row converts the allocator
provider activation closeout family only, removing direct phase-card paths from
three scripts while leaving physical card moves closed.

## TODO

- [x] Convert activation safety closeout card references to
  `guard_require_phase293x_card`.
- [x] Convert activation decision closeout card references to
  `guard_require_phase293x_card`.
- [x] Convert activation diagnostic closeout card references to
  `guard_require_phase293x_card`.
- [x] Add a guard proving the converted scripts no longer contain direct
  phase-293x card paths.
- [x] Keep production allocator port closeout for a later row.

## Scope

- Guard path cleanup only.
- Allocator-provider activation closeout cluster only.
- No card movement.

## Stop Lines

- Do not move numbered cards in this row.
- Do not convert unrelated direct-reference guards in this row.
- Do not change phase README or taskboard assertions in this row.
- Do not wire the resolver helper into `dev_gate.sh` or allocator-wide.
- Do not include production allocator port closeout in this row.

## Required Evidence

```text
bash tools/checks/docs_slim_004_activation_closeout_resolver_guard.sh
bash tools/checks/docs_slim_003_guard_reference_decoupling_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

- Converted `k2_wide_allocator_provider_activation_safety_closeout_guard.sh`.
- Converted `k2_wide_allocator_provider_activation_decision_closeout_guard.sh`.
- Converted `k2_wide_allocator_provider_activation_diagnostic_closeout_guard.sh`.
- Added `docs_slim_004_activation_closeout_resolver_guard.sh`.

## Evidence

```text
bash tools/checks/docs_slim_004_activation_closeout_resolver_guard.sh
bash tools/checks/docs_slim_003_guard_reference_decoupling_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
