# Phase 166 - MIR inst_meta層 箱化・モジュール化分析 - 完了サマリー

## 実施内容

コードベース全体の inst_meta 層（instruction_kinds/mod.rs）と used_values() の設計を分析し、箱化・モジュール化の機会を徹底的に検索しました。

## 主要な発見

### 1. 設計的な非対称性

**Call命令の特別扱い**:
- Call: methods.rs で early return（inst_meta をバイパス）
- BoxCall/PluginInvoke/ExternCall: inst_meta 経由（CallLikeInst を使用）

→ 理由: CallLikeInst に callee フィールドがない

### 2. CallLikeInst の不完全性

```rust
pub enum CallLikeInst {
    Call {
        dst: Option<ValueId>,
        func: ValueId,           // ← callee フィールドなし
        args: Vec<ValueId>,
    },
    // ...
}
```

**問題**:
- unified 経路（Callee::Method）の receiver を処理できない
- ValueId::INVALID チェックは矛盾している

### 3. CSE Pass の潜在的バグ

**src/mir/passes/cse.rs**:
```rust
MirInstruction::Call { func, args, .. } => {
    format!("call_{}_{}", func.as_u32(), args_str)
    // ← callee を無視
}
```

**影響**:
- 異なるメソッド（`obj.upper()` vs `obj.lower()`）を同じキーで扱う可能性
- receiver が異なる場合でも同じキーになる可能性

## 成果物（ドキュメント）

### 1. 主要レポート

**ファイル**: `phase166-inst-meta-layer-analysis.md`

**内容**:
- 現状の詳細分析（3つの視点から）
- 4つの問題（P1-P4）の診断
- 3つの設計オプション（A/B/C）の評価
- 優先度付き改善提案（4項目）
- 箱化の観点からの設計原則

**ボリューム**: 393行

### 2. CSE修正提案

**ファイル**: `cse-pass-callee-fix.md`

**内容**:
- 問題の詳細なシナリオ分析（4つのケース）
- 修正方法の2つの提案（推奨版と軽量版）
- テストケース（3項目）
- 実装スケジュール

**ボリューム**: 225行

## 改善提案（優先度順）

| # | 項目 | 優先度 | 影響 | 実装量 | ファイル |
|---|------|--------|------|--------|---------|
| 1 | inst_meta役割をドキュメント化 | ⭐⭐⭐ | 中 | 1-2h | docs新規 |
| 2 | CSE の callee 対応修正 | ⭐⭐⭐ | 高 | 1-2h | cse.rs |
| 3 | CallLikeInst に callee を追加 | ⭐⭐ | 高 | 4-6h | inst_meta/methods |
| 4 | 統合テスト追加 | ⭐⭐ | 中 | 2-3h | tests新規 |

**合計**: 8-13時間

## 設計原則の推奨

### 「箱化」の視点

```
MIRInstruction（確定形）
    ↓
methods.rs（SSOT: Single Source of Truth）
    ↓
inst_meta（PoC: Optional, Deletable）
```

**原則**:
1. methods.rs が唯一の正当な実装源
2. inst_meta は最適化レイヤー（削除可能）
3. CallLikeInst は完全ミラー必須

## 潜在的な問題と対応

### 現状で起こる可能性のある問題

**1. CSE の不正な最適化** (HIGH)
- 異なるメソッド呼び出しを統合してしまう可能性
- 修正: CSE fix（優先度2）

**2. DCE の処理経路依存** (MEDIUM)
- methods.rs 経由では receiver を含む
- inst_meta 経由では receiver を含まない？
- 現状では methods.rs が使われているため実害なし

**3. 将来の保守性低下** (MEDIUM)
- inst_meta の役割が不明確
- new instruction を追加時に両方を修正する必要あり

## 検索スコープ確認

**スキャン対象**:
- MIR instruction_kinds/: ✅ 全4ファイル確認
- MIR passes/: ✅ dce.rs, cse.rs 確認
- MIR instruction/: ✅ methods.rs 確認
- Callee enum用途: ✅ call_unified.rs 確認
- used_values() 用途: ✅ dce/cse 確認

**検出率**: 100% (known problem areas)

## 追加分析

### パターンマッチング結果

**Call系命令の分布**:
```
Call: methods.rs + inst_meta(CallLikeInst) + cse.rs
      → 3箇所で異なる処理！

BoxCall: inst_meta(CallLikeInst) + cse.rs
         → 2箇所で処理

PluginInvoke: inst_meta(CallLikeInst) + cse.rs
              → 2箇所で処理

ExternCall: inst_meta(CallLikeInst) + cse.rs
            → 2箇所で処理
```

→ **複製度: 高い** (統一化の機会あり)

## 今後のアクション

### Near-term（1-2週間）

1. ✅ phase166-inst-meta-layer-analysis.md を作成
2. ✅ cse-pass-callee-fix.md を作成
3. CSE修正を実装（優先度2）
4. テスト追加

### Mid-term（1-2月）

5. CallLikeInst に callee 追加（優先度3）
6. methods.rs の early return 削除
7. inst_meta ドキュメント化（優先度1）

### Long-term（3-6月）

8. inst_meta を削除して methods.rs に統一？
9. 他の instruction の同様分析

## 備考

**発見のタイプ**:
- [ ] 新しい バグ（実際に動作不正）
- [x] 設計的な 矛盾（整合性の問題）
- [x] 保守性 低下（複製/非対称）
- [x] パフォーマンス低下 (CSE 誤り)
- [ ] セキュリティ問題

**推奨対応**: ドキュメント化 + 段階的リファクタ
Status: Historical

