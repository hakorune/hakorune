---
Status: SSOT
Scope: post-canonical retire lane（RCL-0..3, BoxCount/BoxShape split）
Decision: accepted
Updated: 2026-02-12
Related:
- docs/development/current/main/design/mir-canonical-callsite-lane-ssot.md
- docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md
- CURRENT_TASK.md
- src/mir/passes/callsite_canonicalize.rs
---

# MIR Callsite Retire Lane (SSOT)

## 目的

MCL-0..5 で固定した canonical callsite を起点に、legacy callsite 命令（`BoxCall`/`ExternCall`）を安全に retire する。

- 方針: `Rust 吸収 -> .hako mirbuilder 正規出力 -> strict/dev reject -> enum retire`
- 原則: fail-fast。silent fallback を入れない
- 分離: BoxCount（受理拡張）と BoxShape（責務整理）を混ぜない

## 前提（RCL 開始条件）

- MCL-0..5 が完了している
- runtime lane で `D6-min44` まで完了し、日次 gate が green
- backend 入口で legacy callsite 残存は freeze される

## タスク順序（固定）

### RCL-0 (docs-only) [done]

- retire lane の順序、契約、受け入れ基準を SSOT 化する（この文書）

### RCL-1 (BoxCount) [done]

- `.hako` mirbuilder の emit を canonical callsite へ統一する
  - method call: `Call(callee=Method)`
  - extern call: `Call(callee=Extern)`
- legacy emit (`BoxCall`/`ExternCall`) の新規発行を止める
  - 実装境界: `lang/src/mir/builder/**` の emit は `op=call` + `callee` へ統一、`src/runner/mir_json_v0.rs` は `call.callee` を直接受理

受け入れ:
- `.hako` mirbuilder 由来の新規 MIR で `BoxCall`/`ExternCall` が増えない
- fixture + fast gate で 1受理形ずつ pin

### RCL-2 (BoxShape) [done]

- strict/dev で legacy emit を fail-fast reject する
- reject tag を固定し、診断距離を短くする
- 実装境界: `src/runner/modes/common_util/selfhost/json.rs` の stage1 selfhost MIR(JSON v0) parse preflight で `op=boxcall/externcall` を reject

受け入れ:
- strict/dev で legacy emit が 1件でもあれば freeze
- release 既定挙動は変えない（必要なら既定OFFトグル）

### RCL-3 (BoxShape) [done]

- `MirInstruction` から `BoxCall`/`ExternCall` を retire
- parser/mirbuilder/runtime の移行完了を確認してから削除
- 進捗（RCL-3-min1/min2/min3）:
  - `src/mir/ssot/extern_call.rs` の emit を canonical 化し、新規 extern call 構築は `Call(callee=Extern)` に統一
  - builder emit 境界で legacy `BoxCall` を canonical `Call(callee=Method)` へ統一し、新規 `BoxCall` 構築を停止
  - `MirInstruction` から `BoxCall`/`ExternCall` を削除し、backend/contracts/tests/docs を同期

受け入れ:
- enum/loader/backend に legacy callsite 経路が残らない
- instruction ledger と docs/reference が同期している

## fail-fast 契約

- backend 入口: canonical 以外の callsite は freeze
- strict/dev: legacy emit 検出時に freeze
- log: 1行・安定タグ（`[freeze:contract][callsite-retire:*]` 系）

## 非目標

- `NewClosure` 統合
- `DebugLog` / `Nop` retire
- runtime lane（vm-hako subset）の語彙拡張

## 受け入れコマンド（最小）

```bash
cargo test instruction_diet_ledger_counts_match_docs_ssot -- --nocapture
cargo test mir14_shape_is_fixed --test mir_instruction_set_sync -- --nocapture
cargo check --bin hakorune
./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4
```
