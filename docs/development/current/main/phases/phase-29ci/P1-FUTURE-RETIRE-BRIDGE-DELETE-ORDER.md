---
Status: Accepted
Decision: accepted
Date: 2026-03-13
Scope: `phase-29ci` の `future-retire bridge` bucket を delete-order 目線で固定し、Rust-only の次 slice を outer caller reshape と分離する。
Related:
  - docs/development/current/main/phases/phase-29ci/README.md
  - docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md
  - CURRENT_TASK.md
  - src/runner/stage1_bridge/README.md
  - src/runner/stage1_bridge/program_json_entry/README.md
  - src/runner/stage1_bridge/program_json/README.md
---

# P1 Future-Retire Bridge Delete Order

## Goal

`src/runner/stage1_bridge/**` に残っている `Program(JSON v0)` bridge lane について、

- Rust-only で先に retire できる面
- outer caller / `.hako` / shell contract に触るので後ろへ回す面

を delete-order の SSOT として固定する。

## Exact Bucket Boundary

### Inner bridge cluster

- `src/runner/stage1_bridge/program_json_entry/mod.rs`
- `src/runner/stage1_bridge/program_json_entry/execute.rs`
- `src/runner/stage1_bridge/program_json_entry/request.rs`
- `src/runner/stage1_bridge/program_json/mod.rs`
- `src/runner/stage1_bridge/program_json/orchestrator.rs`
- `src/runner/stage1_bridge/program_json/read_input.rs`
- `src/runner/stage1_bridge/program_json/writeback.rs`

この cluster は `future-retire bridge` bucket の内側であり、次の delete/reduction slice はここだけを主語にしてよい。

### Must-Stay Thin Outer Callers

- `src/runner/mod.rs`
- `src/runner/emit.rs`

この 2 file は bridge bucket の outer caller contract であり、今の phase では reshape target にしない。
`emit-program-json-v0` request predicate と bridge-specific success/error formatting はすでに inner cluster 側へ戻っているので、outer caller は thin caller のまま keep する。

### Later Buckets

- `.hako` live/bootstrap callers
- shell helper keep
- diagnostics/probe keep

これらは Rust-only bridge delete slice と混ぜない。

## Fixed Delete Order

1. `program_json_entry/` と `program_json/` の inner cluster を thin-facade / owner-local policy へ寄せる
2. inner cluster の delete-order note を closeout-ready にする
3. outer caller (`src/runner/mod.rs`, `src/runner/emit.rs`) は thin contract のまま据え置く
4. `.hako` live/bootstrap callers と shell helper keep は別 bucket として audit する
5. caller inventory が空になるまで boundary 本体の delete は行わない

## Guardrails

- outer caller reshape を次 slice の目的にしない
- `.hako` / shell helper audit を inner bridge cleanup と同じ patch に混ぜない
- `phase-29ch` authority migration を reopen しない
- `MirBuilderBox.emit_from_source_v0(...)` を diagnostics/probe bucket へ落とさない

## Retreat Finding

- `future-retire bridge` はもう `entry cluster` と `program_json cluster` の 2 つに見えているので、次の Rust-only delete slice は outer caller へ広げる必要がない
- `program_json_entry/` はさらに `request` / `execute` / `exit` に分かれたので、request-build と request-local execution を同じ slice に戻さず、typed response handoff は `execute.rs` 側に閉じるのが安全
- `src/runner/mod.rs` と `src/runner/emit.rs` は thin caller として十分に縮退しており、ここを先に触ると delete-order ではなく runner root reshaping になる
- inner bridge cluster に残っているのは split debt ではなく exact contract leaf が中心で、delete-order blocker は主に `program_json/orchestrator.rs` に集約された owner-1 helper 依存と、bridge bucket の外側に残る `.hako` / shell caller 側にある
- したがって、次の reduction / delete slice は inner bridge cluster に限定し、外側の caller は `must-stay thin callers` として docs に固定したまま進める

## Immediate Next

1. inner bridge cluster だけで retire できる面をさらに棚卸しする
2. 削れない面は retreat finding として残す
3. そのあとで `.hako` / shell helper keep の delete-order audit に進む
