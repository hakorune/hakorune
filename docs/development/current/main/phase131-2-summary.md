# Phase 131-2: ConsoleBox 問題根治調査 - エグゼクティブサマリ

## 🎯 調査結論（3行サマリ）

1. **LLVM 側は既存の統合事例がある** - Phase 133（archive）を参考にできる
2. **VM backend の Box 解決が分散して見える** - UnifiedBoxRegistry（入口）と BoxFactoryRegistry（plugin provider mapping）の関係が追いにくい
3. **ConsoleBox 自体は問題なし** - 迷子の原因は “どこを見ればよいか” の SSOT 不在

## 📊 問題の本質

### VM Backend の Box 解決が「2層 + 特例」に見える（問題の根源）

```
┌─ BoxFactoryRegistry (src/runtime/box_registry.rs)
│  └─ Plugin-First 設計、グローバルレジストリ
│
├─ UnifiedBoxRegistry (src/box_factory/mod.rs) + global accessor (src/runtime/unified_registry.rs)
│  └─ MIR `NewBox`（VM）の入口
│
└─ VM fast path (NYASH_VM_FAST=1 + StringBox)
   └─ ベンチ/プロファイル用の最適化（入口で分岐するため観測なしだと混乱しやすい）
```

**問題**:
- どのレジストリが「正」なのか不明
- 登録順序・優先度の規約がない
- VM と LLVM で Box 解決方法が異なる

## ✅ LLVM Backend の成功事例（参考モデル）

Phase 133 で既に **完全な SSOT 化** を達成：

```python
# ConsoleLlvmBridge (src/llvm_py/console_bridge.py)
CONSOLE_METHODS = {
    "log": "nyash.console.log",
    "println": "nyash.console.log",  # Phase 122: エイリアス統一
    "warn": "nyash.console.warn",
    "error": "nyash.console.error",
    "clear": "nyash.console.clear",
}
```

**成果**:
- ✅ TypeRegistry との ABI 完全一致（slot 400-403）
- ✅ Phase 122 のエイリアス統一を継承
- ✅ 7/7 テスト全て PASS

## 💡 推奨アクション

### 優先度 1: 入口SSOTの所在確認（VM NewBox）

```bash
# 主要入口の所在（この3箇所を見る）
rg "get_global_unified_registry\\(|struct UnifiedBoxRegistry|struct BoxFactoryRegistry" src/ --type rust
```

**次の判断**:
- UnifiedBoxRegistry を “入口SSOT” として明文化し、BoxFactoryRegistry は plugin provider mapping として位置付ける
- その上で CoreBoxId / CoreServices との接続（Fail-Fast の境界）を設計する

### 優先度 2: CoreBoxId との統合

```rust
// CoreBoxId に基づく必須 Box 検証
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
```

### 優先度 3: Fail-Fast 原則の徹底

```rust
// ❌ 悪い例：フォールバック
if let Err(_) = create_plugin_box() {
    create_builtin_box()  // 隠蔽される！
}

// ✅ 良い例：即座に失敗
create_plugin_box()
    .map_err(|e| format!("ConsoleBox plugin failed: {:?}", e))?
```

## 🔍 未解決の疑問（Phase 131-3 で調査）

1. **UnifiedBoxRegistry ↔ BoxFactoryRegistry の責務境界**
   - “plugin provider mapping” を BoxFactoryRegistry に閉じ込めるなら、どこまで公開するか（例: box_types 一覧）
   - `NYASH_BOX_FACTORY_POLICY` と plugin_config の関係を SSOT 化する場所

2. **Provider Lock の役割**
   - `provider_lock::guard_before_new_box()` の目的
   - 「既定は挙動不変」の意味

3. **CoreServices の実装者**
   - ConsoleService trait を誰が実装しているか
   - Box 呼び出しが trait を経由するか

## 📋 関連ドキュメント

- **詳細レポート**: [phase131-2-consolebox-investigation.md](./phase131-2-consolebox-investigation.md)
- **Phase 133 成果**: [docs/archive/phases/phase-106-156/phase133_consolebox_llvm_integration.md](../../../archive/phases/phase-106-156/phase133_consolebox_llvm_integration.md)
- **CoreBoxId 設計**: `src/runtime/core_box_ids.rs`
- **TypeRegistry**: `src/runtime/type_registry.rs`

## ⏱️ 次のステップ（Phase 131-3）

### タスク概要

1. SSOT 設計決定（1時間）
2. 実装（2-3時間）
3. テスト追加（1時間）

**合計見積もり**: 4-6時間

### 完成条件

- [ ] UnifiedBoxRegistry / BoxFactoryRegistry の責務境界を SSOT として固定
- [ ] CoreBoxId に基づく必須 Box 検証実装
- [ ] プラグイン vs ビルトイン優先順位の明確化
- [ ] 起動時の core_required Box 検証テスト追加
- [ ] VM/LLVM 両方で ConsoleBox 生成成功確認

---

**Status**: Investigation Complete ✅
**Next Phase**: 131-3 (SSOT Implementation)
**Owner**: ChatGPT + Claude 協働
