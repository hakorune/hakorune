# Core Boxes 設計ドキュメント（Phase 87–95 完了版）

Phase 87 で実装された CoreBoxId/CoreMethodId と、Phase 91–95 で統合された CoreServices/PluginHost/Adapter/Service API の仕様。

**目的**: Box名・メソッド名のハードコードを型安全な enum に箱化することで、以下を実現：
- ✅ コンパイル時検証（タイポ撲滅）
- ✅ IDE 支援（補完・リファクタリング）
- ✅ SSOT（Single Source of Truth）確立
- ✅ 保守性向上

**実装状況**:
- ✅ Phase 87: CoreBoxId/CoreMethodId 実装
- ✅ Phase 91: CoreServices/PluginHost skeleton
- ✅ Phase 94: Box → Service Adapter 実装と Dummy 削除
- ✅ Phase 95: CoreServices 実用化（Console/String）+ global accessor

---

## 1. CoreBoxId — コア Box の識別子（実装完了）

### 1.1 役割

- Nyash 言語仕様的に「コア」とみなす Box の ID を定義する。
- 文字列名 `"StringBox"` / `"ArrayBox"` をここに集約し、他のモジュールは enum 経由で参照する。
- 例:
  - 予約型チェック（BoxFactory）
  - core_required 判定（PluginHost / FactoryPolicy）
  - 将来の Method Registry との連携など。

### 1.2 想定 API

```rust
pub enum CoreBoxId {
    String,
    Integer,
    Bool,
    Float,
    Null,
    Array,
    Map,
    Result,
    Method,
    Console,
    File,
    // 将来: Path, Json, Time, Regex, etc.
}

impl CoreBoxId {
    /// "StringBox" / "ArrayBox" などの内部名
    pub fn name(&self) -> &'static str;

    /// 将来のための iterator（予約型チェックなどで利用）
    pub fn iter() -> impl Iterator<Item = CoreBoxId>;
}
```

### 1.3 補足

- Ring0Context からはこの enum を直接は参照しない（あくまで ring1-core の責務）。
- `is_reserved_type(name: &str)` のような関数は、この enum を使って実装する想定：

```rust
fn is_reserved_type(name: &str) -> bool {
    CoreBoxId::iter().any(|id| id.name() == name)
}
```

---

## 2. CoreMethodId — コアメソッドの識別子

### 2.1 役割

- 「どの Box のどのメソッドか」を安全に表現する ID。
- メソッド名（`"length"` / `"push"` など）や arity を 1 箇所に集約し、呼ぶ側は enum だけを見る。
- 代表例:
  - `StringBox.length/0`
  - `ArrayBox.push/1`
  - `MapBox.get/1`
  - `ConsoleBox.println/1`

### 2.2 想定 API

```rust
pub enum CoreMethodId {
    // StringBox
    StringLength,
    StringSubstring,
    StringLastIndexOf,
    StringEscJson,

    // ArrayBox
    ArraySize,
    ArrayGet,
    ArraySet,
    ArrayPush,

    // MapBox
    MapSize,
    MapHas,
    MapGet,
    MapSet,

    // ConsoleBox
    ConsolePrint,
    ConsolePrintln,
    ConsoleLog,

    // FileBox
    FileOpen,
    FileRead,
    FileClose,
}

impl CoreMethodId {
    /// このメソッドが属する Box（CoreBoxId）
    pub fn box_id(&self) -> CoreBoxId;

    /// 実際のメソッド名（"length" など）
    pub fn name(&self) -> &'static str;

    /// 引数個数（将来可変長も考慮）
    pub fn arity(&self) -> usize;
}
```

### 2.3 用途の例

- Method Registry / BoxFactory でのメソッド解決：

```rust
fn resolve_core_method(box_name: &str, method_name: &str, arity: usize) -> Option<CoreMethodId> {
    CoreMethodId::iter().find(|id| {
        id.box_id().name() == box_name && id.name() == method_name && id.arity() == arity
    })
}
```

- 型推論や BoxCall lowering で、「この CoreMethodId なら戻り値型は Integer」という判定に使う。

---

## 3. BoxFactory / PluginHost との関係

### 3.1 BoxFactory 側

- 予約型チェックや FactoryPolicy の判定を、CoreBoxId ベースに書き換える想定：

```rust
fn is_core_required(name: &str) -> bool {
    matches!(
        CoreBoxId::from_name(name),
        Some(CoreBoxId::String | CoreBoxId::Integer | CoreBoxId::Bool
            | CoreBoxId::Float | CoreBoxId::Null
            | CoreBoxId::Array | CoreBoxId::Map | CoreBoxId::Result | CoreBoxId::Method)
    )
}
```

### 3.2 PluginHost 側

- `PluginHost.core: CoreServices` の設計と合わせて、CoreBoxId に基づいた必須サービスセットを定義する：

```rust
pub struct CoreServices {
    pub string: Arc<dyn StringService>,
    pub array: Arc<dyn ArrayService>,
    pub map: Arc<dyn MapService>,
    pub console: Arc<dyn ConsoleService>,
    // ...
}
```

- 初期化時に CoreServices の全フィールドが埋まっているかチェックし、足りなければ起動時に fail-fast する方針にする。

---

## 4. Ring0Context との分離

- Ring0Context はあくまで「OS API」の箱であり、CoreBoxId/CoreMethodId を知らない。
- CoreBoxes は Ring1-core 層として、Ring0Context の上に構築される：

```rust
// Ring0: OS API
pub struct Ring0Context { pub io: Box<dyn IoApi>, /* ... */ }

// Ring1-core: Box 実装（StringBox/ArrayBox/MapBox/FileBox/ConsoleBox）
//             CoreBoxId/CoreMethodId で識別される
```

これにより、ring0 側の変更（IO 実装差し替えなど）が CoreBoxes の識別や FactoryPolicy に影響しないようにできる。

---

## 5. Phase 87 実装完了

### 5.1 実装内容

✅ **CoreBoxId enum 定義** (`src/runtime/core_box_ids.rs`)
- 19個の CoreBox: core_required (6個), core_optional (9個), 特殊型 (4個)
- API: `name()`, `from_name()`, `is_core_required()`, `category()`, `iter()`

✅ **CoreMethodId enum 定義** (`src/runtime/core_box_ids.rs`)
- 30個のメソッド: StringBox (8), IntegerBox (3), BoolBox (3), ArrayBox (4), MapBox (4), ConsoleBox (3), FileBox (3), ResultBox (2)
- API: `box_id()`, `name()`, `arity()`, `return_type_name()`, `from_box_and_method()`, `iter()`

✅ **is_reserved_type() リファクタリング** (`src/box_factory/mod.rs`)
- ハードコード matches! → CoreBoxId による型安全判定
- 環境変数 NYASH_USE_PLUGIN_BUILTINS / NYASH_PLUGIN_OVERRIDE_TYPES 対応維持

✅ **infer_boxcall_return_type() リファクタリング** (`src/mir/builder/utils.rs`)
- 75行のハードコード → 25行の CoreMethodId ベース実装（**67%削減**）
- 型推論が SSOT 化され、保守性が大幅向上

✅ **テスト 11件追加** (`src/runtime/core_box_ids.rs`)
- CoreBoxId: from_name, name, iter, is_core_required, category (5件)
- CoreMethodId: box_id, name, arity, return_type_name, from_box_and_method, iter (6件)

### 5.2 実装効果

| 項目 | Before | After | 効果 |
|------|--------|-------|------|
| infer_boxcall_return_type() | 75行 | 25行 | **67%削減** |
| is_reserved_type() | 12行 | 9行 | 25%削減 |
| 型安全性 | 文字列ハードコード | enum | タイポ不可能 |
| SSOT | 分散 | 1ファイル | 保守性向上 |
| IDE支援 | なし | 補完可能 | 開発体験向上 |

### 5.3 Phase 85 との関係（FileBox 再分類）

Phase 85 の時点では、次の 3 区分で Box を分類していた：
- **core_required (6個)**: StringBox, IntegerBox, BoolBox, ArrayBox, MapBox, ConsoleBox
- **core_optional (9個)**: FloatBox, NullBox, FileBox, PathBox, RegexBox, MathBox, TimeBox, JsonBox, TomlBox
- **特殊型 (4個)**: FunctionBox, ResultBox, MethodBox, MissingBox

その後、Ring0/Ring1-Core の整理と selfhost ラインの安定化を進める中で、
FileBox は selfhost/通常ランタイムでは事実上必須（ログ・ツール・ハコチェックなどで常用）
であることが明確になったため、「core_required 相当」として扱うよう設計を更新した。

現行の分類は次の通り：
- **core_required (7個)**: StringBox, IntegerBox, BoolBox, ArrayBox, MapBox, ConsoleBox, FileBox
- **core_optional (8個)**: FloatBox, NullBox, PathBox, RegexBox, MathBox, TimeBox, JsonBox, TomlBox
- **特殊型 (4個)**: FunctionBox, ResultBox, MethodBox, MissingBox

最終的なソース・オブ・トゥルースは `src/runtime/core_box_ids.rs` の `CoreBoxId::is_core_required()` /
`CoreBoxId::category()` であり、このドキュメントはその意図を補足する設計メモとして位置づけている。

## Phase 106: 設計統一（案B）

### 責務分離原則

- **CoreBoxId**: 「必須かどうか」の判定（is_core_required() / category()）
  - selfhost/default では File が必須
  - 将来 minimal/no-fs プロファイルでは optional に変更可能
- **provider_lock**: 「FileBox provider を登録・読む」のみ（シンプルなロック機構）
- **PluginHost**: startup 時に CoreBoxId.is_core_required() で provider をチェック
  - 未登録なら CoreInitError::MissingService で fail-fast

### Ring0.FsApi との関係（Phase 107-108 完了）✅

**Phase 107 統合完了（2025-12-03）**:

FileBox の実体 I/O は、以下の層構造で Ring0.FsApi を通す設計が確立：

```
[FileBox (Ring1)]
    ↓ provider 経由
[Ring0FsFileIo] (FileIo 実装)
    ↓ read_to_string/write_all 呼び出し
[Ring0.FsApi] (OS I/O 抽象)
    ↓
[std::fs]
```

**Phase 108 実装完了（2025-12-03）**:
- FileBox は Ring0FsFileIo 経由で **read/write 両対応**
- write は **truncate mode**（毎回上書き）
- append モードは Phase 109+ で予定

**設計原則**:
- **FileIo = stateful**（現在開いているファイルハンドルに対する操作）
  - open() でファイルを開く
  - read() で内容を読み込む
  - write() で内容を書き込む（Phase 108 追加）
  - close() でファイルを閉じる
- **FsApi = stateless**（Path → データの直接変換）
  - read_to_string(path) / write_all(path, data)
  - exists(path) / metadata(path)

**実装箇所**:
- `src/providers/ring1/file/ring0_fs_fileio.rs`: Ring0FsFileIo 実装
- `src/runtime/provider_lock/mod.rs`: init_default_filebox_provider() ヘルパー
- `src/runtime/plugin_host.rs`: 起動時自動登録

### 5.4 Phase 109 - RuntimeProfile 機構（2025-12-03 完了）✅

**ゴール**:
- FileBox を **profile 依存の conditional required** に変更
- Default profile（selfhost/standard）では FileBox 必須を維持
- NoFs profile（minimal runtime）では FileBox を optional に

**実装内容**:

1. **RuntimeProfile enum 導入** (`src/runtime/runtime_profile.rs`):
   ```rust
   pub enum RuntimeProfile {
       Default,  // selfhost/standard
       NoFs,     // minimal runtime without filesystem
   }

   impl RuntimeProfile {
       pub fn from_env() -> Self {
           // NYASH_RUNTIME_PROFILE=no-fs → NoFs
           // それ以外 → Default
       }
   }
   ```

2. **CoreBoxId に profile-aware 判定追加** (`src/runtime/core_box_ids.rs`):
   ```rust
   pub fn is_required_in(&self, profile: &RuntimeProfile) -> bool {
       match profile {
           RuntimeProfile::Default => {
               // FileBox は required（Phase 106 互換）
               self.is_core_required()
           }
           RuntimeProfile::NoFs => {
               // FileBox は optional
               matches!(self, String | Integer | Bool | Array | Map | Console)
           }
       }
   }
   ```

3. **PluginHost に profile 引数追加** (`src/runtime/plugin_host.rs`):
   - `with_core_from_registry_optional(ring0, registry, config, profile)` に拡張
   - profile-aware FileBox provider チェック：
     - Default: provider 必須（CoreInitError::MissingService if None）
     - NoFs: provider なくても OK（黙って skip）

4. **NoFsFileIo stub 実装** (`src/providers/ring1/file/nofs_fileio.rs`):
   ```rust
   pub struct NoFsFileIo;

   impl FileIo for NoFsFileIo {
       fn caps(&self) -> FileCaps { FileCaps { read: false, write: false } }
       fn open/read/write/close → Err(FileError::Unsupported)
   }
   ```

5. **initialize_runtime() に profile 読み込み追加** (`src/runtime/mod.rs`):
   - 環境変数から profile を読む（**この層のみで実施**）
   - NoFs profile の場合、NoFsFileIo を登録
   - PluginHost に profile を渡す（**env に依存しない**）

**設計原則（Modification 1: 責務分離）**:
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

**Logger/ConsoleService の有効性（Modification 2）**:
- ✅ **NoFs profile でも有効**:
  - Ring0.log（OS抽象化層 - panic/exit 時の最終出力）
  - ConsoleBox（言語レベル console - stdout/stderr）
  - その他 core_required（String/Integer/Bool/Array/Map/Console）
- ❌ **NoFs profile で無効**:
  - FileBox（ファイルシステム依存）
  - Regex/Time/JSON 等のオプショナル boxes（将来：profile ごとに制御可能）

**将来の拡張予定（Modification 3）**:
- **TestMock**: テスト用（すべてのプラグインが mock に）
- **Sandbox**: サンドボックス（外部 I/O 禁止）
- **ReadOnly**: 読み取り専用（FileBox.write 禁止）
- **Embedded**: 組み込み（メモリ制限あり、GC あり）

**実装箇所**:
- `src/runtime/runtime_profile.rs`: RuntimeProfile enum 定義
- `src/runtime/core_box_ids.rs`: is_required_in() メソッド
- `src/runtime/plugin_host.rs`: profile-aware 初期化ロジック
- `src/runtime/provider_lock/mod.rs`: init_filebox_provider_for_profile()
- `src/providers/ring1/file/nofs_fileio.rs`: NoFs stub 実装
- `src/runtime/mod.rs`: initialize_runtime() に profile 読み込み

**テスト**:
- ✅ test_core_box_id_is_required_in_default
- ✅ test_core_box_id_is_required_in_nofs
- ✅ test_with_core_from_registry_nofs_filebox_optional
- ✅ test_nofs_fileio_caps/open/read/write/close_unsupported

**互換性**:
- Phase 107/108 の既存動作は Default profile で完全維持
- NoFs profile は完全に新規追加（既存コードに影響なし）

### 5.5 今後の拡張（Phase 110+）

Phase 110 以降では、FileBox/FS 周りの扱いをプロファイルと Box 設計の両面から広げていく予定：
- **Phase 110: FileHandleBox**
  - FileBox は「1 ファイル専用 API」としてシンプルに保ち、複数ファイル同時アクセスは FileHandleBox 側に切り出す設計。
  - Ring0FsFileIo を内部で再利用しつつ、ハンドル単位で FsApi をラップする。
- **Phase 111: metadata API 整理**
  - `FsApi::metadata/exists/canonicalize` を FileIo / FileBox 側に橋渡しし、Nyash 側から stat 情報を扱えるようにする。

これらはすべて `CoreBoxId` / Ring0.FsApi / FileIo / FileBox の既存ラインの上に小さく積む形で設計する。

新しいメソッド追加は `src/runtime/core_box_ids.rs` の編集のみで完結：
1. CoreMethodId enum にバリアント追加
2. box_id(), name(), arity(), return_type_name() に対応追加
3. iter() にバリアント追加
4. テスト追加

すべて1ファイルで完結するため、Phase 84-4-B のような分散ハードコード問題は完全解消。

---

## 6. CoreBoxCategory enum（Phase 87 → 106）

### 6.1 役割

CoreBoxId の分類を型安全に表現する enum。Phase 85 調査結果をベースにしつつ、
Phase 106 で FileBox を CoreRequired 側に寄せたため、個数コメントは歴史的な値として扱う。

```rust
pub enum CoreBoxCategory {
    CoreRequired,  // 必須: 起動時に全て揃っていなければならない (現行 7個: String/Integer/Bool/Array/Map/Console/File)
    CoreOptional,  // オプション: 無くても起動できるが、標準機能として提供 (現行 8個)
    Special,       // 特殊: 言語実装に直結（Function/Result/Method/Missing）
}
```

### 6.2 現行対応表（Phase 106 時点）

| Category     | Box 一覧                                          |
|--------------|---------------------------------------------------|
| CoreRequired | String, Integer, Bool, Array, Map, Console, File |
| CoreOptional | Float, Null, Path, Regex, Math, Time, Json, Toml |
| Special      | Function, Result, Method, Missing                |

### 6.3 実装効果

- **is_core_required() の簡略化**: `category() == CoreBoxCategory::CoreRequired`
- **Phase 92 CoreServices 設計**: core_required (6個) のみを CoreServices に含める根拠
- **型安全な分類**: 文字列比較 → enum 比較で完全型安全

---

## 7. Phase 87 完了チェックリスト

### 7.1 実装完了 ✅

- [x] CoreBoxId enum 定義（19個）
- [x] CoreMethodId enum 定義（30個）
- [x] CoreBoxCategory enum 定義（3カテゴリ）
- [x] is_reserved_type() リファクタリング
- [x] infer_boxcall_return_type() リファクタリング（67%削減）
- [x] テスト 11件追加
- [x] Phase 85 調査結果の完全反映

### 7.2 Phase 92 への橋渡し準備 ✅

- [x] core_required (6個) の確定
- [x] CoreBoxCategory による分類体系確立
- [x] SSOT（Single Source of Truth）確立

### 7.3 今後の展開

Phase 92 では、CoreServices の実装時に CoreBoxCategory::CoreRequired のみを対象とすることで、  
**「core_required は必ず揃う」**という設計原則を型レベルで保証する（Phase 106 以降は FileBox を含む 7個）。

---

## 8. Phase 91: PluginHost/CoreServices 実装（2025-12-03）

### 8.1 PluginHost アーキテクチャ

**構造**:
```rust
pub struct PluginHost {
    pub ring0: Arc<Ring0Context>,     // Phase 88-90 実装済み
    pub core: CoreServices,            // Phase 91 skeleton
    pub optional: HashMap<String, Arc<dyn Any>>,
}
```

**役割**:
- Ring0Context（OS API）と CoreServices（Box 実装）の橋渡し
- core_required Box の初期化保証
- optional/user プラグインの管理

### 8.2 CoreServices: Ring1-Core の顔

**定義**:
```rust
pub struct CoreServices {
    pub string: Arc<dyn StringService>,
    pub integer: Arc<dyn IntegerService>,
    pub bool: Arc<dyn BoolService>,
    pub array: Arc<dyn ArrayService>,
    pub map: Arc<dyn MapService>,
    pub console: Arc<dyn ConsoleService>,
}
```

**Phase 87 CoreBoxId との対応**:

| CoreBoxId | CoreBoxCategory | CoreServices | 実装状況 |
|-----------|----------------|--------------|---------|
| String | CoreRequired | ✅ string | Phase 91 skeleton |
| Integer | CoreRequired | ✅ integer | Phase 91 skeleton |
| Bool | CoreRequired | ✅ bool | Phase 91 skeleton |
| Array | CoreRequired | ✅ array | Phase 91 skeleton |
| Map | CoreRequired | ✅ map | Phase 91 skeleton |
| Console | CoreRequired | ✅ console | Phase 91 skeleton |
| File | CoreRequired | (Ring1/FileBox) | Phase 106 で core_required 寄せ |

**設計原則**:
- core_required (6個) は全て CoreServices に含まれる
- 起動時に全フィールドが初期化されていなければ panic
- Phase 92 で UnifiedBoxRegistry から自動生成予定

### 8.3 NyashPlugin trait

**定義**:
```rust
pub trait NyashPlugin: Send + Sync {
    fn descriptor(&self) -> PluginDescriptor;
    fn register(&self, host: &mut PluginRegistry);
}
```

**用途**:
- プラグインの基本情報（名前、バージョン、capabilities）
- Phase 92 以降で既存プラグインシステムと統合予定

### 8.4 Phase 92 以降の計画

- UnifiedBoxRegistry から CoreServices への自動変換
- PluginHost.optional への optional/user プラグイン登録
- 既存 Box 実装と Service trait の接続

---

## 9. Phase 92: UnifiedBoxRegistry 統合計画（2025-12-03）

### 9.1 CoreServices と CoreBoxId の対応テーブル

**実装**: `src/runtime/core_services.rs`

```rust
impl CoreServices {
    pub fn required_ids() -> &'static [CoreBoxId] {
        &[
            CoreBoxId::String,
            CoreBoxId::Integer,
            CoreBoxId::Bool,
            CoreBoxId::Array,
            CoreBoxId::Map,
            CoreBoxId::Console,
        ]
    }
}
```

**Phase 87 整合性**: CoreBoxId::is_core_required() と完全一致

### 9.2 PluginHost 初期化フック

**実装**: `src/runtime/plugin_host.rs`

```rust
impl PluginHost {
    pub fn with_core_from_registry(
        ring0: Arc<Ring0Context>,
        registry: &UnifiedBoxRegistry,
    ) -> Result<Self, CoreInitError> {
        // Phase 93 で実装予定
        todo!("Phase 93: UnifiedBoxRegistry から CoreServices への変換実装")
    }
}
```

**エラー型**: `CoreInitError` で不足している core service を明示

### 9.3 Runtime 初期化接続ポイント

**実装**: `src/runtime/mod.rs`

```rust
pub fn initialize_runtime(ring0: Arc<Ring0Context>) -> Result<PluginHost, CoreInitError> {
    let registry = UnifiedBoxRegistry::new();
    let plugin_host = PluginHost::with_core_from_registry(ring0, &registry)?;
    Ok(plugin_host)
}
```

**Phase 93 で実装**: UnifiedBoxRegistry から CoreServices への実際の変換

### 9.4 ensure_initialized() 呼び出し場所（Phase 93 実装予定）

**決定事項**: Phase 93 以降で以下の4箇所で呼び出す

1. **selfhost ランナー起動時**（`src/runner/selfhost.rs`）
   ```rust
   pub fn run_selfhost(config: &Config) -> Result<(), Error> {
       let plugin_host = initialize_runtime(get_global_ring0())?;
       plugin_host.ensure_core_initialized();
       // ...
   }
   ```

2. **hack_check 実行前**（`src/runner/hack_check.rs`）
   ```rust
   pub fn run_hack_check() -> Result<(), Error> {
       let plugin_host = initialize_runtime(get_global_ring0())?;
       plugin_host.ensure_core_initialized();
       // ...
   }
   ```

3. **VM バックエンド起動時**（`src/runner/modes/vm.rs`）
   ```rust
   pub fn run_vm(program: &str) -> Result<(), Error> {
       let plugin_host = initialize_runtime(get_global_ring0())?;
       plugin_host.ensure_core_initialized();
       // ...
   }
   ```

4. **統合テスト setup**（`tests/integration/setup.rs`）
   ```rust
   pub fn setup_test_runtime() -> PluginHost {
       let plugin_host = initialize_runtime(get_global_ring0()).unwrap();
       plugin_host.ensure_core_initialized();
       plugin_host
   }
   ```

### 9.5 Phase 93 実装計画

- UnifiedBoxRegistry から core_required Box の取得
- Box → Service trait への変換ロジック
- CoreServices の自動構築
- ensure_initialized() の4箇所への配置

---

## 10. Phase 93: with_core_from_registry 実装（2025-12-03）

### 10.1 実装内容

**実装**: `src/runtime/plugin_host.rs`

```rust
impl PluginHost {
    pub fn with_core_from_registry(
        ring0: Arc<Ring0Context>,
        registry: &UnifiedBoxRegistry,
    ) -> Result<Self, CoreInitError> {
        // Phase 93: 各 core_required Box が registry に存在するか確認
        for id in CoreServices::required_ids() {
            let box_name = id.name();
            if !registry.has_type(box_name) {
                return Err(CoreInitError::MissingService {
                    box_id: *id,
                    message: format!("{} not found in registry", box_name),
                });
            }
        }

        // Phase 93: ダミー Service 実装で CoreServices を構築
        let core = CoreServices {
            string: Arc::new(DummyStringService),
            integer: Arc::new(DummyIntegerService),
            bool: Arc::new(DummyBoolService),
            array: Arc::new(DummyArrayService),
            map: Arc::new(DummyMapService),
            console: Arc::new(DummyConsoleService),
        };

        Ok(PluginHost {
            ring0,
            core,
            optional: HashMap::new(),
        })
    }
}
```

### 10.2 段階的展開戦略

**Phase 93 アプローチ**:
- 環境変数 `NYASH_USE_PLUGIN_HOST=1` で有効化
- ダミー Service 実装で動作確認
- selfhost のみ統合（hack_check は Phase 94）

**Phase 94 計画**:
- 実際の Box → Service 変換実装
- 環境変数削除（デフォルトで有効化）
- 全起動パスへの展開

### 10.3 Fail-Fast 設計

**メリット**:
- 起動時に core Box の不足を即座に検出
- CoreInitError で明示的なエラーメッセージ
- デバッグ容易（ランタイムエラーではなく起動時エラー）

### 10.4 動作確認方法

```bash
# Phase 93 動作確認
NYASH_USE_PLUGIN_HOST=1 ./target/release/nyash apps/tests/selfhost_min.hako

# Phase 93 では環境変数なしで従来通り動作
./target/release/nyash apps/tests/selfhost_min.hako
```

### 10.5 実装されたファイル

1. `src/runtime/plugin_host.rs`: with_core_from_registry() + ダミー Service 実装
2. `src/runtime/core_services.rs`: CoreServices::dummy() ヘルパー
3. `src/runtime/mod.rs`: initialize_runtime() 実装（環境変数制御）
4. `src/runner/selfhost.rs`: PluginHost 初期化追加

---

## 11. Phase 95: CoreServices 実用化（2025-12-03）

### 11.1 実装成果

- ✅ **ConsoleService API**: `println(msg)`, `print(msg)` 実装
- ✅ **StringService API**: `len(s) -> i64` 実装
- ✅ **ConsoleBoxAdapter**: 実際に println! を呼び出す実装
- ✅ **StringBoxAdapter**: UTF-8 文字数カウント実装
- ✅ **global accessor**: `get_core_plugin_host()` 実装
- ✅ **代表パス切り替え**: `src/runner/selfhost.rs` で ConsoleService 使用

### 11.2 Service API 定義

```rust
// ConsoleService: 最優先で実装
pub trait ConsoleService: Send + Sync {
    fn println(&self, msg: &str);
    fn print(&self, msg: &str);
}

// StringService: 2番目に実装
pub trait StringService: Send + Sync {
    fn len(&self, s: &str) -> i64;  // UTF-8 文字数
}

// ArrayService, MapService: Phase 96 で実装予定
pub trait ArrayService: Send + Sync { }
pub trait MapService: Send + Sync { }
```

### 11.3 Adapter 実装

**ConsoleBoxAdapter**:
```rust
impl ConsoleService for ConsoleBoxAdapter {
    fn println(&self, msg: &str) {
        // ConsoleBox は直接 println! を呼ぶだけなので、ここでも同様に実装
        println!("{}", msg);
    }

    fn print(&self, msg: &str) {
        print!("{}", msg);
    }
}
```

**StringBoxAdapter**:
```rust
impl StringService for StringBoxAdapter {
    fn len(&self, s: &str) -> i64 {
        // 文字列長を返す（UTF-8 バイト数ではなく文字数）
        s.chars().count() as i64
    }
}
```

### 11.4 global accessor パターン

Ring0Context と同じ OnceLock パターンで実装：

```rust
use std::sync::OnceLock;
static GLOBAL_CORE_PLUGIN_HOST: OnceLock<Arc<PluginHost>> = OnceLock::new();

pub fn init_core_plugin_host(host: PluginHost) {
    GLOBAL_CORE_PLUGIN_HOST.set(Arc::new(host))
        .expect("[Phase 95] CorePluginHost already initialized");
}

pub fn get_core_plugin_host() -> Arc<PluginHost> {
    GLOBAL_CORE_PLUGIN_HOST.get()
        .expect("[Phase 95] CorePluginHost not initialized")
        .clone()
}
```

### 11.5 代表パス切り替え例

**Before** (eprintln):
```rust
eprintln!("[selfhost] PluginHost initialized successfully");
```

**After** (ConsoleService):
```rust
let host = crate::runtime::get_core_plugin_host();
host.core.console.println("[selfhost] PluginHost initialized successfully");
```

### 11.6 テスト実装

- `test_console_service_println`: println 呼び出し確認
- `test_console_service_print`: print 呼び出し確認
- `test_string_service_len`: 文字列長（UTF-8 対応）確認

### 11.7 実装ファイル

1. `src/runtime/core_services.rs`: ConsoleService/StringService API 定義 + Adapter 実装（合計 266行）
2. `src/runtime/mod.rs`: global accessor 実装（77行追加）
3. `src/runtime/plugin_host.rs`: Debug impl 追加、`optional` 型修正
4. `src/runner/selfhost.rs`: ConsoleService 使用デモ（1箇所）

### 11.8 設計原則

- **CoreServices から Box の内部実装を隠蔽**: Service trait 経由で型安全アクセス
- **global accessor で簡単アクセス**: `get_core_plugin_host().core.console.println(...)`
- **Fail-Fast 原則維持**: エラー時は即座に失敗（フォールバック禁止）
- **段階実装**: Phase 95 では Console/String のみ、Phase 96 で Array/Map 追加予定

### 11.9 次のステップ（Phase 96）

- ArrayService 実装（push, get, set, size）
- MapService 実装（get, set, has, size）
- 代表パス拡大（selfhost 以外の箇所にも展開）
- StringService 拡張（substring, concat, replace 等）

---

## 12. Phase 95.5: Ring0 統合完了（2025-12-03）

### 12.1 実装成果

- ✅ **ConsoleService Ring0 直結**: Box を保持せず Ring0Context に直結
- ✅ **StringService 純粋関数化**: Box 状態不要な設計確立
- ✅ **#[allow(dead_code)] 削減**: 6箇所 → 4箇所（2削減）
- ✅ **ログ経路統一**: Ring0 → Ring1-Core → 実行パス

### 12.2 ConsoleService 設計

**Ring0 直結設計**:
```rust
pub struct ConsoleBoxAdapter;

impl ConsoleBoxAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl ConsoleService for ConsoleBoxAdapter {
    fn println(&self, msg: &str) {
        // Ring0Context 経由でログ出力（自動改行）
        use crate::runtime::ring0::get_global_ring0;
        let ring0 = get_global_ring0();
        ring0.log.info(msg);
    }

    fn print(&self, msg: &str) {
        // Ring0Context 経由で stdout 出力（改行なし）
        use crate::runtime::ring0::get_global_ring0;
        let ring0 = get_global_ring0();
        ring0.io.stdout_write(msg.as_bytes()).ok();
    }
}
```

**利点**:
- OS 抽象化レイヤー完全活用
- テスト容易性向上（Ring0 をモック可能）
- Box レイヤーをスキップして効率化
- ログ経路統一（全てのコンソール出力が Ring0 経由）

**設計原則**:
- ConsoleService は OS API (Ring0) の thin wrapper
- println() → Ring0Context.log.info()（ログレベル付き、自動改行）
- print() → Ring0Context.io.stdout_write()（改行なし、生出力）

### 12.3 StringService 設計

**純粋関数設計**:
```rust
pub struct StringBoxAdapter;

impl StringBoxAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl StringService for StringBoxAdapter {
    fn len(&self, s: &str) -> i64 {
        // 純粋関数として実装（Box 状態不要）
        s.chars().count() as i64
    }
}
```

**利点**:
- Box インスタンス不要（メモリ効率向上）
- 純粋関数として再現性保証
- テスト容易性（状態を持たない）

**Phase 96 以降の拡張**:
- substring(s, start, end) → 純粋関数
- concat(a, b) → 純粋関数
- replace(s, from, to) → 純粋関数

### 12.4 PluginHost 初期化変更

**Before** (Phase 95):
```rust
// StringBox
let string_box = registry
    .create_box("StringBox", &[])
    .map_err(|e| CoreInitError::MissingService {
        box_id: CoreBoxId::String,
        message: format!("StringBox creation failed: {}", e),
    })?;
let string_service = Arc::new(StringBoxAdapter::new(string_box));

// ConsoleBox
let console_box = registry
    .create_box("ConsoleBox", &[])
    .map_err(|e| CoreInitError::MissingService {
        box_id: CoreBoxId::Console,
        message: format!("ConsoleBox creation failed: {}", e),
    })?;
let console_service = Arc::new(ConsoleBoxAdapter::new(console_box));
```

**After** (Phase 95.5):
```rust
// StringBox (純粋関数化、存在チェックのみ)
if !registry.has_type("StringBox") {
    return Err(CoreInitError::MissingService {
        box_id: CoreBoxId::String,
        message: "StringBox not found in registry".to_string(),
    });
}
let string_service = Arc::new(StringBoxAdapter::new());

// ConsoleBox (Ring0 直結、存在チェックのみ)
if !registry.has_type("ConsoleBox") {
    return Err(CoreInitError::MissingService {
        box_id: CoreBoxId::Console,
        message: "ConsoleBox not found in registry".to_string(),
    });
}
let console_service = Arc::new(ConsoleBoxAdapter::new());
```

**変更点**:
- Box インスタンス生成不要（存在チェックのみ）
- エラーメッセージ簡略化
- メモリ効率向上

### 12.5 テスト更新

**Ring0 初期化ヘルパー**:
```rust
#[cfg(test)]
fn ensure_ring0_initialized() {
    use crate::runtime::ring0::{default_ring0, GLOBAL_RING0};
    use std::sync::Arc;

    // 既に初期化済みなら何もしない
    if GLOBAL_RING0.get().is_none() {
        GLOBAL_RING0.set(Arc::new(default_ring0())).ok();
    }
}
```

**テスト例**:
```rust
#[test]
fn test_console_service_ring0_integration() {
    // Ring0 初期化（安全な初期化）
    ensure_ring0_initialized();

    // ConsoleService 経由で出力（Ring0 使用）
    let adapter = ConsoleBoxAdapter::new();
    adapter.println("Test message from Ring0");
    adapter.print("No newline");

    // panic しないことを確認
}
```

### 12.6 削減統計

| 項目 | Before | After | 削減 |
|------|--------|-------|------|
| #[allow(dead_code)] | 6箇所 | 4箇所 | 2削減 |
| inner フィールド | 6個 | 4個 | 2削減 |
| Box 依存 | 6箇所 | 4箇所 | 2削減 |

**残存 Adapter**:
- IntegerBoxAdapter: Phase 96 以降で実装方針決定
- BoolBoxAdapter: Phase 96 以降で実装方針決定
- ArrayBoxAdapter: Phase 96 以降で実装方針決定
- MapBoxAdapter: Phase 96 以降で実装方針決定

### 12.7 設計原則確立

**2つの設計パターン**:

1. **Ring0 直結型**（ConsoleService）
   - OS API の thin wrapper
   - Box 状態不要
   - Ring0Context 経由でシステムコール

2. **純粋関数型**（StringService）
   - 副作用なし
   - Box 状態不要
   - 入力のみから出力を決定

**Phase 96 実装方針**:
- ArrayService: 状態管理が必要 → Box 保持型
- MapService: 状態管理が必要 → Box 保持型
- IntegerService: 純粋関数で足りる → 純粋関数型
- BoolService: 純粋関数で足りる → 純粋関数型

### 12.8 実装ファイル

1. `src/runtime/core_services.rs`
   - ConsoleBoxAdapter: Ring0 直結実装
   - StringBoxAdapter: 純粋関数化
   - 他4 Adapter: コメント追加
   - テスト: Ring0 統合テスト追加

2. `src/runtime/plugin_host.rs`
   - StringBox 初期化: 存在チェックのみ
   - ConsoleBox 初期化: 存在チェックのみ
   - テスト: エラーメッセージ更新

### 12.9 次のステップ（Phase 96）

- ArrayService/MapService 実装（状態管理が必要）
- IntegerService/BoolService 実装（純粋関数で実装）
- 代表パス拡大（5-10箇所）
- #[allow(dead_code)] 完全撲滅

---

## 13. Phase 96: ArrayService/MapService 実装 (2025-12-03)

### 13.1 実装成果

**完了項目**:
- ✅ ArrayService trait 定義（len/get/set/push）
- ✅ MapService trait 定義（size/has/get/set）
- ✅ ArrayBoxAdapter/MapBoxAdapter unit struct 化
- ✅ downcast パターンで複数インスタンス対応
- ✅ #[allow(dead_code)] 完全削除（4箇所 → 2箇所）

### 13.2 Adapter パターン完成

**3つのパターン確立**:

1. **Ring0直結型**（ConsoleService）
   - OS API thin wrapper
   - Box 状態不要
   - Ring0Context 経由でシステムコール

2. **純粋関数型**（StringService）
   - 副作用なし
   - Box 状態不要
   - 入力のみから出力を決定

3. **downcast型**（ArrayService/MapService）
   - 複数インスタンス対応
   - Box 状態が必要
   - unit struct + downcast パターン

### 13.3 ArrayService API

**Trait 定義**:
```rust
pub trait ArrayService: Send + Sync {
    /// 配列の要素数を取得
    fn len(&self, arr: &dyn NyashBox) -> i64;

    /// 指定インデックスの要素を取得
    fn get(&self, arr: &dyn NyashBox, index: i64) -> Option<Box<dyn NyashBox>>;

    /// 指定インデックスに要素を設定
    fn set(&self, arr: &dyn NyashBox, index: i64, value: Box<dyn NyashBox>) -> Result<(), String>;

    /// 配列の末尾に要素を追加
    fn push(&self, arr: &dyn NyashBox, value: Box<dyn NyashBox>) -> Result<(), String>;
}
```

**実装例**:
```rust
impl ArrayService for ArrayBoxAdapter {
    fn len(&self, arr: &dyn NyashBox) -> i64 {
        arr.as_any()
           .downcast_ref::<ArrayBox>()
           .map(|a| a.len() as i64)
           .unwrap_or(0)
    }

    fn get(&self, arr: &dyn NyashBox, index: i64) -> Option<Box<dyn NyashBox>> {
        let arr_box = arr.as_any().downcast_ref::<ArrayBox>()?;
        let index_box = Box::new(IntegerBox::new(index));
        Some(arr_box.get(index_box))
    }

    // set/push も同様の downcast パターン
}
```

### 13.4 MapService API

**Trait 定義**:
```rust
pub trait MapService: Send + Sync {
    /// マップのサイズを取得
    fn size(&self, map: &dyn NyashBox) -> i64;

    /// キーが存在するか確認
    fn has(&self, map: &dyn NyashBox, key: &str) -> bool;

    /// 値を取得
    fn get(&self, map: &dyn NyashBox, key: &str) -> Option<Box<dyn NyashBox>>;

    /// 値を設定
    fn set(&self, map: &dyn NyashBox, key: &str, value: Box<dyn NyashBox>) -> Result<(), String>;
}
```

**実装例**:
```rust
impl MapService for MapBoxAdapter {
    fn size(&self, map: &dyn NyashBox) -> i64 {
        map.as_any()
           .downcast_ref::<MapBox>()
           .map(|m| {
               let size_box = m.size();
               size_box.as_any()
                   .downcast_ref::<IntegerBox>()
                   .map(|i| i.value)
                   .unwrap_or(0)
           })
           .unwrap_or(0)
    }

    fn has(&self, map: &dyn NyashBox, key: &str) -> bool {
        let map_box = match map.as_any().downcast_ref::<MapBox>() {
            Some(m) => m,
            None => return false,
        };
        let key_box = Box::new(StringBox::new(key));
        let result = map_box.has(key_box);
        result.as_any()
              .downcast_ref::<BoolBox>()
              .map(|b| b.value)
              .unwrap_or(false)
    }

    // get/set も同様の downcast パターン
}
```

### 13.5 PluginHost 初期化更新

**Phase 95.5 パターンを踏襲**:
```rust
// ArrayBox (Phase 96: downcast パターン、存在チェックのみ)
if !registry.has_type("ArrayBox") {
    return Err(CoreInitError::MissingService {
        box_id: CoreBoxId::Array,
        message: "ArrayBox not found in registry".to_string(),
    });
}
let array_service = Arc::new(ArrayBoxAdapter::new());

// MapBox (Phase 96: downcast パターン、存在チェックのみ)
if !registry.has_type("MapBox") {
    return Err(CoreInitError::MissingService {
        box_id: CoreBoxId::Map,
        message: "MapBox not found in registry".to_string(),
    });
}
let map_service = Arc::new(MapBoxAdapter::new());
```

### 13.6 テスト追加

**ArrayService テスト**:
```rust
#[test]
fn test_array_service_basic_operations() {
    let arr = ArrayBox::new();
    let adapter = ArrayBoxAdapter::new();

    // push
    let value = Box::new(IntegerBox::new(42));
    adapter.push(&arr, value).unwrap();

    // len
    assert_eq!(adapter.len(&arr), 1);

    // get
    let result = adapter.get(&arr, 0).unwrap();
    let int_box = result.as_any().downcast_ref::<IntegerBox>().unwrap();
    assert_eq!(int_box.value, 42);
}
```

**MapService テスト**:
```rust
#[test]
fn test_map_service_basic_operations() {
    let map = MapBox::new();
    let adapter = MapBoxAdapter::new();

    // set
    let value = Box::new(StringBox::new("Hello"));
    adapter.set(&map, "key1", value).unwrap();

    // has
    assert!(adapter.has(&map, "key1"));
    assert!(!adapter.has(&map, "key2"));

    // get
    let result = adapter.get(&map, "key1").unwrap();
    let str_box = result.as_any().downcast_ref::<StringBox>().unwrap();
    assert_eq!(str_box.value, "Hello");

    // size
    assert_eq!(adapter.size(&map), 1);
}
```

### 13.7 削減統計

| 項目 | Before | After | 削減 |
|------|--------|-------|------|
| #[allow(dead_code)] | 4箇所 | 2箇所 | 2削減 |
| inner フィールド | 4個 | 2個 | 2削減 |
| Box 依存 | 4箇所 | 2箇所 | 2削減 |

**残存 Adapter**:
- IntegerBoxAdapter: Phase 97 で実装予定
- BoolBoxAdapter: Phase 97 で実装予定

### 13.8 技術的成果

**型安全性向上**:
- downcast_ref によるコンパイル時検証
- Option/Result による安全なエラーハンドリング
- Box<dyn NyashBox> ↔ Rust型 の明示的変換

**コード簡略化**:
- unit struct 化により inner フィールド削除
- #[allow(dead_code)] 完全削除（2箇所削減）
- 存在チェックのみのシンプルな初期化

**設計明確化**:
- 3パターンの使い分け確立
- Adapter の責務明確化（複数インスタンス対応）

### 13.9 次のステップ（Phase 97）

- IntegerService/BoolService 実装（純粋関数型で実装）
- 代表パス拡大（5-10箇所で実用化テスト）
- #[allow(dead_code)] 完全撲滅（0箇所達成）

---

## 14. Phase 97: IntegerService/BoolService 実装完了 (2025-12-03)

### 14.1 実装成果

**IntegerService trait 定義**:
```rust
pub trait IntegerService: Send + Sync {
    fn add(&self, a: i64, b: i64) -> i64;
    fn sub(&self, a: i64, b: i64) -> i64;
    fn mul(&self, a: i64, b: i64) -> i64;
    fn div(&self, a: i64, b: i64) -> Option<i64>;
}
```

**BoolService trait 定義**:
```rust
pub trait BoolService: Send + Sync {
    fn not(&self, value: bool) -> bool;
    fn and(&self, a: bool, b: bool) -> bool;
    fn or(&self, a: bool, b: bool) -> bool;
    fn xor(&self, a: bool, b: bool) -> bool;
}
```

### 14.2 Adapter unit struct 化

**IntegerBoxAdapter**:
```rust
// Phase 95.5: #[allow(dead_code)] inner フィールド
pub struct IntegerBoxAdapter {
    #[allow(dead_code)]
    inner: Box<dyn NyashBox>,
}

// Phase 97: unit struct 化（Box状態不要）
pub struct IntegerBoxAdapter;

impl IntegerBoxAdapter {
    pub fn new() -> Self {
        Self
    }
}
```

**BoolBoxAdapter**:
```rust
// Phase 95.5: #[allow(dead_code)] inner フィールド
pub struct BoolBoxAdapter {
    #[allow(dead_code)]
    inner: Box<dyn NyashBox>,
}

// Phase 97: unit struct 化（Box状態不要）
pub struct BoolBoxAdapter;

impl BoolBoxAdapter {
    pub fn new() -> Self {
        Self
    }
}
```

### 14.3 Service 実装

**IntegerService 実装**（純粋関数型）:
```rust
impl IntegerService for IntegerBoxAdapter {
    fn add(&self, a: i64, b: i64) -> i64 {
        a.saturating_add(b)  // オーバーフロー対策
    }

    fn sub(&self, a: i64, b: i64) -> i64 {
        a.saturating_sub(b)  // アンダーフロー対策
    }

    fn mul(&self, a: i64, b: i64) -> i64 {
        a.saturating_mul(b)  // オーバーフロー対策
    }

    fn div(&self, a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            None  // ゼロ除算
        } else {
            Some(a / b)
        }
    }
}
```

**BoolService 実装**（純粋関数型）:
```rust
impl BoolService for BoolBoxAdapter {
    fn not(&self, value: bool) -> bool {
        !value
    }

    fn and(&self, a: bool, b: bool) -> bool {
        a && b
    }

    fn or(&self, a: bool, b: bool) -> bool {
        a || b
    }

    fn xor(&self, a: bool, b: bool) -> bool {
        a ^ b
    }
}
```

### 14.4 PluginHost 初期化更新

**IntegerBox 初期化**（Phase 95.5パターン）:
```rust
// IntegerBox (Phase 97: 純粋関数化、存在チェックのみ)
if !registry.has_type("IntegerBox") {
    return Err(CoreInitError::MissingService {
        box_id: CoreBoxId::Integer,
        message: "IntegerBox not found in registry".to_string(),
    });
}
let integer_service = Arc::new(IntegerBoxAdapter::new());
```

**BoolBox 初期化**（Phase 95.5パターン）:
```rust
// BoolBox (Phase 97: 純粋関数化、存在チェックのみ)
if !registry.has_type("BoolBox") {
    return Err(CoreInitError::MissingService {
        box_id: CoreBoxId::Bool,
        message: "BoolBox not found in registry".to_string(),
    });
}
let bool_service = Arc::new(BoolBoxAdapter::new());
```

### 14.5 テスト追加

**IntegerService テスト**:
```rust
#[test]
fn test_integer_service_operations() {
    let adapter = IntegerBoxAdapter::new();

    // add
    assert_eq!(adapter.add(10, 20), 30);
    assert_eq!(adapter.add(i64::MAX, 1), i64::MAX);  // saturating

    // sub
    assert_eq!(adapter.sub(20, 10), 10);
    assert_eq!(adapter.sub(i64::MIN, 1), i64::MIN);  // saturating

    // mul
    assert_eq!(adapter.mul(5, 6), 30);
    assert_eq!(adapter.mul(i64::MAX, 2), i64::MAX);  // saturating

    // div
    assert_eq!(adapter.div(20, 5), Some(4));
    assert_eq!(adapter.div(10, 3), Some(3));  // 整数除算
    assert_eq!(adapter.div(10, 0), None);     // ゼロ除算
}
```

**BoolService テスト**:
```rust
#[test]
fn test_bool_service_operations() {
    let adapter = BoolBoxAdapter::new();

    // not
    assert_eq!(adapter.not(true), false);
    assert_eq!(adapter.not(false), true);

    // and
    assert_eq!(adapter.and(true, true), true);
    assert_eq!(adapter.and(true, false), false);
    assert_eq!(adapter.and(false, false), false);

    // or
    assert_eq!(adapter.or(true, false), true);
    assert_eq!(adapter.or(false, false), false);

    // xor
    assert_eq!(adapter.xor(true, false), true);
    assert_eq!(adapter.xor(true, true), false);
    assert_eq!(adapter.xor(false, false), false);
}
```

### 14.6 成果統計

**#[allow(dead_code)] 完全削除達成**:
- Phase 95.5: 2箇所（IntegerBoxAdapter, BoolBoxAdapter）
- Phase 97: **0箇所（100%削減達成）**

**Adapter パターン完成（全6個）**:
1. **Ring0直結型** (1個): ConsoleService
2. **純粋関数型** (3個): StringService, IntegerService, BoolService
3. **downcast型** (2個): ArrayService, MapService

**テスト結果**:
```
running 13 tests
test runtime::core_services::tests::test_integer_service_operations ... ok
test runtime::core_services::tests::test_bool_service_operations ... ok
test runtime::plugin_host::tests::test_core_services_all_fields ... ok
test runtime::plugin_host::tests::test_core_services_coverage ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured
```

### 14.7 API設計の特徴

**IntegerService**:
- saturating演算でオーバーフロー対策
- div() は Option<i64> を返してゼロ除算を安全に処理

**BoolService**:
- 標準論理演算（not/and/or/xor）
- 純粋関数として実装（副作用なし）

### 14.8 Phase 97 完全達成

**実装完了項目**:
- ✅ IntegerService trait 定義（add/sub/mul/div）
- ✅ BoolService trait 定義（not/and/or/xor）
- ✅ IntegerBoxAdapter/BoolBoxAdapter unit struct化
- ✅ #[allow(dead_code)] 完全削除（0箇所）
- ✅ PluginHost 初期化更新
- ✅ テスト追加（IntegerService/BoolService）
- ✅ 全テストPASS確認
- ✅ cargo build --release SUCCESS

**コード簡略化**:
- #[allow(dead_code)]: 2箇所 → **0箇所（完全削除）**
- innerフィールド削除: 2個（Integer/Bool）
- 存在チェックのみのシンプルな初期化

**設計確立**:
- 3つのAdapterパターン完成（全6個のService実装完了）
- 純粋関数型の利点: Box状態不要、テスト容易、並列安全

### 14.9 次のステップ（Phase 98-99）

**Phase 98**: 代表パス拡大（5-10箇所）
- VM実行器での CoreServices 使用
- 実用コードパスでの検証
- パフォーマンステスト

**Phase 99**: CoreServices 完全統合
- 全6個のService実装完了確認
- ドキュメント完成
- ベストプラクティス確立

---

## Section 15: Phase 98 代表パス拡張（Console 7箇所）

### 15.1 実装完了内容

**ConsoleService 使用箇所**: 7箇所で println!/eprintln! を ConsoleService 経由に移行完了

1. `src/runner/selfhost.rs`: 1箇所（Phase 95.5 確立済み）
2. `src/runner/modes/common_util/selfhost/child.rs`: 3箇所
   - Line 37: spawn失敗エラー
   - Line 52-56: タイムアウトメッセージ（stdout）
   - Line 59-63: タイムアウトメッセージ（stderr）
3. `src/runner/modes/common_util/core_bridge.rs`: 2箇所
   - Line 23: DUMP書き込みエラー
   - Line 55: DUMP_MUT書き込みエラー
4. `src/runner/modes/vm.rs`: 1箇所
   - Line 590-594: RC（return code）出力
5. `src/runner/modes/common_util/selfhost/json.rs`: 2箇所
   - Line 39: PyVM MIR JSON emit エラー
   - Line 44-48: PyVM 使用ログ（verbose時）

**合計**: 7箇所（selfhost関連: 6箇所、VM実行: 1箇所）

### 15.2 実装パターン

**console_println! マクロ導入**:

```rust
/// Phase 98: Helper macro to print using ConsoleService if available, otherwise eprintln
#[macro_export]
macro_rules! console_println {
    ($($arg:tt)*) => {
        if let Some(host) = $crate::runtime::try_get_core_plugin_host() {
            host.core.console.println(&format!($($arg)*));
        } else {
            eprintln!($($arg)*);
        }
    };
}
```

**try_get_core_plugin_host() 追加**:

```rust
/// Phase 98: Safe accessor that returns None if not initialized
pub fn try_get_core_plugin_host() -> Option<std::sync::Arc<plugin_host::PluginHost>> {
    GLOBAL_CORE_PLUGIN_HOST.get().cloned()
}
```

### 15.3 設計判断

**Graceful Degradation 採用**:
- PluginHost 初期化前: eprintln! を使用（フォールバック）
- PluginHost 初期化後: ConsoleService を使用（Ring0直結）

**理由**:
1. **段階的移行**: 全箇所を一度に変更しない
2. **堅牢性**: 初期化タイミングに依存しない
3. **後方互換性**: 既存の動作を壊さない

**Fail-Fast原則との整合性**:
- エラー処理は変更なし（失敗は即座に返す）
- 出力先の選択のみが動的（エラーの隠蔽ではない）

### 15.4 テスト結果

**ビルド**: ✅ 成功（0 errors, 7 warnings - 既存のもの）

**ユニットテスト**: ✅ 全PASS
```bash
cargo test --lib runtime::core_services --release
# 11 passed; 0 failed
```

**代表ケース**: ✅ 正常動作
```bash
# Plugin Host なし
./target/release/hakorune test.hako
# RC: 42  ← println! フォールバック動作

# Plugin Host あり
NYASH_USE_PLUGIN_HOST=1 ./target/release/hakorune test.hako
# RC: 42  ← ConsoleService 動作
```

### 15.5 残りの println!/eprintln! 箇所

**統計**: 約 366箇所の println!/eprintln! が存在

**Phase 98 成果**: 7箇所（約2%）を ConsoleService に移行完了

**Phase 99 以降**: 残り約 359箇所を段階的に移行予定
- 優先度: selfhost/VM 実行パス → エラー処理 → デバッグ出力

### 15.6 実装ファイル

**修正ファイル**:
1. `src/runtime/mod.rs`:
   - `try_get_core_plugin_host()` 追加
   - `console_println!` マクロ追加
2. `src/runner/modes/common_util/selfhost/child.rs`:
   - 3箇所を `console_println!` に置き換え
3. `src/runner/modes/common_util/core_bridge.rs`:
   - 2箇所を `console_println!` に置き換え
4. `src/runner/modes/vm.rs`:
   - 1箇所を条件分岐（try_get_core_plugin_host）に置き換え
5. `src/runner/modes/common_util/selfhost/json.rs`:
   - 2箇所を `console_println!` に置き換え

### 15.7 Phase 98 完了判定

✅ **5-7箇所の目標達成**: 7箇所で ConsoleService 移行完了

✅ **ビルド成功**: cargo build --release 完全成功

✅ **テスト全PASS**: 既存テストが全て通過

✅ **代表ケース動作確認**: hello_world.hako 正常動作

✅ **ドキュメント更新**: Section 15 追加完了

---

### 15.6-A: ログ/出力の統一設計（Phase 99 ポリシー確定）

**Phase 99 で確定したポリシー**: ログ/出力の3層分離設計

#### 3つのレイヤー

**1. Ring0.log（OS API層）**:
- 用途: ランタイム/OSレイヤーの内部ログ
- 対象: 開発者向け（デバッグ・計測・内部状態追跡）
- API: `ring0.log.debug/info/warn/error(...)`
- 特性: ユーザーに直接見せなくてよいメッセージ

**2. ConsoleService（Box層・ユーザー向け）**:
- 用途: CLI の直接的な出力（ユーザー向けメッセージ）
- 対象: エンドユーザー
- アクセス: `console_println!` マクロ or `host.core.console.println(...)`
- 特性: PluginHost 初期化後に有効、初期化前は eprintln! にフォールバック

**3. 素の println!/eprintln!（制限用途）**:
- 用途: テスト・一時デバッグのみに限定
- 制限: 本番経路・selfhost/hack_check/VM ランナーからは撤退すべき
- テスト: 既存テスト内の println! はそのまま許可（約299箇所）

#### ConsoleService の完全統合状況（Phase 98-99 時点）

**Phase 98 達成**:
- selfhost: ✅ 代表パス完了（7箇所）
- VM runner: ✅ 1箇所（RC出力）

**Phase 100-101 予定**:
- hack_check: 未実装（将来対象）
- 残り user-facing: ~359箇所（段階的に拡張）

#### 完全統合の完了条件

**Console（必須）**:
- ✅ selfhost ランナーのユーザ向け出力が全て console_println! 経由
- ✅ hack_check の結果表示が全て console_println! 経由
- ✅ VM ランナーのメイン出力（RC、エラー等）が全て console_println! 経由

**Ring0.log（段階的）**:
- 既存の debug/info/warn/error API を活用
- 優先順: 内部エラー → VM実行ログ → メモリ・GC情報
- Phase 99 では「どこまで広げるか」の計画のみ記載

**テスト（許容）**:
- テスト内 println! (約299箇所): そのまま許可
- 本番経路からは撤退済み

#### 関連ドキュメント

- [Logging Policy](logging_policy.md) - 役割分担・マクロ方針・テスト方針
- [Ring0 Inventory](ring0-inventory.md) - println! 分類・Ring0.log 計画

---

**Phase 98 実装完了日**: 2025-12-03

**Phase 99 ポリシー確定日**: 2025-12-03

---

## Section 16: Phase 110 - FileHandleBox（ハンドルベース複数回アクセス I/O）

### 概要

FileBox（ワンショット I/O）を補完するハンドルベースのファイル I/O。

- **位置づけ**: core_optional（future で core_required に昇格の可能性）
- **API**: open(path, mode) → read/write → close()
- **プロファイル対応**: Default ✅、NoFs ❌
- **実装**: Ring0FsFileIo を内部で再利用

### 設計原則

**FileBox（Phase 108）との違い**:
- FileBox: 1ショット I/O（read/write を1回ずつ、ファイルを開いて→読む/書く→閉じるを隠す）
- FileHandleBox: 複数回アクセス I/O（open → read/write（複数回可）→ close を明示的に制御）

**Fail-Fast 原則**:
- open() 呼び出し時に既に open 済み → 即座に Err
- close() 後の read/write → 即座に Err
- NoFs profile で open → 即座に Err

**独立インスタンス設計**:
- 各 FileHandleBox インスタンスが独立した Ring0FsFileIo を保持
- 複数の FileHandleBox が同時に異なるファイルを open 可能

### API

```rust
pub struct FileHandleBox {
    base: BoxBase,
    path: String,
    mode: String,
    io: Option<Arc<dyn FileIo>>,
}

impl FileHandleBox {
    pub fn new() -> Self;
    pub fn open(&mut self, path: &str, mode: &str) -> Result<(), String>;
    pub fn read_to_string(&self) -> Result<String, String>;
    pub fn write_all(&self, content: &str) -> Result<(), String>;
    pub fn close(&mut self) -> Result<(), String>;
    pub fn is_open(&self) -> bool;
}
```

### サポートモード

**Phase 110**:
- "r" (read): ファイル読み込み専用
- "w" (write): ファイル上書き書き込み（truncate mode）

**Phase 111 予定**:
- "a" (append): ファイル追記モード

### プロファイル対応

| Profile  | FileHandleBox | 動作 |
|----------|---------------|------|
| Default  | ✅ | Ring0FsFileIo を使用（完全なファイル I/O） |
| NoFs     | ❌ | open() が Err を返す（"File I/O disabled in no-fs profile"） |
| TestMock (TBD) | ✅ mock | テスト用ダミー |
| Sandbox (TBD)  | ✅ dir限定 | サンドボックス内のみアクセス可能 |

### 実装詳細

**ファイル**: `src/boxes/file/handle_box.rs`

**主要機能**:
1. new() - ハンドル作成（ファイルは未open）
2. open(path, mode) - ファイルを開く
   - 二重 open チェック（Fail-Fast）
   - mode 検証（"r" or "w"）
   - NoFs profile チェック
   - Ring0FsFileIo インスタンス作成
3. read_to_string() - ファイル内容を読み込み
4. write_all(content) - ファイルに書き込み
   - write mode チェック
5. close() - ファイルを閉じる
   - FileIo インスタンスを drop
6. is_open() - ファイルが open されているかチェック

### テスト

**実装済みテスト** (7個):
1. test_filehandlebox_basic_write_read - 基本的な書き込み・読み込み
2. test_filehandlebox_double_open_error - 二重 open エラー
3. test_filehandlebox_closed_access_error - close 後アクセスエラー
4. test_filehandlebox_write_wrong_mode - read mode で write エラー
5. test_filehandlebox_multiple_writes - 複数回書き込み
6. test_filehandlebox_unsupported_mode - 未サポートモードエラー
7. test_filehandlebox_independent_instances - 独立インスタンス動作確認

**テスト結果**: ✅ 7/7 PASS

### Phase 111: append mode + metadata（完了 ✅）

**実装内容**:
- **append mode**: open(path, "a") で末尾に追記可能に
- **metadata API**: size / exists / is_file / is_dir を内部 Rust API として実装
- **FsApi.append_all()**: write_all() と対称的に追加

**テスト追加** (4個):
1. test_filehandlebox_append_mode - append 動作確認
2. test_filehandlebox_metadata_size - サイズ取得
3. test_filehandlebox_metadata_is_file - ファイル型判定
4. test_filehandlebox_write_readonly_error - read-only 保護

**テスト結果**: ✅ 4/4 新テスト PASS（既存 11/11 も全 PASS）

**実装詳細**: [Phase 111 設計書](phase111_filehandlebox_append_metadata.md) 参照

### 将来の拡張ポイント

- **Phase 112**: Ring0 Service Registry 統一化（metadata に modified フィールド追加）
- **Phase 113**: FileHandleBox NyashBox 公開 API（.hako から metadata 呼び出し可能に）
- **Phase 114**: FileIo 機能拡張（exists/stat/canonicalize を trait に追加）
- **Phase 115**: 並行アクセス安全性（Arc<Mutex<...>>）
- **Phase 116**: file encoding explicit 指定（UTF-8 以外）

### 関連ドキュメント

- [Phase 110 設計書](phase110_filehandlebox_design.md) - 完全仕様
- [Phase 111 設計書](phase111_filehandlebox_append_metadata.md) - append + metadata 実装
- [Phase 113 設計書](phase113_filehandlebox_public_api.md) - Nyash 公開 API 実装
- [Ring0 Inventory](ring0-inventory.md) - FileIo/FsApi レイヤー設計

---

**Phase 110 実装完了日**: 2025-12-03
**Phase 111 実装完了日**: 2025-12-03（Commit fce7555e）
**Phase 113 実装完了日**: 2025-12-04

### Section 16.1: Phase 113 - FileHandleBox Nyash 公開 API

#### 概要

FileHandleBox の内部メソッド（open/read/write/close/exists/size など）を
NyashBox trait の標準パターンで Nyash (.hako) 側に公開。

#### 公開メソッド

**I/O メソッド**:
- `open(path: String, mode: "r"|"w"|"a")` -> Void (panic on error)
- `read()` -> String
- `write(text: String)` -> Void
- `close()` -> Void

**メタデータ メソッド**:
- `exists()` -> Bool
- `size()` -> Integer
- `isFile()` -> Bool
- `isDir()` -> Bool

#### 実装パターン

Rust 内部メソッドとNyash可視メソッドを分離:
- Rust internal: `open()`, `read_to_string()`, `write_all()`, `close()`, etc.
- Nyash-visible: `ny_open()`, `ny_read()`, `ny_write()`, `ny_close()`, etc.

エラーハンドリング:
- Phase 113: panic ベース（unwrap_or_else）
- Phase 114+: Result<T, E> 型への移行を検討

#### Profile 別動作

| Profile | open | read/write | exists/size |
|---------|------|-----------|------------|
| Default | ✅ OK | ✅ OK | ✅ OK |
| NoFs | ❌ panic | - | ❌ panic |

#### Nyash コード例

```nyash
local h = new FileHandleBox()

// ファイル追記
h.open("/tmp/log.txt", "a")
h.write("hello\n")
h.close()

// ファイル読み込み
h.open("/tmp/log.txt", "r")
local content = h.read()
print(content)

// メタデータ確認
if h.exists() {
    local size = h.size()
    print("Size: " + size)
}
h.close()
```

#### テスト

Rust ユニットテスト:
- ✅ `test_phase113_ny_open_read_write_close`
- ✅ `test_phase113_ny_append_mode`
- ✅ `test_phase113_ny_metadata_methods`
- ✅ `test_phase113_ny_open_panic_on_error`
- ✅ `test_phase113_ny_read_panic_when_not_open`
- ✅ `test_phase113_ny_write_panic_in_read_mode`

.hako サンプル:
- ✅ `apps/examples/file_handle/append_and_stat.hako`

---

## Section 17: Phase 112 - Ring0 Service Registry 統一化

### 概要

Ring0Context の初期化を「Ring0Registry::build(profile)」に集約。
profile ごとに実装を切り替える factory パターンで、
プロファイル対応と将来の拡張を簡素化。

### 設計

- **default_ring0()**: Ring0Registry::build(RuntimeProfile::Default) に統一
- **NoFsApi**: NoFs profile で FsApi 無効化（Ring0 レベル）
- **initialize_runtime()**: env 読み込み → Ring0Registry.build() → init_global_ring0()

### プロファイル別の Ring0Context

| Profile | mem | io | time | log | fs | thread |
|---------|-----|----|----|-----|----|----|
| Default | ✅ StdMem | ✅ StdIo | ✅ StdTime | ✅ StdLog | ✅ StdFs | ✅ StdThread |
| NoFs | ✅ StdMem | ✅ StdIo | ✅ StdTime | ✅ StdLog | ❌ NoFsApi | ✅ StdThread |

### 責務分離 (Phase 112)

```
【Layer】              【責務】                    【実装】
─────────────────────────────────────────────────────
env                  User configuration         NYASH_RUNTIME_PROFILE
initialize_runtime  env 読み込み + Ring0 初期化   src/runtime/mod.rs
Ring0Registry       Profile 応じた実装選択      src/runtime/ring0/mod.rs
Std* / NoFsApi      具体実装（std::fs など）   src/runtime/ring0/std_impls.rs
Ring0Context        API 統合                    Ring0
PluginHost/FileBox  Ring0 の利用者             runtime/boxes
```

### Ring0.fs が NoFsApi の場合の連鎖効果

**設計**: Ring0 レベルで NoFsApi を使うと、すべての上位層が自動的に disabled

```
Ring0Registry::build(RuntimeProfile::NoFs)
    ↓
Ring0Context { fs: Arc::new(NoFsApi), ... }
    ↓
Ring0FsFileIo が内部で ring0.fs.read/write/append を呼ぶ
    ↓
→ すべて IoError で失敗する（自動的に disabled）
    ↓
FileBox.read() / FileHandleBox.open() も失敗
    ↓
→ ユーザー側は「FileBox/FileHandleBox が使えない」と認識
```

**つまり**: Ring0.fs が NoFsApi なら、PluginHost/FileBox/FileHandleBox は何もしなくても自動的に disabled になる！

### 実装ファイル

- `src/runtime/ring0/mod.rs`: Ring0Registry struct + build() メソッド
- `src/runtime/ring0/std_impls.rs`: NoFsApi struct（FsApi trait 実装）
- `src/runtime/mod.rs`: initialize_runtime() のドキュメント更新
- `src/runner/mod.rs`: NyashRunner::new() で Ring0Registry 使用

### 将来の拡張例

```rust
// Phase 113+ で以下のように拡張可能
impl Ring0Registry {
    pub fn build(profile: RuntimeProfile) -> Ring0Context {
        match profile {
            RuntimeProfile::Default => Self::build_default(),
            RuntimeProfile::NoFs => Self::build_no_fs(),
            RuntimeProfile::TestMock => Self::build_test_mock(),    // ← 追加
            RuntimeProfile::Sandbox => Self::build_sandbox(),        // ← 追加
            RuntimeProfile::ReadOnly => Self::build_readonly(),      // ← 追加
            RuntimeProfile::Embedded => Self::build_embedded(),      // ← 追加
        }
    }
}
```

### 関連ドキュメント

- [Phase 112 設計書](phase112_ring0_registry_design.md) - 完全仕様
- [Ring0 Inventory](ring0-inventory.md) - Ring0 レイヤー全体設計

---

**Phase 112 実装完了日**: 2025-12-03


### Phase 114: FileIo trait 拡張（exists/stat/canonicalize）

FileIo trait に exists/stat/canonicalize を正式追加。
FileHandleBox の内部メソッド（is_file/is_dir/size）を stat() に統一。
Ring0FsFileIo が path を管理し、stat() で正の情報を返す設計。
NoFsFileIo は exists() → false, stat() → Unsupported エラー。

**設計原則**:
- **FsApi = Stateless**: パスを毎回引数で受け取る（Ring0 レイヤー）
- **FileIo = Stateful**: open() で path を保持（Ring1 レイヤー）
- **FileHandleBox**: metadata_internal() で FileIo::stat() を経由

**実装成果**:
- FileStat 構造体追加（is_file/is_dir/size）
- Ring0FsFileIo: ring0.fs.metadata() を stat() で wrap
- NoFsFileIo: exists=false, stat=Unsupported
- FileHandleBox: downcast 不要、trait 経由で統一

**詳細**: [phase114_fileio_trait_extension.md](./phase114_fileio_trait_extension.md)

---

## Section 18: Phase 122 - ConsoleBox.println / log 統一

### 概要

ConsoleBox の `println` メソッドを `log` のエイリアスとして VM レベルで正規化。
すべての経路（JSON v0 / selfhost / 通常VM）で一貫性を保つ。

### 設計

- **言語レベル**: `println(message)` は `log(message)` の完全なエイリアス
- **VM レベル**: `println` は slot 400（`log` と同じ）に正規化
- **正規化ポイント**: `src/runtime/type_registry.rs` の `CONSOLE_METHODS`

### 実装詳細

**TypeRegistry alias**:
```rust
const CONSOLE_METHODS: &[MethodEntry] = &[
    MethodEntry { name: "log",     arity: 1, slot: 400 },
    MethodEntry { name: "println", arity: 1, slot: 400 },  // alias
    // ...
];
```

**Rust 実装**:
```rust
impl ConsoleBox {
    pub fn println(&self, message: &str) {
        self.log(message);
    }
}
```

### 実装完了日

**Phase 122 実装完了日**: 2025-12-04

---

## Section 19: Phase 125 - ConsoleBox Migration to Plugin

### 背景

Phase 122-125 で ConsoleBox は完全にプラグインベースに移行しました。

**進化の流れ**:
- **Phase 122**: println/log エイリアス統一
- **Phase 122.5**: nyash.toml method_id 修正
- **Phase 123**: WASM/非WASM コード統一（67行削減）
- **Phase 124**: TypeRegistry ベースの統一ディスパッチ（100行削減）
- **Phase 125**: ビルトイン ConsoleBox 削除（52行削減）

### 実装内容

**削除対象**:
- ビルトイン ConsoleBox（`src/box_factory/builtin_impls/console_box.rs`）削除
- `src/box_factory/builtin.rs` の ConsoleBox case 削除
- `src/box_factory/builtin_impls/mod.rs` の mod 宣言削除
- テストコード削除

**保持対象**:
- Rust 実装（`src/boxes/console_box.rs`）は内部用として保持
- プラグイン（`libnyash_console_plugin.so`）のみが対外インターフェース

**現在の実装構造**:
```
src/boxes/console_box.rs        ← Rust 実装（VM が内部的に使用）
   ↓
libnyash_console_plugin.so      ← プラグイン（ユーザー向けインターフェース）
   ↓
src/box_factory/builtin.rs      ← ConsoleBox case は削除済み
```

### 利点

1. **"Everything is Plugin" 原則の完全実装**: ConsoleBox = プラグインのみ
2. **ビルトイン Factory の簡略化**: 1つの Box 削除
3. **プラグイン拡張性の向上**: 単一のプラグイン実装で統一
4. **保守性向上**: 二重実装の排除

### 統合ドキュメント

**詳細な実装背景**: [Phase 125 詳細](phase125_delete_deprecated_console_box.md)

**統合的なガイド**: [ConsoleBox 完全ガイド](consolebox_complete_guide.md)
- ユーザーガイド
- アーキテクチャ設計
- 実装者向けガイド
- FAQ・トラブルシューティング

### 実装完了日

**Phase 125 実装完了日**: 2025-12-04

---

## 📚 Related Documents

### ConsoleBox について知りたい場合
- [ConsoleBox 完全ガイド](consolebox_complete_guide.md) - 統合的なリファレンス
- [Phase 122-125 実装記録](phase122_consolebox_println_unification.md) - 詳細な実装背景

### ログ出力について知りたい場合
- [ログポリシー](logging_policy.md) - Nyash のログ出力全体のポリシー
- [Hako ログ設計](hako_logging_design.md) - Hako コンパイラ側のログ設計

### Core Boxes 設計について知りたい場合
- このドキュメント - Core Box の全体設計
- [TypeRegistry 設計](../architecture/type-registry-design.md) - TypeRegistry の詳細設計
