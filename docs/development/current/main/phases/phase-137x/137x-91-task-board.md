# Phase 137x Task Board

- Status: Active guardrail
- Current split:
  - `137x-A`: string publication contract closeout
  - `137x-B`: container / primitive design cleanout
  - `137x-C`: owner-first optimization return

## Completed

- [x] C1 refresh current pointers / stop-lines
- [x] C2 lock string-only `publish.text(reason, repr)` metadata contract
- [x] C3 land runtime-private publication adapters
- [x] C4 prove `substring_hii` StableView replay on the narrow explicit path
- [x] C5 make publication boundary metadata verifier-visible

## Closed Before Perf Return

- [x] D1 `repr-downgrade-contract`
- [x] D2 `stableview-legality-contract`
- [x] D3 `provenance-freeze-verifier-contract`
- [x] D4 `publish-idempotence-policy`

## Active Design Cleanout Before Perf Return

- [x] B1 `phase-pointer-resplit`
- [x] B2 `array-typed-slot-truth-sync`
- [x] B3 `map-demand-vs-typed-lane-boundary`
- [ ] B4 `primitive-residuals-classification`
- [ ] B5 `container-identity-residence-contract`

## Blocked / Deferred

- [ ] E1 `publish-any-generalization` (blocked; keep deferred until a separate phase explicitly opens it)
- [ ] E2 runtime-wide phase-289x implementation (parked; planning only)
- [ ] E3 typed map lane implementation (blocked until a separate owner proof opens it)
- [ ] E4 heterogeneous / union array slot layout (blocked)

## Exit

- [x] F1 137x-A closeout gate is satisfied
- [ ] F2 137x-B design cleanout gate is satisfied
- [ ] F3 owner-first optimization may reopen as `137x-C`
