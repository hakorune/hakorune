# 293x-541 MIMAP-054A Reclaim Atomic-Claim Contract

Status: selected current
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
