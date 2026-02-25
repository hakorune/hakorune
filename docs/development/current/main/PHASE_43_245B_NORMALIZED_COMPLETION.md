# Phase 43/245B: Normalized JoinIR Completion Summary

**Status**: ✅ COMPLETE (Phases 26-45)
**Date**: 2025-12-12
**Test Coverage**: 937/937 PASS

## Overview

The Normalized JoinIR infrastructure is **complete**. All Pattern1/Pattern2 loops plus JsonParser real-world loops (_skip_whitespace, _atoi, _parse_number) flow through:

1. **Structured JoinIR** (AST → JoinIR lowering)
2. **Normalized JoinIR** (Optional normalization layer)
3. **MIR** (via direct generation or legacy bridge)
4. **VM** (execution)

Both **canonical** (always Normalized→MIR direct) and **dev** (feature-gated Normalized dev) paths are verified.

---

## Architecture Components

### 1. JoinIR Mode System (Phase 45)

**`JoinIrMode` enum** (in `src/config/env/joinir_dev.rs`):
- `StructuredOnly` - Legacy path only
- `NormalizedDev` - Normalized dev (feature-gated)
- `NormalizedCanonical` - Always Normalized→MIR(direct) for P2-Core

**Routing**:
```rust
fn current_joinir_mode() -> JoinIrMode {
    if cfg!(feature = "normalized_dev") {
        NormalizedDev
    } else {
        StructuredOnly
    }
}
```

**Bridge routing** (canonical-first):
- P2-Core canonical shapes → **always** Normalized→MIR(direct) (mode ignored)
- Other shapes → follow `current_joinir_mode()`

### 2. Shape Capability System (Phase 44)

**`ShapeCapabilityKind` enum** (in `src/mir/join_ir/normalized/shape_guard.rs`):
- `P2CoreSimple` - Pattern1Mini, Pattern2Mini
- `P2CoreSkipWs` - skip_whitespace mini/real
- `P2CoreAtoi` - _atoi mini/real
- `P2MidParseNumber` - _parse_number real

**API**:
- `capability_for_shape(shape)` - Map shape to capability
- `is_canonical_shape(shape)` - Exact shape-level check
- `is_p2_core_capability(cap)` - Broad capability family check
- `is_supported_by_normalized(cap)` - Normalized dev support

**Canonical Set** (Phase 41):
- Pattern2Mini
- JsonparserSkipWsMini
- JsonparserSkipWsReal
- JsonparserAtoiMini

### 3. Carrier Infrastructure

**CarrierRole enum** (Phase 227):
- `LoopState` - Accumulator variables (sum, result) - exit PHI needed
- `ConditionOnly` - Boolean flags (is_digit_pos) - header PHI only, no exit PHI

**CarrierInit enum** (Phase 228):
- `DefaultZero` - Initialize to 0/false
- `ExplicitValue(ValueId)` - Explicit initial value

**Key components**:
- `CarrierInfo` - Tracks promoted variables with role/init
- `ConditionPromotionRequest` - Request structure for promotion
- `LoopBodyCondPromoter` - Two-tier orchestrator (A-3 Trim → A-4 DigitPos)

### 4. DigitPos Dual-Value (Phase 247-EX)

**Pattern**: One `indexOf()` → two carriers:
- `is_digit_pos: bool` (ConditionOnly) - for condition: `!is_digit_pos` (break)
- `digit_value: int` (LoopState) - for update: `result = result * 10 + digit_value`

**Components**:
- `DigitPosPromoter` - Detects indexOf pattern, generates dual carriers
- `DigitPosConditionNormalizer` - AST transform: `digit_pos < 0` → `!is_digit_pos`
- `UpdateEnv` - Context-aware resolution (promoted variables vs bool aliases)

### 5. NumberAccumulation (Phase 190)

**Pattern**: `result = result * base + digit`

**Detection**: Handles nested BinaryOp perfectly (no extensions needed for Phase 246-EX):
```rust
pub enum UpdateExpr {
    BinOp { op, lhs, rhs },
    // ...
}

pub enum NumberAccumulation {
    Base10 { digit_var: String },
    BaseN { base: i64, digit_var: String },
}
```

### 6. Step Scheduling (Phase 246-EX Part 2)

**Purpose**: Control evaluation order in Pattern2 loops to avoid "undefined variable" errors.

**Key insight**: Body-local initialization must occur BEFORE break condition check.

**Components** (extracted to `pattern2_step_schedule.rs`):
- `StepScheduleBox` - Determines step order (init → check → update)
- `emit_steps()` - Generates JoinIR in correct order

**Example**:
```
// Correct order:
1. Init ch (body-local)
2. Check break condition (uses ch)
3. Update loop state
```

### 7. Exit PHI & Jump Args (Phase 246-EX Part 2)

**Problem**: Exit PHI connections used wrong ValueIds (header PHI dst instead of exit PHI dst).

**Solution**:
1. Added `jump_args: Option<Vec<ValueId>>` to `BasicBlock` struct
2. `joinir_block_converter.rs` stores args during Jump instruction handling
3. `instruction_rewriter.rs` reads `jump_args`, remaps carriers correctly

**Files modified**:
- `src/mir/basic_block.rs`
- `src/mir/join_ir_vm_bridge/joinir_block_converter.rs`
- `src/mir/builder/control_flow/joinir/merge/instruction_rewriter.rs`

---

## Supported Loop Patterns

### Pattern1: Simple While (P1-Core)

**Example**: `loop_min_while.hako`
```
loop(i < n) {
    i = i + 1
}
```

**Status**: ✅ Normalized→MIR(direct) working

### Pattern2: Break (P2-Core/Mid)

#### P2-Core Simple
**Example**: `phase2_break_minimal.hako`
```
loop(i < n) {
    if (break_condition) { break }
    acc = acc + 1
    i = i + 1
}
```

#### P2-Core skip_whitespace
**Example**: JsonParser `_skip_whitespace`
```
loop(p < s.length()) {
    local ch = s.substring(p, p+1)
    if (ch != " " && ch != "\t" && ch != "\n") { break }
    p = p + 1
}
```

**Carriers**: `is_ch_match: bool` (Trim pattern, ConditionOnly)

#### P2-Core _atoi
**Example**: JsonParser `_atoi`
```
loop(p < s.length()) {
    local ch = s.substring(p, p+1)
    local digit_pos = "0123456789".indexOf(ch)
    if (digit_pos < 0) { break }
    result = result * 10 + digit_pos
    p = p + 1
}
```

**Carriers**:
- `is_digit_pos: bool` (ConditionOnly) - from `indexOf(ch) < 0`
- `digit_value: int` (LoopState) - from same `indexOf(ch)`

**Features**: DigitPos dual-value, NumberAccumulation (base 10)

#### P2-Mid _parse_number
**Example**: JsonParser `_parse_number`
```
loop(p < s.length()) {
    local ch = s.substring(p, p+1)
    local digit_pos = "0123456789".indexOf(ch)
    if (digit_pos < 0) { break }
    num_str = num_str + ch
    result = result * 10 + digit_pos
    p = p + 1
}
```

**Carriers**:
- `is_digit_pos: bool` (ConditionOnly)
- `digit_value: int` (LoopState)
- `num_str: StringBox` (LoopState) - String accumulation

**Status**: ✅ All working, Normalized→MIR(direct) verified

---

## Key Implementation Files

### JoinIR Frontend
- `src/mir/join_ir/lowering/loop_with_break_minimal.rs` - Pattern2 JoinIR generation
- `src/mir/join_ir/lowering/pattern2_step_schedule.rs` - Step scheduling (extracted Phase 246-EX)
- `src/mir/join_ir/lowering/carrier_info.rs` - CarrierRole/CarrierInit enums
- `src/mir/join_ir/lowering/digitpos_condition_normalizer.rs` - AST transform
- `src/mir/join_ir/lowering/update_env.rs` - Context-aware variable resolution

### Pattern Detection
- `src/mir/loop_pattern_detection/loop_body_cond_promoter.rs` - Two-tier orchestrator
- `src/mir/loop_pattern_detection/loop_body_digitpos_promoter.rs` - DigitPos pattern
- `src/mir/loop_pattern_detection/loop_update_analyzer.rs` - NumberAccumulation detection

### Normalized Infrastructure
- `src/mir/join_ir/normalized.rs` - Core normalized types
- `src/mir/join_ir/normalized/shape_guard.rs` - Shape + Capability system
- `src/mir/join_ir/normalized/dev_env.rs` - Dev environment
- `src/mir/join_ir/normalized/fixtures.rs` - Test fixtures
- `src/mir/join_ir_vm_bridge/normalized_bridge.rs` - Bridge
- `src/mir/join_ir_vm_bridge/normalized_bridge/direct.rs` - Direct MIR generation

### Mode & Config
- `src/config/env/joinir_dev.rs` - JoinIrMode enum, current_joinir_mode()
- `src/mir/join_ir_vm_bridge/bridge.rs` - Canonical-first routing
- `src/mir/join_ir_runner.rs` - Mode pattern matching

---

## Test Coverage

**Total**: 937/937 PASS

**Key test groups**:
- Pattern1/2 mini tests
- JsonParser smoke tests (skip_ws, atoi, parse_number)
- DigitPos dual-value tests (11 tests)
- Promoted variable resolution tests (3 tests)
- Normalized dev integration tests (+6 from baseline)

---

## Documentation Map（Normalized JoinIR ライン）

Normalized JoinIR 関連ドキュメントが増えてきたので、このフェーズを SSOT として、他ドキュメントの位置付けを整理しておくよ。

- **SSOT / 現役**
  - `joinir-architecture-overview.md`
    - JoinIR 全体のアーキテクチャと、Structured / Normalized / Bridge の役割をまとめたトップレベル設計図。
    - 3.1–3.3 で Normalized 層（JoinIR→JoinIR 正規化）のゴールと P1〜P4 との関係を定義。
  - `PHASE_43_245B_NORMALIZED_COMPLETION.md`（このファイル）
    - Phase 26–45 にかけて整備した Normalized インフラ（P1/P2 + JsonParser P2-Core/P2-Mid）全体の完了サマリ。
    - 実装ファイル・テスト・今後の Expansion を一覧できる SSOT。
  - `phase44-shape-capabilities-design.md`
    - ShapeCapabilityKind / capability_for_shape など、Normalized ルート選択の基盤設計。
  - `phase45-norm-mode-design.md`
    - JoinIrMode と current_joinir_mode() によるルーティング統一の設計メモ。

- **部分設計 / 詳細メモ（参照推奨）**
  - `phase245-jsonparser-parse-number-joinir-integration.md`
  - `phase245b-num_str-carrier-design.md`
  - `phase245c-function-param-capture-summary.md`
  - `phase246-jsonparser-atoi-joinir-integration.md`
  - `phase247-digitpos-dual-value-design.md`
    - JsonParser `_parse_number` / `_atoi` / DigitPos / num_str / Function param capture など、P2-Mid の詳細設計。
    - 必要に応じてここから構造や前提を掘る想定だよ。

- **Historical（実装済み・設計の足跡として残す）**
  - `phase26-HC-normalized-pattern1-bridge.md`
  - `phase26-HC-normalized-pattern1-bridge` から派生した Phase 26-H.* / 33-16 / 33-17 系の Normalized 初期メモ
    - Pattern1 単体の Normalized 試走とブリッジ検証の指示書。実装は本サマリと joinir-architecture-overview に吸収済み。
    - 新しく Normalized を読むときは、まず本ファイルと overview を見てから、必要に応じて Historical を辿る運用にする。

今後 Normalized の設計や実装を進めるときは：

1. 入口として `joinir-architecture-overview.md` とこの `PHASE_43_245B_NORMALIZED_COMPLETION.md` を読む。
2. JsonParser / DigitPos / num_str などテーマ別の詳細が必要になったら、対応する Phase 24x/25x ドキュメントを参照する。
3. 初期フェーズの試行錯誤に興味があれば、Historical とラベル付けされた Phase 26-H.* / 33-* メモを読む。

---

## Completed Phases

### Phase 26-42: Foundation
- Pattern1/2 infrastructure
- JoinIR → MIR bridge
- Carrier system basics
- ExitLine/Boundary architecture

### Phase 43: Normalized Pipeline
- Structured→Normalized→MIR(direct) pipeline
- JsonParser _parse_number fixture
- dev_env.rs, fixtures.rs created

### Phase 44: Shape Capabilities
- ShapeCapabilityKind enum (4 kinds)
- Capability-based filtering
- Extensible architecture

### Phase 45: Mode Unification
- JoinIrMode enum (3 modes)
- current_joinir_mode() centralized
- Canonical-first routing

### Phase 190: NumberAccumulation
- Detects `result = result * base + digit` patterns
- Handles nested BinaryOp

### Phase 227: CarrierRole
- LoopState vs ConditionOnly separation
- Exit PHI skips ConditionOnly

### Phase 228: CarrierInit
- Explicit initialization values
- Header PHI bool constant allocation

### Phase 246-EX: _atoi Integration
- FromHost carrier fixes (4-fix sequence)
- Exit PHI connection fixes
- Jump args preservation
- Step scheduling extraction

### Phase 247-EX: DigitPos Dual-Value
- One indexOf() → two carriers
- Context-aware resolution (UpdateEnv)
- AST normalization (ConditionNormalizer)

---

## Future Expansion

The infrastructure is ready for:

1. **Phase 46+**: Expand canonical set using capability-based filtering
2. **Carrier role analysis**: Enable `carrier_roles` field for automatic detection
3. **Method signature tracking**: Enable `method_calls` field for Box API requirements
4. **Pattern3/4 Normalized**: Apply infrastructure to if-sum/continue patterns
5. **Selfhost loops**: Complex loops from selfhost compiler

---

## References

- **Architecture**: `docs/development/current/main/joinir-architecture-overview.md`
- **Phase 44 Capabilities**: `docs/development/current/main/phase44-shape-capabilities-design.md`
- **Phase 45 Mode**: `docs/development/current/main/phase45-norm-mode-design.md`
- **Phase 247-EX DigitPos**: `docs/development/current/main/phase247-digitpos-dual-value-design.md`
- **Phase 246-EX _atoi**: `docs/development/current/main/phase246-jsonparser-atoi-joinir-integration.md`
