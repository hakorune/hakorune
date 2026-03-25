---
Status: Historical
Decision: adopted-into-ssot
Date: 2026-03-25
Scope: `stage2` hakorune AOT/native thin-path について外部相談で得た結論と、その採用結果を記録する。
Related:
  - docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md
  - docs/development/current/main/design/stage2-selfhost-and-hako-alloc-ssot.md
  - docs/development/current/main/design/collection-raw-substrate-contract-ssot.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md
  - docs/development/current/main/design/substrate-capability-ladder-ssot.md
  - docs/development/current/main/design/final-metal-split-ssot.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - lang/src/runtime/collections/README.md
  - lang/src/runtime/host/host_facade_box.hako
  - lang/src/hako_alloc/README.md
  - src/backend/mir_interpreter/handlers/extern_provider.rs
  - src/runtime/host_api.rs
---

# Historical Consultation Record: Stage2 AOT-Native Thin Path / Hakozuna Boundary

## Purpose

- この文書は external consultation で使った prompt と、その回答から採用した結論を記録する historical record だよ。
- current design truth は [`stage2-aot-native-thin-path-design-note.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md) を正本にする。

## Prompt Summary

- 相談時点の問いは次だった。
  - `AOT/native` を性能本線にしたい
  - `rust-vm` は reference/debug/parity lane に留めたい
  - `.hako semantics -> substrate -> native` の source layering はすでにかなり揃っている
  - `hako_alloc/hakozuna` は policy/state owner に育てたい
  - public canonical ABI は増やしたくない
  - それでも hot path の call overhead を極小化したい

## External Answer Summary

external answer の要点は次。

1. source layering は作り直さず、そのまま残す
2. collapse すべきなのは execution layering だけ
3. public canonical ABI は 2 面のまま維持する
   - `Core C ABI`
   - `TypeBox ABI v2`
4. AOT 用の backend-private fast lane は許可する
5. hot scalar collection op は monomorphic direct fast entry を主線にする
6. `slot/probe/reserve/grow` は compile-time seam であって、runtime generic dispatcher ではない
7. batched interface は bulk-only に限定する
8. `HostFacade / extern_provider / plugin loader` は cold dynamic lane に隔離する
9. `hako_alloc/hakozuna` は policy/state owner、native は final allocator/TLS/atomics/GC/ABI metal を持つ
10. AOT では lowering が早く route を決め、runtime fallback は cold/debug lane に限定する

## Adopted Conclusions

この consultation から current SSOT に採用した結論は次。

### Adopted as current direction

- source layering stays
- execution layering collapses in `AOT/native` only
- no third public ABI
- backend-private fast lane is allowed
- `hako_alloc/hakozuna` stays policy/state-side
- native keeps the final metal body
- `HostFacade/provider/plugin loader` are cold-path owners
- perf ladder must separate bridge/allocation/semantic-owner/dynamic-fallback cost

### Not yet fixed in detail

- exact schema/name of the backend-private fast leaf manifest
- exact crossing inventory rows for collection/allocator/dynamic fallback
- exact microbench command pack for the new fast-lane proof

## Usage Rule

- この文書は historical evidence であって、current implementation order や current SSOT を持たない。
- 今後この topic を読むときは、まず [`stage2-aot-native-thin-path-design-note.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md) を開く。
