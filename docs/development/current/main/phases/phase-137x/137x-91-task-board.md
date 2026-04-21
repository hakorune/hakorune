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
phase README / current entry. Current active card:
`137x-H36.1 ArrayTextCell operation API split`.

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
- [x] H27 array text len-half insert-mid edit contract
  - target front: `kilo_kernel_small`
  - owner evidence after H26:
    - emitted outer edit loop still calls `nyash.array.string_len_hi`, then
      computes `split = len / 2`, then calls the same-slot insert-mid helper
    - whole asm top includes `nyash.array.string_len_hi` at `20.76%`
  - design:
    - add MIR-owned array/text edit metadata for the same-slot
      `source_len_div_const(2)` insert-mid contract
    - `.inc` consumes metadata at the `array.get(row)` site and emits one
      runtime-private edit helper; it must not prove split/substring/set
      legality from raw JSON
    - runtime executes the mutation and computes the current cell length inside
      the mutation frame; it must not own legality/provenance/publication
  - reject seam:
    - no source-prefix/source-length/ASCII assumption
    - no benchmark-named helper
    - no runtime-owned route selection
    - no C-side raw shape fallback for the new H27 path
  - [x] H27.1 docs/current-task cutover and metadata owner file
    - `src/mir/array_text_edit_plan.rs` owns the route proof and metadata.
  - [x] H27.2 `.inc` metadata reader and one-call emit/skip
    - `hako_llvmc_ffi_array_text_edit_metadata.inc` validates MIR contract
      fields and lowering emits `nyash.array.string_insert_mid_lenhalf_store_hisi`.
  - [x] H27.3 runtime helper for len-half insert-mid cell edit
    - runtime computes `split = current_text.len() / 2` as the MIR-selected
      policy and executes same-slot mutation only.
  - [x] H27.4 keeper/no-regression probe
    - whole `kilo_kernel_small`: `C 83 ms / Ny AOT 10 ms`,
      `ny_aot_instr=144977171`, `ny_aot_cycles=30931233`
    - exact `kilo_micro_array_string_store`: `C 10 ms / Ny AOT 4 ms`
    - middle `kilo_meso_substring_concat_array_set_loopcarry`:
      `C 4 ms / Ny AOT 4 ms`
    - verdict: small keeper / contract cleanup; outer edit
      `nyash.array.string_len_hi` is removed, but wall time remains in the
      same `10 ms` band.
- [ ] H28 array text observer-store search/copy owner split
  - target front: `kilo_kernel_small`
  - owner evidence after H27:
    - asm top: `<&str as Pattern>::is_contained_in` `34.68%`,
      `__memmove_avx512_unaligned_erms` `24.83%`,
      `with_array_text_write_txn` closure `15.16%`, observer-store region
      closure `11.02%`
    - H27 helper itself is around `1%`, so further len-half edit surgery is
      not the next owner
  - design:
    - keep the H26 observer-store region executor as the semantic owner shape:
      MIR proves observer + same-slot store region, `.inc` emits only from
      metadata, runtime executes search/copy/mutation mechanics
    - first inspect whether the next keeper is fixed-literal search mechanics,
      suffix mutation/copy mechanics, or a closeout needing more MIR proof
  - reject seam:
    - no source-prefix/source-length assumption
    - no search-result cache
    - no benchmark-named whole-loop helper
    - no runtime-owned legality/provenance/publication
    - no C-side raw shape fallback
  - [x] H28.1 fixed-literal search executor split
    - runtime-only `text_contains_literal` leaf replaces the generic
      `str::contains` Pattern path inside the existing observer-store helper
    - whole `kilo_kernel_small`: `C 84 ms / Ny AOT 9 ms`,
      `ny_aot_instr=60662079`, `ny_aot_cycles=20100504`
    - exact `kilo_micro_array_string_store`: `C 10 ms / Ny AOT 4 ms`
    - middle `kilo_meso_substring_concat_array_set_loopcarry`:
      `C 3 ms / Ny AOT 3 ms`
    - asm: `Pattern::is_contained_in` is no longer a top owner
  - [x] H28.2 short-literal prefix compare cleanup
    - annotate shows `__memcmp_evex_movbe` is the H28.1 `starts_with` prefix
      check lowering to libc `bcmp`
    - keep this runtime-private and do not change MIR metadata or `.inc`
    - result: whole `kilo_kernel_small`: `C 83 ms / Ny AOT 7 ms`,
      `ny_aot_instr=64501392`, `ny_aot_cycles=18956185`
    - exact guard `kilo_micro_array_string_store`: `C 11 ms / Ny AOT 4 ms`
    - middle guard `kilo_meso_substring_concat_array_set_loopcarry`:
      `C 3 ms / Ny AOT 4 ms`
    - asm: `__memcmp_evex_movbe` is no longer a top owner
  - [x] H28.3 suffix mutation/copy / write-frame owner split
    - inspect the remaining `__memmove_avx512_unaligned_erms` and
      write-frame closure owner before code changes
    - implement only a narrow runtime/backend/MIR change if evidence requires a
      new MIR-owned fact; otherwise keep it runtime-private
    - first active slice: runtime-private short-suffix append leaf, because
      annotate points at `value.push_str(suffix)` after the MIR-proven hit
    - result: whole `kilo_kernel_small`: `C 82 ms / Ny AOT 7 ms`,
      `ny_aot_instr=60615291`, `ny_aot_cycles=17586950`
    - exact guard `kilo_micro_array_string_store`: `C 10 ms / Ny AOT 4 ms`
    - middle guard `kilo_meso_substring_concat_array_set_loopcarry`:
      `C 3 ms / Ny AOT 4 ms`
    - verdict: small keeper; short suffix byte copy no longer calls `memcpy`,
      but residual capacity growth / write-frame `memmove` remains
  - [x] H28.4 capacity growth / write-frame owner decision
    - rejected: Rust-only short append headroom lowered `memmove` share but
      worsened whole instr/cycles/wall
    - code reverted; H28.3 append leaf remains
  - [x] H28.5 residual memmove owner refresh
    - callgraph attributes dominant `memmove` to the outer len-half edit
      closure, not append capacity
    - H28 observer-store search/copy split is closed
  - [x] H29 len-half edit copy owner decision
    - rejected: explicit reserve + suffix shift + middle copy leaf did not
      improve whole and raised `__memmove` share to `40.84%`
    - code reverted; local byte-copy surgery is not the next keeper
  - [ ] H30 array text edit residence representation decision
    - decide whether a narrow runtime-private edit residence representation
      can reduce the H27 len-half mid-insert suffix-copy owner cleanly
    - guard: no benchmark-named helper, no runtime legality/provenance, no
      `.inc` raw shape rediscovery, no public ABI widening
  - [x] H30.1 flat `ArrayTextCell` boundary extraction
    - BoxShape-only preparatory slice; keep the implementation flat-string
      only and make `ArrayStorage::Text` stop exposing raw `String` as the
      long-term representation truth
    - no MIR, `.inc`, public ABI, or behavior change
    - landed as `ArrayStorage::Text(Vec<ArrayTextCell>)`
  - [x] H30.2 array text edit operation boundary extraction
    - before adding any non-flat variant, route the H27 len-half edit helper
      through a runtime-private `ArrayTextCell` edit operation
    - no MIR, `.inc`, public ABI, or behavior change
  - [x] H30.3 non-flat edit residence prototype decision
    - open only after H30.2 is green; compare gap-buffer / piece-cell options
      behind the `ArrayTextCell` operation boundary
    - current preference: piece-cell/deferred-edit residence over gap-buffer
      unless perf evidence contradicts it
    - closed without keeper; code reverted
    - measurement hygiene note: stale perf before release rebuild must not be
      used as keeper evidence
  - [x] H31 post-H30 owner refresh
    - rerun whole stat/asm and attribute the remaining `memmove` owner before
      any new implementation card
    - result: fixed lane hygiene; rebuild release before runtime perf judgment
  - [x] H32 observer-store transaction path decision
    - choose the next narrow observer-store seam before code edits
    - landed transaction façade thinning; valid-release asm removed
      `with_array_text_write_txn` from the top list, but wall stayed
      `Ny AOT 7 ms`
  - [x] H33 valid post-H32 owner decision
    - valid-release direct runner showed no hot `string_len_hi`
    - next code card selected: narrow runtime-private observer-store
      short-byte leaf thinning
  - [x] H34 observer-store short-byte leaf thinning
    - touch only `src/boxes/array/ops/text.rs`
    - optimize short const-needle prefix check / short const-suffix byte write
      as mechanics only
    - no MIR, `.inc`, public ABI, semantic cache, or source-prefix assumption
    - kept: observer-store closure shrank from `27.45%` to `14.03%`;
      whole instructions dropped to `50229601`
  - [x] H35 post-H34 len-half copy owner decision
    - choose the next valid card for residual `memmove` / len-half closure
    - do not repeat H29 byte-copy surgery without a new representation proof
    - result: post-H34 top is `memmove` `48.59%`, len-half closure `26.13%`,
      observer-store closure `16.08%`
  - [x] H36 len-half residence representation design gate
    - decide whether `ArrayTextCell` opens non-flat / gap / piece residence
      for repeated len-half inserts
    - docs/design first; no MIR or `.inc` route change before runtime
      residence contract is clear
    - SSOT: `137x-97-h36-array-text-cell-residence-design-gate.md`
    - result: do not add a non-flat variant yet; first split operation APIs
  - [x] H36.1 ArrayTextCell operation API split
    - BoxShape-only precondition for any future non-flat text residence
    - add operation methods while staying flat-only
    - no MIR, `.inc`, public ABI, or perf keeper claim
    - result: hot-path contains/append operations now go through
      `ArrayTextCell` methods / string leaf wrappers
  - [ ] H36.2 ArrayTextCell residence decision
    - refresh whole stat/asm after H36.1
    - decide narrow non-flat residence pilot vs later-lane rejection

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
