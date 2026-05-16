---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-061A scalar reclaim lane closeout guard.
Related:
  - docs/development/current/main/phases/phase-293x/293x-548-MIMAP-061A-RECLAIM-SCALAR-LANE-CLOSEOUT-GUARD.md
  - tools/checks/k2_wide_hako_alloc_reclaim_scalar_lane_closeout_guard.sh
---

# Hako Alloc Reclaim Scalar Lane Closeout SSOT

## Decision

`MIMAP-061A` closes the scalar reclaim execution lane that was landed through
`MIMAP-060A`.

This is a closeout / guard row only. It adds no allocator behavior and does not
open thread scheduling, page-source calls, OSVM unreserve/release, provider
activation, hooks, or host allocator replacement.

## Closed Row Set

The closeout guard locks these rows as one scalar reclaim lane:

| Row | Owner | Proof |
| --- | --- | --- |
| `MIMAP-051A` | owner-transfer readiness contract | `apps/hako-alloc-reclaim-owner-transfer-contract-proof/` |
| `MIMAP-054A` | atomic-claim contract | `apps/hako-alloc-reclaim-atomic-claim-contract-proof/` |
| `MIMAP-055A` | owner-transfer first execution | `apps/hako-alloc-reclaim-owner-transfer-execution-proof/` |
| `MIMAP-056A` | remote-free drain contract | `apps/hako-alloc-reclaim-remote-free-drain-contract-proof/` |
| `MIMAP-057A` | remote-free drain first execution | `apps/hako-alloc-reclaim-remote-free-drain-execution-proof/` |
| `MIMAP-058A` | post-drain owner-transfer integration | `apps/hako-alloc-reclaim-post-drain-owner-transfer-proof/` |
| `MIMAP-060A` | completion marker route | `apps/hako-alloc-reclaim-completion-marker-proof/` |

## Guard Surface

```text
tools/checks/k2_wide_hako_alloc_reclaim_scalar_lane_closeout_guard.sh
```

The guard verifies:

```text
all closed row cards are landed
all proof apps are listed in tools/checks/proof_apps.toml
all focused guards are listed in docs/tools/check-scripts-index.md
all hako_alloc owners stay exported
no reclaim app/owner matcher leaks into lang/c-abi/shims
provider activation and host allocator replacement remain absent
```

## Stop Lines

The closeout must not add:

```text
new .hako allocator behavior
page-source call
OSVM unreserve / release
thread scheduling
provider activation
hooks
host allocator replacement
backend app/name matcher
```

## Next Row

After this closeout, the current row should be:

```text
MIMAP-062A post-reclaim-scalar-closeout row selection
```

That row decides whether to continue allocator reclaim behavior, open a
compiler/language sidecar, or switch to a Hakorune language feature lane. The
decision must remain one narrow row.
