# Phase 180: Trim/CharComparison P5 Submodule Design

**Status**: Design Complete
**Date**: 2025-12-08
**Author**: Claude Code

## Overview

Phase 180 extracts Trim/P5 (Pattern 5) specific logic from Pattern2 and Pattern4 into a dedicated submodule, improving code organization and maintainability.

## Current State Analysis

### Trim/P5 Components Currently Used

#### 1. **LoopConditionScopeBox** (`src/mir/loop_pattern_detection/loop_condition_scope.rs`)
- **Purpose**: Detects LoopBodyLocal variables in condition scope
- **Usage**: Called by Pattern2/Pattern4 to identify variables that need promotion
- **Current Location**: `loop_pattern_detection` module
- **Action**: Keep in current location (detection, not lowering)

#### 2. **LoopBodyCarrierPromoter** (`src/mir/loop_pattern_detection/loop_body_carrier_promoter.rs`)
- **Purpose**: Promotes LoopBodyLocal variables to carriers
- **Returns**: `TrimPatternInfo` with carrier metadata
- **Current Location**: `loop_pattern_detection` module
- **Action**: Keep in current location (pattern detection/analysis)

#### 3. **TrimLoopHelper** (`src/mir/loop_pattern_detection/trim_loop_helper.rs`)
- **Purpose**: Helper for Trim pattern lowering
- **Contains**: Pattern-specific logic (whitespace chars, carrier name, etc.)
- **Current Location**: `loop_pattern_detection` module
- **Action**: Keep in current location (shared data structure)

#### 4. **TrimPatternLowerer** (`src/mir/builder/control_flow/joinir/patterns/trim_pattern_lowerer.rs`)
- **Purpose**: JoinIR-specific Trim lowering logic
- **Functions**:
  - `generate_trim_break_condition()` - Creates `!is_carrier` break condition
  - `add_to_condition_env()` - Adds carrier to ConditionEnv
- **Current Location**: `joinir/patterns` module
- **Action**: Move to new `trim_loop_lowering.rs` module

#### 5. **TrimPatternValidator** (`src/mir/builder/control_flow/joinir/patterns/trim_pattern_validator.rs`)
- **Purpose**: Validates and extracts Trim pattern structure
- **Functions**:
  - `extract_substring_args()` - Extracts substring pattern
  - `emit_whitespace_check()` - Generates whitespace comparison
- **Current Location**: `joinir/patterns` module
- **Action**: Move to new `trim_loop_lowering.rs` module

#### 6. **Pattern2/Pattern4 Inline Trim Logic**
- **Location**: Lines 180-340 in `pattern2_with_break.rs`
- **Location**: Lines 280+ in `pattern4_with_continue.rs`
- **Logic**:
  - LoopBodyCarrierPromoter invocation
  - TrimPatternValidator calls
  - Carrier initialization code generation
  - Break condition replacement
  - ConditionEnv manipulation
- **Action**: Extract to `TrimLoopLowerer::try_lower_trim_like_loop()`

## New Module Design

### Module Location

```
src/mir/join_ir/lowering/trim_loop_lowering.rs
```

### Module Structure

```rust
//! Phase 180: Trim/P5 Dedicated Lowering Module
//!
//! Consolidates all Trim pattern lowering logic from Pattern2/4.
//!
//! ## Responsibilities
//! - Detect Trim-like loops
//! - Promote LoopBodyLocal variables to carriers
//! - Generate carrier initialization code
//! - Replace break conditions
//! - Setup ConditionEnv bindings

use crate::mir::loop_pattern_detection::{
    LoopConditionScopeBox,
    loop_body_carrier_promoter::{LoopBodyCarrierPromoter, PromotionRequest},
    trim_loop_helper::TrimLoopHelper,
};
use crate::mir::join_ir::lowering::{
    carrier_info::CarrierInfo,
    condition_env::ConditionBinding,
};
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;
use crate::ast::ASTNode;

pub struct TrimLoopLowerer;

/// Result of Trim lowering preprocessing
pub struct TrimLoweringResult {
    /// Replaced break condition (e.g., `!is_carrier`)
    pub condition: ASTNode,

    /// Updated carrier info with promoted carrier
    pub carrier_info: CarrierInfo,

    /// Updated condition environment bindings
    pub condition_bindings: Vec<ConditionBinding>,

    /// Trim helper for pattern-specific operations
    pub trim_helper: TrimLoopHelper,
}

impl TrimLoopLowerer {
    /// Try to lower a Trim-like loop
    ///
    /// Returns:
    /// - `Some(TrimLoweringResult)` if Trim pattern detected and lowered
    /// - `None` if not a Trim pattern (normal loop)
    /// - `Err` if Trim pattern detected but lowering failed
    pub fn try_lower_trim_like_loop(
        builder: &mut MirBuilder,
        scope: &LoopScopeShape,
        loop_cond: &ASTNode,
        break_cond: &ASTNode,
        body: &[ASTNode],
        loop_var_name: &str,
        carrier_info: &mut CarrierInfo,
        alloc_join_value: &mut dyn FnMut() -> ValueId,
    ) -> Result<Option<TrimLoweringResult>, String> {
        // Implementation will consolidate Pattern2/4 logic
    }
}
```

## Refactoring Plan

### Phase 1: Module Creation (Task 180-2)

1. Create `src/mir/join_ir/lowering/trim_loop_lowering.rs`
2. Add to `src/mir/join_ir/lowering/mod.rs`
3. Implement skeleton with stub functions

### Phase 2: Logic Extraction (Task 180-3)

#### From Pattern2 (lines 180-340):

**Extract to `try_lower_trim_like_loop()`:**
1. LoopConditionScopeBox analysis (lines 189-200)
2. LoopBodyCarrierPromoter invocation (lines 201-240)
3. TrimPatternValidator calls (lines 244-340)
4. Carrier initialization code generation (lines 267-313)
5. ConditionEnv binding setup (lines 345-377)

**Pattern2 New Code (~20 lines):**
```rust
// Trim/P5 processing delegation
if let Some(trim_result) = TrimLoopLowerer::try_lower_trim_like_loop(
    self,
    &scope,
    condition,
    &break_condition_node,
    _body,
    &loop_var_name,
    &mut carrier_info,
    &mut alloc_join_value,
)? {
    effective_break_condition = trim_result.condition.clone();
    condition_bindings.extend(trim_result.condition_bindings);
    carrier_info = trim_result.carrier_info;
}
```

**Lines Removed**: ~160 lines
**Lines Added**: ~20 lines
**Net Reduction**: -140 lines in Pattern2

### Phase 3: Pattern4 Integration (Task 180-4)

**Current Pattern4 Trim Logic** (lines 280-306):
- Same LoopBodyCarrierPromoter call
- Trim pattern safety check
- Error handling

**Action**: Replace with same `TrimLoopLowerer::try_lower_trim_like_loop()` call

**Estimated Impact**:
- Lines removed: ~30 lines
- Lines added: ~15 lines
- Net reduction: -15 lines

### Phase 4: Module Consolidation

**Move to `trim_loop_lowering.rs`:**
1. `TrimPatternLowerer` (100+ lines)
2. `TrimPatternValidator` (150+ lines)
3. Inline Trim logic from Pattern2 (~160 lines)
4. Inline Trim logic from Pattern4 (~30 lines)

**Total New Module Size**: ~450 lines (well-scoped, single responsibility)

## Benefits

### 1. **Single Responsibility**
- Pattern2/4 focus on generic loop lowering
- Trim logic isolated in dedicated module

### 2. **Reusability**
- Same Trim lowering for Pattern2, Pattern4, future patterns
- No code duplication

### 3. **Testability**
- Trim logic can be unit tested independently
- Pattern2/4 tests focus on core loop logic

### 4. **Maintainability**
- Trim pattern changes touch only one module
- Clear boundary between generic and pattern-specific logic

### 5. **Code Size Reduction**
- Pattern2: -140 lines (511 → 371 lines)
- Pattern4: -15 lines (smaller impact)
- Overall: Better organized, easier to navigate

## Implementation Safety

### Conservative Approach

1. **No Behavior Changes**: Refactoring only, same logic flow
2. **Incremental**: One pattern at a time (Pattern2 first)
3. **Test Coverage**: Verify existing Trim tests still pass
4. **Commit Per Task**: Easy rollback if issues arise

### Test Strategy

**Existing Tests to Verify**:
```bash
# Trim pattern tests
cargo test --release --lib trim

# JsonParser tests (uses Trim pattern)
./target/release/hakorune apps/tests/test_jsonparser_skip_whitespace.hako

# Pattern2 tests
cargo test --release --lib pattern2

# Pattern4 tests
cargo test --release --lib pattern4
```

## Dependencies

### Keep in Current Locations
- `LoopConditionScopeBox` - Detection logic
- `LoopBodyCarrierPromoter` - Promotion engine
- `TrimLoopHelper` - Shared data structure

### Move to New Module
- `TrimPatternLowerer` - JoinIR lowering
- `TrimPatternValidator` - Pattern validation
- Inline Trim logic from Pattern2/4

## Architecture Update

**Update**: `joinir-architecture-overview.md`

Add new section:
```markdown
### TrimLoopLowerer (P5 Dedicated Module)

**Location**: `src/mir/join_ir/lowering/trim_loop_lowering.rs`

**Purpose**: Dedicated lowering for Trim/CharComparison patterns (Pattern 5)

**Components**:
- `TrimLoopLowerer::try_lower_trim_like_loop()` - Main entry point
- `TrimPatternLowerer` - JoinIR condition generation
- `TrimPatternValidator` - Pattern structure validation

**Used By**: Pattern2, Pattern4 (and future patterns)
```

## Success Criteria

1. ✅ All existing Trim tests pass
2. ✅ Pattern2/4 tests pass unchanged
3. ✅ Build with 0 errors (warnings acceptable)
4. ✅ Code size reduction achieved (-135 lines in Pattern2)
5. ✅ Documentation updated
6. ✅ Each task committed separately

## Pattern4 Analysis

**Finding**: Pattern4 has Trim detection logic (lines 280-318), but it only validates and returns an error:

```rust
// Phase 171-impl-Trim: Validation successful!
// Phase 172+ will implement the actual JoinIR generation for Trim patterns
// For now, return an informative message that the pattern is recognized but not yet lowered
return Err(format!(
    "[cf_loop/pattern4] ✅ Trim pattern validation successful! \
     Carrier '{}' ready for Phase 172 implementation. \
     (Pattern detection: PASS, Safety check: PASS, JoinIR lowering: TODO)",
    helper.carrier_name
));
```

**Decision**: Skip Pattern4 refactoring for now. The Trim logic in Pattern4 doesn't do actual lowering, just detection + error. When Phase 172+ implements Pattern4 Trim lowering, it can use TrimLoopLowerer directly.

## Timeline

- **Task 180-1**: Design document (this file) - 15 minutes ✅
- **Task 180-2**: Skeleton creation - 10 minutes ✅
- **Task 180-3**: Pattern2 refactoring - 30 minutes ✅
- **Task 180-4**: Pattern4 refactoring - SKIPPED (lines 280-318 just return error, no actual lowering)
- **Task 180-5**: Testing and docs - 20 minutes

**Total Estimated Time**: 75 minutes (Pattern4 skipped)

## Notes

- This refactoring does NOT change any behavior
- Focus is on code organization and maintainability
- Trim pattern logic remains identical, just relocated
- Future Phase 181+ can build on this clean foundation
Status: Historical
