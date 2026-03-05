# リファクタリング機会 - クイックインデックス

Phase 223-228 実装を通じて発見されたリファクタリング機会の一覧です。

## 📊 サマリー

| Phase | タイトル | 優先度 | 実装難度 | 推奨時期 | ドキュメント |
|-------|---------|--------|---------|---------|-------------|
| **229** | **ConditionAlias 削除** | ⭐⭐⭐⭐⭐ | 低 | **今すぐ** | [phase229-action-plan.md](phase229-action-plan.md) |
| 230 | ExitLinePolicy Trait | ⭐⭐⭐⭐ | 中 | Phase 229 後 | [phase223-228-refactoring-opportunities.md](phase223-228-refactoring-opportunities.md) |
| 231+ | パターン検出共通化 | ⭐⭐⭐ | 中〜高 | 新パターン追加時 | [phase223-228-refactoring-opportunities.md](phase223-228-refactoring-opportunities.md) |

## 🎯 Phase 229: ConditionAlias 削除（推奨実施）

### 問題

**ConditionAlias は冗長！**

```rust
pub struct CarrierInfo {
    pub promoted_loopbodylocals: Vec<String>,      // ["digit_pos"]
    pub carriers: Vec<CarrierVar>,                  // [CarrierVar{name: "is_digit_pos", ...}]
    pub condition_aliases: Vec<ConditionAlias>,    // ← これは promoted_loopbodylocals + carriers から導出可能
}
```

### 解決策

**動的解決で十分！**

```rust
impl CarrierInfo {
    pub fn resolve_promoted_carrier(&self, old_name: &str) -> Option<&str> {
        if !self.promoted_loopbodylocals.contains(&old_name.to_string()) {
            return None;
        }
        let expected_name = format!("is_{}", old_name);
        self.carriers.iter()
            .find(|c| c.name == expected_name)
            .map(|c| c.name.as_str())
    }
}
```

### 効果

- **削減**: CarrierInfo フィールド 6 → 5
- **保守性**: データ整合性チェック 3箇所 → 1箇所
- **時間**: 1〜2時間
- **リスク**: 低

### 実装計画

📄 **[phase229-action-plan.md](phase229-action-plan.md)** - 詳細な実装手順とテスト計画

---

## 🔧 Phase 230: ExitLinePolicy Trait

### 問題

**ConditionOnly フィルタが4ファイルに分散！**

- `meta_collector.rs` - collection ロジック
- `reconnector.rs` (2箇所) - reconnect ロジック
- `instruction_rewriter.rs` - exit PHI ロジック

### 解決策

**Policy Trait で集約**

```rust
pub struct ExitLinePolicy;

impl ExitLinePolicy {
    pub fn should_collect(role: CarrierRole) -> bool {
        true  // ConditionOnly も collect（latch incoming 用）
    }

    pub fn should_reconnect(role: CarrierRole) -> bool {
        role == CarrierRole::LoopState  // LoopState のみ
    }

    pub fn should_create_exit_phi(role: CarrierRole) -> bool {
        role == CarrierRole::LoopState  // LoopState のみ
    }
}
```

### 効果

- **削減**: ~20行（重複 if 文の削減）
- **可読性**: 判断ロジックが1箇所に
- **拡張性**: 新しい CarrierRole 追加時に1箇所修正
- **時間**: 2〜3時間
- **リスク**: 中

---

## 💡 Phase 231+: パターン検出共通化

### 問題

**Trim と DigitPos Promoter に重複コード**

- `loop_body_carrier_promoter.rs` (658行)
- `loop_body_digitpos_promoter.rs` (713行)

**同じ関数が2箇所に存在**:
- `is_substring_method_call()` - 完全に同一
- 条件変数抽出ロジック - worklist 走査パターンが同じ

### 解決策

**LoopBodyLocalPattern Trait**

```rust
pub trait LoopBodyLocalPattern {
    fn pattern_name() -> &'static str;
    fn allowed_init_methods() -> &'static [&'static str];
    fn detect(...) -> Option<PromotionCandidate>;
}

impl LoopBodyLocalPattern for TrimPattern { ... }
impl LoopBodyLocalPattern for DigitPosPattern { ... }
```

### 効果

- **削減**: ~200行+（重複ロジック統合）
- **拡張性**: 新パターン追加が容易
- **テスト容易性**: 共通部分を単独テスト可能
- **時間**: 1日+
- **リスク**: 高

### 推奨時期

**新しいパターン追加時に検討** - 今は後回しでOK

---

## 📚 参考資料

### 成功例

**Phase 224: MethodCallLowerer 統一化** - 理想的な Box化パターン

- **Before**: loop_body_local_init.rs と condition_lowerer.rs に重複コード
- **After**: MethodCallLowerer Box に統一
- **成果**: ~200行削減、保守性向上、テスト容易性向上

**特徴**:
- Metadata-Driven（CoreMethodId ベース）
- Fail-Fast（whitelist にない method は即エラー）
- Context-Aware（for_init / for_condition で異なる whitelist）

### 設計ガイドライン

✅ **良い Box化**:
1. 単一責任
2. Metadata-Driven
3. Fail-Fast
4. Context-Aware
5. 独立テスト可能

❌ **避けるべき**:
1. 情報の重複
2. ロジックの分散
3. 型の冗長
4. 暗黙の依存

---

## 🧪 テスト戦略

### Phase 229 変更時

**Level 1: ビルド確認**
```bash
cargo build --release
```

**Level 2: 単体テスト**
```bash
cargo test --lib carrier_info
cargo test --lib loop_pattern_detection
```

**Level 3: パターンテスト**
```bash
cargo test --release test_mir_joinir_funcscanner_trim
cargo test --release test_loopbodylocal_digitpos
cargo test --release test_loop_with_break
```

**Level 4: 決定性テスト**
```bash
for i in 1 2 3; do
    echo "=== Run $i ==="
    cargo test --release test_loop_with_break 2>&1 | grep -E "ValueId|test result"
done
```

**Level 5: E2E テスト**
```bash
tools/smokes/v2/run.sh --profile quick --filter "loop_*"
cargo test --release --test '*'
```

---

## 📋 実装チェックリスト

Phase 229 実装時：

- [ ] CarrierInfo::resolve_promoted_carrier() 実装
- [ ] pattern2_with_break.rs の condition_aliases ループ削除
- [ ] loop_body_carrier_promoter.rs 修正
- [ ] loop_body_digitpos_promoter.rs 修正
- [ ] ConditionAlias 型削除
- [ ] pattern4_carrier_analyzer.rs 修正
- [ ] route_prep_pipeline.rs 修正
- [ ] ビルド確認
- [ ] 単体テスト
- [ ] パターンテスト
- [ ] 決定性テスト
- [ ] E2E テスト
- [ ] CURRENT_TASK.md 更新

---

**作成日**: 2025-12-10
**対象**: Phase 223-228 コードベース分析結果
**推奨次アクション**: Phase 229 ConditionAlias 削除の実施
