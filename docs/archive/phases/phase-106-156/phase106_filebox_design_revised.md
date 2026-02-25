# Phase 106: FileBox provider_lock 整理 & Fail-Fast 強化（案B統合版）

## 0. ゴール

- FileBox を「selfhost/通常ランタイムでは事実上必須」として適切に扱う
- FileBox provider 未登録を起動時 or 最初の利用時に必ず検知して Fail-Fast する
- **「必須」概念は CoreBoxId に集約（案B採用）** - provider_lock はシンプルに保つ
- Ring0.FsApi との統合（案C）は Phase 107+ で実施

---

## 1. 現状整理（前提）

### 1.1 実装状況

| 層 | 位置 | 状態 |
|----|-----|------|
| Ring0 | src/runtime/ring0/traits.rs | FsApi trait：read/write/stat 定義済み |
| provider_lock | src/runtime/provider_lock.rs | OnceLock<Arc<dyn FileIo>>：シンプルな登録機構 |
| FileBox | src/boxes/file/mod.rs | provider_lock::get_filebox_provider() 直接呼び出し |
| CoreBoxId | src/runtime/core_box_ids.rs | **is_core_required() と category() が不一致** |

### 1.2 課題：概念の分散

現在「必須」概念が複数箇所に散在：
- `CoreBoxId.is_core_required()` → File を含める
- `CoreBoxId.category()` → File は CoreOptional のまま
- `provider_lock` → 「必須」概念を持たない

**問題**: 分散すると一貫性を保つのが難しい（設計がぼやける）

### 1.3 解決策（案B採用）

**原則**: 「必須」判定は CoreBoxId に一本化する
- provider_lock: 「FileBox provider を登録・読む」だけ（シンプル）
- CoreBoxId: 「File が必須かどうか」を決定する窓口に
- profile パターン: selfhost/default では必須、minimal/no-fs では optional

### 1.4 Phase 107 統合完了（2025-12-03）

**Ring0.FsApi 統合完了**:
- ✅ Ring0FsFileIo 実装追加（src/providers/ring1/file/ring0_fs_fileio.rs）
- ✅ provider_lock に init_default_filebox_provider() 追加
- ✅ PluginHost.with_core_from_registry_optional で自動登録
- ✅ Phase 106 の MissingService チェックは引き続き有効（Fail-Fast 維持）

**Phase 107 の効果**:
- 標準パスで FileBox provider が自動登録される
- MissingService エラーは基本的に起きない（プラグイン未登録時も default で補完）
- プラグイン優先原則は維持（プラグインが先に登録すれば default は使われない）

---

## 2. Task 1: CoreBoxId を修正（カテゴリ統一）

### 2.1 修正内容

ファイル: `src/runtime/core_box_ids.rs`

#### 現状の不整合

```rust
// L112-115
pub fn is_core_required(&self) -> bool {
    matches!(self, String | Integer | Bool | Array | Map | Console | File)  // ← File あり
}

// L118-125
pub fn category(&self) -> CoreBoxCategory {
    match self {
        String | Integer | Bool | Array | Map | Console => CoreBoxCategory::CoreRequired,
        Float | Null | File | Path | ... => CoreBoxCategory::CoreOptional,  // ← File がここに矛盾
    }
}
```

#### 修正方針

`category()` の分岐を修正して両者を統一：

```rust
pub fn category(&self) -> CoreBoxCategory {
    match self {
        // Phase 106: File を CoreRequired 側に移動（selfhost/通常ランタイムでは必須）
        String | Integer | Bool | Array | Map | Console | File => CoreBoxCategory::CoreRequired,
        Float | Null | Path | Regex | Math | Time | Json | Toml => CoreBoxCategory::CoreOptional,
        Function | Result | Method | Missing => CoreBoxCategory::Special,
    }
}
```

#### テスト更新（existing test を修正）

L367 のテストで `CoreBoxId::File.category()` の期待値を修正：

```rust
#[test]
fn test_core_box_id_category() {
    assert_eq!(CoreBoxId::String.category(), CoreBoxCategory::CoreRequired);
    // Phase 106: File の分類を修正
    assert_eq!(CoreBoxId::File.category(), CoreBoxCategory::CoreRequired);  // ← 修正
    assert_eq!(CoreBoxId::Function.category(), CoreBoxCategory::Special);
}
```

#### コメント追加（現在の intent を明示）

L108-115 のコメントを更新：

```rust
/// Phase 106: core_required チェック
///
/// FileBox は Phase 85 では core_optional として分類していたが、
/// selfhost/通常ランタイムでは事実上必須（ログ・ツール・ハコチェック等で常用）
/// であることが明確になったため、「core_required 相当」として扱う設計に統一した。
///
/// **設計原則**:
/// - 必須判定は CoreBoxId に一本化（provider_lock は「登録・読む」だけ）
/// - 将来 minimal/no-fs プロファイルを導入する場合は、ここで profile パラメータを追加可能
pub fn is_core_required(&self) -> bool {
    matches!(self, String | Integer | Bool | Array | Map | Console | File)
}
```

---

## 3. Task 2: provider_lock を単純化（SSOT 原則）

### 3.1 API の状態

ファイル: `src/runtime/provider_lock.rs`

#### 現状（変更不要な部分）

```rust
pub fn set_filebox_provider(provider: Arc<dyn FileIo>) -> Result<(), String>
pub fn get_filebox_provider() -> Option<&'static Arc<dyn FileIo>>
pub fn get_filebox_caps() -> Option<FileCaps>
```

**Decision**: これらの API はそのまま保つ（シンプルで良い）

#### 削除しない理由

- provider_lock の責務は「登録・読む」だけ
- 「必須かどうか」の判定は CoreBoxId の責任
- 層分離が明確になる

### 3.2 get_filebox_provider_strict() は不要

**削除理由**:
- 「Provider 未登録時エラー」は provider_lock の責任ではない
- その判定は、CoreBoxId が「必須」と言ったあとで、呼び出し側が処理すべき
- provider_lock は Option を返すだけで十分

---

## 4. Task 3: FileBox 側から provider_lock を呼び出し（既存パターン継続）

### 4.1 修正内容

ファイル: `src/boxes/file/mod.rs`

#### 現状（OK、変更不要）

L47, L63 の呼び出しはそのまま：

```rust
pub fn new() -> Self {
    FileBox {
        provider: provider_lock::get_filebox_provider().cloned(),
        path: String::new(),
        base: BoxBase::new(),
    }
}

pub fn open(path: &str) -> Result<Self, String> {
    let provider = provider_lock::get_filebox_provider()
        .ok_or("FileBox provider not initialized")?
        .clone();
    // ...
}
```

#### コメント追加（責務明示）

L5 付近にコメント追加：

```rust
// SSOT: FileBox は「FileIo provider を常に経由する」（provider_lock に一元化）。
// provider の有無・必須/optional の判定は provider_lock/CoreBoxId の責務で、
// FileBox 実装内では生の環境変数や静的状態を見ない設計。
```

---

## 5. Task 4: 起動時に FileBox provider 登録を必ず確保（Fail-Fast）

### 5.1 実装内容（概要）

ファイル: `src/runtime/plugin_host.rs`

- `CoreBoxId::is_core_required()` / `CoreServices::required_ids()` を用いて、「必須 Box は registry に型定義が存在すること」を起動時にチェック。
- FileBox については CoreRequired 側に寄せた上で、「FileBox provider が登録されていない場合は CoreInitError::MissingService で fail-fast」するロジックを追加。
- 具体的なコードは実装側に委ね、ここでは責務分離の方針のみを記録する。

#### テスト追加

```rust
#[test]
fn test_with_core_from_registry_filebox_required() {
    // Phase 106: FileBox provider なし → エラー
    let ring0 = Arc::new(default_ring0());
    let registry = UnifiedBoxRegistry::with_env_policy();

    // provider_lock を初期化せず（呼び出さず）
    // provider が無い状態で with_core_from_registry() を呼ぶ

    let result = PluginHost::with_core_from_registry(ring0, &registry);
    assert!(result.is_err());

    if let Err(CoreInitError::MissingService { box_id, .. }) = result {
        assert_eq!(box_id, CoreBoxId::File);
    } else {
        panic!("Expected MissingService error for FileBox");
    }
}
```

### 5.2 selfhost/代表ランナーでの provider 登録

ファイル: `src/runner/selfhost.rs` 等

#### パターン: 起動時に provider を登録

```rust
pub fn run_selfhost(config: &Config) -> Result<(), Error> {
    // Ring0 初期化
    let ring0 = get_global_ring0();

    // FileBox provider 登録（必須なので start-up で必ず実施）
    // 実装: builtin_factory::register_filebox_provider(&ring0)?;
    // または plugin loader から自動ロード

    // PluginHost 初期化（FileBox provider チェック含む）
    let plugin_host = initialize_runtime(ring0)?;
    plugin_host.ensure_core_initialized();

    // 以降処理...
}
```

**Fail-Fast 保証**:
- provider が未登録 → `with_core_from_registry()` で即座にエラー
- CoreInitError::MissingService で明示的に失敗
- アプリケーションが不完全な状態で先に進まない

---

## 6. Task 5: ドキュメント更新（設計の最終確認）

### 6.1 core_boxes_design.md 更新

ファイル: `docs/development/current/main/core_boxes_design.md`

#### Section 5.3 修正（FileBox 再分類）

現在の記述（L230-246）は正しいが、一文を追加：

```markdown
### 5.3 Phase 85 との関係（FileBox 再分類）

[...]

現行の分類は次の通り：
- **core_required (7個)**: StringBox, IntegerBox, BoolBox, ArrayBox, MapBox, ConsoleBox, FileBox
- **core_optional (8個)**: FloatBox, NullBox, PathBox, RegexBox, MathBox, TimeBox, JsonBox, TomlBox
- **特殊型 (4個)**: FunctionBox, ResultBox, MethodBox, MissingBox

## Phase 106: 設計統一（案B）

### 責務分離原則

- **CoreBoxId**: 「必須かどうか」の判定（is_core_required() / category()）
  - selfhost/default では File が必須
  - 将来 minimal/no-fs プロファイルでは optional に変更可能
- **provider_lock**: 「FileBox provider を登録・読む」のみ（シンプルなロック機構）
- **PluginHost**: startup 時に CoreBoxId.is_core_required() で provider をチェック
  - 未登録なら CoreInitError::MissingService で fail-fast

### Ring0.FsApi との関係（Phase 107 延期）

Ring0.FsApi（write 能力あり）と FileIo trait（read-only）の統合は、
Phase 107+ で実施予定。現在は概念を分離したまま。

（理由: Phase 106 は provider_lock 整理に専念し、FsApi 統合は別 phase で）
```

### 6.2 ring0-inventory.md 補足（Option）

理想的には ring0-inventory.md に以下を追加（但し省略可）：

```markdown
### FileBox provider registration

- Phase 106 で provider_lock の整理完了
- startup 時に CoreBoxId::File.is_core_required() でチェック
- 将来の Ring0.FsApi 統合（Phase 107）に向けて概念分離
```

---

## 7. 実装チェックリスト

Phase 106 完了とみなす条件：

- [ ] CoreBoxId::category() の File を CoreRequired 側に移動
- [ ] CoreBoxId テスト更新（L367 の期待値修正）
- [ ] CoreBoxId コメント更新（Phase 106 intent 明示）
- [ ] provider_lock API はそのまま保つ（get_filebox_provider_strict() 追加しない）
- [ ] FileBox コメント追加（SSOT 原則を明示）
- [ ] PluginHost.with_core_from_registry() に FileBox provider チェック追加
- [ ] PluginHost テスト追加（FileBox provider missing case）
- [ ] selfhost 等の起動パスで provider 登録確認
- [ ] core_boxes_design.md Section 5.3 + Phase 106 セクション追加
- [ ] ビルド成功・テスト全PASS確認

---

## 8. 設計原則（Phase 106 で確立）

### 責務分離が明確

```
層                  責務                         概念
─────────────────────────────────────────────────────────
CoreBoxId          「必須かどうか」判定        is_core_required()/category()
provider_lock      「登録・読む」のみ          get_filebox_provider()
PluginHost         startup チェック            with_core_from_registry() で検証
FileBox            provider を通す              provider_lock 経由で呼び出し
```

### 将来への足がかり

**Phase 107 で Ring0.FsApi 統合 (案C)**:
- FileIo を FsApi wrapper に
- Ring0 レベルで read/write 能力を統一
- provider_lock からの参照を Ring0.fs に変更

**現在 Phase 106** では「概念をきれいに分離」し、Phase 107 の統合に備える。

---

**指示書作成日**: 2025-12-03（案B統一版）
Status: Historical
