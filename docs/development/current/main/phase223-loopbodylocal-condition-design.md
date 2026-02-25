# Phase 223-2: LoopBodyLocal Condition Promotion Design

---
**Phase 26-45 Completion**: このフェーズで設計した機能は Phase 43/245B で実装完了。最終状態は [PHASE_43_245B_NORMALIZED_COMPLETION.md](./PHASE_43_245B_NORMALIZED_COMPLETION.md) を参照。
---

## Purpose

This document defines the **API-level design** for promoting LoopBodyLocal variables used in loop conditions (header, break, continue) to bool carriers. This enables Pattern2/Pattern4 to handle loops like JsonParser's `_skip_whitespace` which currently Fail-Fast.

**Scope**: Design-only phase. No implementation. Pure API definitions and integration strategy.

**P0 Target**: Category A-3 (_skip_whitespace pattern with continue)

---

## 1. Problem Statement

### 1.1 Current State

**Fail-Fast Location**: Pattern2/Pattern4 lowerers detect LoopBodyLocal in conditions and reject:

```rust
// src/mir/join_ir/lowering/loop_with_break_minimal.rs
if loop_cond_scope.has_loop_body_local() {
    let body_local_names = extract_body_local_names(&loop_cond_scope.vars);
    return Err(format_unsupported_condition_error("pattern2", &body_local_names));
}
```

**Example Pattern (Category A-3)**:
```hako
loop(i < n) {
    local ch = src.substring(i, i + 1)
    if ch == " " || ch == "\t" || ch == "\n" || ch == "\r" {
        i = i + 1
        continue
    }
    break
}
```

**Current Behavior**: Fail-Fast with `[joinir/freeze] Pattern4: unsupported condition variables (LoopBodyLocal): ch`

**Desired Behavior**: Promote `ch` to bool carrier `is_whitespace`, continue with Pattern4 lowering

---

### 1.2 Existing Infrastructure

| Box | Responsibility | Location |
|-----|---------------|----------|
| **LoopConditionScopeBox** | Classify variables as LoopParam/OuterLocal/LoopBodyLocal | `loop_condition_scope.rs` |
| **LoopBodyCarrierPromoter** | Promote LoopBodyLocal to bool carrier (Trim/P5) | `loop_body_carrier_promoter.rs` |
| **TrimLoopLowerer** | Orchestrate Trim pattern detection + lowering | `trim_loop_lowering.rs` |

**Current Use Case**: Pattern2 (with break) + Trim pattern
- Detects `local ch = s.substring(...)` + `ch == " " || ...`
- Promotes `ch` to `is_whitespace` carrier
- Replaces break condition with `!is_whitespace`

**Gap**: Pattern4 (with continue) does not integrate with TrimLoopLowerer

---

## 2. Existing Box Roles

### 2.1 LoopConditionScopeBox

**File**: `src/mir/loop_pattern_detection/loop_condition_scope.rs`

**Responsibility**:
- Analyze condition AST (header, break, continue)
- Classify each variable by scope:
  - `LoopParam`: Loop parameter (e.g., `i` in `loop(i < len)`)
  - `OuterLocal`: Pre-existing variable from outer scope
  - `LoopBodyLocal`: Variable defined inside loop body

**API**:
```rust
pub struct LoopConditionScope {
    pub vars: Vec<CondVarInfo>,
}

impl LoopConditionScope {
    pub fn has_loop_body_local(&self) -> bool;
    pub fn var_names(&self) -> HashSet<String>;
}
```

**Usage in Pattern2/4**:
```rust
let loop_cond_scope = LoopConditionScopeBox::analyze(break_cond, &scope);
if loop_cond_scope.has_loop_body_local() {
    // Current: Fail-Fast
    // Future: Try promotion
}
```

---

### 2.2 LoopBodyCarrierPromoter

**File**: `src/mir/loop_pattern_detection/loop_body_carrier_promoter.rs`

**Responsibility**:
- Detect Trim-like patterns (substring + equality chain)
- Generate `TrimPatternInfo` with carrier name and initialization logic
- Convert to `CarrierInfo` with placeholder ValueId(0)

**API**:
```rust
pub struct PromotionRequest<'a> {
    pub scope: &'a LoopScopeShape,
    pub cond_scope: &'a LoopConditionScope,
    pub break_cond: Option<&'a ASTNode>,
    pub loop_body: &'a [ASTNode],
}

pub enum PromotionResult {
    Promoted { trim_info: TrimPatternInfo },
    CannotPromote { reason: String, vars: Vec<String> },
}

impl LoopBodyCarrierPromoter {
    pub fn try_promote(req: PromotionRequest) -> PromotionResult;
}
```

**Current Limitation**: Designed for Pattern2 (break), not Pattern4 (continue)

---

### 2.3 TrimLoopLowerer

**File**: `src/mir/builder/control_flow/joinir/patterns/trim_loop_lowering.rs`

**Responsibility**:
- Orchestrate Trim pattern detection (via LoopBodyCarrierPromoter)
- Filter condition-only LoopBodyLocal (Phase 183)
- Generate carrier initialization code (substring extraction + bool expression)
- Build ConditionBinding for promoted carrier

**API**:
```rust
pub struct TrimLoweringResult {
    pub condition: ASTNode,           // Replaced break condition
    pub carrier_info: CarrierInfo,    // Updated with promoted carrier
    pub condition_bindings: Vec<ConditionBinding>,
}

impl TrimLoopLowerer {
    pub fn try_lower_trim_like_loop(
        builder: &mut MirBuilder,
        scope: &LoopScopeShape,
        loop_cond: &ASTNode,
        break_cond: &ASTNode,
        body: &[ASTNode],
        loop_var_name: &str,
        carrier_info: CarrierInfo,
        alloc_join_value: &mut dyn FnMut() -> ValueId,
    ) -> Result<Option<TrimLoweringResult>, String>;
}
```

**Current Integration**: Pattern2 only
**Gap**: Pattern4 does not call TrimLoopLowerer

---

## 3. New API Design: Condition Promotion for Pattern4

### 3.1 Design Philosophy

**Principle**: Minimal new code. Reuse existing Trim infrastructure with thin adapter.

**Options Considered**:

| Option | Pros | Cons | Decision |
|--------|------|------|----------|
| **A**: New `ConditionPromotionBox` | Clean separation | Duplication with TrimLoopLowerer | ❌ Rejected |
| **B**: Extend `TrimLoopLowerer` with `for_condition()` | Reuse existing logic | Name becomes misleading | ❌ Rejected |
| **C**: Extract `LoopBodyCondPromoter` as shared base | Single responsibility | Refactoring required | ✅ **Selected** |

**Selected Approach (Option C)**:
- **Phase 223-2 (Design)**: Define `LoopBodyCondPromoter` API (name + types)
- **Phase 223-3 (Impl)**: Extract from `TrimLoopLowerer`, integrate into Pattern4

---

### 3.2 API Definition: LoopBodyCondPromoter

**New File**: `src/mir/loop_pattern_detection/loop_body_cond_promoter.rs` (design-only, no implementation yet)

```rust
/// Phase 223-2: LoopBodyLocal condition promotion coordinator
///
/// Handles promotion of LoopBodyLocal variables used in loop conditions
/// to bool carriers. Supports Pattern2 (break), Pattern4 (continue),
/// and future patterns.
///
/// ## Responsibilities
///
/// - Detect safe promotion patterns (Category A-3 from phase223-loopbodylocal-condition-inventory.md)
/// - Coordinate with LoopBodyCarrierPromoter for actual promotion logic
/// - Provide uniform API for Pattern2/Pattern4 integration
///
/// ## Design Principle
///
/// This is a **thin coordinator** that reuses existing boxes:
/// - LoopBodyCarrierPromoter: Promotion logic (Trim pattern detection)
/// - TrimLoopHelper: Pattern-specific metadata
/// - ConditionEnvBuilder: Binding generation
pub struct LoopBodyCondPromoter;

/// Promotion request for condition variables
///
/// Unified API for Pattern2 (break) and Pattern4 (continue)
pub struct ConditionPromotionRequest<'a> {
    /// Loop parameter name (e.g., "i")
    pub loop_param_name: &'a str,

    /// Condition scope analysis result
    pub cond_scope: &'a LoopConditionScope,

    /// Loop structure metadata
    pub scope_shape: Option<&'a LoopScopeShape>,

    /// Break condition AST (Pattern2: Some, Pattern4: None)
    pub break_cond: Option<&'a ASTNode>,

    /// Continue condition AST (Pattern4: Some, Pattern2: None)
    pub continue_cond: Option<&'a ASTNode>,

    /// Loop body statements
    pub loop_body: &'a [ASTNode],
}

/// Promotion result
pub enum ConditionPromotionResult {
    /// Promotion successful
    Promoted {
        /// Carrier metadata (from TrimLoopHelper)
        carrier_info: CarrierInfo,

        /// Variable name that was promoted (e.g., "ch")
        promoted_var: String,

        /// Promoted carrier name (e.g., "is_whitespace")
        carrier_name: String,
    },

    /// Cannot promote (Fail-Fast)
    CannotPromote {
        /// Human-readable reason
        reason: String,

        /// List of problematic LoopBodyLocal variables
        vars: Vec<String>,
    },
}

impl LoopBodyCondPromoter {
    /// Try to promote LoopBodyLocal variables in conditions
    ///
    /// Phase 223-2 Design: API signature only
    /// Phase 223-3 Implementation: Extract from TrimLoopLowerer
    ///
    /// ## P0 Requirements (Category A-3)
    ///
    /// - Single LoopBodyLocal variable (e.g., `ch`)
    /// - Definition: `local ch = s.substring(...)` or similar
    /// - Condition: Simple equality chain (e.g., `ch == " " || ch == "\t"`)
    /// - Pattern: Identical to existing Trim pattern
    ///
    /// ## Algorithm (Delegated to LoopBodyCarrierPromoter)
    ///
    /// 1. Extract LoopBodyLocal names from cond_scope
    /// 2. Build PromotionRequest for LoopBodyCarrierPromoter
    /// 3. Call LoopBodyCarrierPromoter::try_promote()
    /// 4. Convert PromotionResult to ConditionPromotionResult
    /// 5. Return result (Promoted or CannotPromote)
    ///
    /// ## Differences from TrimLoopLowerer
    ///
    /// - TrimLoopLowerer: Full lowering pipeline (detection + code generation)
    /// - LoopBodyCondPromoter: Detection + metadata only (no code generation)
    ///
    /// This allows Pattern4 to handle code generation differently than Pattern2
    /// while sharing the same promotion logic.
    pub fn try_promote_for_condition(
        req: ConditionPromotionRequest
    ) -> ConditionPromotionResult {
        // Phase 223-3 implementation:
        // 1. Check P0 constraints (single LoopBodyLocal, simple pattern)
        // 2. Delegate to LoopBodyCarrierPromoter::try_promote()
        // 3. Convert result to ConditionPromotionResult
        unimplemented!("Phase 223-3")
    }
}
```

---

### 3.3 P0 Minimum Requirements

**Target Pattern**: Category A-3 (_skip_whitespace)

```hako
loop(i < n) {
    local ch = src.substring(i, i + 1)
    if ch == " " || ch == "\t" || ch == "\n" || ch == "\r" {
        i = i + 1
        continue
    }
    break
}
```

**Detection Criteria** (must all be true):

1. **Single LoopBodyLocal**: Exactly one LoopBodyLocal variable (e.g., `ch`)
2. **Substring Definition**: `local ch = s.substring(i, i+1)` or equivalent
3. **Simple Condition**: Equality chain (`ch == "lit" || ch == "lit2" || ...`)
4. **No Reassignment**: Variable is not reassigned inside loop body
5. **Trim Pattern Match**: Must match existing TrimLoopHelper pattern

**Fail-Fast Cases** (Phase 223-2 will not handle):

- Multiple LoopBodyLocal variables (Category A-5: string comparison)
- Cascading LoopBodyLocal (Category A-4: `ch` + `digit_pos`)
- Complex expressions (Category B-1: nested if with reassignment)
- Method call chains (Category B-2)

---

## 4. Integration Strategy: Pattern4 Usage

### 4.1 Current Pattern4 Fail-Fast Point

**File**: `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs`

**Current Code**:
```rust
let loop_cond_scope = LoopConditionScopeBox::analyze(
    Some(&continue_cond),
    &pipeline_ctx.loop_scope,
);

if loop_cond_scope.has_loop_body_local() {
    return Err(format!(
        "[joinir/freeze] Pattern4: unsupported condition variables (LoopBodyLocal): {}",
        extract_body_local_names(&loop_cond_scope.vars).join(", ")
    ));
}
```

---

### 4.2 Future Pattern4 Integration (Phase 223-3)

**Proposed Change** (design-only, not implemented):

```rust
// Phase 223-2 Design: Integration point
let loop_cond_scope = LoopConditionScopeBox::analyze(
    Some(&continue_cond),
    &pipeline_ctx.loop_scope,
);

if loop_cond_scope.has_loop_body_local() {
    // Phase 223-3: Try promotion before Fail-Fast
    use crate::mir::loop_pattern_detection::loop_body_cond_promoter::{
        LoopBodyCondPromoter, ConditionPromotionRequest, ConditionPromotionResult
    };

    let promotion_req = ConditionPromotionRequest {
        loop_param_name: &pipeline_ctx.loop_var_name,
        cond_scope: &loop_cond_scope,
        scope_shape: Some(&pipeline_ctx.loop_scope),
        break_cond: None,  // Pattern4: no break condition
        continue_cond: Some(&continue_cond),
        loop_body: &body,
    };

    match LoopBodyCondPromoter::try_promote_for_condition(promotion_req) {
        ConditionPromotionResult::Promoted { carrier_info, promoted_var, carrier_name } => {
            // Success: Merge promoted carrier into existing carrier_info
            let updated_carrier_info = pipeline_ctx.carrier_info
                .merge_from(&carrier_info)
                .map_err(|e| format!("[joinir/freeze] Pattern4: carrier merge failed: {}", e))?;

            // Update pipeline_ctx with new carrier_info
            pipeline_ctx.carrier_info = updated_carrier_info;

            // Continue with Pattern4 lowering (carrier now includes promoted bool)
        }
        ConditionPromotionResult::CannotPromote { reason, vars } => {
            // Fail-Fast: Same as current behavior
            return Err(format!(
                "[joinir/freeze] Pattern4: unsupported condition variables (LoopBodyLocal): {} ({})",
                vars.join(", "),
                reason
            ));
        }
    }
}
```

**Key Points**:

1. **Promotion First**: Try promotion before Fail-Fast
2. **Merge Carriers**: Use existing `CarrierInfo::merge_from()` API
3. **Fail-Fast Fallback**: If promotion fails, maintain current behavior
4. **No Code Generation**: `LoopBodyCondPromoter` only provides metadata, Pattern4 lowerer handles JoinIR emission

---

### 4.3 Code Generation Differences: Pattern2 vs Pattern4

| Aspect | Pattern2 (break) | Pattern4 (continue) |
|--------|------------------|---------------------|
| **Condition Usage** | `if !is_carrier { break }` | `if is_carrier { continue }` |
| **Carrier Initialization** | Before loop entry | Before loop entry (same) |
| **Carrier Update** | End of loop body | End of loop body (same) |
| **JoinIR Structure** | Break to exit block | Continue to header block |

**Design Note**: TrimLoopLowerer currently generates `!is_carrier` for break. Pattern4 will use `is_carrier` directly (no negation).

---

## 5. Relationship to Existing Boxes

### 5.1 Box Hierarchy

```
LoopBodyCondPromoter (新規 - Phase 223-2 design)
  ├── Delegates to: LoopBodyCarrierPromoter (既存)
  │   ├── Uses: TrimPatternInfo (既存)
  │   └── Uses: TrimLoopHelper (既存)
  └── Used by:
      ├── Pattern2 (via TrimLoopLowerer - 既存)
      └── Pattern4 (direct call - Phase 223-3 実装)
```

### 5.2 Responsibility Matrix

| Box | Detection | Metadata | Code Gen | Integration |
|-----|-----------|----------|----------|-------------|
| **LoopConditionScopeBox** | ✅ Classify vars | ❌ | ❌ | Pattern2/4 |
| **LoopBodyCarrierPromoter** | ✅ Trim pattern | ✅ TrimPatternInfo | ❌ | TrimLoopLowerer |
| **TrimLoopLowerer** | ❌ (delegates) | ✅ CarrierInfo | ✅ MIR emission | Pattern2 only |
| **LoopBodyCondPromoter** (新) | ❌ (delegates) | ✅ CarrierInfo | ❌ | Pattern2/4 |

**Design Principle**: Single Responsibility
- **Detection**: LoopBodyCarrierPromoter
- **Metadata**: TrimPatternInfo, CarrierInfo
- **Code Generation**: Pattern-specific lowerers (TrimLoopLowerer for P2, Pattern4 lowerer for P4)
- **Coordination**: LoopBodyCondPromoter (thin wrapper)

---

## 6. Phase 223-3 Implementation Hints

### 6.1 Extraction Strategy

**Source**: `TrimLoopLowerer::try_lower_trim_like_loop()`

**Extract to**: `LoopBodyCondPromoter::try_promote_for_condition()`

**Extraction Boundary**:

```rust
// KEEP in TrimLoopLowerer (Pattern2-specific):
// - carrier initialization code generation (emit_trim_carrier_init)
// - condition replacement (build_negated_carrier_condition)
// - ConditionBinding construction

// MOVE to LoopBodyCondPromoter (pattern-agnostic):
// - LoopBodyCarrierPromoter::try_promote() call
// - PromotionResult → ConditionPromotionResult conversion
// - P0 constraint checking (single var, simple pattern)
```

**Expected Line Count**:
- LoopBodyCondPromoter: ~50-80 lines (thin wrapper)
- TrimLoopLowerer: No change (continues to use new API internally)
- Pattern4: +30-40 lines (integration point)

---

### 6.2 Test Strategy

**Phase 223-2**: No tests (design-only)

**Phase 223-3**: Unit tests for LoopBodyCondPromoter

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p0_skip_whitespace_promotion() {
        // Category A-3: Single LoopBodyLocal, simple equality chain
        let req = ConditionPromotionRequest {
            // ... build request for _skip_whitespace pattern
        };

        match LoopBodyCondPromoter::try_promote_for_condition(req) {
            ConditionPromotionResult::Promoted { carrier_name, .. } => {
                assert_eq!(carrier_name, "is_whitespace");
            }
            _ => panic!("Expected promotion success"),
        }
    }

    #[test]
    fn test_cascading_loopbodylocal_fail_fast() {
        // Category A-4: Cascading (ch + digit_pos) should Fail-Fast in P0
        let req = ConditionPromotionRequest {
            // ... build request for cascading pattern
        };

        match LoopBodyCondPromoter::try_promote_for_condition(req) {
            ConditionPromotionResult::CannotPromote { reason, vars } => {
                assert!(reason.contains("multiple LoopBodyLocal"));
                assert_eq!(vars.len(), 2); // ch + digit_pos
            }
            _ => panic!("Expected Fail-Fast"),
        }
    }
}
```

**E2E Test** (Phase 223-3):
- File: `apps/tests/phase223_p4_skip_whitespace.hako`
- Content: Category A-3 pattern (skip_whitespace with continue)
- Expected: Pattern4 lowering succeeds, MIR execution returns correct result

---

## 7. Design Constraints

### 7.1 P0 Constraints (Strict)

1. **Single LoopBodyLocal Only**: Exactly one variable in condition
2. **Trim Pattern Only**: Must match existing TrimLoopHelper detection
3. **No Cascading**: No dependencies between LoopBodyLocal variables
4. **No Multi-Break**: Only one break/continue condition (not both)

**Rationale**: P0 focuses on unblocking _skip_whitespace (Category A-3), which is identical to Trim pattern except for continue vs break.

---

### 7.2 Future Extensions (P1/P2)

**P1 Extensions** (Phase 224+):
- **Cascading LoopBodyLocal** (Category A-4):
  - `local ch = s.substring(...); local pos = digits.indexOf(ch)`
  - Promote only leaf variable (`pos` → `is_digit`)
  - Requires dependency analysis in LoopBodyCarrierPromoter

**P2 Extensions** (Phase 225+):
- **Multi-Variable Patterns** (Category A-5):
  - `local ch_s = ...; local ch_lit = ...; if ch_s != ch_lit`
  - Promote to single carrier (`chars_match`)
  - Requires multi-variable carrier initialization

**Non-Goals**:
- **Category B** (Complex patterns): Continue to Fail-Fast
- **Nested if with reassignment**: Too complex for safe promotion
- **Method call chains**: Side effects cannot be promoted

---

## 8. Documentation Updates

### 8.1 CURRENT_TASK.md

**Add to "最近まとまった大きな塊" section**:

```markdown
- **Phase 223 LoopBodyLocal Condition Promotion (進行中)**
  - Phase 223-1: 棚卸完了 (6 Category A patterns, 1 Category B, 2 Category C)
  - Phase 223-2: 設計完了 (LoopBodyCondPromoter API, P0 _skip_whitespace 対応)
  - Phase 223-3: 実装予定 (Pattern4 統合, E2E テスト)
```

---

### 8.2 joinir-architecture-overview.md

**Add to Section 2.2 (条件式ライン)**:

```markdown
- **LoopBodyCondPromoter（Phase 223-2 設計完了）**
  - ファイル: `src/mir/loop_pattern_detection/loop_body_cond_promoter.rs`
  - 責務:
    - ループ条件（header/break/continue）に出てくる LoopBodyLocal を carrier に昇格。
    - P0（Category A-3: _skip_whitespace）: 単一変数の Trim パターンのみ対応。
    - Pattern2/Pattern4 の統一 API として機能（既存 TrimLoopLowerer から抽出予定）。
  - 設計原則:
    - Thin coordinator: LoopBodyCarrierPromoter に detection を委譲、metadata のみ返す。
    - Pattern-agnostic: Pattern2 (break) / Pattern4 (continue) 両対応。
    - Fail-Fast: 複雑パターン（Category B）は引き続き reject。
```

---

## 9. Summary: Design Decisions

| Decision | Rationale |
|----------|-----------|
| **Reuse existing Trim infrastructure** | Avoid duplication, leverage proven Pattern2 logic |
| **Thin coordinator pattern** | LoopBodyCondPromoter delegates to LoopBodyCarrierPromoter |
| **P0: Category A-3 only** | Unblock _skip_whitespace, simplest safe pattern |
| **No code generation in coordinator** | Pattern-specific lowerers handle JoinIR emission |
| **Fail-Fast for complex patterns** | Category B continues to reject (safety first) |
| **Extract from TrimLoopLowerer** | Pattern2 continues to work, Pattern4 gains promotion |

---

## 10. Next Phase: Phase 223-3 Implementation

**Deliverables**:

1. **LoopBodyCondPromoter implementation** (~50-80 lines)
   - File: `src/mir/loop_pattern_detection/loop_body_cond_promoter.rs`
   - Extract promotion logic from TrimLoopLowerer
   - Add P0 constraint checking

2. **Pattern4 integration** (+30-40 lines)
   - File: `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs`
   - Call LoopBodyCondPromoter before Fail-Fast
   - Merge promoted carrier into existing CarrierInfo

3. **Unit tests** (5-7 test cases)
   - P0 promotion success (Category A-3)
   - Cascading Fail-Fast (Category A-4)
   - Complex pattern Fail-Fast (Category B-1)

4. **E2E test** (1 file)
   - File: `apps/tests/phase223_p4_skip_whitespace.hako`
   - Pattern: _skip_whitespace with continue
   - Expected: Pattern4 lowering + MIR execution success

**Estimated Size**: +150-200 lines total (net)

---

## Revision History

- **2025-12-10**: Phase 223-2 design document created (API-only, no implementation)
Status: Active  
Scope: LoopBodyLocal condition 設計（JoinIR/ExprLowerer ライン）
