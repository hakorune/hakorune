# Phase 287 P4: `rewriter/stages/plan.rs` 分割指示書（意味論不変）

**Date**: 2025-12-27  
**Status**: Completed ✅  
**Scope**: `src/mir/builder/control_flow/joinir/merge/rewriter/stages/plan.rs`（~741行）を “責務単位” に分割して読みやすくし、`plan.rs` は facade（入口 + 組み立て）へ縮退する。  
**Non-goals**: 仕様変更、エラータグ/ヒント文の変更、検出条件の追加/緩和、silent fallback 追加、恒常ログ増加、`merge/instruction_rewriter.rs` の再分割

---

## 前提（P3 完了）

- P3 で `instruction_rewriter.rs` は facade 化され、pipeline 関数は `rewriter/stages/{scan,plan,apply}.rs` に物理分割済み。
- P4 は Stage 2（Plan）の “中身” を構造で分割するフェーズ。

---

## 目的（SSOT）

- Plan stage の責務（entry 解決 / block map / instruction rewrite / tail call / terminator / carrier_inputs）を **ファイル構造**で見える化する。
- “どの判断がどこにあるか” を迷わず追えるようにし、将来のバグ修正を **局所化**する。
- 意味論不変（copy/move のみ）を守り、Fail-Fast の契約を崩さない。

---

## 現状（問題点）

`rewriter/stages/plan.rs` が長く、以下が同居している:

- entry function 解決（boundary SSOT + fallback heuristic）
- per-function / per-block の main loop
- instruction filtering / remap
- tail call detection + param binding + latch incoming
- terminator remap（jump/branch/return → jump など）
- carrier_inputs 収集（exit jump / skippable continuation）

この同居は “後続で if が増殖する” 典型パターンなので、P4 で責務分離して予防する。

---

## 目標の構造（案）

`plan.rs` は orchestrator、ロジックは `plan/` 配下へ。

```
src/mir/builder/control_flow/joinir/merge/rewriter/stages/
├── plan.rs                         # facade（入口 + 部品の呼び出し）
└── plan/                           # NEW（1 file = 1 responsibility）
    ├── mod.rs
    ├── entry_resolver.rs           # entry_func_name 解決（boundary SSOT）
    ├── local_block_map.rs          # build_local_block_map の使い方を固定
    ├── instruction_rewrite.rs      # inst の skip/remap + branch/phi block remap
    ├── tail_call_rewrite.rs        # tail call 検出→binding→jump 生成
    ├── terminator_rewrite.rs       # terminator の remap（jump/branch/return）
    └── carrier_inputs.rs           # ExitJump(skippable) の carrier_inputs 収集
```

ルール:
- **“移動だけ”** を徹底する（関数分割はするが、条件式・順序は変えない）。
- helper が増えたら `plan/mod.rs` の下に閉じ込める（stages の外へ漏らさない）。

---

## 手順（安全な順序）

### Step 1: `plan/` を追加（空の module）

- `rewriter/stages/plan/mod.rs` を追加し、各 submodule を `pub(super) mod ...;` で宣言する。
- まずは `plan.rs` から `mod plan;` を参照できる状態にする（実体は空でも良い）。

### Step 2: entry 解決を抽出（純粋で安全）

- `entry_func_name` の算出を `entry_resolver.rs` へ移動する。
- 重要: `JoinInlineBoundary.loop_header_func_name` を優先する SSOT を維持。

### Step 3: “instruction rewrite loop” を抽出

- inst 単位の処理を `instruction_rewrite.rs` に移す（skip/remap/branch/phi remap）。
- 既存の `InstructionFilterBox` / `ReturnConverterBox` の呼び出しは維持（置き換えない）。

### Step 4: tail call の rewrite を抽出（最も注意）

- tail call の検出・分類・param binding・latch incoming を `tail_call_rewrite.rs` へ移す。
- 重要: 既存の `classify_tail_call(...)` の引数計算順序（entry-like 判定含む）を変えない。
- 重要: `CarrierInputsCollector` の呼び出し箇所を変えない（ExitJump(skippable) のみ）。

### Step 5: terminator rewrite を抽出

- `terminator` の remap と `ReturnConverterBox` の適用を `terminator_rewrite.rs` へ移す。
- “Return → Jump” の規約（skippable continuation を除く）を変えない。

### Step 6: `plan.rs` を facade へ縮退

- `plan_rewrites(...)` の中身は “部品を呼んで順番に組み立てるだけ” にする。
- `resolve_target_func_name` / `is_joinir_main_entry_block` のようなローカル helper は、責務に応じた module 側へ移す。

---

## テスト（仕様固定）

新規テストは原則不要（意味論不変）。

ただし 1 つだけ足すなら “境界” の unit test（任意）:
- `entry_resolver::resolve_entry_func_name(...)` が boundary を優先し、`MAIN` を除外すること。

---

## 検証手順（受け入れ基準）

```bash
cargo build --release
./target/release/hakorune --backend vm apps/tests/phase1883_nested_minimal.hako   # RC=9
./tools/smokes/v2/run.sh --profile quick
```

受け入れ:
- Build: 0 errors（warnings は増やさない努力はするが 0 を要求しない）
- quick: 154/154 PASS
- Pattern6: RC=9 維持
- 恒常ログ増加なし

---

## Out of Scope（重要）

- “exit_collection へ統合する” などの仕様寄り整理（Phase 分けする）
- contract_checks の新規契約追加（P4 ではやらない）
- plan stage の最適化（性能改善）や algorithm 変更
