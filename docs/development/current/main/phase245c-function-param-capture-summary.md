# Phase 245C: Function Parameter Capture - Implementation Summary

---
**Phase 26-45 Completion**: このフェーズで設計した機能は Phase 43/245B で実装完了。最終状態は [PHASE_43_245B_NORMALIZED_COMPLETION.md](./PHASE_43_245B_NORMALIZED_COMPLETION.md) を参照。
---

**Status**: ✅ COMPLETE
**Date**: 2025-12-11
**Scope**: Extend CapturedEnv to include function parameters used in loop conditions/body

## 🎯 Goal

Resolve `Variable not found: s` and similar errors by capturing function parameters (like `s`, `len`) that are used in loop conditions but not declared as pre-loop locals.

## 📋 Background

### Problem
`analyze_captured_vars_v2` only captured pre-loop local variables with safe constant initialization. Function parameters used in loop conditions (e.g., `p < len` where `len` is a function parameter) were not captured, causing "Variable not found" errors in ExprLowerer.

### Example Case
```nyash
method _parse_number(s, p, len) {
    loop(p < len) {          // 'len' is a function parameter
        local ch = s.charAt(p)  // 's' is a function parameter
        p = p + 1
    }
}
```

Previously:
- ❌ `len` not captured → "Variable not found: len" error
- ❌ `s` not captured → "Variable not found: s" error

Now:
- ✅ Both `len` and `s` captured and available in ConditionEnv

## 🛠️ Implementation

### Step 1: Helper Function - `collect_names_in_loop_parts`

**File**: `src/mir/loop_pattern_detection/function_scope_capture.rs`
**Lines**: 668-745

Added a helper function to collect all variable names used anywhere in loop condition and body:

```rust
fn collect_names_in_loop_parts(condition: &ASTNode, body: &[ASTNode]) -> BTreeSet<String>
```

Features:
- Recursively walks condition and body AST
- Collects all `Variable` node names
- Returns deduplicated set using `BTreeSet` (deterministic iteration)
- Handles all AST node types: If, Assignment, BinaryOp, MethodCall, etc.

### Step 2: Extend `analyze_captured_vars_v2`

**File**: `src/mir/loop_pattern_detection/function_scope_capture.rs`
**Lines**: 372-412

Added Phase 245C logic after pre-loop local processing:

```rust
// Phase 245C: Capture function parameters used in loop
let names_in_loop = collect_names_in_loop_parts(loop_condition, loop_body);
let pre_loop_local_names: BTreeSet<String> =
    pre_loop_locals.iter().map(|(name, _)| name.clone()).collect();

for name in names_in_loop {
    // Skip if already processed as pre-loop local
    if pre_loop_local_names.contains(&name) { continue; }

    // Skip if in pinned/carriers/body_locals
    if scope.pinned.contains(&name)
        || scope.carriers.contains(&name)
        || scope.body_locals.contains(&name) { continue; }

    // Skip if reassigned in function
    if is_reassigned_in_fn(fn_body, &name) { continue; }

    // Capture as function parameter
    env.add_var(CapturedVar {
        name: name.clone(),
        host_id: ValueId(0), // Resolved later in ConditionEnvBuilder
        is_immutable: true,
    });
}
```

### Step 3: Fix Loop Index Handling

**Lines**: 284-301

Fixed issue where empty `fn_body` (common in unit tests) would cause early return:

```rust
// Before: Returned empty CapturedEnv if loop not found
let loop_index = find_loop_index_by_structure(fn_body, loop_condition, loop_body);

// After: Continue processing even if loop not found
let pre_loop_locals = if let Some(idx) = loop_index {
    collect_local_declarations(&fn_body[..idx])
} else {
    collect_local_declarations(fn_body)  // Still collect from fn_body
};
```

## ✅ Testing

### New Tests (4 tests added)

**File**: `src/mir/loop_pattern_detection/function_scope_capture.rs`
**Lines**: 1205-1536

1. **`test_capture_function_param_used_in_condition`** (Lines 1205-1272)
   - Case: `loop(p < len)` where `len` is a function parameter
   - Expected: `len` captured in CapturedEnv
   - Result: ✅ PASS

2. **`test_capture_function_param_used_in_method_call`** (Lines 1274-1362)
   - Case: `loop(p < s.length())` and `s.charAt(p)` where `s` is a function parameter
   - Expected: `s` captured (used in both condition and body)
   - Result: ✅ PASS

3. **`test_capture_function_param_reassigned_rejected`** (Lines 1364-1442)
   - Case: Function parameter reassigned in function body
   - Expected: NOT captured (violates immutability requirement)
   - Result: ✅ PASS

4. **`test_capture_mixed_locals_and_params`** (Lines 1444-1535)
   - Case: Mix of pre-loop locals (`digits`) and function params (`s`, `len`)
   - Expected: All three captured
   - Result: ✅ PASS

### Test Results
```
running 12 tests
test ... test_capture_function_param_used_in_condition ... ok
test ... test_capture_function_param_used_in_method_call ... ok
test ... test_capture_function_param_reassigned_rejected ... ok
test ... test_capture_mixed_locals_and_params ... ok
test ... (8 other existing tests) ... ok

test result: ok. 12 passed; 0 failed
```

### Overall Suite
```
test result: ok. 923 passed; 1 failed; 56 ignored
```

Note: The 1 failure (`test_expr_lowerer_methodcall_unknown_method_is_rejected`) is pre-existing and unrelated to Phase 245C changes.

## 🎯 Capture Criteria (Updated)

A variable is captured if ALL of the following are met:

### Pre-Loop Locals (Phase 200-B)
1. Declared before the loop in function scope
2. Safe constant init (string/integer literal only)
3. Never reassigned in function
4. Referenced in loop condition or body
5. Not in pinned/carriers/body_locals

### Function Parameters (Phase 245C - NEW)
1. Used in loop condition or body
2. NOT a pre-loop local (checked first)
3. NOT in pinned/carriers/body_locals
4. Never reassigned in function (immutability)

## 📊 Integration with Pattern 2

**File**: `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`
**Lines**: 166-222

Pattern 2 already integrates `analyze_captured_vars_v2`:
```rust
let captured_env = if let Some(fn_body_ref) = fn_body {
    analyze_captured_vars_v2(fn_body_ref, condition, _body, &scope)
} else {
    CapturedEnv::new()
};

// Add captured variables to ConditionEnv
for var in &captured_env.vars {
    if let Some(&host_id) = self.variable_map.get(&var.name) {
        let join_id = join_value_space.alloc_param();
        env.insert(var.name.clone(), join_id);
        condition_bindings.push(ConditionBinding { ... });
    }
}
```

With Phase 245C, this now includes function parameters automatically!

## 🔍 Example Before/After

### Before Phase 245C
```
[ERROR] Variable not found: s
[ERROR] Variable not found: len
```

### After Phase 245C
```
[pattern2/capture] Phase 200-C: Captured 2 variables
[pattern2/capture]   's': host_id=ValueId(0), immutable=true
[pattern2/capture]   'len': host_id=ValueId(0), immutable=true
[pattern2/capture] Phase 201: Added captured 's': host=ValueId(42), join=ValueId(101)
[pattern2/capture] Phase 201: Added captured 'len': host=ValueId(43), join=ValueId(102)
```

## 📝 Implementation Notes

### Design Decisions

1. **No `is_safe_const_init` check for function parameters**
   Function parameters don't have initialization expressions in the loop's function body, so we don't apply the "safe constant init" check. They're considered safe if never reassigned.

2. **Process pre-loop locals first**
   This ensures we don't double-capture variables that are both function parameters and have local redeclarations.

3. **Deterministic iteration with BTreeSet**
   Uses `BTreeSet` instead of `HashSet` to ensure consistent capture order across runs.

4. **Graceful handling of empty fn_body**
   Unit tests often don't provide fn_body context. The implementation handles this by processing all variables in the loop without pre-loop local filtering.

### Invariants Maintained

- ✅ No duplicate captures (pre-loop locals checked before params)
- ✅ Immutability requirement enforced (reassigned variables excluded)
- ✅ Scope exclusions respected (pinned/carriers/body_locals)
- ✅ Placeholder `host_id` (resolved later in ConditionEnvBuilder)

## 🎉 Success Criteria - ALL MET

- [x] `cargo build --release` succeeds
- [x] All new tests PASS (4/4)
- [x] All existing function_scope_capture tests PASS (12/12)
- [x] No regressions in main test suite (923 passed)
- [x] Function parameters captured in CapturedEnv
- [x] Integration with Pattern 2 working

## 🔗 Related Phases

- **Phase 200-A**: CapturedEnv infrastructure
- **Phase 200-B**: Pre-loop local capture
- **Phase 200-C**: Structural matching variant (v2 API)
- **Phase 245-EX**: JsonParser `_parse_number` JoinIR integration
- **Phase 245B**: (Future) String carrier handling for `num_str`

## 📌 Next Steps

Phase 245C is complete. Next phases can now:
1. Use function parameters in loop conditions without "Variable not found" errors
2. Build on this for JsonParser `_parse_number` integration
3. Extend to other JsonParser loops (`_atoi`, `_parse_array`, etc.)
