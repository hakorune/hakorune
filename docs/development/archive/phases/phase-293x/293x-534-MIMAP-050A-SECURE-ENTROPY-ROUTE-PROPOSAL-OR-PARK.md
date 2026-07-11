# 293x-534 MIMAP-050A Secure Entropy Route Proposal Or Park

Status: landed
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

## Selection Result

`MIMAP-050A` keeps secure entropy execution parked.

Reason:

```text
MIMAP-049A:
  secure entropy is a named inactive boundary.

RANDOM-CAP-001:
  `uses random` now reaches MIR metadata as hako.random.

RANDOM-CAP-002:
  unsupported random execution can fail before backend emission.

current allocator need:
  secure-list encode/decode can continue with caller-provided cookies and
  deterministic proof keys; no current row needs runtime entropy.
```

Decision:

```text
secure entropy route:
  parked

secure-list hardening pilot:
  not selected; requires a real random/entropy route and audit row first

caller-provided-cookie policy:
  remains canonical for current allocator proofs
```

`MIMAP-050A` selects `MIMAP-051A`.

```text
row:
  MIMAP-051A reclaim owner-transfer contract inventory

classification:
  allocator contract / inventory row

why now:
  the remaining allocator behavior lane is reclaim. Reclaim must not execute
  until ownership transfer, remote-free drain, atomic claim, and rollback
  preconditions are made explicit and observable.

stop lines:
  no reclaim execution
  no atomic ownership claim
  no remote-free drain
  no thread scheduling
  no page-source call
  no provider activation
```

Closeout:

```text
current blocker moves to MIMAP-051A.
```
