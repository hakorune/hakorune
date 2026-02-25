# Phase 124: hako_check レガシー削除 & JoinIR 専用再構築

## 🎯 ゴール

hako_check の MIR 生成経路を **JoinIR 専用化** し、環境変数フラグに頼らずデフォルトで JoinIR 経路を使う。旧 PHI/MIR Builder 経路は hako_check に関しては完全削除。

```
現状（Phase 123）:  NYASH_HAKO_CHECK_JOINIR で 2パス選択可能
        ↓
目標（Phase 124）: JoinIR 一本専用化＆レガシー削除
```

## 📋 スコープ（やること・やらないこと）

### ✅ やること
1. docs の 2パス図を 1本（JoinIR 一本）に畳む
2. NYASH_HAKO_CHECK_JOINIR フラグを廃止
3. MIR Builder から hako_check 用 legacy lowering を削除
4. try_cf_if_joinir() / try_cf_loop_joinir() を JoinIR 専用に確定
5. テスト・CURRENT_TASK を JoinIR Only 前提に更新

### ❌ やらないこと
- JoinIR Lowerer 自体の仕様変更
- hako_check 診断ルール（HC001-031）のロジック変更

## 🏗️ 5 つのタスク

### Task 1: ドキュメントの 2パス図を 1本に畳む

**ファイル**:
- `docs/development/current/main/hako_check_design.md`
- `docs/development/current/main/phase121_hako_check_joinir_design.md`

**現在の図（Phase 123）**:
```
┌─ Legacy Path （NYASH_HAKO_CHECK_JOINIR=0）
│  .hako → Parse → MIR Builder (legacy) → VM
│
└─ JoinIR Path （NYASH_HAKO_CHECK_JOINIR=1）
   .hako → Parse → MIR Builder → JoinIR Lowerer → VM
```

**目標の図（Phase 124）**:
```
JoinIR Only:
.hako → Parse → AST → MIR Builder (JoinIR if/loop lowering) → MIR → VM
```

**更新内容**:

```markdown
## hako_check 実行フロー（Phase 124 JoinIR 専用化）

### アーキテクチャ
```
Input: .hako script
       ↓
Tokenize & Parse → AST
       ↓
MIR Builder (JoinIR lowering for if/loop)
       ↓
MIR (SSA form with PHI nodes)
       ↓
VM Interpreter
       ↓
Diagnostic Output
```

### 設計の進化
- **Phase 121**: JoinIR 統合設計
- **Phase 123**: 環境変数フラグで 2パス選択可能に
- **Phase 124**: JoinIR 一本化＆レガシー削除
```

**Phase 121 docs に追加**:

```markdown
## Phase 122–124 実装完了サマリー

### Phase 123: 環境変数スイッチ導入
- NYASH_HAKO_CHECK_JOINIR で 2パス選択可能に

### Phase 124: JoinIR 専用化＆レガシー削除
- NYASH_HAKO_CHECK_JOINIR を廃止
- MIR Builder から legacy if/loop lowering を削除
- hako_check は JoinIR 一本化
- 旧経路は他モード用途から削除（v確認）

### 成果
✅ hako_check + selfhost Stage-3 が JoinIR 統一パイプラインで動作
✅ ドキュメント・実装・テストが JoinIR 前提に統一
```

### Task 2: NYASH_HAKO_CHECK_JOINIR フラグの完全削除

**方針**: パターン A（完全削除）を採用

**削除対象**:

1. **src/config/env/hako_check.rs**:
   ```rust
   // ❌ 以下を完全削除
   pub fn hako_check_joinir_enabled() -> bool {
       matches!(
           std::env::var("NYASH_HAKO_CHECK_JOINIR").as_deref(),
           Ok("1") | Ok("true")
       )
   }
   ```

2. **src/config/env.rs**:
   - `pub mod hako_check;` を削除 or hako_check モジュール自体を削除

3. **docs/reference/environment-variables.md**:
   - NYASH_HAKO_CHECK_JOINIR セクション削除

**記録方法**:

```markdown
### NYASH_HAKO_CHECK_JOINIR （削除済み - Phase 124）

このフラグは Phase 123 で導入され、hako_check の legacy/JoinIR 2パス切り替え用でした。
Phase 124 で JoinIR 一本化により廃止。

**理由**: hako_check は現在 JoinIR 専用のため、環境変数による選択が不要。
**備考**: 他の用途で legacy if/loop lowering が必要な場合は NYASH_LEGACY_PHI=1 などで対応。
```

### Task 3: MIR Builder から legacy path を削除

**ファイル**:
- `src/mir/builder/if_form.rs` (cf_if())
- `src/mir/builder/control_flow.rs` (cf_loop() など)

**現在の分岐（Phase 123）**:
```rust
pub fn cf_if(&mut self, ...) -> Result<...> {
    if hako_check_joinir_enabled() {
        // JoinIR 経路
        self.try_cf_if_joinir(...)
    } else {
        // Legacy 経路
        self.lower_if_form(...)
    }
}
```

**Phase 124 での処理**:

#### Option 1: hako_check モードの判定で分ける

```rust
pub fn cf_if(&mut self, ...) -> Result<...> {
    if self.is_hako_check_mode() {
        // hako_check は常に JoinIR
        self.try_cf_if_joinir(...)?  // Fail-Fast: error ならエラー確定
    } else {
        // 他のモード（あれば）は legacy をキープ
        self.lower_if_form(...)
    }
}
```

#### Option 2: 完全一本化（推奨）

もし他のモードでも legacy lowering が不要なら：

```rust
pub fn cf_if(&mut self, ...) -> Result<...> {
    // 常に JoinIR（全モード統一）
    self.try_cf_if_joinir(...)  // Fail-Fast
}
```

**確認方針**:
1. hako_check 経路以外で lower_if_form() を使っている箇所を検索
2. 使っていなければ削除
3. 使っていれば、そのモード用のフラグで分岐を維持

**フォールバック処理の廃止**:
- Phase 123 の try_cf_if_joinir() に「JoinIR 不可 → legacy fallback」があれば削除
- Fail-Fast 原則に従い、JoinIR が適用できないパターンはエラーにする

```rust
// ❌ 削除対象（フォールバック）
if use_joinir {
    match self.try_cf_if_joinir() {
        Ok(phi_val) => return Ok(phi_val),
        Err(_) => {
            // fallback to legacy ← これを削除
            return self.lower_if_form();
        }
    }
}

// ✅ Phase 124 版（Fail-Fast）
if self.is_hako_check_mode() {
    self.try_cf_if_joinir()?  // Error は即座に伝播
}
```

### Task 4: テスト更新（JoinIR Only を正とする）

**ファイル**:
- `local_tests/phase123_*.hako` (4ケース)
- `tools/smokes/v2/profiles/integration/hako_check_joinir.sh`

**テストケース修正**:

1. **テスト実行方法を簡略化**:
   ```bash
   # Phase 123 形式（削除対象）
   NYASH_HAKO_CHECK_JOINIR=0 ./target/release/nyash test.hako  # ❌ 削除
   NYASH_HAKO_CHECK_JOINIR=1 ./target/release/nyash test.hako  # ← これだけ残す

   # Phase 124 形式（推奨）
   ./target/release/nyash test.hako  # JoinIR 一本で動く前提
   ```

2. **smoke スクリプト（hako_check_joinir.sh）の修正**:
   ```bash
   #!/bin/bash
   # Phase 124: hako_check JoinIR Only テスト

   test_cases=(
       "phase123_simple_if.hako"
       "phase123_nested_if.hako"
       "phase123_while_loop.hako"
       "phase123_if_in_loop.hako"
   )

   echo "=== hako_check JoinIR Only Mode ==="
   for case in "${test_cases[@]}"; do
       # 環境変数なし（デフォルト JoinIR）で実行
       ./target/release/nyash "local_tests/$case" || exit 1
       echo "✓ $case"
   done
   echo "All tests PASSED"
   ```

3. **テスト名・コメント更新**:
   - ファイル名: `phase123_*.hako` → `hako_check_joinir_*.hako`（または keep）
   - コメント: "Phase 123 JoinIR test" → "Phase 124 JoinIR Only (stable)"

4. **Phase 120/121 docs に追記**:
   ```markdown
   ### hako_check 代表パスの JoinIR 統一化（Phase 124）

   hako_check の stable paths すべてが JoinIR 専用パイプラインで動作確認済み：
   - ✅ simple_if.hako
   - ✅ nested_if.hako
   - ✅ while_loop.hako
   - ✅ if_in_loop.hako

   （legacy 経路は削除済み）
   ```

### Task 5: docs / CURRENT_TASK を第2章クローズ仕様にする

**ファイル**:
- `docs/development/current/main/hako_check_design.md`
- `docs/development/current/main/phase121_hako_check_joinir_design.md`
- `CURRENT_TASK.md`

**hako_check_design.md に追加**:

```markdown
## Phase 124: JoinIR 専用化＆レガシー削除（完了）

### 変更内容
- ✅ NYASH_HAKO_CHECK_JOINIR フラグを廃止
- ✅ MIR Builder から legacy if/loop lowering を削除
- ✅ hako_check は JoinIR 一本化
- ✅ Fail-Fast 原則でエラーハンドリングを統一

### 実行フロー（最終版）
```
.hako script
    ↓
Parse & Tokenize → AST
    ↓
MIR Builder (JoinIR lowering for if/loop)
    ↓
MIR with PHI nodes
    ↓
VM Interpreter
    ↓
Diagnostic Report
```

### テスト状況
- 代表 4 ケース: 全て JoinIR Only で PASS
- legacy 経路: 削除確認済み
```

**Phase 121 docs に "Chapter Close" セクション追加**:

```markdown
## JoinIR/selfhost 第2章 完了

### 設計フェーズ（Phase 121）
- If/Loop の JoinIR 統合設計
- hako_check 代表パス洗出し

### 実装フェーズ（Phase 122-124）
- Phase 122: println/log 統一
- Phase 123: hako_check に JoinIR スイッチ導入
- Phase 124: JoinIR 専用化＆レガシー削除

### 成果
✅ hako_check は JoinIR 一本化
✅ selfhost Stage-3 + hako_check が統一パイプラインで動作
✅ ドキュメント・実装・テストが統一

### 次章予告
次は selfhost Stage-4（高度なパターン対応）への準備を進める。
```

**CURRENT_TASK.md に追加**:

```markdown
### Phase 124: hako_check レガシー削除 & JoinIR 専用再構築 ✅

**完了内容**:
- NYASH_HAKO_CHECK_JOINIR フラグを廃止
- MIR Builder から legacy if/loop lowering を削除（hako_check用）
- hako_check は JoinIR 一本化
- 代表テスト 4 ケースで JoinIR Only 動作確認

**テスト結果**:
- 4/4 PASS（全て JoinIR 専用）

**成果**:
- JoinIR/selfhost 第2章 完了
- hako_check + selfhost Stage-3 が統一パイプラインで動作
- ドキュメント・実装・テストが統一

**次フェーズ**: selfhost Stage-4 拡張 or 次の大型改善へ
```

## ✅ 完成チェックリスト

- [ ] hako_check_design.md の図を 2パス → 1本に更新
- [ ] phase121_hako_check_joinir_design.md に "Phase 122-124 実装完了" セクション追加
- [ ] NYASH_HAKO_CHECK_JOINIR フラグを src/config/env/hako_check.rs から削除
- [ ] docs/reference/environment-variables.md から NYASH_HAKO_CHECK_JOINIR を削除
- [ ] MIR Builder の cf_if() / cf_loop() から NYASH_HAKO_CHECK_JOINIR 分岐を削除
- [ ] try_cf_if_joinir() / try_cf_loop_joinir() のフォールバック処理を削除（Fail-Fast）
- [ ] hako_check モード判定がある場合、legacy path 削除後も分岐を確認
- [ ] local_tests/phase123_*.hako テストを環境変数なしで実行確認
- [ ] hako_check_joinir.sh を更新（環境変数削除、JoinIR Only 前提）
- [ ] テスト実行: 4/4 PASS 確認
- [ ] CURRENT_TASK.md に Phase 124 完了を記録
- [ ] ビルドエラーなし（Zero errors）

## 所要時間

**2.5時間程度**

- Task 1 (ドキュメント更新): 30分
- Task 2 (フラグ削除): 20分
- Task 3 (legacy path 削除): 45分
- Task 4 (テスト更新): 30分
- Task 5 (最終ドキュメント): 15分

## 次のステップ

**selfhost Stage-4 拡張** または **次の大型改善フェーズ**

JoinIR/selfhost 第2章が完了し、hako_check + selfhost が統一パイプラインで動作。
今後はさらに複雑なパターン対応やパフォーマンス最適化へ進める準備が整った。

---

**進捗**:
- ✅ Phase 122-126: ConsoleBox 改善・統合
- ✅ Phase 123 proper: hako_check JoinIR 基盤構築
- 🎯 Phase 124: hako_check レガシー削除 & JoinIR 専用化 ← **現在のフェーズ**
- 📋 次: selfhost Stage-4 or 次大型フェーズ
Status: Historical
