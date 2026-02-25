# 🔄 現在のVM変更状態 (2025-08-21)

> **Status**: Legacy snapshot（Phase 9.78 系の記録）  
> **Note**: 現在の正本はリポジトリ直下の `CURRENT_TASK.md` および `docs/development/roadmap/` / `docs/private/roadmap/` 側に集約しています。このファイルは当時の実装状況メモとしてのみ残しています。

## 📊 Phase 9.78a VM統一Box処理の実装状況

### ✅ 完了したステップ

#### **Step 1: MIR生成修正** ✅
`src/mir/builder.rs`の変更内容：
```rust
// 変更前: RefNew命令（不適切）
match class.as_str() {
    "IntegerBox" | "StringBox" | "BoolBox" => {
        emit(MirInstruction::Const { ... })
    }
    _ => {
        emit(MirInstruction::RefNew { ... })
    }
}

// 変更後: NewBox命令（統一）
emit(MirInstruction::NewBox {
    dst,
    box_type: class,
    args: arg_values,
})
```
**評価**: ✅ 良い変更。すべてのBox型を統一的に扱える。

#### **Step 2: VM構造体拡張** 🔧 部分完了
`src/backend/vm.rs`の変更内容：
1. **新規インポート追加**:
   - `BoxFactory` → ❌ trait/struct混在問題
   - `InstanceBox` ✅
   - `BoxDeclaration` → ⚠️ interpreter依存
   - `ScopeTracker` ✅

2. **VM構造体への追加**:
   ```rust
   box_factory: Arc<BoxFactory>, // ❌ エラー：traitには dyn 必要
   plugin_loader: Option<Arc<PluginLoaderV2>>,
   scope_tracker: ScopeTracker,
   box_declarations: Arc<RwLock<HashMap<String, BoxDeclaration>>>,
   ```

3. **新規メソッド追加**:
   - `new_with_factory()` → 名前変更必要
   - `new_with_plugins()`

#### **Step 3: NewBox統一実装** 🔧 部分完了
VM内のNewBox命令処理を統一実装に更新：
```rust
// BoxFactory経由で作成
let new_box = match self.box_factory.create_box(box_type, arg_boxes) {
    Ok(boxed) => boxed,
    Err(e) => return Err(...),
};
```
**問題**: BoxFactoryがtraitなのでコンパイルエラー

#### **Step 4: BoxCall統一実装** ✅ 完了
- `call_unified_method()`を追加
- 現在は簡易実装（call_box_methodに委譲）

#### **Step 5: ライフサイクル管理** 🔧 部分完了
- `ScopeTracker`を新規作成
- `execute_function()`でスコープ管理追加
- fini実装は簡易版

### 🚨 現在の問題点

1. **BoxFactory trait問題**:
   - VMはBoxFactoryをstructとして期待
   - 実際はtraitとして定義されている
   - `UnifiedBoxRegistry`を使うべきか？

2. **BoxDeclaration依存問題**:
   - `interpreter::BoxDeclaration`を使用
   - VMからinterpreterへの依存は良くない

3. **ビルドエラー**:
   ```
   error[E0782]: expected a type, found a trait
   --> src/backend/vm.rs:175:22
   ```

## 🎯 推奨アクション

### **Option A: 置いておく（推奨）** ✅
**理由**:
- MIR生成修正（Step 1）は良い変更で保持すべき
- VM拡張の方向性は正しい
- インタープリター整理後に再開が効率的

**実行手順**:
```bash
# 現在の変更を一時保存
git stash push -m "Phase 9.78a VM unified Box handling WIP"

# または feature ブランチに保存
git checkout -b feature/vm-unified-box-wip
git add -A
git commit -m "WIP: Phase 9.78a VM unified Box handling"
git checkout main
```

### **Option B: 部分的に保持**
**保持すべき部分**:
- ✅ MIR生成修正（Step 1）
- ✅ ScopeTracker実装

**巻き戻すべき部分**:
- ❌ VM構造体へのBoxFactory追加
- ❌ interpreter::BoxDeclaration依存

### **Option C: 全て巻き戻す**
**非推奨**: MIR生成修正は価値があり、保持すべき

## 📝 今後の計画

1. **Phase 1**: インタープリター整理
   - BoxDeclarationをast.rsへ移動
   - SharedState依存を減らす
   - NyashRuntime共通基盤作成

2. **Phase 2**: VM実装再開
   - 整理されたインターフェースを使用
   - UnifiedBoxRegistryベースで実装
   - プラグインシステム統合

## 🔧 技術的詳細

### 変更されたファイル
- `src/mir/builder.rs`: -72行（RefNew → NewBox）
- `src/backend/vm.rs`: +164行（構造体拡張、メソッド追加）
- `src/lib.rs`: +1行（scope_trackerモジュール）
- `src/scope_tracker.rs`: 新規ファイル（68行）

### 依存関係の問題
```
VM → interpreter::BoxDeclaration ❌
VM → BoxFactory (trait) ❌
VM → UnifiedBoxRegistry ✅ (推奨)
```

---

**結論**: **Option A（置いておく）**を推奨します。現在の実装は方向性として正しく、インタープリター整理後に続きから再開するのが最も効率的です。
