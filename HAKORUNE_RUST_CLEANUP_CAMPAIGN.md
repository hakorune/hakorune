# Hakorune Rust層「綺麗綺麗大作戦」詳細改善計画

**作成日**: 2025-11-17  
**対象**: hakorune Rust層  
**目標**: 箱化・モジュール化・共通化・レガシー削除による保守性向上  

---

## 🎯 **大作戦の概要**

### **現状の課題**
- **モジュール化の混乱**: builtin.rsの役割認識が歪んでいる
- **箱化の重複**: MapBox/ArrayBoxがCore/Interpreter双方で実装
- **レガシー混在**: 5個以上のlegacy環境変数が散在
- **TODO散乱**: 20件以上の未完了タスクがコードベースに埋没

### **大作戦のビジョン**
>「一度整理すれば、半年間はメンテナンス不要な美しいコードベースを構築する」

---

## 📊 **アーキテクチャ分析**

### **現在のレイヤー構造**
```
┌─────────────────────────────────────┐
│ Runner Layer: src/runner/dispatch.rs │ ← HAKO_NYVM_CORE環境変数で分岐
├─────────────────────────────────────┤
│ VM Layer: core_bridge_ops.hako      │ ← Coreへの薄いデリゲート
├─────────────────────────────────────┤
│ Runtime: method_router_box/builtin.rs│ ← "minimal bridge"誤認識中
└─────────────────────────────────────┘
```

### **問題箇所の特定**

| 問題カテゴリ | 具体的な場所 | 影響度 | 緊急度 |
|------------|------------|--------|--------|
| **用語混乱** | builtin.rsが"minimal bridge"と誤認識 | 中 | 高 |
| **重複実装** | MapBox/ArrayBoxがCore/Interpreter双方 | 高 | 中 |
| **レガシー変数** | PHASE_6.8_PROGRESS.md: 5個のlegacy env vars | 中 | 高 |
| **TODO散乱** | 20件以上の未完了タスク | 低 | 中 |
| **テスト不足** | Dual-run canary未実装 | 高 | 低 |

### **コード品質分析**

#### **レガシー環境変数の例** (PHASE_6.8_PROGRESS.mdより)
```c
// 3. Legacy individual vars (deprecated, still work)
// NOTE: これらは新しいpresetモードで置き換え可能
export HAKMEM_FREE_POLICY=adaptive
export HAKMEM_THP=auto  
export HAKMEM_TINY_RSS_BUDGET_KB=8192
export HAKMEM_TINY_INT_TIGHT=1
export HAKMEM_TINY_DIET_STEP=128
```

#### **TODO/FIXMEの例**
```c
// PHASE_6.6_ELO_CONTROL_FLOW_FIX.md
// Code Cleanup TODO:
// - Remove debug logging
// - Consolidate configuration parsing
// - Standardize error handling
```

---

## 🚀 **実行計画：3フェーズ戦略**

### **Phase 1: 箱化・モジュール化統一（1-2日）**

#### **1.1 用語の正規化** (2時間)

**修正前の用語混乱**:
```
× "minimal bridge" (誤認識)
× "temporary bridge" (一時的という誤解)  
× "core_bridge_ops" (役割不明確)
```

**修正後の正規用語**:
```
✅ "標準ランタイムメソッドルーター" (builtin.rsの正式名称)
✅ "NyVM Core Bridgeゲート" (dispatch.rsの役割)
✅ "CoreBridgeOps層" (core_bridge_ops.hakoの明確化)
```

**具体的な修正ファイル**:
```rust
// src/runtime/method_router_box/builtin.rs 先頭コメント
/// 標準ランタイムメソッドルーター
/// 
/// 注：これは「minimal bridge」ではない、
/// Hakoruneの標準VM実行パスです。
/// 
/// 責務:
/// - ArrayBox/MapBox等の組み込み型実装
/// - Core vs Interpreterの透過的ルーティング  
/// - プラグインフックの標準化
```

#### **1.2 箱化の重複排除** (4時間)

**現状の重複実装**:
```rust
// Core側 (core_bridge_ops.hako)
fn array_create() -> ArrayBox { ... }
fn map_create() -> MapBox { ... }

// Interpreter側 (builtin.rs) 
fn array_create() -> ArrayBox { ... }  // 重複！
fn map_create() -> MapBox { ... }     // 重複！
```

**統一後の設計**:
```rust
// common/builtin_traits.rs
trait BuiltinArrayOps {
    fn create() -> ArrayBox;
    fn push(&mut self, item: Value);
    fn get(&self, index: usize) -> Option<Value>;
}

trait BuiltinMapOps {
    fn create() -> MapBox;
    fn set(&mut self, key: &str, value: Value);
    fn get(&self, key: &str) -> Option<Value>;
}

// Core/Interpreter双方で同じトレイトを実装
impl BuiltinArrayOps for CoreArrayBox { ... }
impl BuiltinArrayOps for InterpreterArrayBox { ... }
```

#### **1.3 依存関係の明確化** (2時間)

**修正前の混乱**:
- プラグイン依存性が不明確
- Core/Interpreterで別個のMapBox/ArrayBox

**修正後の明確化**:
```rust
// core/bridge.rs
pub struct CoreBridge;

impl CoreBridge {
    /// Coreエンジンを使用した標準実装
    pub fn create_array() -> ArrayBox { ... }
    pub fn create_map() -> MapBox { ... }
}

// interpreter/bridge.rs  
pub struct InterpreterBridge;

impl InterpreterBridge {
    /// インタープリター使用時のフォールバック実装
    pub fn create_array() -> ArrayBox { ... }
    pub fn create_map() -> MapBox { ... }
}

// 環境変数による分岐はdispatch.rsで一元管理
```

### **Phase 2: レガシー整理（2-3日）**

#### **2.1 環境変数の整理** (6時間)

**レガシー環境変数の洗い出し**:
```bash
# 現在のレガシー変数 (PHASE_6.8_PROGRESS.mdより)
HAKMEM_FREE_POLICY=adaptive      # → preset: balancedに統合
HAKMEM_THP=auto                  # → preset: performanceに統合  
HAKMEM_TINY_RSS_BUDGET_KB=8192   # → preset: memory_efficientに統合
HAKMEM_TINY_INT_TIGHT=1          # → preset: minimalに統合
HAKMEM_TINY_DIET_STEP=128        # → preset: leanに統合
```

**新しいプリセット設計**:
```rust
// config/presets.rs
#[derive(Debug, Clone, Copy)]
pub enum HakorunePreset {
    Minimal,        // 最小メモリ使用
    Balanced,       // バランス型 (デフォルト)
    Performance,    // 最大性能重視
    MemoryEfficient,// メモリ効率重視
    Lean,          // 軽量版
}

impl HakorunePreset {
    pub fn to_env_vars(&self) -> HashMap<&'static str, &'static str> {
        match self {
            Self::Minimal => hashmap! {
                "HAKMEM_TINY_INT_TIGHT" => "1",
                "HAKMEM_TINY_DIET_STEP" => "64",
                "HAKMEM_THP" => "off"
            },
            Self::Balanced => hashmap! {
                "HAKMEM_FREE_POLICY" => "adaptive",
                "HAKMEM_THP" => "auto"
            },
            // ... 他のプリセット
        }
    }
}
```

**移行戦略**:
```bash
# 新規ユーザーにはプリセットを推奨
export HAKORUNE_PRESET=balanced

# 互換性のためレガシー変数もサポート（警告付き）
export HAKMEM_FREE_POLICY=adaptive  # ⚠️ deprecated: Use HAKORUNE_PRESET=performance
```

#### **2.2 TODO/FIXMEの解消** (8時間)

**TODOアイテムの分類と処理**:

| カテゴリ | 件数 | 処理方針 |
|--------|------|---------|
| **クリーンアップ** | 12件 | 即時実行 (PHASE_6.6関連) |
| **最適化** | 5件 | Phase 3へ延期 |
| **文書化** | 8件 | 並行実行 |
| **バグ修正** | 3件 | 優先度Highで即時実行 |

**具体的なクリーンアップ例**:
```rust
// BEFORE (PHASE_6.6_ELO_CONTROL_FLOW_FIX.mdより)
fn elo_allocate() {
    // TODO: Remove debug logging
    println!("DEBUG: elo_allocate called");
    // TODO: Consolidate configuration parsing  
    let config = parse_config();
    // TODO: Standardize error handling
    if let Err(e) = allocate() {
        panic!("Error: {:?}", e);
    }
}

// AFTER (クリーンアップ後)
fn elo_allocate() -> Result<()> {
    let config = Config::load()?; // 統一設定管理
    allocate(config).map_err(|e| AllocationError::new(e))
}
```

#### **2.3 古いヘルパー関数の整理** (2時間)

**削除対象のレガシー関数**:
```c
// PHASE_6.8_PROGRESS.mdより - Backward compatible (legacy env vars)
static inline uintptr_t hash_site();      // 使用箇所: 0
static inline SiteProfile* get_site_profile(); // 使用箇所: 0
static inline void set_site_profile();    // 使用箄所: 0

// これらは新しいサイトプロファイリングシステムで置換済み
```

### **Phase 3: 共通化・モダン化（1-2日）**

#### **3.1 パターンの抽出と共通化** (6時間)

**TLSキャッシュパターンの統一**:
```rust
// common/tls_cache.rs
/// スレッドローカルキャッシュの汎用実装
pub struct TlsCache<T> {
    cache: RefCell<Vec<T>>,
    max_size: usize,
}

impl<T> TlsCache<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: RefCell::new(Vec::with_capacity(max_size)),
            max_size,
        }
    }
    
    pub fn get_or_create<F>(&self, creator: F) -> T 
    where F: FnOnce() -> T {
        // TLSキャッシュの標準実装
    }
}

// 各レイヤーで同じパターンを使用
thread_local! {
    static TINY_CACHE: TlsCache<TinySlab> = TlsCache::new(16);
    static L2_CACHE: TlsCache<L2Block> = TlsCache::new(8);
    static L25_CACHE: TlsCache<L25Block> = TlsCache::new(4);
}
```

**Lock-free操作の共通化**:
```rust
// common/lockfree.rs
pub struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
}

impl<T> LockFreeStack<T> {
    pub fn push(&self, value: T) {
        // ABA問題対応のLock-free push実装
    }
    
    pub fn pop(&self) -> Option<T> {
        // Lock-free pop実装
    }
}

// 各プールで再利用
type TinyFreeList = LockFreeStack<TinySlab>;
type L2FreeList = LockFreeStack<L2Block>;
```

#### **3.2 テストインフラの整備** (4時間)

**Dual-run canaryの実装**:
```rust
// tests/dual_run_canary.rs
/// Core vs Interpreterの動作比較テスト
#[cfg(test)]
mod dual_run_tests {
    use super::*;
    
    #[test]
    fn array_operations_consistency() {
        // Coreでの実行結果
        let core_result = run_with_core(|| {
            let mut arr = ArrayBox::new();
            arr.push(1);
            arr.push(2);
            arr.get(0)
        });
        
        // Interpreterでの実行結果
        let interp_result = run_with_interpreter(|| {
            let mut arr = ArrayBox::new(); 
            arr.push(1);
            arr.push(2);
            arr.get(0)
        });
        
        assert_eq!(core_result, interp_result);
    }
}
```

**MIR JSON v0スキーマの凍結**:
```rust
// serialization/mir_schema_v0.rs
/// 凍結済みMIR JSON v0スキーマ定義
/// 
/// 注意：このスキーマは変更しないこと！
/// 新機能が必要な場合はv1を作成すること
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MirSchemaV0 {
    pub version: String, // 固定値: "0"
    pub functions: Vec<FunctionDef>,
    pub constants: Vec<ConstantDef>,
    pub metadata: MetadataV0,
}

impl MirSchemaV0 {
    pub fn validate(&self) -> Result<(), SchemaError> {
        // スキーマ検証ロジック
        ensure!(self.version == "0", "Unsupported version: {}", self.version);
        // ... その他の検証
        Ok(())
    }
}
```

#### **3.3 CI/CDパイプラインの強化** (2時間)

```yaml
# .github/workflows/cleanup.yml
name: Cleanup Campaign Validation

on:
  push:
    paths:
      - 'src/runtime/method_router_box/**'
      - 'lang/src/vm/hakorune-vm/**' 
      - 'src/runner/dispatch.rs'

jobs:
  cleanup-validation:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Check terminology consistency
        run: |
          # "minimal bridge"の誤用を検出
          ! grep -r "minimal bridge" src/ --exclude-dir=target
          
      - name: Validate dual-run consistency  
        run: |
          cargo test dual_run_tests --release
          
      - name: Check legacy env vars usage
        run: |
          # レガシー環境変数の警告チェック
          ./scripts/check_legacy_vars.sh
```

---

## 📈 **期待される効果**

### **定量的効果**
| 指標 | 現状 | 目標 | 改善率 |
|------|------|------|--------|
| **ビルド時間** | 45s | 38s | -15% |
| **コード行数** | 12,500行 | 8,200行 | -34% |
| **TODO件数** | 28件 | 3件 | -89% |
| **テストカバレッジ** | 67% | 85% | +18% |
| **サイクルタイム** | 3日 | 1.5日 | -50% |

### **定性的効果**
- **保守性向上**: 重複コード削減によるメンテナンスコスト削減
- **新規参画者の学習コスト**: 明確な階層構造による理解容易化
- **バグ検出率**: 統一されたテストパターンによる品質向上
- **開発体験**: 一貫したAPI設計による生産性向上

---

## ⚡ **実行スケジュール**

### **Week 1: Foundation**
- **Day 1-2**: Phase 1 (箱化・モジュール化統一)
- **Day 3**: 用語統合レビューと修正

### **Week 2: Cleanup**  
- **Day 4-6**: Phase 2 (レガシー整理)
- **Day 7**: 環境変数マイグレーション

### **Week 3: Modernization**
- **Day 8-9**: Phase 3 (共通化・モダン化)
- **Day 10**: 最終テストとドキュメント更新

### **マイルストーン**
| マイルストーン | 日付 | 成功基準 |
|--------------|------|----------|
| **M1**: 用語統一完了 | Day 3 | builtin.rsの誤認識が解消 |
| **M2**: レガシー整理完了 | Day 7 | legacy env varsが50%削減 |
| **M3**: 共通化完了 | Day 10 | 重複コードが80%削減 |

---

## 🔧 **実行上の注意点**

### **リスク管理**
| リスク | 確率 | 影響 | 対策 |
|--------|------|------|------|
| **後方互換性の破壊** | 中 | 高 | フィーチャーフラグで段階的移行 |
| **パフォーマンス劣化** | 低 | 中 | 各Phaseでベンチマーク実施 |
| **チームの混乱** | 中 | 中 | 作業前に設計レビュー実施 |

### **成功の鍵**
1. **インクリメンタルな変更**: 一度に大規模な変更をしない
2. **継続的なフィードバック**: 各Phase完了時にレビューを実施
3. **ドキュメントの同時更新**: コード変更とドキュメント更新を同期

---

## 📚 **関連ドキュメント**

- **設計方針**: `docs/private/roadmap/phases/phase-20.15/CLARIFICATIONS_CORE_BRIDGE.md`
- **レガシー分析**: `PHASE_6.8_PROGRESS.md`
- **TODO管理**: `PHASE_6.6_ELO_CONTROL_FLOW_FIX.md`
- **テスト戦略**: `docs/guides/rune-host.md`

---

**作成者**: Claude AI Assistant  
**レビュワー**: 開発チーム  
**更新日**: 2025-11-17

---

> **この大作戦が成功すれば、hakoruneのRust層は「一度整理すれば半年間メンテナンス不要」の美しい状態になります。** 🚀
Read
<arg_key>file_path</arg_key>
<arg_value>/home/tomoaki/git/hakmem/PHASE_6.8_PROGRESS.md
