---
Status: SSOT
Decision: provisional
Date: 2026-05-12
Scope: `stage2` allocator/handle wave and `phase-293x` real-app allocator port stop-lineとして、`hako_alloc` policy/state owner と native metal keep の concrete rows を固定する。
Related:
  - docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md
  - docs/development/current/main/design/stage2-aot-fast-lane-crossing-inventory.md
  - docs/development/current/main/design/stage2-selfhost-and-hako-alloc-ssot.md
  - docs/development/current/main/design/substrate-capability-ladder-ssot.md
  - docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md
  - docs/development/current/main/design/minimal-capability-modules-ssot.md
  - docs/development/current/main/design/minimum-verifier-ssot.md
  - docs/development/current/main/design/raw-array-substrate-ssot.md
  - docs/development/current/main/design/raw-map-substrate-ssot.md
  - docs/development/current/main/design/gc-tls-atomic-capability-ssot.md
  - docs/development/current/main/design/final-metal-split-ssot.md
  - docs/development/current/main/design/helper-boundary-policy-ssot.md
  - docs/development/current/main/design/recipe-scope-effect-policy-ssot.md
  - lang/src/hako_alloc/README.md
  - src/runtime/host_handles.rs
  - src/runtime/host_handles_policy.rs
  - src/runtime/gc_controller.rs
  - src/runtime/gc_trigger_policy.rs
---

# Hako Alloc Policy/State Contract (SSOT)

## Goal

- `hako_alloc` wave で何が policy/state owner で、何が capability substrate / native metal keep に残るかを 1 枚で固定する。
- allocator series の concrete rows を narrow に止め、`RawBuf / MaybeInit` や actual allocator backend migration と混ぜない。
- `host_handles` / `gc_controller` の mixed seam を、挙動不変の policy/body split として読む。
- allocator-like user boxes also follow the shared `recipe / scope / effect / policy / leaf` split; this doc owns only the allocator-specific policy/state rows.
- `mimalloc-lite` model と本物の native allocator fast path の境界を固定し、substrate capability なしで `hako_alloc` に metal owner を持たせない。

## Current Split

| Layer | Current owner | Current first live rows |
| --- | --- | --- |
| policy/state owner | `hako_alloc` reading + narrow Rust policy helpers | handle reuse policy, GC trigger threshold policy, VM-only page/free-list policy-state prototype |
| capability substrate | `lang/src/runtime/substrate/{mem,buf,ptr,atomic,tls,gc}/**` | truthful capability seams only |
| native metal keep | Rust runtime/kernel + C ABI shims | host handle slot table, `drop_epoch`, GC root snapshot/reachability walk, actual alloc/free/realloc, TLS/atomic/GC body |

## Mimalloc / Native Allocator Boundary

`mimalloc-lite` belongs to the current policy/state lane. It can model:

- size-class policy
- VM-only page/free-list state
- allocation/free accounting
- allocator stress behavior in real apps
- statistics and validation output

`mimalloc`-grade native fast path does not belong to this wave. It requires the
substrate capability ladder first:

- numeric substrate for `usize` and fixed-width unsigned arithmetic
- explicit wrapping/checked arithmetic semantics
- `hako.mem` / `hako.buf` / `hako.ptr` plus verifier-backed raw buffers
- fixed layout vocabulary for allocator metadata
- `MaybeInit` and initialized-range verification
- `no_alloc` / `no_safepoint` verifier before fast-path contracts are trusted
- TLS and atomics with memory order before remote-free style algorithms
- OS VM facade before page reserve/commit policy moves upward

Implementation task order is fixed by:

- [`mimalloc-capability-taskboard-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md)

The implementation path is capability modules plus verifier-backed rune
contracts, not a broad C-style unsafe language surface.

The split is:

- `hako_alloc` owns allocator policy/state rows.
- capability substrate owns raw memory, pointer, buffer, verifier, TLS, atomic,
  GC, and OS VM vocabulary.
- native metal keep owns final alloc/free/realloc, platform atomics/TLS, GC body,
  and OS syscall glue.

`LayoutBox` stays narrow VM-only size-class policy. It is not `repr(C)`,
`sizeof`, `alignof`, native alignment, or ABI layout ownership.

Allocator-shaped user boxes do not get special broad lowering here. General
user-box EXE parity follows typed-object planning and shared recipe/scope/effect
boundaries, not allocator-specific C shim branches.

## Current Mimalloc Owner Ladder

The active `.hako` allocator ladder is now explicit through the realloc failure
contract:

1. `M171` `HakoAllocPageMap` owns caller-visible `ptr -> page_id/block_id`
   identity.
2. `M172` `HakoAllocPageMapReleaseSeam` owns page-map-backed release ordering.
3. `M173` `HakoAllocPageMapReleaseObserver` freezes handle lifetime plus
   release/unregister observation around the M172 seam.
4. `M174` `HakoAllocPageMapReallocSameClassPath` owns same-class/no-move realloc.
5. `M175` `HakoAllocPageMapReallocAllocCopyReleasePath` owns grow fallback:
   replacement allocation, modeled copy, then old-ptr release.
6. `M176` `HakoAllocPageMapReallocFailureContract` owns only diagnostics: zero,
   oversized, unknown, stale, released, and alloc-fail classification.

The owner split above is the current stop line. `M176` does not move aligned
allocation, huge-page routing, secure free-list policy, or provider/hook work
into the existing realloc owners.

## Immediate Next Boundary

The next allocator row is `M177 alignment policy object`.

- It may normalize requested alignment, reject non-power-of-two inputs, and
  compute padded-size policy.
- It must not allocate aligned blocks, widen page-map release/realloc owners, or
  claim native/ABI alignment semantics.
- Huge-page routing, huge-page release, and secure-list work remain later rows.

## First Concrete Policy Rows

Implementation order is fixed narrowly:

1. handle reuse policy
2. GC trigger threshold policy
3. VM-only page/free-list policy-state prototype

Do not merge these rows into one broad allocator wave.
The current live implementation row is `VM-only page/free-list policy-state prototype`.
Later native allocator work stays reserved-only until a concrete backend-private
consumer appears.

Reading note: the `VM-only` wording is the historical row name for the first
policy/state prototype. It does not make VM the owner for later allocator
feature growth. Current EXE parity for the real apps rides the shared
typed-object / pure-first compiler seams, while native allocator fast-path work
still belongs to the substrate capability ladder.

### Handle policy

- policy owner:
  - reusable handle take order
  - fresh-handle issue policy
  - recycle policy
  - env mode selection (`NYASH_HOST_HANDLE_ALLOC_POLICY`)
- metal keep:
  - `SlotTable`
  - handle-to-slot storage
  - `RwLock` body
  - `drop_epoch`
  - public handle lookup/access API

### GC trigger policy

- policy owner:
  - `NYASH_GC_COLLECT_SP`
  - `NYASH_GC_COLLECT_ALLOC`
  - trigger decision
  - reason-bit classification
- metal keep:
  - `GcHooks` implementation
  - root snapshot
  - reachability trace
  - collection metrics accumulation
  - logging
  - mode dispatch (`RcCycle` / `Off`)

### VM-only page/free-list policy-state prototype

- policy/state owner:
  - fixed-size class selection used by the real-app allocator lane
  - page capacity policy for the VM smoke front
  - free-list reuse order
  - allocation/free accounting
  - peak usage and requested-byte counters
- metal keep:
  - actual alloc/free/realloc
  - native layout/ABI alignment
  - raw buffer ownership
  - OS VM page mapping
  - EXE lowering of general user-box `newbox`

### Reserved-only future rows

These remain docs/root-reserved only in this wave.

- `RawBuf` policy/state and native-layout-backed buffer ownership
- `MaybeInit`
- native `Layout`
- general size/bin policy
- reclaim/locality policy
- remote-free routing policy

## Stop Line

- This wave does **not** move live allocator backend code under `lang/src/hako_alloc/`.
- This wave does **not** claim direct EXE parity for allocator-shaped user boxes.
- This wave does **not** move `drop_epoch`, root snapshot, or reachability tracing into policy modules.
- This wave does **not** widen `FastLeafManifest`.
- This wave does **not** reopen `HostFacade/provider/plugin loader`.
- This wave does **not** add fixed-width numeric syntax or raw pointer syntax.
- This wave does **not** make `no_alloc` / `no_safepoint` backend-active.
- `LayoutBox` is narrow VM-only size-class policy. It is not native layout,
  alignment, or ABI ownership.

## Acceptance

First-row acceptance (`handle reuse policy`):

- `cargo test host_handle_alloc_policy_invalid_value_panics -- --nocapture`
- `cargo test host_reverse_call_map_slots -- --nocapture`
- `cargo test -p nyash_kernel cache_invalidates_on_drop_epoch_when_handle_is_reused -- --nocapture`
- `cargo test -p nyash_kernel invalid_handle_short_circuits_all_routes -- --nocapture`
- `cargo test -p nyash_kernel string_indexof_hh_cached_pair_route_roundtrip -- --nocapture`

Second-row acceptance (`GC trigger threshold policy`):

- `cargo test gc_trigger_policy_ -- --nocapture`
- `cargo test gc_controller_triggers_collection_on_safepoint_threshold -- --nocapture`
- `cargo test gc_controller_triggers_collection_on_alloc_threshold_after_safepoint -- --nocapture`
- `cargo test gc_controller_triggers_collection_on_both_thresholds -- --nocapture`
- `cargo test gc_controller_off_mode_ignores_trigger_thresholds -- --nocapture`
- `bash tools/checks/k2_wide_hako_alloc_gc_trigger_policy_guard.sh`

Third-row acceptance (`VM-only page/free-list policy-state prototype`):

- `tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight`
- `tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight`

Umbrella gate:

- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_runtime_v0_adapter_fixtures_vm.sh`
- `bash tools/checks/dev_gate.sh quick`

## Follow-Up

- `plugin route-manifest hardening` is landed
- `FastLeafManifest widen judgment` is landed with `no widen now`
- there is no active stage2 code bucket until a concrete backend-private consumer patch appears
- `RawBuf` policy/state, native `Layout`, and `MaybeInit` live migration is a
  later allocator wave; the narrow substrate allocation facade lives under
  `lang/src/runtime/substrate/raw_buf/`
- mimalloc-grade fast-path work follows `substrate-capability-ladder-ssot.md`
  and must name its numeric/layout/verifier gates first
- direct EXE parity waits for typed object planning for general user-box
  `newbox`
