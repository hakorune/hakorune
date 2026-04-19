# Phase 137x Task Board

- Status: Active guardrail
- Current split:
  - `137x-A`: string publication contract closeout
  - `137x-B`: owner-first optimization return

## Completed

- [x] C1 refresh current pointers / stop-lines
- [x] C2 lock string-only `publish.text(reason, repr)` metadata contract
- [x] C3 land runtime-private publication adapters
- [x] C4 prove `substring_hii` StableView replay on the narrow explicit path
- [x] C5 make publication boundary metadata verifier-visible

## Active Before Perf Return

- [x] D1 `repr-downgrade-contract`
- [ ] D2 `stableview-legality-contract` (after D1)
- [ ] D3 `provenance-freeze-verifier-contract` (after D2)
- [ ] D4 `publish-idempotence-policy` (after D2)

## Blocked / Deferred

- [ ] E1 `publish-any-generalization` (blocked until D1-D4 land)
- [ ] E2 runtime-wide phase-289x implementation (parked; planning only)

## Exit

- [ ] F1 reopen the owner-first optimization lane only after D1-D4 are closed
