---
Status: SSOT
Date: 2026-04-18
Scope: perf/asm を使う最適化レーンで、`front split` / `owner/state transition` / `keeper-revert stop-line` を先に固定する運用。
Related:
  - AGENTS.md
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/DOCS_LAYOUT.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md
  - docs/development/current/main/design/string-hot-corridor-runtime-carrier-ssot.md
  - docs/development/current/main/phases/phase-137x/README.md
---

# Perf Owner-First Optimization SSOT

## Goal

この文書の目的は 1 つだけ。

- `perf` / `asm` を見ているのに owner を見誤って一週間失う、という事故を防ぐ

最適化レーンでは `hot function` より先に、`どの front のどの state transition が owner か` を固定する。

## First Principle

- `perf` は「どこで燃えているか」を教える
- keeper の判断に必要なのは「どの owner / state transition を切るべきか」だよ
- したがって、`helper 名` や `wrapper symbol` をそのまま owner と読まない

読む順序は固定する。

1. front を split する
2. failure mode を split する
3. owner/state transition を 1 行で固定する
4. その owner の直上 1 seam だけを触る

## Front Split

最適化レーンは、最低でも次の front を分けて読む。

### exact

- smallest exact front
- 目的:
  - hot helper body の instruction/work tax を測る
- 典型:
  - pure helper path
  - top-level lowering shape と helper entry tax の確認

### meso

- exact と whole の中間 front
- 目的:
  - same corridor のまま row scan / app noise を外して work explosion を確認する
- 典型:
  - `substring + concat + array.set + loopcarry`

### whole

- user-visible whole app front
- 目的:
  - stall / sync / publish / object-world cost を確認する
- 典型:
  - producer kind × stage × bytes が載る front

Rule:

- exact / meso / whole が割れたら、どれか 1 つを都合よく真実扱いしない
- `whole` owner を keeper judgement の tie-breaker に使う場合でも、`meso` を contradiction guard に残す

## Owner Card

code edit の前に、最低でも次の 6 列を 1 枚に固定する。

| field | meaning |
| --- | --- |
| front | exact / meso / whole のどれか |
| failure mode | work explosion / stall collapse / mixed |
| current owner | current broad owner family |
| hot transition | どの state からどの state に変わる所が熱いか |
| next seam | 次に触る 1 seam |
| reject seam | 今は触らない seam |

`current owner` は関数名ではなく state transition で書く。

良い例:

- `OwnedText -> PublishedPublic`
- `BorrowedSource -> OwnedText`
- `source proof lookup -> store retarget`

悪い例:

- `concat_hs`
- `issue_fresh_handle`
- `Registry::alloc`

これらは symbol であって owner ではない。

## Failure-Mode Reading

cycle / instruction / IPC の読みは先に固定する。

### work explosion

次の形は `stall` より `余計な仕事` を疑う。

- IPC が C と近い
- instructions と cycles が同じ方向に大きく膨らむ

読むべきこと:

- same corridor で何回 birth / freeze / objectize / publish しているか
- helper 境界を跨ぐたびに state が public 側へ戻っていないか

### stall collapse

次の形は `仕事量` より `詰まり` を疑う。

- instructions はそこまで増えていない
- cycles が大きく増える
- IPC が崩れる

読むべきこと:

- publication/object-world entry
- sync / refcount / registry / cache miss
- TLS / indirect / vtable / lock instructions

### zero-boundary rule

境界 counter が 0 のときは、その seam を active owner と読まない。

例:

- `publish_boundary.slot_* = 0`
- `publish_reason.need_stable_object = 0`

この状態では `KernelTextSlot` exit は blind spot ではなく、inactive seam だよ。

## Probe Contract

1 仮説 = 1 owner family = 1 seam = 1 commit で進める。

probe 前に、keeper / revert 条件を先に書く。

最低限必要な列:

| field | example |
| --- | --- |
| target seam | `const_suffix publish sink` |
| expected win | `whole publish stall down` |
| primary guard | `whole ms down` |
| contradiction guard | `meso must not regress` |
| counter proof | target site counter must shrink |
| revert condition | exact/meso regress or counter unchanged |

recommended threshold:

- meso:
  - `>= 15%` improvement か、少なくとも no-regression
- whole:
  - `>= 10%` improvement を要求
- counter:
  - target site / target transition が期待通りに減ること

Rule:

- cycles だけの improvement で keeper にしない
- whole だけの微小 win でも、meso が壊れたら keeper にしない

## Stop-the-Line

同じ owner family で non-keeper が 2 回出たら、code edit を止める。

その時点で必ずやること:

1. `CURRENT_TASK.md`
   - current owner
   - rejected seam
   - next seam
2. `10-Now.md`
   - thin mirror
3. active phase README
   - evidence と rejected card
4. 必要なら design consult packet

止める条件:

- same owner family で 2 probes fail
- active boundary counter が 0 の seam をまだ触ろうとしている
- helper 名だけで owner を説明している

## Doc Placement

### `AGENTS.md`

- 普遍ルールだけ
- この文書へのリンクを置く
- lane 固有の owner や numbers は置かない

### `CURRENT_TASK.md`

- current owner
- current front split
- next seam
- rejected seam の短い要約

### `10-Now.md`

- one-screen mirror
- summary + pointer only

### active phase README

- exact/meso/whole evidence
- rejected probes
- keeper/revert judgment history

## Current Lesson Lock

current string lane から固定する lesson はこれだよ。

1. `hot symbol` を見ても `owner` は分からない
2. `exact / meso / whole` は最初に split する
3. `slot boundary counter = 0` なら slot exit を触らない
4. `meso` と `whole` は failure mode が違う
5. `revert` が 2 本出たら、次は code ではなく owner map を更新する

## Quick Re-entry

最適化レーン再開時の最小読み順:

1. `CURRENT_TASK.md`
2. `docs/development/current/main/10-Now.md`
3. active phase README
4. this document
5. lane-specific design SSOT
