# Phase 108: FileBox write/write_all 実装（Ring0 経由での書き込み有効化）

## 0. ゴール

- Phase 107 で作ったパイプライン：
  ```
  FileBox → Ring0FsFileIo (FileIo) → Ring0.FsApi → std::fs
  ```
  の **write 側を有効化** して、「FileBox でちゃんとファイルに書ける」状態を作る。

- 既存の Fail-Fast 方針（caps.write=false なら書けない）は維持しつつ、「標準プロファイルでは write が使える」ように する。

---

## 1. スコープと非スコープ

### スコープ（今回やること）

- FsApi / Ring0FsFileIo / FileBox の write/write_all 経路を実装
- FileCaps の write フラグを、標準プロバイダ（Ring0FsFileIo）では true にする
- 最小限のテキストログ用途（truncate mode）を実装・テスト

### 非スコープ（今回はやらない）

- 高度な機能（ローテーション、同時書き込みロック、append などの複数モード）
- FileBox API の破壊的変更（メソッド名や戻り値型の大きな変更）
- minimal/no-fs プロファイル本体（Phase 109 で扱う候補）

---

## 2. Task 1: 書き込みセマンティクスの設計（docs）

### 2.1 実装内容

**ファイル**:
- `docs/development/current/main/phase107_fsapi_fileio_bridge.md`（追記）
- **新規**: `phase108_filebox_write_semantics.md`（このドキュメント）

### 2.2 設計決定：write mode の明確化（重要）

**Phase 108 での write 挙動**:

```rust
pub fn write(&self, text: &str) -> FileResult<()> {
    // truncate mode: 既存ファイルは毎回上書き
    self.ring0.fs.write_all(Path::new(&self.path), text.as_bytes())
        .map_err(FileError::Io)
}
```

**採用理由**:
- ✅ シンプル実装（append モードは Phase 109+ で追加）
- ✅ ログ出力向け用途に最適
- ✅ テスト容易

**今後の計画**:
- Phase 109+: append メソッド追加時に、write/append の選択を柔軟化予定

### 2.3 テキスト vs バイナリ

**方針**:
- FileBox は **UTF-8 テキストファイル前提**（Phase 107 と同じ）
- write_all: `&[u8]` → `String` に変換して、text として書く
- バイナリ対応は「将来の拡張」（後フェーズ）

### 2.4 エラー処理

- FsApi.write_all の Err → `FileError::Io(String)` にラップ
- FileBox.write 層では：
  - 成功時: "OK" など固定 StringBox を返す
  - 失敗時: "Error: ..." を StringBox で返す（既存スタイル維持）

---

## 3. Task 2: FsApi / Ring0FsFileIo の write 実装

### 3.1 実装内容

**ファイル**:
- `src/runtime/ring0/traits.rs`
- `src/runtime/ring0/std_impls.rs`（FsApi の std 実装）
- `src/providers/ring1/file/ring0_fs_fileio.rs`

### 3.2 やること

1. **FsApi の write_all を確認**：
   ```rust
   pub trait FsApi: Send + Sync {
       fn read_file(&self, path: &Path) -> Result<Vec<u8>, IoError>;
       fn write_all(&self, path: &Path, content: &[u8]) -> Result<(), IoError>;
       // ...
   }
   ```
   - 既に実装済みなら そのまま使用
   - 無ければ `std::fs::write` に薄く委譲する実装を追加

2. **FileIo trait に write を追加**：
   ```rust
   pub trait FileIo: Send + Sync {
       fn caps(&self) -> FileCaps;
       fn open(&self, path: &str) -> FileResult<()>;
       fn read(&self) -> FileResult<String>;
       fn write(&self, text: &str) -> FileResult<()>;  // ← 新規追加
       fn close(&self) -> FileResult<()>;
   }
   ```

3. **Ring0FsFileIo に write メソッドを実装**：
   ```rust
   impl FileIo for Ring0FsFileIo {
       fn write(&self, text: &str) -> FileResult<()> {
           self.ring0.fs.write_all(Path::new(&self.path), text.as_bytes())
               .map_err(FileError::Io)
       }
   }
   ```

4. **FileCaps の write フラグを更新**：
   - Ring0FsFileIo.caps(): `FileCaps { read: true, write: true }`
   - read-only プロバイダや no-fs プロバイダでは `write: false` のままに（Fail-Fast と整合）

### 3.3 重要：FileIo trait 拡張時の互換性

write() を FileIo trait に追加することで、既存のテスト FileIo や mock も実装が必要になります。

**対応**:
1. Phase 107 tests の DummyFileIo を確認
2. 必要に応じて DummyFileIo に write stub 追加：
   ```rust
   fn write(&self, _text: &str) -> FileResult<()> {
       Err(FileError::Unsupported)  // テスト用は write 非対応
   }
   ```
3. すべてのテストが still pass することを確認

---

## 4. Task 3: FileBox の write/write_all を Ring0 経由に変更

### 4.1 実装内容

**ファイル**:
- `src/boxes/file/mod.rs`

### 4.2 やること

1. **FileBox.write_all(&self, buf: &[u8]) → Result<(), String>**：
   ```rust
   pub fn write_all(&self, buf: &[u8]) -> Result<(), String> {
       if let Some(ref provider) = self.provider {
           let caps = provider.caps();
           if !caps.write {
               return Err("Write not supported by FileBox provider".to_string());
           }
           // UTF-8 変換してプロバイダに委譲
           let text = String::from_utf8_lossy(buf).to_string();
           provider.write(&text)
               .map_err(|e| format!("Write failed: {:?}", e))
       } else {
           Err("No provider available".to_string())
       }
   }
   ```

2. **FileBox.write(&self, _content: Box<dyn NyashBox>) → Box<dyn NyashBox>**：
   ```rust
   pub fn write(&self, content: Box<dyn NyashBox>) -> Box<dyn NyashBox> {
       if let Some(ref provider) = self.provider {
           let caps = provider.caps();
           if !caps.write {
               return Box::new(StringBox::new(
                   "Error: write not supported by provider (read-only)".to_string()
               ));
           }
           // content を StringBox にダウンキャストして text を取得
           let text = if let Some(str_box) = content.as_any().downcast_ref::<StringBox>() {
               str_box.to_string_box().value
           } else {
               content.to_string_box().value
           };

           match provider.write(&text) {
               Ok(()) => Box::new(StringBox::new("OK".to_string())),
               Err(e) => Box::new(StringBox::new(format!("Error: {:?}", e))),
           }
       } else {
           Box::new(StringBox::new("Error: no provider available".to_string()))
       }
   }
   ```

3. **delete / copy は Phase 108 では stub のまま**：
   - docs に「将来実装予定」と明記
   - caps.write フラグ チェックは維持

### 4.3 戻り値の扱い

- write_all: `Result<(), String>` 維持（Rust API）
- write: `Box<dyn NyashBox>` 維持（.hako での使用パターン）
- 両者の使い分けは「Rust 側が使うか .hako 側が使うか」で判断

---

## 5. Task 4: テスト追加

### 5.1 実装内容

**ファイル候補**:
- `src/boxes/file/mod.rs` のテストモジュール
- `src/providers/ring1/file/ring0_fs_fileio.rs` のテスト

### 5.2 テストケース

**テスト 1: Round-trip（write → read）**
```rust
#[test]
fn test_filebox_write_read_roundtrip() {
    // 一時ファイル（/tmp/phase108_test.txt）に書き込み
    // その後同じ FileBox で read して内容一致確認
    // テスト終了後は cleanup
}
```

**テスト 2: Read-only provider での write 拒否**
```rust
#[test]
fn test_filebox_write_readonly_provider() {
    // caps.write=false な mock provider を使用
    // write() が Error StringBox を返すことを確認
}
```

**テスト 3: Double-open の挙動確認**（既存テスト参照）
```rust
#[test]
fn test_filebox_double_open() {
    // 同じ path で open 2回 → Err or overwrite
    // Phase 107 で決めたセマンティクスと一致していることを確認
    // (既存テスト test_ring0fs_fileio_double_open_error を参考に)
}
```

---

## 6. Task 5: docs 更新 & CURRENT_TASK 反映

### 6.1 実装内容

**ファイル**:
- `phase107_fsapi_fileio_bridge.md`（追記）
- **新規**: `phase108_filebox_write_semantics.md`（このドキュメント）
- `core_boxes_design.md`（FileBox セクション）
- `CURRENT_TASK.md`

### 6.2 やること

1. **phase107_fsapi_fileio_bridge.md に追記**：
   - Section 8「Phase 108 以降の計画」に一文：
     - 「Phase 108 で write 実装が完了し、FileBox は read/write 両対応になった」

2. **phase108_filebox_write_semantics.md を本ドキュメントとして保存**：
   - write mode（truncate）の設計
   - trait 拡張の互換性ポイント
   - テスト設計

3. **core_boxes_design.md に追記**：
   - FileBox セクションに：
     - 「FileBox は Ring0FsFileIo 経由で read/write をサポート（Phase 108）」
     - 「write は truncate mode（毎回上書き）」
     - 「append モードは Phase 109+ で予定」

4. **CURRENT_TASK.md に反映**：
   - Phase 108 完了行を追加
   - 次候補（Phase 109 minimal/no-fs, Phase 110 FileHandleBox, Phase 111 append mode）をバックログに記載

---

## 7. 実装チェックリスト（Phase 108）

- [ ] FsApi.write_all() 実装と Ring0FsFileIo::write が接続されている
- [ ] FileIo trait に write() が追加され、すべての実装が対応している
- [ ] DummyFileIo などテスト用 FileIo も write() stub を実装している
- [ ] FileBox.write / write_all が provider 経由で実際にファイルを書ける
- [ ] FileCaps.write が Ring0FsFileIo では true になっている
- [ ] Round-trip テスト（write → read）が PASS
- [ ] Read-only provider 拒否テストが PASS
- [ ] Double-open テストが既存セマンティクスと一致
- [ ] core_boxes_design / phase108 docs / CURRENT_TASK が更新済み
- [ ] ビルド・テスト完全成功（特に FileBox 関連）

---

## 8. 設計原則（Phase 108 で確立）

### FileBox I/O の完全パイプライン

```
[FileBox.write(content)]
        ↓
[FileBox.write_all(buf)]
        ↓
[provider.write(text)] ← Ring0FsFileIo が実装
        ↓
[Ring0.FsApi.write_all()]
        ↓
[std::fs::write()]
```

### プロファイル戦略

**標準プロファイル**:
- Ring0FsFileIo（write: true）→ FileBox は read/write 両対応

**将来の minimal/no-fs**:
- DummyFileIo（write: false）→ FileBox は read-only に

### 拡張ポイント

- Phase 109: append モード追加
- Phase 110: FileHandleBox（複数ファイル同時）
- Phase 111: 権限・ロック機構

---

## 9. Phase 109 以降の計画

### Phase 109: RuntimeProfile 機構の追加

**Phase 109 完了により、FileBox は conditional required に変更されました**：

- **RuntimeProfile enum 導入**（Default/NoFs）
- **Default profile**: FileBox は required（Phase 107/108 の動作を維持）
- **NoFs profile**: FileBox は optional（NoFsFileIo stub で無効化）

**設計変更**:
```rust
// Phase 109 以前: FileBox は常に required
CoreBoxId::File.is_core_required() // → true

// Phase 109 以降: profile 依存の判定に
CoreBoxId::File.is_required_in(&RuntimeProfile::Default) // → true
CoreBoxId::File.is_required_in(&RuntimeProfile::NoFs)    // → false
```

**プロファイル別動作**:
- **Default**: Ring0FsFileIo（read/write 両対応）自動登録
- **NoFs**: NoFsFileIo（全操作で Unsupported エラー）登録

**将来の拡張計画**（Phase 109 Modification 3）:
- TestMock: テスト用（全プラグインが mock に）
- Sandbox: サンドボックス（外部 I/O 禁止）
- ReadOnly: 読み取り専用（FileBox.write 禁止）
- Embedded: 組み込み（メモリ制限・GC あり）

**互換性**:
- Phase 107/108 の既存動作は Default profile で完全維持
- NoFs profile は完全に新規追加（既存コードに影響なし）

---

**Phase 108 指示書作成日**: 2025-12-03（微調整版）
**Phase 109 追記**: 2025-12-03（RuntimeProfile 統合完了）
Status: Historical
