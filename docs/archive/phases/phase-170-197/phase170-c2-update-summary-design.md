# Phase 170-C-2: LoopUpdateSummaryBox 設計

## 概要

CaseALoweringShape の検出精度を向上させるため、ループの更新パターンを解析する専用 Box を導入する。

## 背景

### 現状 (Phase 170-C-1)
- `detect_with_carrier_name()` で carrier 名ヒューリスティックを使用
- `i`, `e`, `idx` → StringExamination
- その他 → ArrayAccumulation

### 問題点
- 名前だけでは不正確（`sum` という名前でも CounterLike かもしれない）
- 実際の更新式を見ていない

## 設計

### 1. UpdateKind 列挙型

```rust
/// キャリア変数の更新パターン
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateKind {
    /// カウンタ系: i = i + 1, i = i - 1, i += 1
    /// 典型的な skip/trim パターン
    CounterLike,

    /// 蓄積系: result = result + x, arr.push(x), list.append(x)
    /// 典型的な collect/filter パターン
    AccumulationLike,

    /// 判定不能
    Other,
}
```

### 2. CarrierUpdateInfo 構造体

```rust
/// 単一キャリアの更新情報
#[derive(Debug, Clone)]
pub struct CarrierUpdateInfo {
    /// キャリア変数名
    pub name: String,

    /// 更新パターン
    pub kind: UpdateKind,
}
```

### 3. LoopUpdateSummary 構造体

```rust
/// ループ全体の更新サマリ
#[derive(Debug, Clone, Default)]
pub struct LoopUpdateSummary {
    /// 各キャリアの更新情報
    pub carriers: Vec<CarrierUpdateInfo>,
}

impl LoopUpdateSummary {
    /// 単一 CounterLike キャリアを持つか
    pub fn has_single_counter(&self) -> bool {
        self.carriers.len() == 1
            && self.carriers[0].kind == UpdateKind::CounterLike
    }

    /// AccumulationLike キャリアを含むか
    pub fn has_accumulation(&self) -> bool {
        self.carriers.iter().any(|c| c.kind == UpdateKind::AccumulationLike)
    }
}
```

### 4. analyze_loop_updates_ast 関数

```rust
/// AST からループ更新パターンを解析
///
/// # Phase 170-C-2 暫定実装
/// - 名前ヒューリスティックを内部で使用
/// - 将来的に AST 解析に置き換え
pub fn analyze_loop_updates_ast(
    _condition: &ASTNode,
    _body: &[ASTNode],
    carrier_names: &[String],
) -> LoopUpdateSummary {
    let carriers = carrier_names
        .iter()
        .map(|name| {
            let kind = if is_typical_index_name(name) {
                UpdateKind::CounterLike
            } else {
                UpdateKind::AccumulationLike
            };
            CarrierUpdateInfo {
                name: name.clone(),
                kind,
            }
        })
        .collect();

    LoopUpdateSummary { carriers }
}
```

## LoopFeatures への統合

```rust
pub struct LoopFeatures {
    pub has_break: bool,
    pub has_continue: bool,
    pub has_if: bool,
    pub has_if_else_phi: bool,
    pub carrier_count: usize,
    pub break_count: usize,
    pub continue_count: usize,

    // Phase 170-C-2 追加
    pub update_summary: Option<LoopUpdateSummary>,
}
```

## CaseALoweringShape での利用

```rust
pub fn detect_from_features(
    features: &LoopFeatures,
    carrier_count: usize,
    has_progress_carrier: bool,
) -> Self {
    // ... 既存チェック ...

    // Phase 170-C-2: UpdateSummary を優先
    if let Some(ref summary) = features.update_summary {
        if summary.has_single_counter() {
            return CaseALoweringShape::StringExamination;
        }
        if summary.has_accumulation() {
            return CaseALoweringShape::ArrayAccumulation;
        }
    }

    // フォールバック: carrier 数のみ
    match carrier_count {
        1 => CaseALoweringShape::Generic,
        2.. => CaseALoweringShape::IterationWithAccumulation,
        _ => CaseALoweringShape::NotCaseA,
    }
}
```

## ファイル配置

```
src/mir/join_ir/lowering/
├── loop_update_summary.rs  # 新規: UpdateKind, LoopUpdateSummary
├── loop_to_join.rs         # 更新: analyze_loop_updates_ast 呼び出し
└── loop_scope_shape/
    └── case_a_lowering_shape.rs  # 更新: UpdateSummary 参照
```

## 移行計画

### Phase 170-C-2a (今回)
- `loop_update_summary.rs` 骨格作成
- 名前ヒューリスティックを内部に閉じ込め
- LoopFeatures への統合は保留

### Phase 170-C-2b (将来)
- AST 解析で実際の更新式を判定
- `i = i + 1` → CounterLike
- `result.push(x)` → AccumulationLike

### Phase 170-C-3 (将来)
- MIR ベース解析（AST が使えない場合の代替）
- BinOp 命令パターンマッチング

## 利点

1. **関心の分離**: 更新パターン解析が独立した Box に
2. **差し替え容易**: 名前 → AST → MIR と段階的に精度向上可能
3. **テスト容易**: LoopUpdateSummary を直接テスト可能
4. **後方互換**: LoopFeatures.update_summary は Option なので影響最小
Status: Historical
