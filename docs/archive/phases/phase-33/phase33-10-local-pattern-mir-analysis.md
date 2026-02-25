# Phase 33-10: Local Pattern の実MIR構造分析レポート

**作成日**: 2025-11-27
**分析者**: Claude Code
**目的**: If lowering における「local pattern」の実MIR構造を分析し、Phase 33-10の実装方針を決定する

---

## 1. Executive Summary

### 1.1 重要な結論

**PHI命令を持つMIRは、JoinIR変換の対象外とすべき（Option A推奨）**

理由:
1. **PHI命令は既にSSAの標準的な値合流表現** - これ以上の変換は不要
2. **JoinIR Select/IfMergeの役割は "PHI生成"** - 既存PHIを変換するのは逆行
3. **try_match_local_pattern()の想定は誤り** - 実MIRとの構造差分が根本的

### 1.2 実装への影響

**Phase 33-10の方針**:
- ❌ try_match_local_pattern()の修正は**不要**
- ✅ local patternは**Phase 33-10の対象外**として扱う
- ✅ JoinIR loweringは「PHI命令がない場合のみ」実施
- ✅ 既存のif_phi.rsによるPHI生成は維持（これが正常動作）

---

## 2. 構造比較詳細

### 2.1 ユニットテスト想定（誤り）

**try_match_local_pattern()が期待していた構造**:

```mir
bb0: br cond, bb1, bb2

bb1 (then):
  Copy { dst: x, src: 100 }  ← 単一の代入命令
  Jump bb3

bb2 (else):
  Copy { dst: x, src: 200 }  ← 単一の代入命令
  Jump bb3

bb3 (merge):
  [空 - 命令なし]            ← 重要: 命令がない！
  Return x
```

**検証コード**（if_select.rs:262-263）:
```rust
if !merge_block.instructions.is_empty() {
    return None;  // ← merge blockが空でなければ失敗
}
```

### 2.2 実MIR構造（正しい）

**実際のコンパイラ出力** (`/tmp/test_phi_local.hako`):

```mir
define i64 @main() {
bb0:
    1: %3: Integer = const 1
    1: %4: Integer = copy %3
    1: %5: Integer = copy %4
    1: br %5, label bb3, label bb4

bb3 (then):
    1: %8: Integer = const 100  ← Const命令（Copy不使用）
    1: br label bb5

bb4 (else):
    1: %11: Integer = const 200  ← Const命令（Copy不使用）
    1: br label bb5

bb5 (merge):
    1: %12 = phi [%8, bb3], [%11, bb4]  ← PHI命令が存在！
    1: ret %12
}
```

**構造差分**:

| 要素 | ユニットテスト想定 | 実MIR | 差分の意味 |
|------|------------------|-------|----------|
| Then/Else命令 | `Copy { dst: x, src: 100 }` | `Const { dst: %8, value: 100 }` | Copyではなく直接Const |
| Merge命令 | **空（なし）** | **`Phi { dst: %12, inputs: [...] }`** | PHI命令が存在 |
| Return値 | `Return x`（then/elseで書き込んだ変数） | `Return %12`（PHI結果の新しいValueId） | 異なるValueId |

### 2.3 Phase 33-9.2の修正との比較

**Phase 33-9.2で修正したSimple pattern**:

```rust
// 修正内容: Const/Copyを許容
fn is_side_effect_free(&self, instructions: &[MirInstruction]) -> bool {
    instructions.iter().all(|inst| {
        matches!(inst, MirInstruction::Const { .. } | MirInstruction::Copy { .. })
    })
}
```

**Simple patternの実MIR**:
```mir
bb3 (then):
  %8: Integer = const 100
  ret %8                    ← Return命令（merge blockなし）

bb4 (else):
  %11: Integer = const 200
  ret %11                   ← Return命令（merge blockなし）
```

**重要な違い**:
- **Simple**: merge blockなし → return直接
- **Local**: merge blockあり → **PHI命令で合流**

---

## 3. PHI命令の妥当性検証

### 3.1 SSA形式の正しさ

**実MIRのPHI構造は完全に正しい**:

1. **定義（Definition）**:
   - SSA形式では、複数の前任ブロックから値が流入する場合、PHI命令で合流させる
   - これが**唯一の正規的な表現**

2. **実MIRの構造**:
   ```mir
   bb5 (merge):
       %12 = phi [%8, bb3], [%11, bb4]
   ```
   - bb3から%8が流入
   - bb4から%11が流入
   - bb5では%12として合流
   - **完璧なSSA形式**

3. **検証**:
   - ✅ 各ValueIdは一度だけ定義される（Single Assignment）
   - ✅ 前任ブロックごとの値が明確（Static Single）
   - ✅ PHI配置は支配フロンティアに従っている

### 3.2 PHI命令の配置

**標準的なPHI配置規則**:
- PHI命令はmerge blockの**先頭**に配置される
- Return/Jump等の終端命令の**前**に配置される

**実MIRの配置**:
```mir
bb5:
    1: %12 = phi [%8, bb3], [%11, bb4]  ← 先頭に配置
    1: ret %12                          ← 終端命令
```

✅ **完全に標準的な配置**

---

## 4. JoinIR変換の必要性分析

### 4.1 JoinIR Select/IfMergeの役割

**設計文書より** (if_joinir_design.md):

```rust
/// Phase 33.1: 単純な値選択
Select {
    dst: VarId,
    cond: VarId,
    then_val: VarId,
    else_val: VarId,
}
```

**Select命令の意味論**:
- 条件に応じて値を選択する
- **MIR reverse loweringでPHI命令を生成する**（重要！）

### 4.2 PHI → Select変換の問題点

**仮にPHI → Selectを実装した場合**:

```
MIR (入力):
  bb5: %12 = phi [%8, bb3], [%11, bb4]

↓ lowering (MIR → JoinIR)

JoinIR:
  Select { dst: %12, cond: ???, then_val: %8, else_val: %11 }

↓ reverse lowering (JoinIR → MIR)

MIR (出力):
  bb5: %12 = phi [%8, bb3], [%11, bb4]  ← 元に戻る
```

**問題**:
1. **情報損失**: PHI → Select時に条件式が不明（%5はbb0で消費済み）
2. **無意味な往復**: MIR → JoinIR → MIR で同じPHIに戻る
3. **責務の逆転**: JoinIRの役割は「PHI生成」であり、「PHI変換」ではない

### 4.3 JoinIR変換が有意義なケース

**唯一有意義なのは「PHI命令がまだ生成されていない場合」**:

```
高レベルAST:
  if cond { x = 100 } else { x = 200 }; return x

↓ (従来の方式: if_phi.rs経由)

MIR:
  bb5: %12 = phi [%8, bb3], [%11, bb4]
  ret %12

↓ (新しい方式: JoinIR経由)

JoinIR:
  Select { dst: %12, cond: %5, then_val: %8, else_val: %11 }

↓ reverse lowering

MIR:
  bb5: %12 = phi [%8, bb3], [%11, bb4]
  ret %12
```

**結果**: 最終MIRは同じだが、生成経路が異なる

**価値**:
- ✅ if_phi.rsの削除が可能（コード削減）
- ✅ JoinIR側で型検証が可能
- ❌ 既にPHIがある場合は意味なし

---

## 5. Phase 33の設計意図確認

### 5.1 設計文書の記述

**if_joinir_design.md:11-27より**:

```
1. If/Else の「値としての if」（`if ... then ... else ...` の PHI）を JoinIR 側で表現できるようにする。
   - ループ PHI と同じく、「PHI を関数引数や join 関数の引数に押し出す」設計に寄せる。
   - MIR Builder 側の if_phi.rs / conservative.rs / phi_invariants.rs に依存しない JoinIR lowering を用意する。
```

**重要な前提**:
- **「MIR Builder側のif_phi.rsに依存しない」** = if_phi.rsを**使わずに**PHIを生成する
- **PHI生成の新しい経路**としてJoinIRを使う

### 5.2 if_phi.rsの役割

**現状のif_phi.rs**:
- ASTから直接PHI命令を生成（MIR Builder層）
- if/else構造を解析してPHI配置を決定
- 316行のロジック

**JoinIR経由の場合**:
- ASTからJoinIR Select/IfMergeを生成
- JoinIR → MIR reverse lowering時にPHI命令を生成
- if_phi.rsは**削除可能**

### 5.3 Phase 33のターゲット

**Phase 33-10の実装対象**:

```
入力: AST（まだPHI生成されていない）
出力: MIR（PHI命令を含む）

経路選択:
  Route A（従来）: AST → if_phi.rs → MIR（PHI）
  Route B（新規）: AST → JoinIR lowering → JoinIR Select → reverse lowering → MIR（PHI）
```

**Phase 33-10が**扱うべきでない**ケース**:
```
入力: MIR（既にPHI命令が存在）
出力: ???（何に変換する意味がある？）
```

---

## 6. Phase 33-10実装方針

### 6.1 推奨方針: Option A（PHI命令を変換対象外とする）

**実装内容**:

```rust
// src/mir/join_ir/lowering/if_select.rs
fn try_match_local_pattern(
    &self,
    func: &MirFunction,
    branch: &IfBranch,
    then_block: &crate::mir::BasicBlock,
    else_block: &crate::mir::BasicBlock,
) -> Option<IfPattern> {
    // Phase 33-10: PHI命令が既に存在する場合は対象外
    // （JoinIRの役割はPHI生成であり、既存PHI変換ではない）

    // merge blockの取得
    let merge_block_id = match then_block.terminator.as_ref()? {
        MirInstruction::Jump { target } => *target,
        _ => return None,
    };

    let merge_block = func.blocks.get(&merge_block_id)?;

    // ❌ PHI命令がある場合は変換対象外（重要！）
    for inst in &merge_block.instructions {
        if matches!(inst, MirInstruction::Phi { .. }) {
            return None;  // 既にPHIがあるので、JoinIR変換不要
        }
    }

    // ✅ PHI命令がない場合のみ、local patternとして処理
    // （この場合は現在の実装で正しく動作する）

    // ... 既存のロジック継続 ...
}
```

**効果**:
- ✅ PHI命令がある = 既にif_phi.rsで処理済み → JoinIR変換不要
- ✅ PHI命令がない = JoinIR経由で処理すべき → 現在の実装が機能
- ✅ 責務の明確化: JoinIRは「PHI生成器」として機能

### 6.2 Option B（PHI → Select/IfMerge変換）の問題点

**実装した場合の問題**:

1. **条件式の復元困難**:
   ```mir
   bb0: br %5, label bb3, label bb4  ← %5はここで消費
   ...
   bb5: %12 = phi [%8, bb3], [%11, bb4]  ← %5にアクセスできない
   ```

2. **無意味な往復変換**:
   - MIR（PHI） → JoinIR（Select） → MIR（PHI）
   - 何も変わらない

3. **Phase 33の意図に反する**:
   - 目的: if_phi.rsを削除して、JoinIR経由でPHI生成
   - 実際: 既にあるPHIを変換（意味なし）

### 6.3 Option C（新しいパターン定義）の不要性

**「PHI-based pattern」を定義する必要はない**:

理由:
1. PHIがある = 既にMIR Builder（if_phi.rs）で処理済み
2. JoinIRの役割は「**まだPHIがない**場合にPHI生成する」こと
3. 既存PHIを再処理するパターンは設計意図に反する

---

## 7. Simple PatternとLocal Patternの真の違い

### 7.1 Simple Pattern（正しく動作）

**構造**:
```mir
bb0: br cond, bb3, bb4

bb3 (then):
  %8 = const 100
  ret %8              ← 直接return（merge blockなし）

bb4 (else):
  %11 = const 200
  ret %11             ← 直接return（merge blockなし）
```

**特徴**:
- ❌ merge blockが存在しない
- ❌ PHI命令が不要（各ブロックで直接return）
- ✅ JoinIR Selectで表現可能

### 7.2 Local Pattern（既にPHI生成済み）

**構造**:
```mir
bb0: br cond, bb3, bb4

bb3 (then):
  %8 = const 100
  br bb5              ← mergeへジャンプ

bb4 (else):
  %11 = const 200
  br bb5              ← mergeへジャンプ

bb5 (merge):
  %12 = phi [%8, bb3], [%11, bb4]  ← PHI命令（既に生成済み）
  ret %12
```

**特徴**:
- ✅ merge blockが存在
- ✅ **PHI命令が既に存在**（if_phi.rsで生成済み）
- ❌ JoinIR変換の必要なし（既にSSA形式完成）

### 7.3 真のLocal Pattern（JoinIR変換すべきケース）

**仮想的な構造**（実際には生成されない）:
```mir
bb0: br cond, bb3, bb4

bb3 (then):
  Store { ptr: &x, value: 100 }  ← PHIではなくStore
  br bb5

bb4 (else):
  Store { ptr: &x, value: 200 }  ← PHIではなくStore
  br bb5

bb5 (merge):
  [空 - 命令なし]                ← PHIなし
  %v = Load { ptr: &x }          ← 値の読み込み
  ret %v
```

**現実**:
- このような構造は**MIR Builderが生成しない**
- Nyashは常にSSA形式 → 必ずPHI命令を使用
- したがって、try_match_local_pattern()の想定自体が**実装されないケース**を前提にしている

---

## 8. Phase 33全体への影響

### 8.1 Simple Pattern（Phase 33-9.2）

**現状**: ✅ 完全動作
- Const/Copy許容で100%成功
- PHI命令なし → JoinIR変換が有意義

### 8.2 Local Pattern（Phase 33-10）

**結論**: ⚠️ **Phase 33-10の対象外**
- PHI命令あり → 既にif_phi.rsで処理完了
- JoinIR変換は不要（無意味な往復）

**推奨**:
- try_match_local_pattern()を**削除**するか、
- PHI命令チェックを追加して早期return

### 8.3 IfMerge Pattern

**Phase 33-6の設計との整合性**:

IfMerge設計（if_joinir_design.md:748-823）:
```rust
JoinInst::IfMerge {
    cond: ValueId,
    merges: Vec<MergePair>,  // 複数のPHI相当
}
```

**重要**: IfMergeも「PHI生成」が目的
- 入力: AST（複数変数代入）
- 出力: MIR（複数PHI命令）
- **既存PHI → IfMerge変換ではない**

---

## 9. 最終推奨事項

### 9.1 Phase 33-10の実装方針

**推奨: Option A（PHI命令を対象外とする）**

```rust
// Phase 33-10: 実装修正
fn try_match_local_pattern(...) -> Option<IfPattern> {
    // Step 1: merge blockの取得
    let merge_block_id = ...;
    let merge_block = func.blocks.get(&merge_block_id)?;

    // Step 2: PHI命令チェック（追加）
    if merge_block.instructions.iter().any(|inst| {
        matches!(inst, MirInstruction::Phi { .. })
    }) {
        // PHI命令がある = if_phi.rsで処理済み → JoinIR変換不要
        return None;
    }

    // Step 3: 既存のロジック継続
    // （PHI命令がない場合のみ到達）
    // ...
}
```

### 9.2 Phase 33-10の完了判定

**完了条件**:
1. ✅ try_match_local_pattern()にPHIチェック追加
2. ✅ 既存ユニットテスト維持（PHI命令なしケース）
3. ✅ 実用MIR（PHI命令あり）は正しくフォールバック
4. ✅ ドキュメント更新（本レポート）

**非目標**:
- ❌ PHI → Select変換実装（不要）
- ❌ local patternの実用MIR対応（既にif_phi.rsで処理済み）

### 9.3 Phase 33全体の戦略修正

**修正前の理解**:
- Simple/Local/IfMerge の3パターンをJoinIRで実装 → if_phi.rs削除

**修正後の理解**:
- **Simple**: PHI不要 → JoinIR変換有意義 ✅
- **Local**: PHI既存 → JoinIR変換不要（if_phi.rs保持） ⚠️
- **IfMerge**: 複数PHI → AST→JoinIR経路でのみ有意義 ✅

**結論**:
- if_phi.rsの完全削除は**Phase 34以降に延期**
- Phase 33は「PHI生成前の経路にJoinIR挿入」に集中
- 既存PHIの再処理は無意味 → 実装しない

---

## 10. 参考資料

### 10.1 実MIR出力（完全版）

```mir
; MIR Module: main
; Source: /tmp/test_phi_local.hako

define i64 @main() {
bb0:
    1: %3: Integer = const 1
    1: %4: Integer = copy %3
    1: %5: Integer = copy %4
    1: br %5, label bb3, label bb4

bb3:
    1: %8: Integer = const 100
    1: br label bb5

bb4:
    1: %11: Integer = const 200
    1: br label bb5

bb5:
    1: %12 = phi [%8, bb3], [%11, bb4]
    1: ret %12
}
```

### 10.2 Simple Pattern MIR（Phase 33-9.2）

```mir
define i64 @IfSelectTest.test/1(i64 %0) {
bb2:
    br %3, label bb3, label bb4

bb3:
    %4: Integer = const 10
    ret %4

bb4:
    %6: Integer = const 20
    ret %6
}
```

### 10.3 関連ファイル

- `src/mir/join_ir/lowering/if_select.rs:201-273` - try_match_local_pattern()実装
- `docs/private/roadmap2/phases/phase-33-joinir-if-phi-cleanup/if_joinir_design.md` - 設計文書
- `src/mir/builder/if_phi.rs` - 現行PHI生成ロジック（316行）

---

## 11. ChatGPT実装者への提言

### 11.1 実装不要の理由

**結論**: Phase 33-10で local pattern のMIR変換実装は**不要**

理由:
1. 実MIRは既に正しいPHI命令を含む
2. JoinIRの役割は「PHI生成」であり「PHI変換」ではない
3. 既存のif_phi.rsが正しく動作している
4. 無意味な往復変換を避けるべき

### 11.2 実装すべき内容

**唯一実装すべきこと**: PHI命令チェックの追加

```rust
// Phase 33-10: 5行の追加のみ
if merge_block.instructions.iter().any(|inst| {
    matches!(inst, MirInstruction::Phi { .. })
}) {
    return None;  // PHI命令がある場合は対象外
}
```

**効果**:
- ✅ 責務の明確化
- ✅ 無駄な処理の防止
- ✅ 設計意図との整合性

### 11.3 Phase 33-10完了判定

**完了条件（簡略化）**:
1. ✅ PHIチェック追加（5行）
2. ✅ 既存テスト維持
3. ✅ ドキュメント更新

**作業時間**: 30分以内

---

**Phase 33-10完了判定**: PHIチェック追加のみで完了とする
**次のフェーズ**: Phase 33-11（IfMerge実装、またはPhase 34へ移行）
Status: Historical
