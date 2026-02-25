# Phase 131-2: ConsoleBox 問題根治調査レポート

## 🎯 調査目的

実アプリの E2E で最初に詰まる「Box 登録/認識」問題を特定し、VM/LLVM 共通の土台を確立する。

## 📊 調査結果サマリ

### 問題の本質

**ConsoleBox は既に両 backend で完全実装済み** - 問題は **登録メカニズムの分岐** にある：

1. **VM backend**: Box 解決が「UnifiedBoxRegistry（入口） + BoxFactoryRegistry（plugin provider mapping） + VM fast path」の合成に見え、全体像が追いにくい
2. **LLVM backend**: 別の経路（TypeRegistry/FFI）を使用（Phase 133 の事例あり）
3. **両者の規約不一致**: “どれをSSOTとして読むべきか” が docs/コードで分散している

### 重要な発見

#### ✅ LLVM 側は統合事例がある（Phase 133 / archive）

LLVM backend の ConsoleBox 問題は **Phase 133 で完全解決済み**:
- ConsoleLlvmBridge 箱化モジュール実装済み
- TypeRegistry との ABI 完全一致
- 7/7 テスト全て PASS

**結論**: LLVM 側は “参考モデル” として参照し、当面の焦点は VM 側の経路可視化と SSOT 明文化。

#### ⚠️ VM backend の Box 解決が「2層 + 特例」に見える（問題の根源）

VM backend では “入口” と “plugin provider mapping” が分かれており、加えて入口に特例がある：

```
1. BoxFactoryRegistry (src/runtime/box_registry.rs)
   - Plugin-First アーキテクチャ
   - プラグイン設定で上書き可能
   - グローバルレジストリ

2. UnifiedBoxRegistry + global accessor
   - global accessor: `src/runtime/unified_registry.rs`
   - registry 本体: `src/box_factory/mod.rs`
   - handle_new_box()（VM NewBox）が使用

3. VM fast path（特例）
   - `NYASH_VM_FAST=1` のとき `StringBox` をレジストリ経由せず生成（bench/profile-only）
```

## 🔍 詳細分析

### 1. VM Backend の Box 解決フロー

#### NewBox 命令処理（src/backend/mir_interpreter/handlers/boxes.rs）

```rust
pub(super) fn handle_new_box(
    &mut self,
    dst: ValueId,
    box_type: &str,
    args: &[ValueId],
) -> Result<(), VMError> {
    // ① Provider Lock guard（既定は挙動不変）
    provider_lock::guard_before_new_box(box_type)?;

    // ② Fast path (StringBox のみ、NYASH_VM_FAST=1 時)
    if box_type == "StringBox" { /* ... */ }

    // ③ 統一レジストリ経由で生成 ← ここが主経路
    let reg = unified_registry::get_global_unified_registry();
    let created = reg.lock().unwrap().create_box(box_type, &converted)?;

    // ④ 生成結果を VMValue に変換して格納
    self.regs.insert(dst, VMValue::from_nyash_box(created));

    Ok(())
}
```

**問題点**:
- `unified_registry` は `src/runtime/unified_registry.rs` にあり、`UnifiedBoxRegistry` は `src/box_factory/mod.rs`
- `UnifiedBoxRegistry` は `PluginBoxFactory`（`src/box_factory/plugin.rs`）を通じて `BoxFactoryRegistry` を参照するため、間接的に両者が接続されている
- Provider Lock の役割が曖昧（「既定は挙動不変」）

#### BoxFactoryRegistry の設計（src/runtime/box_registry.rs）

```rust
pub struct BoxFactoryRegistry {
    providers: RwLock<HashMap<String, BoxProvider>>,
}

pub enum BoxProvider {
    Builtin(BoxConstructor),  // 互換用（テスト専用）
    Plugin(String),           // プラグイン実装
}
```

**特徴**:
- Plugin-First 設計（プラグインがビルトインを上書き可能）
- `apply_plugin_config()` で nyash.toml から動的登録
- `create_plugin_box()` → PluginHost 経由で実際の生成

**疑問**:
- このレジストリは `PluginBoxFactory` の provider mapping として使用される（VM NewBox → UnifiedBoxRegistry → PluginBoxFactory → BoxFactoryRegistry）
- provider の populate は `src/runtime/plugin_loader_unified.rs` が行う（`apply_plugin_config`）

#### Builtin Fallback（src/box_factory/builtin_impls/console_box.rs）

```rust
/// Create builtin ConsoleBox instance
///
/// Primary: nyash-console-plugin
/// Fallback: This builtin implementation (selfhost support)
pub fn create(_args: &[Box<dyn NyashBox>]) -> Result<Box<dyn NyashBox>, RuntimeError> {
    // Phase 151: Quiet fallback (no deprecation warning)
    Ok(Box::new(ConsoleBox::new()))
}
```

**用途**:
- セルフホスト Stage-3 パイプライン用
- プラグイン初期化失敗時のバックアップ
- Phase 151 で追加（Phase 150 のエラー対処）

### 2. LLVM Backend の Box 解決フロー（Phase 133 完了済み）

#### ConsoleLlvmBridge（src/llvm_py/console_bridge.py）

```python
CONSOLE_METHODS = {
    "log": "nyash.console.log",
    "println": "nyash.console.log",  # Phase 122: log のエイリアス
    "warn": "nyash.console.warn",
    "error": "nyash.console.error",
    "clear": "nyash.console.clear",
}

def emit_console_call(builder, module, method_name, args, ...):
    """ConsoleBox method call to LLVM IR"""
    if method_name not in CONSOLE_METHODS:
        return False

    runtime_fn_name = CONSOLE_METHODS[method_name]
    # LLVM IR generation...
    return True
```

**特徴**:
- Phase 133 で箱化モジュール化済み
- TypeRegistry の slot 400-403 と完全一致
- Phase 122 の println/log エイリアス統一を継承

#### TypeRegistry との連携（src/runtime/type_registry.rs）

```rust
// ConsoleBox 用 slot 定義
const CONSOLE_METHODS: &[MethodEntry] = &[
    MethodEntry { name: "log", arity: 1, slot: 400 },
    MethodEntry { name: "println", arity: 1, slot: 400 },  // エイリアス
    MethodEntry { name: "warn", arity: 1, slot: 401 },
    MethodEntry { name: "error", arity: 1, slot: 402 },
    MethodEntry { name: "clear", arity: 0, slot: 403 },
];
```

**SSOT 化の成果**:
- ✅ メソッド名 → slot 番号のマッピングが一元管理
- ✅ VM/LLVM 両方で共通の型情報を参照
- ✅ Phase 122 のエイリアス統一が完全適用

### 3. Plugin System との連携

#### TypeBox v2 FFI（plugins/nyash-console-plugin/src/lib.rs）

```rust
#[no_mangle]
pub static nyash_typebox_ConsoleBox: NyashTypeBoxFfi = NyashTypeBoxFfi {
    abi_tag: 0x54594258, // 'TYBX'
    version: 1,
    name: b"ConsoleBox\0".as_ptr() as *const c_char,
    resolve: Some(console_resolve),
    invoke_id: Some(console_invoke_id),
    capabilities: 0,
};
```

**特徴**:
- TypeBox v2 FFI を実装
- `resolve()` でメソッド名 → ID 変換
- `invoke_id()` で実際のメソッド呼び出し

**問題点**:
- VM backend の `unified_registry` がこの FFI を使っているか不明
- プラグインローダーとの接続点が見えない

### 4. CoreBoxId システム（Phase 87）

#### CoreBoxId 定義（src/runtime/core_box_ids.rs）

```rust
pub enum CoreBoxId {
    String, Integer, Bool, Array, Map, Console,  // core_required
    Float, Null, File, Path, Regex, Math, Time, Json, Toml,  // core_optional
    Function, Result, Method, Missing,  // 特殊型
}

impl CoreBoxId {
    pub fn is_core_required(&self) -> bool {
        matches!(self, String | Integer | Bool | Array | Map | Console | File)
    }

    pub fn is_required_in(&self, profile: &RuntimeProfile) -> bool {
        match profile {
            RuntimeProfile::Default => self.is_core_required(),
            RuntimeProfile::NoFs => self.is_core_required() && *self != Self::File,
        }
    }
}
```

**設計思想**:
- Core Box を型安全な enum で管理
- Phase 85 の core_required/core_optional 分類を実装
- Runtime profile による動的要件変更

**問題点**:
- この enum が **Box 登録の強制力** を持っているか不明
- `is_core_required()` の結果が実際の登録処理に反映されているか要確認

### 5. CoreServices（Phase 91）

#### CoreServices 定義（src/runtime/core_services.rs）

```rust
pub trait ConsoleService: Send + Sync {
    fn println(&self, msg: &str);
    fn print(&self, msg: &str);
}

pub struct CoreServices {
    pub string: Option<Arc<dyn StringService>>,
    pub integer: Option<Arc<dyn IntegerService>>,
    pub bool: Option<Arc<dyn BoolService>>,
    pub array: Option<Arc<dyn ArrayService>>,
    pub map: Option<Arc<dyn MapService>>,
    pub console: Option<Arc<dyn ConsoleService>>,
}
```

**特徴**:
- Ring1-Core の Service trait 群
- Phase 87 CoreBoxId の core_required をカバー
- Optional 化により Graceful Degradation 対応

**疑問**:
- この Service trait は **誰が実装しているのか？**
- VM の Box 呼び出しがこの trait を経由するのか？

## 🚨 発見した問題点

### 1. **SSOT の欠如（最重要）**

Box 登録規約が複数箇所に分散：

```
┌─ BoxFactoryRegistry (グローバル)
├─ UnifiedBoxRegistry (NewBox の入口)
├─ BuiltinBoxFactory (UnifiedBoxRegistry 内の factory)
├─ CoreBoxId enum (型定義)
├─ CoreServices (Service trait)
└─ TypeRegistry (メソッド情報)
```

**問題**:
- どのレジストリが「正」なのか不明
- 登録順序・優先度の規約がない
- 初期化タイミングが散在

### 2. **VM と LLVM の分岐**

| 項目 | VM Backend | LLVM Backend |
|------|-----------|--------------|
| Box 登録 | UnifiedBoxRegistry + BoxFactoryRegistry | TypeRegistry + Plugin FFI |
| メソッド解決 | BoxCall handler | ConsoleLlvmBridge |
| ABI | NyashBox trait | i8* + i64 |
| SSOT 化 | ❌ 不明 | ✅ Phase 133 完了 |

### 3. **失敗パターンの特定**

Phase 150/151 で報告されたエラー:

```
[ERROR] ❌ [rust-vm] VM error: Invalid instruction: NewBox ConsoleBox:
  invalid operation: Unknown Box type: ConsoleBox. Available: Main
```

**原因推測**:
1. セルフホスト経由時にプラグイン初期化が失敗
2. plugin provider mapping（BoxFactoryRegistry）が未設定で PluginBoxFactory が失敗
3. BuiltinBoxFactory が無効（`plugins-only`）または該当 Box が builtin factory に登録されていない

**Phase 151 の対処**:
- `builtin_impls/console_box.rs` を builtin 実装として追加/整備（UnifiedBoxRegistry 内の BuiltinBoxFactory 経由）
- "Available: Main" → ConsoleBox が登録されるようになった

## 💡 SSOT 化提案

### 設計原則

1. **Single Source of Truth**: Box 登録規約を一箇所に集約
2. **Explicit Dependencies**: 依存関係を明示化
3. **Fail-Fast**: エラーは早期に明示的に失敗
4. **Box-First**: Phase 33 の箱理論に基づく設計

### 提案 A: UnifiedBoxRegistry を入口SSOTとして強化（保守的アプローチ）

```rust
// src/runtime/unified_registry.rs / src/box_factory/mod.rs を “入口SSOT” として扱い、
// provider mapping や core_required 検証の責務をどこまで寄せるかを整理する。

pub struct UnifiedBoxRegistry {
    // SSOT: すべての Box 登録情報
    factories: RwLock<BTreeMap<String, BoxProvider>>,

    // CoreBoxId による必須 Box 検証
    core_validator: CoreBoxValidator,

    // プラグインローダーとの接続
    plugin_loader: Arc<PluginHost>,
}

impl UnifiedBoxRegistry {
    /// 初期化時に core_required Box を強制登録
    pub fn new(profile: RuntimeProfile) -> Self {
        let mut reg = Self { /* ... */ };

        // CoreBoxId に基づく必須 Box 登録
        for box_id in CoreBoxId::iter() {
            if box_id.is_required_in(&profile) {
                reg.register_core_box(box_id)?;
            }
        }

        reg
    }

    /// Box 生成（優先順位: Plugin > Builtin > Error）
    pub fn create_box(&self, name: &str, args: &[Box<dyn NyashBox>])
        -> Result<Box<dyn NyashBox>, RuntimeError>
    {
        // 1. プラグイン検索
        if let Some(provider) = self.factories.read().unwrap().get(name) {
            match provider {
                BoxProvider::Plugin(plugin_name) => {
                    return self.plugin_loader.create_box(name, args);
                }
                BoxProvider::Builtin(constructor) => {
                    return constructor(args);
                }
            }
        }

        // 2. エラー（SSOT なので見つからない = 存在しない）
        Err(RuntimeError::InvalidOperation {
            message: format!("Unknown Box type: {}", name)
        })
    }

    /// プラグイン設定の適用（nyash.toml から）
    pub fn apply_plugin_config(&mut self, config: &PluginConfig) {
        for (box_name, plugin_name) in &config.plugins {
            self.factories.write().unwrap()
                .insert(box_name.clone(), BoxProvider::Plugin(plugin_name.clone()));
        }
    }
}
```

**利点**:
- 既存コードへの影響最小
- CoreBoxId による型安全性
- プラグイン優先順位の明確化

### 提案 B: CoreBoxRegistry 新設（理想的アプローチ）

```rust
// src/runtime/core_box_registry.rs (新規)

/// Phase 131-2: Core Box 登録の SSOT
pub struct CoreBoxRegistry {
    // CoreBoxId → 登録情報のマッピング
    core_boxes: RwLock<HashMap<CoreBoxId, CoreBoxEntry>>,

    // ユーザー定義 Box（CoreBoxId 以外）
    user_boxes: RwLock<HashMap<String, UserBoxEntry>>,
}

struct CoreBoxEntry {
    box_id: CoreBoxId,
    provider: CoreBoxProvider,
    metadata: CoreBoxMetadata,
}

enum CoreBoxProvider {
    Plugin { plugin_name: String, type_id: u32 },
    Builtin { constructor: fn(&[Box<dyn NyashBox>]) -> Result<Box<dyn NyashBox>, RuntimeError> },
}

impl CoreBoxRegistry {
    /// 初期化時の必須 Box 検証
    pub fn validate_core_boxes(&self, profile: &RuntimeProfile) -> Result<(), String> {
        let missing: Vec<_> = CoreBoxId::iter()
            .filter(|id| id.is_required_in(profile))
            .filter(|id| !self.has_core_box(*id))
            .collect();

        if !missing.is_empty() {
            return Err(format!("Missing core_required boxes: {:?}", missing));
        }

        Ok(())
    }

    /// Box 生成（型安全）
    pub fn create_core_box(&self, box_id: CoreBoxId, args: &[Box<dyn NyashBox>])
        -> Result<Box<dyn NyashBox>, RuntimeError>
    {
        let entry = self.core_boxes.read().unwrap()
            .get(&box_id)
            .ok_or_else(|| RuntimeError::InvalidOperation {
                message: format!("Core box not registered: {:?}", box_id)
            })?
            .clone();

        match entry.provider {
            CoreBoxProvider::Plugin { plugin_name, type_id } => {
                self.create_plugin_box(&plugin_name, type_id, args)
            }
            CoreBoxProvider::Builtin { constructor } => {
                constructor(args)
            }
        }
    }
}
```

**利点**:
- CoreBoxId による完全な型安全性
- 必須 Box の起動時検証
- プラグイン vs ビルトインの明確な分離

## 🎯 次のステップ

### Phase 131-3: SSOT 実装（推奨）

1. **入口SSOTの確定（VM NewBox）**
   - 入口は `src/backend/mir_interpreter/handlers/boxes.rs::handle_new_box` と
     `src/runtime/unified_registry.rs` / `src/box_factory/mod.rs`（UnifiedBoxRegistry）で固定する
   - BoxFactoryRegistry は “plugin provider mapping” として位置付け、関係図を SSOT 化する

2. **CoreBoxId との統合**
   - `CoreBoxValidator` 実装
   - 起動時の必須 Box 検証

3. **プラグインローダーとの接続明確化**
   - plugin_loader_unified が BoxFactoryRegistry を populate し、
     PluginBoxFactory（UnifiedBoxRegistry 内）が BoxFactoryRegistry を参照する流れを明文化
   - TypeBox v2 FFI との整合（method/type id の参照箇所）を整理

4. **既存 BoxFactoryRegistry の整理**
   - provider mapping を BoxFactoryRegistry に閉じるか、UnifiedBoxRegistry に吸収するかを決める
   - 移行ガイドの作成

### Phase 131-4: テストケース追加

```rust
#[test]
fn test_core_box_registration() {
    let reg = UnifiedBoxRegistry::new(RuntimeProfile::Default);

    // core_required が全て登録されていること
    for box_id in CoreBoxId::iter() {
        if box_id.is_core_required() {
            assert!(reg.has_box(box_id.name()));
        }
    }

    // ConsoleBox が生成できること
    let console = reg.create_box("ConsoleBox", &[]).unwrap();
    assert_eq!(console.type_name(), "ConsoleBox");
}

#[test]
fn test_plugin_priority() {
    let mut reg = UnifiedBoxRegistry::new(RuntimeProfile::Default);

    // ビルトイン ConsoleBox 登録済み
    assert!(reg.has_box("ConsoleBox"));

    // プラグイン設定で上書き
    let mut config = PluginConfig::default();
    config.plugins.insert("ConsoleBox".to_string(), "nyash-console".to_string());
    reg.apply_plugin_config(&config);

    // プラグイン版が優先されること
    let console = reg.create_box("ConsoleBox", &[]).unwrap();
    // プラグイン判定ロジック（実装依存）
}
```

## 📋 関連ファイル一覧

### VM Backend
- `src/runtime/box_registry.rs` - BoxFactoryRegistry
- `src/runtime/unified_registry.rs` - global UnifiedBoxRegistry accessor
- `src/box_factory/mod.rs` - UnifiedBoxRegistry（FactoryPolicy / create_box）
- `src/box_factory/plugin.rs` - PluginBoxFactory（BoxFactoryRegistry を参照）
- `src/runtime/core_box_ids.rs` - CoreBoxId enum
- `src/runtime/core_services.rs` - CoreServices trait
- `src/runtime/type_registry.rs` - TypeRegistry (メソッド情報)
- `src/backend/mir_interpreter/handlers/boxes.rs` - handle_new_box()
- `src/box_factory/builtin_impls/console_box.rs` - Builtin 実装（BuiltinBoxFactory 経由）

### LLVM Backend
- `src/llvm_py/console_bridge.py` - ConsoleLlvmBridge（Phase 133）
- `src/llvm_py/instructions/boxcall.py` - BoxCall lowering

### Plugin System
- `plugins/nyash-console-plugin/src/lib.rs` - TypeBox v2 FFI
- `src/runtime/plugin_host.rs` - PluginHost

### Core Infrastructure
- `src/boxes/console_box.rs` - ConsoleBox 実装
- `crates/nyash_kernel/src/lib.rs` - NyRT ランタイム

## 🔍 未解決の疑問

1. **UnifiedBoxRegistry ↔ BoxFactoryRegistry の責務境界**
   - provider mapping の SSOT をどこに置くか（BoxFactoryRegistry を残す/吸収する）
   - `UnifiedBoxRegistry::FactoryPolicy` と plugin_config の関係をどう可視化するか

2. **Provider Lock の役割**
   - `provider_lock::guard_before_new_box()` の実装と目的
   - 「既定は挙動不変」の意味

3. **CoreServices の実装者**
   - ConsoleService trait を実装しているのは誰か
   - VM の Box 呼び出しがこの trait を経由するのか

4. **初期化順序**
   - プラグインローダー、レジストリ、サービスの初期化タイミング
   - 循環依存の有無

## ✅ 成果

### 明確になった事実

1. **LLVM backend は問題なし** - Phase 133 で完全解決済み
2. **VM backend の Box 解決が分散して見える** - UnifiedBoxRegistry（入口）と BoxFactoryRegistry（provider mapping）の関係を SSOT 化する必要がある
3. **ConsoleBox 自体は完全実装** - 登録メカニズムの問題
4. **CoreBoxId システム存在** - 型安全性の基盤あり

### 提案した解決策

1. **UnifiedBoxRegistry 強化** - 保守的アプローチ
2. **CoreBoxRegistry 新設** - 理想的アプローチ
3. **起動時検証** - Fail-Fast 原則の実現

---

**Status**: Investigation Complete - Ready for Implementation
**Next Phase**: 131-3 (SSOT Implementation)
**Estimated Time**: 4-6 hours
