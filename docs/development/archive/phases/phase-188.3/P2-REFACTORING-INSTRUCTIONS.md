# Phase 188.3 P2: Pattern6 merge/latch 周りのリファクタ指示書（意味論不変）

**Date**: 2025-12-27  
**Scope**: JoinIR merge の tail-call 分類・latch 記録の可読性/SSOT化  
**Non-goals**: Pattern6 の選定/Lowering の機能追加、既定挙動変更、フォールバック追加

---

## 目的

Pattern6（NestedLoopMinimal）で露出した merge/rewriter の暗黙ルールを「構造」で固定し、次の拡張（Phase 188.4+）や回帰検知を楽にする。

---

## SSOT（固定する契約）

- `latch_incoming` を記録してよいのは `TailCallKind::BackEdge` のみ（LoopEntry は上書き禁止）
- entry-like は “JoinIR main の entry block のみ”（`loop_step` の entry block を entry-like と誤認しない）
- latch 二重設定は `debug_assert!` で fail-fast（回帰検知）

実装の現行入口:
- `src/mir/builder/control_flow/joinir/merge/instruction_rewriter.rs`
- `src/mir/builder/control_flow/joinir/merge/loop_header_phi_info.rs`

---

## リファクタ方針（構造で解決）

### A) tail-call 分類を「1箇所」で作って再利用する

現状は plan stage の中で `classify_tail_call(...)` を複数回呼び出している。
`TailCallFacts`（struct）を作って、各 block の tail-call に対して以下を 1 回だけ確定し、後続が参照する形にする:

- `target_func_name`
- `is_target_continuation`
- `is_target_loop_entry`
- `is_entry_like_block`（MAIN + entry block）
- `tail_call_kind`

### B) latch 記録を “専用の小箱” に隔離する

`instruction_rewriter.rs` の末尾にある “latch 記録” を以下の形で隔離し、責務を固定する:

- `LatchIncomingRecorder`（new module）を追加し、`record_if_backedge(...)` だけを公開
  - 引数: `tail_call_kind`, `boundary`, `new_block_id`, `args`, `loop_header_phi_info`
  - 返り値: `()`（失敗は `debug_assert!` / 既存の `Result` に委譲）

これで「BackEdge 以外で記録しない」契約が局所化され、将来の変更点が明確になる。

### C) “entry-like” 判定を SSOT helper に寄せる

`func_name == MAIN && old_block_id == entry_block` の判定を helper 化する:

- `is_entry_like_block(func_name, old_block_id, func_entry_block) -> bool`

判定の重複と誤用（loop_step の entry を entry-like と見なす）を構造で防ぐ。

---

## テスト（仕様固定）

最小で良いので、契約をテストで固定する（既存のユニットテストがある範囲で）。

- `LoopHeaderPhiInfo::set_latch_incoming()` の二重セットが debug で fail-fast すること（`#[should_panic]`）
- Pattern6 fixture は既にあるので、スモークで RC を固定する（quick の回帰検知）

---

## 検証手順

1. `cargo build --release`
2. `./target/release/hakorune --backend vm apps/tests/phase1883_nested_minimal.hako`（RC=9）
3. `./tools/smokes/v2/run.sh --profile quick`（154/154 PASS）

