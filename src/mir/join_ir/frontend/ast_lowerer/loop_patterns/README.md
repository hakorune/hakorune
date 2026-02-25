# Loop Patterns - JoinIR Frontend ループパターン処理層

## 📋 この層の責務

**JSON v0 のループ body を、LoopFrontendBinding が渡してきた LoopPattern に従って JoinIR 命令列に変換する**

- LoopFrontendBinding から受け取った `LoopPattern` enum に基づいて適切な lowering 処理を選択
- ループ body の JSON AST を JoinIR 命令（Jump/Call/Select/MethodCall/ConditionalMethodCall 等）に変換
- 各パターンに特化した最適な JoinIR 構造を生成（3関数構造: entry/loop_step/k_exit）

## 🚫 やらないこと（重要！）

### ❌ 関数名ベースの判定
- **理由**: それは LoopFrontendBinding 層の責務
- **例**: `"simple"` や `"filter"` という関数名で分岐する処理は**書かない**
- **正しい設計**: LoopPattern enum を受け取って、enum の variant で分岐

### ❌ Box 名・メソッド名で意味論を変える
- **理由**: それは JoinIR→MIR Bridge / VM 側の責務
- **例**: `ArrayExtBox.filter` だから特別処理、という判定は**書かない**
- **正しい設計**: JSON の "type" フィールド（Var/Method/Call等）だけで判定

### ❌ AST 構造に踏み込みすぎる
- **理由**: JSON v0 の "type" フィールドで十分判定できる
- **例**: "kind" フィールドを見て詳細な AST ノード型を判定する処理は**書かない**
- **正しい設計**: "type": "Method" / "Var" / "Int" 等だけで処理

## 🏗️ 設計原則

### 1. パターン = 1箱 = 1責務

各 LoopPattern の lowering 処理は、独立したモジュール（ファイル）に分離：

```
loop_patterns/
├── filter.rs       # Filter パターン: pred が true のときだけ push
├── print_tokens.rs # PrintTokens パターン: token を順番に print
├── map.rs          # Map パターン（未実装）
├── reduce.rs       # Reduce パターン（未実装）
└── simple.rs       # Simple パターン: 汎用ループ
```

**各ファイルの責務を 1 行で表現**：
- `filter.rs`: 「pred が true のときだけ push するループを ConditionalMethodCall に落とす」
- `print_tokens.rs`: 「token を順番に取り出して print するループを Jump/Call/MethodCall に落とす」

### 2. 共通処理は最小限

- JSON パース、ExtractCtx 初期化、3関数構造生成の**本当に共通な部分だけ**をヘルパー化
- パターン固有のロジックは各モジュールに閉じる

### 3. エラーハンドリング

- 複雑すぎるパターンは素直に `Err(UnimplementedPattern)` を返す
- フェーズを分けて段階的に拡張しやすくする

### 4. trait による統一インターフェース

```rust
pub trait LoopPatternLowerer {
    fn lower(
        &self,
        lowerer: &mut AstToJoinIrLowerer,
        program_json: &serde_json::Value,
    ) -> Result<JoinModule, LoweringError>;
}
```

## 📊 LoopPattern の分類

### ユースケースベースの分類（Phase 55/56 実装済み）

| Pattern | 実装状況 | 責務 | 代表関数 |
|---------|---------|------|---------|
| **PrintTokens** | ✅ Phase 55 | token を順番に print | `JsonTokenizer.print_tokens` |
| **Filter** | ✅ Phase 56 | pred が true のときだけ push | `ArrayExtBox.filter` |
| **Map** | 🔜 Phase 57+ | 各要素を変換して新配列作成 | `ArrayExtBox.map` |
| **Reduce** | 🔜 Phase 58+ | 累積計算（fold） | `ArrayExtBox.reduce` |
| **Simple** | ✅ Phase 34 | 汎用ループ（上記以外） | 各種ループ |

### 制御構造ベースの分類（参考）

実際の実装では**ユースケース優先**だが、内部的には制御構造も考慮：

- **Simple**: break/continue なし
- **Break**: 早期 return ループ
- **Continue**: 条件付きスキップループ
- **Mixed**: break + continue 両方（未実装）

## 🔄 データフロー

```
1. LoopFrontendBinding 層
   ↓ (関数名ベース判定)
   LoopPattern enum を決定

2. loop_patterns.rs (ディスパッチ箱)
   ↓ (enum の variant で分岐)
   適切な lowering モジュールを選択

3. 各 lowering モジュール (filter.rs 等)
   ↓ (JSON "type" フィールドで判定)
   JoinIR 命令列を生成

4. JoinIR→MIR Bridge
   ↓ (Box 名・メソッド名で最適化)
   MIR 命令に変換
```

## 🎯 今後の拡張

新しいループパターンを追加する手順：

1. `LoopPattern` enum に variant 追加
2. 新しい lowering モジュール作成（例: `map.rs`）
3. 責務を 1 行で定義
4. `trait LoopPatternLowerer` を実装
5. `loop_patterns.rs` のディスパッチに 1 行追加

**設計思想**: 各パターンが独立した箱なので、追加・削除・置き換えが容易

## 📦 現在の状態と移行計画

### 現在の状態（Phase P2 完了）

```
loop_patterns/           # 新モジュール構造（将来の呼び出し用）
├── mod.rs              # LoopPattern enum + ディスパッチ
├── common.rs           # 共通処理（parse/ctx/entry/k_exit 生成）
├── filter.rs           # Filter パターン（→simple に委譲）
├── print_tokens.rs     # PrintTokens パターン（→simple に委譲）
└── simple.rs           # Simple パターン（実装済み）

loop_patterns_old.rs     # 現行コード（関数名ベース判定を含む）
```

### 移行計画（Future Work）

1. **LoopFrontendBinding 層の作成** (Phase P3+ 予定)
   - 関数名 → LoopPattern enum の変換を担当
   - `lower_program_json()` から呼び出し

2. **loop_patterns_old.rs の廃止**
   - Break/Continue パターンも新モジュールに移行後
   - `loop_patterns_old.rs` を削除

3. **完全移行後の呼び出し構造**
   ```rust
   // LoopFrontendBinding 層（新設）
   let pattern = detect_loop_pattern(func_name);

   // loop_patterns 層（新モジュール）
   loop_patterns::lower_loop_with_pattern(pattern, lowerer, json)
   ```

---

**Phase**: P2 (Loop Pattern 箱化モジュール化)
**作成日**: 2025-11-29
**原則**: 関数名判定は Binding 層、Box名判定は Bridge/VM 層、この層は LoopPattern だけを見る
