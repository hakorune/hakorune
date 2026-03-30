---
Status: SSOT
Decision: provisional
Date: 2026-03-30
Scope: stage2+ perf 再開前の collection substrate cleanup を、stage 軸と owner/substrate 軸を混ぜずに固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/de-rust-stage-and-owner-axis-ssot.md
  - docs/development/current/main/design/collection-raw-substrate-contract-ssot.md
  - docs/development/current/main/phases/phase-29cm/README.md
  - docs/development/current/main/design/stage2plus-entry-and-first-optimization-wave-task-pack-ssot.md
---

# Stage2 Collection Substrate Cleanup (SSOT)

## Purpose

- `Array/Map body を .hako に書く` を current truth に沿って読み直す。
- visible semantics owner と raw substrate residue を分離して、perf 前に何を片付けるかを 1 枚で固定する。
- `stage0/stage1/stage2+` の stage 軸を collection 実務 detail と混線させない。

## Fixed Reading

- `.hako` visible owner はすでに `ArrayCoreBox` / `MapCoreBox` / `RuntimeDataCoreBox` 側にある。
- 今回の cleanup 対象は「semantic body 移植」ではなく、daily path に残る Rust/native method-shaped residue の demote だよ。
- `stage0` は bootstrap/recovery keep、`stage1` は bridge/proof、`stage2+` は mainline target のまま据え置く。
- collection cleanup の結果として変えてよいのは owner/substrate 境界だけで、stage 軸の役割は変えない。

## Current Cleanup Contract

- Array daily path:
  - `get -> nyash.array.slot_load_hi`
  - `set -> nyash.array.slot_store_hih / nyash.array.slot_store_hii`
  - `push -> nyash.array.slot_append_hh`
  - `has -> nyash.runtime_data.has_hh`
- Map daily path:
  - `get -> nyash.map.slot_load_hh`
  - `set -> nyash.map.slot_store_hhh`
  - `has -> nyash.map.probe_hh`
  - `size -> nyash.map.entry_count_i64`
- compat-only residue:
  - `nyash.array.set_hih` / `nyash.array.set_hii`
  - `nyash.array.get_hi` / `nyash.array.has_hi` / `nyash.array.push_h*`
  - `nyash.map.entry_count_h` / `nyash.map.size_h` / legacy `nyash.map.{get,set,has}_*`

## Next Gate

- perf reopen は collection cleanup green の後にだけ開く。
- first perf wave は既存どおり `route/perf only`。
- Rune backend-active 化、ABI 拡張、broad optimizer はこの lane に入れない。
