# 293x-541 MIMAP-054A Reclaim Atomic-Claim Contract

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-054A` is the allocator prerequisite row selected by `MIMAP-053A`.

Before reclaim owner-transfer execution opens, Hakorune needs a small contract
row that proves the atomic claim semantics:

```text
claim succeeds:
  expected abandoned owner matches
  claimant becomes the modeled owner token

claim fails:
  expected owner does not match
  modeled owner remains unchanged
```

This row is a contract/proof row. It must not execute page reclaim or mutate the
production facade's page owner.

## Scope

- Add an SSOT for reclaim atomic-claim contract vocabulary.
- Add a small `.hako` owner that models claim success/failure over scalar owner
  tokens.
- Add a proof app and focused guard.
- Keep `uses alloc_reclaim` / `hako.alloc.reclaim` visible as intent metadata
  where the proof function needs the reclaim execution lane, but keep
  production reclaim execution unsupported.
- Select the next row after the contract lands.

## Stop Lines

- No reclaim execution.
- No production page owner mutation.
- No remote-free drain.
- No thread scheduling.
- No page-source call.
- No OSVM unreserve / release.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No backend `.inc` app/name matcher.
- No cleanup bundle.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `054A.1` | Write atomic-claim contract SSOT. | success/failure and reason vocabulary are fixed. | no execution |
| `054A.2` | Add `.hako` contract owner. | scalar report shows success/failure without facade mutation. | no page-source |
| `054A.3` | Add proof app. | proof observes success, mismatch failure, and inactive execution flags. | no remote drain |
| `054A.4` | Add focused guard and docs index row. | guard proves stop lines and no backend matcher. | no default gate growth |
| `054A.5` | Close current pointers and select follow-up. | current pointer guard passes. | no bundle |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_reclaim_atomic_claim_contract_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Implementation Result

`MIMAP-054A` adds:

```text
SSOT:
  docs/development/current/main/design/hako-alloc-reclaim-atomic-claim-contract-ssot.md

owner:
  lang/src/hako_alloc/memory/reclaim_atomic_claim_contract_box.hako

proof app:
  apps/hako-alloc-reclaim-atomic-claim-contract-proof/

guard:
  tools/checks/k2_wide_hako_alloc_reclaim_atomic_claim_contract_guard.sh
```

The owner models:

```text
success:
  observed_owner == expected_owner
  owner_after = claimant_owner

failure:
  invalid expected owner, invalid claimant owner, or observed/expected mismatch
  owner_after = observed_owner
```

All production execution flags remain inactive:

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

## Evidence

```text
bash tools/checks/k2_wide_hako_alloc_reclaim_atomic_claim_contract_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Selection Result

`MIMAP-054A` selects `MIMAP-055A`.

```text
row:
  MIMAP-055A reclaim owner-transfer first execution route

classification:
  first guarded reclaim execution row

why now:
  owner-transfer readiness and atomic-claim semantics are both named. The next
  narrow row can execute one modeled owner transfer when contract and claim
  facts both succeed, while keeping remote-free drain, thread scheduling,
  page-source calls, and provider activation closed.

stop lines:
  no remote-free drain
  no thread scheduling
  no page-source call
  no OSVM unreserve / release
  no provider activation
  no cleanup bundle
```

Closeout:

```text
current blocker moves to MIMAP-055A.
```
