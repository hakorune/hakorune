---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M55 allocator hook activation proof vocabulary
---

# 293x-107 M55 Allocator Hook Activation Proof

## Decision

`M55 allocator hook activation proof` is live-docs.

M55 adds a reserved activation proof SSOT and TOML fixture. It does not activate
runtime hooks or process allocator replacement.

Design owners:

```text
docs/development/current/main/design/allocator-hook-activation-proof-ssot.md
docs/development/current/main/design/allocator-hook-activation-proof-v0.toml
```

## Owned

- Reserved activation proof vocabulary.
- Reserved TOML fixture.
- Coverage guard:
  `tools/checks/k2_wide_allocator_hook_activation_proof_guard.sh`
- docs/taskboard/current pointers for M55.

## Not Owned

- Runtime hook install/uninstall implementation.
- Process allocator replacement.
- `#[global_allocator]`.
- Hook environment variables.
- `.inc` hook/facade/policy name matching.
- Pointer `fetch_add`.
- OSVM unreserve/release.
- Native pointer attr widening.

## Gate

```bash
bash tools/checks/k2_wide_allocator_hook_activation_proof_guard.sh
bash tools/checks/k2_wide_allocator_hook_runtime_dry_run_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The guard verifies:

- activation proof SSOT/card/TOML/taskboard/current/docs index/dev_gate/group
  wiring is synchronized;
- proof fixture is reserved-only and inactive;
- M54 guard remains forward-compatible after latest-card movement;
- no runtime hook code, env toggle, process allocator replacement, or `.inc`
  name matching exists.

## Result

Result on 2026-05-10:
`k2_wide_allocator_hook_activation_proof_guard.sh` passes.
