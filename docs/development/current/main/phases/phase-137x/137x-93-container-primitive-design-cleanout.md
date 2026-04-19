---
Status: Active
Date: 2026-04-19
Scope: owner-first optimization return の前に、array / map / primitive residence 設計の SSOT drift を閉じる docs-first gate。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-137x/README.md
  - docs/development/current/main/phases/phase-137x/137x-91-task-board.md
  - docs/development/current/main/design/primitive-family-and-user-box-fast-path-ssot.md
  - docs/development/current/main/phases/phase-163x/README.md
  - docs/development/current/main/phases/phase-289x/README.md
  - docs/development/current/main/phases/phase-289x/289x-94-container-demand-table.md
  - docs/development/current/main/phases/phase-289x/289x-96-demand-backed-cutover-inventory.md
---

# 137x-93 Container / Primitive Design Cleanout

## Goal

最適化へ戻る前に、array / map / primitive の設計読みをきれいに揃える。

この subphase は **perf implementation phase ではない**。
既存実装を広げず、SSOT / task board / stop-line を整理してから
owner-first optimization に戻るための design cleanout だよ。

## Phase Cut

phase-137x は今、この 3 段で読む。

1. **137x-A: string publication contract closeout**（closed）
   - string-only `publish.text` legality / provenance / idempotence を固定済み
2. **137x-B: container / primitive design cleanout**（current）
   - array typed-slot 実装状況と docs vocabulary を同期する
   - map demand bridge と typed map lane の境界を固定する
   - primitive residuals を blocking / non-blocking に分ける
3. **137x-C: owner-first optimization return**（after 137x-B）
   - active read-side owner proof に戻る
   - perf/asm で hot owner を再採取してから narrow seam だけを reopen する

## Active Cards

順序はこのまま固定する。

1. [x] `phase-pointer-resplit`
   - `137x-B` を design cleanout gate に変更する
   - owner-first perf return は `137x-C` に送る
2. [x] `array-typed-slot-truth-sync`
   - `InlineI64` / `InlineBool` / `InlineF64` の現行 runtime support と docs を同期する
   - `array.get` / encoded load の readback contract を過大表現しない
   - mixed / boxed / reflection routes の boxed promotion rule を明記する
   - status:
     - synced in `primitive-family-and-user-box-fast-path-ssot.md`
     - current truth is scalar immediate residence for `InlineI64` / `InlineBool` / `InlineF64`
     - only `InlineI64` has the direct `array_slot_load_encoded_i64` typed readback row
     - f64/bool readback stays under the existing encoded-any/public handle contract
3. [x] `map-demand-vs-typed-lane-boundary`
   - Map key decode / value store / value load demand metadata は landed と読む
   - typed map lane は未開放と明記する
   - map key policy と map value residence / publication を混ぜない
   - status:
     - locked in `phase-289x/289x-94-container-demand-table.md`
     - `289x-6d` / `289x-6e` are demand-metadata cuts, not typed-lane implementation
     - RuntimeData remains a facade that delegates demand ownership to Map rows
4. [ ] `primitive-residuals-classification`
   - `Null` / `Void` は conservative / low-priority residual として分類する
   - enum/sum/generic は別 design owner の partially-landed lane として分離する
   - primitive/user-box fast path と container residence pilot を同じ keeper 証拠として混ぜない
5. [ ] `container-identity-residence-contract`
   - Array / Map は public identity container のまま固定する
   - lane-host してよいのは internal element/key/value residence だけと明記する
   - public ABI / `.hako` syntax / array generics は開かない

## Exit Gate for 137x-B

137x-B は、最低でも次を満たしたら close できる。

- current pointers が `137x-B = design cleanout`, `137x-C = owner-first perf return` で揃っている
- array typed-slot docs が現行 runtime support と future-only items を分けている
- map docs が demand metadata と typed lane implementation を混同していない
- primitive residuals が perf blocker か later backlog かで分類されている
- phase-289x runtime-wide implementation は parked のままで、今回の cleanout から開かれない
- `tools/checks/dev_gate.sh quick` が green

## Stop-Line

この subphase から開いてはいけないもの:

- owner-first perf implementation
- runtime-wide `Value Lane Architecture` implementation
- typed map lane implementation
- heterogeneous / union array slot layout
- full `TextLane`
- `publish.any`
- public ABI widening
- allocator / arena work

## Relationship to Owner-First Perf

137x-B が閉じるまで、perf work は再開しない。

137x-C に入る最初の一手は、AGENTS の perf policy に従って
source reading ではなく current baseline / asm owner の再採取に戻す。
