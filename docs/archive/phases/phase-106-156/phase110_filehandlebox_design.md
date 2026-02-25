# Phase 110: FileHandleBox 設計（複数回アクセス対応）

## 0. ゴール

- Phase 108 で実装した FileBox（「1ショット I/O」：read/write 1回ずつ）を補完する
- FileHandleBox（「複数回アクセス I/O」：open → read/write/read → close）を設計・実装
- **Ring0FsFileIo を内部で再利用**して、レイヤー統一を維持
- **RuntimeProfile との整合**：Default では使用可能、NoFs では使用不可

---

## 1. スコープと非スコープ

### スコープ（今回やること）

1. **設計ドキュメントの作成**
   - FileBox / FileHandleBox / FileIo / FsApi の役割と関係を定義
   - FileHandleBox の API（open/read/write/close 等）とライフサイクルを決める
   - 二重 open 時の仕様を明記

2. **Rust 側の最小実装**
   - FileHandleBox の型スケルトンと trait 実装（NyashBox）を追加
   - Ring0FsFileIo を内部で使い回す形で、最小の open/read/write/close を通す
   - close() 後の挙動を明確に定義

3. **プロファイルとの整合**
   - RuntimeProfile::Default のときは FileHandleBox が使用可能
   - RuntimeProfile::NoFs のときは FileHandleBox は禁止（open 時にエラー）

### 非スコープ（今回はやらない）

- FileHandleBox を Nyash (.hako) 側からフル運用するサンプル大量追加（必要なら 1–2 個の最小例だけ）
- ファイルロック/権限/並行書き込みなどの高度機能
- FileBox API 自体の破壊的変更（既存の read/write の意味は維持）
- append mode（Phase 111 で扱う）

---

## 2. Task 1: 設計ドキュメント作成

### 2.1 ファイル

- `docs/development/current/main/phase110_filehandlebox_design.md`（このドキュメント）

### 2.2 目的と役割分担

**FileBox（Phase 108）**:
- シンプルな 1 ショット I/O
- `read(path) -> String` / `write(path, content) -> OK/Error` のようなメソッド
- ファイルを開いて → 読む/書く → 閉じるをすべて隠す
- ユースケース：ログ書き込み、ワンショット設定ファイル読み込み

**FileHandleBox（Phase 110）**:
- 複数回アクセス I/O
- ハンドルで 1 ファイルを確保 → open/read/write/close を自分で制御
- ユースケース：大きなテキストファイルの行単位読み込み、逐次更新

**レイヤー図**:
```
[FileBox]            [FileHandleBox]
   |                      |
   └──────┬────────────────┘
           | (FileIo trait)
           v
    [Ring0FsFileIo]
           |
           v (FsApi)
      [Ring0.fs]
           |
           v
        [std::fs]
```

### 2.3 ライフサイクル設計

#### FileHandleBox のライフサイクル

```
1. birth() / new()
   └─ path 未指定、io 未初期化（None）
   └ファイルはまだ開かれない

2. open(path, mode)
   └─ Ring0FsFileIo をこのインスタンス用に作成して保持
   └─ FileIo.open(path) を内部で呼び出す
   └─ 以後 read/write が利用可能に

3. 複数回 read/write
   └─ 同じハンドルで繰り返し呼び出し可能
   └─ close() されるまで有効

4. close()
   └─ 内部 FileIo をリセット（Option::None に）
   └─ 以後 read/write は Err("FileHandleBox is not open")

5. Drop 時
   └─ close() 忘れがあれば、警告ログを出す（Phase 110 では実装可能、または後回し）
```

#### .hako 側での使用パターン

```nyash
// パターン1: 新しいファイルに書き込み
local h = new FileHandleBox()
h.open("/tmp/output.txt", "w")
h.write("line 1")
h.write("line 2")
h.close()

// パターン2: 既存ファイルを読む
local h = new FileHandleBox()
h.open("/tmp/input.txt", "r")
local content = h.read()
h.close()

// パターン3: close() 忘れ（警告ログまたはパニック、Phase 110 で決定）
local h = new FileHandleBox()
h.open("/tmp/data.txt", "w")
h.write("data")
// close() なしで終了 ← 警告が出るかもしれない
```

### 2.4 二重 open の仕様（重要）

**方針**:
- 最初の open → 成功、io が Some(FileIo) に
- 2 番目の open → エラーを返す（Fail-Fast）
- 理由：複数ファイルハンドルが必要なら複数 FileHandleBox インスタンスを使う

**実装**:
```rust
pub fn open(&mut self, path: &str, mode: &str) -> Result<(), String> {
    // 既に open 済みなら error
    if self.io.is_some() {
        return Err("FileHandleBox is already open. Call close() first.".to_string());
    }
    // 新規 open
    let io = Arc::new(Ring0FsFileIo::new(self.ring0.clone()));
    io.open(path)?;
    self.io = Some(io);
    Ok(())
}
```

### 2.5 close() 後の読み書き

```rust
pub fn read_to_string(&self) -> Result<String, String> {
    self.io.as_ref()
        .ok_or("FileHandleBox is not open".to_string())?
        .read()
}

pub fn write_all(&self, content: &str) -> Result<(), String> {
    self.io.as_ref()
        .ok_or("FileHandleBox is not open".to_string())?
        .write(content)
}
```

---

## 3. Task 2: API 定義（Rust 側インターフェース）

### 3.1 ファイル

- `src/boxes/file/mod.rs`（または新規 `src/boxes/file/handle_box.rs`）
- `src/boxes/file/provider.rs`（必要に応じて）

### 3.2 Rust 側 struct 定義

```rust
/// Phase 110: FileHandleBox
///
/// ハンドルベースのファイル I/O。
/// open(path, mode) → read/write → close() という複数回アクセスをサポート。
pub struct FileHandleBox {
    base: BoxBase,
    path: String,
    mode: String,
    io: Option<Arc<dyn FileIo>>,  // 各インスタンスが独立した FileIo を保持
}
```

### 3.3 NyashBox 実装

```rust
impl NyashBox for FileHandleBox {
    fn type_name(&self) -> &str {
        "FileHandleBox"
    }

    fn to_string_box(&self) -> StringBox {
        StringBox::new(format!("FileHandleBox(path={}, mode={}, open={})",
            self.path, self.mode, self.is_open()))
    }

    fn equals(&self, _other: &dyn NyashBox) -> bool {
        // FileHandleBox インスタンスは path + mode で比較
        // 厳密でなくて OK
        false  // 簡易実装
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
```

### 3.4 メソッドセット

```rust
impl FileHandleBox {
    /// 新規 FileHandleBox を作成（ファイルはまだ open されない）
    pub fn new() -> Self {
        FileHandleBox {
            base: BoxBase::new(),
            path: String::new(),
            mode: String::new(),
            io: None,
        }
    }

    /// ファイルを開く
    ///
    /// # Arguments
    /// - path: ファイルパス
    /// - mode: "r" (読み込み) or "w" (上書き) ※ Phase 111 で "a" (append) 追加予定
    ///
    /// # Error
    /// - 既に open されている場合: "FileHandleBox is already open. Call close() first."
    /// - ファイルが見つからない場合（mode="r"）: "File not found"
    pub fn open(&mut self, path: &str, mode: &str) -> Result<(), String> {
        // 二重 open チェック
        if self.io.is_some() {
            return Err("FileHandleBox is already open. Call close() first.".to_string());
        }

        // mode 検証
        if mode != "r" && mode != "w" {
            return Err(format!("Unsupported mode: {}. Use 'r' or 'w'", mode));
        }

        // 新規 FileIo 作成（Ring0FsFileIo）
        // ※ provider_lock から取得するか、直接生成するかは実装で決定
        let io = Arc::new(Ring0FsFileIo::new(ring0.clone()));
        io.open(path)?;

        self.path = path.to_string();
        self.mode = mode.to_string();
        self.io = Some(io);
        Ok(())
    }

    /// ファイルの内容を全て読む
    ///
    /// # Error
    /// - open されていない場合: "FileHandleBox is not open"
    pub fn read_to_string(&self) -> Result<String, String> {
        self.io.as_ref()
            .ok_or("FileHandleBox is not open".to_string())?
            .read()
    }

    /// ファイルに内容を書き込む（上書きモード）
    ///
    /// # Error
    /// - open されていない場合: "FileHandleBox is not open"
    /// - write mode でない場合: "FileHandleBox opened in read mode"
    pub fn write_all(&self, content: &str) -> Result<(), String> {
        if self.mode != "w" {
            return Err("FileHandleBox opened in read mode".to_string());
        }

        self.io.as_ref()
            .ok_or("FileHandleBox is not open".to_string())?
            .write(content)
    }

    /// ファイルを閉じる
    ///
    /// # Error
    /// - open されていない場合: "FileHandleBox is not open"
    pub fn close(&mut self) -> Result<(), String> {
        if self.io.is_none() {
            return Err("FileHandleBox is not open".to_string());
        }

        // 内部 FileIo を drop
        self.io.take();
        self.path.clear();
        self.mode.clear();
        Ok(())
    }

    /// ファイルが open されているかチェック
    pub fn is_open(&self) -> bool {
        self.io.is_some()
    }
}
```

### 3.5 Ring0FsFileIo の独立性（重要）

**設計原則**:
- 各 FileHandleBox インスタンスが **独立した FileIo を保持**
- 複数の FileHandleBox が同時に異なるファイルを open できる

**例**:
```rust
let mut h1 = FileHandleBox::new();
h1.open("/tmp/file1.txt", "r")?;

let mut h2 = FileHandleBox::new();
h2.open("/tmp/file2.txt", "w")?;

// h1 と h2 は別々の FileIo（Ring0FsFileIo）を持つ
// h1.read() と h2.write() は同時実行可能

h1.close()?;
h2.close()?;
```

---

## 4. Task 3: プロファイルとの整合

### 4.1 ファイル

- `src/runtime/provider_lock.rs`（必要に応じて）
- `src/runtime/runtime_profile.rs`
- `docs/development/current/main/phase110_filehandlebox_design.md`（このドキュメント）

### 4.2 FileHandleBox のプロファイル位置づけ

**RuntimeProfile::Default**:
- **位置づけ**: optional（コアボックスではない）
- **動作**:
  - provider_lock に FileIo が登録されている
  - FileHandleBox.open() は成功する
  - read/write 可能

**RuntimeProfile::NoFs**:
- **位置づけ**: 使用不可（disabled）
- **動作**:
  - provider_lock に FileIo が登録されない（NoFsFileIo のみ）
  - FileHandleBox.open() を呼ぶと：
    ```
    Err("File I/O disabled in no-fs profile. FileHandleBox is not available.")
    ```

**将来計画（Phase 113+）**:
- **RuntimeProfile::TestMock**: mock FileIo を使用（テスト用）
- **RuntimeProfile::Sandbox**: 指定ディレクトリのみアクセス可能
- **RuntimeProfile::ReadOnly**: 読み取り専用（write Err）

### 4.3 Policy 記述

```markdown
## プロファイル別の可用性

| Profile  | FileBox | FileHandleBox | 特性 |
|----------|---------|---------------|------|
| Default  | ✅ | ✅ | 完全なファイル I/O |
| NoFs     | ❌ | ❌ | ファイル I/O 禁止 |
| TestMock (TBD) | ✅ mock | ✅ mock | テスト用ダミー |
| Sandbox (TBD)  | ✅ dir限定 | ✅ dir限定 | サンドボックス |

**注意**: FileHandleBox は optional boxes だが、NoFs では
「disabled」として扱う（optional ではなく「使用不可」）
```

---

## 5. Task 4: 最小限のテストとサンプル

### 5.1 ファイル

- `src/boxes/file/tests.rs`（新規 or 既存に追加）
- `apps/examples/file_handle_min.hako`（オプション、.hako 側のサンプル）

### 5.2 テスト内容

#### テスト 1: Default プロファイル - 基本動作

```rust
#[test]
fn test_filehandlebox_basic_write_read() {
    // 新しいファイルに書き込み
    let mut h = FileHandleBox::new();
    assert!(!h.is_open());

    let tmp_path = "/tmp/phase110_test_write_read.txt";
    h.open(tmp_path, "w").expect("open failed");
    assert!(h.is_open());

    h.write_all("hello world").expect("write failed");
    h.close().expect("close failed");
    assert!(!h.is_open());

    // 同じファイルを読む
    let mut h2 = FileHandleBox::new();
    h2.open(tmp_path, "r").expect("open failed");
    let content = h2.read_to_string().expect("read failed");
    assert_eq!(content, "hello world");
    h2.close().expect("close failed");

    // cleanup
    std::fs::remove_file(tmp_path).ok();
}
```

#### テスト 2: 二重 open error

```rust
#[test]
fn test_filehandlebox_double_open_error() {
    let mut h = FileHandleBox::new();
    h.open("/tmp/test.txt", "w").expect("first open");

    // 2 番目の open は error
    let result = h.open("/tmp/test2.txt", "w");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("already open"));
}
```

#### テスト 3: close() 後のアクセス error

```rust
#[test]
fn test_filehandlebox_closed_access_error() {
    let mut h = FileHandleBox::new();
    h.open("/tmp/test.txt", "w").expect("open");
    h.close().expect("close");

    // close 後の read は error
    let result = h.read_to_string();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not open"));
}
```

#### テスト 4: NoFs プロファイル - open error

```rust
#[test]
fn test_filehandlebox_nofs_profile_disabled() {
    // NYASH_RUNTIME_PROFILE=no-fs で実行される想定
    let profile = RuntimeProfile::NoFs;

    let mut h = FileHandleBox::new();
    let result = h.open_with_profile("/tmp/test.txt", "r", &profile);

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("disabled in no-fs profile"));
}
```

### 5.3 .hako 側サンプル（オプション）

**apps/examples/file_handle_min.hako**:
```nyash
// File handle を使った行単位読み込みのプロトタイプ（将来）
// Phase 110 では不要。Phase 111 以降で追加予定
```

---

## 6. Task 5: ドキュメント更新

### 6.1 ファイル

- `docs/development/current/main/core_boxes_design.md`（修正）
- `docs/development/current/main/ring0-inventory.md`（修正）
- `CURRENT_TASK.md`（修正）

### 6.2 やること

#### core_boxes_design.md 更新

新セクション「5.5 Phase 110 - FileHandleBox」を追加：

```markdown
### 5.5 Phase 110 - FileHandleBox

FileBox（ワンショット I/O）を補完するハンドルベースのファイル I/O。

- **位置づけ**: core_optional（future で core_required に昇格の可能性）
- **API**: open(path, mode) → read/write → close()
- **プロファイル対応**: Default ✅、NoFs ❌
- **実装**: Ring0FsFileIo を内部で再利用

詳細: [Phase 110 設計書](phase110_filehandlebox_design.md)
```

#### ring0-inventory.md 更新

「File I/O Service」セクションの「Future Expansion」に追記：

```markdown
## Future Expansion

- Phase 110: FileHandleBox（ハンドルベース複数回アクセス）
- Phase 111: append mode サポート
- Phase 112: metadata / stat サポート
- Phase 113: Ring0 service registry 統一化
```

#### CURRENT_TASK.md 更新

- Phase 110 を「計画中 → 進行中」に変更（実装開始時）
- Phase 110 完了後に「進行中 → 完了」に更新
- Backlog の「FileHandleBox」項目を消す
- 次の候補（append mode, metadata）を記載

---

## 7. 実装チェックリスト（Phase 110）

- [ ] phase110_filehandlebox_design.md が存在し、全 5 Task が記述されている
- [ ] Rust 側に FileHandleBox struct が追加されている
- [ ] FileHandleBox が NyashBox を実装している
- [ ] open/read/write/close/is_open メソッドが全て実装されている
- [ ] 二重 open が Err を返すことが確認されている
- [ ] close() 後のアクセスが Err を返すことが確認されている
- [ ] Default プロファイルでの基本動作テストが PASS
- [ ] NoFs プロファイルでの open Err テストが PASS
- [ ] FileBox の既存 API と挙動は変わっていない
- [ ] core_boxes_design.md / ring0-inventory.md / CURRENT_TASK.md が Phase 110 と整合している
- [ ] cargo build --release SUCCESS
- [ ] cargo test --release 全テスト PASS

---

## 8. 設計原則（Phase 110 で確立）

### 複数ファイルアクセス パターン

```
【1つのファイルを複数回】
  local h = new FileHandleBox()
  h.open("/file", "w")
  h.write("data1")
  h.write("data2")
  h.close()

【複数のファイル同時アクセス】
  local h1 = new FileHandleBox()
  h1.open("/file1", "r")

  local h2 = new FileHandleBox()
  h2.open("/file2", "w")

  h1.close()
  h2.close()
```

### Fail-Fast 原則の適用

- open() 呼び出し時に既に open 済み → 即座に Err
- close() 後の read/write → 即座に Err
- NoFs profile で open → 即座に Err

### 後方互換性

- FileBox は完全に独立（既存 API 変更なし）
- FileHandleBox は新規 Box として独立
- Ring0FsFileIo の変更なし

---

## 9. 将来への拡張ポイント

### Phase 111: append mode + metadata（完了 ✅）

- **append mode**: `mode = "a"` を実装、末尾に追記可能に
- **metadata API**: size / exists / is_file / is_dir を内部 Rust API として実装
- **FsApi.append_all()**: write_all と対称的に追加
- **実装完了**: Commit fce7555e で 4 つのテスト全て PASS

### Phase 112 以降の計画

- **Phase 112**: Ring0 Service Registry 統一化（metadata に modified フィールド追加）
- **Phase 113**: FileHandleBox NyashBox 公開 API（.hako から metadata 呼び出し可能に）
- **Phase 114**: FileIo 機能拡張（exists/stat/canonicalize を trait に追加）
- **Phase 115**: 並行アクセス安全性（Arc<Mutex<...>>）
- **Phase 116**: file encoding explicit 指定（UTF-8 以外）

---

**Phase 110 設計書作成日**: 2025-12-03（修正版 5点統合）
**Phase 111 完成日**: 2025-12-03（修正案統合版、4 テスト全 PASS）
Status: Historical
