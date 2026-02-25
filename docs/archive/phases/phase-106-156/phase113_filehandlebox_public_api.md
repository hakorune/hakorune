# Phase 113: FileHandleBox Nyash 公開 API

## 0. ゴール
- Phase 110–111 で実装した FileHandleBox の能力（open/read/write/close + "a" + metadata）を、
  Nyash (.hako) 側から「普通の Box メソッド」として使える形に公開する。

## 1. スコープと非スコープ

### スコープ（今回やること）

1. **設計ドキュメント**: 「公開メソッドセット」と挙動を定義
2. **Rust 側メソッド公開**: NyashBox trait / invoke_method() で MethodBox 登録
3. **MethodBox 登録**: BoxFactory に FileHandleBox メソッドテーブルを追加
4. **ディスパッチ方式**: StringBox と同じ動的ディスパッチパターン採用
5. **.hako サンプル**: 最小限の使用例を提示
6. **Profile 挙動確認**: Default / NoFs での動作明記
7. **ドキュメント更新**: core_boxes_design / ring0-inventory / CURRENT_TASK

### 非スコープ（今回はやらない）

- FileHandleBox を CoreBox に昇格（Phase 114+ 検討）
- Binary モード / 詳細テキストエンコーディング対応（Phase 114+）
- modified / created などの詳細メタデータ（Phase 114+）
- Exception / Result 型の統一（Phase 114+ 検討）
- プロファイル追加（TestMock/Sandbox/Embedded は Phase 114+）

## 2. 設計決定事項（Phase 113 確定）

| 項目 | 決定内容 | 理由 |
|------|---------|------|
| **Return Type** | すべて Void（エラーは panic）| 実装シンプルさ。Phase 114+ で Result 検討 |
| **Mode パラメータ** | "a", "r", "w" のみ | バイナリ対応は Phase 114+ |
| **Box クラス体系** | MethodBox のみ（CoreBox 化は Phase 114+）| Phase 113 は最小スコープ |
| **メソッドディスパッチ** | NyashBox trait メソッド直接実装 | 既存の NyashBox パターンに従う |
| **NoFs プロファイル** | open は panic、他は no-op | Ring0Registry による自動無効化 |
| **if 文条件** | Nyash 既実装のはず（確認）| 疑似コードで Bool 直接使用可能 |
| **テスト方式** | Rust ユニット + .hako 統合テスト両方 | カバレッジ完全化 |

## 3. Task 1: 公開 API の設計

### 3.1 メソッドセット（Nyash 側公開）

```
I/O メソッド:
- open(path: String, mode: String) -> Void
  * mode: "r"=read, "w"=write(truncate), "a"=append
  * パニック on エラー（Mode validation など）

- read() -> String
  * 全内容をいっぺんに読む
  * パニック on エラー or ファイル未open

- write(text: String) -> Void
  * data を書く（mode="w" or "a" での動作に従う）
  * パニック on not open / mode mismatch

- close() -> Void
  * ハンドルを閉じる（Rust 側で file クローズ）
  * 既に closed なら no-op

メタデータ メソッド:
- exists() -> Bool
  * パス存在確認（path は open() 時に保持）

- size() -> Integer
  * ファイルサイズをバイト単位で返す
  * パニック on ファイルなし / metadata 取得失敗

- isFile() -> Bool
  * 通常ファイルか確認

- isDir() -> Bool
  * ディレクトリか確認
```

### 3.2 疑似コード例

```nyash
box FileExample {
    main() {
        local h = new FileHandleBox()

        // ファイル追記
        h.open("/tmp/log.txt", "a")
        h.write("hello\n")
        h.close()

        // ファイル読み込みと統計
        h.open("/tmp/log.txt", "r")
        local content = h.read()

        if h.exists() {
            local n = h.size()
            print("Size: " + n)
        }
        h.close()
    }
}
```

### 3.3 プロファイル別動作

**Default プロファイル**:
- Ring0FsFileIo → FsApi（StdFs）経由でファイルシステムアクセス
- open/read/write/close/exists/size すべて正常動作

**NoFs プロファイル**:
- open() → panic!（"FileSystem operations disabled in no-fs profile"）
- read/write/close → open に達しないので呼ばれない（no-op）
- exists/size → false / panic（メタデータ取得不可）
  * もしくは open せずに呼ばれた場合は panic

## 4. Task 2: Rust 側メソッド公開

### 4.1 実装内容

ファイル:
- `src/boxes/file/handle_box.rs`（既存）
- `src/boxes/mod.rs` or `src/nyash_box.rs`（trait 周辺）
- `src/boxes/factory.rs` or `BoxFactory`（登録）

やること:

1. **FileHandleBox に Nyash メソッド実装**:
   ```rust
   impl FileHandleBox {
       // Nyash-visible methods
       pub fn ny_open(&mut self, path: &str, mode: &str) {
           self.open(path, mode).unwrap_or_else(|e| panic!("{}", e));
       }

       pub fn ny_read(&self) -> StringBox {
           match self.read_to_string() {
               Ok(content) => StringBox::new(content),
               Err(e) => panic!("{}", e),
           }
       }

       pub fn ny_write(&self, text: &str) {
           self.write_all(text).unwrap_or_else(|e| panic!("{}", e));
       }

       pub fn ny_close(&mut self) {
           self.close().unwrap_or_else(|e| panic!("{}", e));
       }

       pub fn ny_exists(&self) -> BoolBox {
           match self.exists() {
               Ok(result) => BoolBox::new(result),
               Err(e) => panic!("{}", e),
           }
       }

       pub fn ny_size(&self) -> IntegerBox {
           match self.size() {
               Ok(size) => IntegerBox::new(size as i64),
               Err(e) => panic!("{}", e),
           }
       }

       pub fn ny_is_file(&self) -> BoolBox {
           match self.is_file() {
               Ok(result) => BoolBox::new(result),
               Err(e) => panic!("{}", e),
           }
       }

       pub fn ny_is_dir(&self) -> BoolBox {
           match self.is_dir() {
               Ok(result) => BoolBox::new(result),
               Err(e) => panic!("{}", e),
           }
       }
   }
   ```

2. **BoxFactory / MethodBox 登録**:
   - FileHandleBox の box type 名を factory に登録
   - メソッドテーブル: ("open", arity=2), ("read", 0), ("write", 1), ("close", 0),
     ("exists", 0), ("size", 0), ("isFile", 0), ("isDir", 0)
   - 既存の StringBox/IntegerBox と同じパターンで登録

3. **実装パターン**:
   - NyashBox trait の既存メソッドを活用
   - メソッド呼び出しは as_any_mut() でダウンキャストして直接呼び出し
   - エラーハンドリングは panic! で統一（Phase 113）

### 4.2 テスト（Rust 側）

```rust
#[test]
fn test_filehandlebox_ny_open_read_default_profile() {
    let profile = RuntimeProfile::Default;
    let ring0 = Ring0Registry::build(profile);

    let mut handle = FileHandleBox::new(ring0);

    // open でテストファイルを作成
    let path = "/tmp/phase113_test_open_read.txt";
    handle.ny_open(path, "w");

    // write する
    handle.ny_write("test content\n");

    // close する
    handle.ny_close();

    // 再度 open して読む
    handle.ny_open(path, "r");

    // read する
    let content = handle.ny_read();
    assert_eq!(content.value, "test content\n");

    handle.ny_close();

    // cleanup
    std::fs::remove_file(path).ok();
}

#[test]
#[should_panic(expected = "disabled")]
fn test_filehandlebox_nofs_profile_panic() {
    let profile = RuntimeProfile::NoFs;
    let ring0 = Ring0Registry::build(profile);

    let mut handle = FileHandleBox::new(ring0);

    // NoFs では open が panic
    handle.ny_open("/tmp/test", "a");
}

#[test]
fn test_filehandlebox_metadata_methods() {
    let path = "/tmp/phase113_metadata_test.txt";
    std::fs::write(path, "hello").unwrap();

    let ring0 = Ring0Registry::build(RuntimeProfile::Default);
    let mut handle = FileHandleBox::new(ring0);

    handle.ny_open(path, "r");

    // Test metadata methods
    assert!(handle.ny_exists().value);
    assert_eq!(handle.ny_size().value, 5);
    assert!(handle.ny_is_file().value);
    assert!(!handle.ny_is_dir().value);

    handle.ny_close();
    std::fs::remove_file(path).ok();
}
```

## 5. Task 3: .hako サンプルと統合テスト

### 5.1 サンプル .hako ファイル

ファイル: `apps/examples/file_handle/append_and_stat.hako`

```nyash
local h = new FileHandleBox()

// 初回: append モードで書き込み
h.open("/tmp/example_log.txt", "a")
h.write("First line\n")
h.close()

// 再度: append モードで追記
h.open("/tmp/example_log.txt", "a")
h.write("Second line\n")
h.close()

// Read mode で全内容を読む
h.open("/tmp/example_log.txt", "r")
local content = h.read()
print(content)

// メタデータ確認
if h.exists() {
    local size = h.size()
    print("File size: " + size)
}
h.close()
```

### 5.2 統合テスト（.hako 実行）

ファイル: `src/runner/tests/filehandlebox_public_api_test.rs` (新規)

内容:
```rust
#[test]
fn test_filehandlebox_public_api_append_and_read() {
    // .hako ファイルを実行し、出力を検証
    let output = run_nyash_example("apps/examples/file_handle/append_and_stat.hako");

    assert!(output.contains("First line"));
    assert!(output.contains("Second line"));
    assert!(output.contains("File size:"));
}

#[test]
fn test_filehandlebox_nofs_disabled() {
    // NYASH_RUNTIME_PROFILE=no-fs で実行した場合、
    // open がパニックして適切なエラーメッセージが出るか確認
    let output = run_nyash_with_profile(
        "apps/examples/file_handle/append_and_stat.hako",
        "no-fs"
    );

    assert!(output.contains("disabled") || output.contains("error"));
}
```

## 6. Task 4: Profile / Ring0 統合確認

ファイル: phase111 / phase112 に追記

追記内容:

**phase111_filehandlebox_append_metadata.md**:
```markdown
### 補足: Phase 113 との関連
- Phase 113 で、これらの Rust メソッドが .hako 側に公開される。
- ny_read(), ny_size() など Nyash-visible メソッドとして提供。
```

**phase112_ring0_registry_design.md**:
```markdown
### FileHandleBox の Ring0 依存
- FileHandleBox は Ring0FsFileIo を内部で保持し、Ring0.fs に依存。
- Ring0Registry で NoFsApi が設定されると、自動的に FileHandleBox.open() は fail（panic）する。
- プロファイル切り替え時の挙動は Phase 113 で明記。
```

## 7. Task 5: ドキュメント更新

### 7.1 core_boxes_design.md への追記

追記位置: FileHandleBox セクション（既存）

```markdown
### Section N: Phase 113 - FileHandleBox Nyash 公開 API

#### 概要
FileHandleBox の内部メソッド（open/read/write/close/exists/size など）を
NyashBox trait の標準パターンで Nyash (.hako) 側に公開。

#### 公開メソッド
- open(path: String, mode: "r"|"w"|"a") -> Void (panic on error)
- read() -> String
- write(text: String) -> Void
- close() -> Void
- exists() -> Bool
- size() -> Integer
- isFile() -> Bool
- isDir() -> Bool

#### メソッドディスパッチ
NyashBox trait の標準パターン。ny_* メソッドとして実装。

#### Profile 別動作
| Profile | open | read/write | exists/size |
|---------|------|-----------|------------|
| Default | ✅ OK | ✅ OK | ✅ OK |
| NoFs | ❌ panic | - | ❌ panic |
```

### 7.2 ring0-inventory.md への追記

```markdown
## Phase 113: FileHandleBox Nyash 公開 API

- 設計: NyashBox trait 標準パターンで実装
- 実装: ny_* メソッド群追加（panic ベースエラーハンドリング）
- テスト: Rust ユニット + .hako 統合テスト両方
- Profile 対応: Default/NoFs 確認済み
```

### 7.3 CURRENT_TASK.md への追記

完了行として追加:
```
| Phase 113 | FileHandleBox Nyash API 公開 | ✅ 完了 | open/read/write/close/exists/size 公開、MethodBox 登録、Profile 対応確認 |
```

## 8. 完成チェックリスト（Phase 113）

- [ ] phase113_filehandlebox_public_api.md が完成（設計+実装詳細記載）
- [ ] FileHandleBox に ny_* メソッド実装済み
- [ ] BoxFactory に FileHandleBox メソッドテーブル登録完了
- [ ] .hako から new FileHandleBox() → open/read/write/close/exists/size 呼び出し可能
- [ ] Rust ユニットテスト: Default プロファイルで全メソッド動作確認
- [ ] Rust ユニットテスト: NoFs プロファイルで panic/no-op 動作確認
- [ ] .hako 統合テスト: append_and_stat.hako が実行・出力確認可能
- [ ] core_boxes_design.md / ring0-inventory.md / CURRENT_TASK.md 更新完了

## 9. 設計原則（Phase 113 で確立）

### NyashBox Standard Pattern

```
Nyash (.hako)
    ↓ Box method call
NyashBox trait methods (direct call)
    ↓
FileHandleBox::ny_*() methods
    ↓ delegate
Rust internal methods (open/read/write/close/exists/size)
```

### Profile カスケード

```
Phase 113 では何もしない
→ Ring0Registry が NoFsApi を設定
→ FileHandleBox.open() が Ring0FsFileIo 経由で FsApi.write_all() 呼び出し
→ NoFsApi が Err を返す
→ FileHandleBox.open() が panic
```

## 10. 実装メモ

### 10.1 メソッド命名規則
- Rust internal: `open()`, `read_to_string()`, `write_all()`, `close()`, etc.
- Nyash-visible: `ny_open()`, `ny_read()`, `ny_write()`, `ny_close()`, etc.
- この命名により、内部実装と公開 API を明確に区別

### 10.2 エラーハンドリング戦略
- Phase 113: panic ベース（unwrap_or_else）
- Phase 114+: Result<T, E> 型への移行を検討
- 理由: シンプルさ優先、段階的な実装

### 10.3 Profile 対応
- Default: 全機能有効
- NoFs: open() で即座に panic
- Phase 114+: TestMock/Sandbox プロファイル追加

---

**Phase 113 実装予定完了日**: 2025-12-04
**実装者**: Claude Code + ChatGPT 協働
**レビュー**: Phase 114 移行時に Result 型統合を検討
Status: Historical
