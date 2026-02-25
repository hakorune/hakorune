# Phase 131-2: Box Resolution Mapping - 現状 vs 理想

## 📍 VM Box 解決の入口SSOT（迷ったらここ）

- **MIR NewBox 受付**: `src/backend/mir_interpreter/handlers/boxes.rs` の `handle_new_box()`
- **Box 登録管理**: `src/runtime/unified_registry.rs` の `UnifiedBoxRegistry`
- **Factory 実装**: `src/box_factory/mod.rs` + Plugin は `src/runtime/box_registry.rs` の `BoxFactoryRegistry`
- **NYASH_VM_FAST**: bench/profile-only 特例（既定OFF、本番では使わない）

## 🗺️ 現状マップ（VM Backend）

```
MIR NewBox ConsoleBox
        ↓
MirInterpreter::handle_new_box()  (src/backend/mir_interpreter/handlers/boxes.rs)
        ↓
    [分岐点 1: Fast path?]
        ├─ Yes (NYASH_VM_FAST=1 + StringBox) → Direct creation（bench/profile-only）
        └─ No → Continue
                ↓
    [分岐点 2: Provider Lock]
        ├─ guard_before_new_box() → OK/NG
        └─ Continue
                ↓
    get_global_unified_registry() (src/runtime/unified_registry.rs)
                ↓
    UnifiedBoxRegistry::create_box("ConsoleBox", args) (src/box_factory/mod.rs)
                ↓
        [分岐点 3: FactoryPolicy による優先順位]
                ├─ BuiltinBoxFactory → builtin_impls（src/box_factory/builtin_impls/*）
                └─ PluginBoxFactory  → BoxFactoryRegistry → PluginHost
                       (src/box_factory/plugin.rs)  (src/runtime/box_registry.rs)
                       ※ plugin_loader_unified が BoxFactoryRegistry を populate する
                         （src/runtime/plugin_loader_unified.rs）
```

**問題点**:
- ❌ 「Box 解決 SSOT」が 1 箇所に見えない（UnifiedBoxRegistry と BoxFactoryRegistry に分散）
- ❌ 優先順位が “FactoryPolicy + provider mapping” の合成で、全体像が追いにくい
- ❌ `NYASH_VM_FAST` の特例が入口にあり、観測なしだと混乱しやすい

## 🎯 理想マップ（SSOT 化後）

```
MIR NewBox ConsoleBox
        ↓
handle_new_box()
        ↓
CoreBoxRegistry::create("ConsoleBox", args)
        ↓
    [SSOT: 登録情報検索]
    CoreBoxId::Console in registry?
        ├─ Yes → Continue
        └─ No → Error: "ConsoleBox not registered" (Fail-Fast!)
                ↓
    [優先順位: Plugin > Builtin]
    registered_provider?
        ├─ Plugin → PluginHost.create_box()
        │            ↓
        │       TypeBox v2 FFI
        │            ↓
        │       console_invoke_id()
        │            ↓
        │       ConsoleInstance::new()
        │
        └─ Builtin → builtin::ConsoleBox::new()
                ↓
        VMValue::BoxRef(ConsoleBox)
```

**改善点**:
- ✅ 単一の Box 解決ルート（SSOT）
- ✅ CoreBoxId による型安全性
- ✅ 明確な優先順位（Plugin > Builtin）
- ✅ Fail-Fast（見つからない = エラー）

## 📊 VM vs LLVM 比較表

| 項目 | VM Backend（現状） | LLVM Backend（Phase 133） | 理想（SSOT化後） |
|------|-------------------|--------------------------|-----------------|
| **Box 登録** | ⚠️ UnifiedBoxRegistry + BoxFactoryRegistry | ✅ TypeRegistry + Plugin FFI | ✅ CoreBoxRegistry |
| **メソッド解決** | ❌ 複数経路（boxes.rs 分岐） | ✅ ConsoleLlvmBridge 箱化 | ✅ 箱化モジュール |
| **ABI** | NyashBox trait | i8* + i64 (llvmlite) | ✅ 統一（TypeRegistry SSOT） |
| **優先順位** | ❓ 不明確 | ✅ Plugin > Builtin | ✅ 明示的優先順位 |
| **Fail-Fast** | ❌ フォールバック多数 | ✅ エラー即座に報告 | ✅ Fail-Fast 原則 |
| **型安全性** | ❌ 文字列ベース | ✅ TypeBox v2 | ✅ CoreBoxId enum |
| **SSOT 化** | ❌ 分散 | ✅ 完了（Phase 133） | ✅ 完了（目標） |

## 🔍 登録システム詳細比較

### 現状（VM Backend）

```rust
// System 1: BoxFactoryRegistry（plugin provider mapping）
impl BoxFactoryRegistry {
    pub fn create_box(&self, name: &str, args: &[Box<dyn NyashBox>])
        -> Result<Box<dyn NyashBox>, String>
    {
        let provider = self.get_provider(name)?;
        match provider {
            BoxProvider::Builtin(constructor) => constructor(args),
            BoxProvider::Plugin(plugin_name) =>
                self.create_plugin_box(&plugin_name, name, args),
        }
    }
}

// System 2: UnifiedBoxRegistry（VM NewBox の入口）
// src/backend/mir_interpreter/handlers/boxes.rs::handle_new_box
let reg = crate::runtime::unified_registry::get_global_unified_registry();
let created = reg.lock().unwrap().create_box(box_type, &converted)?;

// Builtins は UnifiedBoxRegistry 内で BuiltinBoxFactory が担当し、
// builtin_impls/* の実装へ委譲される（外部フォールバックではない）。
```

**問題**:
- 3つのシステムの関係が不明
- どれが優先されるのか不明確
- エラーハンドリングが統一されていない

### Phase 133（LLVM Backend）- 成功モデル

```python
# ConsoleLlvmBridge: 単一の箱化モジュール
def emit_console_call(builder, module, method_name, args, ...):
    if method_name not in CONSOLE_METHODS:
        return False  # Fail-Fast: 即座に不明通知

    runtime_fn_name = CONSOLE_METHODS[method_name]
    callee = _declare(module, runtime_fn_name, i64, [i8p])
    builder.call(callee, [arg0_ptr])
    return True  # 成功
```

**成功要因**:
- ✅ CONSOLE_METHODS が SSOT（唯一の真実）
- ✅ 箱化モジュール（1箇所に集約）
- ✅ Fail-Fast（不明メソッド = False 即座）
- ✅ TypeRegistry との ABI 一致

### 理想（SSOT化後の VM Backend）

```rust
// CoreBoxRegistry: 単一の SSOT
pub struct CoreBoxRegistry {
    core_boxes: RwLock<HashMap<CoreBoxId, CoreBoxEntry>>,
    user_boxes: RwLock<HashMap<String, UserBoxEntry>>,
}

impl CoreBoxRegistry {
    pub fn create(&self, box_name: &str, args: &[Box<dyn NyashBox>])
        -> Result<Box<dyn NyashBox>, RuntimeError>
    {
        // 1. CoreBoxId 変換
        let box_id = CoreBoxId::from_name(box_name)
            .ok_or_else(|| RuntimeError::InvalidOperation {
                message: format!("Unknown Box type: {}", box_name)
            })?;

        // 2. 登録情報取得（SSOT）
        let entry = self.core_boxes.read().unwrap()
            .get(&box_id)
            .ok_or_else(|| RuntimeError::InvalidOperation {
                message: format!("Box not registered: {:?}", box_id)
            })?
            .clone();

        // 3. 優先順位に従って生成
        match entry.provider {
            CoreBoxProvider::Plugin { plugin_name, type_id } => {
                self.plugin_host.create_box(&plugin_name, type_id, args)
            }
            CoreBoxProvider::Builtin { constructor } => {
                constructor(args)
            }
        }
    }
}
```

**改善点**:
- ✅ CoreBoxId による型安全性
- ✅ 単一の登録マップ（SSOT）
- ✅ 明示的な優先順位（Plugin > Builtin）
- ✅ Fail-Fast（登録なし = エラー）

## 🎯 Phase 133 の教訓

### 成功パターン: 箱化モジュール化

```
Before (Phase 132): 40 行の分岐が boxcall.py に埋め込み
After (Phase 133): 1 行の箱化呼び出し

# Before
if method_name in ("print", "println", "log"):
    # ... 40 行のロジック ...

# After
if emit_console_call(builder, module, method_name, args, ...):
    return
```

**成果**:
- 分岐を 1 箇所に集約
- テスト容易性向上
- レガシー削除が簡単

### VM Backend への適用

```rust
// Before（現状）: 複数経路の分岐
let reg = unified_registry::get_global_unified_registry();
let created = reg.lock().unwrap().create_box(box_type, &converted)?;

// After（Phase 131-3）: 箱化モジュール化
let created = CoreBoxRegistry::global()
    .create(box_type, &converted)?;
```

## 📋 Phase 131-3 実装ガイド

### Step 1: 現状SSOTの所在を固定（入口と接続）

```bash
# VM NewBox の入口（ここから追う）
rg "fn handle_new_box\\(" src/backend/mir_interpreter/handlers/boxes.rs

# global accessor と registry 本体
rg "get_global_unified_registry\\(" src/runtime/unified_registry.rs
rg "struct UnifiedBoxRegistry" src/box_factory/mod.rs

# plugin 側の provider mapping
rg "struct BoxFactoryRegistry" src/runtime/box_registry.rs
```

**前提**:
- UnifiedBoxRegistry は既に存在する（NewBox の入口）。
- BoxFactoryRegistry は PluginBoxFactory の provider mapping として間接利用される。

### Step 2: CoreBoxId 統合

```rust
// CoreBoxId に基づく検証ロジック追加
impl CoreBoxRegistry {
    pub fn validate_on_startup(&self, profile: &RuntimeProfile)
        -> Result<(), String>
    {
        for box_id in CoreBoxId::iter() {
            if box_id.is_required_in(&profile) && !self.has(box_id) {
                return Err(format!("Missing core_required box: {:?}", box_id));
            }
        }
        Ok(())
    }
}
```

### Step 3: プラグイン優先順位の明確化

```rust
// 登録時に優先順位を決定
impl CoreBoxRegistry {
    /// プラグイン設定適用（既存ビルトインを上書き）
    pub fn apply_plugin_config(&mut self, config: &PluginConfig) {
        for (box_name, plugin_name) in &config.plugins {
            if let Some(box_id) = CoreBoxId::from_name(box_name) {
                self.register_plugin(box_id, plugin_name);  // 上書き
            }
        }
    }
}
```

### Step 4: Fail-Fast の徹底

```rust
// ❌ 削除対象: フォールバックロジック
if let Err(_) = create_plugin_box() {
    create_builtin_box()  // 隠蔽！
}

// ✅ 追加: 即座にエラー
create_plugin_box()
    .map_err(|e| VMError::InvalidInstruction(
        format!("ConsoleBox plugin failed: {:?}. Check nyash.toml", e)
    ))?
```

## ✅ チェックリスト（Phase 131-3）

- [ ] UnifiedBoxRegistry / BoxFactoryRegistry の責務境界を SSOT として固定
- [ ] CoreBoxRegistry を新設するなら “入口SSOT” を 1 箇所にする（NewBox から見える形）
- [ ] CoreBoxId 統合（型安全性）
- [ ] プラグイン優先順位の明確化（Plugin > Builtin）
- [ ] Fail-Fast 原則の徹底（フォールバック削除）
- [ ] 起動時検証テスト追加
- [ ] VM/LLVM 両方で ConsoleBox 生成確認

---

**Status**: Ready for Implementation
**Next Phase**: 131-3 (SSOT Implementation)
**Estimated Time**: 4-6 hours
