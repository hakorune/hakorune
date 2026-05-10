---
Status: SSOT
Decision: accepted
Date: 2026-05-08
Scope: mimalloc-grade allocator substrate task order, capability-module boundary, and required manual-update contract.
Related:
  - docs/development/current/main/design/substrate-capability-ladder-ssot.md
  - docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md
  - docs/development/current/main/design/minimal-capability-modules-ssot.md
  - docs/development/current/main/design/minimum-verifier-ssot.md
  - docs/development/current/main/design/raw-array-substrate-ssot.md
  - docs/development/current/main/design/gc-tls-atomic-capability-ssot.md
  - docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md
  - docs/development/current/main/design/static-const-table-syntax-ssot.md
  - docs/development/current/main/design/inline-plan-ssot.md
  - docs/development/current/main/design/rune-profile-effect-capability-plan-ssot.md
  - docs/reference/runtime/substrate-capabilities.md
---

# Mimalloc Capability Taskboard (SSOT)

## Decision

The clean path for mimalloc-grade allocator work is not a broad C-style
`unsafe` surface in `.hako`.

The accepted path is:

```text
hako.mem
hako.buf
hako.ptr
hako.atomic
hako.tls
hako.osvm
@rune Contract(...)
MIR InlinePlan / EffectPlan / CapabilityPlan
minimum verifier
```

This keeps low-level power explicit, staged, and auditable. Capability modules
provide the substrate vocabulary. `@rune Contract(...)` states obligations such
as `no_alloc` / `no_safepoint`. Verifiers must prove those obligations before a
backend may trust them for lowering or optimization.

`@rune Profile(...)` is an authoring shortcut over these facts. It is not a new
truth source and is not backend-readable. The live M12c surface accepts only the
reserved names in `docs/reference/mir/rune-profile-registry.md` and expands
them into MIR-owned plan facts.

## Non-Goals

- Do not add unrestricted C-like pointer arithmetic as a language-wide feature.
- Do not add a monolithic `hako.sys` unsafe shelf.
- Do not make allocator-specific C shim branches for `Mi*`, `HakoAlloc*`, or
  any app-specific box name.
- Do not treat syntax acceptance as implementation acceptance.
- Do not make `@rune Contract(...)` backend-active without verifier proof.
- Do not make `@rune Profile(...)` backend-active or let `.inc` / ll_emit branch
  on profile names.
- Do not mix real-app EXE parity cards with broad substrate widening in the
  same commit.

## Status Legend

| Status | Meaning |
| --- | --- |
| `live-narrow` | A first row exists, but it is intentionally small. |
| `reserved` | Vocabulary is named, but not live. |
| `next-card` | Ready to turn into one implementation card when the lane opens. |
| `blocked` | Must wait for an earlier row. |

## Task Rows

| Row | Status | Owner | Required output |
| --- | --- | --- | --- |
| `M0a numeric type-name storage lock` | `live-narrow` | language + MIR + typed-object storage | `usize/isize` and fixed-width integer type-name classifier; typed-object inline i64 storage hints; exact width/range/overflow deferred |
| `M0b numeric arithmetic semantics lock` | `live-narrow` | language + MIR + backends | current `>>` is signed i64 arithmetic shift; logical shift and wrapping/checked arithmetic remain explicit future rows |
| `M1 raw layout vocabulary` | `live-narrow` | language + MIR layout facts | MIR-owned `repr_c_v0` vocabulary for fixed-width numeric fields; source syntax, pointer-sized fields, and backend-active native layout remain future rows |
| `M2 hako.mem/buf/ptr widening` | `live-narrow` | capability substrate | restricted memory/buffer/pointer facades; `BufCoreBox.cap_i64` routes through `PtrCoreBox.slot_cap_i64`; no unrestricted unsafe |
| `M3 RawBuf + RawArray allocator fixture` | `live-narrow` | algorithm substrate | allocator-shaped fixture using RawBuf/RawArray only; RawArray has len/cap/reserve/grow shape; no TLS/atomic/OSVM dependency |
| `M4 minimum verifier hardening` | `live-narrow` | verifier substrate | RawArray remove/insert now pass bounds/initialized-range gates; slice, double-free, and use-after-free remain follow-up splits |
| `M5 rune contract verifier` | `live-narrow` | rune metadata + verifier | `@rune Contract(no_alloc)` / `@rune Contract(no_safepoint)` are checked by the MIR verifier before backend use; backend export/use remains disabled |
| `M6 hako.atomic useful rows` | `live-narrow` | capability substrate | memory-order vocabulary plus ordered fence row are live; fixed-slot CAS/load/store/fetch_add are live through M27-M30 EXE proof rows; M35/M39/M40 activate direct native-pointer store/load/CAS routes; ordered fixed-slot args remain future splits; no allocator policy inside atomic module |
| `M7 hako.tls useful rows` | `live-narrow` | capability substrate | diagnostics TLS status helpers and fixed cache-slot get/set EXE proof are live; generic thread/task-local slot remains a future split; no helper-local cache exposure as final API |
| `M8 hako.osvm allocator rows` | `live-narrow` | capability substrate + native keep | page_size/reserve/commit/decommit facades are live with native metal leaf below; allocator policy remains outside osvm |
| `M9 intrinsic rows` | `live-narrow` | intrinsic metadata + LLVM/VM | `clz_i64`, `ctz_i64`, and `popcnt_i64` current-lane non-negative i64 rows are live; `prefetch`, `assume`, `unreachable`, unsigned-width semantics, and backend optimization use remain future splits |
| `M10a export attrs consistency gate` | `live-narrow` | optimization export service | guard locks current weak export attrs and rejects strong attr names in active LLVM/runtime-decl export points; no backend fact widening |
| `M10b runtime-decl readonly fact guard` | `live-narrow` | optimization export service | manifest `readonly` attrs must match `memory = "read"`; missing readonly remains allowed for conservative rows |
| `M10c-pre pointer/handle return proof vocabulary` | `live-narrow` | optimization export proof | locks handle return classes separately from native pointer return classes before any strong LLVM pointer attrs; no attr export yet |
| `M10c-proof-row runtime-decl return proof row` | `live-narrow` | optimization export proof | locks backend-private return proof row schema and validator in fixture/code while keeping active runtime-decl attrs and `.inc` unchanged |
| `M10c-native-ptr-declare-type` | `live-narrow` | `.hako` ll_emit runtime-decl reader | maps native pointer value classes to LLVM `ptr` spelling only; no attrs, no active native pointer rows, no proof inference |
| `M10c-hako-mem-alloc-row` | `live-narrow` | runtime-decl manifest + hako.mem seam | adds the first active native pointer runtime-decl row for existing `hako_mem_alloc`, nullable only and without `ret_proofs` or strong attrs |
| `M10c-hako-mem-realloc-row` | `live-narrow` | runtime-decl manifest + hako.mem seam | adds the second active native pointer runtime-decl row for existing `hako_mem_realloc`, nullable only and without `ret_proofs` or strong attrs |
| `M10c-native-ptr-call-arg-emit` | `live-narrow` | `.hako` ll_emit call policy/text emit | validates manifest extern arg classes and emits `native_ptr_*` call operands as LLVM `ptr`; no casts, proof inference, or `.inc` logic |
| `M10c-hako-mem-free-void-row` | `live-narrow` | runtime-decl manifest + `.hako` ll_emit | adds the third hako.mem row for existing `hako_mem_free`, with `void` return and nullable native pointer arg; emits `call void`, no ret_proofs or strong attrs |
| `M10c LLVM export attrs widening` | `blocked` | optimization export | `noalias`, `nonnull`, `dereferenceable`, alignment, stronger `nocapture` only after pointer/native-ptr proof and verifier/export consistency proof |
| `M11a static readonly data segment` | `live-narrow` | backend-private const data | backend-private static data manifest emits a readonly u16 size-class fixture as LLVM data; no source syntax or const eval |
| `M11b const eval/static table syntax` | `live-narrow` | language + MIR const data | `M11b-decl` source `u16` static const table declarations, `M11b-load` static table reads, and `M11b-eval` narrow integer initializer expressions are live; const fn remains future |
| `M11c InlinePlan rows` | `live-narrow` | rune metadata + MIR optimizer/verifier | `M11c-preserve` keeps `Hint(inline/noinline/hot/cold)` as MIR `inline_plans`; `M11c-soft-leaf` expands best-effort same-module pure leaf `Hint(inline)` calls in MIR; `M11c-required-vocab` preserves substrate-only `Lowering(inline_required)` as MIR `request=required`; `M11c-contract-repeat` permits distinct `Contract(...)` runes on one declaration; `M11c-required-verify` fail-fast verifies required contracts plus narrow leaf shape and sets accepted plans to `verified=true`; no backend use |
| `M11d EffectPlan/CapabilityPlan boundary` | `live-narrow` | MIR metadata + verifier | `Contract(no_alloc/no_safepoint)` now populates MIR `effect_plans`, the rune contract verifier consumes `EffectPlan`, and `capability_plans` exists as a metadata boundary; no backend use |
| `M12 mimalloc raw-page proof` | `live-narrow` | allocator substrate consumer | `apps/mimalloc-raw-page-proof` proves a fixed raw page/free-list fixture over `RawBufCoreBox` + `RawArrayCoreBox`; fast-path acquire/release carry `Contract(no_alloc/no_safepoint)` and are MIR-verified, with no Profile/Capability parser surface or backend use in that fixture |
| `M12b Profile registry docs` | `live-docs` | rune metadata + docs | `docs/reference/mir/rune-profile-registry.md` reserves `allocator.fast`, `allocator.slow`, `substrate.leaf`, `intrinsic.leaf`, and `raw.layout` expansion targets; no parser acceptance unless a later row explicitly owns parser parity |
| `M12c Profile expansion to facts` | `live-narrow` | rune metadata + MIR plans + verifier | `Profile(...)` accepts the reserved registry names and expands to primitive `Hint` / `Lowering` / `Contract` / EffectPlan / CapabilityPlan facts; backend reads only expanded facts and must not read profile names |
| `M13 allocator fast-path EXE proof` | `live-narrow` | MIR optimizer + EXE proof | `apps/allocator-fast-path-exe-proof` proves scalar `Profile(allocator.fast)` lowering through verified required InlinePlan consumption before pure-first EXE; backend/.inc remain profile-name-free, and RawBuf/RawArray/native pointer EXE lowering remains future |
| `M14 hako.mem extern pure-first route` | `live-narrow` | MIR extern route + pure-first EXE | accepts only `hako_mem_alloc` and `hako_mem_free` as MIR-owned extern route facts; pure-first emits native `ptr` calls plus i64 transport conversion/zero sentinel and links through NyRT exports; no strong pointer attrs, no realloc, no RawBuf/RawArray parity |
| `M15 RawBuf global wrapper generic-i64 route` | `live-narrow` | MIR global route + pure-first EXE | accepts only `RawBufCoreBox.alloc_bytes_i64/free_bytes_i64` as generic-i64 same-module global wrappers over M14 hako.mem routes; preserves void-sentinel trace call results as void sentinel; no RawArray parity, no realloc, no pointer attrs |
| `M16 RawArray slot_append_any generic-i64 route` | `live-narrow` | MIR extern/global route + pure-first EXE | accepts only `RawArrayCoreBox.slot_append_any` over explicit `nyash.any.handle_live_h` and `nyash.array.slot_append_hh` extern routes; no slot load/store, bounds, initialized range, slot_len, or full RawArray parity |
| `M17 RawArray slot_len_i64 generic-i64 route` | `live-narrow` | MIR extern/global route + pure-first EXE | accepts only `RawArrayCoreBox.slot_len_i64`, `BufCoreBox.len_i64`, bounds, and initialized-range wrappers over explicit `nyash.array.slot_len_h` extern route facts; no slot load/store or full RawArray parity |
| `M18 RawArray slot_load_i64 generic-i64 route` | `live-narrow` | MIR extern/global route + pure-first EXE | accepts only `RawArrayCoreBox.slot_load_i64` over ownership, bounds, initialized-range, and explicit `nyash.array.slot_load_hi` extern route facts; no slot_store or full RawArray parity |
| `M19 RawArray slot_store_i64 generic-i64 route` | `live-narrow` | MIR extern/global route + pure-first EXE | accepts only `RawArrayCoreBox.slot_store_i64` over ownership, bounds, and explicit `nyash.array.slot_store_hii` extern route facts; no handle/string store or broad ArrayBox parity |
| `M20 mimalloc raw-page EXE parity guard` | `live-narrow` | pure-first EXE regression guard | locks `apps/mimalloc-raw-page-proof` build/run under pure-first over M14-M19 routes; no new route shape or allocator policy |
| `M21 mimalloc size-class table EXE proof` | `live-narrow` | allocator app proof + pure-first static-data reader | composes M11b static const u16 size-class tables with the M14-M20 raw-page route surface in `apps/mimalloc-size-class-table-proof`; adds only narrow `u16` `static_data_plans` / `static_data_load` lowering in pure-first, with no new source syntax or allocator policy |
| `M22 mimalloc two-class page EXE proof` | `live-narrow` | allocator app proof | composes M21 static tables with two M14-M20 raw pages in `apps/mimalloc-two-class-page-proof`; proves small/medium page reject/release/reuse without new source syntax, table type, route shape, or allocator policy |
| `M23 mimalloc dynamic bin EXE proof` | `live-narrow` | allocator app proof | proves non-constant `static_data_load` indices for `u16` size-class tables in `apps/mimalloc-dynamic-bin-proof`; raw-page operations still use existing M14-M20 route facts |
| `M24 mimalloc size_to_bin inline EXE proof` | `live-narrow` | allocator app proof + MIR inline consumption | proves a `Profile(allocator.fast)` `size_to_bin` helper is verified and inlined before pure-first lowering, then feeds dynamic `static_data_load` indices in `apps/mimalloc-size-to-bin-inline-proof`; no backend profile consumption |
| `M25 mimalloc OSVM page EXE proof` | `live-narrow` | allocator app proof + MIR extern route + NyRT export | proves `OsVmCoreBox.reserve_bytes_i64/commit_bytes_i64/decommit_bytes_i64` as MIR-owned extern route facts in `apps/mimalloc-osvm-page-proof`; pure-first emits only those route rows and links matching NyRT exports, with no page-size route row, unreserve API, TLS, atomic, or allocator policy |
| `M26 mimalloc TLS cache-slot EXE proof` | `live-narrow` | allocator app proof + MIR extern route + NyRT export | proves `TlsCoreBox.cache_slot_get_i64/cache_slot_set_i64` as MIR-owned extern route facts in `apps/mimalloc-tls-cache-slot-proof`; pure-first emits only those route rows and links matching NyRT exports, with no generic TLS cell, atomic remote-free, native pointer attrs, or allocator policy |
| `M27 mimalloc atomic CAS slot EXE proof` | `live-narrow` | allocator app proof + MIR extern route + NyRT export | proves `AtomicCoreBox.cas_i64` as a fixed i64 atomic-slot CAS route in `apps/mimalloc-atomic-cas-proof`; pure-first emits only that route row and links the matching NyRT export, with no load/store/fetch_add, pointer atomics, memory-order args, or remote-free policy |
| `M28 mimalloc atomic load slot EXE proof` | `live-narrow` | allocator app proof + MIR extern route + NyRT export | proves `AtomicCoreBox.load_i64` as a fixed i64 atomic-slot load route in `apps/mimalloc-atomic-load-proof`; pure-first emits only that route row and links the matching NyRT export, with no store/fetch_add, pointer atomics, memory-order args, or remote-free policy |
| `M29 mimalloc atomic store slot EXE proof` | `live-narrow` | allocator app proof + MIR extern route + NyRT export | proves `AtomicCoreBox.store_i64` as a fixed i64 atomic-slot store route in `apps/mimalloc-atomic-store-proof`; pure-first emits only that route row and links the matching NyRT export, with no fetch_add, pointer atomics, memory-order args, or remote-free policy |
| `M30 mimalloc atomic fetch-add slot EXE proof` | `live-narrow` | allocator app proof + MIR extern route + NyRT export | proves `AtomicCoreBox.fetch_add_i64` as a fixed i64 atomic-slot fetch-add route in `apps/mimalloc-atomic-fetch-add-proof`; pure-first emits only that route row and links the matching NyRT export, with no pointer atomics, memory-order args, or remote-free policy |
| `M31 mimalloc remote-free i64 sketch EXE proof` | `live-narrow` | allocator app composition proof | composes the existing CAS/load/store/fetch_add fixed-slot i64 routes in `apps/mimalloc-remote-free-i64-proof` to prove the remote-free push pattern under pure-first EXE; adds no new route row, NyRT export, pointer atomics, memory-order args, or production allocator policy |
| `M32 mimalloc post-M31 task-order lock` | `live-docs` | taskboard/docs | locks the post-M31 order and syncs stale M6/M7 wording; no source, route, `.inc`, NyRT, or allocator policy change |
| `M33 atomic memory-order args docs/route vocabulary lock` | `live-docs` | capability substrate + docs | names ordered fixed-slot i64 facade shapes and route vocabulary for future rows while keeping AtomicCoreBox methods, MIR routes, `.inc`, NyRT exports, pointer atomics, and allocator policy inactive |
| `M34 pointer atomic vocabulary docs lock` | `live-docs` | capability substrate + docs | reserves native-pointer atomic load/store/CAS facade and route vocabulary after M33 while keeping AtomicCoreBox methods, pointer attrs, and allocator policy inactive; M35 activates store, M39 activates load, and M40 activates CAS |
| `M35 native pointer atomic route proof` | `live-narrow` | MIR extern route + pure-first EXE | proves the first active pointer-shaped atomic row, `hako_atomic_ptr_store_ordered(cell_ptr, value_ptr, order)`, through MIR-owned extern route facts, NyRT export, and pure-first native-ptr argument lowering without noalias/nonnull widening |
| `M36 TLS + pointer remote-free composition proof` | `live-narrow` | allocator app composition proof | composes M26 TLS cache-slot rows with the M35 direct pointer-store route in `apps/mimalloc-tls-ptr-remote-free-proof`; proves mailbox pointer publication under pure-first without adding route rows, pointer CAS/fetch_add, or production allocator policy |
| `M37 allocator remote-free policy integration proof` | `live-narrow` | allocator policy app proof | connects the M36 remote-free mailbox seam to `AllocatorRemoteFreePolicy` in `apps/mimalloc-remote-free-policy-proof`; adds no route rows, NyRT exports, pointer CAS/fetch_add, or backend-specific matchers |
| `M38 mimalloc allocator app closeout guard` | `live-narrow` | regression guard | locks the M20-M37 mimalloc allocator app proof path through a fast coverage guard; adds no app, route row, NyRT export, or `.inc` matcher |
| `M39 native pointer atomic load route proof` | `live-narrow` | MIR extern route + pure-first EXE | proves `hako_atomic_ptr_load_ordered(cell_ptr, order)` through MIR-owned extern route facts, NyRT export, and pure-first native-ptr return lowering without activating pointer fetch_add or native pointer attrs |
| `M40 native pointer atomic CAS route proof` | `live-narrow` | MIR extern route + pure-first EXE | proves `hako_atomic_ptr_cas_ordered(cell_ptr, expected_ptr, desired_ptr, success_order, failure_order)` after M39; keeps pointer fetch_add and native pointer attrs inactive |
| `M41 pointer CAS remote-free list proof` | `live-narrow` | allocator app composition proof | composes pointer store/load/CAS routes into a two-node remote-free list push proof; adds no new route row, NyRT export, `.inc` emit behavior, pointer fetch_add, or allocator production policy |
| `M42 allocator remote-free list policy integration proof` | `live-narrow` | allocator policy app proof | moves the M41 list push shape behind a same-module policy box; adds no new route row, NyRT export, `.inc` emit behavior, pointer fetch_add, or backend-specific matcher |
| `M43 allocator remote-free retry-loop proof` | `live-narrow` | allocator policy app proof | proves a bounded CAS retry-loop acceptance shape inside a same-module policy box; adds no new route row, NyRT export, pointer fetch_add, or production allocator policy; adds only generic no-result consumption for existing pointer store/CAS route facts, not app-specific backend matching |
| `M44 mimalloc allocator substrate closeout guard` | `live-narrow` | regression guard | inventories and locks the M20-M43 mimalloc allocator substrate proof path before starting production allocator port work; adds no app, route row, NyRT export, or `.inc` matcher |
| `M45 production allocator port entry plan` | `live-narrow` | task split | turns the M20-M44 substrate proof ladder into a production allocator port plan without widening pointer attrs, activating pointer fetch_add, or adding allocator replacement hooks implicitly |
| `M46 hako_alloc production facade boundary` | `live-narrow` | hako_alloc facade contract | creates the production-facing allocator facade boundary under `hako_alloc` while routing only through existing page/free-list policy state; no replacement hook or route widening |
| `M47 allocator local page policy proof` | `live-narrow` | allocator policy proof | proves local allocate/free policy through the production facade without remote-free or OS VM ownership widening |
| `M48 allocator remote-free policy proof` | `live-narrow` | allocator policy proof | composes the M43 retry-loop shape behind the production facade while keeping pointer atomics in substrate |
| `M49 allocator OSVM page-source proof` | `live-narrow` | allocator page-source proof | composes page reserve/commit/decommit rows as a page-source seam without adding unreserve/release rows |
| `M50 allocator stress production-facade parity` | `live-narrow` | allocator stress app | adds production-facade stress coverage while keeping existing allocator-stress as regression coverage |
| `M51 production allocator port closeout guard` | `live-narrow` | regression guard | inventories M46-M50 production allocator port proof coverage before any allocator replacement hook design |
| `M52 allocator replacement hook boundary` | `live-docs` | hook boundary | adds the allocator replacement hook SSOT and guard while keeping process allocator replacement, hook env toggles, `.inc` name matching, and route widening inactive |
| `M53 allocator HookPlan vocabulary lock` | `live-docs` | hook plan vocabulary | adds reserved HookPlan v0 SSOT and TOML fixture while keeping runtime hooks, process allocator replacement, env toggles, and `.inc` name matching inactive |
| `M54 allocator hook runtime dry-run boundary` | `live-docs` | runtime hook dry-run boundary | names the future runtime dry-run seam while keeping runtime hook code, process allocator replacement, env toggles, and `.inc` name matching inactive |
| `M55 allocator hook activation proof` | `live-docs` | activation proof vocabulary | adds reserved activation proof SSOT and TOML fixture while keeping runtime hooks, process allocator replacement, env toggles, and `.inc` name matching inactive |
| `M56 allocator hook runtime owner row` | `live-docs` | runtime owner boundary | names `src/runtime/allocator_hook_dry_run.rs` as the future dry-run owner while keeping the file and hook implementation absent |
| `M57 allocator hook runtime dry-run code` | `live-narrow` | runtime dry-run code | adds diagnostic-only runtime validation for HookPlan/proof presence; never installs or replaces the process allocator |
| `M58 allocator hook dry-run manifest callsite` | `live-narrow` | runtime manifest callsite | feeds reserved HookPlan/proof TOML text into the diagnostic-only runtime validator without file/env discovery or hook installation |
| `M59 allocator hook dry-run test surface` | `live-narrow` | test-only surface | adds a `#[cfg(test)]` reserved-fixture observation helper without CLI flags, env toggles, file discovery, or hook installation |
| `M60 allocator hook activation proof validator` | `live-narrow` | activation proof validator | validates reserved activation-proof TOML text as diagnostics only while keeping activation, env/CLI surface, file discovery, and process allocator replacement inactive |
| `M61 allocator hook dry-run CLI surface` | `live-narrow` | diagnostic CLI surface | exposes explicit plan/proof file dry-run diagnostics without environment toggles, implicit discovery, runner ownership, activation, or process allocator replacement |
| `M62 allocator hook activation preflight boundary` | `live-docs` | activation preflight boundary | names the reentrancy/bootstrap/no-alloc/rollback/fail-fast handoff required before any allocator hook activation or process allocator replacement row |
| `M63 allocator hook activation preflight shape` | `live-narrow` | activation preflight data shape | adds diagnostic-only runtime preflight facts/report with stable missing-fact names and `would_activate=false`; no activation or replacement hook |
| `M64 allocator provider boundary vocabulary` | `live-docs` | provider boundary vocabulary | reserves provider ids for system allocator, mimalloc, hako model, and guarded debug providers while keeping provider registry, selection, and replacement inactive |
| `M65 allocator provider manifest vocabulary` | `live-docs` | provider manifest vocabulary | adds reserved provider manifest TOML fixture for the M64 provider ids while keeping runtime parser, provider registry, selection, and replacement inactive |
| `M66 allocator provider task breakdown` | `live-docs` | task breakdown | adds a readable task ladder for M52-M65 completion and M67-M75 next rows without runtime/provider activation |
| `M67 allocator provider manifest parser` | `live-narrow` | diagnostic parser | parse caller-provided provider manifest TOML into a diagnostic report; no file discovery, provider selection, or replacement |
| `M68 allocator provider manifest CLI surface` | `live-narrow` | diagnostic CLI surface | exposes explicit provider manifest file diagnostics without env toggles, implicit discovery, runner ownership, provider selection, or replacement |
| `M69 allocator provider readiness preflight shape` | `live-narrow` | provider readiness data shape | connects provider manifest readiness to activation preflight diagnostics while keeping `would_select_provider=false` and `would_activate=false` |
| `M70 combined hook/provider dry-run report` | `live-narrow` | combined diagnostic report | combines hook plan, activation proof, activation preflight, provider manifest, and provider readiness diagnostics while keeping install/selection/activation false |
| `M71 allocator provider registry boundary` | `live-docs` | registry boundary docs | names future registry ownership/API and stop line while keeping active registry implementation, selection, and replacement absent |
| `M72 hako model provider proof fixture` | `live-docs` | model provider proof | adds a reserved hako model provider proof fixture while keeping provider selection, native metal activation, and replacement inactive |
| `M73 debug guarded provider proof fixture` | `live-docs` | guarded provider proof | adds a reserved debug guarded provider proof fixture while keeping provider selection, hook activation, and replacement inactive |
| `M74 native system provider proof boundary` | `live-docs` | native system provider boundary | adds a reserved native system provider proof boundary while keeping `#[global_allocator]`, provider selection, and replacement inactive |
| `M75 native mimalloc provider proof boundary` | `live-docs` | native mimalloc provider boundary | adds a reserved native mimalloc provider proof boundary while keeping production activation, provider selection, and replacement inactive |
| `M76 allocator provider activation entry contract` | `live-docs` | activation entry contract | names future registry/selection ownership, proof consumption, fail-fast diagnostics, and rollback behavior while keeping runtime registry code, activation, and replacement inactive |
| `M77 allocator provider registry snapshot` | `live-docs` | registry snapshot shape | adds a reserved registry snapshot fixture with provider entries and missing diagnostics while keeping runtime registry code, provider selection, and replacement inactive |
| `M78 allocator provider selection decision` | `live-docs` | selection decision shape | adds a reserved caller-provided selection request/decision fixture with no selected provider while keeping selection implementation, activation, and replacement inactive |
| `M79 allocator provider proof bundle consumption` | `live-docs` | proof bundle shape | adds a reserved provider proof bundle consumption fixture with selected-provider proof inputs while keeping runtime proof consumption, activation, and replacement inactive |
| `M80 allocator provider rollback preflight` | `live-docs` | rollback preflight shape | adds a reserved rollback preflight fixture with rollback target facts while keeping rollback preparation, hook activation, and replacement inactive |
| `M81 allocator provider activation safety gate` | `live-docs` | activation safety gate shape | adds a reserved activation evidence bundle fixture with gate-closed facts while keeping gate opening, hook activation, and replacement inactive |
| `M82 allocator provider activation safety diagnostic owner` | `live-docs` | diagnostic owner boundary | names the runtime diagnostic owner and removes stale past-guard pins against future owner files/type names while keeping activation implementation inactive |
| `M83 allocator provider activation safety diagnostic report` | `live-narrow` | runtime diagnostic report | adds the runtime-owned activation safety report over caller-provided TOML text while keeping gate opening, hook activation, and replacement inactive |
| `M84 allocator provider activation safety CLI surface` | `live-narrow` | diagnostic CLI surface | exposes the activation safety report through an explicit caller-provided TOML path while keeping gate opening, hook activation, and replacement inactive |
| `M85 allocator provider activation safety closeout inventory` | `live-narrow` | regression guard | inventories M76-M84 activation safety diagnostic coverage before any later activation decision row |

## Fixed Implementation Order

1. `M0a numeric type-name storage lock`
2. `M0b numeric arithmetic semantics lock`
3. `M1 raw layout vocabulary`
4. `M4 minimum verifier hardening`
5. `M2 hako.mem/buf/ptr widening`
6. `M3 RawBuf + RawArray allocator fixture`
7. `M5 rune contract verifier`
8. `M6 hako.atomic useful rows`
9. `M7 hako.tls useful rows`
10. `M8 hako.osvm allocator rows`
11. `M9 intrinsic rows`
12. `M10a export attrs consistency gate`
13. `M10b runtime-decl readonly fact guard`
14. `M11a static readonly data segment`
15. `M11b-decl source static const table declaration`
16. `M11b-load static table read route`
17. `M11c-docs InlinePlan boundary lock`
18. `M11b-eval const integer expression table generation`
19. `M11c-preserve Hint inline/noinline/hot/cold into MIR InlinePlan`
20. `M11c-soft-leaf best-effort same-module MIR inline`
21. `M10c-pre pointer/handle return proof vocabulary`
22. `M10c-proof-row runtime-decl return proof row`
23. `M10c-native-ptr-declare-type`
24. `M10c-hako-mem-alloc-row`
25. `M10c-hako-mem-realloc-row`
26. `M10c-native-ptr-call-arg-emit`
27. `M10c-hako-mem-free-void-row`
28. `M10c LLVM export attrs widening` (blocked until an eligible native-pointer proof/export row exists)
29. `M11c-required-vocab substrate-only Lowering(inline_required)`
30. `M11c-contract-repeat distinct Contract(...) parser metadata`
31. `M11c-required-verify verifier-backed required inline acceptance`
32. `M11d EffectPlan/CapabilityPlan boundary`
33. `M12 mimalloc raw-page proof`
34. `M12b Profile registry docs`
35. `M12c Profile expansion to facts`
36. `M13 allocator fast-path EXE proof`
37. `M14 hako.mem extern pure-first route`
38. `M15 RawBuf global wrapper generic-i64 route`
39. `M16 RawArray slot_append_any generic-i64 route`
40. `M17 RawArray slot_len_i64 generic-i64 route`
41. `M18 RawArray slot_load_i64 generic-i64 route`
42. `M19 RawArray slot_store_i64 generic-i64 route`
43. `M20 mimalloc raw-page EXE parity guard`
44. `M21 mimalloc size-class table EXE proof`
45. `M22 mimalloc two-class page EXE proof`
46. `M23 mimalloc dynamic bin EXE proof`
47. `M24 mimalloc size_to_bin inline EXE proof`
48. `M25 mimalloc OSVM page EXE proof`
49. `M26 mimalloc TLS cache-slot EXE proof`
50. `M27 mimalloc atomic CAS slot EXE proof`
51. `M28 mimalloc atomic load slot EXE proof`
52. `M29 mimalloc atomic store slot EXE proof`
53. `M30 mimalloc atomic fetch-add slot EXE proof`
54. `M31 mimalloc remote-free i64 sketch EXE proof`
55. `M32 mimalloc post-M31 task-order lock`
56. `M33 atomic memory-order args docs/route vocabulary lock`
57. `M34 pointer atomic vocabulary docs lock`
58. `M35 native pointer atomic route proof`
59. `M36 TLS + pointer remote-free composition proof`
60. `M37 allocator remote-free policy integration proof`
61. `M38 mimalloc allocator app closeout guard`
62. `M39 native pointer atomic load route proof`
63. `M40 native pointer atomic CAS route proof`
64. `M41 pointer CAS remote-free list proof`
65. `M42 allocator remote-free list policy integration proof`
66. `M43 allocator remote-free retry-loop proof`
67. `M44 mimalloc allocator substrate closeout guard`
68. `M45 production allocator port entry plan`
69. `M46 hako_alloc production facade boundary`
70. `M47 allocator local page policy proof`
71. `M48 allocator remote-free policy proof`
72. `M49 allocator OSVM page-source proof`
73. `M50 allocator stress production-facade parity`
74. `M51 production allocator port closeout guard`
75. `M52 allocator replacement hook boundary`
76. `M53 allocator HookPlan vocabulary lock`
77. `M54 allocator hook runtime dry-run boundary`
78. `M55 allocator hook activation proof`
79. `M56 allocator hook runtime owner row`
80. `M57 allocator hook runtime dry-run code`
81. `M58 allocator hook dry-run manifest callsite`
82. `M59 allocator hook dry-run test surface`
83. `M60 allocator hook activation proof validator`
84. `M61 allocator hook dry-run CLI surface`
85. `M62 allocator hook activation preflight boundary`
86. `M63 allocator hook activation preflight shape`
87. `M64 allocator provider boundary vocabulary`
88. `M65 allocator provider manifest vocabulary`
89. `M66 allocator provider task breakdown`
90. `M67 allocator provider manifest parser`
91. `M68 allocator provider manifest CLI surface`
92. `M69 allocator provider readiness preflight shape`
93. `M70 combined hook/provider dry-run report`
94. `M71 allocator provider registry boundary`
95. `M72 hako model provider proof fixture`
96. `M73 debug guarded provider proof fixture`
97. `M74 native system provider proof boundary`
98. `M75 native mimalloc provider proof boundary`
99. `M76 allocator provider activation entry contract`
100. `M77 allocator provider registry snapshot`
101. `M78 allocator provider selection decision`
102. `M79 allocator provider proof bundle consumption`
103. `M80 allocator provider rollback preflight`
104. `M81 allocator provider activation safety gate`
105. `M82 allocator provider activation safety diagnostic owner`
106. `M83 allocator provider activation safety diagnostic report`
107. `M84 allocator provider activation safety CLI surface`
108. `M85 allocator provider activation safety closeout inventory`

This order may be split further, but it must not be inverted unless a new SSOT
card explains the dependency change. `M11c-required-vocab` is allowed to proceed
while `M10c LLVM export attrs widening` remains blocked because it does not
export pointer attributes, infer native-pointer proof, or lower allocator fast
paths. It only widens rune vocabulary and preserves a MIR-owned metadata row
for a later verifier card.

`@rune Profile(...)` stays after the plan-boundary rows. Profile is allowed to
reduce authoring noise only because the primitive facts it expands to already
exist and are MIR/verifier-owned.

## Per-Row Acceptance Contract

Each implementation row must land as:

```text
one row
one fixture/gate
one manual update
one commit
```

Required acceptance fields:

- owner layer
- syntax or API surface, if any
- MIR/value representation row
- VM behavior
- LLVM/EXE behavior
- Stage0 / MIR JSON behavior when exposed across that boundary
- fail-fast diagnostic for unsupported consumers
- smoke or unit gate
- manual update paths

## Manual Update Contract

Every implementation row must update user-facing or reference documentation in
the same commit.

Default manual targets:

- substrate capability manual:
  - `docs/reference/runtime/substrate-capabilities.md`
- numeric/language syntax rows:
  - `docs/reference/language/types.md`
  - `docs/reference/language/EBNF.md`
- rune contract rows:
  - `docs/reference/mir/hints.md`
  - `docs/reference/runtime/substrate-capabilities.md`
- ABI/export rows:
  - `docs/reference/abi/ABI_INDEX.md`
  - `docs/reference/abi/ABI_BOUNDARY_MATRIX.md`
- MIR metadata / route facts:
  - `docs/reference/mir/metadata-facts-ssot.md`

If a row does not update a manual, the implementation card must state why the
row is internal-only and name the future manual update trigger.

## Parser Update Rule

When a row adds syntax, both parser fronts must be considered:

- Rust parser / tokenizer path
- `.hako` selfhost parser path

If the row can be expressed as a capability box call without new syntax, prefer
that first. If new syntax is required, the implementation card must include the
Rust parser, selfhost parser, EBNF, and MIR JSON acceptance plan.

## Rune Contract Rule

`@rune Contract(...)` is not a comment and not a hint once a row is backend
active.

Before backend use:

1. parse and preserve the contract
2. lower it into MIR-owned metadata
3. verify it structurally
4. fail-fast on violation
5. export only the proven fact

Examples:

```hako
@rune Contract(no_alloc)
@rune Contract(no_safepoint)
method alloc_fast(size: usize) -> Ptr<u8> {
  ...
}
```

The backend may not infer `no_alloc` from the method name, app name, or helper
choice.

## Current Reading

Current real-app work may continue on VM/EXE parity using typed-object planning.
That is separate from this taskboard.

The first mimalloc-grade substrate card landed as `M0a numeric type-name
storage lock`. `M0b numeric arithmetic semantics lock` now fixes the current
`>>` behavior as signed i64 arithmetic shift, while logical shift and
wrapping/checked arithmetic remain future explicit rows. `M1 raw layout
vocabulary` now gives future allocator rows a MIR-owned `repr_c_v0` layout
target for fixed-width numeric fields only. `M4 minimum verifier hardening`
now covers RawArray remove/insert verifier gates. `M2` is narrowing the current
memory/buffer/pointer capability split so buffer shape facades do not own direct
backend slot ABI names. `M3` now has a readable RawArray capacity observer over
the buffer facade. `M5` now checks `Contract(no_alloc)` and
`Contract(no_safepoint)` in the MIR verifier, but those facts are not exported
to backend optimization yet. `M9a` now exposes `hako.intrin` bit-count rows for
current-lane non-negative i64 values; this does not activate
`@rune IntrinsicCandidate` or backend optimization use. `M11a` now proves the
backend-private static readonly data seam with a generated u16 size-class
fixture. `M11b-decl` now accepts source-level
`static const NAME: u16[] = [...]` declarations into MIR `static_data_plans`.
`M11b-load` now accepts `NAME[index]` as a MIR-owned static-data load for those
tables. `M11c-docs` now reserves the InlinePlan boundary so allocator fast-path
work does not grow `.inc` or backend-local inliners. `M11b-eval` now evaluates
narrow integer const expressions in source static `u16` table initializers.
`M11c-preserve` now keeps existing `Hint(inline/noinline/hot/cold)` as MIR
InlinePlan metadata with no backend use. `M11c-soft-leaf` now expands narrow
same-module pure leaf `Hint(inline)` calls in the MIR optimizer, while
unsupported shapes keep the call. The next code implementation target is
`M10c-pre` now locks pointer/handle return proof vocabulary in docs, TOML, and
Rust code without exporting strong attrs. `M10c-proof-row` now locks the
runtime-decl return proof row schema in a fixture plus Rust validator, while
keeping the active runtime-decl manifest, generated `.hako` defaults, and `.inc`
free of strong attrs or pointer-proof inference. `M10c-native-ptr-declare-type`
now maps native pointer value classes to LLVM `ptr` spelling in the `.hako`
runtime-decl reader, but still keeps attrs and active native pointer rows
disabled. `M10c-hako-mem-alloc-row` now adds `hako_mem_alloc` as the first
active native pointer runtime-decl row, with nullable return and no
`ret_proofs` or strong attrs. `M10c-hako-mem-realloc-row` adds
`hako_mem_realloc` as the second active native pointer runtime-decl row, with a
nullable native pointer argument and nullable return. The next implementation
target after that is `M10c-native-ptr-call-arg-emit`, because `realloc` makes
native pointer arguments active and the ll_emit text path must emit those
operands as `ptr`, not `i64`. `M10c-native-ptr-call-arg-emit` now validates
manifest extern arg classes before accepting direct extern calls and emits
`native_ptr_*` operands through the manifest arg class. The target after that is
`M10c-hako-mem-free-void-row` now adds the third hako.mem runtime-decl row and
teaches `.hako` ll_emit to emit `call void` for the C ABI free seam instead of
inventing an `i64` result. `M10c LLVM export attrs widening` is still blocked:
the active hako.mem rows are nullable native pointers or void and have no
eligible `nonnull` / `noalias` / `dereferenceable` proof row. `M11d` gives
strict substrate work a MIR-owned effect/capability boundary:
`Contract(no_alloc/no_safepoint)` feeds `effect_plans`, the verifier consumes
that metadata, and `capability_plans` is present but empty until capability
syntax/profile expansion lands. `M12` proves the first raw page/free-list
consumer fixture against those explicit facts. `M12b` now reserves profile names
and primitive expansion targets in `docs/reference/mir/rune-profile-registry.md`.
`M12c` now accepts the reserved `Profile(...)` names and expands them to
primitive InlinePlan / EffectPlan / CapabilityPlan facts while keeping backend
and `.inc` consumers profile-name-free. `M13 allocator fast-path EXE proof` is
live-narrow for the scalar fast-path lane: verified required InlinePlan is
consumed by the MIR optimizer before pure-first EXE, and the backend still sees
only expanded scalar MIR. Full raw-page EXE remains future work and must split
RawBuf/RawArray/native pointer route acceptance into separate rows.
`M14 hako.mem extern pure-first route` is now live-narrow because the first
raw-page EXE probe reached an inner `hako_mem_alloc/free` extern route blocker
through `RawBufCoreBox -> MemCoreBox`. M14 owns only native pointer transport
emission for direct hako.mem extern calls plus NyRT link exports, and keeps
RawBuf/RawArray wrapper parity for later rows.
`M15 RawBuf global wrapper generic-i64 route` is now live-narrow because the
post-M14 raw-page probe reached `RawBufCoreBox.alloc_bytes_i64/free_bytes_i64`
through the `MiRawPageProof.birth/destroy` bodies. M15 owns only RawBuf wrapper
classification over already-routed hako.mem calls and keeps RawArray,
Ownership/Bounds/InitializedRange, PtrCoreBox, and full raw-page EXE parity for
later rows.
`M16 RawArray slot_append_any generic-i64 route` is now live-narrow because the
post-M15 raw-page probe moved to `RawArrayCoreBox.slot_append_any/2` during
free-stack seeding. M16 owns only append-path route facts over
`OwnershipCoreBox._handle_live_i64/1` and `PtrCoreBox.slot_append_any/2`; load,
store, bounds, initialized-range, and slot length stay future.
`M17 RawArray slot_len_i64 generic-i64 route` is now live-narrow because the
post-M16 raw-page probe moved to verifier wrappers blocked by
`PtrCoreBox.slot_len_i64/1`. M17 owns only length-path route facts over
`PtrCoreBox.slot_len_i64/1`, `BufCoreBox.len_i64/1`, bounds, initialized-range,
and `RawArrayCoreBox.slot_len_i64/1`; load, store, and full RawArray parity stay
future.
`M18 RawArray slot_load_i64 generic-i64 route` is now live-narrow because the
post-M17 raw-page probe moved to the actual read leaf
`PtrCoreBox.slot_load_i64/2`. M18 owns only load-path route facts over
`PtrCoreBox.slot_load_i64/2` and the already-routed verifier wrappers; store and
full RawArray parity stay future.
`M19 RawArray slot_store_i64 generic-i64 route` is now live-narrow because the
post-M18 raw-page probe moved to the write leaf `PtrCoreBox.slot_store_i64/3`.
M19 owns only i64 store-path route facts over `PtrCoreBox.slot_store_i64/3` and
the already-routed verifier wrappers; handle/string store variants and broad
ArrayBox parity stay future.
`M20 mimalloc raw-page EXE parity guard` is now live-narrow because M14-M19
compose into a pure-first EXE build/run for `apps/mimalloc-raw-page-proof`.
M20 owns only the regression guard for that composed surface; it adds no new
route shape and no allocator policy.
`M21 mimalloc size-class table EXE proof` is now live-narrow because source
static `u16` size-class tables and the M14-M20 raw-page route surface compose
into a pure-first EXE build/run for `apps/mimalloc-size-class-table-proof`.
M21 owns only that composed app proof; it adds no new route shape, table type,
or allocator policy. The only backend acceptance added by M21 is the narrow
pure-first reader/emitter for MIR-owned `u16` `static_data_plans` and
`static_data_load`; `.inc` must not match app table names.
`M22 mimalloc two-class page EXE proof` is now live-narrow because the M21
static table seam and the M14-M20 raw-page route surface compose for two
classes. M22 owns only the app proof; dynamic bin selection, TLS, atomics,
OSVM, native pointer attrs, and allocator ownership proof remain future rows.
`M23 mimalloc dynamic bin EXE proof` is now live-narrow because the M21
pure-first `static_data_load` reader also accepts runtime `i64` indices. M23
does not add a general `size_to_bin` algorithm or new backend vocabulary.
`M24 mimalloc size_to_bin inline EXE proof` is now live-narrow because the M13
verified required inline path composes with M23 runtime-indexed static tables.
M24 owns only the narrow helper proof; the backend remains profile-name-free.
`M25 mimalloc OSVM page EXE proof` is now live-narrow because the existing
`hako.osvm` facade composes with MIR-owned extern route facts and NyRT exports
for reserve/commit/decommit. M25 owns only those three OSVM route rows, matching
runtime exports, and the app proof; page-size pure-first lowering, unreserve,
TLS, atomics, and allocator ownership proof remain future rows.
