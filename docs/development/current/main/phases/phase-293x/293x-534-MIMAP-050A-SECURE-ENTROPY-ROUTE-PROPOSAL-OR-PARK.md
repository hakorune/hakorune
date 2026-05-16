# 293x-534 MIMAP-050A Secure Entropy Route Proposal Or Park

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-050A` is the allocator planning row selected by `RANDOM-CAP-002`.

It chooses exactly one next step for secure entropy after:

```text
MIMAP-049A:
  secure entropy inventory

RANDOM-CAP-001:
  uses random capability metadata

RANDOM-CAP-002:
  unsupported random route preflight
```

## Scope

- Decide whether a real random/entropy route should be proposed now or kept
  parked.
- If parked, select the next allocator behavior row that can continue using
  deterministic proof keys / caller-provided cookies.
- If proposed, write a separate focused route card with backend/substrate stop
  lines and proof requirements.
- Update current pointers and taskboard after selection.

## Stop Lines

- No random/entropy execution in this planning row.
- No random/entropy extern route.
- No OS entropy source.
- No secure-list encode/decode behavior change.
- No cryptographic hardening claim.
- No provider activation, host allocator replacement, hook, or
  `#[global_allocator]`.
- No cleanup bundle.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `050A.1` | Read entropy inventory and random capability/preflight evidence. | route need is classified as propose-now or park. | no execution |
| `050A.2` | Select exactly one next row. | one current blocker token is named. | no multi-row bundle |
| `050A.3` | Update current pointers and taskboard. | current pointer guard passes. | no landed history copy expansion |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
