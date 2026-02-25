# Phase 33-22: Common Pattern Initialization & Conversion Pipeline

## Overview

This phase integrates CommonPatternInitializer and JoinIRConversionPipeline across all 4 loop patterns, eliminating code duplication and establishing unified initialization and conversion flows.

## Current State Analysis (Before Refactoring)

### Pattern 1 (pattern1_minimal.rs)

**Lines 66-76**: Loop var extraction
```rust
let loop_var_name = self.extract_loop_variable_from_condition(condition)?;
let loop_var_id = self.variable_map.get(&loop_var_name)
    .copied()
    .ok_or_else(|| format!("[cf_loop/pattern1] Loop variable '{}' not found", loop_var_name))?;
```

**Lines 147-153**: Boundary creation
```rust
let mut boundary = JoinInlineBoundary::new_inputs_only(
    vec![ValueId(0)],
    vec![loop_var_id],
);
boundary.loop_var_name = Some(loop_var_name.clone());
```

**Lines 126-156**: JoinIR conversion and merge
- Manual stats logging
- convert_join_module_to_mir_with_meta call
- merge_joinir_mir_blocks call

**Status**:
- ✅ boundary.loop_var_name: Set (Phase 33-23 fix)
- ExitMeta: None (simple loop)
- exit_bindings: None

### Pattern 2 (pattern2_with_break.rs)

**Lines 58-68**: Loop var extraction (duplicated from Pattern 1)
```rust
let loop_var_name = self.extract_loop_variable_from_condition(condition)?;
let loop_var_id = self.variable_map.get(&loop_var_name)
    .copied()
    .ok_or_else(|| format!("[cf_loop/pattern2] Loop variable '{}' not found", loop_var_name))?;
```

**Lines 73-126**: Condition variable handling (Pattern 2 specific)
- ConditionEnv building
- ConditionBinding creation

**Lines 191-205**: Boundary creation with exit_bindings
```rust
let exit_bindings = ExitMetaCollector::collect(self, &exit_meta, debug);
let mut boundary = JoinInlineBoundary::new_inputs_only(...);
boundary.condition_bindings = condition_bindings;
boundary.exit_bindings = exit_bindings.clone();
boundary.loop_var_name = Some(loop_var_name.clone());
```

**Lines 175-208**: JoinIR conversion and merge
- Manual stats logging
- convert_join_module_to_mir_with_meta call
- merge_joinir_mir_blocks call

**Status**:
- ✅ boundary.loop_var_name: Set
- ExitMeta: Break-triggered vars
- exit_bindings: From ExitMetaCollector
- condition_bindings: Custom handling

### Pattern 3 (pattern3_with_if_phi.rs)

**Lines 60-82**: Loop var + carrier extraction
```rust
let loop_var_name = self.extract_loop_variable_from_condition(condition)?;
let loop_var_id = self.variable_map.get(&loop_var_name).copied()?;
let sum_var_id = self.variable_map.get("sum").copied()?;
```

**Lines 139-154**: Boundary creation with exit_bindings
```rust
let mut boundary = JoinInlineBoundary::new_with_exit_bindings(
    vec![ValueId(0), ValueId(1)],
    vec![loop_var_id, sum_var_id],
    vec![
        LoopExitBinding {
            carrier_name: "sum".to_string(),
            join_exit_value: ValueId(18),
            host_slot: sum_var_id,
        }
    ],
);
boundary.loop_var_name = Some(loop_var_name.clone());
```

**Lines 125-156**: JoinIR conversion and merge
- Manual stats logging
- convert_join_module_to_mir_with_meta call
- merge_joinir_mir_blocks call

**Status**:
- ✅ boundary.loop_var_name: Set (Phase 33-23 fix)
- ExitMeta: i + sum carriers
- exit_bindings: Hardcoded for "sum"

### Pattern 4 (pattern4_with_continue.rs)

**Lines 144-152**: Loop var extraction (duplicated)
```rust
let loop_var_name = self.extract_loop_variable_from_condition(condition)?;
let loop_var_id = self.variable_map.get(&loop_var_name)
    .copied()
    .ok_or_else(|| format!(...)?;
```

**Lines 155-179**: CarrierInfo building (Pattern 4 specific)
```rust
let mut carriers = Vec::new();
for (var_name, &var_id) in &self.variable_map {
    if var_name != &loop_var_name {
        carriers.push(CarrierVar {
            name: var_name.clone(),
            host_id: var_id,
        });
    }
}
```

**Lines 137-142**: Continue normalization (Pattern 4 specific)
```rust
let normalized_body = ContinueBranchNormalizer::normalize_loop_body(_body);
let body_to_analyze = &normalized_body;
```

**Status**:
- ✅ boundary.loop_var_name: Set
- ExitMeta: Dynamic carrier analysis
- exit_bindings: Comprehensive
- Special: ContinueBranchNormalizer + LoopUpdateAnalyzer

## Commonalization Strategy

### Shared Initialization (ALL patterns)

The following steps are identical across all 4 patterns and can be unified:

1. **Extract loop variable from condition**
   - Call `extract_loop_variable_from_condition(condition)`
   - Look up ValueId in variable_map
   - Error handling with pattern-specific message

2. **Build CarrierInfo from variable_map**
   - Iterate through variable_map
   - Filter out loop variable
   - Create CarrierVar structs
   - Optional: exclude specific variables (Pattern 2)

3. **Set boundary.loop_var_name** ← Critical invariant
   - Required for header PHI generation
   - Must be set for all patterns

### Pattern-Specific (Handled after init)

Each pattern has specific needs that happen AFTER common initialization:

- **Pattern 1**: No special handling (simplest case)
- **Pattern 2**:
  - ConditionEnv building
  - ConditionBinding creation
  - ExitMetaCollector usage
- **Pattern 3**:
  - Hardcoded exit_bindings for "sum"
  - Multiple carriers (i + sum)
- **Pattern 4**:
  - ContinueBranchNormalizer
  - LoopUpdateAnalyzer
  - Dynamic carrier filtering

### Conversion Pipeline (ALL patterns)

The JoinIR → MIR → Merge flow is identical:

1. Log JoinIR stats (functions, blocks)
2. Convert JoinModule → MirModule
3. Log MIR stats (functions, blocks)
4. Call merge_joinir_mir_blocks
5. Return result

## Code Duplication Identified

### Loop Variable Extraction (4 occurrences × ~10 lines = 40 lines)

```rust
// Pattern 1, 2, 3, 4: All have this
let loop_var_name = self.extract_loop_variable_from_condition(condition)?;
let loop_var_id = self.variable_map.get(&loop_var_name)
    .copied()
    .ok_or_else(|| format!("[cf_loop/patternX] Loop variable '{}' not found", loop_var_name))?;
```

### Conversion + Merge (4 occurrences × ~30 lines = 120 lines)

```rust
// Pattern 1, 2, 3, 4: All have this
trace::trace().joinir_stats(...);
let mir_module = convert_join_module_to_mir_with_meta(&join_module, &empty_meta)?;
trace::trace().joinir_stats(...);
let exit_phi_result = self.merge_joinir_mir_blocks(&mir_module, Some(&boundary), debug)?;
```

### CarrierInfo Building (3 occurrences × ~15 lines = 45 lines)

```rust
// Pattern 3, 4: Build carriers from variable_map
let mut carriers = Vec::new();
for (var_name, &var_id) in &self.variable_map {
    if var_name != &loop_var_name {
        carriers.push(CarrierVar { name: var_name.clone(), host_id: var_id });
    }
}
```

**Total Duplication**: ~205 lines

## Implementation Plan

### Phase 1: CommonPatternInitializer Integration

For each pattern:
1. Replace loop var extraction with `CommonPatternInitializer::initialize_pattern`
2. Use returned `carrier_info` instead of manual building
3. Keep pattern-specific processing (condition_bindings, normalizer, etc.)

### Phase 2: JoinIRConversionPipeline Integration

For each pattern:
1. Replace manual conversion + merge with `JoinIRConversionPipeline::execute`
2. Remove duplicate stats logging
3. Maintain error handling

## Expected Results

### Code Reduction
- Pattern 1: -20 lines (initialization + conversion)
- Pattern 2: -25 lines (initialization + conversion, keep condition handling)
- Pattern 3: -20 lines (initialization + conversion)
- Pattern 4: -25 lines (initialization + conversion, keep normalizer)
- **Total**: -90 lines (conservative estimate)

### Maintainability Improvements
- Single source of truth for initialization
- Unified conversion pipeline
- Easier to add new patterns
- Reduced cognitive load

### Testing Requirements
- All 4 patterns must pass existing tests
- No behavioral changes
- SSA-undef checks must pass

## Migration Guide

### Before (Pattern 1)
```rust
let loop_var_name = self.extract_loop_variable_from_condition(condition)?;
let loop_var_id = self.variable_map.get(&loop_var_name).copied()?;
let mut boundary = JoinInlineBoundary::new_inputs_only(...);
boundary.loop_var_name = Some(loop_var_name.clone());
let mir_module = convert_join_module_to_mir_with_meta(&join_module, &empty_meta)?;
let exit_phi_result = self.merge_joinir_mir_blocks(&mir_module, Some(&boundary), debug)?;
```

### After (Pattern 1)
```rust
use super::common_init::CommonPatternInitializer;
use super::conversion_pipeline::JoinIRConversionPipeline;

let (loop_var_name, loop_var_id, _carrier_info) =
    CommonPatternInitializer::initialize_pattern(self, condition, &self.variable_map, None)?;

let mut boundary = JoinInlineBoundary::new_inputs_only(
    vec![ValueId(0)],
    vec![loop_var_id],
);
boundary.loop_var_name = Some(loop_var_name.clone());

let exit_phi_result = JoinIRConversionPipeline::execute(
    self,
    join_module,
    Some(&boundary),
    "pattern1",
    debug,
)?;
```

## Success Criteria

✅ CommonPatternInitializer used in all 4 patterns
✅ JoinIRConversionPipeline used in all 4 patterns
✅ At least 90 lines of code removed
✅ All existing tests pass
✅ No new SSA-undef errors
✅ No new compiler warnings
✅ Documentation updated

## Implementation Results

### Code Reduction Achievement

**Pattern File Line Counts (After Refactoring)**:
- Pattern 1: 151 lines
- Pattern 2: 191 lines
- Pattern 3: 148 lines
- Pattern 4: 316 lines
- **Total**: 806 lines

**Infrastructure**:
- CommonPatternInitializer: 117 lines
- JoinIRConversionPipeline: 127 lines
- **Total Infrastructure**: 244 lines

**Net Result**: All 4 patterns + infrastructure = 1,050 lines

### Before/After Comparison

**Pattern-Specific Changes**:

#### Pattern 1
- **Removed**: Loop var extraction (10 lines), JoinIR conversion + merge (30 lines)
- **Added**: CommonPatternInitializer call (7 lines), JoinIRConversionPipeline call (8 lines)
- **Net Reduction**: ~25 lines

#### Pattern 2
- **Removed**: Loop var extraction (10 lines), JoinIR conversion + merge (30 lines)
- **Added**: CommonPatternInitializer call (7 lines), JoinIRConversionPipeline call (8 lines)
- **Net Reduction**: ~25 lines

#### Pattern 3
- **Removed**: Loop var + carrier extraction (22 lines), JoinIR conversion + merge (30 lines)
- **Added**: CommonPatternInitializer call (19 lines - includes carrier extraction), JoinIRConversionPipeline call (8 lines)
- **Net Reduction**: ~25 lines

#### Pattern 4
- **Removed**: Loop var extraction (10 lines), Carrier building (15 lines), JoinIR conversion + merge (30 lines)
- **Added**: CommonPatternInitializer call (7 lines), JoinIRConversionPipeline call (8 lines)
- **Net Reduction**: ~40 lines (Pattern 4 had most duplication)

**Total Estimated Reduction**: ~115 lines across all patterns

### Test Results

All 4 patterns tested successfully:

```bash
=== Pattern 1: Simple While ===
0, 1, 2, RC: 0 ✅

=== Pattern 2: JoinIR Min Loop (with break) ===
RC: 0 ✅

=== Pattern 3: If-Else PHI ===
sum=9, RC: 0 ✅

=== Pattern 4: Then-Continue ===
25, RC: 0 ✅
```

### Quality Improvements

1. **Single Source of Truth**: All initialization logic consolidated
2. **Unified Conversion Flow**: Consistent JoinIR→MIR→Merge pipeline
3. **Reduced Duplication**: Zero duplicate initialization or conversion code
4. **Maintainability**: Future changes only need to happen in 2 places
5. **Testability**: Infrastructure can be tested independently

### Breaking Changes

**None**: This is a pure refactoring with no API changes.

## References

- CommonPatternInitializer: `src/mir/builder/control_flow/joinir/patterns/common_init.rs`
- JoinIRConversionPipeline: `src/mir/builder/control_flow/joinir/patterns/conversion_pipeline.rs`
- Phase 33-23 fix: boundary.loop_var_name setting
- Pattern 1-4: `src/mir/builder/control_flow/joinir/patterns/pattern*.rs`
Status: Historical
