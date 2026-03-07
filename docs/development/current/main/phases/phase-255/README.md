Status: Completed
Scope: Phase 255 (Pattern 6 multi-param loop wiring/PHI 対応)
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/phases/phase-254/README.md

# Phase 255: Multi-param loop の boundary/PHI/wiring を SSOT 化する

## 前フェーズ（Phase 254）からの引き継ぎ

Phase 254 で Pattern 6 (ScanWithInit) の実装が完了したが、integration テストで失敗：

```
[ERROR] ❌ [rust-vm] VM error: Invalid value:
[rust-vm] use of undefined value ValueId(10)
(fn=StringUtils.index_of/2, last_block=Some(BasicBlockId(4)),
 last_inst=Some(Compare { dst: ValueId(14), op: Ge, lhs: ValueId(10), rhs: ValueId(13) }))
```

## 根本原因

**JoinIR→MIR merge/boundary システムが複数ループ変数を想定していない**

### 現状の仕様（Pattern 1-5）

- 単一ループ変数前提（例: `i` のみ）
- `with_loop_var_name()` が1つの変数名を受け取る
- `exit_bindings` が単一 carrier を想定

### Pattern 6 の要求（3変数ループ）

```nyash
index_of(s, ch) {
    local i = 0
    loop(i < s.length()) {
        if s.substring(i, i + 1) == ch { return i }
        i = i + 1
    }
    return -1
}
```

必要な変数:
- `s`: haystack（ループ不変）
- `ch`: needle（ループ不変）
- `i`: loop index（ループ状態）

### 問題の詳細

1. **PHI ノード作成の不足**:
   ```rust
   // 期待: 3つの PHI ノード
   %4 = phi [%0, entry], [%4, loop]  // s
   %5 = phi [%1, entry], [%5, loop]  // ch
   %6 = phi [%2, entry], [%6, loop]  // i

   // 実際: 1つだけ作られる
   %4 = phi [%0, entry], [%4, loop]  // s だけ
   // %5 と %6 が undefined → 実行時エラー！
   ```

2. **影響箇所**:
   - `JoinInlineBoundaryBuilder::with_loop_var_name()` - 単一変数想定
   - `LoopExitBinding` の使い方 - exit_bindings に複数指定しても PHI が作られない
   - `JoinIRConversionPipeline` の merge 処理 - 複数 PHI 作成に未対応

## 解決方針（SSOT 化）

### 方針: LoopState と Invariants を分けて wiring

**LoopState（変化する変数）**:
- `i`: ループカウンタ
- 毎イテレーションで更新される
- PHI ノードが必要
- exit_binding で final value を取得

**Invariants（不変な変数）**:
- `s`, `ch`: ループ内で参照するが変化しない
- PHI ノードは必要だが、すべてのイテレーションで同じ値
- `phi [init, init, init, ...]` の形になる

### 実装戦略

#### Option A（推奨）: Boundary に invariants フィールド追加

```rust
pub struct JoinInlineBoundary {
    pub join_inputs: Vec<ValueId>,  // 既存（main の params）
    pub host_inputs: Vec<ValueId>,  // 既存（host 側の ValueId）

    // 新規追加
    pub loop_invariants: Vec<(String, ValueId)>,  // (変数名, host ValueId)

    pub exit_bindings: Vec<LoopExitBinding>,  // 既存（LoopState 用）
    pub loop_var_name: Option<String>,        // 既存（単一変数用、廃止予定）
}
```

**利点**:
- 既存の Pattern 1-5 に影響なし
- invariants の扱いを明示的に分離
- PHI 生成ロジックを追加しやすい

#### Option B: exit_bindings を拡張

```rust
pub enum CarrierRole {
    LoopState,       // 既存: 変化する変数
    ConditionOnly,   // 既存: 条件のみ
    LoopInvariant,   // 新規: ループ不変
}
```

**問題点**:
- `CarrierRole::LoopInvariant` を追加しても、現状の merge ロジックが対応していない
- exit_bindings は "exit value" を想定しているが、invariants は "同じ値を保持" という意味合いが異なる

### 推奨: Option A

- invariants を明示的に分離
- merge ロジックに invariants 専用の PHI 生成を追加

## 実装タスク（P0）

### Task 1: Boundary 構造拡張

**ファイル**: `src/mir/join_ir/lowering/inline_boundary.rs`

1. `JoinInlineBoundary` に `loop_invariants` フィールド追加
2. `JoinInlineBoundaryBuilder::with_loop_invariants()` メソッド追加

### Task 2: PHI 生成ロジック拡張

**ファイル**: `src/mir/join_ir/lowering/inline_boundary.rs` または merge 関連

1. `loop_invariants` から PHI ノードを生成
2. すべてのブロックで同じ値を持つ PHI として作成
3. variable_map に登録

### Task 3: scan_with_init route（historical Pattern6 label）の boundary 構築を修正

**current route files**:
- `src/mir/join_ir/lowering/scan_with_init_minimal.rs`
- `src/mir/builder/control_flow/plan/facts/loop_scan_with_init.rs`

**historical path token**: `src/mir/builder/control_flow/joinir/patterns/pattern6_scan_with_init.rs`

1. `with_loop_invariants()` を使って s, ch を登録
2. `exit_bindings` は i のみ（LoopState）

```rust
let boundary = JoinInlineBoundaryBuilder::new()
    .with_inputs(
        vec![s_param, ch_param, i_param],
        vec![s_host, ch_host, i_host],
    )
    .with_loop_invariants(vec![
        (parts.haystack.clone(), s_host),
        (parts.needle.clone(), ch_host),
    ])
    .with_exit_bindings(vec![i_exit_binding])
    .with_loop_var_name(Some(parts.loop_var.clone()))
    .build();
```

### Task 4: Unit Tests

**ファイル**: `src/mir/join_ir/lowering/inline_boundary.rs` (tests module)

1. invariants を含む boundary 作成テスト
2. PHI ノードが正しく生成されることを確認

### Task 5: Integration Tests

**実行**:
```bash
HAKORUNE_BIN=./target/release/hakorune bash tools/smokes/v2/profiles/integration/apps/archive/phase254_p0_index_of_vm.sh
HAKORUNE_BIN=./target/release/hakorune bash tools/smokes/v2/profiles/integration/apps/archive/phase254_p0_index_of_llvm_exe.sh
```

**期待**: 両方 PASS（exit code 1）

## 禁止事項

- ❌ workaround / by-name 分岐 / ハック禁止
- ❌ Pattern 1-5 の動作を変更しない（regression 禁止）
- ❌ フォールバック処理の追加禁止（Fail-Fast 原則維持）

## 受け入れ基準

- ✅ phase254_p0_index_of_vm.sh が PASS
- ✅ phase254_p0_index_of_llvm_exe.sh が PASS
- ✅ 最終的に `--profile quick` の最初の FAIL が次へ進む（index_of で freeze しない）
- ✅ Pattern 1-5 の既存テストがすべて PASS（regression なし）

## 進捗（P0）

- Task 1: Boundary 構造拡張: 未着手
- Task 2: PHI 生成ロジック拡張: 未着手
- Task 3: Pattern 6 boundary 修正: 未着手
- Task 4: Unit Tests: 未着手
- Task 5: Integration Tests: 未着手
