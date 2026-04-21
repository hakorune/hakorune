# Phase 137x Task Board

- Status: Closed guardrail; 137x-H active
- Current split:
  - `137x-A`: string publication contract closeout
  - `137x-B`: container / primitive design cleanout (closed)
  - `137x-C`: structure completion gate before perf return (closed)
  - `137x-D`: owner-first optimization return (exact route-shape keeper landed)
  - `137x-E0`: MIR / backend seam closeout before TextLane (closed)
  - `137x-E1`: TextLane / ArrayStorage::Text implementation (closed)
  - `137x-F`: runtime-wide Value Lane implementation bridge (closed)
  - `137x-G`: allocator / arena lane pilot (rejected for now)
  - `137x-H`: kilo optimization return after F/G land or reject (active)

## Rule

Owner-first optimization already reopened as `137x-D` and landed the exact
array store route-shape keeper. The old rule that kept `TextLane`, runtime-wide
Value Lane, and allocator/arena work closed is retired. The
`137x-F/G implementation gates before next kilo optimization` are closed:
`137x-F` landed and `137x-G` is rejected for now, so kilo optimization proceeds
as `137x-H`.

`all done` for this board means:

- every active structure/contract cleanup task below is closed
- every non-current successor is explicitly marked opened or blocked with a target phase
- `CURRENT_TASK.md` and `10-Now.md` point to the same active lane
- the final gate command is recorded

It now means the storage/value gates are landed, allocator/arena is rejected
with evidence for now, and the active H-series optimization card lives in the
phase README. Current active card: `137x-H25 array text residence session
contract`.

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

## Active H25 Task Breakdown

- [x] H25a `array_text_residence_sessions` metadata-only eligibility
  - MIR owns session eligibility.
  - `.inc` and runtime behavior unchanged.
- [x] H25b `array_text_residence_sessions` placement metadata
  - MIR emits begin/update/end placement and skip indices.
  - `.inc` must not derive preheader or exit shape from CFG.
- [x] H25c.1 `.inc` residence-session metadata reader
  - active `.inc` array/text reader seams use `*_route_metadata` naming.
  - current lowering consumes session metadata first but maps to the existing
    loopcarry update helper.
- [x] H25c.2a runtime-private session substrate
  - add `ArrayTextSlotSession` under ArrayBox text mechanics.
  - add kernel-private `ArrayTextWriteTxn` glue if needed.
  - no public ABI, no session handle table, no guard across C ABI calls.
  - no perf keeper claim unless later evidence proves a safe executor boundary.
- [x] H25c.2b single-call executor design gate
  - decide whether `slot_text_len_store_session` can become one
    capability-generic runtime call.
  - reject benchmark-named whole-loop helpers and runtime-owned legality.
- [x] H25c.2c single-region executor contract
  - add nested executor contract under `array_text_residence_sessions`, not a
    new sibling plan family.
  - [x] H25c.2c-1 MIR route metadata emits the nested
    `executor_contract` and route tests assert it.
  - [x] H25c.2c-2 `.inc` validates the nested contract without CFG/raw shape
    rediscovery.
  - [x] H25c.2c-3 extend MIR with required loop/PHI/exit mapping before
    region replacement.
  - [x] H25c.2c-4 backend region replacement without SSA redefinition.
  - `.inc` remains metadata-to-call emit only.
  - runtime gets a one-call RAII executor only under MIR-owned legality.
- [x] H25c.3 keeper probe
  - result: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 5 ms`
  - target transition: per-iteration exported fused helper left the emitted hot
    loop; owner moved into the runtime-private region executor.
- [x] H25d region executor inner mutation owner
  - perf-first next slice.
  - inspect/annotate `slot_text_region_update_sum_raw` before code changes.
  - no MIR widening unless a new legality/materialization fact is required.
  - [x] H25d.1 direct text-resident region loop
    - bypass per-iteration `ArrayTextSlotSession` dispatch when the write guard
      already exposes `ArrayStorage::Text`
    - keep compatible boxed/stringlike fallback unchanged
  - [x] H25d.2 hot/cold len-store mutation split
    - isolate the fixed in-place update path from the generic materialization
      fallback
    - keep UTF-8 boundary checks; no ASCII assumption without MIR proof
  - [x] H25d.3 small overlap copy specialization
    - avoid libc `memmove` for short text cells in the fixed in-place path
    - keep generic `ptr::copy` fallback for larger strings
    - rejected: instruction/cycle regression versus H25d.2
  - [x] H25d.4 observe flag hoist
    - compute `observe::enabled()` once at region helper entry, not per
      iteration
    - rejected: instruction/cycle regression versus H25d.2
  - [x] H25d.5 residual memmove / mutation owner decision
    - H25d final accepted code is H25d.1 + H25d.2
    - current result: `C 3 ms / Ny AOT 3 ms`,
      `ny_aot_instr=16570267`, `ny_aot_cycles=3471656`
    - verdict: close H25d; residual `memmove` / mutation surgery is not a
      keeper without new MIR proof, because H25d.3/H25d.4 both regressed
- [x] H25e post-parity owner refresh
  - re-baseline the current kilo exact/middle/whole fronts before opening new
    code
  - do not optimize from the H25d residual `memmove` percentage alone; pick the
    next owner from fresh stat + asm evidence
  - keeper gate remains owner-first: no helper-name shortcut, no runtime-owned
    legality, and no `.inc` planner drift
  - result:
    - exact `kilo_micro_array_string_store`: `C 10 ms / Ny AOT 4 ms`
    - middle `kilo_meso_substring_concat_array_set_loopcarry`: `C 3 ms / Ny AOT 4 ms`
    - whole `kilo_kernel_small`: `C 81 ms / Ny AOT 20 ms`
  - verdict: next code owner is the whole-front inner scan
    observer/conditional-store region, not H25d residual memmove surgery
- [x] H26 array text observer-store region contract
  - target front: `kilo_kernel_small`
  - target shape: `array.get(j).indexOf(const) >= 0` followed by same-array,
    same-index const-suffix store in the taken branch
  - design:
    - extend the existing MIR-owned `array_text_observer_routes` with a nested
      region executor contract instead of creating a benchmark-named helper
      family
    - `.inc` validates metadata and emits one runtime call; it must not rescan
      raw MIR CFG to rediscover the shape
    - runtime holds residence/guard mechanics inside one call and only executes
      needle search + suffix mutation
  - reject seam:
    - no indexOf result cache
    - no runtime-owned legality/provenance
    - no source-prefix assumption such as "`line` is always present"
    - no C-ABI session handle carrying guards
  - [x] H26.1 MIR nested observer-store executor contract
    - add `executor_contract` under existing `array_text_observer_routes`
    - emitted whole-front metadata includes `single_region_executor`,
      `observe.indexof`, `store.cell`, const needle `"line"`, and suffix `"ln"`
    - implementation split: route owner stays in
      `src/mir/array_text_observer_plan.rs`; nested contract proof lives in
      `src/mir/array_text_observer_region_contract.rs`
  - [x] H26.2 `.inc` metadata validation and one-call emit
    - `begin_block` / `begin_to_header_block` are MIR-owned fields
    - `.inc` preloads the contract before block emission and marks covered
      blocks unreachable without raw CFG rediscovery
  - [x] H26.3 runtime one-call observer-store executor
    - `nyash.array.string_indexof_suffix_store_region_hisisi`
      executes compare-only `indexOf` + same-slot suffix store under one guard
  - [x] H26.4 keeper/no-regression probe
    - whole `kilo_kernel_small`: `C 82 ms / Ny AOT 10 ms`
    - exact `kilo_micro_array_string_store`: `C 10 ms / Ny AOT 3 ms`
    - middle `kilo_meso_substring_concat_array_set_loopcarry`:
      `C 3 ms / Ny AOT 4 ms`

## Opened Implementation Order Before Next Kilo Optimization

- [x] G0 `137x-E0 MIR / backend seam closeout before TextLane`
  - closed preflight for `137x-E`
  - SSOT: `docs/development/current/main/phases/phase-137x/137x-95-mir-backend-seam-closeout-before-textlane.md`
  - MIR owns read-side alias continuation legality, publication contracts, provenance, and downgrade decisions
  - `.inc` consumes plan metadata and emits backend calls; it must not rediscover semantic legality
  - runtime array/string slot code may split by mechanism only, without becoming a semantic owner
- [x] G1 `137x-E implementation gate before next kilo optimization`
  - closed token: `137x-E TextLane implementation gate`
  - SSOT: `docs/development/current/main/phases/phase-137x/137x-94-textlane-value-allocator-implementation-gate.md`
- [x] G2 minimal `TextLane` / `ArrayStorage::Text`
  - start with array string hot paths
  - keep `String = value`; `TextLane` is storage/residence only
  - landed as runtime-private `ArrayStorage::Text`; array-string kernel routes use text raw APIs and mixed/generic arrays degrade to Boxed
- [x] G3 runtime-wide `Value Lane` implementation bridge
  - closed token: `137x-F Value Lane bridge`
  - use phase-289x ledgers as vocabulary/demand SSOT
  - keep Array / Map public identity unchanged
  - [x] G3a `137x-F1 demand-to-lane executor bridge`
    - map runtime-private `DemandSet` to `ValueLanePlan` action
    - first target is array-string TextCell residence vs generic boxed residence
    - no public ABI widening, no Map typed lane, no runtime-side legality/provenance inference
  - [x] G3b `137x-F closeout decision`
    - `137x-F2 producer outcome manifest split` is landed
    - verdict: do not open `137x-G`; current hot owners are string len/indexof/slot-write paths, with allocator/copy only secondary
- [x] G4 allocator / arena pilot
  - rejected / not opened by `137x-F` closeout
  - reopen only after exact/middle/whole proof shows copy/allocation tax is structural and dominant
  - current evidence: middle `cfree` 9.45%, whole `__memmove_avx512_unaligned_erms` 5.39%

## Exit

- [x] F1 137x-A closeout gate is satisfied
- [x] F2 137x-B design cleanout gate is satisfied
- [x] F3 137x-C structure completion gate is satisfied
- [x] F4 owner-first optimization reopened as `137x-D`
- [x] F5 implementation gate series is closed through `137x-F`; `137x-G` is rejected for now
- [x] F6 kilo optimization returns as `137x-H`; `137x-F` landed and `137x-G` is rejected for now
