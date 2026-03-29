# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-03-29
Scope: repo root の再起動入口。詳細の status / phase 進捗は `docs/development/current/main/` を正本とする。

## Purpose

- root から最短で current blocker / active lane / next fixed order に到達する。
- 本ファイルは薄い入口に保ち、長い phase 履歴や retired lane detail は phase README / design SSOT へ逃がす。

## Quick Restart Pointer

- `docs/development/current/main/05-Restart-Quick-Resume.md`
- `docs/development/current/main/15-Workstream-Map.md`
- `git status -sb`
- `tools/checks/dev_gate.sh quick`

## Current Lanes

### phase-29bq

- status: `active (failure-driven; blocker=none)`
- scope: selfhost `.hako` migration (`mirbuilder first / parser later`)
- current SSOT:
  - `docs/development/current/main/phases/phase-29bq/README.md`
  - `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
  - `docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md`
  - `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`
- next exact leaf: `none` until the next blocker is captured

### phase-29x

- status: `active compare bridge retirement / archive decisions`
- scope: shrink the remaining compare bridge / archive wrapper surfaces
- current truth:
  - `archive-home is sufficient`
  - `delete-ready is none`
  - Hako front-door `env.codegen.compile_json_path` retirement is landed
  - launcher root-first transport cut is landed
  - builder-side `compile_json_path` recognition is retired
  - Rust runtime dispatcher `compile_json_path` branches are retired
  - route-env helper `lang/src/shared/backend/backend_route_env_box.hako` is retired from code
  - remaining live set is compare bridge / archive wrapper surfaces
  - dead wrapper `lang/src/shared/host_bridge/codegen_bridge_box.hako::compile_json_path_args` is retired in this slice
- fixed order:
  1. keep `.ll` as the Rust/LLVM tool seam
  2. thin compare bridge wrapper surfaces caller-by-caller
  3. review archive/delete only after the wrapper inventory reaches zero
- current prep SSOT:
  - `docs/development/current/main/design/backend-owner-cutover-ssot.md`
  - `docs/development/current/main/design/runtime-decl-manifest-v0.toml`
  - `docs/development/current/main/phases/phase-29x/29x-96-backend-owner-legacy-ledger-ssot.md`
  - `docs/development/current/main/phases/phase-29x/29x-97-compare-bridge-retirement-prep-ssot.md`
- next exact leaf:
  - keep archive-later compare wrapper inventory closed and do not reopen daily ownership
  - treat delete/archive review as blocked until the remaining wrapper inventory actually reaches zero

### stage2-hako-owner

- status: `active docs-first owner/shim split`
- scope: stage2 を mostly `.hako` owner に寄せ、`.inc` を thin shim に薄化する。native metal keep は残す。
- current SSOT:
  - `docs/development/current/main/design/stage2-hako-owner-vs-inc-thin-shim-ssot.md`
  - `docs/development/current/main/design/stage2-selfhost-and-hako-alloc-ssot.md`
  - `docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md`
  - `docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md`
  - `docs/development/current/main/design/hako-runtime-c-abi-cutover-order-ssot.md`
- fixed order:
  1. docs-first owner/shim SSOT
  2. classify `.inc` partitions into semantic owner / compiler owner / thin shim / native leaf
  3. define compiler-state capability and lowering builder seam
  4. first code slice: extract emit primitives into `hako_llvmc_ffi_emit_seam.inc`
  5. second code slice: split generic method classification into `hako_llvmc_ffi_generic_method_match.inc`
  6. move semantic owner decisions to `.hako`
  7. thin shim cleanup and README sync
- next exact leaf:
  - do not touch runtime code until the owner/shim split is pinned in docs
  - keep native metal leafs resident; this lane is about authority migration, not full source-zero

### phase-29ck

- status: `monitor/evidence only`
- current details stay in phase29ck docs

### perf-kilo

- status: `active micro/kilo optimization`
- scope: string materialization / array store memory motion
  - current SSOT:
    - `docs/development/current/main/10-Now.md`
    - `docs/development/current/main/design/kilo-meso-benchmark-ladder-ssot.md`
    - `docs/development/current/main/design/recipe-scope-effect-policy-ssot.md`
    - `docs/development/current/main/design/retained-boundary-and-birth-placement-ssot.md`
    - `docs/development/current/main/design/post-store-observer-facts-ssot.md`
    - `docs/development/current/main/design/concat3-array-store-placement-window-ssot.md`
    - `docs/development/current/main/design/string-birth-placement-ssot.md`
    - `docs/development/current/main/design/string-birth-sink-ssot.md`
    - `docs/development/current/main/design/transient-text-pieces-ssot.md`
  - `docs/tools/README.md`
  - current leaf status:
  - normalized transient text pieces (`TextPlan` / `PiecesN`) pilot landed
  - `micro -> meso -> kilo` observation ladder landed
  - compile-time placement helper `string_birth_placement.rs` landed
  - string export surface is now split by responsibility: `string.rs` (entrypoints/sink), `string_debug.rs`, `string_search.rs`, `string_plan.rs`, and `string_view.rs`
  - current sub-slice:
    - meso first reading is fixed: `len = 37 ms`, `array_set = 69 ms`, `loopcarry = 69 ms` (`warmup=1 repeat=3`)
    - the first large jump is `len -> array_set`, not `array_set -> loopcarry`
    - landed narrow store-boundary cut: `array_set_by_index_string_handle_value` now resolves the source handle in-place inside the write closure instead of cloning a temporary `Arc` before the hot path
    - latest store-boundary recheck: `kilo_meso_substring_concat_array_set = 66 ms`, `kilo_kernel_small_hk = 708 ms` (`warmup=1 repeat=3`, `aot_status=ok`)
  - landed concat3 reuse-only specialization: `concat3_plan_from_spans(...)` is now fixed to the reuse-allowed lane, so the dead `allow_handle_reuse = false` branch is gone and span emptiness checks use byte-range length directly
    - latest same-artifact proof after this specialization is `kilo_meso_substring_concat_len = 34 ms`, `kilo_meso_substring_concat_array_set = 66 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 65 ms`, `kilo_kernel_small_hk = 668 ms` (`warmup=1 repeat=3`, `aot_status=ok`)
  - rejected small carrier cleanup retry:
    - sending owned fast paths directly through `string_handle_from_owned(...)`, removing the `resolve_string_span_from_handle(...)` fallback after `TextPlan::from_handle(...)`, and using the relative range length directly inside `borrowed_substring_plan_from_handle(...)` regressed stable main to `777 ms`; keep the span-backed / helper-backed current lane for now
  - rejected pair span-length retry:
    - changing `concat_pair_from_spans(...)` to use span byte lengths instead of `as_str().is_empty()` regressed stable main to `904 ms`; keep the existing span-read check there for now
    - latest same-artifact proof after the retained-boundary parent split stayed flat: `kilo_meso_substring_concat_len = 35 ms`, `kilo_meso_substring_concat_array_set = 68 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 69 ms`, `kilo_kernel_small_hk = 760 ms` (`warmup=1 repeat=3`, `aot_status=ok`)
  - rejected StringViewBox stable-id retry:
    - replacing `StringViewBox::new(...)`'s `BoxBase::new()` with a derived stable id to dodge the atomic allocator regressed stable main to `814 ms` under `repeat=3`; keep the current atomic view birth until fresh evidence says otherwise
  - rejected StringViewBox borrow/retarget expansion:
    - extending `maybe_borrow_string_handle_with_epoch(...)` / `try_retarget_borrowed_string_slot_with_source(...)` to accept `StringViewBox` as a string source regressed stable main to `844 ms` under `repeat=3`; keep the current StringBox-only borrow/retarget lane for now
  - rejected direct array-slot insert helper:
    - wiring `nyash.array.string_insert_hisi` from `string_insert_mid_window` when both substrings traced back to the same `array.get` source regressed stable main to `1020 ms` on `repeat=3`; the `repeat=20` recheck still stayed above the kept `668 ms` line at `716 ms`
    - the quick ASM probe still centered on `string_handle_from_owned`, `concat3_hhh`, `substring_hii`, `array.set_his`, `string_len_from_handle`, and `BoxBase::new`, so this helper did not displace the real birth-density residue
  - shared store-ready string materialization boundary
  - string-specific store helper for array/string hot paths
  - single handle/span resolution in `concat_const_suffix_fallback`
  - follow-up design front: `freeze.str` as the single birth sink for `concat_hs` / `insert_hsi` / `concat3_hhh`
  - retained-boundary parent split is now docs-first: `BoundaryKind` owns retained reason and `RetainedForm` owns retained result
  - attempted canonical sink re-home: moving `freeze.str` into `string_store.rs` regressed stable main (`kilo_kernel_small_hk = 834 -> 909 ms` on back-to-back checks), so keep the explicit `freeze_text_plan(...)` sink helper in `string.rs` for now
  - landed planner cleanup: const-suffix / insert recipe helpers now live in `crates/nyash_kernel/src/exports/string_plan.rs`, leaving `string.rs` as the boundary/sink site
  - latest kept recheck after branch-check trim: `kilo_meso_substring_concat_array_set = 68 ms`, `kilo_kernel_small_hk = 707 ms` (`warmup=1 repeat=3`, `aot_status=ok`)
  - accepted concat3 lock-safe fast path:
    - `concat3_plan_from_fast_str(...)` and `concat_pair_from_fast_str(...)` no longer freeze while holding the host-handle read lock; they now return a reuse-or-owned decision first and freeze outside the lock
    - `resolve_string_span_triplet_from_handles(...)` plus `string_span_cache_get_triplet(...)` now land the triple-span route
    - latest recheck after this concat3 fix is `kilo_meso_substring_concat_len = 36 ms`, `kilo_meso_substring_concat_array_set = 67 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 67 ms`, `kilo_kernel_small_hk = 704 ms` (`warmup=1 repeat=3`, `aot_status=ok`)
  - compiler-local placement trace is now visible in the direct compiler bundle after rebuilding `libhako_llvmc_ffi.so`
    - `string_direct_array_set_consumer` now carries `producer_kind=Concat3` / `boundary_kind=Store` / `post_store_use=None` / `known_len=-1` when the concat3 chain reaches the direct array-set consumer
    - `array_string_len_window` now carries `producer_kind=ArrayGet` / `boundary_kind=Store` / `post_store_use=LenObserver` / `known_len=-1`
    - timing-only recheck stayed in the same kept lane on this machine: `kilo_kernel_small_hk = 725 ms` (`warmup=1 repeat=3`) and `741 ms` (`warmup=1 repeat=20`), with `aot_status=ok`
  - rejected follow-up: concat3 reuse-only alias to earlier insert birth regressed stable main to `754-755 ms` under `repeat=3/20`; keep the current canonical birth split as-is until a fresh placement reason appears
  - rejected follow-up: canonical `concat3_hhh` birth with later reuse alias regressed stable main to `723 ms` on `repeat=3` and `777 ms` on `repeat=20`; keep the current upstream placement lane open instead of forcing another birth-site alias
  - rejected follow-up: rewriting the insert-mid route to emit `concat3_hhh` directly still regressed main to `775 ms` and tripped `build_failed_after_helper_retry` on the ladder lane; keep the current helper-backed insert route for now and do not treat the concat3 rewrite as the canonical birth
  - accepted short-slice substring freeze cut:
    - `BorrowedSubstringPlan` now returns `FreezeSpan(StringSpan)` for short freeze-only slices instead of wrapping them in `TextPlan::from_span(...)`
    - `substring_hii` materializes those short spans directly via `string_handle_from_span(...)`, keeping the current `<= 8 bytes` policy but removing one `TextPlan` / `into_owned()` hop
    - latest same-artifact recheck after this cut is `kilo_meso_substring_concat_len = 35 ms`, `kilo_meso_substring_concat_array_set = 67 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 69 ms`, `kilo_kernel_small_hk = 704 ms` (`warmup=1 repeat=3`, `aot_status=ok`)
  - accepted array string-length observer cut:
    - `array_string_len_by_index(...)` now uses `handle_cache::with_array_box(...)` instead of `host_handles::with_handle(...) + ArrayBox` downcast, so `nyash.array.string_len_hi` stays on the typed handle-cache path
    - latest `repeat=3` proof is `kilo_meso_substring_concat_len = 35 ms`, `kilo_meso_substring_concat_array_set = 68 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 69 ms`, `kilo_kernel_small_hk = 721 ms` (`warmup=1 repeat=3`, `aot_status=ok`)
    - latest `repeat=20` WSL recheck is `kilo_meso_substring_concat_len = 36 ms`, `kilo_meso_substring_concat_array_set = 67 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 68 ms`, `kilo_kernel_small_hk = 688 ms` (`warmup=1 repeat=20`, `aot_status=ok`)
    - latest microasm still keeps `nyash.array.string_len_hi` in the hot tier (`6.34%`), so the observer route remains a real target but this typed-cache cut is generic and keepable
  - rejected length-aware store-boundary classifier retry:
    - changing `has_direct_array_set_consumer(...)` to classify `array.set` plus trailing `length()` as a combined store boundary regressed stable main to `746 ms` on `repeat=3` and `757 ms` on `repeat=20`; keep the direct-set-only guard for this wave
  - rejected known-len propagation retry:
    - threading `known_len` / post-store facts from `concat_hs` / `array.set` into `length()` lowering kept the lane flat-to-worse (`kilo_meso_substring_concat_len = 38 ms`, `kilo_meso_substring_concat_array_set = 66 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 70 ms`, `kilo_kernel_small_hk = 705 ms` on `repeat=3`; `692 ms` on `repeat=20`)
    - keep `array_set` as the first Store boundary and keep trailing `length()` as a separate post-store observer fact
  - rejected short-slice owned materialize retry:
    - changing the short freeze lane to `FreezeOwned(String)` and materializing inside `borrowed_substring_plan_from_handle(...)` regressed stable main to `866 ms`; keep the span-backed short freeze contract for now
  - same-artifact proof after the retained-boundary parent split stayed flat, so code-side `RetainedForm` split remains deferred unless fresh asm evidence appears
  - next fixed order is now:
    1. keep `BoundaryKind` and `RetainedForm` split as the parent retained-boundary contract
    2. keep `array_set` as the first `Store` proof boundary and avoid new `set_his` splits
    3. same-artifact meso/main proof stayed flat, so keep code-side retained-form split deferred unless fresh asm evidence appears
    4. keep `BoxBase::new` and further sink-local tuning out unless fresh asm evidence changes
  - landed sink-local read-side cut: `Registry::get` now uses a direct clone path without the extra clone helper
  - current optimization summary lives in `docs/development/current/main/investigations/perf-kilo-string-birth-hotpath-summary-2026-03-28.md`
  - sink-local lane is now exhausted; no further safe code cut is known without fresh upstream birth-density evidence
  - compile-time placement helper landed, so the next exact lane is upstream birth-density proof rather than more sink-local cuts
  - latest asm read keeps `Registry::alloc`, `Registry::get`, `BoxBase::new`, `substring_hii`, and `array_set_his` in the hot tier; that still confirms the next cut is upstream placement proof, not more sink-local tuning
  - rejected follow-up:
    - ArrayBox / RuntimeDataBox string-pointer store boundary route (`nyash.array.set_his_p`) was wired through the LLVM-Py lowering path and unit-tested successfully, but the 2026-03-29 meso/main recheck stayed flat-to-worse (`kilo_meso_substring_concat_array_set = 69 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 73 ms`, `kilo_kernel_small_hk = 720 ms`, `aot_status=ok`) and the ASM probe still did not surface `nyash.array.set_his_p`; keep the route as a capability, but do not count it as an active perf lane yet
    - widening the C-side direct-store consumer test to tolerate one trailing `length()` observer after `array.set` kept the same-artifact lane flat-to-worse (`kilo_meso_substring_concat_len = 36 ms`, `kilo_meso_substring_concat_array_set = 70 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 70 ms`, `kilo_kernel_small_hk = 706 ms` under `repeat=3`); keep the stricter store-only consumer guard
    - direct-set-preferring `concat3_hhh` route ordering in `string_concat_add_route` looked promising in trace, but the timing-only 3-run regressed to `kilo_kernel_small_hk = 745 ms` (`c_ms = 74`, `aot_status=ok`); keep the existing fallback order and do not treat this route-order tweak as a win
    - compiler-side `string.length()` arithmetic lowering for the insert-shaped concat recipe improved meso (`33 / 63 / 65`) but still regressed stable main to `695 ms` versus the kept `668 ms` concat3 reuse-only line under the same artifact pair; keep runtime `nyash.string.len_h` on that observer for now
    - combining the two cuts (`array.set + trailing length()` consumer widening plus compiler-side insert-recipe length arithmetic) still regressed stable main to `732 ms` (`34 / 66 / 69` under `repeat=3`), so keep both lanes rejected and do not reopen them separately without fresh placement evidence
    - `borrowed_substring_plan_from_handle(...)` cache-first retry dropped meso to `33 / 64 / 67` and lowered `substring_hii` hot share, but stable main still sat at `706 ms`; keep the current direct `handles::with_handle(...)` planner until a future placement wave can remove more than one substring birth at a time
    - `string_len_from_handle(...)` / `string_is_empty_from_handle(...)` observer cache-first retry flipped the helper order to consult `string_len_impl(...)` / `string_is_empty_impl(...)` before the direct fast-string path, but the same-artifact recheck stayed at `35 / 68 / 71` and regressed stable main to `764 ms`; keep the current fast-str-first observer order until a future placement wave removes more than one retained observer together
    - widening `handle_cache` to keep the latest+previous handles and routing `array_set_by_index_string_handle_value(...)` through a detached array cache path lowered meso to `35 / 65 / 69`, but stable main still stayed at `701 ms`; keep the current one-slot cache and direct `with_array_box(...) + handles::with_handle(...)` store path for now
    - compiler-local `has_direct_array_set_consumer(...)` first-use relaxation (keeping `array.set` as the first consumer even when `out.length()` remains afterward) still only reached `35 / 67 / 67` on meso and `698 / 697 ms` on back-to-back main checks; keep the stricter single-use predicate for now
    - the `insert_hsi` one-resolve helper looked good on the first `repeat=3` probe (`kilo_kernel_small_hk = 694 ms`) but regressed back to `727 ms` under `repeat=20`; keep the current helper-backed route on WSL
    - seeding `string_span_cache` at `materialize_owned_string(...)` birth looked good on the first `repeat=3` probe (`35 / 69 / 71`, `kilo_kernel_small_hk = 692 ms`) but drifted back to `36 / 70 / 69`, `kilo_kernel_small_hk = 730 ms` under `repeat=20`; keep span-cache admission on resolve-side only for now
    - direct `concat_hs` / `concat3` copy materialization regressed stable `kilo_kernel_small_hk` (`736 -> 757 ms`) and did not improve micro; keep `TextPlan`-backed concat routes until new asm evidence appears
    - piece-preserving `insert_inline` plus store/freeze restructuring regressed stable `kilo_kernel_small_hk` to `895 ms`; do not reopen that cut without a fresh `concat_hs` / `array_set_by_index_string_handle_value` reason
    - blanket `#[inline(always)]` on host registry / hako forward string wrappers held stable main around `740 ms` and did not beat the current `736 ms` line; keep that slice reverted
    - `concat_hs` duplicate span-resolution removal plus span-resolver inlining regressed stable `kilo_kernel_small_hk` to `796 ms`; keep the existing `TextPlan::from_handle(...)` route until a new asm reason appears
    - specialized `StringBox`-only store leaf under `nyash.array.set_his` regressed the kept store-boundary line (`kilo_meso_substring_concat_array_set = 66 -> 69 ms`, `kilo_kernel_small_hk = 708 -> 791 ms`); keep the generic string-source helpers and the in-place source borrow cut only
    - borrowed triple-span miss resolution via `handles::with3(...)` plus local `StringViewBox` flattening kept meso flat (`67 -> 68 ms`) and regressed stable main (`704 -> 745 -> 819 ms` on back-to-back checks); keep the explicit uncached miss wave in `resolve_string_span_triplet_from_handles(...)`
  - notes:
    - generic optimization unit is `recipe family`, not benchmark name
    - keep the generalized scope/method machinery
    - keep docs-first alignment between the transient carrier and the existing string docs
    - the current pilot uses normalized `PiecesN` only for the targeted concat/insert path; keep the carrier backend-local and non-observable
    - avoid reopening route / fallback policy until the memory-motion slice is exhausted
    - compiler-local placement trace is available under `NYASH_LLVM_ROUTE_TRACE=1`; use the narrow stages `string_direct_array_set_consumer`, `string_insert_mid_window`, and `string_concat_add_route` when deciding the next placement cut
    - Rust-side string trace is now split into `placement`, `carrier`, `sink`, and `observer` lines under the same route-trace gate; use it to see `BoundaryKind` / `RetainedForm`, borrowed substring lineage, freeze/birth sinks, and post-store observer resolution without reopening leaf hacks
    - trace gate split: tests consume `NYASH_LLVM_ROUTE_TRACE`, runtime consumes `NYASH_VM_ROUTE_TRACE`; the bench compare harness still suppresses both stdout and stderr, so the visible probe evidence comes from the unit contracts below
    - canonical probe entrypoint: `tools/perf/run_kilo_string_trace_probe.sh`
      - it collects the unit trace contracts into one summary without touching timing lanes
      - bench compare stays timing-only; do not make it carry trace output
    - trace+asm bundle entrypoint: `tools/perf/run_kilo_string_trace_asm_bundle.sh`
      - it keeps trace and asm in the same out-dir while leaving bench compare timing-only
      - the bundle resolves symbol names from `perf report` before annotate, so we no longer carry stale Rust-path guesses into the asm note files
      - current bundle hot symbols are `nyash.string.concat_hh`, `nyash.string.concat3_hhh`, `nyash.string.substring_hii`, `nyash.array.set_his`, `nyash.array.string_len_hi`, `nyash_kernel::exports::string::string_handle_from_owned`, and `nyash_rust::box_trait::BoxBase::new`
    - trace probe results were frozen via `NYASH_LLVM_ROUTE_TRACE=1 cargo test -p nyash_kernel -- --nocapture` (bench compare still suppresses trace output):
      - `string_concat_hs_contract`: `placement=keep_transient -> sink=freeze_plan -> sink=fresh_handle -> placement=return_handle`
      - `string_insert_hsi_contract`: `observer=fast_hit -> placement=keep_transient -> sink=freeze_plan -> sink=fresh_handle -> observer=fast_hit -> placement=return_handle`
      - `substring_hii_short_slice_materializes_under_fast_contract`: `placement=must_freeze -> carrier=freeze_span -> sink=fresh_handle -> sink=span_materialize -> observer=fast_hit`
      - this is probe-only evidence; it does not change the acceptance lane
    - judgment policy: `repeat < 3` is probe-only; keep/reject decisions require at least 3 runs plus a quick ASM probe; if WSL jitter or allocator-like noise remains, recheck with `repeat=20` before closing the lane

## Immediate Next Task

- active lane is now the upstream placement proof, not another leaf tweak:
  1. keep `retained-boundary-and-birth-placement-ssot.md` as the parent contract
  2. keep `array_set` as the consumer boundary / first `Store` proof while `post-store-observer-facts-ssot.md` owns the trailing `length()` observer
  3. use `concat3-array-store-placement-window-ssot.md` as the next exact rollout contract for `concat3_hhh -> array.set -> trailing length()`
  4. gather compiler-local facts from `remember_string_concat_*`, `remember_string_substring_call(...)`, `remember_string_length_call(...)`, `has_direct_array_set_consumer(...)`, and `analyze_array_string_len_window_candidate(...)`
  5. only after a same-artifact improvement is visible, revisit code-side `RetainedForm` wiring
- keep rejected `concat_hs` / `insert_inline` perf cuts documented and out of the active lane
- keep the landed meso benchmark ladder as the gate for the next string cut
- rejected follow-up:
  - canonicalizing `freeze.str` in `string_store.rs` regressed `kilo_kernel_small_hk` to `834 ms` and `909 ms` on back-to-back checks; keep the shared `freeze_text_plan(...)` helper local to `string.rs` until new asm evidence appears
- do not reopen `set_his` helper splitting before the retained-boundary proof lands
- do not reopen loop-carry shaping before the `array_set` boundary gap shrinks
- keep genericization work on `recipe / scope / effect / policy`, not on benchmark-named branches
- keep the generalized cache/scope machinery intact while tightening the hot leaf path
- next implementation cut must be compiler-local and large:
  - do not reopen helper-local widening
  - do not merge `array.set` and trailing `length()` into one semantic boundary
  - prefer the trace+asm bundle over new leaf retries when deciding the next slice
- do not reopen `route.rs` / compare-bridge policy unless new evidence shows route cost dominates again
- keep the stage0 llvmlite lane and stage1 root-first mainline intact

## Notes

- `compile_json_path` / `mir_json_to_object*` are no longer daily-facing.
- No new delete-ready surface is known.
