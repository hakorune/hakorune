# Phase 213: Pattern 3 If-Sum AST-based Lowerer

## Overview

Phase 213 implements an AST-based JoinIR lowerer for "simple if-sum" patterns in Pattern 3 (Loop with If-Else PHI).

## Design Decision: Approach B

**Dual-mode architecture**:
1. **if-sum mode**: AST-based lowering for simple patterns (e.g., `phase212_if_sum_min.hako`)
2. **legacy mode**: Hardcoded PoC lowering for existing tests (e.g., `loop_if_phi.hako`)

This approach:
- Minimizes risk by keeping legacy code path intact
- Only generalizes for detected if-sum patterns
- Enables incremental migration

## Implementation

### Files Modified/Created

1. **`loop_update_summary.rs`**
   - Added `is_simple_if_sum_pattern()` method
   - Detects: 1 CounterLike + 1-2 AccumulationLike carriers

2. **`pattern_pipeline.rs`**
   - Added `is_if_sum_pattern()` to PatternPipelineContext
   - Added `extract_if_statement()` helper

3. **`pattern3_with_if_phi.rs`**
   - Dual-mode dispatch: `ctx.is_if_sum_pattern()` → branch
   - `lower_pattern3_if_sum()`: calls AST-based lowerer
   - `lower_pattern3_legacy()`: existing hardcoded logic

4. **`loop_with_if_phi_if_sum.rs`** (NEW)
   - AST-based if-sum lowerer (~420 lines)
   - Extracts from AST:
     - Loop condition (`i < len`)
     - If condition (`i > 0`)
     - Then update (`sum = sum + 1`)
     - Counter update (`i = i + 1`)
   - Generates JoinIR with dynamic values

### Pattern Detection

```rust
pub fn is_simple_if_sum_pattern(&self) -> bool {
    if self.counter_count() != 1 { return false; }
    if self.accumulation_count() < 1 { return false; }
    if self.accumulation_count() > 2 { return false; }
    true
}
```

### AST Extraction

The lowerer extracts pattern components from AST:
- `extract_loop_condition()`: `i < 3` → (var="i", op=Lt, limit=3)
- `extract_if_condition()`: `i > 0` → (var="i", op=Gt, value=0)
- `extract_then_update()`: `sum = sum + 1` → (var="sum", addend=1)
- `extract_counter_update()`: `i = i + 1` → (var="i", step=1)

## Testing Results

### AST Extraction: Working ✅
```
[joinir/pattern3/if-sum] Loop condition: i Lt 3
[joinir/pattern3/if-sum] If condition: i Gt 0
[joinir/pattern3/if-sum] Then update: sum += 1
[joinir/pattern3/if-sum] Counter update: i += 1
```

### Known Issue: Pattern 3 Pipeline

**Both if-sum mode and legacy mode return RC=0 instead of expected values.**

This is a pre-existing issue in the JoinIR → MIR conversion pipeline (Phase 33-21/177):
- Loop back branch targets `bb5` instead of loop header `bb4`
- PHI nodes not properly updated on loop back

**This issue is NOT specific to Phase 213** - the legacy `loop_if_phi.hako` has the same problem.

## Future Work

1. **Phase 214+**: Fix Pattern 3 JoinIR → MIR pipeline
   - Investigate loop back branch target
   - Ensure PHI updates are correctly wired

2. **Phase 213-B** (optional): Support variable limits
   - Currently only integer literals supported
   - `i < len` where `len` is a variable → Phase 214+

## File Locations

- Lowerer: `src/mir/join_ir/lowering/loop_with_if_phi_if_sum.rs`
- Dispatcher: `src/mir/builder/control_flow/joinir/patterns/pattern3_with_if_phi.rs`
- Pattern detection: `src/mir/join_ir/lowering/loop_update_summary.rs`
- Pipeline context: `src/mir/builder/control_flow/joinir/patterns/pattern_pipeline.rs`
Status: Active  
Scope: If-sum 実装（JoinIR v2）
