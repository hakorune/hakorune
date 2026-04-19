# Phase 137x Task Board

- Status: Closed guardrail
- Current split:
  - `137x-A`: string publication contract closeout
  - `137x-B`: container / primitive design cleanout (closed)
  - `137x-C`: structure completion gate before perf return (closed)
  - `137x-D`: owner-first optimization return (exact route-shape keeper landed)
  - `137x-E`: TextLane implementation gate before next kilo optimization (open)
  - `137x-F`: runtime-wide Value Lane implementation bridge (next)
  - `137x-G`: allocator / arena lane pilot (next)
  - `137x-H`: kilo optimization return after E/F/G land or reject

## Rule

Owner-first optimization already reopened as `137x-D` and landed the exact
array store route-shape keeper. The old rule that kept `TextLane`, runtime-wide
Value Lane, and allocator/arena work closed is retired. Before the next kilo
optimization pass, open the implementation gates in `137x-E/F/G`.

`all done` for this board means:

- every active structure/contract cleanup task below is closed
- every non-current successor is explicitly marked opened or blocked with a target phase
- `CURRENT_TASK.md` and `10-Now.md` point to the same active lane
- the final gate command is recorded

It now means the storage/value/allocator successor implementations are opened in
order before the next kilo optimization return.

## Closed String Publication Closeout (137x-A)

- [x] A1 refresh current pointers / stop-lines
- [x] A2 lock string-only `publish.text(reason, repr)` metadata contract
- [x] A3 land runtime-private publication adapters
- [x] A4 prove `substring_hii` StableView replay on the narrow explicit path
- [x] A5 make publication boundary metadata verifier-visible

## Closed Publication Contract Gates Before Perf Return

- [x] A6 `repr-downgrade-contract`
- [x] A7 `stableview-legality-contract`
- [x] A8 `provenance-freeze-verifier-contract`
- [x] A9 `publish-idempotence-policy`

## Closed Design Cleanout Before Perf Return

- [x] B1 `phase-pointer-resplit`
- [x] B2 `array-typed-slot-truth-sync`
- [x] B3 `map-demand-vs-typed-lane-boundary`
- [x] B4 `primitive-residuals-classification`
- [x] B5 `container-identity-residence-contract`

## Active Structure Completion Before Perf Return

- [x] C1 `current-pointer-stop-line-resplit`
  - set active lane to `137x-C structure completion gate`
  - move owner-first optimization return to `137x-D`
  - keep perf commands as blocked next-step only, not active mode
- [x] C2 `all-before-perf-task-normalization`
  - no open cleanup task may be hidden under "optimization return"
  - blocked successors must list their target phase
  - current cleanup scope must list its exit gate
- [x] C3 `primitive-array-map-done-definition`
  - Array typed-slot truth:
    - `InlineI64` / `InlineBool` / `InlineF64` residence exists
    - only `InlineI64` has direct typed encoded-load readback
    - Bool/F64 readback remains existing encoded-any/public handle contract
  - Map truth:
    - demand metadata is landed
    - typed map lane remains unopened
  - primitive residual truth:
    - `Null` / `Void` are non-blocking conservative residuals
    - enum/sum/generic remains separate SSOT
- [x] C4 `source-only-array-string-contract-index`
  - current source-only get/store contracts are listed with fixtures/smokes
  - insert-mid source-only contract:
    - fixture: `apps/tests/mir_shape_guard/array_string_len_insert_mid_source_only_min_v1.mir.json`
    - smoke: `tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_array_string_len_insert_mid_source_only_min.sh`
  - concat3 subrange source-only contract:
    - fixture: `apps/tests/mir_shape_guard/array_string_len_piecewise_concat3_source_only_min_v1.mir.json`
    - smoke: `tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_array_string_len_piecewise_concat3_source_only_min.sh`
  - live-after-get regression remains listed
    - fixture: `apps/tests/mir_shape_guard/array_string_len_live_after_get_min_v1.mir.json`
    - smoke: `tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_len_live_after_get_min.sh`
  - no broad `TextLane` / allocator / MIR legality work is inferred from these contracts
- [x] C5 `final-structure-gate`
  - `git status -sb`
  - `tools/checks/dev_gate.sh quick`
  - after this only, `137x-D` may start from fresh perf/asm evidence
  - result: `tools/checks/dev_gate.sh quick` PASS on 2026-04-20

## Blocked / Deferred

- [ ] E1 `publish-any-generalization` (blocked; target successor: separate `publish-any-generalization` phase, unnumbered)
- [ ] E3 typed map lane implementation (blocked; target successor `289x-6c`, owner proof required)
- [ ] E4 heterogeneous / union array slot layout (blocked; target successor: separate `heterogeneous-array-slot-layout` phase, unnumbered)

## Opened Implementation Order Before Next Kilo Optimization

- [ ] G1 `137x-E implementation gate before next kilo optimization`
  - active token: `137x-E TextLane implementation gate`
  - SSOT: `docs/development/current/main/phases/phase-137x/137x-94-textlane-value-allocator-implementation-gate.md`
- [ ] G2 minimal `TextLane` / `ArrayStorage::Text`
  - start with array string hot paths
  - keep `String = value`; `TextLane` is storage/residence only
- [ ] G3 runtime-wide `Value Lane` implementation bridge
  - use phase-289x ledgers as vocabulary/demand SSOT
  - keep Array / Map public identity unchanged
- [ ] G4 allocator / arena pilot
  - open after TextLane / Value Lane proof shows copy/allocation tax remains structural
  - require exact/middle/whole evidence and rollback notes

## Exit

- [x] F1 137x-A closeout gate is satisfied
- [x] F2 137x-B design cleanout gate is satisfied
- [x] F3 137x-C structure completion gate is satisfied
- [x] F4 owner-first optimization reopened as `137x-D`
- [ ] F5 implementation gate is open as `137x-E`
- [ ] F6 kilo optimization returns as `137x-H` only after `137x-E/F/G` land or reject
