---
Status: SSOT
Decision: provisional
Date: 2026-03-23
Scope: `phase-29ct` の C6 として、`.hako owner` と `native metal keep` の最終線引きを detailed docs lock として固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/rune-and-stage2plus-final-shape-ssot.md
  - docs/development/current/main/phases/phase-29ct/README.md
  - docs/development/current/main/design/substrate-capability-ladder-ssot.md
  - docs/development/current/main/design/gc-tls-atomic-capability-ssot.md
  - docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md
  - docs/development/current/main/design/collection-raw-substrate-contract-ssot.md
  - lang/src/runtime/substrate/README.md
---

# Final Metal Split (SSOT)

## Goal

- `.hako owner` と `native metal keep` の最終線引きを、実装前に detailed docs lock として固定する。
- `C5 Hakozuna portability layer` は future ladder item のまま残し、今回の current detail lock には含めない。
- allocator/runtime policy owner を `.hako` に寄せる終点と、native に残す metal service の境界を曖昧にしない。

## Reading

- この slice は docs-only。
- physical staging root はまだ増やさない。
- `C5` は deferred future item として fixed order 上に残すが、current implementation target ではない。

## Final Split Table

| Responsibility | `.hako owner` | `native metal keep` | Not yet moved now |
| --- | --- | --- | --- |
| allocator control | allocator state machine | final allocator backend call | actual `.hako` body |
| size/bin policy | size-class policy, bin policy, ptr/bin routing | platform allocator layout quirks | live migration |
| free/reclaim path | remote-free routing, queue policy, reclaim heuristics | final free backend call | queue implementation body |
| locality/cache | TLS cache policy | platform TLS fallback | live TLS substrate body |
| concurrency | atomic/TLS/GC policy usage | platform atomics fallback, final GC integration hooks | live capability body |
| observability | telemetry/profile policy | final ABI/export stubs for host integration | runtime metrics implementation |
| virtual memory | none | OS VM syscall glue | `hako.osvm` implementation body |

## Litmus

### Put It In `.hako`

- policy
- heuristics
- routing
- reclaim decisions
- size/bin classification
- telemetry/profile decisions

### Keep It Native

- syscall glue
- final allocator backend entry
- final ABI entry stub
- platform TLS fallback
- platform atomics fallback
- final GC integration hook

## Fail-Fast Reading

- do not describe the lane as `fully metal split` until allocator state machine and policy owner actually move.
- do not treat `atomic/tls/gc` capability lock as equivalent to final metal split.
- do not reopen perf based on naming/docs alone.
- do not treat Rune metadata as a substitute for moving metal responsibilities.

## Non-Goals

- new physical staging roots under `runtime/substrate/`
- `.hako` implementation body for allocator/runtime metal
- OS VM rewrite
- final allocator backend rewrite
- final ABI rewrite
- `C5` portability-layer implementation
- perf lane reopen

## Follow-Up

After this detail lock:

1. `C5 Hakozuna portability layer` stays ladder-only and deferred
2. implementation planning can choose one narrow owner-shift slice without re-deciding the final boundary
3. current first landed policy/body splits are:
   - handle reuse policy vs host handle slot-table body
   - GC trigger threshold policy vs root-trace/metrics body
4. `plugin route-manifest hardening` is landed; next exact bucket is `FastLeafManifest widen judgment`
