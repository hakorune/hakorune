# Normalized Expression Lowering (ExprLowererBox)

Status: SSOT  
Scope: Normalized shadow での expression lowering（pure のみ）を、パターン総当たりではなく AST walker で一般化する。  
Related:
- `docs/development/current/main/30-Backlog.md`
- `docs/development/current/main/phases/phase-138/README.md`
- `docs/development/current/main/phases/phase-140/README.md`
- `docs/development/current/main/phases/phase-141/README.md`
- `src/mir/control_tree/normalized_shadow/common/return_value_lowerer_box.rs`
- `src/mir/control_tree/normalized_shadow/common/expr_lowerer_box.rs`
- `src/mir/control_tree/normalized_shadow/common/expr_lowering_contract.rs`
- `src/mir/builder/control_flow/normalization/README.md`

---

## 目的（なぜ必要か）

Phase 131–138 で loop(true) 系の Normalized shadow を段階的に拡張したが、return/式の表現力を「形ごとのパターン追加」で増やし続けると、パターン爆発で持続不可能になる。

ここでの収束方針は次の通り:

- **制御フローの骨格（loop/if/post_k/continuation）は正規化（段階投入）で固める**
- **式（return value を含む）は一般化（AST walker）で受ける**

## 非目標

- Call/MethodCall の一般化（effects + typing を含む）を Phase 140 でやらない  
  → Phase 141+ に分離して段階投入する。
- PHI 生成を Normalized 側に戻すこと（PHI-free 維持。必要なら後段に押し出す）

## SSOT: NormalizedExprLowererBox

### 役割

`NormalizedExprLowererBox` は、pure expression を JoinIR（Normalized dialect）へ lowering する SSOT になる。

- 入力: AST（`AstNodeHandle` / `ASTNode`）+ env（`BTreeMap<String, ValueId>`）+ 出力先 `Vec<JoinInst>` + `next_value_id`
- 出力: `Result<Option<ValueId>, String>`
  - `Ok(Some(vid))`: lowering 成功（`vid` が式の値）
  - `Ok(None)`: out-of-scope（既存経路へフォールバック、既定挙動不変）
  - `Err(...)`: 内部不整合のみ（strict では fail-fast）

### Pure Expression の定義（Phase 140 の範囲）

副作用がなく、Control-flow を内包しない式のみ。

- `Variable`
- `Literal`（Integer/Bool を主対象。String/Float は必要になったら追加）
- `UnaryOp`（`not`, `-`）
- `BinaryOp`（`+ - * /`）
- `Compare`（`== < <= > >=` 等の比較系）

Call/MethodCall は対象外（Phase 141+）。

## ReturnValueLowererBox の縮退方針

Phase 138 時点では `ReturnValueLowererBox` が return の形（var/int/add）を直接扱っている。

Phase 140 以降は、`ReturnValueLowererBox` を「return 構文（None/Some）を処理する薄い箱」に縮退し、実体は `NormalizedExprLowererBox` に委譲する方針とする。

実装 SSOT:
- `src/mir/control_tree/normalized_shadow/common/expr_lowerer_box.rs`
- `src/mir/control_tree/normalized_shadow/common/return_value_lowerer_box.rs`

## 収束ロードマップ

- Phase 139: if-only `post_if_post_k.rs` の return を `ReturnValueLowererBox` へ統一（出口 SSOT 完成）
- Phase 140: `NormalizedExprLowererBox` 初版（pure のみ）導入、return は ExprLowerer 経由へ寄せる
- Phase 141 P0: impure 拡張点（contract）を SSOT 化（Call/MethodCall はまだ out-of-scope）
- Phase 141 P1: “既知 intrinsic だけ” を許可して段階投入（それ以外は out-of-scope のまま）
- Phase 141 P2+: Call/MethodCall を effects + typing で分離しつつ段階投入

## パターン爆発の分解（実務）

パターン爆発には “別種” が混ざるため、対策も分ける。

- **式（return 値 / 右辺）の爆発**: `return 7`, `return x+2`, `return x+y`, `return (x+2)+3` …
  - 対策: ExprLowerer（AST walker）の対応ノードを増やす 1 箇所に収束させる。
- **制御（loop/if/break/continue）の爆発**: `loop(true){break}` と `loop(true){if(...) break}` 等
  - 対策: ControlTree/StepTree の語彙（If/Loop/Exit）を増やし、同じ lowering に通す。
  - “新パターン” ではなく “語彙追加” にするのが収束の鍵。

## 正規化単位（NormalizationPlan の粒度）

現状は `NormalizationPlanBox` が block suffix を単位として正規化（loop + post assigns + return など）を扱う。

将来の収束案として、正規化単位を **statement（loop 1 個）** に寄せ、post/return は通常の lowering に任せる選択肢がある。

- 利点: suffix パターン増殖の圧力が下がる
- 注意: 既存の Phase 132–138 の成果（post_k/DirectValue/exit reconnect）と整合する形で段階移行すること
- 既定方針: Phase 140（pure expr）で ExprLowerer を確立した後に再検討する（Phase 141+ の設計判断）

## 受け入れ基準（設計）

- "形ごとの if 分岐追加" ではなく、AST walker の対応ノード追加が主たる拡張点になっている
- out-of-scope は `Ok(None)` でフォールバックし、既定挙動不変を維持する
- JoinIR merge は by-name 推測をせず、boundary/contract を SSOT として従う

---

## Available Inputs SSOT (Phase 141 P1.5)

### 問題（Problem）

Suffix normalization couldn't access prefix-built local variables (e.g., `s = "abc"`).

**Bug scenario**:
```hako
s = "abc"          // Prefix: builder.variable_map["s"] = ValueId(42)
flag = 1
if flag == 1 {     // Suffix normalization starts
    s = s
}
return s.length()  // ERROR: 's' not in available_inputs!
```

**Root cause**: `AvailableInputsCollectorBox::collect(builder, None)` only collected function params and CapturedEnv, missing prefix-built local variables.

### 解決策（Solution）

3-source merge with priority order:

1. **Function params** (highest priority - from `scope_ctx.function_param_names`)
2. **Prefix variables** (medium priority - from `builder.variable_map`)
3. **CapturedEnv** (lowest priority - from closure captures)

### Contract

```rust
AvailableInputsCollectorBox::collect(
    builder: &MirBuilder,
    captured_env: Option<&CapturedEnv>,
    prefix_variables: Option<&BTreeMap<String, ValueId>>, // NEW: Phase 141 P1.5
) -> BTreeMap<String, ValueId>
```

### Usage

```rust
// At call site (e.g., stmts.rs)
let prefix_variables = Some(&self.variable_ctx.variable_map);

// Execute normalization
NormalizationExecuteBox::execute(
    builder,
    &plan,
    remaining,
    func_name,
    debug,
    prefix_variables,  // Pass through
)?;

// Inside execute_loop_only
let available_inputs = AvailableInputsCollectorBox::collect(
    builder,
    None,
    prefix_variables,  // Merge prefix variables
);
```

### Edge Cases

- **Variable shadowing**: Function params override prefix variables
- **Empty prefix**: `prefix_variables = None` (e.g., suffix with no prefix execution)
- **Mutation**: `variable_map` is cloned/borrowed, not moved

### SSOT Location

- **Implementation**: `src/mir/control_tree/normalized_shadow/available_inputs_collector.rs`
- **Call sites**:
  - `src/mir/builder/control_flow/normalization/execute_box.rs` (`execute_loop_only`)
  - active module surface `crate::mir::builder::control_flow::joinir::route_entry::policies::normalized_shadow_suffix_router_box`
    (historical path notes may still mention `src/mir/builder/control_flow/joinir/patterns/policies/`; see `route-physical-path-legacy-lane-ssot.md`)
  - `src/mir/builder/stmts.rs` (build_block suffix router call)

---

## Known Intrinsic SSOT (Phase 141 P1.5)

### 目的

“既知 intrinsic だけ” の段階投入を、by-name の直書き増殖ではなく SSOT registry に収束させる。

### SSOT

- Registry: `src/mir/control_tree/normalized_shadow/common/known_intrinsics.rs`
  - `KnownIntrinsicRegistryBox::{lookup,get_spec}`
- Marker enum: `src/mir/control_tree/normalized_shadow/common/expr_lowering_contract.rs`
  - `KnownIntrinsic` は “識別子” のみ（metadata は registry に集約）

### Diagnostics

- `OutOfScopeReason::IntrinsicNotWhitelisted` を追加し、`MethodCall` の out-of-scope 理由を精密化する。

---

## ValueId Space Contract (Phase 143 fix)

### 問題

Normalized shadow modules allocate env params using `alloc_value_id()` starting from 1, but JoinValueSpace contract requires Param region (100-999).

**Wrong** (before fix):
```rust
let mut next_value_id: u32 = 1;
let params = NormalizedHelperBox::alloc_env_params(&fields, &mut next_value_id);
// → [ValueId(1), ValueId(2), ...] — PHI Reserved region!
```

**Correct** (after fix):
```rust
let (params, mut next_local) = NormalizedHelperBox::alloc_env_params_param_region(&fields);
// → [ValueId(100), ValueId(101), ...] — Param region ✅
```

### Contract (Phase 201 SSOT)

```
 0          100        1000                     u32::MAX
 ├──────────┼──────────┼──────────────────────────┤
 │  PHI     │  Param   │       Local             │
 │  Reserved│  Region  │       Region            │
 └──────────┴──────────┴──────────────────────────┘
```

| Region | Range | Purpose |
|--------|-------|---------|
| PHI Reserved | 0-99 | Loop header PHI dst |
| **Param** | **100-999** | **env params (flag, counter, etc.)** |
| Local | 1000+ | Const, BinOp, condition results |

### SSOT

- Constants: `src/mir/join_ir/lowering/join_value_space.rs`
  - `PARAM_MIN = 100`
  - `LOCAL_MIN = 1000`
- API: `NormalizedHelperBox::alloc_env_params_param_region()`
  - Location: `src/mir/control_tree/normalized_shadow/common/normalized_helpers.rs`
  - Returns: `(Vec<ValueId>, u32)` — (params in 100+ range, next_local starting at 1000)

### Affected Files

- `loop_true_if_break_continue.rs`
- `loop_true_break_once.rs`
- `if_as_last_join_k.rs`
- `post_if_post_k.rs`

All normalized shadow modules must use `alloc_env_params_param_region()` instead of `alloc_env_params()` to ensure env params are in the correct region.
