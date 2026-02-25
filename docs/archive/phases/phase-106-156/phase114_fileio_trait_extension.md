# Phase 114: FileIo trait 拡張 & メタデータ統一

## 目標

FsApi 側に揃った情報（exists / metadata / canonicalize）を、FileIo trait 経由で一貫して扱えるようにする。
FileHandleBox 内部の is_file/is_dir/size などをすべて FileIo::stat() の上に乗せる。
.hako 側 API（Phase 113 で公開したメソッド群）は変更しない（内部実装だけきれい化）。

## FileIo trait 設計（FsApi との分離：stateless vs stateful）

### FsApi (Ring0) - Stateless OS抽象

```rust
pub trait FsApi {
    fn exists(&self, path: &Path) -> bool;
    fn metadata(&self, path: &Path) -> Result<FsMetadata>;
    fn canonicalize(&self, path: &Path) -> Result<PathBuf>;
    fn read_to_string(&self, path: &Path) -> Result<String>;
    fn write_all(&self, path: &Path, data: &[u8]) -> Result<()>;
    fn append_all(&self, path: &Path, data: &[u8]) -> Result<()>;
}
```

- **Role**: OS ファイルシステムの直接抽象化
- **State**: なし（パス引数で毎回指定）
- **Usage**: Ring0Context 経由でアクセス

### FileIo (Ring1) - Stateful ハンドル抽象

```rust
pub struct FileStat {
    pub is_file: bool,
    pub is_dir: bool,
    pub size: u64,
}

pub trait FileIo: Send + Sync {
    fn caps(&self) -> FileCaps;
    fn open(&self, path: &str) -> FileResult<()>;
    fn read(&self) -> FileResult<String>;
    fn write(&self, text: &str) -> FileResult<()>;
    fn close(&self) -> FileResult<()>;
    fn as_any(&self) -> &dyn std::any::Any;

    // Phase 114: Metadata operations
    fn exists(&self) -> bool;
    fn stat(&self) -> FileResult<FileStat>;
    fn canonicalize(&self) -> FileResult<String>;
}
```

- **Role**: 現在開いているファイルハンドルに対する操作
- **State**: あり（open() で path を内部保持）
- **Usage**: FileHandleBox 経由でアクセス

### 設計原則

1. **FsApi = Stateless**: パスを毎回引数で受け取る
2. **FileIo = Stateful**: open() で path を保持、以降は引数不要
3. **分離理由**:
   - FsApi: OS レイヤーの直接操作（低レベル）
   - FileIo: ハンドルベースの高レベル API（.hako からアクセス）

## Ring0FsFileIo/NoFsFileIo の実装詳細

### Ring0FsFileIo (Default プロファイル)

```rust
pub struct Ring0FsFileIo {
    ring0: Arc<Ring0Context>,
    path: RwLock<Option<String>>,  // open() で設定
    mode: RwLock<Option<String>>,  // "r", "w", "a"
}

impl FileIo for Ring0FsFileIo {
    fn exists(&self) -> bool {
        // path が設定されていれば ring0.fs.exists() を呼ぶ
        // path が None なら false
    }

    fn stat(&self) -> FileResult<FileStat> {
        // path が設定されていれば ring0.fs.metadata() を呼ぶ
        // FileStat に変換して返す
    }

    fn canonicalize(&self) -> FileResult<String> {
        // path が設定されていれば ring0.fs.canonicalize() を呼ぶ
        // PathBuf を String に変換して返す
    }
}
```

### NoFsFileIo (NoFs プロファイル)

```rust
pub struct NoFsFileIo;

impl FileIo for NoFsFileIo {
    fn exists(&self) -> bool {
        // NoFs プロファイルでは常に false
        false
    }

    fn stat(&self) -> FileResult<FileStat> {
        // Unsupported エラーを返す
        Err(FileError::Unsupported("..."))
    }

    fn canonicalize(&self) -> FileResult<String> {
        // Unsupported エラーを返す
        Err(FileError::Unsupported("..."))
    }
}
```

## FileHandleBox の metadata_internal() 統一設計

### Before Phase 114

```rust
// Ring0FsFileIo.metadata() を直接ダウンキャスト
fn metadata_internal(&self) -> Result<FsMetadata, String> {
    self.io.as_ref()
        .ok_or_else(|| "FileHandleBox not open".to_string())?
        .as_any()
        .downcast_ref::<Ring0FsFileIo>()
        .ok_or_else(|| "FileIo is not Ring0FsFileIo".to_string())?
        .metadata()  // Ring0FsFileIo 専用メソッド
}
```

**問題点**:
- Ring0FsFileIo 依存（downcast 必要）
- FileIo trait を経由していない
- NoFsFileIo では動作しない

### After Phase 114

```rust
// FileIo::stat() を使用（trait 経由）
fn metadata_internal(&self) -> Result<FileStat, String> {
    let io = self.io.as_ref()
        .ok_or_else(|| "FileHandleBox is not open".to_string())?;

    io.stat()  // FileIo trait 経由
        .map_err(|e| format!("Metadata failed: {}", e))
}

// 他のメソッドも metadata_internal() の上に統一
fn size(&self) -> Result<u64, String> {
    self.metadata_internal().map(|meta| meta.size)
}

fn is_file(&self) -> Result<bool, String> {
    self.metadata_internal().map(|meta| meta.is_file)
}

fn is_dir(&self) -> Result<bool, String> {
    self.metadata_internal().map(|meta| meta.is_dir)
}
```

**改善点**:
- FileIo trait 経由で統一
- downcast 不要
- NoFsFileIo でも正しくエラーを返す

## Profile 別動作

### Default プロファイル（Ring0FsFileIo）

| メソッド | 動作 |
|---------|------|
| `exists()` | ファイルが存在すれば `true`、存在しないか path 未設定なら `false` |
| `stat()` | `FileStat { is_file, is_dir, size }` を返す。path 未設定なら `Err` |
| `canonicalize()` | 絶対パスを返す。path 未設定なら `Err` |

### NoFs プロファイル（NoFsFileIo）

| メソッド | 動作 |
|---------|------|
| `exists()` | 常に `false` |
| `stat()` | `Err(FileError::Unsupported("..."))` |
| `canonicalize()` | `Err(FileError::Unsupported("..."))` |

## テスト結果

### Ring0FsFileIo テスト（Default プロファイル）

```
✅ test_ring0_fs_fileio_stat_default_profile
✅ test_ring0_fs_fileio_exists_default_profile
✅ test_ring0_fs_fileio_canonicalize_default_profile
✅ test_ring0_fs_fileio_stat_without_open
✅ test_ring0_fs_fileio_canonicalize_without_open
```

### NoFsFileIo テスト

```
✅ test_nofs_fileio_exists (常に false)
✅ test_nofs_fileio_stat_error (Unsupported エラー)
✅ test_nofs_fileio_canonicalize_error (Unsupported エラー)
```

### FileHandleBox テスト

```
✅ test_filehandlebox_metadata_internal_default (stat() 経由で FileStat 取得)
✅ test_filehandlebox_metadata_internal_not_open (エラー確認)
✅ test_filehandlebox_ny_size_uses_stat (ny_size() が stat() 経由)
✅ test_filehandlebox_exists_uses_fileio (exists() が FileIo::exists() 経由)
✅ test_filehandlebox_is_file_is_dir_via_stat (is_file/is_dir が stat() 経由)
```

## 実装成果

### 統計

- **修正ファイル**: 4ファイル
  - `src/boxes/file/provider.rs`
  - `src/providers/ring1/file/ring0_fs_fileio.rs`
  - `src/providers/ring1/file/nofs_fileio.rs`
  - `src/boxes/file/handle_box.rs`
  - `src/providers/ring1/file/core_ro.rs`
- **追加行**: 約 +150 行
- **削除行**: 約 -20 行
- **新規テスト**: 11個

### 技術的成果

1. **FileIo trait 拡張**: exists/stat/canonicalize 正式追加
2. **統一設計**: FileHandleBox 内部が metadata_internal() に統一
3. **Profile 対応**: Default/NoFs 両プロファイルで正しく動作
4. **後方互換性**: .hako 側 API は変更なし（内部実装のみ統一）

## 次のステップ

- **Phase 115**: PathBox 実装（パス操作専用Box）
- **Phase 116**: ディレクトリ操作（mkdir/rmdir/readdir）

## 関連ドキュメント

- [core_boxes_design.md](./core_boxes_design.md) - FileHandleBox 設計
- [ring0-inventory.md](./ring0-inventory.md) - Ring0 機能一覧
- [Phase 113 実装](./phase113_filehandlebox_api.md) - Nyash API 公開
Status: Historical
