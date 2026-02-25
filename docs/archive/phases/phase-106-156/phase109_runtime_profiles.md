# Phase 109: minimal/no-fs プロファイル設計（FileBox optional モード）

## 0. ゴール

- Phase 107-108 で実装完了した FileBox/Ring0.FsApi パイプラインを、**RuntimeProfile システムで条件付き有効化** する
- selfhost/standard では FileBox が core_required、minimal/no-fs では optional に動的に切り替え可能にする
- **Fail-Fast 原則を維持**：required な場合は初期化時にエラー、optional な場合は黙って無効化

---

## 1. スコープと非スコープ

### スコープ（今回やること）

- RuntimeProfile enum 定義（Default, NoFs）+ phase 108 系 統合
- CoreBoxId に `is_required_in(profile: &RuntimeProfile) -> bool` ヘルパー追加
- PluginHost に profile-aware 初期化ロジック追加
- no-fs profile での FileBox provider チェック（missing OK、Err は "disabled for this profile"）
- ドキュメント + テスト追加

### 非スコープ（今回はやらない）

- 実際の TestMock/Sandbox/ReadOnly/Embedded プロファイル実装（Phase 110 以降で検討）
- profile ごとのプラグイン自動フィルタリング（手動制御に）
- Ring0 service registry 統一化（Phase 112 候補）

---

## 2. Task 1: RuntimeProfile enum + is_required_in() ヘルパー

### 2.1 実装内容

**ファイル**:
- `src/runtime/runtime_profile.rs`（新規）
- `src/runtime/core_box_ids.rs`（修正）

### 2.2 RuntimeProfile 定義

```rust
// src/runtime/runtime_profile.rs

/// Phase 109: RuntimeProfile
///
/// FileBox（およびその他オプションサービス）の有効/無効を制御する。
///
/// - Default: selfhost/standard - ほぼすべてのサービス有効
/// - NoFs: 最小ランタイム - FileBox/Regex/Time 等をスキップ
///
/// 拡張予定：TestMock（テスト用）, Sandbox（サンドボックス）, ReadOnly（読み取り専用）, Embedded（組み込み）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeProfile {
    /// Standard runtime (selfhost/default)
    Default,
    /// Minimal runtime without FileSystem
    NoFs,
}

impl RuntimeProfile {
    /// str から RuntimeProfile を取得
    pub fn from_env() -> Self {
        match std::env::var("NYASH_RUNTIME_PROFILE").as_deref() {
            Ok("no-fs") | Ok("nofs") => RuntimeProfile::NoFs,
            _ => RuntimeProfile::Default,
        }
    }

    /// デバッグ出力
    pub fn name(&self) -> &'static str {
        match self {
            RuntimeProfile::Default => "Default",
            RuntimeProfile::NoFs => "NoFs",
        }
    }
}
```

### 2.3 CoreBoxId に is_required_in() 追加

```rust
// src/runtime/core_box_ids.rs - impl CoreBoxId ブロック内に追加

/// Phase 109: profile-aware required チェック
///
/// - Default: Phase 106 の is_core_required() と同じ（FileBox required）
/// - NoFs: FileBox は optional に（その他 core_required は維持）
pub fn is_required_in(&self, profile: &RuntimeProfile) -> bool {
    use CoreBoxId::*;
    let core_required = matches!(self, String | Integer | Bool | Array | Map | Console);

    match profile {
        RuntimeProfile::Default => {
            // Phase 106: File を実質必須扱い
            self.is_core_required()
        }
        RuntimeProfile::NoFs => {
            // File 以外は core_required と同じ
            core_required
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_core_box_id_is_required_in_default() {
        let profile = RuntimeProfile::Default;
        assert!(CoreBoxId::String.is_required_in(&profile));
        assert!(CoreBoxId::File.is_required_in(&profile));  // Default では required
    }

    #[test]
    fn test_core_box_id_is_required_in_nofs() {
        let profile = RuntimeProfile::NoFs;
        assert!(CoreBoxId::String.is_required_in(&profile));
        assert!(!CoreBoxId::File.is_required_in(&profile));  // NoFs では optional
    }
}
```

### 2.4 Profile 拡張予定（設計メモ）

```rust
// 将来の enum 拡張予定
//
// TestMock:   テスト用（すべてのプラグインが mock に）
// Sandbox:    サンドボックス（外部 I/O 禁止）
// ReadOnly:   読み取り専用（FileBox.write 禁止）
// Embedded:   組み込み（メモリ制限あり、GC あり）
```

---

## 3. Task 2: PluginHost profile-aware 初期化

### 3.1 実装内容

**ファイル**:
- `src/runtime/plugin_host.rs`（修正）

### 3.2 修正内容

```rust
// src/runtime/plugin_host.rs

impl PluginHost {
    /// Phase 109: profile-aware with_core_from_registry
    pub fn with_core_from_registry(
        ring0: Arc<Ring0Context>,
        registry: &UnifiedBoxRegistry,
        profile: &RuntimeProfile,  // ← 新規引数
    ) -> Result<Self, CoreInitError> {
        // Phase 106: 必須 Box の registered 状態を確認
        for box_id in CoreBoxId::iter() {
            if box_id.is_required_in(profile) && !registry.contains(box_id.name()) {
                return Err(CoreInitError::MissingService {
                    box_id,
                    hint: format!(
                        "Core Box {} is required in {:?} profile",
                        box_id.name(),
                        profile.name()
                    ),
                });
            }
        }

        // FileBox provider チェック（Phase 107）
        match profile {
            RuntimeProfile::Default => {
                // Phase 108: FileBox provider 必須
                if provider_lock::get_filebox_provider().is_none() {
                    return Err(CoreInitError::MissingService {
                        box_id: CoreBoxId::File,
                        hint: "FileBox provider not initialized in Default profile".to_string(),
                    });
                }
            }
            RuntimeProfile::NoFs => {
                // Phase 109: FileBox provider 無くても OK（optional profile）
                // provider_lock は無視、下記 Task 3 の disable_filebox() で対応
            }
        }

        // ... 以下既存処理
        Ok(self)
    }
}

#[test]
fn test_with_core_from_registry_nofs_filebox_optional() {
    // Phase 109: NoFs profile では FileBox provider なしで OK
    let ring0 = Arc::new(default_ring0());
    let registry = UnifiedBoxRegistry::with_env_policy();
    let profile = RuntimeProfile::NoFs;

    // provider_lock をクリア（PluginHost が無視するはず）
    // → 実装時に適切なクリーンアップロジック追加

    let result = PluginHost::with_core_from_registry(ring0, &registry, &profile);
    assert!(result.is_ok());  // ✅ 必須でないので OK
}
```

---

## 4. Task 3: initialize_runtime() に profile 読み込み機構

### 4.1 実装内容

**ファイル**:
- `src/runner/initialize_runtime.rs`（新規 or 修正）
- `src/runner/modes/vm.rs`（修正）

### 4.2 修正内容：profile 読み込み層の責務分離

**修正1（Task 3 責務明示）**:
```rust
// src/runner/initialize_runtime.rs

/// Phase 109: profile-aware runtime 初期化
///
/// **責務分離**:
/// - initialize_runtime: 環境変数から profile を読む（唯一の env reader）
/// - PluginHost: profile を引数として受け取る（env に依存しない）
pub fn initialize_runtime(ring0: &Arc<Ring0Context>) -> Result<PluginHost, InitError> {
    // 1. Profile を環境変数から読む（この層のみで実施）
    let profile = RuntimeProfile::from_env();

    // 2. No-FS profile の場合、FileBox provider を明示的に disabled に
    if profile == RuntimeProfile::NoFs {
        disable_filebox_provider();
    }

    // 3. PluginHost に profile を渡す
    let registry = UnifiedBoxRegistry::with_env_policy();
    PluginHost::with_core_from_registry(ring0, &registry, &profile)
}

/// Phase 109: no-fs profile 用 FileBox 無効化
fn disable_filebox_provider() {
    // provider_lock に特別な "disabled" マーカーを設定
    // または、Task 4 の ReadOnlyFileIo で Err を返すようにする
}
```

---

## 5. Task 4: no-fs profile での FileBox 無効化実装

### 5.1 実装内容

**ファイル**:
- `src/providers/ring1/file/nofs_fileio.rs`（新規）
- `src/runtime/provider_lock.rs`（修正）
- `src/runtime/plugin_host.rs`（修正）

### 5.2 NoFsFileIo（スタブ実装）

```rust
// src/providers/ring1/file/nofs_fileio.rs

/// Phase 109: no-fs profile 用 FileBox stub
///
/// すべてのメソッドが Err を返す。
pub struct NoFsFileIo;

impl FileIo for NoFsFileIo {
    fn caps(&self) -> FileCaps {
        FileCaps { read: false, write: false }
    }

    fn open(&self, path: &str) -> FileResult<()> {
        Err(FileError::Unsupported(
            "FileBox is disabled in no-fs profile".to_string()
        ))
    }

    fn read(&self) -> FileResult<String> {
        Err(FileError::Unsupported(
            "FileBox is disabled in no-fs profile".to_string()
        ))
    }

    fn write(&self, _text: &str) -> FileResult<()> {
        Err(FileError::Unsupported(
            "FileBox is disabled in no-fs profile".to_string()
        ))
    }

    fn close(&self) -> FileResult<()> {
        Err(FileError::Unsupported(
            "FileBox is disabled in no-fs profile".to_string()
        ))
    }
}
```

### 5.3 provider_lock の profile-aware init

```rust
// src/runtime/provider_lock.rs

/// Phase 109: profile を考慮した provider 初期化
pub fn init_filebox_provider_for_profile(
    ring0: &Arc<Ring0Context>,
    profile: &RuntimeProfile,
) -> Result<(), String> {
    match profile {
        RuntimeProfile::Default => {
            // Phase 107: 標準プロファイルでは Ring0FsFileIo を使用
            init_default_filebox_provider(ring0)
        }
        RuntimeProfile::NoFs => {
            // Phase 109: no-fs プロファイルでは NoFsFileIo を使用
            set_filebox_provider(Arc::new(NoFsFileIo))
        }
    }
}
```

### 5.4 Logger/ConsoleService はそのまま有効（修正2: Logger関係）

**修正2（Task 4 Logger関係）**:
```rust
// docs/comment

/// Phase 109: no-fs プロファイルでのサービス有効性
///
/// ✅ 有効（no-fs でも必須）:
/// - Ring0.log（OS抽象化層 - panic/exit 時の最終出力）
/// - ConsoleBox（言語レベル console - stdout/stderr）
/// - その他 core_required（String/Integer/Array 等）
///
/// ❌ 無効（no-fs では disabled）:
/// - FileBox（ファイルシステム依存）
/// - Regex/Time/JSON等のオプショナル boxes（将来：profile ごとに制御可能）
```

---

## 6. Task 5: docs 更新 & CURRENT_TASK 反映

### 6.1 実装内容

**ファイル**:
- `phase108_filebox_write_semantics.md`（追記）
- **新規**: `phase109_runtime_profiles.md`（このドキュメント）
- `core_boxes_design.md`（更新）
- `CURRENT_TASK.md`

### 6.2 やること

1. **phase108_filebox_write_semantics.md に追記**：
   - Section 9「Phase 109 以降の計画」に：
     - 「Phase 109 で RuntimeProfile 機構が追加され、FileBox は conditional required に」

2. **phase109_runtime_profiles.md を本ドキュメントとして保存**：
   - RuntimeProfile enum + is_required_in() 設計
   - profile 読み込み層の責務分離（修正1）
   - Logger/ConsoleService の有効性（修正2）
   - 将来 Profile 拡張予定（修正3）

3. **core_boxes_design.md に追記**：
   - Section 5.4「Phase 109 - RuntimeProfile」に：
     - 「Default では FileBox required、NoFs では optional」
     - 「profile = env var 読み込み（initialize_runtime 層のみ）」
     - 「将来 TestMock/Sandbox/ReadOnly/Embedded への拡張計画」

4. **CURRENT_TASK.md に反映**：
   - Phase 109 完了行を追加
   - 次候補（Phase 110 FileHandleBox, Phase 111 append mode）をバックログに記載

---

## 7. 実装チェックリスト（Phase 109）

- [ ] RuntimeProfile enum + RuntimeProfile::from_env() 実装
- [ ] CoreBoxId.is_required_in(profile) ヘルパー実装
- [ ] NoFsFileIo スタブ実装
- [ ] PluginHost.with_core_from_registry(profile 引数追加) に profile-aware チェック
- [ ] initialize_runtime に profile 読み込み責務（修正1）
- [ ] initialize_runtime が disable_filebox_provider() 呼び出し（no-fs 時）
- [ ] Logger/ConsoleService の有効性を文書化（修正2）
- [ ] Profile 拡張予定を列挙（修正3）
- [ ] PluginHost.test_with_core_from_registry_nofs_filebox_optional() パス
- [ ] core_boxes_design / phase109 docs / CURRENT_TASK 更新済み
- [ ] ビルド・テスト完全成功

---

## 8. 設計原則（Phase 109 で確立）

### RuntimeProfile の位置づけ

```
【Layer】          【責務】                    【Example】
────────────────────────────────────────────────────────
env               User configuration         NYASH_RUNTIME_PROFILE=no-fs
initialize_runtime() env → RuntimeProfile   profile = RuntimeProfile::from_env()
PluginHost        profile-aware checks      is_required_in(&profile)
CoreBoxId         条件付き required 判定    is_required_in(&profile)
provider_lock     provider 登録（Profile 後）set_filebox_provider()
FileBox           provider 経由             read/write 実装
```

### Fail-Fast の段階的サポート

```
【Profile】      【FileBox チェック】      【Error 時】
────────────────────────────────────────────────────────
Default          init 時に provider 必須   CoreInitError::MissingService
NoFs             init 時に provider OK    (optional なので無視)
                 (実行時 read/write)      FileError::Unsupported
```

### 拡張ポイント

- Phase 110: FileHandleBox（複数ファイル同時）
- Phase 111: append mode 追加
- Phase 112: Ring0 service registry 統一化
- Phase 113: TestMock/Sandbox/ReadOnly/Embedded profile 実装

---

**Phase 109 指示書作成日**: 2025-12-03（3修正案統合版）
Status: Historical
