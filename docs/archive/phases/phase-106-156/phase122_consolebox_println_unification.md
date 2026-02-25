# Phase 122: ConsoleBox.println / log の統一（JSON v0 共通ルート）

⚠️ **Note**: このドキュメントは Phase 122 の実装記録です。
           統合的なガイドは [ConsoleBox 完全ガイド](consolebox_complete_guide.md) をご参照ください。

## 0. ゴール

- .hako 側の `ConsoleBox.println(...)` と、VM/Rust 側の `ConsoleBox.log(...)` を **構造的に同じルートに揃える**
- selfhost Stage-3 → JSON v0 → Rust VM の経路でも:
  - `ConsoleBox.println` がエラーにならず
  - 内部では `ConsoleBox.log` と同じスロットに正規化される
- **代表ケース** `apps/tests/esc_dirname_smoke.hako` を JoinIR Strict + selfhost 経路で green にする

---

## 1. スコープと非スコープ

### スコープ（今回やること）

1. **現状分析ドキュメント作成**: ConsoleBox の「言語API」と「VM実装API」のズレを整理
2. **TypeRegistry 修正**: `println` を `log` のエイリアス（slot 400）として追加
3. **ドキュメント更新**: console_box.rs / hako_logging_design.md / logging_policy.md
4. **selfhost 再検証**: esc_dirname_smoke.hako が selfhost Stage-3 + JoinIR Strict で通ることを確認
5. **hako_check 影響確認**: ConsoleBox.println の alias 化が hako_check に影響しないことを確認

### 非スコープ（今回はやらない）

- **ConsoleService 統合**: ConsoleBox と ConsoleService の統合（Phase 123+ で検討）
- **LoggerBox 統合**: ConsoleBox と LoggerBox の統合（Phase 123+ で検討）
- **パフォーマンス最適化**: println/log の実行速度改善（Phase 124+ で検討）

---

## 2. 設計方針（どこで揃えるか）

### 2.1 言語レベルの正解

**ConsoleBox の「公式 API」定義**:

| メソッド | 引数 | 役割 | VM slot |
|---------|-----|------|---------|
| `log(message)` | 1 | コアメソッド（標準出力） | 400 |
| `warn(message)` | 1 | 警告メッセージ | 401 |
| `error(message)` | 1 | エラーメッセージ | 402 |
| `clear()` | 0 | コンソールクリア | 403 |
| **`println(message)`** | 1 | **`log` のエイリアス（ユーザー向け sugar）** | **400** |

**設計決定**:
- `println` は `log` の完全なエイリアス
- ユーザー向けは `println` で書いても `log` で書いてもよい
- 内部実装上は同じ slot 400 を使う

### 2.2 正規化ポイント（どこで println→log を吸収するか）

**✅ Option A: type_registry.rs の CONSOLE_METHODS に println を追加** (採用)

**理由**:
- VM の TypeRegistry で alias を張るだけで、全経路に適用される
- JSON v0 / selfhost / 通常VM どの経路でも同じスロットを見る
- 正規化ポイントが一箇所に固定できる（保守性が高い）

**実装**:
```rust
const CONSOLE_METHODS: &[MethodEntry] = &[
    MethodEntry { name: "log",     arity: 1, slot: 400 },
    MethodEntry { name: "warn",    arity: 1, slot: 401 },
    MethodEntry { name: "error",   arity: 1, slot: 402 },
    MethodEntry { name: "clear",   arity: 0, slot: 403 },
    // Phase 122: println は log のエイリアス
    MethodEntry { name: "println", arity: 1, slot: 400 },
];
```

**❌ Option B: MIR/JSON 生成時に "println" → "log" に書き換え** (却下)

**理由**:
- Bridge が増えたときに再び散る
- 正規化ポイントが複数箇所になる（保守性が低い）

---

## 3. Task 1: 現状の API 実態を docs に固定

### 3.1 実装内容

**ファイル**: `docs/development/current/main/phase122_consolebox_println_unification.md`（本ドキュメント）

**記載内容**:

#### 現状の整理

**Phase 120 での観測結果**:
- `apps/tests/esc_dirname_smoke.hako` が selfhost Stage-3 + JoinIR Strict 経路で失敗
- エラーメッセージ: `Unknown method 'println' on ConsoleBox`

**原因分析**:

| 層 | 現状 | 問題 |
|----|------|------|
| **.hako サンプル** | `console.println("...")` 前提 | ✅ ユーザー向け API |
| **src/boxes/console_box.rs** | `log/warn/error/clear` のみ実装 | ❌ `println` 未実装 |
| **type_registry.rs** | `CONSOLE_METHODS` に `log/warn/error/clear` のみ | ❌ `println` 未登録 |
| **selfhost Stage-3 経路** | JSON v0 → VM で `println` を解決できない | ❌ エラー発生 |

**設計決定**:

- `ConsoleBox.println` を「`log` と同じ意味のユーザー向け sugar」と定義
- VM の TypeRegistry で `println` → slot 400（`log` と同じ）に正規化
- すべての経路（JSON v0 / selfhost / 通常VM）で一貫性を保つ

---

## 4. Task 2: TypeRegistry に println を alias として追加

### 4.1 実装内容

**ファイル**: `src/runtime/type_registry.rs`（修正）

**修正箇所**:

```rust
const CONSOLE_METHODS: &[MethodEntry] = &[
    MethodEntry { name: "log",     arity: 1, slot: 400 },
    MethodEntry { name: "warn",    arity: 1, slot: 401 },
    MethodEntry { name: "error",   arity: 1, slot: 402 },
    MethodEntry { name: "clear",   arity: 0, slot: 403 },
    // Phase 122: println は log のエイリアス
    // JSON v0/selfhost が println を吐いても log と同じスロットを使うための alias
    MethodEntry { name: "println", arity: 1, slot: 400 },
];
```

**コメント追加**:
- 「`println` は `log` の別名。JSON v0/selfhost が `println` を吐いても `log` と同じスロットを使うための alias」

### 4.2 core_boxes_design.md への追記

**ファイル**: `docs/development/current/main/core_boxes_design.md`（修正）

**追記内容**:

```markdown
## Section 18: Phase 122 - ConsoleBox.println / log 統一

### 概要

ConsoleBox の `println` メソッドを `log` のエイリアスとして VM レベルで正規化。
すべての経路（JSON v0 / selfhost / 通常VM）で一貫性を保つ。

### 設計

- **言語レベル**: `println(message)` は `log(message)` の完全なエイリアス
- **VM レベル**: `println` は slot 400（`log` と同じ）に正規化
- **正規化ポイント**: `src/runtime/type_registry.rs` の `CONSOLE_METHODS`

### 実装完了日

**Phase 122 実装完了日**: 2025-12-04（予定）
```

---

## 5. Task 3: ConsoleBox 実装ドキュメントの調整

### 5.1 実装内容

#### 5.1.1 console_box.rs のドキュメント更新

**ファイル**: `src/boxes/console_box.rs`（修正）

**修正箇所**:

```rust
//! ConsoleBox - コンソール出力ボックス
//!
//! ## 利用可能メソッド
//!
//! - `log(message)`: 標準出力にメッセージを出力
//! - `println(message)`: `log` のエイリアス（ユーザー向け sugar）
//! - `warn(message)`: 警告メッセージを出力
//! - `error(message)`: エラーメッセージを出力
//! - `clear()`: コンソールをクリア
//!
//! ## Phase 122: println / log の統一
//!
//! `println` は `log` の完全なエイリアスです。内部的には同じ slot 400 を使用します。
//! ユーザーコードでは `println` を使用することを推奨しますが、`log` も同様に動作します。
```

**実装オプション**:

**Option 1: Rust 側でラッパを追加**（完全統一）
```rust
impl ConsoleBox {
    /// Phase 122: println は log の別名
    pub fn println(&self, message: &str) {
        self.log(message);
    }
}
```

**Option 2: VM の alias に任せる**（最小実装）
- Rust 側では実装せず、VM の TypeRegistry に任せる
- docs のみで「`println` は `log` の別名」と明記

**推奨**: Option 1（Rust 側でもラッパを追加）
- 理由: Rust から直接 ConsoleBox を使う場合にも対応できる

#### 5.1.2 hako_logging_design.md への追記

**ファイル**: `docs/development/current/main/hako_logging_design.md`（修正）

**追記内容**:

```markdown
## ConsoleBox の使い方（Phase 122 更新）

### 基本パターン

```nyash
local console = new ConsoleBox()
console.println("Hello")  // 内部的には log と同じスロット
console.log("World")      // println と同じ動作
```

### ConsoleBox vs LoggerBox vs ConsoleService

- **ConsoleBox**: ユーザーコードで直接使用（`println` / `log`）
- **LoggerBox**: 構造化ログ・ログレベル管理
- **ConsoleService**: CLI/システム内部での出力（Ring0 経由）

### Phase 122 での統一

- `ConsoleBox.println` は `ConsoleBox.log` の完全なエイリアス
- VM の TypeRegistry で slot 400 に正規化される
- すべての経路（JSON v0 / selfhost / 通常VM）で一貫性を保つ
```

#### 5.1.3 logging_policy.md への追記

**ファイル**: `docs/development/current/main/logging_policy.md`（修正）

**追記内容**:

```markdown
## Phase 122: ConsoleBox.println / log の統一

### 使い分けガイドライン

| 用途 | 推奨 API | 理由 |
|------|---------|------|
| **selfhost / CLI** | `ConsoleService` / `console_println!` | Ring0 経由で安定 |
| **ユーザーコード** | `ConsoleBox.println` | ユーザー向け sugar |
| **内部実装** | `ConsoleBox.log` | VM レベルでは同じ |

### 正規化ルール

- `ConsoleBox.println` は VM の TypeRegistry で `ConsoleBox.log`（slot 400）に正規化される
- JSON v0 / selfhost / 通常VM のすべての経路で同じ動作を保証
- Rust から直接使用する場合も `println` / `log` の両方が使用可能
```

---

## 6. Task 4: selfhost / esc_dirname_smoke 再検証

### 6.1 実装内容

**ファイル/コマンド**:
- `tools/smokes/v2/profiles/integration/selfhost/phase120_stable_paths.sh`
- `docs/development/current/main/phase120_baseline_results.md`

**実行コマンド**:

```bash
# JoinIR Strict モードで selfhost 経路を再検証
NYASH_FEATURES=stage3 \
NYASH_USE_NY_COMPILER=1 \
NYASH_NY_COMPILER_EMIT_ONLY=1 \
NYASH_SELFHOST_KEEP_RAW=1 \
NYASH_JOINIR_STRICT=1 \
  ./tools/smokes/v2/profiles/integration/selfhost/phase120_stable_paths.sh
```

### 6.2 期待結果

| テストケース | Phase 120 | Phase 122（期待） |
|-------------|-----------|------------------|
| `peek_expr_block.hako` | ✅ 成功 | ✅ 成功 |
| `loop_min_while.hako` | ✅ 成功 | ✅ 成功 |
| `esc_dirname_smoke.hako` | ❌ `Unknown method 'println'` | ✅ **成功** |

**esc_dirname_smoke.hako の期待動作**:
- エラー `Unknown method 'println' on ConsoleBox` が消える
- 出力として esc_json / dirname の結果が正しく表示される

### 6.3 ドキュメント更新

**phase120_baseline_results.md への追記**:

```markdown
### 3. esc_dirname_smoke.hako

| 項目 | Phase 120 結果 | Phase 122 結果 |
|------|---------------|---------------|
| **実行結果** | ❌ エラー | ✅ **成功** |
| **エラーメッセージ** | Unknown method 'println' on ConsoleBox | （なし） |
| **修正内容** | - | Phase 122: TypeRegistry に println alias 追加 |
| **備考** | ConsoleBox.println 未実装 | println → log に正規化 |
```

**CURRENT_TASK.md への追記**:

```markdown
### 🎯 Phase 122: ConsoleBox.println / log 統一（完了）

- ✅ 現状分析ドキュメント作成: phase122_consolebox_println_unification.md
- ✅ TypeRegistry 修正: println を log のエイリアス（slot 400）として追加
- ✅ ConsoleBox 実装ドキュメント調整: console_box.rs / hako_logging_design.md / logging_policy.md
- ✅ selfhost 再検証: esc_dirname_smoke.hako が selfhost Stage-3 + JoinIR Strict で通ることを確認
- ✅ hako_check 影響確認: ConsoleBox.println の alias 化が hako_check に影響しないことを確認

**Phase 120 の問題解決**:
- ✅ esc_dirname_smoke.hako の `Unknown method 'println'` エラー解消

**次のステップ**: Phase 123（ConsoleService / LoggerBox 統合検討）
```

---

## 7. Task 5: hako_check / JoinIR に影響がないことを確認

### 7.1 実装内容

**ファイル**: `docs/development/current/main/phase121_hako_check_joinir_design.md`（確認・追記）

**確認事項**:

1. **hako_check が ConsoleBox を使用しているか確認**:
   ```bash
   rg "ConsoleBox" tools/hako_check/ --type hako
   rg "println\|log" tools/hako_check/ --type hako
   ```

2. **確認結果に応じて対応**:

   **ケース A: hako_check が ConsoleBox を使用している**
   - `phase121_hako_check_joinir_design.md` に追記:
     ```markdown
     ## Phase 122 での影響

     - ConsoleBox.println は log に正規化される（TypeRegistry レベル）
     - hako_check のログ出力設計: ConsoleBox.println / log の両方が使用可能
     - 動作に影響なし（VM の alias 機能で自動対応）
     ```

   **ケース B: hako_check が ConsoleBox を使用していない**
   - `phase121_hako_check_joinir_design.md` に追記:
     ```markdown
     ## Phase 122 での影響

     - hako_check は ConsoleBox を直接使用していない
     - ConsoleBox.println の alias 化は hako_check に影響なし
     ```

### 7.2 追加確認

**MirBuilder / JoinIR Lowering への影響確認**:

```bash
# MirBuilder が ConsoleBox.println を特別扱いしていないか確認
rg "println" src/mir/builder/ --type rust

# JoinIR Lowering への影響確認
rg "println" src/mir/join_ir/ --type rust
```

**期待結果**: どちらも特別扱いしていない（TypeRegistry に任せる設計）

---

## 8. 完成チェックリスト（Phase 122）

- [ ] phase122_consolebox_println_unification.md に現状と設計がまとまっている
- [ ] type_registry.rs の CONSOLE_METHODS に println alias が追加されている
- [ ] console_box.rs に println メソッドのラッパが追加されている（Option 1 採用時）
- [ ] console_box.rs / hako_logging_design.md / logging_policy.md に ConsoleBox.println / log の関係が明記されている
- [ ] apps/tests/esc_dirname_smoke.hako が selfhost Stage-3 + JoinIR Strict 経路で通る（旧エラーメッセージが消える）
- [ ] phase120_baseline_results.md が更新され、esc_dirname_smoke.hako の結果が ❌ → ✅ に変わっている
- [ ] CURRENT_TASK.md が更新され、「ConsoleBox.println 問題 resolved」となっている
- [ ] hako_check への影響が確認され、phase121_hako_check_joinir_design.md に記録されている
- [ ] ビルド・テスト全 PASS（cargo build --release && cargo test --release）

---

## 9. 設計原則（Phase 122 で確立）

### Alias First

```
【Phase 122 の哲学】
複数の名前を持つ API は、VM レベルで alias に統一する

Flow:
    ユーザーコード: ConsoleBox.println("...")
        ↓
    VM TypeRegistry: println → slot 400（log と同じ）
        ↓
    ConsoleBox 実装: log の実装が実行される
        ↓
    出力: 標準出力にメッセージ表示
```

### 正規化ポイントの一元化

**重要な約束**:
- **alias は TypeRegistry で管理**: VM レベルで一元管理
- **MirBuilder は関与しない**: 特別扱いなし
- **すべての経路で一貫**: JSON v0 / selfhost / 通常VM

### Phase 120 との連携

**Phase 120 の成果を活用**:
- Phase 120: selfhost 経路のベースライン確立 → 問題発見
- Phase 122: 問題の根本解決（TypeRegistry レベル） → ベースライン改善
- Phase 123+: 追加機能の統合検討

---

**Phase 122 指示書完成日**: 2025-12-04（Phase 120-121 完了直後）
Status: Historical
