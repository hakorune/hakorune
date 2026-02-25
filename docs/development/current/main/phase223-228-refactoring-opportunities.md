# Phase 223-228 リファクタリング機会調査レポート

## 🎯 調査の目的

Phase 223-228 の実装を通じて、以下の観点でリファクタリング機会を探す：

1. **削除可能なレガシーコード** - 新しい設計に置き換わったもの
2. **重複コード** - 同じ処理が複数箇所に存在するもの
3. **未使用の型/関数** - 削除して構造を簡潔に
4. **箱化のチャンス** - 分散している処理を1箇所に集約

## 📊 調査結果サマリー

### ✅ 発見した重複コード

| カテゴリ | 重複箇所 | 行数 | 改善優先度 |
|---------|---------|------|-----------|
| パターン検出 | Trim/DigitPos Promoter | 1371行 | 中 |
| ConditionOnly フィルタ | ExitLine 3ファイル | 540行 | 高 |
| ConditionAlias 冗長性 | CarrierInfo 連携 | - | 高 |
| MethodCall Lowering | 既に統一化済み | - | ✅完了 |

### 🎯 重要発見

1. **ConditionAlias は必要か？** - CarrierVar.role と promoted_loopbodylocals で代替可能
2. **ConditionOnly フィルタ分散** - 4ファイルに同じロジックが存在
3. **MethodCallLowerer 成功例** - Phase 224で既に Box化済み（参考にすべき）

---

## 🔍 詳細調査

### A. パターン検出の重複 (優先度: 中)

#### 現状

**2つの独立した Promoter**:
- `loop_body_carrier_promoter.rs` (658行) - Trim パターン
- `loop_body_digitpos_promoter.rs` (713行) - DigitPos パターン

**重複している処理**:

1. **メソッド検出ロジック**
   ```rust
   // Trim promoter (line 279)
   fn is_substring_method_call(node: &ASTNode) -> bool {
       matches!(node, ASTNode::MethodCall { method, .. } if method == "substring")
   }

   // DigitPos promoter (line 433)
   fn is_substring_method_call(node: &ASTNode) -> bool {
       matches!(node, ASTNode::MethodCall { method, .. } if method == "substring")
   }
   ```
   **→ 完全に同じ関数が2箇所に存在！**

2. **条件変数抽出ロジック**
   - Trim: `extract_equality_literals()` (line 301)
   - DigitPos: `extract_comparison_var()` (line 317)
   - 両方とも AST を worklist で走査する同じパターン

3. **CarrierInfo 構築**
   - 両方とも `carrier_info.carriers.push()` と `condition_aliases.push()` を呼ぶ

#### 改善案

**Option 1: 共通トレイト抽出** (中リスク)

```rust
/// 共通のパターン検出トレイト
pub trait LoopBodyLocalPattern {
    /// パターン名（デバッグ用）
    fn pattern_name() -> &'static str;

    /// 初期化式が許可されるメソッド名
    fn allowed_init_methods() -> &'static [&'static str];

    /// 条件式が許可される比較パターン
    fn allowed_condition_patterns() -> &'static [ConditionPattern];

    /// パターン検出
    fn detect(
        cond_scope: &LoopConditionScope,
        loop_body: &[ASTNode],
        break_cond: Option<&ASTNode>,
    ) -> Option<PromotionCandidate>;
}

impl LoopBodyLocalPattern for TrimPattern { ... }
impl LoopBodyLocalPattern for DigitPosPattern { ... }
```

**メリット**:
- 共通ロジックを1箇所に集約
- 新しいパターン追加が容易
- テスト容易性向上

**デメリット**:
- 既存コードへの影響大（2ファイル全体のリファクタリング）
- 回帰テストが必須
- 実装難度: 中〜高

**推奨度**: ⭐⭐⭐ (中) - 新しいパターン追加時に検討

---

### B. ConditionAlias の必要性検証 (優先度: 高)

#### 現状

**ConditionAlias の目的**:
```rust
pub struct ConditionAlias {
    pub old_name: String,           // "digit_pos"
    pub carrier_name: String,       // "is_digit_pos"
}
```

条件式で `digit_pos` を参照した時に `is_digit_pos` に変換する。

**使用箇所**:
- 定義: `carrier_info.rs` (line 96)
- 追加:
  - `loop_body_digitpos_promoter.rs` (line 203)
  - `loop_body_carrier_promoter.rs` (line 87)
- 使用: `pattern2_with_break.rs` (line 356-379)

**使用パターン**:
```rust
// pattern2_with_break.rs (line 356)
for alias in &carrier_info.condition_aliases {
    if let Some(carrier) = carriers_with_join_ids.iter().find(|c| c.name == alias.carrier_name) {
        if let Some(join_id) = carrier.join_id {
            env.insert(alias.old_name.clone(), join_id);
        }
    }
}
```

#### 問題点

**既に `promoted_loopbodylocals` がある！**

```rust
pub struct CarrierInfo {
    pub promoted_loopbodylocals: Vec<String>, // ["digit_pos"]
    pub carriers: Vec<CarrierVar>,            // [CarrierVar{name: "is_digit_pos", ...}]
    pub condition_aliases: Vec<ConditionAlias>, // [ConditionAlias{old: "digit_pos", carrier: "is_digit_pos"}]
}
```

**冗長性の分析**:
1. `promoted_loopbodylocals` に "digit_pos" が記録されている
2. `carriers` に "is_digit_pos" が記録されている（role 付き）
3. `condition_aliases` で両者をマッピング

→ **`condition_aliases` は `promoted_loopbodylocals` と `carriers` の情報から導出可能！**

#### 改善案

**Option 1: ConditionAlias 削除** (低リスク、高リターン)

```rust
// ConditionAlias を削除して、必要な時に動的に解決
fn resolve_promoted_variable(
    var_name: &str,
    carrier_info: &CarrierInfo,
) -> Option<String> {
    // promoted_loopbodylocals に含まれていたら carrier 名を探す
    if carrier_info.promoted_loopbodylocals.contains(&var_name.to_string()) {
        // 命名規則: "digit_pos" → "is_digit_pos"
        // または carriers を検索して一致するものを探す
        let promoted_name = format!("is_{}", var_name);
        if carrier_info.carriers.iter().any(|c| c.name == promoted_name) {
            return Some(promoted_name);
        }
    }
    None
}
```

**メリット**:
- **データ構造の簡素化** - CarrierInfo のフィールドが1つ減る
- **保守コスト削減** - 3箇所でデータ整合性を保つ必要がなくなる
- **confusion 削減** - 「何のためのフィールドか」が明確になる

**デメリット**:
- 命名規則の依存（"is_" prefix）が必要
- または carriers を線形探索（パフォーマンス無視可能）

**影響範囲**:
- `carrier_info.rs` - ConditionAlias 型削除 (line 96)
- `pattern2_with_break.rs` - resolve 関数に置き換え (line 356)
- 2つの promoter - condition_aliases.push() 削除

**推奨度**: ⭐⭐⭐⭐⭐ (高) - **今すぐ実施推奨**

**実装難度**: 低 (削除が主、1〜2時間)

---

### C. ConditionOnly フィルタの分散 (優先度: 高)

#### 現状

**ConditionOnly フィルタが4ファイルに分散**:

1. **meta_collector.rs** (line 131)
   ```rust
   let is_condition_only = if let Some(ci) = carrier_info {
       ci.carriers.iter().any(|c| c.name == *carrier_name && c.role == CarrierRole::ConditionOnly)
   } else {
       false
   };
   ```

2. **reconnector.rs** (line 109, 192)
   ```rust
   if binding.role == CarrierRole::ConditionOnly {
       // Skip ConditionOnly carriers (no variable_map update)
       continue;
   }
   ```

3. **instruction_rewriter.rs** (line 615)
   ```rust
   if binding.role == crate::mir::join_ir::lowering::carrier_info::CarrierRole::ConditionOnly {
       eprintln!("Skipping ConditionOnly carrier from exit PHI");
       continue;
   }
   ```

**計25箇所の ConditionOnly 参照** (merge/ 全体)

#### 問題点

**同じロジックが繰り返される**:
- 「ConditionOnly なら exit PHI を skip」
- 「ConditionOnly なら variable_map を更新しない」
- 「ConditionOnly なら exit_bindings に含める（latch 用）が reconnect しない」

→ **これらの判断ロジックが4ファイルに散らばっている**

#### 改善案

**Option 1: ExitLinePolicy Trait** (中リスク)

```rust
/// Exit line 処理のポリシー箱
pub trait ExitLinePolicy {
    /// exit_bindings に含めるべきか？
    fn should_collect(role: CarrierRole) -> bool {
        true // ConditionOnly も collect（latch incoming 用）
    }

    /// variable_map を更新すべきか？
    fn should_reconnect(role: CarrierRole) -> bool {
        role == CarrierRole::LoopState // LoopState のみ
    }

    /// exit PHI を生成すべきか？
    fn should_create_exit_phi(role: CarrierRole) -> bool {
        role == CarrierRole::LoopState // LoopState のみ
    }
}

/// デフォルト実装
pub struct DefaultExitLinePolicy;
impl ExitLinePolicy for DefaultExitLinePolicy {}
```

**使用例**:
```rust
// reconnector.rs
if !DefaultExitLinePolicy::should_reconnect(binding.role) {
    continue;
}

// instruction_rewriter.rs
if !DefaultExitLinePolicy::should_create_exit_phi(binding.role) {
    continue;
}
```

**メリット**:
- **単一責任** - ロールに基づく判断が1箇所に集約
- **テスト容易** - Policy をモックできる
- **拡張性** - 新しいロール追加時に1箇所修正

**デメリット**:
- 新しい抽象層の追加
- 既存の4ファイルを修正
- 実装難度: 中

**Option 2: ExitLineOrchestrator Box** (高リスク)

```rust
/// ExitLine 処理の統括箱
pub struct ExitLineOrchestrator;

impl ExitLineOrchestrator {
    /// exit_bindings を収集して、reconnect と exit PHI を実行
    pub fn process_exit_line(
        boundary: &JoinInlineBoundary,
        carrier_phis: &BTreeMap<String, ValueId>,
        variable_map: &mut BTreeMap<String, ValueId>,
        blocks: &mut Vec<MirBasicBlock>,
        exit_block_id: BasicBlockId,
    ) -> Result<(), String> {
        // 1. meta_collector の処理
        // 2. reconnector の処理
        // 3. exit PHI builder の処理
        // を統合
    }
}
```

**メリット**:
- 最も「箱理論的」
- ExitLine の全処理が1箇所に

**デメリット**:
- Phase 33 で分離した構造を再統合（後退？）
- 影響範囲が非常に大きい
- 実装難度: 高

**推奨度**:
- Option 1: ⭐⭐⭐⭐ (高) - **Policy Trait を推奨**
- Option 2: ⭐⭐ (低) - 統合しすぎ

**実装難度**:
- Option 1: 中 (2〜3時間)
- Option 2: 高 (1日+)

---

### D. MethodCall Lowering 統一化 (優先度: ✅完了)

#### 現状

**Phase 224 で既に Box化完了！**

- `method_call_lowerer.rs` - 統一的な MethodCallLowerer Box
- `loop_body_local_init.rs` - MethodCallLowerer::lower_for_init() を呼ぶ
- `condition_lowerer.rs` - MethodCallLowerer::lower_for_condition() を呼ぶ

**設計の特徴**:
- **Metadata-Driven** - CoreMethodId ベースで判断
- **Fail-Fast** - whitelist にない method は即エラー
- **Context-Aware** - for_condition / for_init で異なる whitelist

#### 教訓

**これが理想的な Box化！**

**Before** (Phase 223):
```rust
// loop_body_local_init.rs
match method {
    "substring" => { /* inline lowering */ }
    "indexOf" => { /* inline lowering */ }
    _ => Err("Unknown method")
}

// condition_lowerer.rs
match method {
    "length" => { /* inline lowering */ }
    _ => Err("Unknown method")
}
```

**After** (Phase 224):
```rust
// loop_body_local_init.rs
MethodCallLowerer::lower_for_init(recv, method, args, ...)

// condition_lowerer.rs
MethodCallLowerer::lower_for_condition(recv, method, args, ...)

// method_call_lowerer.rs (統一箇所)
impl MethodCallLowerer {
    pub fn lower_for_init(...) -> Result<ValueId, String> {
        let method_id = CoreMethodId::iter().find(|m| m.name() == method_name)?;
        if !method_id.allowed_in_init() {
            return Err("Method not allowed in init");
        }
        // ... 統一的なロジック
    }
}
```

**成果**:
- 重複コード削減: 推定 200行+
- 保守性向上: method 追加時に1箇所修正
- テスト容易性: MethodCallLowerer を単独テスト可能

→ **A, C の改善案でも同じパターンを採用すべき**

---

## 🎯 推奨アクション

### 📋 優先順位

| 項目 | 優先度 | 実装難度 | リターン | 推奨実施時期 |
|-----|-------|---------|---------|-------------|
| **B. ConditionAlias 削除** | ⭐⭐⭐⭐⭐ | 低 | 高 | **Phase 229** |
| **C. ExitLinePolicy Trait** | ⭐⭐⭐⭐ | 中 | 高 | Phase 230 |
| **A. パターン検出共通化** | ⭐⭐⭐ | 中 | 中 | Phase 231+ |

### ✅ Phase 229 推奨アクション

**1. ConditionAlias 削除** (1〜2時間)

**Step 1**: resolve 関数実装
```rust
// carrier_info.rs に追加
impl CarrierInfo {
    /// 昇格された変数の carrier 名を解決
    pub fn resolve_promoted_carrier(&self, old_name: &str) -> Option<&str> {
        if !self.promoted_loopbodylocals.contains(&old_name.to_string()) {
            return None;
        }

        // 命名規則: "digit_pos" → "is_digit_pos"
        let expected_name = format!("is_{}", old_name);
        self.carriers.iter()
            .find(|c| c.name == expected_name)
            .map(|c| c.name.as_str())
    }
}
```

**Step 2**: pattern2_with_break.rs 修正
```rust
// 削除: for alias in &carrier_info.condition_aliases { ... }

// 追加:
for promoted_var in &carrier_info.promoted_loopbodylocals {
    if let Some(carrier_name) = carrier_info.resolve_promoted_carrier(promoted_var) {
        if let Some(carrier) = carriers_with_join_ids.iter().find(|c| c.name == carrier_name) {
            if let Some(join_id) = carrier.join_id {
                env.insert(promoted_var.clone(), join_id);
            }
        }
    }
}
```

**Step 3**: ConditionAlias 型削除
- `carrier_info.rs` - struct ConditionAlias 削除
- 2つの promoter - condition_aliases.push() 削除

**Step 4**: テスト
```bash
# Trim pattern
cargo test --release test_mir_joinir_funcscanner_trim

# DigitPos pattern
cargo test --release test_loopbodylocal_digitpos

# Pattern 2 integration
cargo test --release test_loop_with_break
```

**期待される成果**:
- CarrierInfo のフィールド: 6 → 5
- 保守コスト削減: 3箇所のデータ整合性チェック不要
- confusion 削減: 「condition_aliases は何のため？」問題解消

---

## 📐 設計ガイドライン

### ✅ 良い Box化の特徴 (MethodCallLowerer から学ぶ)

1. **単一責任** - 「この Box は X を判断する」が明確
2. **Metadata-Driven** - ハードコードより設定駆動
3. **Fail-Fast** - フォールバックより明示的エラー
4. **Context-Aware** - 用途に応じた API (for_init / for_condition)
5. **独立テスト可能** - 単独で動作検証できる

### ❌ 避けるべきパターン

1. **情報の重複** - 同じ情報を複数の場所に保存
2. **ロジックの分散** - 同じ判断が4箇所に散らばる
3. **型の冗長** - CarrierVar.role があるのに binding.role も持つ
4. **暗黙の依存** - 命名規則に依存しすぎ（"is_" prefix など）

---

## 🧪 回帰テスト戦略

### Phase 229 変更時のテストセット

**Level 1: 単体テスト**
```bash
cargo test --lib carrier_info
cargo test --lib loop_pattern_detection
```

**Level 2: パターンテスト**
```bash
# Trim pattern
cargo test --release test_mir_joinir_funcscanner_trim

# DigitPos pattern
cargo test --release test_loopbodylocal_digitpos

# Pattern 2 (break)
cargo test --release test_loop_with_break
```

**Level 3: E2E テスト**
```bash
# Smoke tests
tools/smokes/v2/run.sh --profile quick --filter "loop_*"

# Full MIR test suite
cargo test --release --test '*' 2>&1 | grep -E "(test result|FAILED)"
```

**決定性テスト** (重要！):
```bash
# 3回実行して ValueId が変わらないことを確認
for i in 1 2 3; do
    echo "=== Run $i ==="
    cargo test --release test_loop_with_break 2>&1 | grep -E "ValueId|test result"
done
```

---

## 📝 実装時の注意事項

### ConditionAlias 削除時

1. **命名規則の明文化**
   - "digit_pos" → "is_digit_pos" の変換ルールをドキュメント化
   - または CarrierVar に `original_name: Option<String>` フィールド追加

2. **エラーメッセージの改善**
   ```rust
   if carrier_info.resolve_promoted_carrier(var_name).is_none() {
       return Err(format!(
           "Variable '{}' was promoted but carrier not found. Expected 'is_{}' in carriers.",
           var_name, var_name
       ));
   }
   ```

3. **デバッグログの追加**
   ```rust
   eprintln!(
       "[pattern2/resolve] Promoted variable '{}' resolved to carrier '{}'",
       old_name, carrier_name
   );
   ```

### ExitLinePolicy 実装時

1. **trait より struct + impl 推奨**
   - trait は over-engineering になりがち
   - static メソッドで十分

2. **ドキュメント重視**
   ```rust
   /// Phase 227: ConditionOnly Filtering Policy
   ///
   /// ## Design Decision
   ///
   /// ConditionOnly carriers are:
   /// - Collected in exit_bindings (for latch incoming)
   /// - NOT reconnected in variable_map (no exit PHI)
   /// - NOT included in exit PHI
   impl ExitLinePolicy {
       pub fn should_reconnect(role: CarrierRole) -> bool { ... }
   }
   ```

---

## 📊 期待される改善効果

### ConditionAlias 削除 (Phase 229)

- **削減行数**: ~50行 (carrier_info.rs, 2 promoters, pattern2)
- **保守コスト削減**: CarrierInfo のフィールド管理が簡単に
- **confusion 削減**: 「なぜ3箇所に情報があるか？」問題解消
- **実装時間**: 1〜2時間
- **リスク**: 低（既存のテストで検証可能）

### ExitLinePolicy 実装 (Phase 230)

- **削減行数**: ~20行 (重複 if 文の削減)
- **可読性向上**: 判断ロジックが1箇所に
- **拡張性**: 新しい CarrierRole 追加時に1箇所修正
- **実装時間**: 2〜3時間
- **リスク**: 中（4ファイル修正）

### パターン検出共通化 (Phase 231+)

- **削減行数**: ~200行+ (重複ロジック統合)
- **拡張性**: 新パターン追加が容易
- **テスト容易性**: 共通部分を単独テスト可能
- **実装時間**: 1日+
- **リスク**: 高（2大ファイルの全面リファクタリング）

---

## 🎯 まとめ

### 今すぐやるべきこと (Phase 229)

✅ **ConditionAlias 削除** - 低リスク・高リターン・1〜2時間

### 次にやるべきこと (Phase 230)

⭐ **ExitLinePolicy 実装** - 中リスク・高リターン・2〜3時間

### 後回しでOK (Phase 231+)

💡 **パターン検出共通化** - 新パターン追加時に検討

---

## 📚 参考資料

- **Phase 224 MethodCallLowerer**: `src/mir/join_ir/lowering/method_call_lowerer.rs`
- **Phase 227 CarrierRole**: `src/mir/join_ir/lowering/carrier_info.rs`
- **Phase 33 Exit Line**: `src/mir/builder/control_flow/joinir/merge/exit_line/`

---

**調査日**: 2025-12-10
**調査者**: Claude Code Assistant
**対象**: Phase 223-228 コードベース
Status: Active  
Scope: Refactoring 機会の整理（JoinIR/ExprLowerer ライン）
