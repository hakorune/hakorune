---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-054A no-execution reclaim atomic-claim contract.
Related:
  - docs/development/current/main/design/hako-alloc-reclaim-owner-transfer-contract-ssot.md
  - docs/development/current/main/design/reclaim-execution-preflight-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-541-MIMAP-054A-RECLAIM-ATOMIC-CLAIM-CONTRACT.md
  - lang/src/hako_alloc/memory/reclaim_atomic_claim_contract_box.hako
  - apps/hako-alloc-reclaim-atomic-claim-contract-proof/
---

# Hako Alloc Reclaim Atomic-Claim Contract SSOT

## Decision

`MIMAP-054A` adds a no-execution `.hako` owner that models the owner-token
claim contract required before reclaim owner-transfer execution can open.

The contract is intentionally scalar:

```text
success:
  observed_owner == expected_owner
  expected_owner >= 0
  claimant_owner >= 0
  owner_after = claimant_owner

failure:
  expected_owner invalid, claimant_owner invalid, or observed_owner mismatch
  owner_after = observed_owner
```

This row names the atomic compare-and-claim semantics. It does not call an
atomic substrate route, mutate production page ownership, or execute reclaim.

## Owner

```text
lang/src/hako_alloc/memory/reclaim_atomic_claim_contract_box.hako
```

Responsibilities:

```text
classify owner-token claim success/failure
preserve expected / observed / claimant tokens in the report
publish hypothetical owner_after for the future execution row
count success, mismatch, and invalid-token cases
report inactive execution flags
```

Non-responsibilities:

```text
real atomic CAS
production page owner mutation
remote-free drain
thread scheduling
page-source calls
OSVM unreserve / release
provider / hook / replacement
secure entropy execution
```

## Reason Vocabulary

| Reason | Meaning |
| --- | --- |
| `0` | claim contract succeeds |
| `1` | observed owner does not match expected owner |
| `2` | expected owner token is invalid |
| `3` | claimant owner token is invalid |

## Proof Surface

```text
apps/hako-alloc-reclaim-atomic-claim-contract-proof/
tools/checks/k2_wide_hako_alloc_reclaim_atomic_claim_contract_guard.sh
```

Required inactive facts:

```text
would_execute_reclaim = 0
would_change_page_owner = 0
would_atomic_claim = 0
would_drain_remote_free = 0
would_schedule_thread = 0
would_call_page_source = 0
would_unreserve = 0
would_release_osvm = 0
```

## Stop Lines

No part of `MIMAP-054A` may add:

```text
reclaim execution
production page owner mutation
real atomic CAS / substrate call
remote-free drain
thread scheduling
page-source call
OSVM unreserve / release
provider activation
hooks
host allocator replacement
backend app/name matcher
```
