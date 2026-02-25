Status: VerificationReport, Historical

# Phase 166 Validation Report: JsonParserBox Unit Test with BoolExprLowerer

**Date**: 2025-12-06 (Updated: 2025-12-07 Phase 170)
**Status**: ⚠️ **Blocked** - ValueId boundary mapping issue
**Blocker**: Condition variables not included in JoinInlineBoundary

**Phase 170 Update**: BoolExprLowerer is now integrated (Phase 167-169), but a critical ValueId boundary mapping bug prevents runtime execution. See [phase170-valueid-boundary-analysis.md](phase170-valueid-boundary-analysis.md) for details.

---

## Phase 170 Re-validation Results (2025-12-07)

After Phase 167-169 (BoolExprLowerer integration), Phase 170 re-tested JsonParserBox with the following results:

### ✅ Whitelist Expansion Complete
- Added 6 JsonParserBox methods to routing whitelist
- Methods now route to JoinIR instead of `[joinir/freeze]`
- Pattern matching works correctly (Pattern2 detected for `_trim`)

### ⚠️ Runtime Failure: ValueId Boundary Issue
**Test**: `local_tests/test_trim_main_pattern.hako`
**Pattern Matched**: Pattern2 (twice, for 2 loops)
**Result**: Silent runtime failure (no output)

**Root Cause**: Condition variables (`start`, `end`) are resolved from HOST `variable_map` but not included in `JoinInlineBoundary`, causing undefined ValueId references.

**Evidence**:
```
[ssa-undef-debug] fn=TrimTest.trim/1 bb=BasicBlockId(12) inst_idx=0 used=ValueId(33)
[ssa-undef-debug] fn=TrimTest.trim/1 bb=BasicBlockId(12) inst_idx=0 used=ValueId(34)
```

**Solution**: Option A in [phase170-valueid-boundary-analysis.md](phase170-valueid-boundary-analysis.md) - Extract condition variables and add to boundary.

---

## Executive Summary (Original Phase 166)

Phase 166 aimed to validate that JsonParserBox can parse JSON through the JoinIR path, confirming Pattern1-4 support. However, investigation revealed that:

1. **✅ JoinIR Pattern Detection Works**: Pattern 2 (break) correctly detected ← **Still true in Phase 170**
2. **✅ Simple JSON Parsing Works**: Non-loop or simple-condition patterns execute fine
3. **~~❌ Complex Conditions Blocked~~** ← **FIXED in Phase 169**: BoolExprLowerer integrated
4. **❌ NEW BLOCKER (Phase 170)**: ValueId boundary mapping prevents runtime execution

---

## Test Results

### ✅ Test 1: Simple JSON Parser (No Loops)
**File**: `local_tests/test_json_parser_simple_string.hako`

```bash
./target/release/hakorune local_tests/test_json_parser_simple_string.hako
# Output: PASS: Got 'hello'
```

**Result**: **SUCCESS** - Basic string parsing without complex conditions works.

---

### ❌ Test 2: _trim Pattern with OR Chains
**File**: `local_tests/test_trim_or_pattern.hako`

```bash
./target/release/hakorune local_tests/test_trim_or_pattern.hako
# Output: [joinir/freeze] Loop lowering failed
```

**Result**: **BLOCKED** - OR condition causes `[joinir/freeze]` error.

---

### ⚠️ Test 3: _trim Pattern in main() with Simple Condition
**File**: `local_tests/test_trim_main_pattern.hako`

```bash
./target/release/hakorune local_tests/test_trim_main_pattern.hako
# Output: FAIL - Result: '  hello  ' (not trimmed)
```

**Result**: **PATTERN DETECTED BUT LOGIC WRONG** - Pattern 2 matches, but uses hardcoded `i < 3` instead of actual condition.

**Debug Output**:
```
[trace:pattern] route: Pattern2_WithBreak MATCHED
[trace:varmap] pattern2_start: end→r9, s→r4, start→r6
Final start: 0  (unchanged - loop didn't execute properly)
```

---

## Root Cause Analysis

### Discovery 1: Function Name Whitelisting

**File**: `src/mir/builder/control_flow/joinir/routing.rs` (lines 44-68)

JoinIR is **ONLY enabled for specific function names**:
- `"main"`
- `"JoinIrMin.main/0"`
- `"JsonTokenizer.print_tokens/0"`
- `"ArrayExtBox.filter/2"`

**Impact**: `JsonParserBox._trim/1` is NOT whitelisted → `[joinir/freeze]` error.

**Workaround**: Test in `main()` function instead.

---

### Discovery 2: Hardcoded Conditions in Minimal Lowerers

**File**: `src/mir/join_ir/lowering/loop_with_break_minimal.rs` (lines 171-197)

```rust
// HARDCODED: !(i < 3)
loop_step_func.body.push(JoinInst::Compute(MirLikeInst::Const {
    dst: const_3,
    value: ConstValue::Integer(3),  // ← HARDCODED VALUE
}));

loop_step_func.body.push(JoinInst::Compute(MirLikeInst::Compare {
    dst: cmp_lt,
    op: CompareOp::Lt,           // ← HARDCODED OPERATOR
    lhs: i_param,
    rhs: const_3,
}));
```

**Impact**: Pattern 2 lowerer generates fixed `i < 3` check, **ignoring the actual AST condition**.

**Current Behavior**:
- AST condition: `start < end` with `ch == " "` check
- Generated JoinIR: `i < 3` with `i >= 2` break check
- Result: Loop doesn't execute correctly

---

### Discovery 3: BoolExprLowerer Not Integrated

**Files**:
- `src/mir/join_ir/lowering/bool_expr_lowerer.rs` (Phase 167-168, 436 lines, complete)
- `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs` (line 58)

```rust
// Current code:
let loop_var_name = self.extract_loop_variable_from_condition(condition)?;

// Missing:
// use crate::mir::join_ir::lowering::bool_expr_lowerer::BoolExprLowerer;
// let mut bool_lowerer = BoolExprLowerer::new(self.builder);
// let cond_val = bool_lowerer.lower_condition(&ctx.condition)?;
```

**Impact**: BoolExprLowerer exists but isn't called by Pattern 2/4 lowerers.

---

### Discovery 4: LoopBuilder Hard Freeze

**File**: `src/mir/builder/control_flow/mod.rs` (lines 112-119)

```rust
// Phase 186: LoopBuilder Hard Freeze - Legacy path disabled
// Phase 187-2: LoopBuilder module removed - all loops must use JoinIR
return Err(format!(
    "[joinir/freeze] Loop lowering failed: JoinIR does not support this pattern, and LoopBuilder has been removed.\n\
     Function: {}\n\
     Hint: This loop pattern is not supported. All loops must use JoinIR lowering.",
    self.current_function.as_ref().map(|f| f.signature.name.as_str()).unwrap_or("<unknown>")
));
```

**Impact**: NO fallback exists when JoinIR patterns don't match.

---

## Architecture Issues

### Issue 1: Minimal Lowerers Are Test-Specific

**Design**: Pattern 1-4 lowerers are "minimal implementations" for specific test cases:
- Pattern 1: `apps/tests/joinir_simple_loop.hako` (`i < 5`)
- Pattern 2: `apps/tests/joinir_min_loop.hako` (`i < 3`, `i >= 2`)
- Pattern 3: `apps/tests/loop_if_phi_sum.hako` (hardcoded sum accumulation)
- Pattern 4: `apps/tests/loop_continue_pattern4.hako` (hardcoded continue logic)

**Problem**: These lowerers are **NOT** generic - they can't handle arbitrary conditions.

---

### Issue 2: Condition Extraction vs. Evaluation

**Current**:
- `extract_loop_variable_from_condition()` - Extracts variable name (`i`, `start`)
- Used for: Carrier detection, not condition evaluation
- Only supports: Simple comparisons like `i < 3`

**Missing**:
- Dynamic condition evaluation (BoolExprLowerer)
- OR chain support
- Complex boolean expressions

---

### Issue 3: JoinIR Generation Architecture

**Current Pipeline**:
```
AST Loop → Pattern Detection → Hardcoded JoinIR Generator
                                     ↓
                             Fixed condition (i < 3)
```

**Needed Pipeline**:
```
AST Loop → Pattern Detection → BoolExprLowerer → Dynamic JoinIR Generator
                                      ↓                    ↓
                             Condition MIR → Convert to JoinInst
```

---

## Phase 166 Status Update

### ✅ Completed Validation
1. **Pattern Detection**: Pattern 2 (break) correctly identified
2. **Simple Cases**: Non-loop JSON parsing works
3. **Infrastructure**: JoinIR pipeline functional
4. **Whitelist Behavior**: Function name routing confirmed

### ❌ Remaining Blockers
1. **OR Chains**: `ch == " " || ch == "\t"...` not supported
2. **Dynamic Conditions**: Hardcoded `i < 3` instead of actual condition
3. **BoolExprLowerer Integration**: Phase 167-168 code not used
4. **JsonParserBox._trim**: Cannot execute due to whitelisting

---

## Recommended Next Steps

### Phase 169: BoolExprLowerer Integration (HIGH PRIORITY)

**Goal**: Make JoinIR patterns support arbitrary conditions.

**Tasks**:
1. **Modify Pattern 2 Lowerer** (`loop_with_break_minimal.rs`):
   - Accept `condition: &ASTNode` parameter
   - Call `BoolExprLowerer::lower_condition(condition)`
   - Generate JoinIR instructions from condition MIR
   - Replace hardcoded `const_3`, `cmp_lt` with dynamic values

2. **Modify Pattern 4 Lowerer** (`loop_with_continue_minimal.rs`):
   - Same changes as Pattern 2

3. **Update Caller** (`pattern2_with_break.rs`):
   - Pass `ctx.condition` to lowerer
   - Handle condition evaluation errors

4. **Test Coverage**:
   - `_trim` pattern with OR chains
   - Complex boolean expressions
   - Nested conditions

**Estimated Effort**: 2-3 hours (architecture already designed in Phase 167-168)

---

### Phase 170: Function Whitelist Expansion (MEDIUM PRIORITY)

**Goal**: Enable JoinIR for JsonParserBox methods.

**Options**:
1. **Option A**: Add to whitelist:
   ```rust
   "JsonParserBox._trim/1" => true,
   "JsonParserBox._skip_whitespace/2" => true,
   ```

2. **Option B**: Enable JoinIR globally for all functions:
   ```rust
   let is_target = true;  // Always try JoinIR first
   ```

3. **Option C**: Add pattern-based routing (e.g., all `_trim*` functions)

**Recommended**: Option A (conservative, safe)

---

### Phase 171: JsonParserBox Full Validation (POST-169)

**Goal**: Validate all JsonParserBox methods work through JoinIR.

**Tests**:
- `_trim` (OR chains)
- `_skip_whitespace` (OR chains)
- `_parse_number` (digit loop)
- `_parse_string` (escape sequences)
- `_parse_array` (recursive calls)
- `_parse_object` (key-value pairs)

---

## Files Modified This Session

### Created Test Files
1. `local_tests/test_json_parser_simple_string.hako` - Simple JSON test (PASS)
2. `local_tests/test_trim_or_pattern.hako` - OR chain test (BLOCKED)
3. `local_tests/test_trim_simple_pattern.hako` - Simple condition test (BLOCKED)
4. `local_tests/test_trim_main_pattern.hako` - Whitelisted function test (WRONG LOGIC)
5. `local_tests/test_trim_debug.hako` - Debug output test

### Documentation
1. `docs/development/current/main/phase166-validation-report.md` (this file)

---

## Conclusion

**Phase 166 Validation Status**: ⚠️ **Partially Complete**

**Key Findings**:
1. JoinIR Pattern Detection **works correctly**
2. Simple patterns **execute successfully**
3. Complex OR chains **are blocked** by hardcoded conditions
4. BoolExprLowerer (Phase 167-168) **exists but isn't integrated**

**Next Critical Phase**: **Phase 169 - BoolExprLowerer Integration** to unblock JsonParserBox._trim and enable dynamic condition evaluation.

**Timeline**:
- Phase 169: 2-3 hours (integration work)
- Phase 170: 30 minutes (whitelist update)
- Phase 171: 1 hour (full validation testing)

**Total Estimated Time to Complete Phase 166**: 4-5 hours

---

## References

- **Phase 166 Goal**: `docs/development/current/main/phase166-joinir-json-parser-validation.md`
- **Phase 167-168**: `src/mir/join_ir/lowering/bool_expr_lowerer.rs`
- **Pattern 2 Lowerer**: `src/mir/join_ir/lowering/loop_with_break_minimal.rs`
- **Routing Logic**: `src/mir/builder/control_flow/joinir/routing.rs`
- **LoopBuilder Freeze**: `src/mir/builder/control_flow/mod.rs` (lines 112-119)
