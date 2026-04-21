# Phase 137x: main kilo reopen selection

- Status: observe-only guardrail for app-lane work
- 目的: `137x-E0/E/F` の MIR/backend seam, storage, value implementation gate を閉じ、`137x-G` allocator pilot を reject した状態から、owner-first evidence に従って kilo 最適化を進める。
- Active entry SSOT:
  - `137x-current.md`
- Array/text ownership map:
  - `137x-array-text-contract-map.md`
- Historical note:
  - this README is now the phase ledger and closed-history store; do not append
    new current-only details here unless the phase state changes.
- 対象:
  - `docs/development/current/main/CURRENT_STATE.toml`
  - `CURRENT_TASK.md`
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md`
  - `docs/development/current/main/design/value-corridor-generic-optimization-contract.md`
  - `crates/nyash_kernel/src/exports/string.rs`
  - `crates/nyash_kernel/src/plugin/map_substrate.rs`
  - `crates/nyash_kernel/src/plugin/map_aliases.rs`
  - `crates/nyash_kernel/src/exports/string_helpers.rs`
  - `crates/nyash_kernel/src/observe/backend/tls.rs`

## Quick Scan

- current-state token: `phase-290x ArrayBox surface canonicalization`
- current lane: `phase-137x observe-only guardrail`
- active current entry: `137x-current.md`
- active contract map: `137x-array-text-contract-map.md`
- semantic lock:
  - `String = value`
  - `publish = boundary effect`
  - `freeze.str = only birth sink`
  - `TextLane` is now an opened storage/residence implementation gate, not semantic truth
- mirror rule:
  - semantic authority stays in `docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md`
  - this README stays a current-state mirror and handoff note
- current exact front: `kilo_micro_array_string_store` is closed again by the compact 8-block route-shape matcher
- first 137x-D keeper proof: same-array/same-index piecewise concat3 subrange store originally lowered to `nyash.array.string_insert_mid_subrange_store_hisiii`; current direct lowering uses explicit-length `nyash.array.string_insert_mid_subrange_store_hisiiii`
- current owner split is now explicit:
  - `const_suffix freeze_fallback = 479728 / 480000`
  - `materialize total = 539728` (`~4.5 GB`)
  - `publish_reason.generic_fallback = 539728`
  - whole-side `site.string_concat_hh.* = 0`
  - whole-side `site.string_substring_concat_hhii.* = 0`
  - reading:
    - this is the historical producer-owner split that led to the landed phase-2 cuts
    - the current owner proof has moved to read-side encode/materialize/objectize and libc copy/alloc tax
- current middle guard: `kilo_meso_substring_concat_array_set_loopcarry`
- current middle evidence: `C 3 ms / Ny AOT 3 ms` (`repeat=3`, H25d.2 region executor inner mutation keeper)
- direct-only correctness: `Result: 2880064`, exit code `64`
- current stop-line:
  - `KernelTextSlot` exit is observed and inactive (`publish_boundary.slot_* = 0`)
  - `137x-F/G` implementation gates before next kilo optimization are closed: `137x-F` landed and `137x-G` is rejected for now
  - continue kilo optimization only as `137x-H` with owner-first evidence per slice
- current phase cut before next kilo optimization:
  - `137x-A`: string publication contract closeout (`137x-92-string-publication-contract-closeout.md`)
  - `137x-B`: container / primitive design cleanout (`137x-93-container-primitive-design-cleanout.md`) is closed
  - `137x-C`: structure completion gate before perf return is closed (`137x-91-task-board.md`)
  - `137x-D`: owner-first optimization return landed the exact array store route-shape keeper
  - `137x-E0`: MIR / backend seam closeout before TextLane is closed
  - `137x-E1`: minimal `TextLane` / `ArrayStorage::Text` implementation is landed
  - `137x-F`: runtime-wide `Value Lane` implementation bridge is closed
  - `137x-G`: allocator / arena pilot is rejected / not opened by F closeout
  - `137x-H`: next kilo optimization return is current
- current closeout status:
  - done: `repr-downgrade-contract`
    - verifier now rejects unproven `stable_view` repr requests before runtime; lowering must downgrade to `stable_owned` until StableView legality is verifier-visible
  - done: `stableview-legality-contract`
    - `stable_view_provenance` now names the only accepted string-only StableView witnesses: `already_stable`, `immutable_host_owned`, `pinned_no_mutation`
  - done: `provenance-freeze-verifier-contract`
    - `publish.text` now requires borrow provenance, source root, and the freeze/publish separation publication contract before codegen
  - done: `publish-idempotence-policy`
    - repeated slot publish is no-op after `Published`; cache handle reissue may reuse an existing stable object/view, but must not rebirth fresh text for the same stable source/cell
  - closeout gate:
    - 137x-A is satisfied
    - 137x-B design cleanout is satisfied
    - 137x-C structure completion gate is satisfied
    - 137x-D exact route-shape keeper is landed
    - 137x-E0, 137x-E1, and 137x-F are closed; 137x-G is rejected for now before the next kilo optimization
    - `publish.any` remains blocked here
- closed design cleanout gate:
  - closed: `137x-93-container-primitive-design-cleanout.md`
  - purpose: sync array typed-slot, map demand/typed-lane, primitive residual, and container identity/residence docs before perf work resumes
  - done:
    - `array-typed-slot-truth-sync`
      - ArrayBox typed-slot truth is scalar immediate residence for `InlineI64` / `InlineBool` / `InlineF64`
      - only `InlineI64` has direct typed encoded-load readback; f64/bool stay under encoded-any/public handle readback
    - `map-demand-vs-typed-lane-boundary`
      - Map demand metadata is landed for key decode, value store, and value load
      - typed map lane remains closed; RuntimeData stays a facade, not semantic owner
    - `primitive-residuals-classification`
      - `Null` / `Void` are conservative, non-blocking residuals
      - enum/sum/generic stays under its separate SSOT and does not authorize optimization return
      - primitive/user-box, enum/sum, and ArrayBox residence proofs are sibling proofs, not interchangeable keeper evidence
    - `container-identity-residence-contract`
      - Array / Map public identity and ABI rows stay unchanged
      - lane-host eligibility is limited to internal element/key/value residence
      - runtime-wide Value Lane implementation is now opened only through the constrained `137x-F` bridge
  - no public ABI widening starts from this gate
- successor implementation order:
  - `137x-E0`: MIR / backend seam closeout before TextLane
  - `137x-E1`: minimal `TextLane` / `ArrayStorage::Text` (landed)
  - `137x-F`: runtime-wide `Value Lane` implementation bridge
  - `137x-G`: allocator / arena pilot
  - SSOT: `137x-94-textlane-value-allocator-implementation-gate.md`
  - preflight SSOT: `137x-95-mir-backend-seam-closeout-before-textlane.md`
- legacy retirement SSOT:
  - planned deletions for the active compiler cleanup live in the `Legacy Retirement Ledger` section of this README
  - do not scatter deletion TODOs across lowering/runtime files; code comments may only point back to this ledger when a compatibility row would otherwise look accidental
- current first seam: phase-2.5 read-side alias lane; producer-side unpublished outcome under `const_suffix` is already landed
- current rollout order:
  - `Phase 1`: producer outcome -> canonical sink (`KernelTextSlot` first)
  - `Phase 2`: cold publish effect
  - `Phase 2.5`: read-side alias lane split
  - `Phase 2.6`: string publication contract closeout / legality lock
  - `Phase 3`: `TextLane` storage/residence implementation (`137x-E`)
  - `Phase 4`: Value Lane bridge closed and allocator pilot deferred before the next kilo optimization (`137x-F/G`)

## 137x-E1 TextLane Slice

Status: landed.

Scope:
- add `ArrayStorage::Text` as runtime-private array residence for text-heavy string-store routes
- keep `String = value`; `TextLane` is not a semantic carrier, public handle, or MIR truth
- connect only array-string kernel read/store/mutate routes to text raw APIs
- degrade generic/mixed array operations to Boxed rather than widening Array semantics

Acceptance:
- existing Array/String public behavior stays unchanged
- active phase137x array-string smokes stay green
- `TextLane` can be observed through ArrayBox debug/storage tests without requiring public ABI changes

Implementation notes:
- `ArrayStorage::Text(Vec<String>)` is internal residence only; generic/mixed writes materialize back to `Boxed`.
- array-string read/write/store routes use `slot_with_text_raw`, `slot_update_text_raw`, and `slot_store_text_raw`.
- the old array-string store `BorrowedHandleBox` retarget executor path is removed from the active store route; alias legality must stay MIR-owned rather than runtime-replanned.

## 137x-F1 Value Lane Bridge Slice

Status: landed.

Scope:
- add a runtime-private bridge from `DemandSet` to concrete executor lane action
- start with the landed array text residence route only: TextCell residence vs generic boxed residence
- keep Array / Map public identity, public ABI, and MIR legality unchanged
- do not infer borrow/provenance/publication legality from helper names or runtime class names

Acceptance:
- array-string store chooses TextLane residence through the demand-backed bridge, not through a helper-local ad hoc action
- non-string / mixed array-string store remains generic boxed residence
- unit tests lock the demand-to-lane mapping for text cell, generic box, and publish-boundary demands
- existing phase137x array-string route smokes remain green

Implementation guard:
- `ValueLanePlan` is runtime-private executor plumbing only.
- `DemandSet` names the requested lane; MIR/lowering remain the authority for whether that demand is legal.
- `137x-F1` does not start Map typed lane, allocator, or runtime-wide value object rewrites.

Verification:
- `cargo test -q -p nyash_kernel --lib value_lane` PASS
- `cargo test -q -p nyash_kernel --lib value_demand` PASS
- `cargo test -q -p nyash_kernel --lib array::tests` PASS
- `cargo test -q array::tests --lib` PASS
- `phase137x_direct_emit_array_store_string_contract.sh` PASS
- `phase137x_direct_emit_const_suffix_kernel_slot_store_contract.sh` PASS
- `phase137x_boundary_array_string_len_insert_mid_source_only_min.sh` PASS
- `tools/checks/current_state_pointer_guard.sh` PASS
- `tools/checks/dev_gate.sh quick` PASS

## 137x-F2 Producer Outcome Manifest Split

Status: landed.

Scope:
- split the producer outcome manifest so owned bytes stay separate from the publish boundary effect
- keep `freeze_text_plan_with_site` behavior unchanged at the call boundary
- keep this bridge runtime-private; do not open Map typed lanes, allocator, or public ABI rows

Acceptance:
- the freeze outcome helper keeps owned bytes available before publish
- non-`FreezeTextPlanPieces3` sites stay preserved through normalization
- publish still goes through the owned-bytes generic fallback boundary for the recorded site

Implementation guard:
- `FrozenTextPlan` is an internal manifest only.
- `freeze_text_plan_outcome_with_site` may prepare the publish step, but it must not publish itself.
- `137x-F2` does not reopen legality/provenance inference in runtime code.

Verification:
- `cargo test -q -p nyash_kernel --lib freeze_text_plan_outcome -- --test-threads=1` PASS
- `cargo test -q -p nyash_kernel --lib value_lane -- --test-threads=1` PASS
- `cargo test -q -p nyash_kernel --lib freeze_text_plan_with_site_publishes_owned_bytes -- --test-threads=1` PASS

## 137x-H1 MIR String Value Lowering Cleanup

Status: closed.

Scope:
- keep source string values in MIR value-world form until an explicit object/publication boundary
- stop using `value_origin_newbox[StringBox]` as the proof that a string literal or string `Add` result exists
- keep `Callee::Method(StringBox.*)` as a runtime method-dispatch compatibility boundary for now
- do not introduce first-class `text.ref` / `text.owned` / `publish` MIR instructions in this slice; the active contract stays in `StringKernelPlan` metadata and verifier checks
- keep Rust handle issuance behind named publish/cache boundary helpers instead of exporting raw fresh-handle calls to string helpers

Acceptance:
- string constants emitted by builder constant lowering are typed as `MirType::String`
- string `Add` results are typed as `MirType::String`
- match-return literal composition records string literals as `MirType::String`
- method resolution still maps `MirType::String` receivers to `StringBox` runtime dispatch without creating `value_origin_newbox`
- no new public ABI, no `publish.any`, no runtime-wide object rewrite

Implementation guard:
- `value_origin_newbox` remains reserved for explicit `NewBox` / constructor-origin object facts.
- A string value may be *dispatched through* `StringBox` runtime methods, but that dispatch does not prove the value was born as a `StringBox`.
- `publish.text` remains MIR-owned metadata; `freeze.str` remains the string birth sink.

## 137x-H2 MIR JSON Text Object Boundary Shrink

Status: closed.

Scope:
- retire the active `compat_text_primitive.rs` module name from Rust callers
- keep the remaining `MIR(JSON text) -> object path` contract as an explicit backend boundary, not as a legacy helper replacement surface
- do not change route order, C-API keep behavior, provider keep behavior, or boundary-default behavior
- do not touch the array/string-store exact seed bridge in this slice

Acceptance:
- `compat_text_primitive` has no active code references
- the shared object emission entry is named for the contract it owns: MIR JSON text to object
- plugin-loader `emit_object` and compiled-stage1 LLVM backend surrogate still share one chokepoint
- quick gate stays green

Implementation guard:
- This slice is no-behavior cleanup.
- The new boundary must normalize MIR JSON input once and delegate route selection to the existing route layer.
- Exact seed bridge deletion remains blocked until active array-string store route coverage is proven outside the exact matcher.

Verification:
- `rustfmt --check src/host_providers/llvm_codegen/mir_json_text_object.rs src/host_providers/llvm_codegen.rs src/runtime/plugin_loader_v2/enabled/compat_codegen_receiver.rs crates/nyash_kernel/src/plugin/module_string_dispatch/compat/llvm_backend_surrogate.rs` PASS
- `git diff --check` PASS
- `cargo check -q` PASS
- `cargo check -q -p nyash_kernel` PASS
- `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_array_store_string_contract.sh` PASS
- `tools/checks/dev_gate.sh quick` PASS
- note: full `cargo fmt --check` still reports pre-existing observe-module formatting drift; this slice does not rewrite those files.

## 137x-H3 Exact Array-String-Store Emitted Length Seam

Status: closed.

Scope:
- shrink only the temporary `kilo_micro_array_string_store` exact seed bridge
- remove the emitted per-iteration `strlen(slot)` from the specialized stack-array IR
- use only facts already proven by the bridge guard: `seed_len == 16`, `size == 128`, `ops == 800000`, and the store writes `seed + "xy"` plus nul
- do not widen route matching, do not add semantic legality, and do not promote this bridge into keeper architecture

Perf-first baseline:
- `tools/checks/dev_gate.sh quick` PASS before the slice
- `kilo_micro_array_string_store = C 10 ms / Ny AOT 10 ms`
- `ny_aot_instr=26917228`, `ny_aot_cycles=34122757`
- exact `ny_main` top owner is `__strlen_evex 53.84%`; annotated loop calls `strlen@plt` after copying the known 18-byte slot payload

Acceptance:
- `phase137x_direct_emit_array_store_string_contract.sh` still proves exact seed emitter selection and no runtime/public helper calls in `ny_main`
- regenerated exact asm has no `strlen@plt` call in `ny_main`
- exact micro perf does not regress in instructions or wall time
- `tools/checks/dev_gate.sh quick` stays green

Implementation:
- `hako_llvmc_emit_array_string_store_micro_ir(...)` now derives `store_len = seed_len + 2` after the exact bridge guard and emits that constant into the sum update.
- the emitted IR no longer declares or calls `strlen`; the exact bridge still only accepts the compact guarded micro shape.

Verification:
- `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_array_store_string_contract.sh` PASS
- `bash tools/perf/build_perf_release.sh` PASS
- `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_array_string_store 1 3`:
  - `kilo_micro_array_string_store = C 10 ms / Ny AOT 9 ms`
  - `ny_aot_instr=18866112`, `ny_aot_cycles=27213979`
- `bash tools/perf/bench_micro_aot_asm.sh kilo_micro_array_string_store 'ny_main' 3`:
  - top owner moved to `ny_main 99.19%`
  - annotated `ny_main` has no `strlen@plt`; the sum update is `addq $0x12`

## 137x-H4 Exact Array-String-Store Out-Buffer Seam

Status: closed.

Scope:
- shrink only the temporary `kilo_micro_array_string_store` exact seed bridge
- remove the emitted `out` stack buffer from the specialized stack-array IR
- write `text + "xy"` directly into the selected array slot, then update loop-carried `text` from `slot + 2`
- do not widen route matching, do not add MIR legality, and do not introduce runtime/public helpers

Perf-first baseline:
- after `137x-H3`, exact micro is `kilo_micro_array_string_store = C 10 ms / Ny AOT 9 ms`
- exact `ny_main` top owner is now the remaining stack-copy loop
- annotated samples sit on the `out` temp copy (`vmovaps %xmm0,-0x50(%rsp)`) and the slot tail store

Acceptance:
- `phase137x_direct_emit_array_store_string_contract.sh` still proves exact seed emitter selection and no runtime/public helper calls in `ny_main`
- regenerated exact asm has no `strlen@plt` and no separate `out` temp copy before the slot store
- exact micro perf does not regress in instructions or wall time
- `tools/checks/dev_gate.sh quick` stays green

Implementation:
- `hako_llvmc_emit_array_string_store_micro_ir(...)` no longer emits an `out` alloca.
- the loop now copies `text` directly to `%slot`, writes `xy\0` at the slot boundary, accounts the known length, then copies `%slot + 2` back to loop-carried `text`.

Verification:
- `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_array_store_string_contract.sh` PASS
- `bash tools/perf/build_perf_release.sh` PASS
- `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_array_string_store 1 3`:
  - `kilo_micro_array_string_store = C 10 ms / Ny AOT 8 ms`
  - `ny_aot_instr=11666577`, `ny_aot_cycles=20300845`
- `bash tools/perf/bench_micro_aot_asm.sh kilo_micro_array_string_store 'ny_main' 3`:
  - top owner remains `ny_main 98.53%`
  - annotated loop writes the slot directly and reloads loop-carried text from `slot + 2`; there is no separate `out` stack-buffer copy and no runtime/public helper call

## 137x-H5 Middle Same-Slot Subrange Store Materialization Seam

Status: landed.

Scope:
- keep the existing runtime-private `nyash.array.string_insert_mid_subrange_store_hisiiii` ABI
- add only an executor-side fast path for the safe same-slot shape where insert-const + subrange preserves the source byte length
- avoid allocating a fresh `String` on that shape by mutating the text-resident slot in place after checking only the required UTF-8 byte boundaries
- do not widen MIR route matching, do not add public ABI, and do not make helper names semantic truth

Perf-first baseline:
- `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 10 ms`
- top report:
  - `array_string_len_by_index` closure is the largest owner
  - same-slot subrange store closure is the second owner
  - Rust dealloc remains visible, indicating materialization churn in the store path

Acceptance:
- existing array-string insert-mid subrange tests stay green
- middle perf does not regress in instructions or wall time
- `tools/checks/dev_gate.sh quick` stays green

Implementation:
- `array_string_insert_const_mid_subrange_by_index_store_same_slot_str(...)` now first tries a narrow in-place update for the same-length shape.
- The keeper path checks the virtual subrange window and only the required UTF-8 byte boundaries, then performs `insert_str`, drains the leading byte, and truncates back to the source length.
- The fallback materialization path remains unchanged for all non-matching or non-boundary-safe inputs.

Verification:
- `cargo test -q -p nyash_kernel --lib insert_mid_subrange_store_by_index -- --test-threads=1` PASS
- `bash tools/perf/build_perf_release.sh` PASS
- `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_array_set_loopcarry 1 3`:
  - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 8 ms`
  - `ny_aot_instr=83021976`, `ny_aot_cycles=26509766`
- `bash tools/perf/bench_micro_aot_asm.sh kilo_meso_substring_concat_array_set_loopcarry 'ny_main' 3`:
  - ASCII full-scan regression is absent after boundary-only checks
  - top owners remain `array_string_len_by_index` and the same-slot store closure
- `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_kernel_small 1 3`:
  - `kilo_kernel_small = C 81 ms / Ny AOT 19 ms`
- `tools/checks/dev_gate.sh quick` PASS

## 137x-H6 Array Text Length Substrate Seam

Status: landed.

Scope:
- keep the existing runtime-private `nyash.array.string_len_hi` ABI
- thin only the executor-side array text length substrate behind that ABI
- do not add MIR known-length inference in this slice
- do not widen routes or add public ABI

Perf-first baseline:
- `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 8 ms`
- top report after H5:
  - `array_string_len_by_index` closure remains the largest owner
  - same-slot subrange store closure is reduced but still visible

Acceptance:
- array text lane tests stay green
- middle perf does not regress in instructions/cycles; wall stays in the same 8-9 ms noise band
- `tools/checks/dev_gate.sh quick` stays green

Implementation:
- `ArrayBox::slot_text_len_raw(...)` gives array text residence a direct length substrate.
- `array_string_len_by_index(...)` now uses that substrate instead of routing through `slot_with_text_raw(...)` with a closure.
- This keeps the existing `nyash.array.string_len_hi` ABI and intentionally does not add known-length MIR inference.

Verification:
- `cargo test -q array::tests::slot_store_text_births_text_lane -- --test-threads=1` PASS
- `bash tools/perf/build_perf_release.sh` PASS
- `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_array_set_loopcarry 1 5`:
  - `kilo_meso_substring_concat_array_set_loopcarry = C 4 ms / Ny AOT 9 ms`
  - `ny_aot_instr=80862657`, `ny_aot_cycles=26265409`
- `bash tools/perf/bench_micro_aot_asm.sh kilo_meso_substring_concat_array_set_loopcarry 'ny_main' 3`:
  - top owner is now the `nyash.array.string_len_hi` call boundary itself
  - next keeper candidate is MIR/lowering same-length proof, not more runtime substrate thinning
- `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_kernel_small 1 3`:
  - `kilo_kernel_small = C 84 ms / Ny AOT 19 ms`
- `tools/checks/dev_gate.sh quick` PASS

## 137x-H7 Same-Length Loop-Carry Length Call Seam

Status: closed.

Scope:
- remove the standalone `nyash.array.string_len_hi` call only when lowering proves the full same-slot loop-carry window
- keep length ownership as MIR/backend proof plus runtime-private execution, not a hard-coded array seed length
- fuse only the shape `len -> split=len/2 -> insert const middle -> substring(1, len+1) -> same-slot set -> carry.length`
- do not generalize this into array-wide known-length inference or allocator work

Perf-first baseline:
- `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_array_set_loopcarry 1 3`:
  - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 8 ms`
  - `ny_aot_instr=80862956`, `ny_aot_cycles=26254374`
- `bash tools/perf/bench_micro_aot_asm.sh kilo_meso_substring_concat_array_set_loopcarry 'ny_main' 3`:
  - top owner is `nyash.array.string_len_hi`
  - `ny_main` still computes `len`, `split`, `end`, then calls `nyash.array.string_insert_mid_subrange_store_hisiiii`

Acceptance:
- generated `ny_main` no longer calls `nyash.array.string_len_hi` for the proven loop-carry window
- total result stays `Result: 2880064`, exit code `64`
- `tools/checks/dev_gate.sh quick` stays green
- keep the fused helper runtime-private; no public Array/String ABI widening

Result:
- implementation:
  - backend lowering now recognizes the proven same-slot loop-carry window and emits one runtime-private helper:
    - `nyash.array.string_insert_mid_subrange_len_store_hisi(handle, idx, middle_ptr, middle_len) -> i64`
  - the helper mutates the selected array text slot and returns the resulting length, so `ny_main` no longer needs a standalone `nyash.array.string_len_hi` call
  - the matcher rejects the fusion if skipped intermediate values are used after the matched window
- `PERF_AOT_DIRECT_ONLY=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_array_set_loopcarry 1 3`:
  - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 6 ms`
  - `ny_aot_instr=58004175`, `ny_aot_cycles=17079682`
- `PERF_AOT_DIRECT_ONLY=1 bash tools/perf/bench_micro_aot_asm.sh kilo_meso_substring_concat_array_set_loopcarry 'ny_main' 1`:
  - hot loop calls `nyash.array.string_insert_mid_subrange_len_store_hisi`
  - no `nyash.array.string_len_hi` call remains in `ny_main`
  - new top owner moves inside the fused runtime helper plus `__memmove_avx512_unaligned_erms` / `alloc::string::Drain`
- guard held:
  - no array-wide known-length inference
  - no public ABI widening
  - no route legality ownership moved into runtime

## 137x-H8 Same-Length Loop-Carry Byte Rewrite Seam

Status: closed.

Scope:
- optimize only the runtime-private same-length subrange mutation that H7 already proved at lowering time
- replace `insert_str -> drain -> truncate` with a fixed-length byte rewrite when all required UTF-8 boundaries are proven locally
- do not add allocator/arena work, array-wide length inference, or new MIR legality

Perf-first baseline:
- after H7, `PERF_AOT_DIRECT_ONLY=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_array_set_loopcarry 1 3`:
  - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 6 ms`
  - `ny_aot_instr=58004175`, `ny_aot_cycles=17079682`
- `PERF_AOT_DIRECT_ONLY=1 bash tools/perf/bench_micro_aot_asm.sh kilo_meso_substring_concat_array_set_loopcarry 'array_string_insert_const_mid_subrange_len_by_index_store_same_slot_str' 3`:
  - top owner is the fused helper closure
  - visible secondary owners include `__memmove_avx512_unaligned_erms` and `alloc::string::Drain`

Acceptance:
- `try_update_insert_const_mid_subrange_same_len_in_place(...)` keeps string length unchanged without allocating or using `String::Drain`
- generated `ny_main` remains on the H7 fused helper route
- `kilo_meso_substring_concat_array_set_loopcarry` stays correct and improves or holds instruction/cycle count
- `tools/checks/dev_gate.sh quick` stays green before landing

Result:
- implementation:
  - the proven same-length interior shape now rewrites the existing `String` bytes in place
  - fallback materialization remains for non-interior or non-boundary-safe shapes
- `PERF_AOT_DIRECT_ONLY=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_array_set_loopcarry 1 3`:
  - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 6 ms`
  - `ny_aot_instr=49691974`, `ny_aot_cycles=12941203`
- `PERF_AOT_DIRECT_ONLY=1 bash tools/perf/bench_micro_aot_asm.sh kilo_meso_substring_concat_array_set_loopcarry 'ny_main' 3`:
  - `ny_main` still calls only `nyash.array.string_insert_mid_subrange_len_store_hisi` in the hot loop
  - `String::Drain` and libc `memmove` no longer appear as top owners
- guard held:
  - runtime remains executor-only for the MIR-proven window
  - no public ABI widening
  - no allocator/arena rewrite

## 137x-H9 Same-Length Loop-Carry Small Shift Seam

Status: rejected.

Scope:
- optimize only the runtime-private same-length byte rewrite introduced by H8
- remove libc `memmove` from the proven small shift path when the mutation can stay inside the existing `String` allocation
- do not add new MIR legality, route inference, public ABI, or benchmark-name dispatch

Perf-first baseline:
- after H8, `PERF_AOT_DIRECT_ONLY=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_array_set_loopcarry 1 3`:
  - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 6 ms`
  - `ny_aot_instr=49693471`, `ny_aot_cycles=13039022`
- `PERF_AOT_DIRECT_ONLY=1 bash tools/perf/trace_optimization_bundle.sh --input kilo_meso_substring_concat_array_set_loopcarry --route direct --function main --callee-substr string_insert_mid --lookahead 16 --microasm-runs 3 --symbol array_string_insert_const_mid_subrange_len_by_index_store_same_slot_str --out-dir target/perf_state/optimization_bundle/137x-h9-loopcarry-owner`:
  - top owner remains the fused helper closure
  - libc `__memmove_avx512_unaligned_erms` is still visible as the secondary owner
  - hot block confirms the H8 `ptr::copy` shifts lower to `memmove` calls

Acceptance:
- generated `ny_main` remains on the H7 fused helper route
- H8 same-length semantics remain unchanged
- libc `memmove` leaves the exact-front top report or instruction/cycle count improves enough to justify the local executor change
- `tools/checks/dev_gate.sh quick` stays green before landing

Result:
- rejected as non-keeper
- trial: replace the small H8 `ptr::copy` shifts with bytewise overlap loops while keeping large strings on the existing `ptr::copy` path
- `PERF_AOT_DIRECT_ONLY=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_array_set_loopcarry 1 3`:
  - `ny_aot_instr=54372282`, `ny_aot_cycles=13785721`
  - worse than H8 baseline `ny_aot_instr=49693471`, `ny_aot_cycles=13039022`
- decision:
  - keep the H8 `ptr::copy` implementation
  - do not replace libc `memmove` with manual byte loops unless a later proof can remove the extra loop/control-flow cost

## 137x-H10 Text-Resident Slot Update Fast Path Seam

Status: closed.

Scope:
- optimize only the H7/H8 fused helper when the target array is already `ArrayStorage::Text`
- keep existing mixed/boxed behavior as a cold fallback through `slot_update_text_raw`
- do not change public ArrayBox semantics, storage promotion rules, MIR legality, or public ABI

Perf-first baseline:
- after H8 and rejected H9, `PERF_AOT_DIRECT_ONLY=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_array_set_loopcarry 1 3`:
  - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 6 ms`
  - `ny_aot_instr=49693471`, `ny_aot_cycles=13039022`
- saved bundle: `target/perf_state/optimization_bundle/137x-h9-loopcarry-owner`
  - top owner is the fused helper closure
  - hot annotate points at the text slot write-lock / fast-path entry, not string materialization

Acceptance:
- generated `ny_main` remains on the H7 fused helper route
- text-resident arrays skip boxed/text promotion checks in the hot helper
- mixed/boxed arrays keep the existing fallback contract
- exact-front instruction/cycle count improves or holds
- `tools/checks/dev_gate.sh quick` stays green before landing

Result:
- implementation:
  - added a text-resident-only ArrayBox update path
  - H7/H8 fused helper now tries the text-resident path first and falls back to existing `slot_update_text_raw` for mixed/boxed arrays
- `PERF_AOT_DIRECT_ONLY=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_array_set_loopcarry 1 3`:
  - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 6 ms`
  - `ny_aot_instr=40152332`, `ny_aot_cycles=12636090`
  - improvement from H8 baseline `ny_aot_instr=49693471`, `ny_aot_cycles=13039022`
- `PERF_AOT_DIRECT_ONLY=1 bash tools/perf/bench_micro_aot_asm.sh kilo_meso_substring_concat_array_set_loopcarry 'ny_main' 3`:
  - generated `ny_main` remains on `nyash.array.string_insert_mid_subrange_len_store_hisi`
  - top owner remains inside the fused helper closure
- guard held:
  - no public ABI widening
  - no new MIR legality
  - no public ArrayBox semantics change

## 137x-H11 Exclusive Text-Region Lock Owner

Status: blocked.

Observation:
- after H10, `target/perf_state/optimization_bundle/137x-h11-loopcarry-owner` shows the fused helper closure remains the dominant owner
- hot annotate:
  - the largest sample is the `ArrayBox` text slot write-lock fast path (`lock cmpxchg`)
  - libc `memmove` is now secondary noise, not the primary owner
- `PERF_AOT_DIRECT_ONLY=1 bash tools/perf/run_kilo_string_split_pack.sh 1 3 0`:
  - `kilo_micro_substring_only`: `c_instr=1623550`, `ny_aot_instr=1667573`
  - `kilo_micro_substring_views_only`: `c_instr=123590`, `ny_aot_instr=467225`
  - `kilo_micro_len_substring_views`: `c_instr=1623552`, `ny_aot_instr=1673034`
  - `kilo_kernel_small_hk`: `C 81 ms / Ny AOT 26 ms`, parity ok

Decision:
- do not remove the `ArrayBox` lock inside runtime based on the helper name or benchmark shape
- lock removal requires a MIR-owned exclusive text-region / proof-region contract that can justify hoisting or eliding synchronization
- until that contract exists, continue with other measured owners or open a separate design slice for exclusive mutable array regions

## 137x-H12 MIR-Owned Loopcarry Route SSOT

Status: closed for the active loopcarry route; adjacent direct-set,
slot-hop, and exact-seed cleanup moved into H13.

Purpose:
- close the remaining route ownership leak in the active loopcarry middle guard before opening a lock-elision or allocator slice
- make MIR metadata the owner of the fused `array.get -> string edit -> array.set -> length` route decision
- remove the C `.inc` window matcher from the active route path once the metadata route covers the active direct front

Boundary:
- MIR may recognize the full fused window and emit a backend-consumable route plan
- `.inc` may read the selected route plan, emit `nyash.array.string_insert_mid_subrange_len_store_hisi`, and skip the planned instructions
- `.inc` must not rediscover substring / insert / set / trailing length legality when a MIR route plan exists
- runtime remains executor only; no public ABI or ArrayBox semantic widening starts here

Acceptance:
- `kilo_meso_substring_concat_array_set_loopcarry` still lowers to the H7 fused helper route
- MIR JSON exposes a route plan for the active loopcarry len-store window
- `.inc` consumes the MIR route plan without consulting the legacy window matcher
- legacy C matcher has been removed from the active lowering path for this route
- `tools/checks/dev_gate.sh quick` stays green before landing

First slice result:
- MIR JSON now emits `metadata.array_text_loopcarry_len_store_routes` for the active loopcarry len-store window
- direct backend route trace hits `array_string_loopcarry_len_store_window` with `reason=mir_route_plan`
- probe-only perf guard after legacy removal: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 8 ms`, `aot_status=ok`
- `tools/checks/dev_gate.sh quick`: green
- next deletion gate: extend the same metadata-first treatment to the remaining active direct/front loopcarry windows, if any

## 137x-H13 MIR-Owned Backend Route Cleanup

Status: active; the piecewise direct-set consumer, slot-hop substring route,
exact array-string seed bridge, and concat-const-suffix seed bridge now have
MIR-owned metadata for the active fronts.

Purpose:
- continue the H12 ownership cleanup on the adjacent direct-front `Pieces3` route
- make MIR `StringKernelPlan` own whether a piecewise text value may be consumed by direct `array.set`
- make generic MIR `value_consumer_facts` own the single direct `set` sink fact
- make MIR `StringKernelPlan` own the same-block slot-hop substring continuation route and skip indices
- make MIR `FunctionMetadata.array_string_store_micro_seed_route` own the current compact 8-block exact seed proof
- make MIR own the still-active `kilo_micro_concat_const_suffix` exact seed proof before deleting its C-side scanner
- keep `.inc` as a plan reader and emitter instead of a consumer-shape scanner for this decision

Boundary:
- MIR may scan current uses and expose `read_alias.direct_set_consumer`
- MIR may scan canonical value uses and expose `metadata.value_consumer_facts[*].direct_set_consumer`
- MIR may expose `slot_hop_substring` with `consumer_value`, `start`, `end`, `instruction_index`, and `copy_instruction_indices`
- MIR may expose `array_string_store_micro_seed_route` with the guarded seed/size/ops/suffix proof for the current exact micro front
- MIR may expose a concat-const-suffix micro route with the guarded seed/suffix/ops proof for the still-active exact micro front
- `.inc` may use that fact to defer piecewise publication or reject the fast route
- `.inc` must not decide direct-set legality for the selected `Pieces3` value from raw JSON when the MIR fact is present
- `.inc` must not rediscover slot-hop substring callee/receiver legality from raw JSON; it may only consume the MIR route and apply skip marks
- `.inc` must not rediscover the array-string seed bridge 8-block shape from raw JSON; it may only consume MIR route metadata and select the existing temporary emitter

Acceptance:
- `StringKernelPlan.read_alias.direct_set_consumer` is exported in MIR JSON
- `metadata.value_consumer_facts` is exported in MIR JSON
- `StringKernelPlan.slot_hop_substring` is exported in MIR JSON
- `metadata.array_string_store_micro_seed_route` is exported in MIR JSON for the active exact micro front
- `metadata.concat_const_suffix_micro_seed_route` is exported in MIR JSON for the active exact micro front
- string concat/insert direct-front emit routes use the MIR fact for direct-set consumer decisions
- `has_direct_array_set_consumer(...)` is removed from the backend shim surface
- `match_piecewise_slot_hop_substring_consumer(...)` is removed from the backend shim surface
- the raw C-side 8-block array-string seed JSON scanner is removed from `hako_llvmc_ffi_array_string_store_seed.inc`
- the raw C-side concat-const-suffix 5-block scanner is removed from `hako_llvmc_ffi_concat_const_suffix_seed.inc` after metadata replacement lands
- existing route guards remain green

Second slice result:
- `.inc` now uses `hako_llvmc_value_has_direct_set_consumer(...)`, a metadata reader over MIR-owned `value_consumer_facts`
- the old C-side `has_direct_array_set_consumer(...)` JSON scanner and its trace-only helper were deleted
- remaining cleanup is not direct-set ownership; it is slot-hop substring skip planning and the exact array-string seed bridge

Third slice result:
- `StringKernelPlan.slot_hop_substring` now records the slot-hop substring route and same-block skip indices in MIR metadata
- `.inc` uses `hako_llvmc_string_kernel_plan_read_slot_hop_substring(...)` and no longer scans MIR JSON to rediscover the next substring callee/receiver
- next cleanup target was the exact array-string seed bridge

Fourth slice result:
- `FunctionMetadata.array_string_store_micro_seed_route` now records the active compact 8-block exact seed route: seed, seed length, size, ops, suffix, stored length, and proof
- MIR JSON exports `metadata.array_string_store_micro_seed_route` for `kilo_micro_array_string_store`
- `hako_llvmc_match_array_string_store_micro_seed(...)` now only reads the MIR route metadata and selects the existing specialized stack-array emitter
- the previous raw C-side 8-block scanner macros and per-block callee checks were deleted from `hako_llvmc_ffi_array_string_store_seed.inc`
- remaining adjacent inventory: other exact micro seed families still have raw `.inc` matchers, but the active array-string store exact bridge is no longer the kilo route-owner blocker
- verification:
  - direct MIR metadata probe shows `metadata.array_string_store_micro_seed_route` with `seed_len=16`, `size=128`, `ops=800000`, `suffix=xy`, `store_len=18`, and proof `kilo_micro_array_string_store_8block`
  - `cargo test array_string_store_micro_seed --lib` PASS
  - `bash tools/perf/build_perf_release.sh` PASS
  - `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_array_store_string_contract.sh` PASS
  - `tools/checks/dev_gate.sh quick` PASS
  - `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_array_string_store 1 1`: `C 10 ms / Ny AOT 8 ms`, `aot_status=ok`
  - `git diff --check` PASS

Fifth slice result:
- retired `hako_llvmc_ffi_concat_hh_len_seed.inc` and removed its pure-compile dispatch slot
- current direct MIR for `kilo_micro_concat_hh_len` has already moved away from the old 5-block exact matcher shape, so generic/metadata lowering preserves the active front without the bridge
- `concat_const_suffix` was tested but kept: removing its exact bridge regressed `kilo_micro_concat_const_suffix` from `Ny AOT 3 ms` to `101 ms`, so that surface needs a MIR-owned replacement before deletion
- verification:
  - `bash tools/perf/build_perf_release.sh` PASS
  - `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_concat_hh_len 1 3`: `C 3 ms / Ny AOT 3 ms`, `aot_status=ok`
  - `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_concat_const_suffix 1 3`: `C 3 ms / Ny AOT 3 ms`, `aot_status=ok`
  - `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_array_string_store 1 1`: `C 10 ms / Ny AOT 8 ms`, `aot_status=ok`

Sixth slice plan:
- add MIR-owned metadata for the active `kilo_micro_concat_const_suffix` route proof
- make `hako_llvmc_match_concat_const_suffix_micro_seed(...)` consume that metadata and select the existing temporary emitter
- delete the raw C-side 5-block scanner macros and per-block callee checks from `hako_llvmc_ffi_concat_const_suffix_seed.inc`
- keep the emitter temporary until the generic string lane can match the exact front without a dedicated bridge

Sixth slice result:
- `FunctionMetadata.concat_const_suffix_micro_seed_route` now records the active 5-block exact seed route: seed, seed length, suffix, suffix length, ops, result length, and proof
- MIR JSON exports `metadata.concat_const_suffix_micro_seed_route` for `kilo_micro_concat_const_suffix`
- `hako_llvmc_match_concat_const_suffix_micro_seed(...)` now only reads MIR route metadata and selects the existing temporary emitter
- the previous raw C-side 5-block scanner macros and per-block callee checks were deleted from `hako_llvmc_ffi_concat_const_suffix_seed.inc`
- verification:
  - direct MIR metadata probe shows `metadata.concat_const_suffix_micro_seed_route` with `seed_len=16`, `suffix_len=2`, `ops=600000`, `result_len=18`, and proof `kilo_micro_concat_const_suffix_5block`
  - `cargo test concat_const_suffix_micro_seed --lib` PASS
  - `bash tools/perf/build_perf_release.sh` PASS
  - `tools/checks/dev_gate.sh quick` PASS
  - `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_concat_const_suffix 1 3`: `C 3 ms / Ny AOT 4 ms`, `aot_status=ok`
  - `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_array_string_store 1 1`: `C 10 ms / Ny AOT 6 ms`, `aot_status=ok`
  - `git diff --check` PASS

Seventh slice plan:
- shrink `hako_llvmc_ffi_string_loop_seed_substring_concat.inc` without adding a new MIR route vocabulary
- use existing MIR `StringKernelPlan.loop_payload` as the seed/middle/loop-bound/split owner for `kilo_micro_substring_concat`
- use existing `stable_length_scalar` relation metadata as the length-only legality witness
- delete the raw C-side block/op scanner macros from the substring-concat exact seed matcher
- keep the temporary emitter until the generic string lane can emit the same exact front directly

Seventh slice result:
- `hako_llvmc_match_substring_concat_loop_ascii_seed(...)` now reads existing MIR `StringKernelPlan.loop_payload` metadata to select the substring-concat exact emitter
- the length-only route now reads existing `stable_length_scalar` relation metadata by base plan root instead of rediscovering the header/source-length witness from blocks
- the raw C-side block/op scanner macros and direct `fn.blocks` scan were deleted from `hako_llvmc_ffi_string_loop_seed_substring_concat.inc`
- verification:
  - direct MIR metadata probe shows `string_kernel_plans[*].loop_payload` for `kilo_micro_substring_concat` with seed `line-seed-abcdef`, seed length `16`, loop bound `300000`, split length `8`, and middle literal `xx`
  - `bash tools/perf/build_perf_release.sh` PASS
  - `NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_substring_concat_phi_merge_contract.sh` PASS
  - `NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_substring_concat_post_sink_shape.sh` PASS
  - `tools/checks/dev_gate.sh quick` PASS
  - `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_substring_concat 1 3`: `C 3 ms / Ny AOT 4 ms`, `aot_status=ok`
  - `git diff --check` PASS

Eighth slice plan:
- add MIR-owned metadata for the active `kilo_micro_substring_views_only` route proof
- keep existing `StringKernelPlan` borrowed-slice plans as the window proof owner
- add only the exact bridge payload missing from generic plans: source literal/length and loop bound
- make `hako_llvmc_match_substring_views_only_micro_seed(...)` consume that metadata and select the existing temporary emitter
- delete the raw C-side block/op scanner from `hako_llvmc_ffi_string_loop_seed_views_only.inc`

Eighth slice result:
- `FunctionMetadata.substring_views_micro_seed_route` now owns the exact bridge payload for `kilo_micro_substring_views_only`: source literal, source length, loop bound, and proof name
- existing `StringKernelPlan` borrowed-slice plans remain the window proof owner; the new route does not add borrowed-window legality
- `hako_llvmc_match_substring_views_only_micro_seed(...)` now consumes metadata and keeps only validation plus the existing temporary emitter selection
- the raw C-side block/op scanner was deleted from `hako_llvmc_ffi_string_loop_seed_views_only.inc`
- verification:
  - direct MIR metadata probe shows `metadata.substring_views_micro_seed_route` with `source_len=16`, `loop_bound=300000`, and proof `kilo_micro_substring_views_only_5block`
  - `cargo test substring_views_micro_seed --lib` PASS
  - `bash tools/perf/build_perf_release.sh` PASS
  - `tools/checks/dev_gate.sh quick` PASS
  - `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_substring_views_only 1 3`: `C 3 ms / Ny AOT 3 ms`, `aot_status=ok`
  - `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_len_substring_views 1 3`: `C 3 ms / Ny AOT 3 ms`, `aot_status=ok`
  - `git diff --check` PASS

Ninth slice plan:
- retire the dead length-hot exact matcher family instead of metadata-porting it
- current direct MIR for `kilo_micro_len_substring_views` is a 4-block generic/metadata route with borrowed-slice `StringKernelPlan` facts; it does not hit the old 5/6-block exact matcher family
- current direct MIR for `method_call_only_small` is also a 4-block generic route and does not hit the old runtime-data length-hot matcher
- remove `hako_llvmc_ffi_string_loop_seed_length_hot_loop.inc` from the active pure-compile route table
- remove its now-dead `hako_llvmc_emit_string_length_hot_loop_ir(...)` emitter from the shared seed emitter file
- keep generic len/substr policy metadata as the owner for length consumers

Ninth slice result:
- `hako_llvmc_ffi_string_loop_seed_length_hot_loop.inc` is deleted and pure-compile dispatch no longer calls the old runtime-data/string length-hot exact matchers
- `hako_llvmc_emit_string_length_hot_loop_ir(...)` is deleted because it had no remaining caller after the matcher retirement
- removing the include exposed a hidden include-order brace dependency; the stray leading `}` in `hako_llvmc_ffi_string_loop_seed_substring_concat.inc` is removed so each `.inc` file is syntactically self-contained
- verification:
  - `bash tools/perf/build_perf_release.sh` PASS
  - `tools/checks/dev_gate.sh quick` PASS
  - `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_len_substring_views 1 3`: `C 3 ms / Ny AOT 3 ms`, `aot_status=ok`
  - `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh method_call_only_small 1 3`: `C 2 ms / Ny AOT 2 ms`, `aot_status=ok`
  - `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_substring_concat 1 3`: `C 3 ms / Ny AOT 3 ms`, `aot_status=ok`
  - `git diff --check` PASS

## 137x-H14 MIR-Owned String Search Seed Route

Problem:
- `hako_llvmc_ffi_string_search_seed.inc` still proves the exact `indexOf` leaf/line micro routes by parsing raw MIR JSON in C.
- This duplicates the MIR route-owner rule established in H12/H13 and makes `.inc` a small delayed planner instead of a thin emitter selector.

Decision:
- MIR owns the string search exact seed route proof as function metadata.
- Metadata payload: `variant`, `rows`, `ops`, `line_seed`, `line_seed_len`, `none_seed`, `none_seed_len`, `needle`, `needle_len`, optional `flip_period`, and proof name.
- `.inc` may keep the temporary specialized emitters for now, but it must select them only from MIR metadata and must not rescan raw blocks/instructions for route legality.
- Historical guard: `NYASH_LLVM_SKIP_INDEXOF_LINE_SEED` remained a backend guard while generic lowering was slower; H15.7 retires it after the text-state residence route reaches keeper speed.

First slice plan:
- add `FunctionMetadata.indexof_search_micro_seed_route`
- export it through MIR JSON
- make `hako_llvmc_match_indexof_leaf_ascii_seed(...)` and `hako_llvmc_match_indexof_line_ascii_seed(...)` metadata consumers
- remove the C-side raw block/op scanners from `hako_llvmc_ffi_string_search_seed.inc`
- verify `kilo_leaf_array_string_indexof_const` and `kilo_micro_indexof_line` direct fronts before returning to broader kilo optimization

First slice result:
- `FunctionMetadata.indexof_search_micro_seed_route` now owns the current string-search exact seed proof.
- Direct MIR JSON exports:
  - `variant=leaf`, `proof=kilo_leaf_array_string_indexof_const_10block`, `rows=64`, `ops=400000`, `flip_period=null`
  - `variant=line`, `proof=kilo_micro_indexof_line_15block`, `rows=64`, `ops=400000`, `flip_period=16`
- `hako_llvmc_match_indexof_leaf_ascii_seed(...)` and `hako_llvmc_match_indexof_line_ascii_seed(...)` now read `metadata.indexof_search_micro_seed_route`.
- The raw C-side block/instruction scanners, including the obsolete 18-block line guard, are deleted.
- The temporary specialized emitters remain as backend-local bridge code.

Verification:
- `cargo test indexof_search_micro_seed --lib` PASS
- `rustfmt --check src/mir/indexof_search_micro_seed_plan.rs src/mir/function/types.rs src/mir/semantic_refresh.rs src/mir/mod.rs src/runner/mir_json_emit/root.rs` PASS
- `bash tools/perf/build_perf_release.sh` PASS
- direct MIR metadata probes for `kilo_leaf_array_string_indexof_const` and `kilo_micro_indexof_line` PASS
- manual route trace: `indexof_leaf_micro result=emit reason=exact_match extra=kilo_leaf_array_string_indexof_const mir_route_plan`
- `tools/checks/dev_gate.sh quick` PASS
- `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_indexof_line 1 3`: `C 4 ms / Ny AOT 4 ms`, `aot_status=ok`
- `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_leaf_array_string_indexof_const 1 3`: `C 4 ms / Ny AOT 64 ms`, `aot_status=ok`
- note: leaf route is selected, but the existing exact emitter still calls `nyash.string.indexOf_ss` in the loop; keep this as the next performance seam instead of mixing it into the SSOT cleanup slice.

## 137x-H14.1 MIR-Owned IndexOf Predicate Action

Problem:
- H14 moved the leaf/line exact search route proof into MIR metadata, but `hako_llvmc_emit_indexof_leaf_ir(...)` still emits a per-iteration runtime call to `nyash.string.indexOf_ss`.
- That makes the backend executor pay runtime string-search tax even though the MIR proof already constrains the candidate set to two literals with stable outcomes.

Decision:
- Extend `FunctionMetadata.indexof_search_micro_seed_route` from route proof to route action.
- MIR owns:
  - `result_use = found_predicate`
  - `backend_action = literal_membership_predicate`
  - candidate outcomes: `line_seed => found`, `none_seed => not_found`
- `.inc` may only validate and emit this action. It must not derive predicate legality from helper names, literal spelling, or raw block/op scans.
- Keep this as a temporary exact bridge shrink; it does not promote the exact emitter into keeper architecture and does not open generic `publish.any` or runtime-wide Value Lane work.

First slice plan:
- add predicate/action fields to `IndexOfSearchMicroSeedRoute`
- export them through MIR JSON
- make `hako_llvmc_read_indexof_search_micro_seed_route(...)` validate the action contract
- replace the leaf loop `nyash.string.indexOf_ss` call with the same metadata-owned literal membership predicate already used by the line bridge
- verify leaf and line fronts before deciding whether to delete or further shrink the remaining exact search emitter surface

First slice result:
- `IndexOfSearchMicroSeedRoute` now exports `result_use=found_predicate`, `backend_action=literal_membership_predicate`, and `candidate_outcomes=[line_seed=>found, none_seed=>not_found]`.
- `hako_llvmc_read_indexof_search_micro_seed_route(...)` validates the action contract before selecting the temporary exact emitter.
- `hako_llvmc_emit_indexof_leaf_ir(...)` no longer declares or calls `nyash.string.indexOf_ss`; the leaf loop uses the metadata-owned literal membership predicate.
- The line bridge remains on the same predicate form and continues to require the MIR route proof plus `flip_period=16`.

Verification:
- `cargo test indexof_search_micro_seed --lib` PASS
- `rustfmt --check src/mir/indexof_search_micro_seed_plan.rs src/mir/function/types.rs src/mir/semantic_refresh.rs src/mir/mod.rs src/runner/mir_json_emit/root.rs` PASS
- `bash tools/perf/build_perf_release.sh` PASS
- direct MIR metadata probes for `kilo_leaf_array_string_indexof_const` and `kilo_micro_indexof_line` export `result_use`, `backend_action`, and `candidate_outcomes` PASS
- manual route trace: `indexof_leaf_micro result=emit reason=exact_match extra=kilo_leaf_array_string_indexof_const mir_route_plan`
- `nm -u /tmp/indexof_leaf_pred.o | rg 'indexOf|nyash\.string'` and `objdump -d /tmp/indexof_leaf_pred.o | rg 'call|nyash|string|indexOf'` found no runtime search call in the leaf object
- `tools/checks/dev_gate.sh quick` PASS
- `git diff --check` PASS
- `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_leaf_array_string_indexof_const 1 3`: `C 4 ms / Ny AOT 4 ms`, `aot_status=ok`
- `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_indexof_line 1 3`: `C 5 ms / Ny AOT 4 ms`, `aot_status=ok`

## 137x-H14.2 Exact Search Emitter Surface Shrink

Problem:
- The remaining temporary search bridge still has separate leaf and line emitters even though both consume the same MIR-owned route proof/action and emit the same literal membership predicate.
- Keeping two emitters increases the backend surface that must eventually be deleted or replaced by generic indexOf lowering.

Decision:
- Collapse the two backend emitters into one optional-flip emitter.
- MIR remains the owner of route proof, predicate action, candidate outcomes, and flip eligibility.
- `.inc` may only validate metadata and pass `flip_period=0` for leaf or `flip_period=16` for line; it must not regain route legality or predicate inference.

First slice plan:
- replace `hako_llvmc_emit_indexof_leaf_ir(...)` and `hako_llvmc_emit_indexof_line_ir(...)` with one `hako_llvmc_emit_indexof_seed_ir(...)`
- keep leaf/line matcher functions as metadata consumers only, because route gating and the temporary backend guard still live at dispatch level in this slice
- verify leaf and line micro fronts remain green before considering exact bridge deletion

First slice result:
- `hako_llvmc_emit_indexof_seed_ir(...)` is now the only exact search seed emitter.
- Leaf and line still keep separate dispatch functions, but both are metadata consumers: leaf passes `flip_period=0`; line passes the MIR-owned `flip_period=16`.
- Route proof, predicate action, candidate outcomes, and flip eligibility remain MIR-owned metadata; the `.inc` surface only validates and emits.

Verification:
- `git diff --check` PASS
- `bash tools/perf/build_perf_release.sh` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_leaf_array_string_indexof_const 1 3`: `C 4 ms / Ny AOT 3 ms`, `aot_status=ok`
- `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_indexof_line 1 3`: `C 4 ms / Ny AOT 4 ms`, `aot_status=ok`

## 137x-H14.3 Exact Search Matcher Surface Shrink

Problem:
- H14.2 collapsed the actual emitter, but `hako_llvmc_match_indexof_leaf_ascii_seed(...)` and `hako_llvmc_match_indexof_line_ascii_seed(...)` still duplicate the same JSON parse, metadata validation, trace emission, and emitter call.
- Deleting the exact bridge is not yet keeper-safe in this slice: with `NYASH_LLVM_SKIP_INDEXOF_LINE_SEED=1`, `kilo_micro_indexof_line` remains correct but falls back to `C 5 ms / Ny AOT 11 ms`. H15.7 supersedes this after text-state residence reaches keeper speed.

Decision:
- Keep the leaf/line wrapper names as dispatch-level surfaces for this slice.
- Move shared parse/validation/trace/emitter mechanics into one `hako_llvmc_match_indexof_ascii_seed_variant(...)` helper.
- Do not add a new env guard and do not reopen C-side route proof/action; wrappers may only provide variant/proof/trace constants.

First slice plan:
- add `hako_llvmc_match_indexof_ascii_seed_variant(...)`
- make leaf and line wrappers call the shared helper
- verify exact leaf/line micro fronts remain green

First slice result:
- `hako_llvmc_match_indexof_ascii_seed_variant(...)` owns the shared JSON parse, MIR metadata validation, route trace, and `hako_llvmc_emit_indexof_seed_ir(...)` call.
- `hako_llvmc_match_indexof_leaf_ascii_seed(...)` and `hako_llvmc_match_indexof_line_ascii_seed(...)` remain only as dispatch-level wrapper names with variant/proof/trace constants.
- The remaining exact search bridge is thinner, but it is not deleted because the generic line fallback is still materially slower.

Verification:
- `NYASH_LLVM_SKIP_INDEXOF_LINE_SEED=1 PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_indexof_line 1 3`: `C 5 ms / Ny AOT 11 ms`, `aot_status=ok` (deletion rejected for now)
- `git diff --check` PASS
- `bash tools/perf/build_perf_release.sh` PASS
- `tools/checks/dev_gate.sh quick` PASS
- `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_leaf_array_string_indexof_const 1 3`: `C 5 ms / Ny AOT 4 ms`, `aot_status=ok`
- `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_indexof_line 1 3`: `C 4 ms / Ny AOT 4 ms`, `aot_status=ok`

## 137x-H15 Generic Array/Text Observer Route

- Closed cleanup lane:
  - MIR owns `array_text_observer_routes`
  - exact dispatch bridge is retired in H15.7
  - remaining text-state residence temporary emitter/payload stays quarantined under `array_text_state_residence_route.temporary_indexof_seed_payload`
  - exported `indexof_search_micro_seed_route` is retired in H15.9; `array_text_state_residence_route` is the only backend metadata owner for this path
  - `array_text_state_residence_route` is now a real `FunctionMetadata` field, not a JSON alias of the exact bridge key
  - `array_text_state_residence_route` top-level now contains only the generic residence contract; exact proof/action/literal data is quarantined under `temporary_indexof_seed_payload`
  - raw observer analyzer/trace `.inc` files are removed from active compilation; active observer lowering consumes MIR metadata only
  - remaining temporary emitter surface is named `hako_llvmc_ffi_indexof_text_state_residence.inc`
  - closeout verdict: `temporary_indexof_seed_payload` remains explicit, fixture-backed, and quarantined until a generic residence emitter replaces it
  - next step: return to owner-first kilo optimization and rerun perf evidence before source edits
- Detailed H15.1-H15.9 history and closeout live in [137x-96-h15-array-text-residence-cleanup.md](./137x-96-h15-array-text-residence-cleanup.md).
- Closed gate:
  - `hako_llvmc_ffi_indexof_text_state_residence.inc` remains quarantined as the temporary text-state residence payload reader/emitter
  - further deletion stays blocked until a generic residence emitter no longer needs `temporary_indexof_seed_payload`

## 137x-H16 Exact Array-String Store Text-Shift Seam

Status: closed.

Scope:
- shrink only the temporary `kilo_micro_array_string_store` exact bridge
- make MIR route metadata expose the follow-up substring window that updates loop-carried text
- let `.inc` emit text-state update mechanics from metadata instead of copying back from the just-written slot
- keep route legality, public ABI, and runtime ownership unchanged

Perf-first baseline:
- `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_array_string_store 1 3`
  - `C 10 ms / Ny AOT 7 ms`
  - `ny_aot_instr=11671010`, `ny_aot_cycles=20593774`
- `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_aot_asm.sh kilo_micro_array_string_store 'ny_main' 3`
  - `ny_main` owns 97.81% of AOT cycles
  - hot annotate points at the emitted 16-byte `slot + 2 -> text` copy

Acceptance:
- `cargo test array_string_store_micro_seed --lib`
- `bash tools/perf/build_perf_release.sh`
- `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_array_store_string_contract.sh`
- route trace still emits `array_string_store_micro result=emit reason=exact_match`
- exact `kilo_micro_array_string_store` microstat does not regress
- `tools/checks/current_state_pointer_guard.sh`

Implementation:
- `array_string_store_micro_seed_route` now exports `next_text_window_start=2` and `next_text_window_len=16`.
- The exact emitter validates those metadata fields and emits a vector text-state update from the MIR-owned follow-up substring window.
- The old loop-body `slot + 2 -> text` copy is gone; the post-change asm uses `vpalignr` after storing the current text into the selected slot.

Result:
- `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_array_string_store 1 3`
  - `C 10 ms / Ny AOT 5 ms`
  - `ny_aot_instr=11670690`, `ny_aot_cycles=9512639`
- `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_aot_asm.sh kilo_micro_array_string_store 'ny_main' 3`
  - `ny_main` still owns the hot loop
  - top local owner is now the slot store plus suffix stores; the previous `slot + 2 -> text` copy is absent
- Guard held: no route widening, no public ABI, no runtime ownership, and the bridge remains temporary exact metadata.

## 137x-H17 Exact Text Terminator Store Seam

Status: closed.

Scope:
- shrink only the temporary `kilo_micro_array_string_store` exact bridge
- remove the loop-body terminator store for loop-carried `text`
- keep selected slot terminator stores unchanged
- keep route legality, metadata, public ABI, and runtime ownership unchanged

Perf-first baseline:
- H16 post-change `kilo_micro_array_string_store = C 10 ms / Ny AOT 5 ms`
- `ny_aot_instr=11670690`, `ny_aot_cycles=9512639`
- H16 asm still shows `movb $0, text+16` in the hot loop

Acceptance:
- `cargo test array_string_store_micro_seed --lib`
- `bash tools/perf/build_perf_release.sh`
- `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_array_store_string_contract.sh`
- exact `kilo_micro_array_string_store` microstat does not regress
- `tools/checks/current_state_pointer_guard.sh`

Implementation:
- remove only the loop-body `store i8 0` for `text+16`
- keep selected array-slot terminator stores unchanged
- leave `array_string_store_micro_seed_route` metadata and legality untouched

Result:
- `kilo_micro_array_string_store = C 10 ms / Ny AOT 5 ms`
- `ny_aot_instr=10870861`, `ny_aot_cycles=9526782`
- regenerated asm no longer contains the loop-body `movb $0, text+16`
- remaining local owners are slot vector store, suffix store, and `vpalignr`
- Guard held: no route widening, no public ABI, no runtime ownership, and the exact bridge remains temporary metadata.

## 137x-H18 Exact Loop-Carried Text SSA Seam

Status: closed.

Scope:
- shrink only the temporary `kilo_micro_array_string_store` exact bridge
- carry loop-carried `text` as an LLVM SSA vector phi instead of stack memory
- keep array slot stores, suffix stores, route legality, public ABI, and runtime ownership unchanged
- do not perform array-store deadness / no-escape removal in this slice

Perf-first baseline:
- `kilo_micro_array_string_store = C 9 ms / Ny AOT 5 ms`
- `ny_aot_instr=10870942`, `ny_aot_cycles=9536178`, `ny_aot_ipc=1.14`
- asm: `ny_main` 93.62%; local owners are slot vector store, loop-carried text state store, loop increment, suffix stores

Decision:
- H16 metadata already proves the next loop-carried text window.
- The backend may choose an SSA vector carrier for that already-proven value.
- This is a backend-local physical carrier change, not new route legality.

Acceptance:
- `tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test array_string_store_micro_seed --lib`
- `bash tools/perf/build_perf_release.sh`
- `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_array_store_string_contract.sh`
- exact `kilo_micro_array_string_store` microstat and asm confirm the stack `text.ptr` loop store/load is gone

Implementation:
- load the seed text once as `<16 x i8>`
- make `loop` carry `%text.cur` as an LLVM SSA vector phi
- update `%text.next` via the existing H16 `next_text_window` shuffle
- keep selected array slot writes unchanged

Result:
- `kilo_micro_array_string_store = C 10 ms / Ny AOT 4 ms`
- `ny_aot_instr=9270464`, `ny_aot_cycles=2343815`, `ny_aot_ipc=3.96`
- asm now carries text in `%xmm0`; the stack `text.ptr` loop load/store is gone
- remaining local owners are slot vector store, slot terminator/suffix stores, sum update, and `vpalignr`
- Guard held: no route widening, no public ABI, no runtime ownership, and no array-store deadness removal.

## 137x-H19 Whole IndexOf Slot-Consumer Liveness Seam

Status: closed.

Owner card:
- front: `kilo_kernel_small_hk` whole direct pure-first
- failure mode: work explosion
- current owner: `TextLane slot -> boxed StringBox object` through unused `array.get_hi` before `array.string_indexof_hisi`
- hot transition: `ArrayBox::boxed_from_text` / memmove / malloc
- next seam: MIR `array_text_observer_routes` must classify same-slot const suffix store as a slot-capable consumer of the get source
- reject seam: do not delete array stores and do not infer source liveness in `.inc`

Perf-first evidence:
- `kilo_kernel_small_hk = C 82 ms / Ny AOT 6653 ms`
- perf top: `__memmove_avx512_unaligned_erms` 50.13%, `_int_malloc` 17.28%, `ArrayBox::boxed_from_text` 14.09%
- `ny_main` shows `array.get_hi` immediately before `array.string_indexof_hisi`
- MIR metadata currently exports one `array_text_observer_routes` route with `keep_get_live=true` because the same source feeds `current + "ln"`

Decision:
- same-slot `current + const_suffix -> lines.set(j, ...)` is a slot-capable consumer when it targets the same array/index as the observer get
- the source does not need public object materialization for that use
- MIR owns this consumer coverage by setting `keep_get_live=false`; backend continues to consume metadata only

Acceptance:
- `tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test array_text_observer --lib`
- `bash tools/perf/build_perf_release.sh`
- trace bundle or lowered IR confirms the row-scan `array.get_hi` before `array.string_indexof_hisi` is gone
- whole `kilo_kernel_small_hk` rerun improves or, if still blocked, records the next owner

Result:
- MIR route now exports `array_text_observer_routes[0].keep_get_live=false` for the whole row-scan case.
- Lowered IR emits `nyash.array.string_indexof_hisi` directly and no longer emits the row-scan `nyash.array.get_hi` / `nyash.array.slot_load_hi` materialization before it.
- `PERF_AOT_DIRECT_ONLY=0 NYASH_LLVM_SKIP_BUILD=1 tools/perf/run_kilo_hk_bench.sh strict 1 3`
  - `kilo_kernel_small_hk = C 82 ms / Ny AOT 28 ms`
  - previous owner baseline was `C 82 ms / Ny AOT 6653 ms`
  - parity stayed `ok`
- Guard held: `.inc` remains a metadata consumer; no array stores were deleted and no runtime legality/provenance inference was added.

## 137x-H20 Meso Substring Concat Len Fusion Seam

Status: closed.

Owner card:
- front: `kilo_meso_substring_concat_len`
- failure mode: work explosion in runtime helper calls
- current owner: concat length observer lowers to two `nyash.string.substring_len_hii` calls even when the slices partition one const source
- hot transition: virtual text view length crosses the runtime handle registry boundary; `perf annotate` shows `lock cmpxchg` / `lock xadd` in `nyash.string.substring_len_hii`
- next seam: MIR string-corridor fusion must fold `len(left + const + right)` for complementary substring slices back to `source_len + const_len`
- reject seam: do not add runtime caches, do not revive `.inc` exact seed matching, and do not move legality into helper names

Perf-first evidence:
- `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 tools/perf/run_kilo_kernel_split_ladder.sh 1 3`
  - `kilo_meso_substring_concat_len = C 3 ms / Ny AOT 8 ms`
  - `ny_aot_instr=66356109`, `ny_aot_cycles=21046448`
- `bash tools/perf/bench_micro_aot_asm.sh kilo_meso_substring_concat_len 'nyash.string.substring_len_hii' 3`
  - top owner: `nyash.string.substring_len_hii` 98.40%
  - helper annotate samples are on host-handle registry entry/exit atomics, not useful string bytes work
- lowered IR currently emits two calls:
  - `nyash.string.substring_len_hii(seed, 0, split)`
  - `nyash.string.substring_len_hii(seed, split, len)`

Decision:
- For complementary substring slices over the same source, `len(left + const + right)` is a MIR-owned arithmetic fact.
- If the concat length observer is the only consumer, the substring view producer calls are also dead and may be removed by the corridor plan.
- Const-source length can witness the source length when the end value equals the const string byte length; no runtime helper or backend seed matcher is needed.

Acceptance:
- `tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- targeted string-corridor tests prove `bench_kilo_meso_substring_concat_len` has no loop `substring_len_hii` or substring materialization calls
- `bash tools/perf/build_perf_release.sh`
- rerun `kilo_meso_substring_concat_len` and record the next owner if it remains blocked

Result:
- `cargo test string_corridor_sink::tests::benchmarks --lib`
  - added and passed `benchmark_meso_substring_concat_len_compiles_to_arithmetic_len`
- lowered IR has no `substring_len_hii`, no `substring_hii`, and `mir_calls=0` for `kilo_meso_substring_concat_len`
- `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_len 1 3`
  - `kilo_meso_substring_concat_len = C 3 ms / Ny AOT 3 ms`
  - `ny_aot_instr=1190457`, `ny_aot_cycles=918004`
- split ladder confirmation:
  - `kilo_meso_substring_concat_len = C 3 ms / Ny AOT 3 ms`
  - `ny_aot_instr=1190204`, `ny_aot_cycles=909543`
- Guard held: no runtime cache, no `.inc` exact seed revival, and no helper-name legality shift.

## 137x-H21 Meso Array Text Loopcarry Len/Store Seam

Status: closed.

Owner card:
- front: `kilo_meso_substring_concat_array_set_loopcarry`
- failure mode: work explosion in runtime array text helper pair
- current owner: loop calls `nyash.array.string_len_hi` and `nyash.array.string_insert_mid_subrange_store_hisiiii` every iteration
- hot transition: array slot length is read through a runtime helper immediately before same-slot insert-mid subrange store
- next seam: MIR/lowering should avoid the separate length helper when the same slot store already has a known resulting length or can carry the previous slot length as scalar state
- reject seam: do not delete array stores, do not add semantic cache to `ArrayBox`, and do not infer same-slot legality in `.inc`

Perf-first evidence:
- `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 tools/perf/run_kilo_kernel_split_ladder.sh 1 3`
  - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 8 ms`
  - `ny_aot_instr=72914136`, `ny_aot_cycles=22417148`
- `bash tools/perf/bench_micro_aot_asm.sh kilo_meso_substring_concat_array_set_loopcarry 'ny_main' 3`
  - `nyash.array.string_len_hi` 54.74%
  - `array_string_insert_const_mid_subrange_by_index_store_same_slot_str` 43.77%
- asm loop still performs:
  - `call nyash.array.string_len_hi`
  - arithmetic from returned length
  - `call nyash.array.string_insert_mid_subrange_store_hisiiii`

Acceptance:
- write a front/failure/owner/seam/reject card before editing
- prove lowered IR removes or reduces the standalone `array.string_len_hi` loop call without deleting the same-slot store
- rerun `kilo_meso_substring_concat_array_set_loopcarry`

Decision:
- The `get -> length -> split -> substring pair -> substring_concat3_hhhii -> same-slot set -> end-start` shape is a MIR-owned loopcarry len-store route.
- `.inc` must consume `array_text_loopcarry_len_store_routes` and emit the existing `nyash.array.string_insert_mid_subrange_len_store_hisi` helper; it must not rediscover route legality from raw JSON shape.
- The same-slot store remains live. The optimization only fuses the observer/store helper pair.

Result:
- Added a benchmark contract proving `bench_kilo_meso_substring_concat_array_set_loopcarry` exposes one `array_text_loopcarry_len_store_routes` entry.
- Trace bundle `target/perf_state/optimization_bundle/137x-h21-loopcarry-route-after`:
  - route trace hits `array_string_loopcarry_len_store_window` with reason `mir_route_plan`
  - MIR JSON has `result_len_value=52`, `middle_length=2`
  - lowered loop body calls only `nyash.array.string_insert_mid_subrange_len_store_hisi`
  - the hot loop no longer calls standalone `nyash.array.string_len_hi` or `nyash.array.string_insert_mid_subrange_store_hisiiii`
- `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_array_set_loopcarry 1 3`
  - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 6 ms`
  - `ny_aot_instr=40155587`, `ny_aot_cycles=12429857`
- split ladder confirmation:
  - `kilo_meso_substring_concat_array_set_loopcarry = C 4 ms / Ny AOT 6 ms`
  - `ny_aot_instr=40154852`, `ny_aot_cycles=12350248`
  - `kilo_kernel_small_hk = C 81 ms / Ny AOT 26 ms`, parity `ok`
- Guard held: route legality moved to MIR metadata; `.inc` remains a metadata consumer and array stores remain live.

## 137x-H22 Array Text Len-Store Helper Residency Seam

Status: closed; local helper surgery rejected.

Owner card:
- front: `kilo_meso_substring_concat_array_set_loopcarry`
- failure mode: remaining runtime helper residence/mutation cost
- current owner: after H21, the loop body is one `nyash.array.string_insert_mid_subrange_len_store_hisi` call
- hot transition: the helper still enters the generic array-handle / text-slot mutation path every iteration
- next seam: reduce helper residency overhead while keeping semantic legality in MIR and mechanics in runtime
- reject seam: do not add semantic search/result caches, do not delete array stores, and do not move route legality into runtime

Perf-first evidence:
- `PERF_AOT_DIRECT_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 tools/perf/run_kilo_kernel_split_ladder.sh 1 3`
  - `kilo_meso_substring_concat_array_set_loopcarry = C 4 ms / Ny AOT 6 ms`
  - `ny_aot_instr=40154852`, `ny_aot_cycles=12350248`
- `bash tools/perf/bench_micro_aot_asm.sh kilo_meso_substring_concat_array_set_loopcarry 'nyash.array.string_insert_mid_subrange_len_store_hisi' 3`
  - top owner: `array_string_insert_const_mid_subrange_len_by_index_store_same_slot_str` closure 96.51%
- `bash tools/perf/bench_micro_aot_asm.sh kilo_meso_substring_concat_array_set_loopcarry 'array_string_insert_const_mid_subrange_len_by_index_store_same_slot_str' 3`
  - top owner: same closure 85.22%
  - secondary observed cost: `__strncmp_evex` 11.28%

Acceptance:
- keep the H21 MIR route as the only legality owner
- reduce the helper residency/mutation cost without adding runtime semantic cache
- rerun `kilo_meso_substring_concat_array_set_loopcarry` and record the next owner if it remains above C

Rejected probes:
- small-overlap copy in `try_update_insert_const_mid_subrange_same_len_in_place`
  - result: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 6 ms`
  - `ny_aot_instr=41954192`, `ny_aot_cycles=12355443`
  - reading: replaced small `memmove` calls but increased instruction count; not a keeper
  - code is reverted
- fast-path return of pre-update `source_len`
  - result: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 6 ms`
  - `ny_aot_instr=40154996`, `ny_aot_cycles=12377624`
  - reading: removes only a post-update length read; no owner move
  - code is reverted
- route helper update through `slot_update_text_raw` only
  - result: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 6 ms`
  - `ny_aot_instr=50413920`, `ny_aot_cycles=13248445`
  - reading: the resident-first split is necessary; unifying the helper entry regresses the hot path
  - code is reverted

Current verdict:
- H21 already removed the MIR/backend route work explosion. The remaining gap is not a small string-copy or helper-branch issue.
- The owner remains runtime-private array text residence mutation, including the uncontended write-lock / slot update substrate.
- Do not reopen H22 with local helper surgery unless a fresh `perf annotate` shows a new intra-helper block above the lock/residence transition.
- The next keeper candidate needs a structural residence/session design, or this seam should be deferred to a later allocator/residence pilot with a smaller rollback surface.

Closeout:
- H22 is closed as a no-keeper local helper card.
- The next owner hypothesis moves to a separate runtime-private residence/session card.
- This avoids changing the card meaning from helper-local surgery to lock/session architecture mid-stream.

## 137x-H23 Array Text Write Transaction Pilot

Status: rejected/closed.

Purpose:
- Test a narrow runtime-private residence/session substrate for one array text slot.
- Keep H21's MIR route metadata as the legality owner.
- Keep `.inc` as metadata consumer / emit-only.
- Keep Rust runtime as executor/mechanics owner only.

Owner card:
- front: `kilo_meso_substring_concat_array_set_loopcarry`
- failure mode: remaining `C 3-4 ms / Ny AOT 6 ms` gap after H21/H22
- current owner:
  - H21 removed route work explosion
  - H22 rejected local helper surgery
  - remaining hot transition is runtime-private array text residence mutation / uncontended write-lock substrate
- next seam: introduce or prototype a helper-local `ArrayTextWriteTxn` / `ArrayTextSlotSession` shape that resolves the array slot and write residence once inside a runtime-private boundary
- reject seam:
  - no MIR legality change unless a later block-local session contract explicitly needs it
  - no `.inc` shape rediscovery
  - no runtime legality/provenance inference
  - no search-result cache / semantic cache
  - no lock held across publish / objectize / generic fallback / host handle calls

Worker inventory:
- active hot helper:
  - `crates/nyash_kernel/src/plugin/array_string_slot_write.rs`
  - `array_string_insert_const_mid_subrange_len_by_index_store_same_slot_str(...)`
  - current shape is `with_array_box(handle, |arr| arr.slot_update_text_resident_raw(...).or_else(|| arr.slot_update_text_raw(...)))`
- storage substrate:
  - `src/boxes/array/ops/text.rs`
  - `slot_update_text_resident_raw(...)` owns text-resident mutation under one array write guard
  - `slot_update_text_raw(...)` owns compatibility fallback and may promote boxed string-like slots
- handle/cache substrate:
  - `crates/nyash_kernel/src/plugin/array_handle_cache.rs`
  - preserve `valid_handle_idx`, cached `ArrayBox`, and drop-epoch invalidation behavior
- observe substrate:
  - `crates/nyash_kernel/src/observe/README.md`
  - `NYASH_PERF_COUNTERS` is the only counter gate for H23
  - new H23 counters, if needed, must follow the observe counter change rule and stay evidence-only

Design boundary:
- Allowed:
  - helper-local transaction only
  - acquire the array text write guard once
  - resolve the slot once
  - expose transient resident text mutation mechanics inside the helper
  - commit within the helper boundary
- Placement:
  - `src/boxes/array/ops/text.rs` remains the owner for storage-layout and write-guard mechanics because `ArrayStorage` and the guard are private substrate
  - `crates/nyash_kernel/src/plugin/array_string_slot_write.rs` remains the owner for runtime-private string edit mechanics and observe calls
  - an optional small `crates/nyash_kernel/src/plugin/array_text_write_txn.rs` wrapper may orchestrate handle acquisition and delegate to the ArrayBox substrate, but it must not own storage layout or legality
- Candidate API shape:
  - ArrayBox substrate: one narrow text-slot writer that distinguishes resident hit / fallback / miss without exposing `ArrayStorage`
  - kernel wrapper: one helper-local `ArrayTextWriteTxn` / `ArrayTextSlotSession` only if the probe proves repeated handle/slot/session mechanics are the owner
  - no public ABI, no `.hako` surface, no MIR-facing API
- Deferred:
  - block-local session
  - loop-wide session
  - allocator / arena pilot
- Forbidden in this card:
  - publishing while a write transaction is live
  - extending transaction lifetime across loop backedge, safepoint, generic object call, panic/unwind, or externally visible alias boundary
  - making helper symbol names semantic truth
  - changing `ArrayStorage` layout or promotion rules as part of the transaction pilot

First probe:
- Build a perf-observe binary:
  - `bash tools/perf/build_perf_observe_release.sh`
- Capture attribution counters from direct perf-observe AOT executables; do not use `bench_micro_c_vs_aot_stat.sh` for counter capture because it suppresses child stderr:
  - `source tools/perf/lib/aot_helpers.sh; perf_emit_and_build_aot_exe "$PWD" "$PWD/target/release/hakorune" "$PWD/benchmarks/bench_kilo_meso_substring_concat_array_set_loopcarry.hako" "$PWD/target/perf_state/h23_loopcarry.perf_observe.exe"; NYASH_PERF_COUNTERS=1 NYASH_GC_MODE=off NYASH_SCHED_POLL_IN_SAFEPOINT=0 NYASH_SKIP_TOML_ENV=1 target/perf_state/h23_loopcarry.perf_observe.exe > target/perf_state/h23_loopcarry.out 2> target/perf_state/h23_loopcarry.err`
  - `source tools/perf/lib/aot_helpers.sh; perf_emit_and_build_aot_exe "$PWD" "$PWD/target/release/hakorune" "$PWD/benchmarks/bench_kilo_meso_substring_concat_array_set.hako" "$PWD/target/perf_state/h23_noloopcarry.perf_observe.exe"; NYASH_PERF_COUNTERS=1 NYASH_GC_MODE=off NYASH_SCHED_POLL_IN_SAFEPOINT=0 NYASH_SKIP_TOML_ENV=1 target/perf_state/h23_noloopcarry.perf_observe.exe > target/perf_state/h23_noloopcarry.out 2> target/perf_state/h23_noloopcarry.err`
- Rejected capture path:
  - direct `target/release/hakorune --backend vm ...` currently fails this benchmark with `Unknown: nyash.string.insert_hsi`; keep H23a on the AOT perf-observe executable path until VM route parity is fixed separately.
- Read buckets:
  - guard/handle candidate: `store.array.str.total`, cache hit/miss, and `lookup.*`
  - slot resolve candidate: `plan.source_kind_*`, `plan.slot_kind_*`, `lookup.registry_slot_read`, and `lookup.caller_latest_fresh_tag`
  - storage dispatch candidate: resident text hit versus fallback through `slot_update_text_resident_raw(...)` / `slot_update_text_raw(...)`
  - commit candidate: `existing_slot`, `append_slot`, and `source_store`
- If the first probe is ambiguous, add only temporary `NYASH_PERF_COUNTERS`-gated counters:
  - split `slot_update_text_resident_raw(...)` hit/miss
  - split `slot_update_text_raw(...)` text-resident / boxed-string / miss
  - optionally bracket `with_array_box(...)` in the hot helper
  - no new env var and no unconditional logging

H23a/H23b observation (2026-04-21):
- Command results:
  - `bash tools/perf/build_perf_observe_release.sh`: pass after fixing a perf-observe-only brace mismatch in `observe/backend/tls/flush.rs`.
  - direct VM counter capture: rejected, `Unknown: nyash.string.insert_hsi`.
  - perf-observe AOT loopcarry stdout: `Result: 2880064`, exit `64`.
  - perf-observe AOT control stdout: `Result: 3240064`, exit `128`.
  - release timing after returning to non-observe build: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 6 ms`, `ny_aot_instr=40329994`, `ny_aot_cycles=12403515`.
  - release asm top: hot helper closure `array_string_insert_const_mid_subrange_len_by_index_store_same_slot_str(...)` at `94.86%`.
- Split counter result:
  - loopcarry: `store.array.str total=180000`, `existing_slot=180000`, `source_store=180000`, `update_text_resident_hit=179999`, `update_text_resident_miss=1`, `update_text_fallback_hit=1`, `update_text_fallback_miss=0`.
  - non-loopcarry control: `store.array.str total=180000`, `existing_slot=180000`, `source_store=180000`, but H23 hot-helper update counters are all `0`; this control uses the non-loopcarry store path, not the H23 len-store helper.
- Reading:
  - fallback/promotion is not the active owner for loopcarry; the steady-state hot path is already text-resident.
  - next implementation must not widen MIR or `.inc`; H23b may only compact the runtime-private resident mutation boundary for the one hot helper.
- H23b rejected prototype:
  - attempted shape: one `ArrayBox` resident/fallback/miss outcome writer, keeping `ArrayStorage` private and letting the hot helper consume only the outcome.
  - result: `ny_aot_instr=45910743`, `ny_aot_cycles=12677425`, `ny_aot_ms=6`.
  - baseline before H23b: `ny_aot_instr=40329994`, `ny_aot_cycles=12403515`, `ny_aot_ms=6`.
  - verdict: non-keeper; instruction count regressed and wall time did not improve.
  - code reverted; do not retry single-method resident/fallback unification without a fresh `perf annotate` block showing a different owner.

H23 closeout:
- Status: rejected/closed.
- Final reading:
  - helper-local transaction compaction is not enough; the remaining gap is not fallback/promotion.
  - resident-first split remains necessary.
  - opening a broader block-local or loop-wide session would require a new MIR-owned lifetime/alias contract, so it is out of scope for H23.
  - next card must reclassify the owner from fresh `perf annotate` evidence before choosing between lock/session contract, lower-level array substrate, or allocator/copy work.

Acceptance:
- First probe must measure whether the active owner is actually write guard acquire / slot resolve / storage dispatch / commit.
- If the probe does not show those substrate costs as the owner, close H23 without implementation.
- If the probe confirms substrate ownership, implement the smallest helper-local transaction surface and rerun:
  - `kilo_meso_substring_concat_array_set_loopcarry`
  - split ladder including `kilo_kernel_small_hk`
- Keeper requires instruction/cycle improvement or a clear owner move without regressing exact/micro guards.
- Non-win result closes H23 and sends remaining cost to a later allocator/residence pilot only if `memmove` / `malloc` / `_int_malloc` becomes structural evidence.

## 137x-H24 Post-H23 Resident Mutation Owner Reclassification

Status: closed.

Purpose:
- Reclassify the remaining `kilo_meso_substring_concat_array_set_loopcarry` owner after H23 rejected helper-local transaction compaction.
- Keep the next optimization owner-first: no new API shape, transaction lifetime, cache, or allocator work until fresh top-block evidence identifies the active owner family.

Owner card:
- front: `kilo_meso_substring_concat_array_set_loopcarry`
- failure mode: remaining `C 3 ms / Ny AOT 6 ms` gap after H21/H22/H23
- current rejected seams:
  - H22 local helper surgery: rejected
  - H23 helper-local resident/fallback outcome writer: rejected
  - fallback/promotion: not owner (`update_text_resident_hit=179999`)
- current owner hypothesis:
  - runtime-private resident mutation remains hot, but helper-local compaction is insufficient
  - next evidence must distinguish string edit bytes work, uncontended lock mechanics, handle/cache call boundary, and generated call overhead

Required first step:
- Rebuild release after reverting H23b non-keeper code:
  - `bash tools/perf/build_perf_release.sh`
- Reconfirm timing:
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_array_set_loopcarry 1 3`
- Capture current top block:
  - `bash tools/perf/bench_micro_aot_asm.sh kilo_meso_substring_concat_array_set_loopcarry '' 20`

Result (2026-04-21):
- `bash tools/perf/build_perf_release.sh`: pass.
- `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_array_set_loopcarry 1 3`:
  - `c_ms=3`, `ny_aot_ms=6`, `ny_aot_instr=40330160`, `ny_aot_cycles=12366672`.
- `bash tools/perf/bench_micro_aot_asm.sh kilo_meso_substring_concat_array_set_loopcarry '' 20`:
  - top owner remains `array_string_insert_const_mid_subrange_len_by_index_store_same_slot_str` closure at `97.51%`.
- closure IP probe:
  - `413483`: 38 samples, immediately after the write-lock acquire `lock cmpxchg`.
  - `4138e5`: 33 samples, immediately after the write-lock release `lock cmpxchg`.
  - `413454`: 1 sample, prologue/load.
- Reading:
  - H24 confirms guard mechanics, not fallback/promotion and not the byte-edit/memmove body, as the active owner.
  - One-helper-per-iteration remains the cost shape; helper-local compaction cannot remove the acquire/release pair.
  - Next work must be a MIR-owned residence/session contract. Runtime may execute the mechanics, but it must not infer legality.

Allowed next seams:
- If hot block is inside the in-place byte edit, open a narrow string-edit kernel card.
- If hot block is lock acquire/release or guard mechanics, open a new MIR-owned block-local session contract card; do not smuggle it into runtime alone.
- If hot block is handle/cache or extern call boundary, open a backend/call-boundary card.
- If `memmove` / malloc family becomes dominant, send to allocator/copy pilot with evidence.

Forbidden:
- Reopening single-method resident/fallback unification.
- Loop-wide session without a MIR lifetime/alias contract.
- Runtime legality/provenance inference.
- `.inc` shape rediscovery.

## 137x-H25 Array Text Residence Session Contract

Status: active.

Purpose:
- Remove the per-iteration `ArrayStorage` write-lock acquire/release from the loopcarry route only when MIR proves a residence session is legal.
- Keep `.inc` as metadata consumer and Rust as executor-only: MIR owns session eligibility, lifetime, alias, and publication-boundary facts.

Owner card:
- front: `kilo_meso_substring_concat_array_set_loopcarry`
- failure mode: remaining `C 3 ms / Ny AOT 6 ms` gap after H21/H22/H23/H24.
- current owner: uncontended write-lock acquire/release inside the fused loopcarry helper.
- rejected seams:
  - H22 local helper surgery
  - H23 helper-local resident/fallback compaction
  - H24 byte-edit/memmove owner hypothesis

Contract shape:
- MIR metadata may expose a `array_text_residence_sessions` route only when all are true:
  - one array root and one text-resident slot family are used by the selected loop region
  - the selected region has no publish/objectize/generic object call boundary while the session is live
  - all covered operations are slot-capable and already represented by MIR-owned route plans
  - the session has explicit begin/end scope and does not rely on helper names as truth
- Backend may:
  - read the session metadata
  - emit begin/update/end calls
  - skip only instructions covered by the MIR route
- Runtime may:
  - acquire the write guard once
  - resolve text storage/slot under the guard
  - perform repeated resident text mutations
  - release the guard at the MIR-selected end boundary

Forbidden:
- Runtime deciding session legality from residence state.
- `.inc` rediscovering loop/session shape from raw MIR JSON.
- Holding a session across publish/objectize/generic fallback, externally visible alias calls, panic/unwind, or unknown side-effect calls.
- Adding benchmark-specific whole-loop helpers.

First implementation slice:
- Add metadata-only MIR session eligibility for the existing loopcarry len-store route.
- Emit JSON and a unit test proving the benchmark exposes exactly one session route.
- Do not change lowering/runtime behavior until the metadata contract is visible and tested.

H25a result (2026-04-21):
- Added metadata-only `array_text_residence_sessions`.
- The existing loopcarry len-store route now exposes one session candidate for the benchmark:
  - `scope=loop_backedge_single_body`
  - `proof=loopcarry_len_store_only`
  - `consumer_capability=slot_text_len_store_session`
  - `publication_boundary=none`
- Contract guard:
  - session eligibility is derived only from MIR-owned `array_text_loopcarry_len_store_routes`.
  - uncovered loop-body instructions must be pure loop bookkeeping.
  - `.inc` and runtime behavior are unchanged in H25a.
- Validation:
  - `cargo check -q`
  - `cargo test -q benchmark_meso_substring_concat_array_set_loopcarry_has_len_store_route -- --nocapture`
  - `cargo run -q --bin hakorune -- --emit-mir-json target/perf_state/h25_loopcarry.mir.json benchmarks/bench_kilo_meso_substring_concat_array_set_loopcarry.hako` emits one `array_text_residence_sessions` entry.

H25b result (2026-04-21):
- Worker design check rejected implementing long-lived runtime begin/end guard calls directly:
  - runtime write guards must not leak across C ABI call boundaries.
  - `.inc` must not infer preheader/exit placement from raw CFG.
- Extended `array_text_residence_sessions` with MIR-owned placement metadata:
  - `begin_block` / `begin_to_header_block` / `begin_placement=before_preheader_jump`
  - `update_block` / `update_instruction_index` / `update_placement=route_instruction`
  - `end_block` / `end_placement=exit_block_entry`
  - `skip_instruction_indices`
- Behavior remains unchanged. H25c may consume the metadata in `.inc` and add a runtime-private executor surface without making `.inc` a planner.

H25c.1 result (2026-04-21):
- Small vocabulary cleanup: active array/text `.inc` readers are named
  `*_route_metadata`; `plan` stays MIR-internal.
- `hako_llvmc_ffi_generic_method_get_window.inc` now reads and validates
  `array_text_residence_sessions` placement metadata.
- `hako_llvmc_ffi_generic_method_get_lowering.inc` prefers that session
  metadata and maps it to the existing loopcarry update helper.
- Behavior remains unchanged: no begin/end emission and no runtime session
  helper yet.

H25c.2a substrate-only landed:
- Landed a closure-scoped runtime substrate without claiming a perf keeper.
- Added `ArrayTextSlotSession` under ArrayBox text mechanics and kept existing
  raw update methods as compatibility adapters.
- Added kernel-private `ArrayTextWriteTxn` glue for handle lookup and
  resident-first/fallback outcome mapping.
- No exported `nyash.array.*` symbols, session handle tables, new env vars, or
  guard-bearing C ABI begin/end calls were added.
- Substrate gates:
  - `cargo test -q --lib slot_update_text`
  - `cargo test -q --lib array_text`
  - `cargo check -q -p nyash_kernel`

H25c.2b single-call executor design gate:
- Verdict: closed as clean non-keeper.
- The current metadata can select the update instruction and keep `.inc`
  metadata-consumer-only, but it still emits one Rust call per iteration and
  does not remove the measured write-lock acquire/release owner.
- Keep H25c.2b as contract closeout. Do not claim perf keeper from this slice.

H25c.2c single-region executor contract:
- Open the next keeper path as a nested `executor_contract` under
  `array_text_residence_sessions`; do not add a new sibling plan family.
- MIR owns proof region, publication boundary, effects, consumer capability,
  and any materialization/fallback policy.
- `.inc` may emit one call from the MIR-selected begin site and skip the covered
  region, but must not rediscover preheader/exit/PHI/legality from raw CFG.
- Runtime may implement a one-call RAII executor that acquires the array write
  guard once and drops it before returning; no session table, no begin/end ABI,
  no hidden legality.
- First implementation slice landed as metadata-only:
  - `ArrayTextResidenceSessionRoute.executor_contract` now carries
    `single_region_executor`, `loop_backedge_single_body`,
    `publication_boundary=none`, `array_lane_text_cell`, `store.cell`,
    `length_only_result_carry`, `sink_store`, `length_only`, and
    `text_resident_or_stringlike_slot`.
  - MIR JSON emits the nested contract and the loopcarry route test asserts it.
  - No behavior change yet; next slice is `.inc` reader validation, followed by
    any missing MIR loop/PHI/exit mapping before a region replacement.
- Second implementation slice landed as backend validation only:
  - `hako_llvmc_ffi_generic_method_get_window.inc` now requires the nested
    `executor_contract` fields before accepting the residence-session metadata.
  - The active backend trace remains metadata-owned:
    `array_text_residence_session hit mir_route_metadata`.
  - Lowering still emits the existing per-iteration fused helper; the keeper
    path is blocked on MIR-owned loop/PHI/exit mapping before region
    replacement.
- Third implementation slice landed as loop-region mapping:
  - `executor_contract.region_mapping` now carries loop index PHI/init/next,
    loop bound const, accumulator PHI/init/next, exit accumulator value, row
    index, and row modulus const.
  - The backend reader validates the mapping presence and the minimum
    cross-field invariants without deriving CFG shape.
  - Lowering still emits the existing per-iteration fused helper. The next
    open problem is replacing the header/body/PHI/exit-use region without
    redefining SSA values.
- Fourth implementation slice landed as backend region replacement:
  - MIR additionally proves `loop_index_initial_const=0` and
    `accumulator_initial_const=0`; the runtime executor does not infer these.
  - `.inc` now matches the MIR-selected begin block, emits one
    `nyash.array.string_insert_mid_subrange_len_store_region_hiisi` call, and
    skips the covered header/body region without redefining PHI values.
  - Runtime executes the proven loop inside
    `ArrayBox::slot_text_region_update_sum_raw(...)`; the write guard stays
    inside one Rust call stack, with no session table and no begin/end ABI.
  - Route trace:
    `stage=array_text_residence_region_begin result=hit reason=mir_region_mapping`.
  - Keeper probe:
    `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 5 ms`,
    `ny_aot_instr=28630426`, `ny_aot_cycles=7033574`.
  - Latest asm top:
    `slot_text_region_update_sum_raw` closure `79.54%`,
    `__memmove_avx512_unaligned_erms` `9.74%`, region store closure `6.16%`.
  - Verdict: H25c.2c/H25c.3 are closed as a partial keeper. The next owner is
    H25d region executor inner mutation/copy; re-annotate before editing.

H25d region executor inner mutation result:

- H25d.1 landed:
  - `ArrayBox::slot_text_region_update_sum_raw(...)` now loops directly over
    `ArrayStorage::Text(Vec<String>)` after taking the single write guard.
  - The compatible boxed/stringlike fallback remains unchanged.
  - Probe: `ny_aot_instr=24851120`, `ny_aot_cycles=6700078`, `Ny AOT 5 ms`.
- H25d.2 landed:
  - `update_insert_const_mid_subrange_len_value(...)` is split into a hot
    fixed in-place path and a cold semantic materialization fallback.
  - UTF-8 boundary checks remain in the hot path; no ASCII assumption is added.
  - Final repeated stat:
    `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 3 ms`,
    `ny_aot_instr=16570267`, `ny_aot_cycles=3471656`.
  - Final asm top:
    region store mutation closure `52.65%`,
    `__memmove_avx512_unaligned_erms` `35.67%`.
- Rejected probes:
  - H25d.3 manual byte moves regressed to
    `ny_aot_instr=22511003`, `ny_aot_cycles=4765539`, `Ny AOT 4 ms`; reverted.
  - H25d.4 `observe::enabled()` hoist regressed to
    `ny_aot_instr=22510404`, `ny_aot_cycles=4773551`, `Ny AOT 4 ms`; reverted.
- Verdict:
  - H25d.1/H25d.2 are keepers.
  - H25d.5 closes the residual memmove / mutation owner decision:
    H25d.3/H25d.4 both regressed, so do not reopen local byte-copy or observe
    surgery without new MIR proof.
  - H25e post-parity owner refresh selected the next code owner:
    whole-front inner scan observer + conditional same-slot suffix store.
  - The next slice is H26 array text observer-store region contract.
  - Do not add source-length or ASCII assumptions unless MIR provides an
    explicit generic proof.

H25e post-parity owner refresh:

- exact `kilo_micro_array_string_store`:
  - stat: `C 10 ms / Ny AOT 4 ms`,
    `ny_aot_instr=9265624`, `ny_aot_cycles=2385663`
  - asm top: `ny_main` `76.27%`; libc `memmove` is below 1%
- middle `kilo_meso_substring_concat_array_set_loopcarry`:
  - stat: `C 3 ms / Ny AOT 4 ms`,
    `ny_aot_instr=16570861`, `ny_aot_cycles=3387096`
  - asm top: region mutation closure `61.25%`,
    `__memmove_avx512_unaligned_erms` `25.60%`
  - verdict: H25d remains closed; residual memmove percentage alone is not a
    new owner because local copy/observe probes regressed
- whole `kilo_kernel_small`:
  - stat: `C 81 ms / Ny AOT 20 ms`,
    `ny_aot_instr=232160997`, `ny_aot_cycles=83942461`
  - asm top: `memchr::...find_avx2` `34.56%`,
    `with_array_text_write_txn` closure `29.63%`,
    `LocalKey::with` `12.69%`,
    `__memmove_avx512_unaligned_erms` `6.75%`,
    `nyash.array.string_len_hi` `6.33%`
  - MIR evidence: `array_text_observer_routes` already records the
    `array_get_receiver_indexof` route with `consumer_shape=found_predicate`,
    `publication_boundary=none`, and const needle `"line"`
  - H26 must extend the existing observer metadata with a nested region
    executor contract; do not create a benchmark-named helper family.

H26.1 MIR nested observer-store executor contract:

- Landed metadata:
  - existing `array_text_observer_routes` now owns an optional nested
    `executor_contract`
  - whole-front MIR JSON emits one `single_region_executor` contract for the
    inner scan shape with `effects=[observe.indexof, store.cell]`
  - region mapping records loop index PHI/init/next/bound, observer block,
    predicate value, same-slot store block, latch/exit blocks, const needle
    `"line"`, and const suffix `"ln"`
- Structure:
  - public route family remains `array_text_observer_routes`
  - nested proof logic is isolated in
    `src/mir/array_text_observer_region_contract.rs` so
    `src/mir/array_text_observer_plan.rs` stays under 1000 lines
- Next:
  - H26.2 `.inc` metadata validation and one-call emit
    - extend `region_mapping` with `begin_block` /
      `begin_to_header_block`
    - `.inc` validates the MIR-owned nested contract and emits one call; it
      does not scan raw MIR to rediscover the observer/store shape
  - H26.3 runtime one-call observer-store executor
    - runtime helper is generic to the observer-store contract, not benchmark
      named
    - write guard and resident slot access stay inside the call; runtime does
      not own legality/provenance/publication

H26.2/H26.3/H26.4 observer-store region executor keeper:

- Landed:
  - `executor_contract.region_mapping` now carries `begin_block` and
    `begin_to_header_block`.
  - `.inc` preloads the MIR-owned observer-store region before block emission,
    emits one `nyash.array.string_indexof_suffix_store_region_hisisi` call at
    the begin block, and marks the MIR-covered header/observer/store/latch
    blocks unreachable.
  - Runtime executes the compare-only `indexOf` + same-slot const suffix store
    under one array write guard. It does not decide legality, provenance, or
    publication.
- Keeper evidence:
  - whole `kilo_kernel_small`: `C 82 ms / Ny AOT 10 ms`,
    `ny_aot_instr=149657283`, `ny_aot_cycles=31829608`.
  - exact `kilo_micro_array_string_store`: `C 10 ms / Ny AOT 3 ms`,
    `ny_aot_instr=9266329`, `ny_aot_cycles=2400782`.
  - middle `kilo_meso_substring_concat_array_set_loopcarry`:
    `C 3 ms / Ny AOT 4 ms`,
    `ny_aot_instr=16570773`, `ny_aot_cycles=3435120`.
  - whole asm top: `<&str as core::str::pattern::Pattern>::is_contained_in`
    `35.05%`, `__memmove_avx512_unaligned_erms` `23.82%`,
    `nyash.array.string_len_hi` `20.97%`.
- Next seam:
  - Decide via owner refresh whether residual search / length observer deserves
    another generic MIR consumer capability card, or whether H26 closes and the
    next card starts from fresh perf evidence.

H26e owner refresh / H27 cut:

- Owner refresh commands:
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_kernel_small 1 3`
  - `bash tools/perf/bench_micro_aot_asm.sh kilo_kernel_small '' 20`
  - `bash tools/perf/trace_optimization_bundle.sh --input kilo_kernel_small --route direct --callee-substr string_len --out-dir target/perf_state/h26e_owner_refresh`
- Current result:
  - whole `kilo_kernel_small`: `C 82 ms / Ny AOT 10 ms`,
    `ny_aot_instr=149657100`, `ny_aot_cycles=31814977`.
  - asm top: `<&str as core::str::pattern::Pattern>::is_contained_in`
    `29.05%`, `__memmove_avx512_unaligned_erms` `27.28%`,
    `nyash.array.string_len_hi` `20.76%`.
  - emitted outer edit path:
    `array.get(row) -> nyash.array.string_len_hi -> split=len/2 ->
    nyash.array.string_insert_mid_store_hisii`.
- Verdict:
  - H26 is closed; do not widen H26 with source-prefix/source-length/ASCII
    assumptions.
  - Open H27 as an array/text edit contract: MIR owns
    `source_len_div_const(2)` and same-slot insert-mid legality; `.inc` emits
    from metadata; runtime computes the current cell length and mutation only.
- H27 guard:
  - no benchmark-named helper
  - no raw C-side legality rediscovery for the new path
  - no runtime-owned provenance/publication/route selection
  - exact and middle guards must remain no-regression

H27 landed / H28 cut:

- Implementation:
  - MIR now emits `array_text_edit_routes` for the active same-slot edit
    contract:
    `array.get(row) -> length -> source_len_div_const(2) -> substring concat
    -> same-array set`.
  - `.inc` validates the MIR-owned metadata and emits one
    `nyash.array.string_insert_mid_lenhalf_store_hisi` call from the get site;
    it skips only covered MIR instructions and does not rediscover legality
    from raw JSON.
  - Runtime computes `split = current_text.len() / 2` inside the selected cell
    mutation frame as the MIR-selected policy; it does not decide legality,
    provenance, publication, or route fallback.
- Verification:
  - `cargo test -q array_text_edit_plan --lib`
  - `cargo check -q -p nyash_kernel`
  - `bash tools/perf/build_perf_release.sh`
  - `NYASH_LLVM_ROUTE_TRACE=1 bash tools/perf/trace_optimization_bundle.sh --input kilo_kernel_small --route direct --callee-substr string_len --out-dir target/perf_state/h27_lenhalf_edit`
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_kernel_small 1 3`
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_array_string_store 1 3`
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_array_set_loopcarry 1 3`
  - `bash tools/perf/bench_micro_aot_asm.sh kilo_kernel_small '' 20`
  - `bash tools/checks/dev_gate.sh quick`
- Evidence:
  - route trace hits `stage=array_text_edit_lenhalf result=hit
    reason=mir_route_metadata`.
  - emitted outer edit block calls
    `nyash.array.string_insert_mid_lenhalf_store_hisi` and no longer calls
    `nyash.array.string_len_hi`.
  - whole `kilo_kernel_small`: `C 83 ms / Ny AOT 10 ms`,
    `ny_aot_instr=144977171`, `ny_aot_cycles=30931233`.
  - exact `kilo_micro_array_string_store`: `C 10 ms / Ny AOT 4 ms`.
  - middle `kilo_meso_substring_concat_array_set_loopcarry`:
    `C 4 ms / Ny AOT 4 ms`.
- Verdict:
  - H27 is a small keeper / contract cleanup: instructions dropped from
    `149657100` to `144977171` and cycles from `31814977` to `30931233`, but
    wall time stayed in the `10 ms` band.
  - The next owner is not the len-half edit helper. H28 starts from the
    observer-store region executor: fixed const-needle search and suffix
    mutation/copy mechanics under the MIR-owned H26 region contract.
  - H28 guard: no source-prefix assumption, no search-result cache, no
    benchmark-named whole-loop helper, no runtime-owned legality/provenance,
    and no C-side raw shape rediscovery.

H28.1 landed / H28.2 cut:

- Implementation:
  - Runtime observer-store execution now uses a private `text_contains_literal`
    short-literal search leaf instead of the generic `str::contains` Pattern
    path.
  - MIR metadata and `.inc` lowering are unchanged; the existing H26
    observer-store contract still owns legality/provenance/publication, and the
    backend still emits one metadata-selected helper call.
- Verification:
  - `cargo test -q text_contains_literal --lib`
  - `bash tools/perf/build_perf_release.sh`
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_kernel_small 1 3`
  - `bash tools/perf/bench_micro_aot_asm.sh kilo_kernel_small '' 20`
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_array_string_store 1 3`
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_array_set_loopcarry 1 3`
- Evidence:
  - whole `kilo_kernel_small`: `C 84 ms / Ny AOT 9 ms`,
    `ny_aot_instr=60662079`, `ny_aot_cycles=20100504`.
  - exact `kilo_micro_array_string_store`: `C 10 ms / Ny AOT 4 ms`,
    `ny_aot_instr=9265703`, `ny_aot_cycles=2442083`.
  - middle `kilo_meso_substring_concat_array_set_loopcarry`:
    `C 3 ms / Ny AOT 3 ms`, `ny_aot_instr=16570264`,
    `ny_aot_cycles=3533303`.
  - asm top moved to `__memmove_avx512_unaligned_erms` `43.99%`,
    `with_array_text_write_txn` closure `23.17%`, and
    `__memcmp_evex_movbe` `15.35%`; `Pattern::is_contained_in` is no longer a
    top owner.
- Verdict:
  - H28.1 is a keeper. It removes the fixed-literal search owner without
    shifting authority into runtime or `.inc`.
  - H28.2 first corrects the remaining compare owner: annotate shows
    `__memcmp_evex_movbe` comes from the H28.1 `starts_with` prefix check
    lowering to libc `bcmp`, not from suffix copy.

H28.2 landed / H28.3 cut:

- Implementation:
  - Runtime observer-store short-literal search now checks the prefix with a
    private byte loop instead of `starts_with`.
  - MIR metadata and `.inc` lowering are unchanged; this is runtime-private
    search mechanics under the existing H26 observer-store contract.
- Verification:
  - `cargo test -q text_contains_literal --lib`
  - `cargo fmt --check`
  - `bash tools/perf/build_perf_release.sh`
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_kernel_small 1 3`
  - `bash tools/perf/bench_micro_aot_asm.sh kilo_kernel_small '' 20`
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_array_string_store 1 3`
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_array_set_loopcarry 1 3`
- Evidence:
  - whole `kilo_kernel_small`: `C 83 ms / Ny AOT 7 ms`,
    `ny_aot_instr=64501392`, `ny_aot_cycles=18956185`.
  - exact `kilo_micro_array_string_store`: `C 11 ms / Ny AOT 4 ms`,
    `ny_aot_instr=9266032`, `ny_aot_cycles=2341864`.
  - middle `kilo_meso_substring_concat_array_set_loopcarry`:
    `C 3 ms / Ny AOT 4 ms`, `ny_aot_instr=16571251`,
    `ny_aot_cycles=3446763`.
  - asm top after H28.2 is `__memmove_avx512_unaligned_erms` `39.78%`,
    `with_array_text_write_txn` closure `29.06%`, and observer-store region
    closure `23.51%`; `__memcmp_evex_movbe` is no longer a top owner.
- Verdict:
  - H28.2 is a keeper. It removes the accidental libc compare owner without
    shifting authority into runtime or `.inc`.
  - H28.3 starts from the short suffix append copy under the same MIR-owned
    observer-store region contract.

H28.3 landed / H28.4 cut:

- Implementation:
  - Runtime observer-store execution now uses a private `append_text_suffix`
    leaf for `1..=8` byte suffixes.
  - The short suffix path appends bytes directly under the existing `String`
    UTF-8 invariant; long suffixes stay on `String::push_str`.
  - MIR metadata and `.inc` lowering are unchanged.
- Verification:
  - `cargo test -q append_text_suffix --lib`
  - `cargo test -q text_contains_literal --lib`
  - `cargo fmt --check`
  - `bash tools/perf/build_perf_release.sh`
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_kernel_small 1 3`
  - `bash tools/perf/bench_micro_aot_asm.sh kilo_kernel_small '' 20`
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_array_string_store 1 3`
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_array_set_loopcarry 1 3`
- Evidence:
  - whole `kilo_kernel_small`: `C 82 ms / Ny AOT 7 ms`,
    `ny_aot_instr=60615291`, `ny_aot_cycles=17586950`.
  - exact `kilo_micro_array_string_store`: `C 10 ms / Ny AOT 4 ms`,
    `ny_aot_instr=9266365`, `ny_aot_cycles=2326918`.
  - middle `kilo_meso_substring_concat_array_set_loopcarry`:
    `C 3 ms / Ny AOT 4 ms`, `ny_aot_instr=16571079`,
    `ny_aot_cycles=3398840`.
  - asm top after H28.3 is `__memmove_avx512_unaligned_erms` `38.17%`,
    `with_array_text_write_txn` closure `26.80%`, and observer-store region
    closure `26.43%`.
  - annotate of the observer-store closure shows the short suffix append path
    no longer calls `memcpy`; residual `memmove` belongs to capacity growth /
    old-content copy or adjacent write-frame mechanics.
- Verdict:
  - H28.3 is a small keeper: whole-front instruction/cycle count improves
    without shifting authority into runtime or `.inc`.
  - H28.4 starts from capacity growth / write-frame owner decision.

H28.4 cut:

- Owner split:
  - this is a new owner-first slice under H28, not a continuation of the H25
    write-lock guard mechanics card
  - target owner is resident `String` append capacity miss leading to realloc /
    old-content copy under the H26 observer-store suffix append executor
- Decision:
  - keep MIR metadata, `.inc` lowering, and public ABI unchanged
  - probe the append leaf first; only then try a Rust-only runtime-private text
    append headroom policy
  - the policy may look only at storage facts such as suffix length, current
    length, and current capacity
- Forbidden:
  - source-prefix / benchmark-name branches
  - search-result cache
  - runtime-owned legality/provenance/publication
  - C-side shape planning or new MIR metadata for capacity tuning
  - plugin facade policy ownership; the policy belongs at the runtime-private
    string growth leaf
- Keeper gate:
  - whole `kilo_kernel_small` instruction/cycle count improves and
    `__memmove` share drops
  - exact `kilo_micro_array_string_store` and middle
    `kilo_meso_substring_concat_array_set_loopcarry` stay no-regression
  - reject if `memmove` only moves into allocator / `_int_malloc`

H28.4 rejected / H28.5 cut:

- Trial:
  - Rust-only short append headroom policy in `append_short_text_suffix`
  - no MIR metadata, `.inc` lowering, or public ABI change
- Evidence:
  - whole `kilo_kernel_small` first run: `C 82 ms / Ny AOT 7 ms`,
    `ny_aot_instr=61363741`, `ny_aot_cycles=17616053`
  - whole rerun: `C 82 ms / Ny AOT 8 ms`,
    `ny_aot_instr=61364376`, `ny_aot_cycles=17951505`
  - exact `kilo_micro_array_string_store`: `C 10 ms / Ny AOT 4 ms`,
    `ny_aot_instr=9265802`, `ny_aot_cycles=2367573`
  - middle `kilo_meso_substring_concat_array_set_loopcarry`:
    `C 3 ms / Ny AOT 4 ms`, `ny_aot_instr=16570977`,
    `ny_aot_cycles=3472466`
  - asm after trial: `__memmove_avx512_unaligned_erms` dropped to `34.76%`,
    but `with_array_text_write_txn` rose to `31.09%` and the observer-store
    closure rose to `27.10%`
- Verdict:
  - reject; the target transition did not improve instruction/cycle/wall
    enough to be a keeper
  - code was reverted, leaving the H28.3 runtime append leaf intact
  - H28.5 starts with residual `memmove` owner refresh. Gather
    callsite/callgraph evidence before opening more runtime copy/capacity
    surgery.

H28.5 landed / H29 cut:

- Commands:
  - `bash tools/perf/build_perf_release.sh`
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_kernel_small 1 3`
  - `bash tools/perf/bench_micro_aot_asm.sh kilo_kernel_small '' 20`
  - manual `perf record --call-graph dwarf` on the generated whole-front AOT
    executable
- Evidence:
  - whole `kilo_kernel_small`: `C 84 ms / Ny AOT 7 ms`,
    `ny_aot_instr=60616017`, `ny_aot_cycles=17782048`
  - asm top after reverting H28.4 code:
    - `__memmove_avx512_unaligned_erms`: `37.20%`
    - observer-store region closure: `28.98%`
    - `with_array_text_write_txn` closure: `26.22%`
    - `nyash.array.string_insert_mid_lenhalf_store_hisi`: `3.26%`
  - callgraph: dominant `__memmove` child is
    `array_string_insert_const_mid_lenhalf_by_index_store_same_slot_str`
    closure (`27.91%`)
  - append / realloc growth through `alloc::raw_vec::finish_grow` is only
    about `0.93%`
- Verdict:
  - H28 observer-store search/copy split is closed
  - residual `memmove` is not primarily append capacity; do not chase H28.4
    headroom further
  - H29 opens as len-half edit copy owner decision under the existing
    MIR-owned H27 edit contract

H29 rejected / H30 cut:

- Commands:
  - `cargo test -q -p nyash_kernel insert_mid_lenhalf_store_by_index_returns_result_len`
  - `cargo test -q -p nyash_kernel insert_mid_store_by_index`
  - `cargo test -q detects_lenhalf_insert_mid_same_slot_edit_route --lib`
  - `cargo fmt --check`
  - `bash tools/perf/build_perf_release.sh`
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_kernel_small 1 3`
  - `bash tools/perf/bench_micro_aot_asm.sh kilo_kernel_small '' 20`
- Trial:
  - replaced the len-half helper's `String::insert_str` call with a
    runtime-private explicit reserve + suffix shift + middle copy leaf
  - no MIR metadata, `.inc` lowering, or public ABI changed
- Evidence:
  - whole `kilo_kernel_small`: `C 83 ms / Ny AOT 7 ms`,
    `ny_aot_instr=60494965`, `ny_aot_cycles=17790198`
  - asm top after trial:
    - `__memmove_avx512_unaligned_erms`: `40.84%`
    - `with_array_text_write_txn` closure: `30.00%`
    - observer-store region closure: `20.99%`
    - `nyash.array.string_insert_mid_lenhalf_store_hisi`: `3.21%`
- Verdict:
  - H29 local byte-copy surgery is rejected and reverted
  - the contiguous `String` mid-insert executor still requires moving the
    suffix bytes for the MIR-owned H27 edit contract
  - H30 opens as a representation decision: only a narrow array text edit
    residence representation may attack this owner cleanly; do not add
    benchmark-named helpers, runtime legality, or `.inc` rediscovery

## Legacy Retirement Ledger

Purpose: keep compiler cleanup work visible without spreading TODOs through the codebase. This ledger is the SSOT for planned deletion candidates in the active phase-137x lane.

Rules:
- A row may be deleted only when its removal gate is green in the same commit.
- Compatibility rows must stay runtime-private unless a separate public ABI phase explicitly opens them.
- Do not delete a legacy row just because the latest exact front does not hit it; prove no active guard/front needs it or move it to an explicit legacy regression fixture first.

| Surface | Status | Why It Still Exists | Removal Gate |
| --- | --- | --- | --- |
| `nyash.array.string_suffix_store_his` | compatibility row | Pointer/CStr validated suffix helper retained after direct lowering moved to `nyash.array.string_suffix_store_hisi` | Delete only after all source-only/indexof branch smokes require `hisi`, pure declarations no longer emit `his`, and no fixture/asm grep observes a `his` call. |
| `nyash.array.string_insert_mid_store_hisi` | compatibility row | Pointer/CStr validated insert-mid helper retained after direct lowering moved to `nyash.array.string_insert_mid_store_hisii` | Delete only after `phase137x_boundary_array_string_len_insert_mid_source_only_min.sh` and related generic-lowering guards require `hisii`, and pure declarations no longer emit `hisi`. |
| `nyash.array.string_insert_mid_subrange_store_hisiii` | compatibility row | Pointer/CStr validated subrange helper retained after direct lowering moved to `nyash.array.string_insert_mid_subrange_store_hisiiii` | Delete only after concat3/subrange source-only smokes require `hisiiii`, docs no longer name `hisiii` as active direct route, and pure declarations no longer emit `hisiii`. |
| `lang/c-abi/shims/hako_llvmc_ffi_array_string_store_seed.inc` exact seed emitter | temporary bridge surface | Pure-first array/string-store micro seed still has a specialized stack-array emitter for the current micro front; the route-shape proof is now MIR-owned metadata, not a C-side scanner. | Delete after TextLane / ArrayStorage::Text direct lowering owns the active array-string store route, or move the exact seed emitter into an explicit legacy regression fixture with failure expectation. |
| `lang/c-abi/shims/hako_llvmc_ffi_indexof_text_state_residence.inc` text-state residence temporary emitter | temporary bridge surface | The exact leaf/line dispatch bridge and backend env guard are retired in H15.7, and exported `indexof_search_micro_seed_route` is retired in H15.9. The file remains because current residence emission still consumes `array_text_state_residence_route.temporary_indexof_seed_payload`. | Delete after MIR owns a non-exact residence payload and `.inc` can emit from generic residence metadata without `temporary_indexof_seed_payload`. |
- retired in `137x-E0.1`: the old `kilo_micro_array_string_store` `9-block` exact seed matcher branch is deleted after the compact `8-block` direct producer stayed green under `phase137x_direct_emit_array_store_string_contract.sh`.
- retired in `137x-E0.2`: shared-receiver legacy scanner fallback is deleted after the active const-suffix / insert-mid shared-receiver fixtures gained MIR-owned `read_alias.shared_receiver` metadata and stayed green metadata-only.
- retired in `137x-E1`: array-string store no longer keeps a `BorrowedHandleBox` retarget executor path or kernel-slot-to-StringBox overwrite helper; the active route stores runtime-private text residence and degrades mixed arrays to Boxed.
- retired in `137x-H2`: `src/host_providers/llvm_codegen/compat_text_primitive.rs` is renamed out of active code; remaining Rust-side `MIR(JSON text) -> object path` emission lives in `mir_json_text_object.rs` as a no-helper backend boundary.
- retired in `137x-H13`: `match_piecewise_slot_hop_substring_consumer(...)` is deleted; slot-hop substring consumer, window, and skip indices are now MIR-owned `StringKernelPlan.slot_hop_substring` metadata.
- retired in `137x-H13`: the raw C-side 8-block scanner in `hako_llvmc_match_array_string_store_micro_seed(...)` is deleted; exact seed bridge selection now consumes MIR-owned `metadata.array_string_store_micro_seed_route`.
- retired in `137x-H13`: `hako_llvmc_ffi_concat_hh_len_seed.inc` is deleted; the current `kilo_micro_concat_hh_len` direct front stays green through generic/metadata lowering and no longer needs a dedicated exact bridge.
- retired in `137x-H13`: the raw C-side 5-block scanner in `hako_llvmc_match_concat_const_suffix_micro_seed(...)` is deleted; exact seed bridge selection now consumes MIR-owned `metadata.concat_const_suffix_micro_seed_route`.
- retired in `137x-H13`: the raw C-side block/op scanner in `hako_llvmc_match_substring_concat_loop_ascii_seed(...)` is deleted; exact seed bridge selection now consumes existing MIR `StringKernelPlan.loop_payload` and `stable_length_scalar` relation metadata.
- retired in `137x-H13`: the raw C-side 5-block scanner in `hako_llvmc_match_substring_views_only_micro_seed(...)` is deleted; exact seed bridge selection now consumes MIR-owned `metadata.substring_views_micro_seed_route`, while borrowed-window legality stays in `StringKernelPlan`.
- retired in `137x-H13`: `hako_llvmc_ffi_string_loop_seed_length_hot_loop.inc` and `hako_llvmc_emit_string_length_hot_loop_ir(...)` are deleted; current length-hot fronts use generic/metadata lowering instead of the obsolete 5/6-block exact matcher family.
- retired in `137x-H14`: the raw C-side block/op scanners in `hako_llvmc_match_indexof_leaf_ascii_seed(...)` and `hako_llvmc_match_indexof_line_ascii_seed(...)` are deleted; exact search bridge selection now consumes MIR-owned `metadata.indexof_search_micro_seed_route`.
- shrunk in `137x-H14.1`: the leaf exact search emitter no longer calls runtime `nyash.string.indexOf_ss`; it emits only the MIR-owned literal membership predicate after validating candidate outcomes metadata.
- shrunk in `137x-H14.2`: `hako_llvmc_emit_indexof_leaf_ir(...)` and `hako_llvmc_emit_indexof_line_ir(...)` are collapsed into `hako_llvmc_emit_indexof_seed_ir(...)`; the remaining exact search bridge is one temporary backend surface with MIR-owned proof/action metadata.
- shrunk in `137x-H14.3`: leaf/line matcher wrappers now call `hako_llvmc_match_indexof_ascii_seed_variant(...)`; shared parse/validation/trace/emitter mechanics are no longer duplicated.
- opened in `137x-H15`: generic `array_text_observer_routes` becomes the deletion path for the remaining exact search bridge; MIR owns observer legality/provenance/consumer facts while `.inc` stays helper selection and emit only.
- shrunk in `137x-H15`: active indexOf observer prepass/get lowering now consumes `array_text_observer_routes`; raw C scanner calls are absent from those active surfaces. The specialized exact bridge remains until the generic route is keeper-fast.
- retired in `137x-H15`: unused raw observer analyzer/trace `.inc` files are deleted; active observer lowering includes only metadata defer state and metadata consumer lowering.
- retired in `137x-H15.7`: exact leaf/line search dispatch wrappers and backend env `NYASH_LLVM_SKIP_INDEXOF_LINE_SEED` are deleted; exact and compatibility-skip runs now route through `array_text_state_residence_route`.
- renamed in `137x-H15.8`: `hako_llvmc_ffi_string_search_seed.inc` becomes `hako_llvmc_ffi_indexof_text_state_residence.inc`; the remaining backend surface is named after the MIR residence contract instead of the retired exact seed bridge.
- retired in `137x-H15.9`: `FunctionMetadata.indexof_search_micro_seed_route` and the standalone MIR JSON key are deleted; `array_text_state_residence_route` is the only exported backend route owner for this path.
- current phase-2 start:
  - `string_handle_from_owned{,_concat_hh,_substring_concat_hhii,_const_suffix}` now enter explicit cold publish adapters
  - `publish_owned_bytes_*_boundary` / `objectize_kernel_text_slot_stable_box` are outlined cold boundaries
  - latest reread:
    - `kilo_micro_array_string_store = C 10 ms / Ny AOT 3 ms`
    - `kilo_kernel_small = C 81 ms / Ny AOT 768 ms`
  - reading:
    - exact stays closed
    - whole remains neutral inside the same publication/source-capture owner family
    - next phase-2 slice must reduce publish/source-capture frequency, not just outline it
- current phase-2 source-capture prework:
  - `with_array_store_str_source(...)` checks a latest-fresh stable-box cache before registry slot lookup
  - cache validity is guarded by `drop_epoch`
  - latest reread:
    - `kilo_micro_array_string_store = C 10 ms / Ny AOT 3 ms`
    - `kilo_kernel_small = C 80 ms / Ny AOT 1068 ms`
  - reading:
    - exact stays closed
    - whole remains neutral in the same owner family
    - legacy coexistence remains temporary and should be deleted once the new path proves keeper-grade
- latest whole-shape probe is now closed:
  - emitted LLVM IR on `kilo_kernel_small` proves both hot store sites already lower to:
    - `insert_hsi -> kernel_slot_insert_hsi -> kernel_slot_store_hi`
    - `current + "ln" -> kernel_slot_concat_hs -> kernel_slot_store_hi`
  - reading:
    - compiler widening is no longer the live blocker on the whole bench
    - the next owner is runtime materialization/copy tax inside the kernel-slot lane
- latest phase-2 store-side narrow cut:
  - `kernel_slot_store_hi` now overwrites an existing `StringBox` array slot in place instead of replacing the outer box
  - latest reread:
    - `kilo_micro_array_string_store = C 10 ms / Ny AOT 3 ms`
    - `kilo_kernel_small = C 80 ms / Ny AOT 781 ms`
  - reading:
    - exact stays closed
    - whole remains neutral
    - next card stays materialization-side: `kernel_slot_concat_hs` first, then `insert_const_mid_into_slot`
- latest phase-2 materialize cut:
  - `kernel_slot_concat_hs` now prefers borrowed-text direct materialization under `with_text_read_session_ready(...)`
  - `insert_const_mid_into_slot` now follows the same borrowed-text direct path before owned fallback
  - latest reread:
    - `kilo_micro_array_string_store = C 9 ms / Ny AOT 3 ms`
    - `kilo_kernel_small = C 80 ms / Ny AOT 739 ms`
    - `kilo_kernel_small_hk = C 79 ms / Ny AOT 748 ms` (`strict`, parity ok)
  - reading:
    - exact stays closed
    - whole improved against the prior `781 ms` reread
    - strict whole also stays in the same better band
    - keep the lane open until this proves keeper-grade and not just a favorable band read
- latest phase-2 deferred `const_suffix` slot cut:
  - `kernel_slot_concat_hs` now leaves a deferred `const_suffix` state inside the current `KernelTextSlot` layout
  - `kernel_slot_store_hi` consumes that state before generic freeze/objectize
  - existing `StringBox` array slots append in place when the deferred source still matches the slot text
  - latest reread:
    - `kilo_micro_array_string_store = C 10 ms / Ny AOT 3 ms`
    - `kilo_kernel_small = C 79 ms / Ny AOT 726 ms`
    - `kilo_kernel_small_hk = C 81 ms / Ny AOT 808 ms` (`strict`, parity ok)
  - reading:
    - exact stays closed
    - plain whole improved again
    - strict whole needs a stability reread before keeper/reject
- rejected follow-up probe:
  - replacing BorrowedHandleBox unpublished retarget objectization with an owned-string keep regressed whole:
    - `kilo_kernel_small = C 81 ms / Ny AOT 980 ms`
    - `kilo_kernel_small_hk = C 80 ms / Ny AOT 1015 ms`
  - reason:
    - read-side borrowed-alias encode lost cheap stable-object reuse and started allocating on `array.get`
  - restored reread after reverting that probe:
    - `kilo_kernel_small = C 81 ms / Ny AOT 810 ms`
    - `kilo_kernel_small_hk = C 82 ms / Ny AOT 864 ms`
  - next seam had to keep `array.get` cheap; do not reopen `owned-string keep` / `owned-text keep`
  - follow-up card was read-side alias lane split:
    - `TextReadOnly`
    - `EncodedAlias`
    - `StableObject`
    - stable objectize must stay cold and cache-backed
  - first phase 2.5 slice is now landed:
    - `BorrowedHandleBox` caches the encoded runtime handle for unpublished keeps
    - `array.get` can reuse the cached stable handle instead of fresh-promoting on every read
    - latest strict reread: `kilo_kernel_small_hk = C 79 ms / Ny AOT 791 ms` (`repeat=3`, parity ok)
  - latest phase 2.5 follow-on slices are now landed:
    - map value stores preserve borrowed string aliases through `CodecProfile::MapValueBorrowString`
    - borrowed-alias runtime-handle cache is shared across alias lineage, so map reads keep the same cached encoded handle after clone-for-read
    - `perf-observe` and end-to-end tests now lock all three read outcomes for both array/map routes:
      - `live source`
      - `cached handle`
      - `cold fallback`
  - latest strict reread on the updated lane:
    - `kilo_micro_array_string_store = C 10 ms / Ny AOT 3 ms`
    - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 61 ms`
    - `kilo_kernel_small_hk = C 82 ms / Ny AOT 809 ms`
    - `kilo_kernel_small_hk = C 80 ms / Ny AOT 892 ms`
  - cleanup-parked strict reread:
    - `kilo_kernel_small_hk = C 80 ms / Ny AOT 872 ms` (`repeat=3`, parity ok)
    - `kilo_kernel_small_hk = C 79 ms / Ny AOT 842 ms` (`repeat=3`, parity ok)
  - cleanup-parked asm/top owner proof:
    - command:
      - `PERF_VM_FORCE_NO_FALLBACK=1 PERF_AOT_DIRECT_ONLY=1 bash tools/perf/bench_micro_aot_asm.sh kilo_kernel_small_hk 'ny_main' 1`
    - top report:
      - libc copy/alloc remains dominant: `__memmove_avx512_unaligned_erms 21.41%`, `_int_malloc 9.26%`, `malloc 1.51%`
      - hottest named repo read/materialization family:
        - `objectize_kernel_text_slot_stable_box 4.42%`
        - `array_get_index_encoded_i64::{closure} 4.25%`
        - nested `array_get_index_encoded_i64` closure `2.70%`
        - `TextKeepBacking::clone_stable_box_cold_fallback 0.94%`
      - store/producer helpers are lower:
        - `array_string_store_kernel_text_slot_at::{closure} 1.99%`
        - `array_string_indexof_by_index... 1.00%`
        - `string_span_cache_get 0.61%`
        - `nyash.string.kernel_slot_concat_hs 0.40%`
        - `nyash.array.kernel_slot_store_hi 0.30%`
        - `insert_const_mid_into_slot::{closure} 0.22%`
    - reading:
      - current owner proof has moved from store publication to read-side encode/materialize/objectize around `array.get`
      - preserve cheap alias encode; stable objectization must remain cached/cold, not per-read
      - do not open a new `TextLane` / MIR legality card before this seam gets keeper/reject evidence
  - latest read-encode BoxShape cleanup:
    - `array.get` now routes into a scalar-checked borrowed-alias encoder after its local int/bool probes
    - borrowed-alias encode planning now snapshots `drop_epoch` once and passes it into cached-handle validation
    - this removes duplicate immediate-scalar probes in the read encode path while preserving the existing borrowed-alias contract:
      - live-source reuse first
      - cached stable handle reuse second
      - cold stable objectize fallback last
    - validation:
      - targeted borrowed-alias array/map tests
      - `cargo check -q -p nyash_kernel`
      - `cargo test -q -p nyash_kernel --lib`
      - `tools/checks/dev_gate.sh quick`
    - perf reread:
      - exact stays closed: `kilo_micro_array_string_store = C 10 ms / Ny AOT 3 ms`
      - meso remains open/noisy: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 65 ms`
      - strict whole is noisy: `kilo_kernel_small_hk = C 80 ms / Ny AOT 1740 ms` then rerun `C 80 ms / Ny AOT 808 ms`
    - fresh owner proof after the epoch-snapshot cleanup:
      - exact stays closed: `kilo_micro_array_string_store = C 9 ms / Ny AOT 4 ms`
      - meso remains open/noisy: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 61 ms`
      - strict whole remains in-band: `kilo_kernel_small_hk = C 80 ms / Ny AOT 812 ms` (`repeat=3`, parity ok)
      - asm/top:
        - `__memmove_avx512_unaligned_erms 25.02%`
        - `_int_malloc 9.58%`
        - `array_get_index_encoded_i64::{closure} 4.39%`
        - `objectize_kernel_text_slot_stable_box 3.62%`
        - nested `array_get_index_encoded_i64` closure `2.09%`
        - `array_string_store_kernel_text_slot_at::{closure} 2.01%`
        - `TextKeepBacking::clone_stable_box_cold_fallback 0.49%`
    - reading:
      - this cleanup is not keeper evidence
      - next owner remains stable keep creation / first-read handle publication plus materialization/copy around the existing borrowed-alias store-read chain
      - old blocker rule is superseded; `TextLane`, Value Lane, and allocator now open through `137x-E/F/G`
    - rejected follow-up probe after that proof:
      - attempted unpublished `owned-text keep` for `KernelTextSlot -> existing BorrowedHandleBox` retarget without changing public ABI or `KernelTextSlot` layout
      - exact guard stayed closed: `kilo_micro_array_string_store = C 10 ms / Ny AOT 4 ms`
      - meso stayed noisy/open: `kilo_meso_substring_concat_array_set_loopcarry = C 4 ms / Ny AOT 62 ms`
      - strict whole regressed: `kilo_kernel_small_hk = C 84 ms / Ny AOT 902 ms`, rerun `C 82 ms / Ny AOT 892 ms`
      - asm/top removed `objectize_kernel_text_slot_stable_box`, but shifted cost into `__memmove_avx512_unaligned_erms 28.32%`, `_int_malloc 12.47%`, and `array_string_store_kernel_text_slot_at::{closure} 5.89%`
      - reject reason:
        - active whole still calls `array.get_hi`, so delaying stable birth from store to read does not remove object-world demand
        - the seam moved publication/copy tax and increased store/read residence work
        - code was reverted; do not reopen store-side `owned-string keep` or `owned-text keep` without a front that no longer demands object handles on read
    - rejected follow-up probe: array-slot concat-by-index helper
      - attempted runtime-private `nyash.array.kernel_slot_concat_his(slot, array_h, idx, suffix)` and lowered the hot `array.get_hi -> const_suffix concat -> kernel_slot_store_hi` store to it
      - exact guard stayed closed: `kilo_micro_array_string_store = C 10 ms / Ny AOT 4 ms`
      - meso stayed noisy/open: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 62 ms`
      - strict whole regressed: first `kilo_kernel_small_hk = C 82 ms / Ny AOT 1571 ms`, rerun `C 80 ms / Ny AOT 1033 ms`
      - IR proof:
        - `nyash.array.kernel_slot_concat_his` was emitted at the hot concat store
        - the preceding `nyash.array.slot_load_hi` still remained before `nyash.array.string_indexof_hih`
      - reject reason:
        - adding a direct concat helper without removing the live `array.get_hi` read only adds another executor path
        - code was reverted; do not retry array-slot concat helpers unless the same card also proves the preceding `slot_load_hi` is removed safely
    - latest branch-target-aware retry:
      - same helper shape is reopened only for the exact branch-target proof:
        - `array.get -> indexOf("line") -> compare -> branch`
        - the branch target uses the get result solely as `copy -> const suffix -> Add -> same array.set(idx, value)`
      - lowering now records the get source and emits:
        - `nyash.array.string_indexof_hih` for the observer
        - `nyash.array.kernel_slot_concat_his(slot, array_h, idx, suffix)` for the same-slot suffix store
        - `nyash.array.kernel_slot_store_hi` for the store sink
      - the same-slot path must not call `nyash.array.slot_load_hi`; other live-after-get reuse shapes still keep `slot_load_hi`
      - validation:
        - `cargo test -q -p nyash_kernel --lib kernel_slot_concat_by_index_reads_string_slot_directly`
        - phase29ck boundary string indexOf smoke set, including branch/select/interleaved/cross-block controls
      - keeper status:
        - structure and smoke gates are green
        - perf keeper proof is green:
          - `kilo_micro_array_string_store = C 9 ms / Ny AOT 3 ms`
          - `kilo_kernel_small = C 80 ms / Ny AOT 214 ms`
          - `kilo_kernel_small_hk = C 81 ms / Ny AOT 218 ms` (`repeat=3`, parity ok)
        - this is a narrow phase-137x keeper cut; successor generalization now uses the separate `137x-E/F/G` gates
    - active follow-up structure card: same-slot exact-route helper interior
      - owner family:
        - `array_string_concat_const_suffix_by_index_store_same_slot_str`
        - `array_string_indexof_by_index_str`
        - `append_const_suffix_to_string_box_value`
      - purpose:
        - reduce same-slot exact-route copy/search tax without widening public ABI
      - perf/asm reading:
        - fresh owner proof is still reject-side after the exact route-shape keeper
        - whole-front asm still clusters around the same-slot exact-route helper family
      - boundary:
        - this is structure, not keeper proof
        - old helper-local next-card rule is superseded; the next implementation sequence is `137x-E/F/G`
    - current source-only get suppression + same-slot string store keeper:
      - `array.get -> length -> substring(0, split) + const + substring(split, len) -> array.set(...)` now has a dedicated source-only len-window guard
      - lowering records the array text source with `remember_array_string_get_source(...)`, emits `nyash.array.string_len_hi`, and skips the object-handle get when no later consumer needs the fetched object
      - same-slot insert-mid store now lowers to runtime-private `nyash.array.string_insert_mid_store_hisii(array_h, idx, middle_ptr, middle_len, split)`
      - branch same-slot const-suffix store now lowers to runtime-private `nyash.array.string_suffix_store_hisi(array_h, idx, suffix_ptr, suffix_len)`
      - same-slot insert-mid subrange direct lowering now uses `nyash.array.string_insert_mid_subrange_store_hisiiii(array_h, idx, middle_ptr, middle_len, split, start, end)`
      - the same-slot suffix branch no longer allocates a `KernelTextSlot`
      - runtime residence rule:
        - existing raw `StringBox` slot is mutated in place
        - borrowed-handle slot is materialized into an unpublished raw `StringBox` residence
        - the source stable handle behind the borrowed alias is not mutated
      - new guard:
        - fixture: `apps/tests/mir_shape_guard/array_string_len_insert_mid_source_only_min_v1.mir.json`
        - smoke: `tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_array_string_len_insert_mid_source_only_min.sh`
      - regression guard:
        - `tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_len_live_after_get_min.sh`
        - live-after-get substring reuse still keeps `nyash.array.slot_load_hi`
      - perf/asm proof:
        - exact keeper: `kilo_micro_array_string_store = C 11 ms / Ny AOT 10 ms`, `ny_aot_instr=26922130`
        - exact route proof: `array_string_store_micro result=emit reason=exact_match`
        - meso: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 9 ms`, `ny_aot_instr=127269397`
        - strict whole: `kilo_kernel_small_hk = C 82 ms / Ny AOT 28 ms` (`repeat=3`, parity ok)
        - `ny_main` no longer calls `nyash.array.get_hi`
        - hot edit path is `nyash.array.string_len_hi -> nyash.array.string_insert_mid_store_hisii`
        - branch suffix path is `nyash.array.string_indexof_hih -> nyash.array.string_suffix_store_hisi`
        - same-slot paths have no `nyash.array.kernel_slot_insert_hisi`, `nyash.array.kernel_slot_concat_his`, or `nyash.array.kernel_slot_store_hi`
        - `__strlen_evex` and `core::str::converts::from_utf8` are absent from the current whole asm hot report
      - boundary:
        - narrow source-only window only
        - successor generalization now uses `137x-E/F/G`; public ABI widening remains blocked
      - next owner proof seam:
        - asm top moved to `memchr::arch::x86_64::memchr::memchr_raw::find_avx2`, `array_string_concat_const_suffix_by_index_store_same_slot_str`, `__memmove_avx512_unaligned_erms`, `array_string_indexof_by_index_str`, `array_string_insert_const_mid_by_index_store_same_slot_str`, and `array_string_len_by_index`
        - old helper-local next-card rule is superseded; next cut starts from `137x-E`
  - reading:
    - phase 2.5 no longer has only the `array.get` cached-handle proof
    - exact stays closed, but meso / strict whole reopened upward versus the prior `57 ms` / `791 ms` band
    - cleanup queue is parked after the smallest BoxShape cards
    - current reading remains reject-side for keeper judgement on this lane
    - current reading now hands off to `137x-E` before any new kilo owner proof
    - parked cleanup-card details live in `phase137x-text-lane-rollout-checklist.md`
- current next seam: phase-2.5 remains the active judge, but code needs a fresh narrow owner proof before another edit
  - direct-set-only `insert_hsi` and deferred `Pieces3 substring` widenings are already landed on the unpublished contract
  - do not reopen phase-1 producer widening as the current next step unless new evidence moves the owner back there
- current reject: slot-store delayed publication probes and string-specialized handle payload probe
- read order:
  1. `CURRENT_TASK.md`
  2. `docs/development/current/main/10-Now.md`
  3. this README
  4. `docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md`
  5. `docs/development/current/main/design/string-value-model-phased-rollout-ssot.md`
  6. `docs/development/current/main/phases/phase-137x/phase137x-text-lane-rollout-checklist.md`
  7. `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`

## Decision Now

- fixed implementation order before next kilo optimization:
  1. `137x-E1`: minimal `TextLane` / `ArrayStorage::Text` (landed)
  2. `137x-F`: runtime-wide `Value Lane` implementation bridge
  3. `137x-G`: allocator / arena pilot
  4. `137x-H`: next kilo optimization return
- current local rule:
  - implement the storage/value/allocator gates before returning to helper-local kilo tuning
  - use exact micro + middle + whole-kilo as accept gates after each implementation slice
- current rollout rule:
  - do not skip phase order
  - prove canonical sink continuity before publish isolation
  - `TextLane` storage rewrite is now the active next gate, not a future-only blocker
- `phase-134x` structural split is landed
- `phase-138x` / `phase-139x` / `phase-140x` / `phase-141x` semantic-owner corridor is landed
- contract-first corridor は landed
- Birth / Placement vocabulary lock is now landed in design SSOT
- `perf-observe` seam split is now landed for the hot `piecewise_subrange_hsiii` corridor
- `vm-hako` stays parked as reference/conformance

## Completed Audit Lock (2026-04-18)

- confirmed exact asm/perf audit on `kilo_micro_array_string_store`:
  - stat: `C 10 ms / Ny AOT 131 ms`
  - top samples: `substring_concat_hhii_export_impl 22.38%`, `string_concat_hh_export_impl 21.70%`, array string-store closure `17.34%`, `from_i8_string_const 13.07%`, `LocalKey::with 6.07%`, `memmove 3.51%`, `_int_malloc 1.75%`
  - hot instructions carry host-handle atomics (`lock xadd/cmpxchg/inc/dec`), TLS publish stores, alloc shim calls, and array-store handle/publication branches
  - extra loop-hot calls per iter vs C: `from_i8_string_const` x2, `concat_hh` x1, `set_his` x1, `substring_concat_hhii` x1
  - wrapper functions are not the owner; current evidence points to inner publication / object-world entry
- confirmed whole asm/perf audit on `kilo_kernel_small`:
  - stat: `C 80 ms / Ny AOT 741 ms`
  - top user symbols: `nyash.string.concat_hs 11.19%`, `execute_store_array_str_contract` closure `7.01%`, `insert_const_mid_fallback` closure `3.89%`, `array_get_index_encoded_i64` closure `3.62%`, `from_i8_string_const 3.52%`, libc `memmove 14.92%`, `_int_malloc 4.65%`
  - hot instructions in `concat_hs` are TLS/helper-entry, not the copy body
  - `insert_const_mid_fallback` and array store/read closures spend samples on registry fetch, `lock cmpxchg`, vtable probes, and handle/cache publication
  - the whole path still pays many helper boundaries before store completes
- confirmed observability-gap audit:
  - prior evidence was not enough to split generic-fallback boundary cost from its children
  - landed observability-only patch in `crates/nyash_kernel/src/plugin/value_codec/string_materialize.rs`
  - new site-specific noinline generic-fallback boundary symbols:
    - `string_concat_hh`
    - `string_substring_concat_hhii`
    - `const_suffix`
    - `freeze_text_plan_pieces3`
  - tests passed with and without `perf-observe`
- latest reread after the shared-receiver `KernelTextSlot` widening:
  - exact `kilo_micro_array_string_store`:
    - stat: `C 10 ms / Ny AOT 4 ms`
    - reading:
      - the exact `store.array.str` front is now closed
      - `bench_micro_aot_asm` top report is startup/env dominated, so this is no longer the active owner proof front
  - middle `kilo_meso_substring_concat_array_set_loopcarry`:
    - stat: `C 3 ms / Ny AOT 57 ms`
    - reading:
      - still inside the prior `56-59 ms` band
      - producer-side unpublished outcome widening remains live, but this landing is not a meso keeper by itself
  - whole `kilo_kernel_small`:
    - stat: `C 86 ms / Ny AOT 856 ms` (`repeat=3`)
    - reading:
      - pure-first helper/direct replay still compiles after the declaration/need-flag fixes
      - loop-body `KernelTextSlot` allocas no longer crash the whole bench after `stacksave/stackrestore`
      - direct-set-only `insert_hsi` and deferred `Pieces3 substring` now both lower through `KernelTextSlot -> kernel_slot_store_hi`
      - this improves the blocked reread, but still does not make the current landing a whole-front keeper
      - latest microasm top user symbols are now `array_string_store_kernel_text_slot_at` closure `6.29%`, `array_get_index_encoded_i64` closure `4.38%`, `insert_const_mid_into_slot` closure `1.81%`, `nyash.string.kernel_slot_concat_hs` `1.61%`, `nyash.array.kernel_slot_store_hi` `0.92%`
      - remaining dominant tax is still allocator / copy side (`memmove 15.82%`, `_int_malloc 6.19%`) plus array/string slot work
      - next code cut still targets post-store reuse / non-direct-set `freeze_text_plan(Pieces3)` rather than generic helper ABI widening
- next-cut reading (separate from confirmed evidence):
  - perf/asm is now sufficient to choose the next keeper without another broad observability round
  - keep exact and whole separate when judging the next keeper
  - current evidence points to publication/object-world entry as the live owner; do not read this as proof of a representation / ABI change
  - current implementation choice is now fixed:
    - keep whole-front ownership as the tiebreaker
    - before Card A/B code cuts, slot publish-boundary verifier/counters are now landed:
      - `publish_boundary.slot_publish_handle_total`
      - `publish_boundary.slot_objectize_stable_box_total`
      - `publish_boundary.slot_empty`
      - `publish_boundary.slot_already_published`
      - `objectize_kernel_text_slot_stable_box` records `publish_reason.need_stable_object`
      - this reduces the blind spot from `generic_fallback` to the upstream producer/retarget owner before slot exit
      - latest exact / meso / whole reread keeps these slot-boundary counters at `0`, so slot exit is observed and inactive on the live fronts
    - first code seam is now producer-side unpublished outcome:
      - add a narrow runtime-private `const_suffix -> KernelTextSlot` seam
      - keep `KernelTextSlot -> store.array.str` as the immediate sink-side consumer
      - do not widen generic publish helpers or public ABI
    - intended shape:
      - preserve the existing `set_his` fast-path / alias-retarget contract
      - let producer code return slot-owned text without forcing `StringBox -> handle`
      - landed narrow compiler/backend widening for:
        - direct-set-only `const_suffix -> set(...)`
        - shared-receiver exact front where the same `const_suffix` producer feeds:
          - `set(...)`
          - known-length observers
          - trailing `substring(...)` without early publish
      - next widening target is fixed:
        - `freeze_text_plan(Pieces3)` / `insert_const_mid_fallback`
        - keep the same unpublished contract and do not widen generic helper ABI
      - guard the landed direct-set bridge with:
        - fixture: `apps/tests/mir_shape_guard/string_const_suffix_kernel_slot_direct_set_min_v1.mir.json`
        - smoke: `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_const_suffix_kernel_slot_store_contract.sh`
      - guard the landed shared-receiver widening with:
        - fixture: `apps/tests/mir_shape_guard/string_const_suffix_kernel_slot_shared_receiver_min_v1.mir.json`
        - smoke: `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_const_suffix_kernel_slot_shared_receiver_contract.sh`
      - guard the landed direct-set-only `insert_hsi` widening with:
        - smoke: `tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_string_insert_mid_direct_set_min.sh`
      - guard the landed direct-set-only deferred `Pieces3 substring` widening with:
        - fixture: `apps/tests/mir_shape_guard/string_piecewise_kernel_slot_store_min_v1.mir.json`
        - smoke: `tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_string_piecewise_direct_set_min.sh`
      - guard the landed source-only `substring_concat3_hhhii` subrange store widening with:
        - fixture: `apps/tests/mir_shape_guard/array_string_len_piecewise_concat3_source_only_min_v1.mir.json`
        - smoke: `tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_array_string_len_piecewise_concat3_source_only_min.sh`
    - landed source-only concat3 subrange store:
      - exact front is closed again after the compact 8-block matcher fix: `kilo_micro_array_string_store = C 11 ms / Ny AOT 10 ms`, `ny_aot_instr=26922130`
      - middle guard after the exact route-shape follow-up: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 9 ms`, `ny_aot_instr=127269397`
      - loop IR now keeps the source in array text residence:
        - `array.string_len_hi`
        - `array.kernel_slot_insert_hisi`
        - `string.kernel_slot_substring_hii_in_place`
        - `array.kernel_slot_store_hi`
      - forbidden loop calls are gone on this shape:
        - `array.slot_load_hi`
        - `string.substring_hii`
        - `string.substring_concat3_hhhii`
        - `array.set_his`
      - next owner proof must start from fresh whole-front perf/asm before picking the next helper interior
    - older local probe after landing the cold retirement sink:
      - `kilo_meso_substring_concat_array_set_loopcarry = 53 ms` (`repeat=3`, prior local reread `56 ms`)
      - `kilo_kernel_small_hk = 733 ms`, `736 ms` (`repeat=3` x2)
      - current read: this is a valid narrow probe and a slight meso lift, but the whole-front keeper win is not locked yet
    - rejected probe:
      - direct `StringBox -> handle` publish plus string-specialized host-handle payload
      - `kilo_meso_substring_concat_array_set_loopcarry = 68 ms`
      - `kilo_kernel_small = 950 ms`
      - reverted; this seam does not shrink producer publication
    - `kilo_meso_substring_concat_array_set_loopcarry` remains the contradiction guard:
      - if whole improves but meso stays flat-to-worse, reopen `substring_hii -> borrowed_substring_plan_from_handle` as the next card

## Restart Handoff

- this block is the current truth for restart; if older numbers below disagree, prefer this block
- restart with the code as it is now
- current owner snapshot is kept separately at:
  - `docs/development/current/main/investigations/phase137x-array-store-owner-snapshot-2026-04-18.md`
- runtime-wide pattern anchor is now:
  - `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
- hot-corridor carrier design anchor is now:
  - `docs/development/current/main/design/string-hot-corridor-runtime-carrier-ssot.md`
- current upstream string corridor design anchor is now:
  - `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
- current sibling/background lanes:
  - `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
  - `docs/development/current/main/phases/phase-163x/README.md`
  - `docs/development/current/main/design/primitive-family-and-user-box-fast-path-ssot.md`
- pre-optimization cleanup anchor is now:
  - `docs/development/current/main/design/vm-fallback-lane-separation-ssot.md`
- perf release gate now builds `ny-llvmc` as well; do not run exact/asm probes after editing compiler sources without refreshing release artifacts first
- mixed accept gate stays `kilo_micro_substring_only`
- substring exact fronts are now keeper checks, not active blockers:
  - `kilo_micro_substring_only`
  - `kilo_micro_substring_concat`
  - `kilo_micro_substring_views_only`
  - `kilo_micro_len_substring_views`
- current active owner proof front after phase-2.5 is strict whole `kilo_kernel_small_hk`; `kilo_micro_array_string_store` is the exact guard
- current side diagnostic front is `indexOf`
- current owner split is now explicit:
  - exact micro owner: shared generic publish/objectize behind `string_concat_hh` + `string_substring_concat_hhii`
  - historical whole producer owner: `const_suffix` fallback + `freeze_text_plan(Pieces3)` publication
  - current phase-2.5 owner proof: read-side encode/materialize/objectize plus libc copy/alloc tax
  - accepted first implementation cut:
    - runtime-private producer substrate only
    - `const_suffix -> KernelTextSlot -> store.array.str`
    - no general `TransientText` rollout on this lane
- current live reread after parking the rejected slot-store boundary probes:
  - keeper fronts:
    - `kilo_micro_substring_only`
      - `C: 3 ms`
      - `Ny AOT: 3 ms`
    - `kilo_micro_substring_concat`
      - `C: 2 ms`
      - `Ny AOT: 3 ms`
  - active owner fronts:
    - `kilo_micro_array_string_store`
      - `C: 10 ms`
      - `Ny AOT: 131 ms`
    - `kilo_kernel_small`
      - `C: 80 ms`
      - `Ny AOT: 741 ms`
  - current keeper diff:
    - perf AOT direct emit now uses the same trusted stage1 route as the phase direct-route smokes
    - active perf MIR is back on the proof-bearing `substring_concat3_hhhii` payload instead of the older plain `insert_hsi -> substring_hii` payload
    - keep the landed `substring + const + substring -> insert_hsi + final substring_hii` MIR rewrite fixed
    - pure-first now defers publication on the active `insert_hsi -> substring_hii` corridor and emits runtime-private `nyash.string.piecewise_subrange_hsiii`
    - runtime helper stays single-session and materializes once after the text-read session closes; the deadlock-inducing in-session handle issue path is removed
    - current main owner moved away from substring and onto array/string-store family
    - trusted direct MIR no longer duplicates the `text + "xy"` producer across `set(...)` and trailing `substring(...)`
    - runtime wall time stayed open after the compiler-side fix, so duplicated producer birth is no longer the live owner
    - compiler-side known string-length propagation is now landed across const / substring-window / same-length string `phi`
    - active AOT entry IR on this front no longer emits `nyash.string.len_h` in `ny_main`
- latest `perf-observe` top report on the active array-store front:
  - `issue_fresh_handle: 16.60%`
  - `freeze_owned_bytes: 13.31%`
  - `StringBox::perf_observe_from_owned: 12.29%`
  - `capture_store_array_str_source: 11.47%`
  - `string_concat_hh_export_impl: 9.18%`
  - `string_substring_concat_hhii_export_impl: 6.50%`
  - `execute_store_array_str_slot_boundary: 6.40%`
  - `LocalKey::with: 6.35%`
  - `string_substring_concat_hhii_export_impl: 5.61%`
  - `host_handles::with_text_read_session closure: 5.23%`
  - `execute_store_array_str_contract: 4.47%`
- current counter reread on the same front:
  - `str.substring.route total=0`
  - `slow_plan=0`
  - `slow_plan_view_span=0`
  - `piecewise_subrange total=300000`
  - `piecewise_subrange single_session_hit=300000`
  - `piecewise_subrange fallback_insert=0`
  - `piecewise_subrange all_three=300000`
  - `birth.placement fresh_handle=300000`
  - `birth.backend materialize_owned_total=300000`
  - `birth.backend string_box_new_total=300000`
  - `birth.backend arc_wrap_total=300000`
  - `birth.backend handle_issue_total=300000`
  - `stable_box_demand text_read_handle_latest_fresh=299999`
- latest landed observability split:
  - exact micro:
    - `lookup.registry_slot_read=800000`
    - `lookup.caller_latest_fresh_tag=800000`
    - `site.string_concat_hh.materialize_owned_total=800000`
    - `site.string_substring_concat_hhii.materialize_owned_total=800000`
    - slot exit boundary counters are now available:
      - `publish_boundary.slot_publish_handle_total`
      - `publish_boundary.slot_objectize_stable_box_total`
      - `publish_boundary.slot_empty`
      - `publish_boundary.slot_already_published`
  - whole kilo:
    - `const_suffix freeze_fallback=479728`
    - `freeze_text_plan_pieces3=60000`
    - `publish_reason.generic_fallback=539728`
    - `site.string_concat_hh.*=0`
    - `site.string_substring_concat_hhii.*=0`
- current design verdict:
  - the active perf keeper was blocked first by a direct-emit route mismatch
  - perf AOT had been using bare `hakorune --emit-mir-json`, which emitted the older plain `insert_hsi -> substring_hii` payload on this benchmark
  - the trusted stage1 direct route already emitted the proof-bearing `substring_concat3_hhhii` payload pinned by the phase smokes
  - aligning perf AOT to that trusted route collapses the active exact front to near parity without reopening runtime/public-ABI work
  - on the trusted route, route selection / publication boundary is not the current blocker for `kilo_micro_substring_concat`
  - keep the landed runtime-private slot seam as background structure, not as the active blocker
  - this does not mean Hakorune is “a language that cannot remove boxes”; the MIR/publication contract already permits delayed publication on this lane
  - phase-137x should be read as `value-first / box-on-demand / publish-last`
    rather than `box禁止`
  - `publish-last` on this lane is a cold-adapter rule, not a ban on boxes
  - the current deficit is that the box-delayed shape is not yet the natural mainline runtime carrier: the active string lane still returns to public handle world at the executor tail instead of flowing through unpublished outcome as the steady-state representation
  - the current `.hako -> MIR proof/publication -> runtime-private executor -> LLVM consumer` design is still coherent
  - before a `.hako` pilot, lock kernel-common observability vocabulary and keep the comparison protocol-normalized
  - do not compare `Rust vs .hako` by changing language and seam at the same time
  - compare in two stages:
    - `Stage A: same protocol`
    - `Stage B: same public ABI / different internal seam`
  - the first pilot stays narrow on `store.array.str`
  - keep `host_handles` / objectize / fresh-handle issue in Rust during Stage A
  - landed Stage A owner-side pilot on the VM/reference lane:
    - `.hako` `ArrayCoreBox` now routes proven string-handle `set(...)` through `nyash.array.set_his`
    - `RawArrayCoreBox` / `PtrCoreBox` now carry the same-protocol string-store seam
    - `ArrayStateCoreBox` now has a text-aware state helper for the owner-side VM lane
    - source-contract locks:
      - `tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh`
      - `tools/smokes/v2/profiles/integration/apps/phase29cc_runtime_v0_adapter_fixtures_vm.sh`
      - `tools/smokes/v2/profiles/integration/ring1_providers/ring1_array_string_provider_vm.sh`
  - Stage A exact reread is closed and parked:
    - active AOT already reaches the current concrete `store.array.str` lowering without the VM/reference owner pilot
    - trusted direct MIR on the same benchmark still carries generic `RuntimeDataBox.set(...)` / `substring(...)` calls
    - active AOT lowering fact stays pinned separately:
      - direct MIR stays generic
      - entry LLVM IR still calls `nyash.array.set_his`
      - guard: `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_array_store_string_contract.sh`
    - latest locked exact counter facts on this front remain:
      - `store.array.str total=800000`
      - `cache_hit=800000`
      - `plan.action_retarget_alias=800000`
      - `plan.action_store_from_source=0`
      - `plan.action_need_stable_object=0`
      - `carrier_kind.source_keep=0`
      - `carrier_kind.owned_bytes=1600000`
      - `carrier_kind.stable_box=1600000`
      - `carrier_kind.handle=1600000`
      - `publish_reason.generic_fallback=1600000`
  - rejected slot-store boundary delayed-publication probes:
    - v1:
      - `kilo_micro_array_string_store = 252 ms`
      - `kilo_kernel_small_hk = 765 ms`
    - v2:
      - `kilo_micro_array_string_store = 211 ms`
      - `kilo_kernel_small_hk = 1807 ms`
    - producer-side unpublished-outcome active probe:
      - `kilo_micro_array_string_store = 236 ms`
      - `kilo_kernel_small_hk = 2173 ms`
    - keeper from that card:
      - `b35382cf9 feat: add kernel text slot store helpers`
      - runtime-side alias-retarget repair for kernel-slot store into existing string slots
    - rejected reading:
      - the bad cut was the array-store boundary itself
      - the probe bypassed the existing `set_his` fast path / alias-retarget behavior
      - the producer-side slot route also regressed once activated on the live front, so it stays parked
  - latest reverted-baseline 3-run reread:
    - `kilo_micro_array_string_store = C 10 ms / Ny AOT 127 ms`
    - `kilo_kernel_small_hk = C 81 ms / Ny AOT 755 ms`
    - judge future keepers on repeated 3-run windows; current WSL variance is real
  - next step is not more owner widening; it is reopening publication/source-capture before `nyash.array.set_his`
  - the `concat_hh + len_h` compiler-known-length slice is now landed; next first slice is no longer `len_h` removal
  - design tighten before code:
    - keep carrier and publication physically separated; the corridor-local slot transports value, the cold adapter owns `StringBox` / `Arc` / handle issue
    - treat published-ness as boundary bookkeeping, not as the steady-state hot-lane value shape
    - do not broaden this proof into a generic slot API or remembered chain substrate
  - latest design consult is accepted in narrowed form:
    - do not add syntax or public raw-string carriers on this lane
    - `const_suffix` is now the first whole-front narrow probe on the reopened lane
    - keep `Pieces3` / `insert_hsi` as the secondary comparison / guard lane, not the first code cut
    - if that probe reopens, prefer existing runtime-private `TextPlan::Pieces2` / `OwnedBytes` seams before inventing a new carrier
    - treat unique-`OwnedBytes` in-place append as a second-stage follow-up, not the first proof
  - latest owner verdict tighten:
    - whole first owner is `const_suffix` / `nyash.string.concat_hs`
    - `ny_main` loop shape is already close to C; the live gap is inside helper bodies
    - target shape remains:
      - hot path = source read -> size calc -> alloc/copy leaf -> sink
      - cold path = publish adapter / bridge / TLS init / tracing
- result-representation consult triage:
  - adopt:
    - separate semantic result birth from public handle publication
    - keep the public handle-based surface stable on this lane
    - keep `proof_region` and `publication_boundary` MIR-owned
    - treat the generic contract as `same-corridor unpublished outcome`
    - realize it on this lane as `string-lane unpublished text outcome`
    - keep the contract generic, but keep the current implementation string-first; this lane is proving a string-local unpublished text outcome, not a generic helper substrate
    - use the existing runtime-private seams `OwnedBytes` / `TextPlan`
    - add an internal result manifest between Birth / Placement and Value Repr / ABI
    - keep public handle ABI and internal direct-kernel result ABI as separate layers
    - add publication/objectization legality verifier rules so early `StableBoxNow` / `FreshRegistryHandle` becomes mechanically illegal on the active corridor
    - keep new executor legality out of runtime and shim code
  - hold:
    - the exact runtime-private outcome shape (`PlacementOutcome`, out-param, tagged return, etc.)
    - the exact runtime-private result ABI shape and where the cold publish adapter lives
    - whether a later keep-token class is needed after the phase-137x minimal `OwnedBytes` lane
  - reject:
    - runtime/shim remembered-chain legality or route re-recognition
    - generic helper widening
    - public ABI rethink on this lane
    - registry-backed unpublished carriers
    - syntax expansion or public raw-string exposure on this lane
- recommended execution order from the current state:
  1. keep the compiler-known-length keeper fixed and guarded
  2. reopen producer-side publication/source-capture before `nyash.array.set_his`
  3. preserve the existing `set_his` fast path while adding a narrow unpublished-outcome A/B probe on `const_suffix`
  4. add plan-local counters for that probe before any route widening
  5. only if the producer-side unpublished probe wins, run a narrow `const_suffix -> TextPlan::Pieces2` exact-front A/B
  6. only after `Pieces2` proves out, consider unique-`OwnedBytes` in-place append as the next fast path
- current test acceptance note:
  - use `cargo test -q -p nyash_kernel --lib -- --test-threads=1` as the deterministic lane gate
  - parallel `cargo test -q -p nyash_kernel --lib` is still monitor-only on this lane because cache/view tests are parallel-flaky
  - use `cargo check --features perf-observe -p nyash_kernel` before trusting seam-level perf reads
- rejected executor-local non-wins after the publication-boundary keeper:
  - attempted a runtime-private direct string birth to bypass part of the generic materialize/objectize path
  - exact front reread:
    - `kilo_micro_substring_concat`
      - `C: instr=1,622,920 / cycles=507,287 / ms=3`
      - `Ny AOT: instr=261,219,009 / cycles=66,448,479 / ms=23`
  - reading:
    - a direct owned-string birth inside the current handle/box representation did not beat the keeper baseline
  - attempted to switch the `piecewise` source read to `with_text_read_session_ready(...)`
  - exact front reread:
    - `kilo_micro_substring_concat`
      - `C: instr=1,622,875 / cycles=495,910 / ms=3`
      - `Ny AOT: instr=261,219,612 / cycles=66,434,822 / ms=22`
  - reading:
    - registry-ready read entry is not the remaining bottleneck on this front
  - combined reading:
    - the current gap is no longer “which read/helper path do we take”
    - the current gap is “what representation do we force for the final result”
- rejected runtime-private piecewise carrier probe:
  - attempted to issue a transient piecewise box/handle from `insert_const_mid_fallback` and then fast-path `substring_hii` through that carrier
  - exact front reread:
    - `kilo_micro_substring_concat`
      - `C: instr=1,622,877 / cycles=498,662 / ms=3`
      - `Ny AOT: instr=1,027,840,243 / cycles=316,717,873 / ms=78`
  - accept gate stayed healthy:
    - `kilo_micro_substring_only`
      - `C: instr=1,622,874 / cycles=497,164 / ms=3`
      - `Ny AOT: instr=1,669,164 / cycles=1,117,447 / ms=3`
  - rejected asm/top reread:
    - `nyash.string.substring_hii: 29.61%`
    - `insert_const_mid_fallback closure: 25.13%`
    - `PiecewiseTextBox::clone: 16.28%`
    - `string_span_cache_put: 5.27%`
    - `TextPlan::from_pieces: 4.64%`
  - reading:
    - transient piecewise object birth, clone, and allocation dominated the hot lane
    - this front should not mint runtime-private box/handle carriers as the next executor cut
- rejected single-session memo follow-up:
  - attempted to remember `source_handle/split/middle_ptr` behind the produced `insert_hsi` handle and short-circuit the next `substring_hii` before the generic slow-plan lane
  - exact front reread:
    - `kilo_micro_substring_concat`
      - `C: instr=1,622,875 / cycles=484,039 / ms=2`
      - `Ny AOT: instr=1,027,840,321 / cycles=315,379,190 / ms=80`
  - accept gate stayed healthy:
    - `kilo_micro_substring_only`
      - `C: instr=1,622,875 / cycles=502,466 / ms=3`
      - `Ny AOT: instr=1,669,594 / cycles=1,098,352 / ms=3`
  - asm/top reread:
    - `nyash.string.substring_hii: 31.81%`
    - `insert_const_mid_fallback closure: 24.92%`
    - `PiecewiseTextBox::clone: 13.63%`
    - `string_span_cache_put: 9.73%`
    - `TextPlan::from_pieces: 4.31%`
  - reading:
    - a raw handle-keyed sticky memo does not delete the hot executor body; it only adds another shortcut in front of it
    - do not reopen memo-based substring shortcuts on this front; the next cut must stay executor-local and non-sticky
- rejected generic direct-build widening:
  - attempted to read the non-empty `insert_hsi` source in-session and materialize the inserted string directly before the old `TextPlan` fallback
  - exact front reread:
    - `kilo_micro_substring_concat`
      - `C: instr=1,622,920 / cycles=526,196 / ms=3`
      - `Ny AOT: instr=474,559,696 / cycles=165,012,319 / ms=45`
  - accept gate stayed healthy:
    - `kilo_micro_substring_only`
      - `C: instr=1,622,875 / cycles=491,060 / ms=3`
      - `Ny AOT: instr=1,669,350 / cycles=1,050,465 / ms=3`
  - whole-kilo guard regressed:
    - `kilo_kernel_small_hk: 789 ms`
  - asm/top reread:
    - `nyash.string.substring_hii: 30.29%`
    - `insert_const_mid_fallback closure: 28.59%`
    - `borrowed_substring_plan_from_handle: 17.38%`
    - `LocalKey::with: 15.34%`
    - `__memmove_avx512_unaligned_erms: 2.45%`
  - reading:
    - the generic `insert_hsi` direct-build is too wide for this card: it wins the exact front but loses whole-kilo
    - keep generic `insert_const_mid_fallback` materialization unchanged; the next executor cut must stay corridor-local to the active front
- rejected runtime-private deferred-owned-text publication:
  - attempted to keep the public handle-based surface stable while storing fresh `piecewise_subrange_hsiii` results as deferred owned text in the host-handle registry
  - exact front reread:
    - `kilo_micro_substring_concat`
      - `C: instr=1,622,919 / cycles=485,619 / ms=2`
      - `Ny AOT: instr=655,162,062 / cycles=284,596,162 / ms=65`
  - accept gate stayed healthy:
    - `kilo_micro_substring_only`
      - `C: instr=1,622,875 / cycles=488,508 / ms=3`
      - `Ny AOT: instr=1,669,455 / cycles=1,013,477 / ms=3`
  - asm/top reread:
    - `insert_const_mid_fallback closure: 53.28%`
    - `nyash.string.substring_hii: 18.79%`
    - `LocalKey::with: 9.76%`
    - `borrowed_substring_plan_from_handle: 4.37%`
  - reading:
    - the registry-backed deferred owned-text handle was not transparent to the loop-carried active corridor
    - the exact front fell off the landed `piecewise_subrange_hsiii` fast path and repinned to the generic `insert_hsi -> substring_hii` route
    - the broken property was loop-carried fast-path continuity, not legality or publication-boundary proof
    - treat registry-backed deferred owned-text publication as reject on this lane unless a later proof keeps next-iteration pure-string consumers on the landed piecewise fast path
- landed BoxShape cleanup before reopen:
  - `string_helpers/concat.rs` hot/cold split is landed
  - `string_view.rs` now keeps `substring_plan` / `span_resolve` behind submodule seams
  - `runtime/host_handles.rs` now keeps `perf_observe` / `text_read` behind submodule seams
- current restart order after those cleanup commits:
  1. keep the new exact-front keeper (`260,619,140 instr / 21 ms`) as the only reopen baseline
  2. lock the generic borrowed-view substrate and delete-oriented task order in the SSOT docs
  3. keep the landed `mir-rewrite` fixed and choose the next `mir-proof` card from the refreshed consult + source review, not from pre-cleanup numbers
  4. only after that proof lands, reopen the matching `runtime-executor` follow-on
  5. do not reopen more BoxShape cleanup unless the next keeper attempt points at a new mixed-responsibility seam
- adopted reading for the next local cut:
  - this front is a borrowed-view lane continuity problem, not a cache-first or leaf-semantics problem
  - keep `borrowed-view -> materialize-on-escape` as the generic substrate
  - do not add a new string-only MIR dialect
  - landed measurement: the slow-plan arm split is now frozen evidence and the live hot arm is `ViewSpan` only
  - the required BoxShape cleanup is already landed; do not reopen more structure work before another executor-local cut unless tests or asm point at a new mixed-responsibility seam
  - the delete-oriented `mir-rewrite` is now landed on the active front
  - measurement is now closed on this front; the landed `mir-proof` card fixed the publication contract and the next live card is `runtime-executor`
  - the next local target is:
    - pause local tail thinning
    - keep the landed `piecewise_subrange_hsiii` publication boundary fixed
    - carry the measured fast path as frozen evidence: `single_session_hit=300000`, `fallback_insert=0`, `all_three=300000`
  - landed `mir-proof`:
    - the active corridor result no longer relies on shim-local heuristics alone
    - pure-first now requires the MIR-owned publication contract before using the deferred `piecewise_subrange_hsiii` route
    - shim-local `remember_deferred_piecewise_subrange(...)` / `find_deferred_piecewise_subrange(...)` stay transport-only glue under that contract; they are not proof owners and must not grow legality
  - next explicit card is `runtime-executor`:
    - consume the landed MIR publication contract; do not add new proof on this card
    - split runtime-private freeze vs publish on the active corridor only
    - use existing `OwnedBytes` / `TextPlan` seams before considering any broader representation work
    - touch set is intentionally narrow:
      - `crates/nyash_kernel/src/exports/string_helpers/concat/piecewise.rs`
      - `crates/nyash_kernel/src/exports/string_helpers/materialize.rs`
      - `crates/nyash_kernel/src/plugin/value_codec/string_materialize.rs`
    - next delete target is the eager publication tail only:
      - `StringBox`
      - `Arc`
      - fresh `handle_issue`
    - preserve loop-carried fast-path continuity explicitly: next-iteration pure-string consumers must stay on the landed `piecewise_subrange_hsiii` route
    - do not touch:
      - `src/runtime/host_handles.rs`
      - `src/runtime/host_handles/text_read.rs`
      - public export signatures in `crates/nyash_kernel/src/exports/string.rs`
    - do not reopen route logic, piece-shape branching, transient box/handle carriers, sticky memo shortcuts, generic direct-build widening, registry-backed deferred carriers, or runtime/shim re-recognition while this two-card return is pending
  - the follow-on `llvm-export` card only starts after that executor card lands:
    - consume the stabilized corridor with truthful facts
    - do not reopen route eligibility in LLVM metadata
  - string is the first consumer, not the MIR truth itself; keep the substrate generic enough for later `len` / `compare` / `store` consumers
  - keep handle/TLS/cache lookup isolated as the cold adapter path; reject cache/helper accretion without lane-continuity proof
  - do not treat helper names as MIR truth; keep `root/provenance/start/len/materialize_policy/consumer_capability` as the generic minimum
  - split the MIR-side contract in two:
    - `proof_region`: where the borrowed corridor fact is valid
    - `publication_boundary`: where the runtime-private executor may be published
  - do not call that split `scope_lock`; it conflicts with `.hako` lexical/semantic scope
  - the executor itself should be generic enough to serve future piecewise consumers; do not mint another front-specific helper family if `piecewise_subrange_exec(...)` can carry the same load
  - the rejected runtime-executor probes now show both failure modes:
    - transient box/handle carriers lose on this front
    - raw handle-keyed sticky memo shortcuts also lose on this front
    - generic non-empty `insert_hsi` direct-build widening also loses once whole-kilo is checked
    - structure-only `freeze -> publish` splitting on the active `piecewise` tail also loses without deleting eager publication
    - piecewise-local uncached fresh-result len-cache publication is also a non-win; cache seeding is not the dominant publication tax here
    - registry-backed deferred owned-text publication also loses because it breaks the loop-carried active fast path and repins the exact front to the generic `insert_hsi -> substring_hii` corridor
  - the next attempt must keep pieces executor-local and non-sticky
  - latest runtime-private seam reread:
    - `freeze_owned_bytes(...) -> publish_owned_bytes(...)` on the active `piecewise_subrange_hsiii` tail compiled and passed targeted tests, but exact-front reread moved to `261,219,101 instr / 21 ms`
    - treat that slice as reject on this lane; it validates the seam shape but does not remove the hot publication tax
  - rejected shared-materialize OwnedBytes seam:
    - making the phase-137x minimal internal-result seam explicit through the shared materialize/publication path kept the exact front near the keeper band (`261,218,390 instr / 19 ms`) and kept `substring_only` healthy
    - but whole-kilo regressed to `ny_aot_ms=1965` while `c_ms=77`
    - read this as shared-helper scope widening, not as a rejection of the `OwnedBytes` carrier direction itself
    - do not reopen this shape by changing generic `string_handle_from_owned` or other shared materialize/publication helpers; future `OwnedBytes` work must stay corridor-local or direct-kernel-local
  - landed corridor-local slot seam:
    - `piecewise_subrange_hsiii` now freezes into a local `KernelTextSlot` and publishes from that slot at the executor tail
    - exact front reread:
      - `kilo_micro_substring_concat`
        - `C: instr=1,622,875 / cycles=483,683 / ms=3`
        - `Ny AOT: instr=261,218,727 / cycles=65,812,780 / ms=21`
      - `kilo_micro_substring_only`
        - `C: instr=1,622,876 / cycles=495,319 / ms=3`
        - `Ny AOT: instr=1,669,235 / cycles=1,025,821 / ms=3`
      - `kilo_kernel_small_hk`
        - `Ny AOT: ms=684`
    - reading:
      - this is a scope correction, not the final loop-carried slot transport
      - shared helpers stay untouched
      - registry remains cold publish only
      - next card must carry the slot across same-corridor consumers

## Before Next Optimization

The next perf cut should not start until these mechanical contracts are fixed.

1. `KernelTextSlot` lifecycle
   - caller-owned
   - clear-on-overwrite
   - clear-on-drop / early-return
2. slot-capable consumer rule
   - same-corridor `slot_text` consumers may stay in slot form
   - non-slot consumers must get one explicit cold publish from lowering
3. lowering verifier
   - reject early `StableBoxNow`
   - reject early `FreshRegistryHandle`
   - reject registry-backed carrier
4. direct-kernel-local slot transport
   - current landing is corridor-local only
   - next follow-on must thread slot -> slot across same-corridor consumers
- current broader-corridor genericization rule:
  - do not add a new string-only MIR dialect
  - landed: `string_corridor_candidates` now carry proof-bearing plan metadata for borrowed-slice and concat-triplet routes
  - landed: direct `substring_concat3_hhhii` helper results now stay on the same proof-bearing lane with concat-triplet-backed `publication_sink` plan metadata
  - landed: direct helper-result `length()` / `substring()` now consume that same `publication_sink` plan in `string_corridor_sink`
  - landed: first non-`phi` `materialization_sink` slice now sinks a direct `substring_concat3_hhhii` helper birth to a single local `ArrayBox.set` boundary when only copy aliases separate the helper from the store
  - landed: first post-store observer slice now keeps `array.set` as the first `Store` boundary while rewriting one trailing helper-result `length()` observer to `end - start` and deleting the copy-only observer/store chains
  - landed: first plan-selected `direct_kernel_entry` slice now reads `string_corridor_candidates[*].plan.start/end` on direct helper-result receivers and lowers `length()` as window arithmetic in boundary `pure-first`
  - targeted proof: `apps/tests/mir_shape_guard/string_direct_kernel_plan_len_window_min_v1.mir.json` + `tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_string_direct_kernel_plan_len_min.sh`
  - landed: second plan-selected `direct_kernel_entry` slice now reads concat-triplet piece carriers from `string_corridor_candidates[*].plan.proof` on helper-result receivers and lowers `substring()` through `substring_concat3_hhhii` without relying on remembered concat-chain state on that lane
  - targeted proof: `apps/tests/mir_shape_guard/string_direct_kernel_plan_substring_window_min_v1.mir.json` + `tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_string_direct_kernel_plan_substring_min.sh`
  - next: keep loop-carried `phi_merge` outside this cut, and treat the remaining concat backlog as the final emitted-MIR return-carrier cleanup rather than more exact-bridge rediscovery
  - landed exact-front follow-on: `phase-171x` now keeps the pure-first exact seed loop-shape cut on `kilo_micro_substring_concat`; latest reread after that cut is `ny_aot_instr=5,565,470 / ny_aot_cycles=5,893,313 / ny_aot_ms=5`, and current `ny_main` now keeps only the latch compare
  - landed exact-front follow-on: `phase-172x` now consumes the landed `%21 stable_length_scalar -> %5` witness through the header string-lane phi, so the exact seed switches from text rotation to the existing length-only route
  - latest reread after that cut is `ny_aot_instr=1,666,187 / ny_aot_cycles=1,049,205 / ny_aot_ms=4`; the first `instr < 5.5M` keeper target is cleared and the broader publication reopen is now down to the final emitted-MIR return-carrier cleanup
  - landed `phase-173x`: same-block direct-helper `return` publication sink now consumes the same `publication_sink` plan metadata under a focused unit guard
  - landed `phase-174x`: same-block canonical `Store { value, .. }` / `FieldSet { value, .. }` write boundaries now consume that same `publication_sink` plan metadata under a focused unit guard
  - landed `phase-175x`: same-block `RuntimeDataBox.set(...)` now consumes that same `publication_sink` plan metadata as the first host-boundary publication slice under a focused unit guard
  - keep the remaining broader backlog separate: final emitted-MIR return-carrier cleanup stays outside these cuts
  - fixed return order:
    1. continue shrinking exact-seed structural checks only where the live post-sink metadata contract already proves the route
    2. landed: loop-carried base/root interpretation now sits behind the generic MIR seam `src/mir/phi_query.rs`
    3. landed first narrow `plan window across phi_merge` cut on the single-input backedge phi `%22`; merged `%21` is now explicitly `stop_at_merge` and any broader widening stays in a separate metadata-contract phase
  - migration-safe reading: this lane should keep landing in canonical MIR facts/candidates/sink plus kernel/backend substrate, not in Rust-builder-local shape logic
  - treat exact seed logic in `lang/c-abi/shims/hako_llvmc_ffi_string_loop_seed.inc` as temporary bridge surface to shrink after generic plan-selected routes prove out
  - current bridge-retirement reading is now landed in `phase-179x`: MIR JSON already exports corridor facts/relations/candidates, the backend now exports `metadata.string_kernel_plans`, `string_loop_seed` consumes that plan first for the stable-length len route, and the old loop matcher no longer carries the 14-op len-route fallback
  - current structure-only follow-on is `phase-180x`: extract `StringKernelPlan` owner, stop `relation -> candidate` reverse dependency, split shim readers, then move the remaining substring-concat full-loop route onto exported plan payload before broader DCE resumes
- pure Rust reference compare lane:
  - `benchmarks/rust/bench_kilo_micro_substring_views_only.rs`
  - `tools/perf/bench_rust_vs_hako_stat.sh kilo_micro_substring_views_only 1 3`
  - latest pure Rust reference: `instr=5,667,104 / cycles=1,572,750 / cache-miss=5,254 / ms=3`
  - latest C-like Rust reference: `instr=12,566,914 / cycles=3,404,383 / cache-miss=5,256 / ms=3`
- latest exact reread on the mixed accept gate:
  - `kilo_micro_substring_only: instr=1,669,659 / cycles=1,077,794 / cache-miss=8,810 / AOT 3 ms`
  - split exact reread:
    - `kilo_micro_substring_views_only: instr=466,001 / cycles=841,958 / cache-miss=9,391 / AOT 3 ms`
    - `kilo_micro_len_substring_views: instr=1,672,096 / cycles=1,009,964 / cache-miss=8,902 / AOT 3 ms`
  - current broader-corridor reopen front:
  - `kilo_micro_substring_concat: instr=1,665,135 / cycles=1,127,472 / cache-miss=9,899 / AOT 4 ms`
  - `kilo_micro_array_string_store: c_ms=9 / ny_aot_ms=9`; this family is not the current blocker
- target band for the next keeper:
  - mixed accept gate: hold `instr <= 1.8M`
  - local split `kilo_micro_substring_views_only`: hold `instr <= 0.6M`
  - control split `kilo_micro_len_substring_views`: hold `instr <= 1.8M`
  - broader-corridor reopen `kilo_micro_substring_concat`: hold `instr <= 1.8M` while the final emitted-MIR return-carrier cleanup stays separated from the landed publication slices
  - whole strict: keep `<= 709 ms`; ideal band is `690-705 ms`
- ideal `len_h` steady-state asm shape:
  - direct `STRING_DISPATCH_FN` load once; no `STRING_DISPATCH_STATE` state machine in `nyash.string.len_h`
  - direct `host_handles::DROP_EPOCH` load once
  - primary/secondary handle compare only
  - `JIT_TRACE_LEN_ENABLED_CACHE` load once with cold init off the hot return path
  - trace-off fast hit returns directly
- current whole-kilo health:
  - `tools/checks/dev_gate.sh quick` is green
  - `kilo_kernel_small_hk` strict accepted reread: `ny_aot_ms=709`
  - parity: `vm_result=1140576`, `aot_result=1140576`
- current landed substring truth:
  - boundary `pure-first` now lands the first retained-view `substring_hii` exact-micro consumer slice:
    - `kilo_micro_substring_views_only` now matches a known-positive loop-bound exit-len shape and collapses before `substring_hii` / `len_h` replay
    - exact reread on `2026-04-10`: `instr=465,637 / cycles=704,757 / cache-miss=8,280 / AOT 3 ms`
    - current microasm dump now shows `ny_main` as `mov $0x20, %eax ; ret`
    - reading: the sibling exact micro is no longer the blocker; the next string keeper must move back to the mixed accept gate and broader corridor rewrite family
  - `substring_hii` can reissue a fresh handle from a cached `StringViewBox` object when the transient result handle dropped but the source handle still points to the same live source object
  - `str.substring.route` observe read is now dominated by the steady-state handle-hit path: `view_arc_cache_handle_hit=599,998 / total=600,000`
  - current keeper removes redundant `view_enabled` state from `SubstringViewArcCache`; the cache only runs under `view_enabled`, so the extra key dimension was dead hot-path work
  - `2026-04-09` perf reread on `kilo_micro_substring_views_only`:
    - exact: `instr=34,363,814 / cycles=6,537,017 / cache-miss=10,232 / AOT 4 ms`
    - top: `nyash.string.substring_hii 87.04%`, `ny_main 6.00%`
    - annotate says the first visible tax is still inside the caller entry:
      1. `SUBSTRING_ROUTE_POLICY_CACHE` load/decode
      2. `substring` provider state read + `SUBSTRING_VIEW_ARC_CACHE` TLS entry/state check
      3. only then the steady-state compare path
      4. slow plan / materialize is not the dominant block on this front
  - latest baseline asm reread still shows the next visible tax before the view-arc cache compare block:
    1. `SUBSTRING_ROUTE_POLICY_CACHE` decode
    2. `substring_view_enabled` / fallback provider state reads
    3. only then `SubstringViewArcCache` steady-state compare
  - boundary `pure-first` now consumes MIR JSON `string_corridor_*` for `substring(...).length()`:
    - direct route trace now hits `string_len_corridor -> placement_effect_route_window`
    - single-use retained-slice `length()` / `len()` consumers now also rewrite through the same direct entry even when the slice producer dominates from another block through local copy aliases
    - the current bridge shrink also removes the `substring_len_hii` declaration need from this plan-window lane; metadata is now the only direct-kernel proof source here
    - landed sibling string follow-on: `phase-219x placement-effect route-window len fold`
      - boundary `pure-first` now reads `placement_effect_routes` window first for `substring(...).length()` and the smoke expects `placement_effect_route_window`
    - landed BoxShape-only sibling follow-on: `phase-220x placement-effect route-window len helper cleanup`
      - the route-window branch is now shared behind one helper with identical behavior
    - latest exact reread on `kilo_micro_len_substring_views`: `instr=1,672,259 / cycles=1,022,005 / cache-miss=10,525 / AOT 3 ms`
    - latest split-pack reread on `kilo_micro_substring_views_only`: `instr=466,001 / cycles=841,958 / cache-miss=9,391 / AOT 3 ms`
    - reading: the split single-use retained-view fronts are now closed; multiple-use retained-slice length stays backlog and the next string keeper reopens on broader corridor publication/materialization work
  - boundary `pure-first` now also lands the first generic concat observer pilot:
    - single-use `concat pair/triple -> len()` now defers the concat producer and reads known chain length without forcing handle birth
    - observe direct probe on `kilo_micro_concat_hh_len` now shows:
      - `birth.placement`: all `0`
      - `birth.backend`: `freeze_text_plan_total=0`, `string_box_new_total=0`, `handle_issue_total=0`, `materialize_owned_total=0`, `gc_alloc_called=0`
      - `str.concat2.route=0`, `str.len.route=0`
    - exact reread on `kilo_micro_concat_hh_len`: `instr=7,657,032 / cycles=2,284,266 / cache-miss=8,479 / AOT 4 ms`
    - reading: this closes the first `concat -> len` observer slice
  - boundary `pure-first` now also lands the first generic non-`len` concat consumer slice:
    - compiler-visible `concat pair/triple -> substring(...)` now routes to `nyash.string.substring_concat_hhii` / `nyash.string.substring_concat3_hhhii`
    - dynamic route proof hits `string_substring_route -> substring_concat3_hhhii`
    - reading: this removes the intermediate concat handle birth for substring consumers; remaining concat backlog is the final emitted-MIR return-carrier cleanup
  - broader-corridor keeper repair is now landed:
    - `string_corridor_sink` rewrites `concat(left_slice, const, right_slice).length()` into `substring_len_hii(left) + const_len + substring_len_hii(right)` and keeps `substring(concat3(...))` on `substring_concat3_hhhii`
    - the exact `pure-first` `kilo_micro_substring_concat` seed now accepts both the pre-sink and post-sink body shapes, so this generic sink no longer ejects the exact lane into the slow fallback route
    - landed follow-on `phase-169x` now adds a narrow `stable_length_scalar` relation on merged header `%21` while keeping `%21 = stop_at_merge` for plan windows, so the live `--emit-mir-json` route now emits the collapsed post-sink `interesting_n = 14` body instead of the older complementary-length loop shape
    - the phase29x daily-owner blocker is now cleared too: plain `backend=mir` executes the compiled module again, and the `.hako ll emitter` runtime decl manifest now accepts `nyash.string.substring_len_hii` / `nyash.string.substring_concat3_hhhii`, so the daily smoke reaches the expected owner evidence on the same post-sink fixture
    - current live post-sink shape is now pinned separately by `phase137x_direct_emit_substring_concat_post_sink_shape.sh`, and that smoke now requires the collapsed `source_len + const_len` loop body with no loop `substring_len_hii`; helper-result `%36` still keeps `publication_sink` / `direct_kernel_entry` plans on the live MIR, and the phase29x daily smoke uses the same post-sink contract as its daily owner proof
    - the same post-sink probe now also pins the seed preheader/exit semantics (`StringBox.length()` on entry, then exit `length() + ... + ret`), so those truths are visible outside the seed even though the exact seed still owns the current semantic guard
    - `phase137x_direct_emit_substring_concat_phi_merge_contract.sh` now pins the landed metadata-contract follow-on too: live direct MIR still carries `%21 = phi([4,0], [22,20])` and `%22 = phi([36,19])`, helper-result `%36` still owns the proof-bearing plan window, relation metadata keeps `%22 = preserve_plan_window` and `%21 = stop_at_merge`, and merged header `%21` now also carries `stable_length_scalar` with the entry-length witness
    - the same phi smoke now also pins the header/latch loop semantics (`phi/phi/phi`, positive loop bound, compare `<`, branch, and the latch `const 1` increment), so the remaining exact-seed work moved to a semantic-boundary decision rather than more raw body-shape cleanup
    - structure lock: loop-carried corridor continuity now consumes the generic MIR seam in `src/mir/phi_query.rs`; `src/mir/string_corridor_relation.rs` is now the string-side relation layer, and `string_corridor_placement` only maps stored `facts -> relations -> candidates` continuity to string-lane optimization candidates
    - latest exact reread on `kilo_micro_substring_concat`: `instr=1,666,187 / cycles=1,049,205 / cache-miss=8,799 / AOT 4 ms`
    - latest reread after the stable-length exact-route switch: `instr=1,666,187 / cycles=1,049,205 / cache-miss=8,799 / AOT 4 ms`
    - decision now fixed: stop shrinking the exact seed at the semantic-guard boundary for this phase
      - keep preheader/exit `length` truth plus header/latch loop truth in the seed as the current miscompile-prevention owner
      - treat any future retirement of those semantic guards as a separate contract phase, not as more bridge cleanup in this wave
    - `phase-171x` now keeps only the exact seed loop-shape change beneath that semantic guard
    - `phase-172x` is now landed as the last exact-route-local consumer cut on top of the same semantic guard; it does not reopen the metadata contract itself
  - first broader-corridor `publication_sink` inventory slice is now landed:
    - emitted MIR JSON on `kilo_micro_substring_concat` now keeps the direct `substring_concat3_hhhii` helper result on the same corridor lane with `borrowed_corridor_fusion` / `publication_sink` / `materialization_sink` / `direct_kernel_entry` candidates
    - the helper-result plan is concat-triplet-backed and points at the shared source root plus outer `start/end`
    - reading: helper-result inventory is no longer the gap
  - first broader-corridor `publication_sink` actual transform is now landed too:
    - `string_corridor_sink` rewrites direct helper-result `length()` to `end - start`
    - `string_corridor_sink` composes direct helper-result `substring()` back into `substring_concat3_hhhii` by adding the inner window to the helper's outer window
    - reading: the remaining exact-front gap is the loop-carried `text = out.substring(...)` `phi_merge` route, not missing helper-result inventory or direct helper-result consumers
  - first direct-set insert-mid smoke is now pinned too:
    - `phase137x_boundary_string_insert_mid_direct_set_min.sh` uses the synthetic direct-set probe to observe `string_insert_mid_window`, keep `nyash.string.insert_hsi` in the lowered IR, and require the plan-backed `plan_window_match` route on the synthetic fixture
  - current `substring_len_hii` pilot uses `with_text_read_session_ready(...)` to avoid the hot `REG` ready probe; current helper perf is the mixed sink candidate above
  - split exact reread now puts the retained-view exact micro below `0.5M instr`, so `substring_hii` is no longer the active blocker on that split and `len_h` remains the control split
  - current keeper is on `len_h`: hoist one `handles::drop_epoch()` read in `string_len_fast_cache_lookup()` and reuse it for both cache slots
  - current keeper also keeps `len_h` trace-off steady state thin by tail-calling a tiny fast-return helper instead of carrying `trace_len_fast_hit(...)` inline in the hot cache-hit block
  - current keeper removes the `STRING_DISPATCH_STATE` state machine from emitted `len_h` hot asm by probing `STRING_DISPATCH_FN` directly once
  - current keeper also splits trace state into raw-read + cold-init helpers, so the hot cache-hit path sees one `JIT_TRACE_LEN_ENABLED_CACHE` load
  - current keeper also lands the `drop_epoch()` global mirror: `nyash.string.len_h` now reads `host_handles::DROP_EPOCH` directly, and the `host_handles::REG` ready probe is gone from the hot block
  - split exact reread now clears the sibling retained-view exact micro at boundary `pure-first`; next priority moves back to the mixed accept gate and corridor rewrite family
  - pure Rust reference is the current lower bound for this front; current AOT is about `6.06x instr / 4.10x cycles` over it
  - C-like Rust reference is the current contract-aligned comparison point; current AOT is about `2.73x instr / 1.91x cycles` over it
  - upstream corridor pilot is now structurally landed:
    - single-use `substring(...).length()` chains sink to `nyash.string.substring_len_hii`
    - kernel export + MIR interpreter fallback are in place
    - current status is structural plus perf-positive candidate: compile/test are green, and the mixed accept gate now rereads at `instr=47,270,021 / cycles=28,264,307 / cache-miss=9,191 / AOT 8 ms`
  - `nyash.string.substring_hii` / `nyash.string.len_h` / `trace_borrowed_substring_plan` stay as the fallback semantic carrier
  - WSL validation rule stays `3 runs + perf`
- do not reopen for this lane:
  - `OwnedText` backing for substring source lifetime
  - live-source direct-read widening on `as_str_fast()`
  - global `dispatch` / `trace` false-state fast probes outside `string_len_export_impl()`
  - lifting substring runtime cache mechanics into `.hako` or `MIR`
  - widening `@rune` beyond declaration-local metadata for this lane
  - generic scalar/cache/route frameworks before a second lane proves the same keeper pattern
- rejected local probes are now centralized in:
  - [phase137x-substring-rejected-optimizations-2026-04-08.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase137x-substring-rejected-optimizations-2026-04-08.md)
  - current rejected list:
    1. broad `NyashBox` substring-source contract widening
    2. `substring_view_arc_cache_lookup` / `entry_hit` hot-path fusion
    3. birth-side second `with_handle(...)` removal via planner-local source metadata carry
    4. reissue-side slot carry / `refresh_handle` rematch removal
    5. concrete `Arc<StringViewBox>` cache carrier narrowing
    6. `len_h` cache-first reorder
    7. `drop_epoch_if_ready()` fast accessor probe
    8. global `dispatch` / `trace` false-state fast probes
    9. `len_h` dispatch-hit cold split
    10. `trace_len_state()` helper / trace cache single-load probe
    11. `len_h` two-slot pre-match + single epoch-guard probe
    12. local `dispatch_known_absent_fast` + cold dispatch probe combo
    13. `drop_epoch_after_cache_hit()` ready-after-hit probe
    14. `len_h` dispatch single-probe + raw trace-state split
    15. `len_h` 1-probe hash-slot cache shape
    16. registry-pointer epoch read on len cache hits
    17. `len_h` `ReadOnlyScalarLane` separation-only slice
    18. `len_h` combined `ReadOnlyScalarLane` + entry snapshot slice
    19. `len_h`-specific 4-box slice (`façade + control snapshot + pure cache probe + cold path`)
    20. `SubstringViewArcCache` global compare reorder (`start/end` before `source_handle`)
    21. `SubstringViewArcCache` `same_source_pair` specialization
    22. `substring_hii` common-case body duplication via `route_raw == 0b111`
    23. `substring` provider `raw read + cold init` adoption (`substring_view_enabled` / fallback policy / route policy)
    24. `substring_route_policy()` cold init split while keeping the active caller shape unchanged
    25. `substring_hii` route/provider snapshot + eager `DROP_EPOCH` snapshot
    26. `SubstringViewArcCache::entry_hit` reissue/clear cold split
- next active cut:
  1. keep `kilo_micro_substring_only` as accept gate
  2. use `kilo_micro_substring_views_only` for local `substring_hii` cuts
  3. keep `len_h` runtime mechanics stable unless split fronts move again
  4. latest keeper already removed the remaining `len_h` control-plane hot loads
  5. current pivot is upstream, not another leaf-local `substring_hii` split:
     - `.hako policy -> canonical MIR facts -> placement/effect pass -> Rust microkernel -> LLVM`
  6. do not add a permanent second public MIR dialect for this wave
  7. both `len_lane` separation-only and combined lane+snapshot retries were rejected; lane boundary alone is not the next keeper slice
  8. the earlier `drop_epoch()` global mirror rejection was invalidated by stale release artifacts; the hypothesis is now landed, and future perf reads must rebuild release artifacts first
  9. fixed task order:
     - step 1: docs-first; treat `string-canonical-mir-corridor-and-placement-pass-ssot.md` as the active design owner
     - step 2: landed; inventory canonical string corridor sites and current lowering carriers for `str.slice` / `str.len` / `freeze.str` via `src/mir/string_corridor.rs`
     - step 3: landed; canonical MIR-side fact carrier is `FunctionMetadata.string_corridor_facts`, and verbose dumps plus MIR JSON expose it with no runtime behavior change
     - step 4: landed; `src/mir/string_corridor_placement.rs` now reads `FunctionMetadata.string_corridor_facts`, emits no-op candidate decisions into `FunctionMetadata.string_corridor_candidates`, and exposes them in verbose MIR dumps plus MIR JSON
     - step 5: landed structurally; the first borrowed-corridor sinking pilot now rewrites single-use `substring(...).length()` chains to `nyash.string.substring_len_hii`
     - step 6: landed; `phase-162x vm fallback lane separation cleanup` is complete, so this front now reads through `ny-llvmc(boundary pure-first)` without mixing fallback owners
     - step 7: landed; boundary `pure-first` now consumes MIR JSON `string_corridor_*` metadata for `substring(...).length()` and now reads the route as `string_len_corridor -> placement_effect_route_window`; the route-window fold is landed in `phase-219x`
     - step 8: landed; boundary `pure-first` now also routes compiler-visible concat pair/triple `substring(...)` consumers to `nyash.string.substring_concat_hhii` / `nyash.string.substring_concat3_hhhii`
     - step 9: landed; `FunctionMetadata.string_corridor_candidates` now carries proof-bearing plan metadata on the broader-corridor reopen front `kilo_micro_substring_concat`, and MIR JSON exports the same plan surface
     - step 10: landed; direct `substring_concat3_hhhii` helper results now stay on the corridor metadata lane with concat-triplet-backed `publication_sink` proof
     - step 11: landed; direct helper-result `length()` / `substring()` now consume that same `publication_sink` proof in `string_corridor_sink`
     - step 12: landed; `materialization_sink` now covers the non-`phi` local `ArrayBox.set` store boundary and the first trailing `length()` post-store observer window on the same canonical MIR lane
     - step 13: landed first plan-selected `direct_kernel_entry` slice; boundary `pure-first` now reads plan windows on direct helper-result receivers, lowers `length()` as window arithmetic, and no longer keeps the `substring_len_hii` declaration bridge on that lane
     - step 14: next shrink the remaining dynamic/exact bridge paths that still bypass the plan
     - step 15: landed first narrow `phi_merge` handoff; the single-input backedge phi `%22` now keeps the proof-bearing plan window, while merged header phi `%21` is explicitly `stop_at_merge` and keeps only non-window corridor continuity
     - step 16: landed narrow metadata-contract follow-on in `phase-169x`; merged header `%21` still keeps `stop_at_merge` for plan windows, but now also carries `stable_length_scalar` so the exact front can collapse the loop body to `source_len + const`
     - step 17: any broader plan-window carry across merged `%21`, or `call` / `boxcall` / `return` barrier relaxation, still requires another metadata-contract update first; `phase137x_direct_emit_substring_concat_phi_merge_contract.sh` remains the live guard for that stop line
     - step 18: only after that reopen new `substring_hii` runtime leaf cuts, and only with exact/asm proof
     - step 19: do not retry the same `len_h`-specific 4-box slice as-is; it did not clear exact or asm gates
     - step 20: keep this lane specific; do not generalize into a reusable scalar framework until a second lane wins the same pattern
     - step 20: do not swap the active `substring` providers to `raw read + cold init` as one slice; that provider-adoption cut regressed the local split
     - step 21: do not duplicate the common-case `substring_hii` body again; the earlier `route_raw == 0b111` duplication regressed badly
     - step 20: `substring_route_policy()` cold split alone is also blocked; even with the caller unchanged it regressed the local split
     - step 21: any future `len_h` reopen must preserve direct dispatch probe + single trace-state load + direct `DROP_EPOCH` load
     - step 22: do not retry the same `substring_hii` route/provider snapshot with eager `DROP_EPOCH` capture; it widened the caller prologue and regressed exact/whole together
     - step 23: do not cold-split `SubstringViewArcCache::entry_hit` reissue/clear in isolation; it regressed every split front and whole strict
     - step 24: primitive/user-box follow-on work now lives in `phase-163x`; keep this README string-only
  10. next local cut must show an exact-visible or asm-visible change on `substring_hii`, but only after the upstream corridor slices are in place
- safe restart order:
  1. `git status -sb`
  2. `tools/checks/dev_gate.sh quick`
  3. `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
  4. `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
  5. `docs/development/current/main/phases/phase-163x/README.md`
  6. `src/mir/string_corridor.rs`
  7. after any `nyash_kernel` / `hakorune` runtime source edit, rerun `bash tools/perf/build_perf_release.sh` before exact micro / asm probes
  8. `tools/perf/run_kilo_string_split_pack.sh 1 3`
  9. `tools/perf/bench_micro_aot_asm.sh kilo_micro_substring_views_only 'nyash.string.substring_hii' 200`
  10. read the rejected ledger before retrying any substring-local cut
- documentation rule for failed perf cuts:
  1. keep a short current summary in this README
  2. keep exact rejected-cut evidence in one rolling investigation doc per front/family/date
  3. do not create test-by-test folders unless that artifact family itself becomes an independent lane
- promotion policy for this cache family:
  1. the first proven win can stay local in Rust when one exact front is isolated and measurable
  2. once the same alternating-access pattern appears in another exact front, stop adding route-local cache variants and evaluate a shared hot-cache policy above Rust
  3. lift only when the semantics are common and lifetime / ownership boundaries remain explicit at the higher layer
  4. avoid repeating Rust-local cache additions in the same family without rechecking that promotion condition
- immediate substring follow-up:
  1. `substring_hii` is first target again under the split pair
  2. keep runtime cache mechanics as-is; broad provider adoption into the hot caller lost the local split
  3. read the rejected ledger before retrying any substring-local cut
  4. use the split exact pair before and after every provider-side change
  5. use the landed `substring(...).length()` corridor consumer plus the landed `concat -> substring(...)` carrier as the templates for the next retained-view `substring_hii` cut
  6. retained-view `substring_hii` local shapes remain the next string-only keeper front
  7. next cleanup task must stay narrower than the rejected provider-adoption slice
- lifecycle placement is fixed:
  - `.hako`: source-preserve / identity / publication demand
  - `MIR`: visibility carrier and escalation contract
  - `Rust`: mechanics only
- if the next session needs a quick read order, start here:
  1. this block
  2. `CURRENT_TASK.md`
  3. `git status -sb`

## Fresh Read

- current front is structure-first:
  - `kilo_micro_substring_only`
  - `nyash.string.substring_hii` / `nyash.string.len_h` / `trace_borrowed_substring_plan` source contract as fallback semantic carrier
  - `substring` / `len` cache state now uses flattened TLS records to reduce lookup shape overhead
  - `SourceLifetimeKeep`
  - `RetargetAlias` source-lifetime semantics
  - `concat_birth` fresh-box materialization landed
  - AOT compiler-side literal `string + string` fold landed
 - whole-kilo read order is now fixed through a supported contract split ladder:
    - `kilo_micro_concat_hh_len`
    - `kilo_micro_array_string_store`
    - `kilo_meso_substring_concat_len`
    - `kilo_meso_substring_concat_array_set_loopcarry`
    - `kilo_kernel_small_hk`
  - supported meso ladder is now:
    - `kilo_meso_substring_concat_len`
    - `kilo_meso_substring_concat_array_set`
    - `kilo_meso_substring_concat_array_set_loopcarry`
  - middle benchmark for the current lane is `kilo_meso_substring_concat_array_set_loopcarry`:
    - it keeps `substring + concat + array.set + loopcarry`
    - it drops the `indexOf("line")` row-scan noise from whole-kilo
    - current 3-run lock: `C 3 ms / Ny AOT 53 ms`
  - `kilo_meso_indexof_append_array_set` stays as side diagnosis only:
    - use it when the `indexOf` branch loop itself is the active owner
  - use `tools/perf/run_kilo_kernel_split_ladder.sh` when re-reading whole-kilo after a structural slice
- current probe-only split-ladder reread (`repeat=1`):
  - `kilo_micro_concat_hh_len: 61 ms`
  - `kilo_micro_array_string_store: 176 ms`
  - `kilo_meso_substring_concat_len: 33 ms`
  - `kilo_meso_indexof_append_array_set: 152 ms`
  - `kilo_kernel_small_hk: 700 ms`
- current lifecycle visibility lock for `store.array.str`:
  - public row stays `store.array.str`
  - `.hako` owns source-preserve / identity / publication demand
  - MIR carries that visibility through the existing lowering carrier:
    - `GenericMethodRouteState`
    - `GenericMethodEmitPlan`
  - no-behavior carrier fields are now landed:
    - `array_store_string_source_preserve`
    - `array_store_string_identity_demand_stable_object`
    - `array_store_string_publication_demand_publish_handle`
  - `.hako` owner-side policy methods are now landed:
    - `array_store_string_source_preserve(...)`
    - `array_store_string_identity_demand(...)`
    - `array_store_string_publication_demand(...)`
  - current mirror reads those lifecycle policy fields through `set_route`
  - Rust only executes:
    - `SourceKindCheck`
    - `SourceLifetimeKeep`
    - `AliasUpdate`
    - `NeedStableObject`
- benchmark numbers stay current truth, but they are now validation, not the driver for widening Rust transport
- latest structural visibility split:
  - `value_codec/string_materialize.rs` now owns `OwnedBytes -> StableBoxNow -> FreshRegistryHandle`
  - `value_codec/string_store.rs` now stays on store-from-source execution
  - accept-gate reread:
    - `kilo_micro_array_string_store: 179 ms`
    - `kilo_meso_indexof_append_array_set: 150 ms`
    - `kilo_kernel_small_hk: 695 ms`
- `exports/string.rs` is now a thin export shell with helpers split out
- `plugin/map_substrate.rs` is now raw substrate helpers only
- `plugin/map_aliases.rs` now owns the ABI alias surface
- `nyash_kernel` is ready to be re-baselined under the new responsibility split
- `src/tests.rs` has been split into `tests/filebox.rs` and `tests/string.rs`, so the root test module is no longer a 1000+ line monolith
- reopened perf read:
  - baseline: `kilo_kernel_small_hk`: `c_ms=81 / ny_aot_ms=1529`
  - after string const-path branch collapse: `c_ms=82 / ny_aot_ms=775`
  - after const-handle cache follow-up: `c_ms=84 / ny_aot_ms=731`
  - after const empty-flag cache: `c_ms=81 / ny_aot_ms=723`
  - after shared text-based const-handle helper: `c_ms=80 / ny_aot_ms=903`
  - after single-closure const suffix fast path: `c_ms=83 / ny_aot_ms=820`
  - latest whole-kilo reread after visibility lock: `c_ms=83 / ny_aot_ms=762`
  - `kilo_micro_indexof_line`: `c_ms=4 / ny_aot_ms=4`
  - `kilo_micro_substring_concat`: `c_ms=3 / ny_aot_ms=3`
  - `kilo_micro_array_getset`: `c_ms=4 / ny_aot_ms=4`
  - `kilo_micro_concat_const_suffix`: `c_ms=3 / ny_aot_ms=36`
  - `kilo_micro_concat_hh_len`: `c_ms=3 / ny_aot_ms=4`
  - `kilo_micro_concat_birth`: `c_ms=6 / ny_aot_ms=3`
  - `kilo_micro_array_string_store`: `c_ms=9 / ny_aot_ms=173`
 - latest whole-kilo reread after keep API narrowing: `c_ms=77 / ny_aot_ms=708`
  - latest whole-kilo reread after keep-anchor cold fallback narrowing: `c_ms=79 / ny_aot_ms=696`
- latest bundle read:
  - string contracts remain `keep_transient -> fresh_handle` for non-empty const concat/insert
  - `20260406-024104` still shows `crates/nyash_kernel/src/exports/string_helpers.rs::concat_const_suffix_fallback` as the top explicit hot symbol (`11.70%`)
  - `crates/nyash_kernel/src/plugin/array_string_slot.rs::array_string_store_handle_at` remains second (`5.68%`)
  - exact micro gap is currently larger on `array_string_store`
 - deeper observe drill-down now exists for:
   - `store.array.str`: `existing_slot / append_slot / source_string_box / source_string_view / source_missing`
   - `const_suffix`: `empty_return / cached_fast_str_hit / cached_span_hit`
   - generic string consumer:
     - `str.concat2.route`: `total / dispatch_hit / fast_str_owned / fast_str_return_handle / span_freeze / span_return_handle / materialize_fallback / unclassified`
     - `str.len.route`: `total / dispatch_hit / fast_str_hit / fallback_hit / miss / latest_fresh_handle_fast_str_hit / latest_fresh_handle_fallback_hit / unclassified`
   - `birth.placement`: `return_handle / borrow_view / freeze_owned / fresh_handle / materialize_owned / store_from_source`
   - `birth.backend`: `freeze_text_plan_total / view1 / pieces2 / pieces3 / pieces4 / owned_tmp / materialize_owned_total / materialize_owned_bytes / string_box_new_total / string_box_new_bytes / string_box_ctor_total / string_box_ctor_bytes / arc_wrap_total / handle_issue_total / gc_alloc_called / gc_alloc_bytes / gc_alloc_skipped`
 - exact observe read:
   - `kilo_micro_array_string_store` AOT direct probe is saturated on one shape:
     - `cache_hit=800000`
     - `retarget_hit=800000`
     - `existing_slot=800000`
     - `source_string_box=800000`
   - current cache-churn hypothesis is not supported on that exact micro
   - `kilo_micro_concat_const_suffix` AOT direct probe does not hit `const_suffix`
   - `kilo_micro_concat_hh_len` isolated the generic `concat -> len` consumer without substring carry, and that slice is now landed
   - `kilo_micro_concat_birth` now isolates fresh concat birth/materialize with only final `len`
   - `kilo_micro_concat_birth` direct probe currently shows:
     - `birth.placement`: `fresh_handle=800000`
     - `birth.backend`: `materialize_owned_total=800000`, `materialize_owned_bytes=14400000`, `gc_alloc_called=800000`, `gc_alloc_bytes=14400000`
     - `str.concat2.route`: `fast_str_owned=800000`, other classified routes `0`, `unclassified=0`
     - `str.len.route`: `fast_str_hit=1`, `latest_fresh_handle_fast_str_hit=1`, other classified routes `0`, `unclassified=0`
   - `kilo_micro_concat_hh_len` observe direct probe now shows:
     - `birth.placement`: `return_handle=0 / borrow_view=0 / freeze_owned=0 / fresh_handle=0 / materialize_owned=0 / store_from_source=0`
     - `birth.backend`: `freeze_text_plan_total=0`, `string_box_new_total=0`, `handle_issue_total=0`, `materialize_owned_total=0`, `gc_alloc_called=0`
     - `str.concat2.route`: `total=0`
     - `str.len.route`: `total=0`
     - latest exact reread: `instr=7,657,032 / cycles=2,284,266 / cache-miss=8,479 / AOT 4 ms`
   - `NYASH_PERF_BYPASS_GC_ALLOC=1` diagnostic observe lane still matters only for `kilo_micro_concat_birth`:
     - `kilo_micro_concat_birth`: `50 -> 51 ms`
     - observe-build `kilo_kernel_small_hk`: `1077 -> 1084 ms`
     - direct probe cleanly flips:
       - `gc_alloc_called=800000 -> 0`
       - `gc_alloc_skipped=0 -> 800000`
   - current evidence keeps `kilo_micro_concat_birth` as the remaining concat birth front; the landed `concat_hh_len` observer slice no longer exercises runtime concat/len routes
   - external design lock after the latest exact/whole split:
     - do not treat birth as one fused event
     - read current backend as:
       - byte birth = `MaterializeOwned`
       - object birth = `StableBoxNow`
       - publication birth = `FreshRegistryHandle`
     - next backend-private carriers are:
       - `OwnedBytes`
       - `TextReadSession`
     - next structural goal is to reduce `StableBoxNow` demand before trying to
       make `next_box_id` or registry issue cheaper again
   - source-backed private seam slice is now in place:
     - `OwnedBytes` exists in `string_store.rs`
     - `TextReadSession` exists in `host_handles.rs`
     - `string_len_from_handle(...)`, `string_is_empty_from_handle(...)`,
       `concat_pair_from_fast_str(...)`, and `concat3_plan_from_fast_str(...)`
       now read through the session seam
     - this slice does not reintroduce deferred objectization behavior
   - `StableBoxNow` demand probe now also exists:
     - `kilo_micro_concat_birth`
       - `object_get_latest_fresh=0`
       - `object_with_handle_latest_fresh=0`
       - `object_pair_latest_fresh=0`
       - `object_triple_latest_fresh=0`
       - `text_read_handle_latest_fresh=1`
       - `text_read_pair_latest_fresh=0`
       - `text_read_triple_latest_fresh=0`
     - `kilo_micro_concat_hh_len`
       - `object_get_latest_fresh=0`
       - `object_with_handle_latest_fresh=0`
       - `object_pair_latest_fresh=0`
       - `object_triple_latest_fresh=0`
       - `text_read_handle_latest_fresh=800000`
       - `text_read_pair_latest_fresh=0`
       - `text_read_triple_latest_fresh=0`
     - latest fresh handles are staying inside the single-handle text-read seam on the current exact fronts
     - exact micro evidence does not support object-world leakage as the current first cause
   - delayed `StableBoxNow` retry truth:
     - exact micro improved:
       - `kilo_micro_concat_birth`: `50 -> 37 ms`
       - `kilo_micro_concat_hh_len`: `67 -> 57 ms`
     - whole-kilo still regressed:
       - `kilo_kernel_small_hk`: `764 ms`
     - whole observe probe points at early object-world escalation instead of exact-path leakage:
       - `stable_box_demand.object_with_handle_latest_fresh=540000`
       - `stable_box_demand.object_get_latest_fresh=0`
       - `stable_box_demand.object_pair_latest_fresh=0`
       - `stable_box_demand.object_triple_latest_fresh=0`
       - `stable_box_demand.text_read_handle_latest_fresh=0`
       - `stable_box_demand.text_read_pair_latest_fresh=938`
     - current read:
       - exact micro stays inside the single-handle text-read seam
       - whole-kilo quickly promotes latest fresh string handles into generic object `with_handle(...)`
       - delayed objectization must not be relanded until that consumer is widened or bypassed
       - target assembly shape:
         - `concat_hh + len_h` should stay on text/materialize paths for as long as possible
         - registry/object traffic should appear only at sink/object boundaries, not between concat and immediate len
     - caller-attributed whole-kilo truth:
       - `stable_box_demand.object_with_handle_array_store_str_source_latest_fresh=540000`
       - `stable_box_demand.object_with_handle_substring_plan_latest_fresh=0`
       - `stable_box_demand.object_with_handle_decode_array_fast_latest_fresh=0`
       - `stable_box_demand.object_with_handle_decode_any_arg_latest_fresh=0`
       - `stable_box_demand.object_with_handle_decode_any_index_latest_fresh=0`
     - source-backed `store.array.str` split confirms that this whole-kilo latest-fresh demand is entirely retarget-side:
       - `store.array.str latest_fresh_retarget_hit=540000`
       - `store.array.str latest_fresh_source_store=0`
     - no-behavior-change planner truth now confirms the source-contract mismatch itself:
       - whole-kilo:
         - `plan.source_kind_string_like=540000`
         - `plan.source_kind_other_object=0`
         - `plan.source_kind_missing=0`
         - `plan.slot_kind_borrowed_alias=540000`
         - `plan.slot_kind_other=0`
         - `plan.action_retarget_alias=540000`
         - `plan.action_store_from_source=0`
         - `plan.action_need_stable_object=0`
       - exact `kilo_micro_array_string_store`:
         - `plan.source_kind_string_like=800000`
         - `plan.slot_kind_borrowed_alias=800000`
       - `plan.action_retarget_alias=800000`
       - `plan.action_store_from_source=0`
       - `plan.action_need_stable_object=0`
     - no-behavior-change reason truth now clarifies the remaining contract:
       - whole-kilo:
         - `reason.source_kind_via_object=540000`
         - `reason.retarget_keep_source_arc=540000`
         - `reason.retarget_alias_update=540000`
       - exact `kilo_micro_array_string_store`:
         - `reason.source_kind_via_object=800000`
         - `reason.retarget_keep_source_arc=800000`
         - `reason.retarget_alias_update=800000`
     - borrowed alias whole-kilo truth:
       - `borrowed.alias.borrowed_source_fast=540000`
       - `borrowed.alias.as_str_fast=540064`
       - `borrowed.alias.as_str_fast_live_source=540064`
       - `borrowed.alias.as_str_fast_stale_source=0`
       - `borrowed.alias.array_len_by_index_latest_fresh=1`
       - `borrowed.alias.array_indexof_by_index_latest_fresh=938`
       - `borrowed.alias.encode_epoch_hit=0`
       - `borrowed.alias.encode_ptr_eq_hit=0`
       - `borrowed.alias.encode_to_handle_arc=0`
       - `borrowed.alias.encode_to_handle_arc_array_get_index=0`
       - `borrowed.alias.encode_to_handle_arc_map_runtime_data_get_any=0`
     - current read:
       - retargeted latest-fresh aliases are not escaping through encoder fallback
       - caller-attributed encode-to-handle paths are also closed in current behavior
       - `BorrowedHandleBox::as_str_fast()` stays entirely on the live-source side in whole-kilo
       - `array_string_len_by_index(...)` / `array_string_indexof_by_index(...)` are not the 540k latest-fresh culprit
       - the remaining stable object pressure stays on `store.array.str -> with_handle(ArrayStoreStrSource)` itself, not alias runtime encode
     - full object API demand also stays closed on the current culprit:
       - `borrowed.alias.to_string_box_latest_fresh=0`
       - `borrowed.alias.equals_latest_fresh=0`
       - `borrowed.alias.clone_box_latest_fresh=0`
     - latest landed keep-anchor cold fallback narrowing:
       - `BorrowedHandleBox::{to_string_box,type_name,is_identity}` now derive cold semantics from verified text anchor + keep class instead of stable object fallback
       - `to_string_box` now uses cold owned copy-out
       - `type_name` is derived from `TextKeepClass`
       - `is_identity` is fixed `false`
       - current exact/whole reread:
         - `kilo_micro_array_string_store`: `182 ms`
         - `kilo_micro_concat_hh_len`: `65 ms`
         - `kilo_kernel_small_hk`: `696 ms`
       - read:
         - this is a structure-first slice, not a hot-path trim
         - it removes more object-like behavior from the keep surface
         - remaining cold object fallback work is mostly `equals` and explicit promotion paths
       - current hot path is not using `BorrowedHandleBox` full stable-object APIs at all
     - latest landed encode object-demand sealing:
       - borrowed-alias encode planning/fallback execution now stays inside `value_codec/borrowed_handle.rs`
       - `encode.rs` no longer reaches into alias cold object helpers for:
         - fallback scalar check
         - pointer-equality reuse
         - fallback handle issue
       - removed encode-only cross-module helper surface:
         - `encode_fallback_box_ref()`
         - `clone_stable_box_for_encode_fallback()`
         - `ptr_eq_source_object()`
       - probe-only split-ladder reread (`repeat=1`):
         - `kilo_micro_concat_hh_len: 62 ms`
         - `kilo_micro_array_string_store: 183 ms`
         - `kilo_meso_substring_concat_len: 40 ms`
         - `kilo_meso_indexof_append_array_set: 148 ms`
         - `kilo_kernel_small_hk: 693 ms`
     - latest landed borrowed-alias equals cold split:
       - `BorrowedHandleBox::equals` now routes through a cold helper in `value_codec/borrowed_handle.rs`
       - `maybe_borrow_string_handle_with_epoch(...)` and `maybe_borrow_string_keep_with_epoch(...)` now use cold owned-box promotion helpers for `StringView` / non-borrowable keep paths
       - regression tests now pin:
         - borrowed-alias equality against plain `StringBox`
         - borrowed-alias equality across distinct source handles with the same text
         - `StringView` store-from-source materialization into owned `StringBox`
       - test gate:
         - `cargo test --manifest-path crates/nyash_kernel/Cargo.toml --lib store_string_box_from_source` -> 4 passed
         - `cargo test --manifest-path crates/nyash_kernel/Cargo.toml --lib borrowed_alias_equals_same_text_from_distinct_sources` -> 1 passed
     - latest landed `store.array.str` non-string source-presence split:
       - `ArrayStoreStrSource::OtherObject` no longer transports `Arc<dyn NyashBox>` across the executor seam
       - `with_array_store_str_source(...)` still classifies under `with_handle(...)`, but the non-string branch now carries presence-only contract
       - `maybe_store_non_string_box_from_verified_source(...)` now consumes only `source_handle` / `drop_epoch`
       - regression tests now pin:
         - `with_array_store_str_source(...)` -> `OtherObject` for live non-string handles
         - `with_array_store_str_source(...)` -> `Missing` for dropped handles
       - test gate:
         - `cargo test --manifest-path crates/nyash_kernel/Cargo.toml --lib plugin::value_codec::tests` -> 19 passed
         - `cargo check --manifest-path crates/nyash_kernel/Cargo.toml` -> OK
     - latest landed source-lifetime helper sealing:
       - `array_string_slot.rs` no longer clones keep / rebuilds proof directly from `VerifiedTextSource`
       - string-like retarget/store now go back through `value_codec` helpers:
         - `try_retarget_borrowed_string_slot_take_verified_text_source(...)`
         - `store_string_box_from_verified_text_source(...)`
       - `VerifiedTextSource` now owns keep-preserving rewrite helpers for the retry path
       - regression tests now pin:
         - retarget success from verified string source into borrowed alias slot
         - retry `Err` path preserves `StringView` semantics before store fallback
       - test gate:
         - `cargo test --manifest-path crates/nyash_kernel/Cargo.toml --lib plugin::value_codec::tests` -> 21 passed
         - `cargo check --manifest-path crates/nyash_kernel/Cargo.toml` -> OK
     - latest landed by-value `VerifiedTextSource` consumption:
       - hot retarget path no longer clones keep before calling `try_retarget_borrowed_string_slot_take_keep(...)`
       - `VerifiedTextSource` now hands off keep by value:
         - `into_keep()`
       - string-like store now also consumes keep by value through:
         - `store_string_box_from_source_keep_owned(...)`
         - `store_string_box_from_verified_text_source(...)`
       - regression tests now pin:
         - retarget success from verified string source into borrowed alias slot
         - retry `Err` path preserves `StringView` semantics before store fallback
         - owned keep store path keeps borrowed alias for string handles
       - test gate:
         - `cargo test --manifest-path crates/nyash_kernel/Cargo.toml --lib plugin::value_codec::tests` -> 22 passed
         - `cargo check --manifest-path crates/nyash_kernel/Cargo.toml` -> OK
       - exact front probe (`repeat=3`):
         - `kilo_micro_array_string_store: 174 ms`
     - latest landed source-kind observation split:
       - `with_array_store_str_source(...)` now returns `StringHandleSourceKind` alongside the payload
       - `array_string_slot.rs` now consumes the kind explicitly for planning instead of asking the payload again
       - regression tests now pin:
         - `StringLike`
         - `OtherObject`
         - `Missing`
       - test gate:
         - `cargo test --manifest-path crates/nyash_kernel/Cargo.toml --lib plugin::value_codec::tests` -> 22 passed
         - `cargo check --manifest-path crates/nyash_kernel/Cargo.toml` -> OK
       - exact front probe (`repeat=5`):
         - `kilo_micro_array_string_store: 178 ms`
    - latest landed const-suffix cache split:
      - `execute_const_suffix_contract(...)` now uses module-level cache helpers instead of carrying the text-cache closure shape inside the function body
      - hot cached-handle lookup stays on the same semantics, but the cache/read structure is flatter
       - accept-gate reread:
         - `kilo_micro_concat_const_suffix: 81 ms`
         - `kilo_meso_indexof_append_array_set: 166 ms`
         - `kilo_kernel_small_hk: 696 ms`
     - latest landed const-suffix meta/text cache split:
       - cached metadata and cached suffix text are now stored in separate TLS caches
       - hot cached-handle lookup reads only metadata:
         - `ptr`
         - `handle`
         - `is_empty`
       - `RefCell<Option<String>>` text cache is only touched on reload or cold text fallback
       - accept-gate reread:
         - `kilo_micro_concat_const_suffix: 74 ms`
         - `kilo_meso_indexof_append_array_set: 149 ms`
         - `kilo_kernel_small_hk: 695 ms`
     - landed structural slice:
       - `ArrayStoreStrSource` now owns the source `Arc`
       - `with_array_store_str_source(...)` completes host-handle source read before `arr.with_items_write(...)`
       - `store.array.str` no longer nests host-handle read-lock across planner/retarget execution
     - current 3-run plain-release recheck on the landed slice:
       - `kilo_micro_array_string_store: 189 ms`
       - `kilo_micro_concat_hh_len: 67 ms`
       - `kilo_kernel_small_hk: 745 ms`
     - current read:
       - this is not a large exact-front win
       - but it is a cleaner source-contract split and keeps whole-kilo near the good end of the current band
     - target assembly shape remains:
         - planner-proved `RetargetAlias` should become metadata-heavy code
         - generic object fetch/downcast should disappear from the hot retarget path except for true source-lifetime keep
  - current design freeze:
      - do not add a public MIR op for `RetargetAlias`
      - carry only:
        - `source_preserve`
        - `identity_demand`
  - latest keep stop-line slice:
    - `SourceLifetimeKeep` is now opaque on its surface; backing representation stays internal
    - target reading is now explicit:
      - `TextKeep`
      - `AliasSourceMeta`
      - cold copy-out to owned text
    - `ArrayStoreStrSource::StringLike(...)` now carries a typed `VerifiedTextSource`
    - `StringView` keep fallback now uses cold owned text copy-out instead of object-like fallback cloning through the keep surface
    - current accept gate:
      - `kilo_micro_array_string_store: 170 ms`
      - `kilo_micro_concat_hh_len: 61 ms`
      - `kilo_kernel_small_hk: 715 ms`
         - `publication_demand`
         above Rust
       - keep runtime narrowables as backend-private seams:
         - `StringLikeProof`
         - `TextKeep`
         - `AliasSourceMeta`
     - latest landed structural split:
       - `BorrowedHandleBox` now separates `TextKeep` from `AliasSourceMeta`
       - `SourceLifetimeKeep` remains the current keep carrier, still backed by `StableBox(...)`
       - this is a no-behavior split to narrow the next cut to keep semantics
       - accept-gate reread:
         - `kilo_micro_array_string_store: 175 ms`
         - `kilo_micro_concat_hh_len: 63 ms`
         - `kilo_kernel_small_hk: 703 ms`
     - closed follow-up:
       - replacing `with_handle(ArrayStoreStrSource)` with direct `get()` source load regressed slightly
       - 3-run plain release:
         - `kilo_micro_array_string_store: 192 ms`
         - `kilo_micro_concat_hh_len: 69 ms`
         - `kilo_kernel_small_hk: 747 ms`
       - revert the behavior change; keep `with_handle_caller(...)` for now
        - planner says this hot path is pure `RetargetAlias`
        - the expensive escalation therefore happens before action selection, not because planner asked for `NeedStableObject`
        - but current `retarget` still needs source-object keep:
          - `source_kind_via_object`
         - `retarget_keep_source_arc`
         - `retarget_alias_update`
       - no-behavior-change `source_kind_check` split is now landed:
         - `StringHandleSourceKind`
         - `classify_string_handle_source(...)`
         - `array_string_slot.rs` planning now reads that contract instead of open-coding string-like checks
       - next structural slice is therefore:
         - split `source_kind_check` from `keep_source_arc`
         - do not assume object entry can simply disappear
     - current first widening target is therefore:
       - `store.array.str` source read under `array_string_slot.rs`
     - attempted widening truth:
       - redirecting `store.array.str` source read into `TextReadSession` moved latest fresh demand out of `object_with_handle(...)`
       - but plain release regressed:
         - `kilo_micro_array_string_store: 181 -> 187 ms`
         - `kilo_kernel_small_hk: 757 -> 916 ms`
       - the behavior change is reverted; keep the caller attribution only
     - narrow `retarget` retry truth:
       - a no-op guard in `try_retarget_borrowed_string_slot_verified(...)` for unchanged `(source_handle, source_drop_epoch)` did not materially move the front
       - plain release recheck:
         - `kilo_micro_array_string_store: 183 ms`
         - `kilo_kernel_small_hk: 746 ms`
       - the behavior change is reverted; keep the counter truth only
     - latest-fresh stable object cache truth:
       - caching the newest `Arc<dyn NyashBox>` in TLS and short-circuiting `with_handle(ArrayStoreStrSource)` regressed exact and whole
       - plain release 3-run:
         - `kilo_micro_array_string_store: 210 ms`
         - `kilo_micro_concat_hh_len: 78 ms`
         - `kilo_kernel_small_hk: 760 ms`
       - the behavior change is reverted
     - borrowed alias raw string cache truth:
       - caching source string addr/len inside `BorrowedHandleBox` and bypassing `inner.as_str_fast()` regressed exact and whole
       - plain release 3-run:
         - `kilo_micro_array_string_store: 196 ms`
         - `kilo_micro_concat_hh_len: 69 ms`
         - `kilo_kernel_small_hk: 798 ms`
       - the behavior change is reverted
     - typed string payload truth:
       - issuing fresh string handles through a typed `StringBox` payload and using a typed retarget fast path regressed the exact fronts immediately
       - plain release 3-run:
         - `kilo_micro_array_string_store: 201 ms`
         - `kilo_micro_concat_hh_len: 72 ms`
       - whole-kilo was not pursued; the behavior change is reverted
     - cloned source-arc retarget truth:
       - hot `RetargetAlias` was retried with a narrow `clone source Arc first, then retarget` slice
       - plain release 3-run regressed across exact and whole:
         - `kilo_micro_array_string_store: 205 ms`
         - `kilo_micro_concat_hh_len: 91 ms`
         - `kilo_kernel_small_hk: 959 ms`
       - the behavior change is reverted
       - keep only the no-behavior structural split:
         - `StoreArrayStrPlan` separates planner from executor
         - borrowed retarget now exposes `keep_source_arc` / `alias_update` helpers
     - typed `ArrayStoreStrSource` helper is now landed:
       - `with_array_store_str_source(...)` wraps `with_handle_caller(ArrayStoreStrSource)`
       - `store.array.str` now consumes a typed source contract instead of open-coding generic object entry at the executor callsite
       - this remains the landed no-behavior seam
     - typed helper internal bypass truth:
       - trying `with_str_handle(...)` first inside `with_array_store_str_source(...)` regressed exact and whole
       - plain release 3-run:
         - `kilo_micro_array_string_store: 190 ms`
         - `kilo_micro_concat_hh_len: 67 ms`
         - `kilo_kernel_small_hk: 783 ms`
       - the behavior change is reverted
     - `keep_source_arc` ptr-eq truth:
       - observe direct probe now runs again via sync-stamp aligned perf-observe lane
       - exact `kilo_micro_array_string_store`:
         - `reason.retarget_keep_source_arc_ptr_eq_hit=0`
         - `reason.retarget_keep_source_arc_ptr_eq_miss=800000`
       - whole `kilo_kernel_small_hk`:
         - `reason.retarget_keep_source_arc_ptr_eq_hit=0`
         - `reason.retarget_keep_source_arc_ptr_eq_miss=540000`
     - `keep_source_arc` always sees a different source object on the current culprit path
     - clone-elision / ptr-eq guard ideas are closed
     - borrowed string keep seam is now landed:
       - `BorrowedHandleBox` keep-side contract is explicit as `BorrowedStringKeep`
       - current behavior still uses `StableBox(...)` only
       - this is still no-behavior-change
       - next structural cut can target source-lifetime keep without widening generic object payloads
     - closed follow-up:
       - a typed `BorrowedStringKeep::StringBox` fast path regressed on both exact and whole
       - 3-run plain release:
         - `kilo_micro_array_string_store: 198 ms`
         - `kilo_micro_concat_hh_len: 71 ms`
         - `kilo_kernel_small_hk: 777 ms`
       - behavior change is reverted
       - current read:
         - transport-only typed keep is not enough
         - source-lifetime keep semantics must move before keep representation changes again
     - `TextSnapshot` keep retry truth:
       - narrow retarget-only `TextSnapshot` keep improved exact fronts:
         - `kilo_micro_array_string_store: 178 ms`
         - `kilo_micro_concat_hh_len: 65 ms`
       - but whole-kilo collapsed:
         - `kilo_kernel_small_hk: 1792 ms`
       - the behavior change is reverted
       - current read:
         - snapshot keep can win on the exact retarget micro
         - but mixed generic consumers still force enough on-demand objectization to lose badly on whole-kilo
    - latest landed retarget success slice:
      - move source `Arc` into alias keep on successful `RetargetAlias`
      - this removes one extra clone from the hot retarget path without widening host-handle payloads
      - 3-run plain release:
        - `kilo_micro_array_string_store: 178 ms`
        - `kilo_micro_concat_hh_len: 65 ms`
        - `kilo_kernel_small_hk: 740 ms`
    - latest landed live-source alias slice:
      - gate `BorrowedHandleBox::as_str_fast()` live/stale epoch probe behind `observe::enabled()`
      - this keeps observe truth unchanged while removing the plain-release hot-path epoch read
      - 3-run plain release:
        - `kilo_micro_array_string_store: 169 ms`
        - `kilo_micro_concat_hh_len: 61 ms`
        - `kilo_kernel_small_hk: 717 ms`
    - latest landed source-lifetime keep split:
      - `ArrayStoreStrSource::StringLike(...)` now carries `SourceLifetimeKeep`
      - retarget success path now consumes `try_retarget_borrowed_string_slot_take_keep(...)`
      - this is still no-behavior-change and keeps `StableBox(...)` underneath; it only fixes the next cut onto keep semantics
      - 3-run plain release reread:
        - `kilo_micro_array_string_store: 169 ms`
        - `kilo_micro_concat_hh_len: 63 ms`
        - `kilo_kernel_small_hk: 703 ms`
    - latest landed string-like proof split:
      - `SourceKindCheck` now carries `StringLikeProof` separately from `SourceLifetimeKeep`
      - `ArrayStoreStrSource::StringLike(...)` now keeps both:
        - `proof: StringLikeProof`
        - `keep: SourceLifetimeKeep`
      - `execute_store_array_str_slot(...)` now records string-like source observe truth from the typed source contract instead of repeating local downcasts
      - this is still no-behavior-change; it narrows the next cut to keep semantics rather than source-kind transport
      - 3-run plain release reread:
        - `kilo_micro_array_string_store: 173 ms`
        - `kilo_micro_concat_hh_len: 68 ms`
        - `kilo_kernel_small_hk: 713 ms`
    - latest landed keep API narrowing:
      - `SourceLifetimeKeep` now exposes only text/lifetime-side API on the keep seam
      - full object API stays on `BorrowedHandleBox` through `stable_box_ref()` instead of the keep carrier
      - representation is still `StableBox(...)`; this is API narrowing only
      - 3-run plain release reread:
        - `kilo_micro_array_string_store: 173 ms`
        - `kilo_micro_concat_hh_len: 63 ms`
        - `kilo_kernel_small_hk: 708 ms`
    - closed proof-carrying keep direct path:
      - carrying `StringLikeProof` inside `TextKeep` and using proof-specific `as_str_fast()` regressed
      - 3-run plain release:
        - `kilo_micro_array_string_store: 178 ms`
        - `kilo_micro_concat_hh_len: 67 ms`
        - `kilo_kernel_small_hk: 730 ms`
      - current read:
        - keep proof on the source-contract side
        - do not widen alias keep semantics with proof transport again
    - read-contract freeze:
      - `BorrowedHandleBox::as_str_fast()` stays a stable-object read only
      - `host_handles::with_str_handle(...)` / `with_text_read_session(...)` stay live-source session reads only
      - do not push registry-backed direct read into `as_str_fast()`
    - latest landed read-contract naming cleanup:
      - `SourceLifetimeKeep` stable-object text read is now named as stable-object read rather than generic fast read
      - `ArrayStoreStrSource::object_ref()` is now `stable_object_fallback_ref()`
      - this is still no-behavior-change; it aligns backend naming with the read-contract split
      - accept-gate reread:
        - `kilo_micro_array_string_store: 173 ms`
        - `kilo_micro_concat_hh_len: 62 ms`
        - `kilo_kernel_small_hk: 698 ms`
    - latest landed typed store-from-source split:
      - `store.array.str` now sends the string-like store path through `SourceLifetimeKeep` directly
      - generic object fallback remains only for `OtherObject / Missing`
      - this is still no-behavior-change at the representation layer; it narrows the next actual cut away from object-centric store fallback
      - accept-gate reread:
        - `kilo_micro_array_string_store: 175 ms`
        - `kilo_micro_concat_hh_len: 65 ms`
        - `kilo_kernel_small_hk: 699 ms`
    - latest landed object-fallback API narrowing:
      - removed the unified `ArrayStoreStrSource` object-fallback accessor
      - `StringLike` and `OtherObject` no longer rejoin through one object-ref API
      - this is still no-behavior-change; it keeps the string-like branch and object fallback branch structurally separate
      - accept-gate reread:
        - `kilo_micro_array_string_store: 171 ms`
        - `kilo_micro_concat_hh_len: 64 ms`
        - `kilo_kernel_small_hk: 700 ms`
    - latest landed object-demand API narrowing:
      - raw `stable_box_ref()` access no longer crosses module boundaries from keep/alias internals into encode/store callers
      - encode-side object demand now goes through intent helpers:
        - `encode_fallback_box_ref()`
        - `clone_stable_box_for_encode_fallback()`
        - `ptr_eq_source_object()`
      - store-from-source keep demand now goes through:
        - `clone_stable_box_for_store_fallback()`
      - this is still no-behavior-change; it narrows object-demand API shape without changing keep representation
      - accept-gate reread:
        - `kilo_micro_array_string_store: 172 ms`
        - `kilo_micro_concat_hh_len: 68 ms`
        - `kilo_kernel_small_hk: 711 ms`
    - latest landed compatibility shim removal:
      - removed unused pre-keep string/source helpers:
        - `try_retarget_borrowed_string_slot_with_source(...)`
        - `try_retarget_borrowed_string_slot_verified(...)`
        - `keep_borrowed_string_slot_source_arc(...)`
        - `store_string_box_from_string_source(...)`
      - current structural path is now explicit:
        - retarget: `try_retarget_borrowed_string_slot_take_keep(...)`
        - store-from-source: `store_string_box_from_source_keep(...)`
      - this is still no-behavior-change; it removes compatibility entry points
        that were keeping the old object-centric shape visible
      - accept-gate reread:
        - `kilo_micro_array_string_store: 176 ms`
        - `kilo_micro_concat_hh_len: 63 ms`
        - `kilo_kernel_small_hk: 691 ms`
    - latest landed module string dispatch cleanup:
      - removed `plugin/module_string_dispatch.rs` direct `host_handles::get()/to_handle_arc()` bypass for compat string handle decode/encode
      - compat decode now goes through `value_codec::owned_string_from_handle(...)`
      - compat encode now goes through `materialize_owned_string(...)`
      - this keeps the compat path inside the `value_codec` seam and removes the review's last direct `host_handles` bypass from `module_string_dispatch.rs`
      - accept-gate reread:
        - `kilo_micro_array_string_store: 176 ms`
        - `kilo_micro_concat_hh_len: 67 ms`
        - `kilo_kernel_small_hk: 712 ms`
      - physical `string_store.rs` file split remains deferred until the keep semantics change lands
    - latest landed encode planner/executor split:
      - `runtime_i64_from_box_ref_caller(...)` no longer mixes borrowed-alias reuse planning and fallback handle issue in one block
      - planner now decides:
        - `ReuseSourceHandle`
        - `ReturnScalar`
        - `EncodeFallback`
      - executor now performs only the fallback publication mechanics
      - this is no-behavior-change structure cleanup for the review's `encode.rs` concern
      - accept-gate reread:
        - `kilo_micro_array_string_store: 179 ms`
        - `kilo_kernel_small_hk: 739 ms`
      - one `1014 ms` whole-kilo outlier was discarded after the immediate reread returned to the current WSL band
    - latest landed `SourceLifetimeKeep` subtype semantics:
      - keep contract now distinguishes verified string-like source subtype:
        - `StringBox`
        - `StringView`
      - borrowed alias creation now consumes keep semantics directly:
        - `maybe_borrow_string_keep_with_epoch(...)`
      - `store.array.str` string-like path now constructs keep from the verified subtype instead of treating keep as a generic stable object
      - representation is still `Arc<dyn NyashBox>` underneath; this is a keep-semantics cut, not a transport widening
      - accept-gate reread:
        - `kilo_micro_array_string_store: 175 ms`
        - `kilo_kernel_small_hk: 699 ms`
    - latest landed `string_classify.rs` split:
      - moved `SourceKindCheck` and typed `ArrayStoreStrSource` construction out of `string_store.rs` into `value_codec/string_classify.rs`
      - `string_store.rs` now keeps materialize/objectize/publication and store fallback execution only
      - this is a no-behavior physical split after the module-string layer bypass removal
      - accept-gate reread:
        - `kilo_micro_array_string_store: 174 ms`
        - `kilo_kernel_small_hk: 715 ms`
      - one `1894 ms` whole-kilo outlier was discarded after the immediate reread returned to the current band
    - latest landed array string-store perf seam split:
      - commit: `93e390455 refactor: split array string-store perf seams`
      - runtime-private measurement seams are now explicit:
        - `capture_store_array_str_source(...)`
        - `store_array_str_value_from_source(...)`
        - `execute_store_array_str_slot_boundary(...)`
        - `store_string_box_from_verified_text_source(...)`
      - verification:
        - `cargo check --features perf-observe -p nyash_kernel` PASS
        - `cargo test -p nyash_kernel set_his_alias_sets_string_handle_value -- --test-threads=1` PASS
      - observe reread on exact `kilo_micro_array_string_store`:
        - `freeze_owned_bytes: 15.76%`
        - `issue_fresh_handle: 14.54%`
        - `StringBox::perf_observe_from_owned: 11.70%`
        - `capture_store_array_str_source: 8.53%`
        - `string_concat_hh_export_impl: 7.23%`
        - `string_len_export_slow_path: 6.74%`
        - `LocalKey::with: 5.72%`
        - `__memmove_avx512_unaligned_erms: 4.63%`
        - `nyash.string.concat_hs: 4.49%`
        - `execute_store_array_str_contract: 4.44%`
        - `execute_store_array_str_slot_boundary: 4.30%`
        - `string_substring_concat_hhii_export_impl: 3.28%`
      - reading:
        - dominant cost is still upstream birth/publication plus source capture
        - slot mutation itself is not the first owner once the source has already been published
    - latest rejected non-direct-set `const_suffix` widening:
      - changed `classify_string_concat_pair_route(...)` so a const-suffix pair could select `const_suffix` without `direct_set` when it was not a concat3 chain
      - exact reread after rebuild:
        - `kilo_micro_array_string_store: 174 ms`
        - `kilo_micro_concat_const_suffix: 3 ms`
        - `kilo_kernel_small_hk: 708 ms`
      - trusted direct MIR for `bench_kilo_micro_array_string_store.hako` still shows duplicated producer birth:
        - one `text + "xy"` result feeds `set(...)`
        - a second `text + "xy"` result feeds trailing `substring(...)`
      - reading:
        - widening `const_suffix` alone does not delete the current exact gap
        - next cut moves to compiler-local placement proof around the duplicated producer window, not further runtime or route widening
    - next observation order is fixed:
     1. keep the landed `store.array.str` perf seam split and use it only for measurement
     2. prove or reject the duplicated `const_suffix -> store + substring` placement window on the trusted direct MIR
     3. keep borrowed alias string-read trimming closed; live-source fast read was not enough
     4. keep typed `StringBox` payload widening closed at the host-handle layer
     5. keep `keep_source_arc` clone-elision ideas closed; ptr-eq never hits on the current culprit
     6. keep typed `BorrowedStringKeep::StringBox` fast path closed; transport-only specialization still loses
     7. do not add more typed-helper transport; move the next cut to compiler-local placement/source-lifetime facts before representation changes again
     8. only then retry delayed `StableBoxNow`
   - `DeferredString` experiment truth:
     - exact micro improved:
       - `kilo_micro_concat_hh_len`: `57 -> 51 ms`
       - `kilo_micro_concat_birth`: `47 -> 35 ms`
     - whole-kilo probe regressed:
       - `kilo_kernel_small_hk`: `741 -> 952 ms`
     - code was reverted
     - next widening choice is now:
       1. explain the whole-kilo regression first
       2. only then reconsider pair/span widening
   - `host_handles` now has a source-backed payload seam:
     - slot storage reads through `HandlePayload::StableBox(...)`
     - public registry APIs still return `Arc<dyn NyashBox>`
     - this does not change behavior yet; it only narrows the future widening point for `DeferredStableBox`
     - single-handle string-only access is also separated now:
       - `host_handles::with_str_handle(...)`
       - `string_len_from_handle(...)` and `string_is_empty_from_handle(...)` consume that seam
   - current exact backend front is therefore:
     - `FreshHandle`
     - `MaterializeOwned`
   - target string-chain assembly shape:
     - `concat_hh + len_h` should spend most cycles in text/materialize work, not registry/object machinery
     - `StableBoxNow` and `FreshRegistryHandle` should move to sink/object boundaries only
   - current birth backend split now reads:
     - `StringBox` ctor side before registry issue
     - direct probe now also shows:
       - `string_box_ctor_total=800000`
       - `string_box_ctor_bytes=14400000`
       - `arc_wrap_total=800000`
     - observe-build `kilo_micro_concat_birth` microasm top:
       - `birth_string_box_from_owned`: `38.23%` to `41.46%`
       - `issue_string_handle_from_arc`: `27.66%` to `31.54%`
       - `__memmove_avx512_unaligned_erms`: `9.10%` to `10.88%`
       - `string_concat_hh_export_impl`: `11.53%` to `12.73%`
   - release observe direct probe now confirms second-axis counters too:
     - `objectize_stable_box_now_total=800000`
     - `objectize_stable_box_now_bytes=14400000`
     - `issue_fresh_handle_total=800000`
   - `kilo_micro_concat_birth` observe-build microasm after backend split now reads:
     - `materialize_owned_bytes`: `25.81%`
     - `issue_fresh_handle`: `24.62%`
     - `StringBox::perf_observe_from_owned`: `21.27%`
     - `__memmove_avx512_unaligned_erms`: `14.67%`
     - `nyash.string.concat_hh`: `5.81%`
   - annotate for `issue_fresh_handle(...)` shows the dominant local leaf is the final registry unlock/release path
   - next backend front is therefore:
     1. `materialize_owned_bytes`
     2. `issue_fresh_handle`
     3. `StringBox::perf_observe_from_owned`
   - do not spend more time on concat/len route guessing for these exact fronts unless a future counter contradicts the current read
   - `objectize_stable_string_box` stays as the seam name, but most runtime cost is currently absorbed by ctor/issue leaves
   - backend second-axis lock:
     - top-level Birth / Placement vocabulary stays unchanged
     - `box_id` is not promoted into that vocabulary
     - backend-only reading is now:
       - `Objectization = None | StableBoxNow | DeferredStableBox`
       - `RegistryIssue = None | ReuseSourceHandle | FreshRegistryHandle`
     - current `concat_birth` path still couples:
       - `FreshHandle`
       - `MaterializeOwned`
       - `StableBoxNow`
       - `FreshRegistryHandle`
     - current source-backed backend split is now visible in `string_store.rs`:
       - `materialize_owned_bytes`
       - `objectize_stable_string_box`
       - `issue_fresh_handle`
     - second-axis counters now also exist for:
       - `objectize_stable_box_now_total / bytes`
       - `issue_fresh_handle_total`
     - observe lane contract is now fail-fast:
       - default perf AOT lane aborts unless `target/release/.perf_release_sync` is newer than both `target/release/libnyash_kernel.a` and `target/release/hakorune`
       - `NYASH_PERF_COUNTERS=1` / `NYASH_PERF_TRACE=1` still require `target/release/.perf_observe_release_sync`
       - canonical rebuild orders are fixed in `tools/perf/build_perf_release.sh` and `tools/perf/build_perf_observe_release.sh`
       - helper-local ranking rule:
         - plain release asm = real cost ranking
         - observe build = counts and symbol split
         - `materialize_owned_bytes(...)` observe annotate is currently dominated by TLS counter work, so it is not sufficient by itself for first-front ordering
  - current microasm read:
     - `string_concat_hh_export_impl`: `54.04%`
     - `string_len_from_handle`: `21.37%`
     - `__memmove_avx512_unaligned_erms`: `15.40%`

## Next

1. keep the restart handoff above as the active truth
   - older exact-front notes in this file are historical unless the restart handoff names them as guards
   - `kilo_micro_substring_only` is no longer the current exact owner front for new implementation edits
2. open implementation gates before the next kilo optimization
   - current blocker: `137x-H owner-first optimization return`
   - `137x-E0`: MIR / backend seam closeout is closed
   - `137x-E1`: minimal `TextLane` / `ArrayStorage::Text` is landed
   - `137x-F`: runtime-wide Value Lane implementation bridge is closed
   - `137x-G`: allocator / arena pilot is rejected / not opened by F closeout
   - `137x-H`: owner-first optimization return
3. keep landed `137x-D` cuts fixed
   - same-slot piecewise subrange store originally lowered through `nyash.array.string_insert_mid_subrange_store_hisiii`
   - current direct lowering uses `nyash.array.string_insert_mid_subrange_store_hisiiii`
   - same-slot compiler-emitted string literals now pass explicit byte length through runtime-private helpers:
     - `nyash.array.string_suffix_store_hisi`
     - `nyash.array.string_insert_mid_store_hisii`
     - `nyash.array.string_insert_mid_subrange_store_hisiiii`
   - C-shim const-hoist use counting now includes `phi.incoming`; this prevents skipped string constants from turning into undefined `%rN` values in generic pure lowering
   - fresh evidence:
     - `kilo_micro_array_string_store = C 10 ms / Ny AOT 10 ms`, `ny_aot_instr=26922384`
     - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 9 ms`, `ny_aot_instr=129614388`
     - `kilo_kernel_small_hk = C 84 ms / Ny AOT 26 ms`, parity ok
     - `ny_main` hot path calls `nyash.array.string_insert_mid_store_hisii` and `nyash.array.string_suffix_store_hisi`
     - allocator/copy is secondary in the current hot reports: middle `cfree` is 9.45%, whole `__memmove_avx512_unaligned_erms` is 5.39%
   - exact route-shape keeper:
     - previous watch: `kilo_micro_array_string_store = C 10 ms / Ny AOT 144 ms`
     - current proof: `kilo_micro_array_string_store = C 11 ms / Ny AOT 10 ms`, `ny_aot_instr=26922130`
     - route proof: `array_string_store_micro result=emit reason=exact_match`
     - asm proof: `ny_main` is the stack-array seed IR and runtime/public helper calls are absent from the loop body
4. closed exact optimization card
   - closed card: `137x-D exact array store route-shape proof`
   - cause: the exact seed matcher accepted only the older 9-block MIR shape; current direct MIR emits the compact 8-block shape
   - implementation: MIR now owns the compact 8-block direct shape as `metadata.array_string_store_micro_seed_route`; `hako_llvmc_match_array_string_store_micro_seed(...)` only reads that metadata and still emits the existing specialized stack-array IR
   - smoke: `phase137x_direct_emit_array_store_string_contract.sh` now requires exact seed emitter selection and no runtime/public helper calls in `ny_main`
   - guards:
     - middle `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 9 ms`, `ny_aot_instr=127269397`
     - strict whole `kilo_kernel_small_hk = C 83 ms / Ny AOT 28 ms`, parity ok
   - old blocker rule is retired: `TextLane` and runtime-wide Value Lane closed through `137x-E/F`; allocator/arena is rejected for now by `137x-F` closeout
   - still blocked here: typed map, `publish.any`, heterogeneous / union array slot layout, and public ABI widening
