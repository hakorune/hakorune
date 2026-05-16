# 293x-533 RANDOM-CAP-002 Random Unsupported Route Preflight

Status: selected current
Date: 2026-05-17

## Decision

`RANDOM-CAP-002` is the Hakorune core diagnostics row selected by
`RANDOM-CAP-001`.

It must make unsupported random/entropy execution fail before backend emission
when MIR metadata proves a function needs the `hako.random` capability but no
accepted random route exists.

## Scope

- Define the preflight contract for `hako.random` capability plans.
- Add the smallest diagnostic owner needed to classify unsupported random
  execution before backend lowering.
- Keep metadata-only `uses random` legal when no execution route is requested.
- Add a focused guard for the unsupported-route diagnostic contract.

## Stop Lines

- No random/entropy extern route.
- No entropy source.
- No secure-list encode/decode behavior change.
- No cryptographic hardening claim.
- No provider activation, host allocator replacement, hook, or
  `#[global_allocator]`.
- No broad capability checker expansion beyond `hako.random` unsupported-route
  diagnostics.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `RANDOM-CAP-002.1` | Write the unsupported random preflight SSOT. | preflight states when metadata-only `uses random` is legal and when execution must fail. | no route |
| `RANDOM-CAP-002.2` | Add the narrow preflight/diagnostic owner. | unsupported random execution has a stable reason token. | no backend matcher |
| `RANDOM-CAP-002.3` | Add focused guard. | guard proves route remains inactive and diagnostic contract exists. | no behavior change |
| `RANDOM-CAP-002.4` | Close out current pointers. | current pointer guard passes and next row is selected. | no multi-row bundle |

## Required Evidence

```text
bash tools/checks/k2_wide_random_capability_preflight_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
