# JoinIR中期リファクタリング：優先3項目の分析結果

**日付**: 2025-12-07
**フェーズ**: Phase 33+ (JoinIR Modularization)

---

## Priority 1: Pattern 4 二重実装の統合分析

### 発見: 分離構造（A→B呼び出し）

**ファイルA**: `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs` (361行)
- **役割**: LoopForm検出 → Pattern 4判定 → JoinIR lowering wrapper
- **主要関数**: `cf_loop_pattern4_with_continue()` (120-361行)
  - CarrierInfo構築
  - LoopUpdateAnalyzer実行
  - **B を呼び出し**: `lower_loop_with_continue_minimal()`
  - JoinInlineBoundary構築
  - MIRマージ

**ファイルB**: `src/mir/join_ir/lowering/loop_with_continue_minimal.rs` (506行)
- **役割**: Pattern 4の純粋JoinIR生成（ローカルValueId空間）
- **主要関数**: `lower_loop_with_continue_minimal()` (104-506行)
  - JoinModule生成（main/loop_step/k_exit）
  - Select命令による continue セマンティクス
  - ExitMeta構築

### 責務分離構造の確認

```rust
// A: パターン検出とホスト統合 (pattern4_with_continue.rs)
pub fn lower() -> Result<Option<ValueId>, String> {
    builder.cf_loop_pattern4_with_continue(condition, body, func_name, debug)
}

impl MirBuilder {
    fn cf_loop_pattern4_with_continue() {
        // Step 1: CarrierInfo構築
        let carrier_info = CommonPatternInitializer::initialize_pattern(...)?;

        // Step 2: LoopUpdateAnalyzer実行
        let carrier_updates = LoopUpdateAnalyzer::analyze_carrier_updates(...);

        // Step 3: B を呼び出し（純粋JoinIR生成）
        let (join_module, exit_meta) = lower_loop_with_continue_minimal(
            scope, condition, self, &carrier_info, &carrier_updates
        )?;

        // Step 4: JoinInlineBoundary構築
        let boundary = JoinInlineBoundary::new_with_exit_bindings(...);

        // Step 5: MIRマージ
        JoinIRConversionPipeline::execute(...)?;
    }
}

// B: 純粋JoinIR生成 (loop_with_continue_minimal.rs)
pub fn lower_loop_with_continue_minimal(
    scope: LoopScopeShape,
    condition: &ASTNode,
    builder: &mut MirBuilder,
    carrier_info: &CarrierInfo,
    carrier_updates: &HashMap<String, UpdateExpr>,
) -> Result<(JoinModule, ExitMeta), String> {
    // ローカルValueId空間でJoinIR生成
    // main/loop_step/k_exit 3関数生成
    // Select命令でcontinueセマンティクス実装
}
```

### 結論: 責務分離は正しい設計

- **A (pattern4_with_continue.rs)**: ホスト統合レイヤー
  - MirBuilder依存
  - variable_map管理
  - CarrierInfo/LoopUpdateAnalyzer統合

- **B (loop_with_continue_minimal.rs)**: 純粋JoinIR生成レイヤー
  - ホストValueId無関係
  - 再利用可能な変換ロジック
  - テスト容易性

**推奨アクション**: **現状維持**（統合不要）

**理由**:
1. 責務分離が明確（ホスト統合 vs. 純粋変換）
2. Bは他のパターンでも再利用可能（将来拡張性）
3. テスト容易性（Bは独立してテスト可能）
4. 統合すると461行のヘルパーがAに混入（可読性低下）

**削減見込み**: **0行**（統合しない）

---

## Priority 2: LoopToJoin 構造の箱化

### 現状分析

**ファイル**: `src/mir/join_ir/lowering/loop_to_join.rs` (590行)

**構造**:
```rust
pub struct LoopToJoinLowerer {
    debug: bool,
}

impl LoopToJoinLowerer {
    // 1. サポート判定 (180行)
    fn is_supported_case_a_loop_view(...) -> bool { ... }

    // 2. lowering調整 (343行)
    fn lower_with_scope(...) -> Option<JoinModule> {
        // Pattern 1判定
        // CaseALoweringShape検出
        // Shape別ディスパッチ
        // 名前ベースフォールバック
    }

    // 3. メインエントリ (89行)
    pub fn lower(...) -> Option<JoinModule> {
        // MirQuery構築
        // LoopFormIntake構築
        // LoopScopeShape構築
        // サポート判定
        // lower_with_scope呼び出し
    }
}
```

### 責務混在の問題点

1. **LoopPatternValidator**: サポート判定ロジック (180行)
   - ExitAnalysis検証
   - Header successor数チェック
   - Progress carrier検証

2. **LoopViewBuilder**: Shape検出とディスパッチ (343行)
   - Pattern 1検出
   - CaseALoweringShape検出
   - 各lowerer呼び出し

3. **LoopToJoinLowerer**: コーディネーター (89行)
   - MirQuery/Intake/Scope構築
   - Validator/Builder呼び出し

### 推奨分割

#### 新構造:

```
src/mir/join_ir/lowering/
├── loop_pattern_validator.rs (新規作成)
│   └── LoopPatternValidator Box
│       ├── validate_exit_structure()
│       ├── validate_header_structure()
│       ├── validate_progress_carrier()
│       └── is_supported_case_a() (統合版)
│
├── loop_view_builder.rs (新規作成)
│   └── LoopViewBuilder Box
│       ├── detect_pattern1()
│       ├── detect_case_a_shape()
│       ├── dispatch_shape_lowering()
│       └── dispatch_name_fallback()
│
└── loop_to_join.rs (140行に削減)
    └── LoopToJoinLowerer (coordinator のみ)
        ├── build_query()
        ├── build_intake()
        ├── build_scope()
        └── lower() (orchestration)
```

#### 実装順序:

1. **Phase 2-A**: LoopPatternValidator抽出
   - is_supported_case_a_loop_view() → validate()メソッドに
   - has_safe_progress() をメソッド化
   - 180行を新ファイルに移動

2. **Phase 2-B**: LoopViewBuilder抽出
   - lower_with_scope() → build()メソッドに
   - Pattern検出ロジックを分離
   - 343行を新ファイルに移動

3. **Phase 2-C**: LoopToJoinLowerer簡略化
   - Validator/Builder呼び出しに委譲
   - 590→140行 (76%削減)

**削減見込み**: 150-200行（重複削減・責務明確化）

---

## Priority 3: Generic Case-A 統一

### 現状分析

**ディレクトリ**: `src/mir/join_ir/lowering/generic_case_a/` (7ファイル)

**構成**:
```
generic_case_a/
├── mod.rs (93行) - entry dispatcher (Phase 192完了)
├── skip_ws.rs (258行) - whitespace skipping
├── trim.rs (537行) - string trimming (最大)
├── append_defs.rs (202行) - array concatenation
├── stage1_using_resolver.rs (228行) - namespace resolution
├── entry_builder.rs (165行) - helper (共通初期化)
└── whitespace_check.rs (151行) - helper (共通検証)

合計: 1,634行 (7モジュール)
```

### 重複コードパターンの分析

各lowerer関数の共通構造:
```rust
pub fn lower_case_a_PATTERN_with_scope(
    scope: LoopScopeShape
) -> Option<JoinModule> {
    // 1. Carrier/Pinned検証 (各20-30行、30%重複)
    let progress = scope.carriers.iter().find(...)?;
    let fixed = scope.pinned.iter().find(...)?;

    // 2. JoinModule骨組み (各50-80行、30%重複)
    let mut module = JoinModule::new();
    let main_id = JoinFuncId::new(0);
    let loop_step_id = JoinFuncId::new(1);
    let k_exit_id = JoinFuncId::new(2);

    // 3. ValueId割り当て (各30-50行、30%重複)
    let mut value_counter = 0u32;
    let progress_init = alloc_value();
    let fixed_init = alloc_value();

    // 4. パターン固有ロジック (各100-400行、0%重複)
    // trim: substring/equality checks
    // skip_ws: whitespace classification
    // stage1: using resolver

    // 5. main/loop_step/k_exit生成 (各50-80行、30%重複)
    let main_func = build_main(...);
    let loop_step_func = build_loop_step(...);
    let k_exit_func = build_k_exit(...);

    Some(module)
}
```

### Trait設計

#### Phase 3-A: CaseASpecialization Trait定義

```rust
// 新ファイル: generic_case_a/case_a_trait.rs

pub trait CaseASpecialization {
    /// パターン名
    fn get_name(&self) -> &'static str;

    /// Carrier/Pinned検証
    fn validate_scope(&self, scope: &LoopScopeShape) -> Option<ScopeBinding>;

    /// ループボディ命令生成（パターン固有）
    fn build_body_instructions(
        &self,
        binding: &ScopeBinding,
        alloc: &mut dyn FnMut() -> ValueId,
    ) -> Vec<JoinInst>;

    /// PHI更新式生成（パターン固有）
    fn build_phi_updates(
        &self,
        binding: &ScopeBinding,
    ) -> Vec<(ValueId, ValueId)>;
}

pub struct ScopeBinding {
    pub progress: (String, ValueId),
    pub fixed: Option<(String, ValueId)>,
    pub extra: Vec<(String, ValueId)>,
}
```

#### Phase 3-B: 共通ボイラープレート抽出

```rust
// 新ファイル: generic_case_a/unified_lowering.rs

pub fn lower_generic_case_a<T: CaseASpecialization>(
    spec: T,
    scope: LoopScopeShape,
) -> Option<JoinModule> {
    // 1. 検証（Trait委譲）
    let binding = spec.validate_scope(&scope)?;

    // 2. JoinModule骨組み（共通）
    let mut module = JoinModule::new();
    let (main_id, loop_step_id, k_exit_id) = allocate_func_ids();

    // 3. ValueId割り当て（共通）
    let mut value_counter = 0u32;
    let mut alloc_value = || { ... };

    // 4. main関数生成（共通）
    let main_func = build_main_func(main_id, &binding, &mut alloc_value);

    // 5. loop_step関数生成（パターン固有部分を委譲）
    let body_insts = spec.build_body_instructions(&binding, &mut alloc_value);
    let phi_updates = spec.build_phi_updates(&binding);
    let loop_step_func = build_loop_step_func(
        loop_step_id, k_exit_id, &binding, body_insts, phi_updates
    );

    // 6. k_exit関数生成（共通）
    let k_exit_func = build_k_exit_func(k_exit_id, &binding);

    module.add_function(main_func);
    module.add_function(loop_step_func);
    module.add_function(k_exit_func);
    module.entry = Some(main_id);

    Some(module)
}
```

#### Phase 3-C: Trait実装（各パターン）

```rust
// skip_ws.rs
struct SkipWsCaseA;

impl CaseASpecialization for SkipWsCaseA {
    fn get_name(&self) -> &'static str { "skip_ws" }

    fn validate_scope(&self, scope: &LoopScopeShape) -> Option<ScopeBinding> {
        let progress = scope.carriers.iter().find(|n| n == "i")?;
        let fixed = scope.pinned.iter().find(|n| n == "s")?;
        Some(ScopeBinding { progress, fixed, extra: vec![] })
    }

    fn build_body_instructions(
        &self,
        binding: &ScopeBinding,
        alloc: &mut dyn FnMut() -> ValueId,
    ) -> Vec<JoinInst> {
        // skip_ws固有のwhitespace判定ロジック
        // c == ' ' || c == '\t' など
        vec![...]
    }

    fn build_phi_updates(&self, binding: &ScopeBinding) -> Vec<(ValueId, ValueId)> {
        // i_next = i + 1
        vec![(binding.progress.1, i_next)]
    }
}

// 公開関数（既存互換性）
pub fn lower_case_a_skip_ws_with_scope(scope: LoopScopeShape) -> Option<JoinModule> {
    lower_generic_case_a(SkipWsCaseA, scope)
}
```

### 実装順序

1. **Phase 3-A**: Trait定義 + ScopeBinding構造体
2. **Phase 3-B**: unified_lowering.rs 実装
3. **Phase 3-C**: 各パターンのTrait実装（順次移行）
   - skip_ws (最小、258行)
   - append_defs (202行)
   - stage1 (228行)
   - trim (最大、537行)

**削減見込み**: 200-300行（30%共通化）

---

## 実装優先順位と期待効果

| Priority | タスク | 削減見込み | リスク | 価値 |
|----------|-------|-----------|-------|-----|
| 1 | Pattern 4統合 | 0行 | 低 | 低（統合不要と判明） |
| 2 | LoopToJoin箱化 | 150-200行 | 中 | 高（責務分離・保守性向上） |
| 3 | CaseA統一 | 200-300行 | 高 | 高（共通化・拡張性向上） |

**合計削減見込み**: 350-500行 (約22-31%削減)

---

## 次のアクション

### Priority 1: ✅ 分析完了（統合不要）
- 責務分離が正しいと確認
- 現状維持を推奨

### Priority 2: LoopToJoin箱化実装
1. Phase 2-A: LoopPatternValidator抽出 (180行)
2. Phase 2-B: LoopViewBuilder抽出 (343行)
3. Phase 2-C: LoopToJoinLowerer簡略化 (140行に)

### Priority 3: CaseA統一実装
1. Phase 3-A: Trait設計
2. Phase 3-B: unified_lowering実装
3. Phase 3-C: 各パターン移行（4モジュール）

---

**作成者**: Claude Code (Sonnet 4.5)
**レビュー**: 要ユーザー承認
