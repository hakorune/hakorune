# Phase 112: Ring0 Service Registry 統一化

## 0. ゴール

- 現状バラバラに見える Ring0 サービス（MemApi/IoApi/TimeApi/LogApi/FsApi/ThreadApi）を、
  **1つの service registry** として扱える構造にまとめる。
- 目的は：
  - 初期化コードの散乱を止める（StdMem/StdFs/StdLog…の生成場所を1カ所に集約）
  - プロファイル（Default/NoFs/TestMock…）ごとに Ring0 を差し替えやすくする
  - 将来の mock / embedded / sandbox プロファイルの足場を先に用意しておく。

---

## 1. スコープと非スコープ

### スコープ（今回やること）

1. **設計ドキュメント**: 「Ring0 Service Registry」の形を定義。
2. **Ring0Registry struct**: `build(profile) -> Ring0Context` メソッド実装。
3. **NoFsApi struct**: NoFs profile 用の FsApi stub 実装。
4. **初期化パスの統一**:
   - RuntimeProfile::from_env() で profile 読み込み（env 読み込み唯一の場所）
   - Ring0Registry::build(profile) で Ring0Context 構築
   - init_global_ring0() で GLOBAL_RING0 登録
5. **責務分離**:
   - env 読み込み → initialize_runtime() のみ
   - Profile に応じた実装選択 → Ring0Registry::build()
   - 各 Std* の具体実装 → std_impls.rs に閉じ込める
6. **ドキュメント**: Phase 112 設計書 + core_boxes_design.md / ring0-inventory.md / CURRENT_TASK.md 更新。

### 非スコープ（今回はやらない）

- Nyash 側（.hako）から Ring0 を差し替える仕組み（Phase 113+）。
- FileHandleBox の NyashBox 公開 API（metadata を .hako から触るライン）。
- 並行アクセス・エンコーディング・ACL などの高機能。

---

## 2. Task 1: 設計ドキュメント作成（Ring0 Registry の全体像）

### 2.1 実装内容

**ファイル**（新規）:
- `docs/development/current/main/phase112_ring0_registry_design.md`（本ドキュメント）

### 2.2 現状の構造

**Ring0Context の構成**:
```rust
pub struct Ring0Context {
    pub mem: Arc<dyn MemApi>,
    pub io: Arc<dyn IoApi>,
    pub time: Arc<dyn TimeApi>,
    pub log: Arc<dyn LogApi>,
    pub fs: Arc<dyn FsApi>,       // Phase 90-A / Phase 107-111
    pub thread: Arc<dyn ThreadApi>, // Phase 90-D
}
```

**現在の初期化パス**:
- `default_ring0()`: StdMem/StdIo/StdTime/StdLog/StdFs/StdThread で構築
- `init_global_ring0()`: GLOBAL_RING0 OnceLock に登録
- `get_global_ring0()`: GLOBAL_RING0 から取得

**問題点**:
- profile に応じた切り替え（NoFs など）が default_ring0() では硬い
- initialize_runtime() の中で Ring0 初期化を完全に管理していない

### 2.3 目指す構造

```
【initialize_runtime()】
   ↓ (profile/env 読み込み)
【Ring0Registry::build(profile)】
   ↓ (profile に応じた実装選択)
【Ring0Context { mem, io, time, log, fs, thread }】
   ↓ (init_global_ring0())
【GLOBAL_RING0】
   ↓
【PluginHost / FileBox / FileHandleBox / Logger / Runner …】
```

### 2.4 設計原則（Phase 112 で確立）

| 層 | 責務 | 例 |
|-----|------|-----|
| env | User configuration | NYASH_RUNTIME_PROFILE=no-fs |
| initialize_runtime() | env 読み込み + Ring0 初期化 | profile = RuntimeProfile::from_env() |
| Ring0Registry | 実装選択と構築 | build(profile) → Default か NoFs か |
| Std*/NoFs* | 具体実装 | StdFs / NoFsApi |
| Ring0Context | 統合されたコンテキスト | 全 API を一括提供 |

---

## 3. Task 2: Ring0Registry インターフェース定義

### 3.1 実装内容

**ファイル**:
- `src/runtime/ring0/mod.rs`（修正）
- `src/runtime/runtime_profile.rs`（既存、確認のみ）

### 3.2 Ring0Registry 構造体の追加

```rust
// src/runtime/ring0/mod.rs

use crate::runtime::runtime_profile::RuntimeProfile;

/// Phase 112: Ring0 service registry
///
/// profile ごとに適切な FsApi 実装（等）を選択して Ring0Context を構築する factory。
pub struct Ring0Registry;

impl Ring0Registry {
    /// Ring0Context を profile に応じて構築
    pub fn build(profile: RuntimeProfile) -> Ring0Context {
        match profile {
            RuntimeProfile::Default => Self::build_default(),
            RuntimeProfile::NoFs => Self::build_no_fs(),
        }
    }

    fn build_default() -> Ring0Context {
        Ring0Context {
            mem: Arc::new(StdMem::new()),
            io: Arc::new(StdIo),
            time: Arc::new(StdTime),
            log: Arc::new(StdLog),
            fs: Arc::new(StdFs),
            thread: Arc::new(StdThread),
        }
    }

    fn build_no_fs() -> Ring0Context {
        Ring0Context {
            mem: Arc::new(StdMem::new()),
            io: Arc::new(StdIo),
            time: Arc::new(StdTime),
            log: Arc::new(StdLog),
            fs: Arc::new(NoFsApi),     // Phase 112: NoFs profile では FsApi を disabled に
            thread: Arc::new(StdThread),
        }
    }
}
```

### 3.3 default_ring0() の位置付け

既存の `default_ring0()` は、**互換性のため** 以下のように修正：

```rust
/// Phase 88: デフォルト Ring0Context を作成
///
/// Phase 112 以降は、initialize_runtime() を通じて
/// Ring0Registry::build(profile) 経由で初期化されることが推奨。
///
/// この関数は直接呼び出しに対する互換性レイヤーとして保持。
pub fn default_ring0() -> Ring0Context {
    Ring0Registry::build(RuntimeProfile::Default)
}
```

**重要**: 今後は initialize_runtime() → Ring0Registry::build() の流れが SSOT。

---

## 4. Task 3: NoFsApi 実装（Phase 109/111 の FsApi stub）

### 4.1 実装内容

**ファイル**:
- `src/runtime/ring0/std_impls.rs`（NoFsApi 追加）
- `src/runtime/ring0/mod.rs`（re-export 追加）

### 4.2 NoFsApi 構造体

```rust
// src/runtime/ring0/std_impls.rs

/// Phase 112: No-FS profile 用 FsApi stub
///
/// FileSystem 操作がすべて「無効」として機能する。
/// Phase 109 の NoFsFileIo（FileIo trait）と異なり、
/// Ring0 レベルの FsApi trait を実装する。
pub struct NoFsApi;

impl FsApi for NoFsApi {
    fn read_to_string(&self, _path: &Path) -> Result<String, IoError> {
        Err(IoError::Io(
            "FileSystem operations disabled in no-fs profile".to_string()
        ))
    }

    fn read(&self, _path: &Path) -> Result<Vec<u8>, IoError> {
        Err(IoError::Io(
            "FileSystem operations disabled in no-fs profile".to_string()
        ))
    }

    fn write_all(&self, _path: &Path, _data: &[u8]) -> Result<(), IoError> {
        Err(IoError::Io(
            "FileSystem operations disabled in no-fs profile".to_string()
        ))
    }

    fn append_all(&self, _path: &Path, _data: &[u8]) -> Result<(), IoError> {
        Err(IoError::Io(
            "FileSystem operations disabled in no-fs profile".to_string()
        ))
    }

    fn exists(&self, _path: &Path) -> bool {
        false
    }

    fn metadata(&self, _path: &Path) -> Result<FsMetadata, IoError> {
        Err(IoError::Io(
            "FileSystem operations disabled in no-fs profile".to_string()
        ))
    }

    fn canonicalize(&self, _path: &Path) -> Result<PathBuf, IoError> {
        Err(IoError::Io(
            "FileSystem operations disabled in no-fs profile".to_string()
        ))
    }
}
```

### 4.3 re-export

```rust
// src/runtime/ring0/mod.rs

pub use std_impls::{NoopMem, StdFs, StdIo, StdLog, StdMem, StdThread, StdTime, NoFsApi};
```

---

## 5. Task 4: initialize_runtime() と Global Ring0 の統一

### 5.1 実装内容

**ファイル**:
- `src/runner/initialize_runtime.rs`（既存、確認・修正）
- `src/runtime/ring0/mod.rs`（确认のみ）

### 5.2 initialize_runtime() の責務

```rust
// src/runner/initialize_runtime.rs（または src/runtime/mod.rs）

use crate::runtime::runtime_profile::RuntimeProfile;
use crate::runtime::ring0::{Ring0Context, Ring0Registry, init_global_ring0, get_global_ring0};

/// Phase 112: Runtime 初期化（唯一の env 読み込み / Ring0 初期化ポイント）
///
/// **責務**:
/// 1. RuntimeProfile を env から読む（env 読み込みはここだけ）
/// 2. Ring0Registry::build(profile) で Ring0Context を構築
/// 3. init_global_ring0() で GLOBAL_RING0 に登録
pub fn initialize_runtime() -> Arc<Ring0Context> {
    // 1. Profile を env から読む（唯一の場所）
    let profile = RuntimeProfile::from_env();

    // 2. Ring0Context を構築（profile に応じた実装選択）
    let ctx = Ring0Registry::build(profile);

    // 3. GLOBAL_RING0 に登録
    init_global_ring0(ctx);

    // 4. グローバル Ring0Context を取得して返す
    get_global_ring0()
}
```

### 5.3 設計原則

**重要な約束**:
- **env 直読み禁止**: すべての環境変数読み込みは initialize_runtime() を通す
- **Ring0Context 入口の一本化**: default_ring0() ではなく initialize_runtime() を呼ぶ
- **Profile-aware**: Ring0Registry::build(profile) が、runtime 全体のふるまいを決定する

---

## 6. Task 5: Provider / PluginHost 側から見た Ring0 の統一

### 6.1 実装内容

**ファイル**:
- `src/runtime/plugin_host.rs`（確認、修正不要の可能性）
- `src/runtime/provider_lock.rs`（確認）
- ドキュメント更新（phase110/111）

### 6.2 Ring0.fs が NoFsApi の場合の動作（新規の一貫性 ✅）

**Phase 112 で確立される新しい約束**:

```
【設計】Ring0 レベルで NoFsApi を使うと、すべての上位層が自動的に disabled

Flow:
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

### 6.3 PluginHost との関係

**既存の logic（Phase 109/111）はそのまま**:
- PluginHost.with_core_from_registry(ring0, registry, profile)
  - FileBox provider 存在確認は profile に応じて切り替え（既実装）
- FileHandleBox.open()
  - NoFs profile では provider_lock から provider が無いので、即座に「disabled」エラー
- Ring0FsFileIo
  - 内部で ring0.fs を呼び出す構造はそのまま。NoFsApi なら自動的に disabled

**結論**: Phase 112 で Ring0Registry を導入しても、既存のロジックは変わらない！

---

## 7. Task 6: ドキュメント更新

### 7.1 実装内容

**ファイル**:
- `phase112_ring0_registry_design.md`（本ドキュメント）
- `core_boxes_design.md`（Ring0 セクション追加）
- `ring0-inventory.md`（Phase 112 エントリ追加）
- `CURRENT_TASK.md`（Phase 112 反映）

### 7.2 core_boxes_design.md への追記

Ring0 セクションに以下を追加：

```markdown
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

### 実装完了日

**Phase 112 実装完了日**: 2025-12-03（予定）
```

---

## 8. 完成チェックリスト（Phase 112）

- [ ] Ring0Registry struct と build(profile) メソッド実装
- [ ] NoFsApi struct 実装（FsApi trait）
- [ ] default_ring0() を Ring0Registry::build(RuntimeProfile::Default) に統一
- [ ] initialize_runtime() が env 読み込み → Ring0Registry → init_global_ring0() の流れ
- [ ] PluginHost / FileBox / FileHandleBox からの Ring0 アクセス経路が変わらない（互換性維持）
- [ ] NoFsApi による自動的な disabled 動作確認（integration test）
- [ ] phase112_ring0_registry_design.md / core_boxes_design.md / CURRENT_TASK.md 更新済み
- [ ] ビルド・ファイル I/O 関連テスト全 PASS

---

## 9. 設計原則（Phase 112 で確立）

### 責務分離

```
【Layer】              【責務】                    【誰が実装】
─────────────────────────────────────────────────────
env                  User configuration         User
initialize_runtime  env 読み込み + Ring0 初期化   runner
Ring0Registry       Profile 応じた実装選択      ring0
Std* / NoFsApi      具体実装（std::fs など）   ring0/std_impls
Ring0Context        API 統合                    Ring0
PluginHost/FileBox  Ring0 の利用者             runtime/boxes
```

### Profile 拡張の足場

**将来の追加が簡単**:

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

---

**Phase 112 指示書完成日**: 2025-12-03（修正案統合版）
Status: Historical
