# Phase 212: if-sum ミニ実装 & 実行フェーズ - 観測レポート

**Phase**: 212
**Date**: 2025-12-09
**Status**: ⚠️ **BLOCKED** - AST→MIR 変換層の制約発見
**Prerequisite**: Phase 211 完了（設計フェーズ）

---

## 🎯 Phase 212 の目的

Phase 211 で設計した「if-sum パターン」（ループ内 if での条件付き更新）を、既存 JoinIR インフラ（P1+P3+multi-carrier）だけで実際に動かす。

**戦略**: Fail-Fast - 問題が出たら「どこで止まったか」を記録するところまでに留める。

---

## Task 212-1: .hako テスト関数の追加 ✅

**ファイル**: `apps/tests/phase212_if_sum_min.hako`

**初期実装**:
```hako
static box IfSumTest {
    sum_def_count(defs) {
        local sum = 0
        local i = 0
        local len = 3

        loop(i < len) {
            if i > 0 {
                sum = sum + 1  // ← 条件付き更新
            }
            i = i + 1
        }
        return sum
    }

    main() {
        local result = IfSumTest.sum_def_count(0)
        return result
    }
}
```

**期待結果**: RC=2 (i=1, i=2 で sum が increment されるため)

---

## Task 212-2: ルーティング条件の確認 ✅

**確認内容**:
- `loop_pattern_router.rs` (Phase 194) は構造ベースで Pattern 1-4 を自動分類
- `loop_pattern_detection::classify()` が CFG 構造から Pattern 判定
- → 名前ベースの whitelist は不要（既存ロジックで対応可能）

**結論**: 構造ベースルーティングで自動的に Pattern が選ばれるはず

---

## Task 212-3: JoinIR 経路で E2E 実行 ⚠️

### 実行コマンド

```bash
NYASH_JOINIR_CORE=1 ./target/release/hakorune apps/tests/phase212_if_sum_min.hako
```

### 実行結果

```
[joinir/pattern1] Generated JoinIR for Simple While Pattern
[joinir/pattern1] Functions: main, loop_step, k_exit
...
[DEBUG-177] Phase 33-21: carrier_phis count: 1, names: ["i"]
...
RC: 0
```

**期待**: RC=2
**実際**: RC=0

### Pattern ルーティング観測

- **選ばれた Pattern**: **Pattern 1 (Simple While)**
- **Carrier 数**: **1 つのみ** (`i`)
- **欠落している Carrier**: `sum` が carrier として認識されていない

### MIR ダンプ分析

```bash
./target/release/hakorune --dump-mir apps/tests/phase212_if_sum_min.hako
```

**MIR 出力** (`IfSumTest.sum_def_count/1`):

```mir
define i64 @IfSumTest.sum_def_count/1(? %0) effects(read) {
bb1:
    %2 = const 0    ; ← sum 初期化
    %4 = const 0    ; ← i 初期化
    br label bb3

bb2:
    ret %2          ; ← return sum

bb3:
    %7 = phi [%4, bb1], [%16, bb6]  ; ← i の PHI のみ
    br label bb4

bb4:
    %12 = const 3
    %13 = icmp Lt %7, %12
    %14 = Not %13
    br %14, label bb5, label bb6

bb5:
    br label bb2

bb6:
    extern_call env.console.log(%7) [effects: pure|io]  ; ← print(i)
    %15 = const 1
    %16 = %7 Add %15   ; ← i = i + 1
    %7 = copy %16
    br label bb3
}
```

### 🚨 **重大な発見**

#### 現象

**ループ内 if/else ブロックが MIR に存在しない！**

- `.hako` ソースコードには `if i > 0 { sum = sum + 1 }` が書いてあるのに、
- MIR には bb6 ブロックに `print(i)` と `i = i + 1` しかない
- `sum` 変数に関する処理（条件分岐・加算・PHI）が **完全に消失**

#### print 追加による検証

if ブロックが DCE で消えている可能性を考え、if 内に `print(sum)` を追加：

```hako
if i > 0 {
    sum = sum + 1
    print(sum)  // ← Force if to stay in MIR
} else {
    print(0)  // ← Ensure else branch exists
}
```

**結果**: MIR は変わらず。if/else ブロック自体が MIR に現れない。

---

## Task 212-4: 観測結果の記録 ✅

### ✅ 成功した部分

1. **Pattern Routing 動作**: Pattern 1 が構造ベースで正しく選ばれた
2. **JoinIR 生成**: Pattern 1 lowerer が動作し、JoinIR 関数 (main/loop_step/k_exit) を生成
3. **Carrier 処理**: `i` carrier の PHI 配線は正常

### ❌ 失敗した部分

**Root Cause**: **AST → MIR 変換層でループ内 if/else が消失**

#### 発見した制約

| 項目 | 内容 |
|-----|-----|
| **制約層** | AST → MIR 変換（Parser or MIR Builder） |
| **現象** | ループ内 if/else の代入文が MIR に変換されない |
| **影響範囲** | JoinIR Pattern 3 (IfPHI) が動作する以前の問題 |
| **エラーメッセージ** | なし（silent failure） |
| **再現性** | 100%（print 追加でも変わらず） |

#### 詳細分析

**予想される原因**:

1. **Parser の制限**:
   - ループ本体の if/else が正しく AST に変換されていない可能性
   - AST ノードが生成されても、型やスコープ情報が不完全

2. **MIR Builder の制限**:
   - `build_block()` がループ本体の if を処理する際に、条件付き代入を無視
   - ループ内 if の Merge PHI 生成ロジックが未実装

3. **変数スコープ問題**:
   - `sum` がループ外で `local` 宣言されているが、ループ内 if での更新が「新しい定義」として認識されない
   - ループ内変数更新が SSA 形式に変換されない

**確認が必要な層**:

- `src/mir/builder/control_flow/if_form.rs` - if 式の MIR 変換ロジック
- `src/mir/builder/control_flow/loop_form.rs` - ループ本体の処理
- `src/mir/builder/build_block.rs` - ブロック構築ロジック
- `src/parser/` - AST 生成の正確性

### JoinIR インフラへの影響

**Phase 212 の結論**:

- ✅ JoinIR Pattern Routing 自体は正常動作
- ✅ Pattern 1 (Simple While) の carrier 処理は完璧
- ❌ **ループ内 if の AST→MIR 変換が Phase 212 のブロッカー**

**Phase 213 への影響**:

- Phase 213 (3-carrier テスト) も同じ問題に遭遇する可能性が高い
- **先に AST→MIR 層の修正が必要**

---

## 📊 Phase 212 Overall Evaluation

### 成果

1. **Fail-Fast 成功**: Phase 211 の設計段階では見えなかった制約を 1 回の実行で発見
2. **制約の層を特定**: JoinIR ではなく **AST→MIR 変換層** の問題と判明
3. **再現性確認**: MIR ダンプで問題を可視化・記録

### 次のステップ

**Phase 212.5 (緊急対応)**: AST→MIR ループ内 if 変換の調査・修正

**調査項目**:
1. ループ内 if の AST ノードが正しく生成されているか確認
2. `build_block()` がループ本体の if をどう処理しているか追跡
3. ループ内変数更新の SSA 変換ロジックを確認

**実装方針**:
- Phase 212 は「観測フェーズ」として完了
- Phase 212.5 で AST→MIR 修正（別タスク）
- Phase 213 以降は Phase 212.5 完了後に再開

---

## 📝 参考情報

### 関連ドキュメント

- Phase 211 設計: `docs/development/current/main/phase211-loop-candidate-selection.md`
- JoinIR アーキテクチャ: `docs/development/current/main/joinir-architecture-overview.md`
- Pattern Routing: `src/mir/join_ir/lowering/loop_pattern_router.rs`

### 関連コード

- テストファイル: `apps/tests/phase212_if_sum_min.hako`
- Pattern 1 Lowerer: `src/mir/join_ir/lowering/loop_patterns/simple_while.rs`
- Pattern 3 Lowerer: `src/mir/join_ir/lowering/loop_patterns/with_if_phi.rs`

---

**Phase 212 ステータス**: ⚠️ BLOCKED（AST→MIR 層の制約により中断）
**次のアクション**: Phase 212.5（AST→MIR ループ内 if 修正）
Status: Active  
Scope: If-sum 実装メモ（JoinIR v2）
