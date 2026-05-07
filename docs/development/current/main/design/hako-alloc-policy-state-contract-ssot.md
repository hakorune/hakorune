---
Status: SSOT
Decision: provisional
Date: 2026-05-07
Scope: `stage2` allocator/handle wave and `phase-293x` real-app allocator port stop-lineとして、`hako_alloc` policy/state owner と native metal keep の concrete rows を固定する。
Related:
  - docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md
  - docs/development/current/main/design/stage2-aot-fast-lane-crossing-inventory.md
  - docs/development/current/main/design/stage2-selfhost-and-hako-alloc-ssot.md
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

## Current Split

| Layer | Current owner | Current first live rows |
| --- | --- | --- |
| policy/state owner | `hako_alloc` reading + narrow Rust policy helpers | handle reuse policy, GC trigger threshold policy, VM-only page/free-list policy-state prototype |
| capability substrate | `lang/src/runtime/substrate/{mem,buf,ptr,atomic,tls,gc}/**` | truthful capability seams only |
| native metal keep | Rust runtime/kernel + C ABI shims | host handle slot table, `drop_epoch`, GC root snapshot/reachability walk, actual alloc/free/realloc, TLS/atomic/GC body |

## First Concrete Policy Rows

Implementation order is fixed narrowly:

1. handle reuse policy
2. GC trigger threshold policy
3. VM-only page/free-list policy-state prototype

Do not merge these rows into one broad allocator wave.
The current live implementation row is `VM-only page/free-list policy-state prototype`.
Later native allocator work stays reserved-only until a concrete backend-private
consumer appears.

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

- `RawBuf`
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
- `RawBuf / native Layout / MaybeInit` live migration is a later allocator wave
- direct EXE parity waits for typed object planning for general user-box
  `newbox`
