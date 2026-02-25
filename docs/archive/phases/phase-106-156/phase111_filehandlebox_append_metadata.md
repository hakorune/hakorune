# Phase 111: FileHandleBox append モード + metadata 拡張

## 0. ゴール

- Phase 110 で作った FileHandleBox（複数回アクセス I/O）に対して:
  - **"a" append モード** を追加して、ログ／追記用途に対応する。
  - **ファイルメタデータ取得 API**（size / exists / is_file / is_dir）を設計・実装する。
  - すべて Ring0FsFileIo → Ring0.FsApi 経由で行い、レイヤー構造と Fail-Fast を崩さない。

---

## 1. スコープと非スコープ

### スコープ（今回やること）

1. **設計ドキュメント**:
   - append モードの意味（truncate との違い）を正確に定義。
   - metadata API（size/exists/is_file/is_dir）を決定。
   - FsApi 拡張（append_all）の位置づけを明記。

2. **FsApi / Ring0FsFileIo 側の拡張**:
   - FsApi trait に `append_all(path, data)` メソッドを追加。
   - Ring0FsFileIo で append/truncate の 2 モード実装。

3. **FileHandleBox API の拡張**:
   - open(path, mode) に "a" サポート（"r"/"w"/"a" の 3 モード）。
   - write_all が mode に応じて truncate/append を切り替え。
   - metadata helper（size/exists/is_file/is_dir）を内部 Rust API として実装。

4. **Profile との整合**:
   - Default: append & metadata 有効。
   - NoFs: open 自体がエラー、metadata もエラー。

5. **テスト + ドキュメント**:
   - FileHandleBox append テスト。
   - metadata テスト。
   - NoFs profile 対応テスト。

### 非スコープ（今回はやらない）

- modified（mtime）情報（Phase 112+ で検討）。
- FileBox（ワンショット API）側への append 追加（必要になれば後フェーズ）。
- パスワイルドカード、ディレクトリ列挙、ウォッチャなどの高度機能。
- ACL/パーミッション／ロックなどの OS 特有機能。
- **NyashBox 公開 API**（metadata メソッドの .hako 側からの呼び出し）→ Phase 112+ で検討。

---

## 2. Task 1: 設計ドキュメント（append ＋ metadata）

### 2.1 実装内容

**ファイル**:
- 本ドキュメント（phase111_filehandlebox_append_metadata.md）

### 2.2 設計決定

#### append モードの仕様

**open(path, "a") の意味**:
- 存在しない場合 → 新規作成。
- 存在する場合 → ファイル末尾に書き足す。

**write_all(content) の挙動**:
- Mode "w" → truncate（毎回上書き、Phase 108 決定どおり）。
- Mode "a" → append（末尾に追記）。
- Mode "r" で write_all → "FileHandleBox is opened in read-only mode" エラー。

**truncate vs append の明確な使い分け**:
```rust
// truncate mode: ログファイルを毎回リセット
handle.open("output.log", "w")?;
handle.write_all("Session started\n")?;

// append mode: 複数回の実行結果を累積
handle.open("history.log", "a")?;
handle.write_all("Operation 1\n")?;  // 末尾に追記
handle.write_all("Operation 2\n")?;  // さらに末尾に追記
```

#### metadata API の仕様

**最小限の属性セット**:
- `size()` → ファイルサイズ（バイト数）
- `exists()` → ファイルが存在するか
- `is_file()` → ファイルであるか（ディレクトリではない）
- `is_dir()` → ディレクトリであるか

**注: modified（mtime）は Phase 112 以降で検討**（FsMetadata への追加が必要）。

**ライフサイクルと metadata の関係**:
- **path が設定されていれば**（open 済みでなくても）metadata 取得可能。
- 例: close() 後でも stat 可能。
- 例: open せずに path だけ指定して metadata 取得（予定）。

理由: FsApi.metadata(path) は stateless な操作なので、FileIo インスタンスの有無に依存しない。

#### FsApi 拡張の位置づけ

**FsApi.append_all(path, data) の追加**:
- Phase 107: read_to_string / read / write_all が確立。
- Phase 111: append_all を追加（write_all と対称的）。

```rust
pub trait FsApi: Send + Sync {
    fn read_to_string(&self, path: &Path) -> Result<String, IoError>;
    fn read(&self, path: &Path) -> Result<Vec<u8>, IoError>;
    fn write_all(&self, path: &Path, data: &[u8]) -> Result<(), IoError>;
    fn append_all(&self, path: &Path, data: &[u8]) -> Result<(), IoError>;  // ← 新規
    fn exists(&self, path: &Path) -> bool;
    fn metadata(&self, path: &Path) -> Result<FsMetadata, IoError>;
    fn canonicalize(&self, path: &Path) -> Result<PathBuf, IoError>;
}
```

**Ring0FsFileIo での実装**:
- open(path, mode) で path と mode を保持。
- write(text) で:
  - Mode "w" → FsApi.write_all(path, text.as_bytes())（truncate）。
  - Mode "a" → FsApi.append_all(path, text.as_bytes())（append）。
- 内部 metadata_helper で FsApi.metadata(path) を呼び出し。

---

## 3. Task 2: FsApi / Ring0FsFileIo 拡張

### 3.1 実装内容

**ファイル**:
- `src/runtime/ring0/traits.rs`（FsApi trait 拡張）
- `src/runtime/ring0/std_impls.rs`（FsApi 実装）
- `src/providers/ring1/file/ring0_fs_fileio.rs`（Ring0FsFileIo 拡張）

### 3.2 やること

#### FsApi trait 拡張（src/runtime/ring0/traits.rs）

1. **append_all メソッドを追加**:

```rust
pub trait FsApi: Send + Sync {
    // ... 既存メソッド ...

    /// ファイルに追記（append）
    ///
    /// ファイルが存在しない場合は新規作成、存在する場合は末尾に追記。
    /// Phase 111: write_all と対称的に提供。
    fn append_all(&self, path: &Path, data: &[u8]) -> Result<(), IoError>;
}
```

2. **FsMetadata の確認**:
   - 既に実装済みの FsMetadata を確認：
     ```rust
     pub struct FsMetadata {
         pub is_file: bool,
         pub is_dir: bool,
         pub len: u64,
     }
     ```
   - Phase 111 では modified は追加しない（後フェーズで）。

#### std_impls.rs での実装

```rust
impl FsApi for StdFsApi {
    fn append_all(&self, path: &Path, data: &[u8]) -> Result<(), IoError> {
        use std::fs::OpenOptions;
        use std::io::Write;

        let mut file = OpenOptions::new()
            .create(true)        // 存在しなければ作成
            .append(true)        // append モードで開く
            .open(path)
            .map_err(|e| IoError::Io(format!("append_all failed: {}", e)))?;

        file.write_all(data)
            .map_err(|e| IoError::Io(format!("write failed: {}", e)))
    }
}
```

#### Ring0FsFileIo での append 対応（src/providers/ring1/file/ring0_fs_fileio.rs）

1. **struct フィールド確認**:
   - path, mode を保持していることを確認。

2. **write() メソッドを拡張**:

```rust
impl FileIo for Ring0FsFileIo {
    fn write(&self, text: &str) -> FileResult<()> {
        if self.mode == "a" {
            // Append mode
            self.ring0.fs.append_all(Path::new(&self.path), text.as_bytes())
                .map_err(FileError::Io)
        } else if self.mode == "w" {
            // Truncate mode (default)
            self.ring0.fs.write_all(Path::new(&self.path), text.as_bytes())
                .map_err(FileError::Io)
        } else if self.mode == "r" {
            Err(FileError::Unsupported(
                "Cannot write in read-only mode".to_string()
            ))
        } else {
            Err(FileError::Unsupported(
                format!("Unsupported mode: {}", self.mode)
            ))
        }
    }

    // 内部 metadata helper（phase 111）
    fn metadata(&self) -> FileResult<crate::runtime::ring0::traits::FsMetadata> {
        self.ring0.fs.metadata(Path::new(&self.path))
            .map_err(FileError::Io)
    }
}
```

3. **FileIo trait は追加しない** ← Phase 111 での決定：
   - metadata メソッドは FileIo trait には追加せず、Ring0FsFileIo のプライベートメソッドとして実装。
   - FileHandleBox が metadata 取得時に、Ring0FsFileIo の内部ヘルパを呼び出す形にする。

---

## 4. Task 3: FileHandleBox API 実装（append + metadata）

### 4.1 実装内容

**ファイル**:
- `src/boxes/file/handle_box.rs`

### 4.2 やること

#### open() メソッド拡張

```rust
pub fn open(&mut self, path: &str, mode: &str) -> Result<(), String> {
    // Double-open チェック
    if self.is_open() {
        return Err(already_open());
    }

    // Mode バリデーション（"r", "w", "a" のみ）
    if mode != "r" && mode != "w" && mode != "a" {
        return Err(unsupported_mode(mode));
    }

    // NoFs profile チェック（既存）
    if self.io.is_none() {  // provider が無い
        return Err(provider_disabled_in_nofs_profile());
    }

    // path と mode を保存
    self.path = path.to_string();
    self.mode = mode.to_string();

    // FileIo に open を委譲（mode を含める）
    self.io
        .as_ref()
        .unwrap()
        .open(path)
        .map_err(|e| format!("Open failed: {:?}", e))
}
```

**注**: mode パラメータはファイル操作のモード制御に使用し、FileIo.open() 呼び出し時には既に self.mode に保存されている。

#### write_all() メソッド拡張

```rust
pub fn write_all(&self, content: &str) -> Result<(), String> {
    // open 済みチェック
    if !self.is_open() {
        return Err(not_open());
    }

    // read-only チェック
    if self.mode == "r" {
        return Err("FileHandleBox is opened in read-only mode".to_string());
    }

    // write (mode に基づいて truncate/append を切り替え)
    self.io
        .as_ref()
        .unwrap()
        .write(content)
        .map_err(|e| format!("Write failed: {:?}", e))
}
```

**write() メソッド内**（Ring0FsFileIo）で mode チェックして append/truncate を判定。

#### metadata メソッド群（内部 Rust API）

```rust
impl FileHandleBox {
    /// ファイルサイズを取得（バイト数）
    ///
    /// Path さえあれば（open 済みでなくても）stat 可能。
    pub fn size(&self) -> Result<u64, String> {
        if self.path.is_empty() {
            return Err("FileHandleBox path not set".to_string());
        }

        // Ring0FsFileIo の内部 metadata_helper を呼び出し
        // または Ring0Context から直接 FsApi.metadata() を取得
        // 詳細は実装時に決定
        self.metadata_internal()
            .map(|meta| meta.len)
    }

    /// ファイルが存在するか確認
    pub fn exists(&self) -> Result<bool, String> {
        if self.path.is_empty() {
            return Err("FileHandleBox path not set".to_string());
        }

        self.metadata_internal()
            .map(|_| true)
            .or_else(|_| Ok(false))  // not found → false
    }

    /// ファイルであるか確認
    pub fn is_file(&self) -> Result<bool, String> {
        if self.path.is_empty() {
            return Err("FileHandleBox path not set".to_string());
        }

        self.metadata_internal()
            .map(|meta| meta.is_file)
    }

    /// ディレクトリであるか確認
    pub fn is_dir(&self) -> Result<bool, String> {
        if self.path.is_empty() {
            return Err("FileHandleBox path not set".to_string());
        }

        self.metadata_internal()
            .map(|meta| meta.is_dir)
    }

    /// 内部ヘルパー: FsApi.metadata を呼び出し
    fn metadata_internal(&self) -> Result<crate::runtime::ring0::traits::FsMetadata, String> {
        // 実装パターン:
        // Option 1: io.as_ref().unwrap() から metadata() ヘルパを呼び出す
        // Option 2: provider_lock から Ring0Context を取得して FsApi を呼び出す
        // Phase 111 では Option 1 を推奨（io と path が一体的に扱える）
        todo!("metadata_internal implementation")
    }
}
```

**設計方針**:
- metadata メソッド群は **Rust 内部 API** として実装。
- .hako から呼び出す場合は、Phase 112+ で MethodBox 登録を検討。
- 現在は Unit テスト で動作確認するだけ。

---

## 5. Task 4: Profile との整合チェック

### 5.1 実装内容

**ファイル**:
- `src/runtime/runtime_profile.rs`（確認のみ、修正不要）
- `src/runtime/plugin_host.rs`（確認のみ、修正不要）
- `docs/development/current/main/phase111_filehandlebox_append_metadata.md`（本ドキュメント）

### 5.2 現行設計確認

#### Default プロファイル

- FileBox / FileHandleBox 両方が Ring0FsFileIo を使用。
- append / metadata も**フル機能**で動作。

#### NoFs プロファイル

- FileBox: open せず read/write でエラー "FileBox disabled in no-fs profile"。
- FileHandleBox: open 自体が "FS disabled in NoFs profile" エラーで拒否。
- metadata: NoFs で path を stat しない（実装上、open 済みだからこそ stat 可能という前提）。

**Phase 111 での確認**:
- NoFs profile でも metadata_internal() が呼び出されうるか？
  - path が設定されている状態で metadata() を呼ぶ場合、どうするか。
  - 現在の設計では io が無い状態では metadata も失敗（OK）。

---

## 6. Task 5: テスト + docs 更新

### 6.1 テスト

**ファイル**:
- `src/boxes/file/handle_box.rs` 内の test モジュール

#### テスト 1: append モード動作確認

```rust
#[test]
fn test_filehandlebox_append_mode() {
    use std::fs;

    let path = "/tmp/phase111_append_test.txt";
    let _ = fs::remove_file(path);  // cleanup

    // First write (truncate)
    let mut handle = FileHandleBox::new();
    handle.open(path, "w").unwrap();
    handle.write_all("hello\n").unwrap();
    handle.close().unwrap();

    // Append
    let mut handle = FileHandleBox::new();
    handle.open(path, "a").unwrap();
    handle.write_all("world\n").unwrap();
    handle.close().unwrap();

    // Verify
    let content = fs::read_to_string(path).unwrap();
    assert_eq!(content, "hello\nworld\n");

    let _ = fs::remove_file(path);
}
```

#### テスト 2: metadata（size）確認

```rust
#[test]
fn test_filehandlebox_metadata_size() {
    use std::fs;

    let path = "/tmp/phase111_metadata_test.txt";
    let _ = fs::remove_file(path);

    // Write test file
    let mut handle = FileHandleBox::new();
    handle.open(path, "w").unwrap();
    handle.write_all("hello").unwrap();  // 5 bytes
    handle.close().unwrap();

    // Check size
    let mut handle = FileHandleBox::new();
    let size = handle.size().unwrap();
    assert_eq!(size, 5);

    let _ = fs::remove_file(path);
}
```

#### テスト 3: metadata（is_file / is_dir）確認

```rust
#[test]
fn test_filehandlebox_metadata_is_file() {
    use std::fs;

    let path = "/tmp/phase111_file_test.txt";
    let _ = fs::remove_file(path);

    // Create file
    let mut handle = FileHandleBox::new();
    handle.open(path, "w").unwrap();
    handle.close().unwrap();

    // Check is_file
    let mut handle = FileHandleBox::new();
    let is_file = handle.is_file().unwrap();
    assert!(is_file);

    let is_dir = handle.is_dir().unwrap();
    assert!(!is_dir);

    let _ = fs::remove_file(path);
}
```

#### テスト 4: read-only mode での write 拒否

```rust
#[test]
fn test_filehandlebox_write_readonly_error() {
    use std::fs;

    let path = "/tmp/phase111_readonly_test.txt";
    let _ = fs::remove_file(path);

    // Create file
    fs::write(path, "content").unwrap();

    // Open in read mode
    let mut handle = FileHandleBox::new();
    handle.open(path, "r").unwrap();

    // Try to write → Error
    let result = handle.write_all("new");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("read-only"));

    let _ = fs::remove_file(path);
}
```

#### テスト 5: NoFs profile での open 拒否

```rust
#[test]
fn test_filehandlebox_nofs_profile_error() {
    // NYASH_RUNTIME_PROFILE=no-fs を設定して実行
    let mut handle = FileHandleBox::new();
    let result = handle.open("/tmp/test.txt", "w");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("disabled"));
}
```

### 6.2 ドキュメント更新

#### phase110_filehandlebox_design.md に追記

Section 「Phase 111 との関係」を追加：

```markdown
### Phase 111: append モード + metadata 拡張

- **append モード**: open(path, "a") で末尾に追記。truncate (mode "w") と明確に区別。
- **metadata**: size / exists / is_file / is_dir を Ring0FsFileIo 経由で取得。
- **FsApi 拡張**: append_all(path, data) を追加（write_all と対称的）。
- **内部 Rust API**: metadata メソッド群は .hako 公開せず、テスト・内部用のみ。
- **modified（mtime）**: Phase 112+ で検討予定。
```

#### core_boxes_design.md 更新

FileHandleBox セクションに 1–2 行追記：

```markdown
- Phase 111: append モード + metadata API（size/exists/is_file/is_dir）実装完了。
```

#### CURRENT_TASK.md 更新

Phase 111 の完了行を追加：

```
## Phase 111: FileHandleBox append + metadata 拡張（完了予定）

- [ ] FsApi.append_all() 追加
- [ ] Ring0FsFileIo での append/truncate 切り替え実装
- [ ] FileHandleBox.open に "a" mode サポート
- [ ] FileHandleBox.metadata_internal / size / exists / is_file / is_dir 実装
- [ ] append テスト + metadata テスト + NoFs profile テスト
- [ ] core_boxes_design / phase111 docs 更新済み
- [ ] ビルド・テスト完全成功

## Backlog

### Phase 112: Ring0 Service Registry 統一化

- Ring0.log の統一レジストリ化
- provider_lock の汎用化
- metadata を FsMetadata に modified フィールド追加

### Phase 113: FileHandleBox NyashBox 公開 API

- metadata メソッドを .hako から呼び出し可能に
- MethodBox 登録

### Phase 114: FileIo 機能拡張

- exists / stat / canonicalize を FileIo trait に追加
- 権限・ロック機構の検討
```

---

## 7. 実装チェックリスト（Phase 111）

- [ ] FsApi trait に append_all() メソッド追加
- [ ] StdFsApi で append_all() 実装（OpenOptions.append()）
- [ ] Ring0FsFileIo.write() を mode チェックして append/truncate 切り替え
- [ ] FileHandleBox.open() に mode バリデーション（"r"/"w"/"a" のみ）
- [ ] FileHandleBox.metadata_internal() が FsApi.metadata() を呼び出し
- [ ] FileHandleBox.size / exists / is_file / is_dir を内部 Rust API として実装
- [ ] append テスト（write → append → 内容確認）
- [ ] metadata テスト（size, is_file, is_dir）
- [ ] read-only mode での write 拒否テスト
- [ ] NoFs profile 対応テスト（open がエラー）
- [ ] core_boxes_design / phase111 docs / CURRENT_TASK 更新済み
- [ ] ビルド・テスト完全成功

---

## 8. 設計原則（Phase 111 で確立）

### append vs truncate の明確な使い分け

```
【Mode】    【挙動】                  【用途】
─────────────────────────────────────────────
"w"         毎回上書き（truncate）   ログファイル初期化、設定ファイル更新
"a"         末尾に追記（append）     履歴ログ、イベント記録
"r"         読み取り専用             ファイル読み込み
```

### FsApi の成熟度

**Phase 107-108**:
- read_to_string, read, write_all, exists, metadata

**Phase 111**:
- append_all（write_all と対称的）

**Phase 112+**:
- modified フィールド（SystemTime）の FsMetadata 追加
- Ring0 service registry 統一化

### FileHandleBox のメタデータ戦略

**Phase 111**:
- metadata は内部 Rust API のみ（.hako 非公開）
- テスト・デバッグ用途

**Phase 112+**:
- MethodBox 登録で .hako 公開
- modified を FsMetadata に追加

---

**Phase 111 指示書完成日**: 2025-12-03（修正案統合版）
Status: Historical
