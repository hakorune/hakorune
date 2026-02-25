# inst_meta層とused_values()の設計分析レポート

## 概要

MIRのメタデータシステム（inst_meta層）と使用値判定（used_values()）に関する設計的な問題と改善機会について分析します。

## 現状の構造

### 1. Call命令の特殊扱い（methods.rs lines 148-170）

```rust
// methods.rs: used_values()
if let MirInstruction::Call { callee, func, args, .. } = self {
    // Callee::Method { receiver: Some(r), .. } を特殊処理
    match callee {
        Some(Callee::Method { receiver: Some(r), .. }) => {
            used.push(*r);  // ← receiver を明示的に抽出
        }
        None => {
            used.push(*func);  // ← legacy path
        }
        _ => {}
    }
    used.extend(args.iter().copied());
    return used;  // ← Early return: inst_meta をバイパス
}
```

**特徴**:
- Call命令のみ methods.rs で完結
- CallLikeInst に callee フィールドがないため inst_meta をバイパス
- receiver を含む unified 経路と legacy 経路を統一的に処理

### 2. CallLikeInst の部分実装（instruction_kinds/mod.rs lines 718-803）

```rust
pub enum CallLikeInst {
    Call {
        dst: Option<ValueId>,
        func: ValueId,           // ← callee フィールドがない！
        args: Vec<ValueId>,
    },
    BoxCall { dst, box_val, args },
    PluginInvoke { dst, box_val, args },
    ExternCall { dst, args },
}

impl CallLikeInst {
    pub fn used(&self) -> Vec<ValueId> {
        match self {
            CallLikeInst::Call { func, args, .. } => {
                let mut v = Vec::new();
                if *func != ValueId::INVALID {  // ← INVALID チェック？
                    v.push(*func);
                }
                v.extend(args.iter().copied());
                v
            }
            // ... BoxCall, PluginInvoke, ExternCall ...
        }
    }
}
```

**問題**:
- CallLikeInst::Call は callee フィールドを持たない
- unified 経路（Callee::Method）のreceiver 処理が欠落
- ValueId::INVALID チェックは方針がはっきりしない

### 3. inst_meta の統合パス（instruction_kinds/mod.rs lines 275-352）

```rust
pub fn used_via_meta(i: &MirInstruction) -> Option<Vec<ValueId>> {
    // ... 多くの instruction の処理 ...
    
    if let Some(k) = CallLikeInst::from_mir(i) {
        return Some(k.used());  // ← CallLikeInst::used() を呼び出す
    }
    
    // ... rest ...
    None
}
```

**現状**:
- used_via_meta() は CallLikeInst::used() を呼び出す
- しかし methods.rs の used_values() は early return で inst_meta をバイパス
- **結果**: CallLikeInst::used() は実質的に使われていない（DCE等では methods.rs 経路）

## 問題分析

### P1: inst_meta層の役割あいまいさ

**症状**:
1. Call命令: methods.rs で early return（inst_meta 経由でない）
2. BoxCall/PluginInvoke: inst_meta 経由で CallLikeInst を使用
3. ExternCall: inst_meta 経由で CallLikeInst を使用

**根本原因**:
- inst_meta はPoC（Proof of Concept）段階の不完全な実装
- Call命令の callee フィールド対応が遅れている
- CallLikeInst に callee を追加できない設計的理由がない

### P2: CallLikeInst::Call の不完全な used()

**症状**:
```rust
// CallLikeInst::Call::used()
if *func != ValueId::INVALID {
    v.push(*func);
}
```

- INVALID チェックは unified 経路（callee: Some(_)）を前提？
- しかし CallLikeInst には callee フィールドがない
- どちらの経路か判定不可

**結論**: 設計的に矛盾している

### P3: methods.rs の early return がもたらす非対称性

**症状**:
- Call: methods.rs の manual match（callee 対応）
- BoxCall/PluginInvoke: inst_meta 経由（CallLikeInst 経由）

**問題**:
- 新しい Call の用途が追加されたとき、methods.rs と CallLikeInst の両方を修正しないといけない
- 意図的な分離か偶発的な分割か不明確

### P4: DCE の信頼性

**症状**（dce.rs lines 60-87）:
```rust
let mut used_values: HashSet<ValueId> = HashSet::new();

// Mark values used by side-effecting instructions and terminators
for instruction in &block.instructions {
    if !instruction.effects().is_pure() {
        for u in instruction.used_values() {  // ← used_values() を使用
            used_values.insert(u);
        }
    }
}

// Backward propagation
for instruction in &block.instructions {
    if used_values.contains(&dst) {
        for u in instruction.used_values() {
            if used_values.insert(u) { changed = true; }
        }
    }
}
```

**潜在的リスク**:
- Call命令で Callee::Method { receiver: Some(r), .. } の receiver が使用値に含まれるか？
- **YES** (methods.rs の early return で处理)
- **だが**、inst_meta::used_via_meta() から入った場合は？
- **NO** (CallLikeInst::Call は callee を知らない)

**結論**: 処理経路によって結果が異なる可能性

## 設計的な分岐点

### Option A: CallLikeInst に callee を追加

**メリット**:
- inst_meta を完全統一化できる
- methods.rs の early return を削除可能
- CallLikeInst::Call が unified 経路に対応

**デメリット**:
- CallLikeInst が大きくなる（Box-heavy）
- Clone/from_mir の複雑性増加
- Callee enum 自体が methods.rs との結合度を上げる

**実装量**: 中程度

### Option B: methods.rs を強化（inst_meta 側は軽量に保つ）

**メリット**:
- inst_meta をPoC段階のまま保つことができる
- methods.rs が「Call系の単一ソース・オブ・トゥルース」になる
- 将来 inst_meta を削除しても影響ない

**デメリット**:
- inst_meta の役割があいまい（不完全なPoC）
- ドキュメント化が重要になる

**実装量**: 少ない（コメント追加程度）

### Option C: inst_meta を CallLikeInst から分離（Method層として実装）

**メリット**:
- inst_meta と methods.rs の役割を完全に分離
- 将来の拡張に柔軟

**デメリット**:
- コード複製が増える
- 維持が大変

**実装量**: 高い

## パターンスキャン結果

### 他の同じ問題がある箇所

**1. CSE pass（passes/cse.rs lines 72-91）**:
```rust
fn instruction_key(i: &MirInstruction) -> String {
    match i {
        // ...
        MirInstruction::Call { func, args, .. } => {
            format!("call_{}_{}", func.as_u32(), args_str)
            // ← callee を無視している！
        }
        // ...
    }
}
```

**問題**: Call命令が Callee::Method { receiver: Some(r), .. } を持つ場合、receiver を含めずにキーを生成

**影響**: 異なる receiver を持つ同じメソッド呼び出しを「同一」と判定する可能性

**例**:
```mir
%r1 = call Method { receiver: Some(%obj1), ... } "upper"()
%r2 = call Method { receiver: Some(%obj2), ... } "upper"()
```
→ 同じキーになる → CSE で不正な最適化？

### 他に detected する可能性のある問題

**2. 新しい instruction_kinds 追加時**:
- inst_meta に追加する人は effects/dst/used の3つを実装する
- methods.rs との同期漏れリスク

**3. BoxCall/PluginInvoke の method_id**:
- instruction_kinds/mod.rs は method_id を無視している
- methods.rs は method_id を見ていない（フィールドがない）

## 改善提案（優先度順）

### 【優先度1】docs: inst_meta の役割と制約をドキュメント化

**内容**:
- inst_meta は PoC であること
- methods.rs が「単一ソース・オブ・トゥルース」であること
- CallLikeInst は callee フィールドがないこと（意図的）
- 将来統一する際の手順

**ファイル**: `docs/development/current/main/inst-meta-layer-design.md`

**実装量**: 1-2時間

**効果**: 中（開発者の混乱を減らす）

### 【優先度2】fix: CSE の instruction_key に callee を含める

**内容**:
```rust
fn instruction_key(i: &MirInstruction) -> String {
    match i {
        MirInstruction::Call { callee, args, .. } => {
            // callee をキーに含める
            let callee_str = format!("{:?}", callee);  // or structured key
            let args_str = args.iter()...
            format!("call_{}_{}_{}", callee_str, ...)
        }
        // ...
    }
}
```

**ファイル**: `src/mir/passes/cse.rs`

**実装量**: 1-2時間

**効果**: 高（CSEの正確性向上）

### 【優先度3】refactor: CallLikeInst に callee を追加（段階的）

**Phase 1**: CallLikeInst::Call に callee: Option<Callee> を追加
```rust
pub enum CallLikeInst {
    Call {
        dst: Option<ValueId>,
        func: ValueId,
        callee: Option<Callee>,  // 新規
        args: Vec<ValueId>,
    },
    // ...
}
```

**Phase 2**: CallLikeInst::used() を更新して receiver を処理
```rust
CallLikeInst::Call { func, callee, args, .. } => {
    let mut v = Vec::new();
    if let Some(Callee::Method { receiver: Some(r), .. }) = callee {
        v.push(*r);
    } else if *func != ValueId::INVALID {
        v.push(*func);
    }
    v.extend(args.iter().copied());
    v
}
```

**Phase 3**: methods.rs の early return を削除
```rust
// methods.rs: Remove early return for Call
// Let inst_meta::used_via_meta handle it
```

**ファイル**: 
- `src/mir/instruction_kinds/mod.rs`
- `src/mir/instruction/methods.rs`

**実装量**: 4-6時間

**効果**: 高（inst_meta 統一化）

### 【優先度4】test: 統合テスト（Callee::Method の receiver 判定）

**内容**: DCE/CSE で receiver を含む Call を正確に処理することを確認

**テストケース**:
```mir
// Case 1: Method call with receiver
%obj = new StringBox()
%r1 = call Method { receiver: Some(%obj), ... } "upper"()
// ↑ obj は使用値に含まれるべき

// Case 2: Different receivers
%s1 = new StringBox()
%s2 = new StringBox()
%r1 = call Method { receiver: Some(%s1), ... } "upper"()
%r2 = call Method { receiver: Some(%s2), ... } "upper"()
// ↑ CSE key は異なるべき
```

**ファイル**: 
- `src/mir/instruction_kinds/tests.rs` (新規)
- または既存テストに統合

**実装量**: 2-3時間

**効果**: 中（回帰テスト）

## 設計原則の推奨

### 「箱化」の視点から見た改善

**現状の問題**:
- inst_meta 層が「箱」として完全ではない
- methods.rs との責任の分離がはっきりしていない

**推奨アーキテクチャ**:
```
┌─────────────────────────────────┐
│      MIRInstruction             │
│  (Callee enum を含む確定形)      │
└─────────────────────────────────┘
            ↓
┌─────────────────────────────────┐
│      methods.rs                 │
│  effects() / dst_value()        │
│  used_values() (single source)  │
└─────────────────────────────────┘
            ↓
┌─────────────────────────────────┐
│  inst_meta (PoC: optional)      │
│  高速化用スキップ層              │
│  (検証/デバッグ用)               │
└─────────────────────────────────┘
```

**原則**:
1. methods.rs が SSOT（Single Source of Truth）
2. inst_meta は最適化用レイヤー（将来削除可）
3. CallLikeInst は methods.rs を完全ミラー

## まとめ

| 問題 | 影響 | 優先度 | 改善方針 |
|------|------|--------|---------|
| inst_meta 役割あいまい | 開発者混乱 | 1 | ドキュメント化 |
| CSE の callee 無視 | 最適化誤り可能 | 2 | fix CSE |
| CallLikeInst::Call 不完全 | 潜在バグ | 3 | callee 追加 |
| DCE 処理経路の非対称 | テスト困難 | 3 | 統合テスト追加 |
Status: Historical

