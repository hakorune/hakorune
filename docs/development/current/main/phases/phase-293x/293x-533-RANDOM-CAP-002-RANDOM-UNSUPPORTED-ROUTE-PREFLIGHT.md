# 293x-533 RANDOM-CAP-002 Random Unsupported Route Preflight

Status: landed
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

## Implementation Result

`RANDOM-CAP-002` adds:

```text
SSOT:
  docs/development/current/main/design/random-capability-preflight-ssot.md
preflight owner:
  tools/checks/pure_first_route_preflight.py
guard:
  tools/checks/k2_wide_random_capability_preflight_guard.sh
```

Default preflight still accepts metadata-only `uses random`. Execution rows can
opt into unsupported random checking with:

```bash
tools/checks/pure_first_route_preflight.py \
  --reject-unsupported-random \
  app.mir.json
```

The explicit check fails with:

```text
reason=random_capability_route_unsupported
owner=capability_plans
contract=metadata.capability_plans[hako.random]
```

No random extern route, entropy source, secure-list behavior, provider
activation, or backend matcher is added.

## Evidence

```text
bash tools/checks/k2_wide_random_capability_preflight_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Selection Result

`RANDOM-CAP-002` selects `MIMAP-050A`.

```text
row:
  MIMAP-050A secure entropy route proposal-or-park
classification:
  allocator planning row
why now:
  `uses random` metadata and unsupported-route preflight now exist. The
  allocator lane can decide whether to propose a real entropy route or keep
  secure entropy execution parked.
stop lines:
  no entropy execution during selection
  no secure-list hardening behavior change
  no provider activation
```
