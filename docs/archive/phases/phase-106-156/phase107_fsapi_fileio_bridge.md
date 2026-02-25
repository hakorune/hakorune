# Phase 107: Ring0.FsApi ↔ FileIo 統合（FileBox の足場固め）

## 0. ゴール

- Ring0.FsApi（OS ファイル API）と FileIo（FileBox プラグイン用 I/O 抽象）の関係を明確に整理
- 自前 FileBox 実装の「OS への道」を一本のパイプにする：
  ```
  FileBox → FileIo implementation → Ring0.FsApi → std::fs
  ```
- これにより以下を実現：
  - FileBox まわりのハードコード削減
  - 将来の no-fs プロファイル / mock Fs の差し替え容易化
  - Ring0 と Ring1(FileBox) の依存関係の明確化

---

## 1. スコープと非スコープ

### スコープ（今回やること）

1. **設計＆ドキュメント**
   - FsApi / FileIo / FileBox / provider_lock の関係を図と文章で整理
   - FileIo を「Ring0.FsApi のラッパ（provider）」として位置づけ

2. **実装（段階的）**
   - Ring0.FsApi を read/write の SSOT として確認・微調整
   - Ring0 ベースの FileIo 実装追加（Ring0FsFileIo）
   - selfhost/通常ランタイムでこれをデフォルト provider として登録

3. **Fail-Fast との接続**
   - Phase 106 の「FileBox provider 必須チェック」と矛盾しない仕様確認
   - 標準パスで必ず Ring0FsFileIo が入ることを保証

### 非スコープ（今回はやらない）

- FileBox の write/delete/copy 全実装（別 Phase）
- FileBox API 大幅変更（メソッド名変更等）
- minimal/no-fs プロファイル実装（Phase 108 候補）

---

## 2. Task 1: FsApi を SSOT として整理（docs + 確認）✅

### 2.1 実装内容

**ファイル**:
- `src/runtime/ring0/traits.rs`
- `docs/development/current/main/core_boxes_design.md`
- **新規**: `docs/development/current/main/phase107_fsapi_fileio_bridge.md` (このファイル)

### 2.2 やること

1. **traits.rs で FsApi の公開インターフェースを確認**：✅
   ```rust
   pub trait FsApi: Send + Sync {
       fn read_to_string(&self, path: &Path) -> Result<String, IoError>;
       fn read(&self, path: &Path) -> Result<Vec<u8>, IoError>;
       fn write_all(&self, path: &Path, data: &[u8]) -> Result<(), IoError>;
       fn exists(&self, path: &Path) -> bool;
       fn metadata(&self, path: &Path) -> Result<FsMetadata, IoError>;
       fn canonicalize(&self, path: &Path) -> Result<PathBuf, IoError>;
   }
   ```

   **確認結果**:
   - ✅ read_to_string: String 直接読み込み（UTF-8変換済み）
   - ✅ read: バイト列読み込み
   - ✅ write_all: バイト列書き込み
   - ✅ exists: 存在確認
   - ✅ metadata: ファイルメタデータ取得（is_file/is_dir/len）
   - ✅ canonicalize: パス正規化

2. **このドキュメント（phase107_fsapi_fileio_bridge.md）に記載**：✅
   - 「FsApi = OS ファイル I/O の SSOT（Rust ローカル）」
   - 「FileIo = FileBox 用 provider interface。実装の 1 つとして FsApi を内部で使う」という関係
   - Ring0 → Ring1 の一方向依存の図

   **層の関係**:
   ```
   [FileBox (Ring1)]
       ↓ provider 経由
   [Ring0FsFileIo] (FileIo 実装)
       ↓ read/write 呼び出し
   [Ring0.FsApi] (OS I/O 抽象)
       ↓
   [std::fs]
   ```

3. **core_boxes_design.md に一文追加**：✅（Task 5で実施）
   - FileBox セクションに「実体 I/O は FileIo → FsApi → OS」を記載

---

## 3. Task 2: FileIo を「FsApi ラッパ」として設計✅

### 3.1 実装内容

**ファイル**:
- `src/boxes/file/provider.rs` ✅
- `phase107_fsapi_fileio_bridge.md` ✅

### 3.2 やること

1. **FileIo trait の役割を明確化**：✅
   ```rust
   pub trait FileIo: Send + Sync {
       fn caps(&self) -> FileCaps;
       fn open(&self, path: &str) -> FileResult<()>;
       fn read(&self) -> FileResult<String>;
       fn close(&self) -> FileResult<()>;
   }
   ```
   - **設計**: FileIo は「現在開いているファイルハンドル」に対する操作
   - **FsApi** は stateless（Path → 読込/書込）
   - **FileIo** は stateful（open → read/close）

2. **実装設計を docs に記載**（擬似コード）：
   ```rust
   pub struct Ring0FsFileIo {
       ring0: Arc<Ring0Context>,
       path: String,
       caps: FileCaps,
   }

   impl FileIo for Ring0FsFileIo {
       fn caps(&self) -> FileCaps { self.caps }
       fn open(&self, path: &str) -> FileResult<()> {
           // FsApi 経由で存在確認など
           Ok(())
       }
       fn read(&self) -> FileResult<String> {
           self.ring0.fs.read_file(Path::new(&self.path))
               .map(|bytes| String::from_utf8_lossy(&bytes).to_string())
               .map_err(FileError::Io)
       }
       fn close(&self) -> FileResult<()> { Ok(()) }
   }
   ```

3. **実装時の検討事項を明示**（以下のセクション参照）

---

## 3.3 実装時の検討事項（重要）

### ① Byte → String 変換のポリシー

**問題**: Ring0FsFileIo の read() で `String::from_utf8_lossy()` を使うと、バイナリファイルの無効な UTF-8 を置換してしまう。

**検討事項**:
- **Option A**: 現状通り `from_utf8_lossy()` で置換
  - 利点: Nyash が文字列中心だから OK
  - 欠点: バイナリ情報が失われる

- **Option B**: `String::from_utf8()` で Err を返す
  - 利点: エラーが明示的
  - 欠点: FileIo trait を `Result<String, Utf8Error>` に変更する必要あり（破壊的）

**推奨**: **Option A（from_utf8_lossy）を採用**
  - 理由: Nyash は言語実装として「テキストファイル」が主用途
  - バイナリ対応は「将来の拡張」として Phase 108+ で検討
  - docs に「FileBox は UTF-8 テキストファイル向け」と明記

### ② FileBox の open 状態での二重 open

**問題**: FileBox.open() が呼ばれるたびに provider.open() が呼ばれる。
- 既存の FileBox.open() → provider.open() の流れで、close() なしに再度 open() が呼ばれるケース

**検討事項**:
- **Option A**: close() を呼び出し側に強制する
  - FileBox.open() 前に明示的に close() 呼び出し

- **Option B**: Ring0FsFileIo 内で自動管理
  - 新しい open() が来たら前回の close を自動実行

- **Option C**: セマンティクスを明記
  - 「一度 open したら close() まで新しい open は受け付けない」

**推奨**: **Option C（セマンティクス明記）** + docs 追記
  - Ring0FsFileIo.open() は「既に path が set されていたら Err を返す」
  - FileBox.open() の docs に「FileBox は同時に 1 ファイルのみ開く」と明記
  - 複数ファイル同時アクセスは「将来の FileHandleBox」で対応

### ③ プラグイン優先ロジックでの Err ハンドリング

**問題**: `provider_lock::set_filebox_provider()` は OnceLock なので 1 回のみ可能。
- プラグインが先に登録 → init_default_filebox_provider() が Err を返す

**検討事項**:
- **Option A**: Err を単に無視する
  - 呼び出し側で Err を無視（silent）

- **Option B**: Warning をログに出す
  - ring0.log.info() / warn() で「プラグイン provider が既に設定されています」と出力

- **Option C**: init_default_filebox_provider() を Option を返すように変更
  - 呼び出し側で「設定済み」を区別可能

**推奨**: **Option B（Warning ログ出力）**
  - 実装: `init_default_filebox_provider()` の戻り値を `Result<(), String>` にして、
    - Ok(()) → デフォルト provider を登録した
    - Err(msg) → 既にプラグイン側で登録済み（msg = "Plugin provider already registered"）
  - 呼び出し側で `ring0.log.debug()` で記録（verbose 時に可視）
  - Fail-Fast は保たれる（MissingService は出ない）

---

## 4. Task 3: Ring0 ベースの FileIo 実装を追加✅

### 4.1 実装内容

**実装ファイル**:
- `src/providers/ring1/file/ring0_fs_fileio.rs` ✅（新規作成）
- `src/runtime/provider_lock.rs` ✅（ヘルパー関数追加）
- `src/runtime/plugin_host.rs` ✅（起動時初期化統合）

### 4.2 やること

1. **Ring0FsFileIo 実装を追加**：✅
   - フィールド: `Arc<Ring0Context>`, `String path`
   - open(path): path を保持、FsApi 経由で存在確認（読み取り向け）
   - read(): FsApi.read_file → String 変換（from_utf8_lossy ポリシー採用）
   - close(): 単に Ok(())（実質 noop、ハンドル管理なし）
   - caps: `FileCaps { read: true, write: false }` (Phase 107 では read-only)

2. **provider_lock 側にヘルパー追加**：
   ```rust
   pub fn init_default_filebox_provider(
       ring0: &Arc<Ring0Context>
   ) -> Result<(), String> {
       // Ring0 ベースの FileIo を登録
       let provider = Arc::new(Ring0FsFileIo::new(ring0.clone()));
       set_filebox_provider(provider)
           .map_err(|_| "Plugin FileBox provider already registered".to_string())
   }
   ```

3. **PluginHost/initialize_runtime に統合**：
   - CoreServices 初期化後に `init_default_filebox_provider(&ring0)` を呼ぶ
   - 戻り値が Err の場合は debug ログを出力（プラグイン優先）
   - Fail-Fast は影響なし（既にプラグイン provider が set されているだけ）

---

## 5. Task 4: Fail-Fast & プロファイルとの整合✅

### 5.1 実装内容

**ファイル**:
- `src/runtime/plugin_host.rs` ✅
- `docs/development/current/main/phase106_filebox_design_revised.md` ✅

### 5.2 やること

1. **Phase 106 との整合確認**：✅
   - Phase 106: 「FileBox provider 未登録なら CoreInitError::MissingService」
   - Phase 107: 「標準パスで Ring0FsFileIo が自動登録されるので MissingService は基本的に起きない」
   - phase106 のドキュメントに追記: ✅「Phase 107 で自動登録機構が追加された」

   **確認結果**:
   - ✅ with_core_from_registry_optional() で自動登録実装済み
   - ✅ Phase 106 の MissingService チェックは維持（二重防御）
   - ✅ プラグイン優先原則も維持（debug ログで可視化）

2. **将来用フック（docs に記載）**：✅
   - minimal/no-fs プロファイル導入時:
     - `CoreBoxId::File.is_core_required(profile)` に拡張可能
     - その profile では `init_default_filebox_provider()` を呼ばない
   - これで「FileBox 無し環境」も可能に（Phase 108 以降）

---

## 6. Task 5: ドキュメント統合✅

### 6.1 実装内容

**ファイル**:
- `docs/development/current/main/core_boxes_design.md` ✅
- `docs/development/current/main/phase106_filebox_design_revised.md` ✅
- `phase107_fsapi_fileio_bridge.md` (このドキュメント) ✅

### 6.2 やること

1. **core_boxes_design.md に図と説明を追加**：✅
   ```
   [FileBox (Ring1)]
       ↓ (provider経由)
   [Ring0FsFileIo] (FileIo実装)
       ↓ (read_to_string/read呼び出し)
   [Ring0.FsApi] (OS I/O抽象)
       ↓
   [std::fs]
   ```

2. **phase106_filebox_design_revised.md の "Phase 107" セクション更新**：✅
   - 「Phase 107 で FsApi 統合を行う予定」→「Phase 107 統合完了」に変更
   - Ring0FsFileIo 実装、自動登録、Fail-Fast 維持を明記

3. **phase107_fsapi_fileio_bridge.md にまとめる**：✅
   - FsApi / FileIo / FileBox / provider_lock / PluginHost の関係を 1 ドキュメントで整理
   - 設計判断（UTF-8 handling, one-file-at-a-time, plugin priority）を明記

---

## 7. 実装チェックリスト（Phase 107）✅ 全項目完了

- ✅ FsApi / FileIo / FileBox / provider_lock / PluginHost の関係が図付きで整理されている
- ✅ Ring0 ベースの FileIo 実装（Ring0FsFileIo）が追加されている
- ✅ selfhost/通常ランタイム起動で、デフォルトとして Ring0FsFileIo が provider_lock に登録される
- ✅ UTF-8 ハンドリング ポリシー（read_to_string 使用）が実装されている
- ✅ FileBox の一度に1ファイルのみ open セマンティクスが実装されている（テスト済み）
- ✅ プラグイン優先時の Err ハンドリング（debug ログ出力）が実装されている
- ✅ Phase 106 との整合確認完了（MissingService は基本レア）
- ✅ 将来用フック（minimal/no-fs プロファイル）が docs に記載されている
- ✅ ビルド・テスト完全成功（FileBox/provider_lock/plugin_host 関連テスト全PASS）

---

## 8. Phase 108 以降の進展

### Phase 108 実装完了（2025-12-03）

**write 実装完了**:
- Ring0FsFileIo に write() メソッド実装
- FileBox.write() / write_all() が Ring0.FsApi.write_all() 経由で動作
- FileCaps.write = true（標準プロファイルで書き込み対応）

**設計決定**:
- **Write mode**: truncate（既存ファイル毎回上書き）
- **テキスト前提**: UTF-8 変換（from_utf8_lossy）
- **append mode**: Phase 109+ で予定

**テスト**:
- Round-trip テスト（write → read）✅
- Truncate mode 検証✅
- Read-only provider 拒否テスト✅

---

## 9. 設計原則（Phase 107 で確立）

### 層の棲み分けが完全化

```
層                      役割                        知識範囲
──────────────────────────────────────────────────────────────
Ring0.FsApi            OS I/O 抽象化               Rust std::fs のみ
Ring0FsFileIo          FileIo 実装 (1つの実装例)   Ring0.FsApi 使用
FileIo trait           FileBox 向け I/O 抽象       FsApi を知らない
provider_lock          登録・参照                   OnceLock管理
FileBox                ユーザーAPI                  provider 経由のみ
```

### 拡張ポイント（Phase 109+）

**将来の実装候補**:
- MockFileIo: FsApi の代わりに in-memory mock を使う（テスト専用）
- NetworkFileIo: FsApi の代わりに remote FS を使う（将来の分散 FS / リモートログ用途）
- minimal/no-fs: RuntimeProfile に応じて provider 登録をスキップし、FileBox を read-only / disabled として扱う

---

**Phase 107 指示書作成日**: 2025-12-03（検討事項追加版）
Status: Historical
