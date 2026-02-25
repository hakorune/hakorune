# Phase 176: Pattern2 Lowerer Multi-Carrier Limitations Report

## Overview

This document identifies all locations in `loop_with_break_minimal.rs` where the Pattern2 lowerer currently only handles the position carrier (`i`) and ignores additional carriers from `CarrierInfo.carriers`.

## Limitation Points

### 1. ValueId Allocation (Line 172-173)

**Location**: ValueId allocation section
**Current Behavior**: Only allocates ValueIds for the position carrier (`i_init`, `i_param`, `i_next`, `i_exit`)
**Missing**: No allocation for additional carriers (e.g., `sum_init`, `sum_param`, `sum_next`, `sum_exit`)

```rust
// TODO(Phase 176): Multi-carrier support - currently only allocates position carrier
// Future: iterate over CarrierInfo.carriers and allocate for each carrier
```

**Impact**: Cannot represent multi-carrier loops in JoinIR local ValueId space.

---

### 2. Main Function Parameters (Line 208-209)

**Location**: `main()` function creation
**Current Behavior**: Only takes `i_init` as parameter
**Missing**: Should take all carrier init values as parameters

```rust
// TODO(Phase 176): Multi-carrier support - main() params should include all carriers
// Future: params = vec![i_init, sum_init, count_init, ...] from CarrierInfo
let mut main_func = JoinFunction::new(main_id, "main".to_string(), vec![i_init]);
```

**Impact**: Additional carriers cannot be passed into the JoinIR fragment.

---

### 3. Loop Step Call Arguments (Line 214-215)

**Location**: `main()` → `loop_step()` call
**Current Behavior**: Only passes `i_init` to `loop_step()`
**Missing**: Should pass all carrier init values

```rust
// TODO(Phase 176): Multi-carrier support - Call args should include all carrier inits
// Future: args = vec![i_init, sum_init, count_init, ...] from CarrierInfo
main_func.body.push(JoinInst::Call {
    func: loop_step_id,
    args: vec![i_init],  // Only position carrier
    k_next: None,
    dst: Some(loop_result),
});
```

**Impact**: Additional carriers lost at loop entry.

---

### 4. Loop Step Function Parameters (Line 234-235)

**Location**: `loop_step()` function creation
**Current Behavior**: Only takes `i_param` as parameter
**Missing**: Should take all carrier params

```rust
// TODO(Phase 176): Multi-carrier support - loop_step params should include all carriers
// Future: params = vec![i_param, sum_param, count_param, ...] from CarrierInfo
let mut loop_step_func = JoinFunction::new(
    loop_step_id,
    "loop_step".to_string(),
    vec![i_param],  // Only position carrier
);
```

**Impact**: Cannot access additional carriers inside loop body.

---

### 5. Natural Exit Jump Arguments (Line 257-258)

**Location**: Natural exit condition → k_exit jump
**Current Behavior**: Only passes `i_param` to k_exit
**Missing**: Should pass all carrier values

```rust
// TODO(Phase 176): Multi-carrier support - Jump args should include all carrier values
// Future: args = vec![i_param, sum_param, count_param, ...] from CarrierInfo
loop_step_func.body.push(JoinInst::Jump {
    cont: k_exit_id.as_cont(),
    args: vec![i_param],  // Only position carrier
    cond: Some(exit_cond),
});
```

**Impact**: Additional carrier values lost at natural exit.

---

### 6. Break Exit Jump Arguments (Line 272-273)

**Location**: Break condition → k_exit jump
**Current Behavior**: Only passes `i_param` to k_exit
**Missing**: Should pass all carrier values

```rust
// TODO(Phase 176): Multi-carrier support - Jump args should include all carrier values
// Future: args = vec![i_param, sum_param, count_param, ...] from CarrierInfo
loop_step_func.body.push(JoinInst::Jump {
    cont: k_exit_id.as_cont(),
    args: vec![i_param],  // Only position carrier
    cond: Some(break_cond_value),
});
```

**Impact**: Additional carrier values lost at break exit.

---

### 7. Loop Body Updates (Line 284-285)

**Location**: Loop body computation
**Current Behavior**: Only computes `i_next = i + 1`
**Missing**: Should compute updates for all carriers

```rust
// TODO(Phase 176): Multi-carrier support - need to compute updates for all carriers
// Future: for each carrier in CarrierInfo.carriers, emit carrier_next = carrier_update
loop_step_func.body.push(JoinInst::Compute(MirLikeInst::BinOp {
    dst: i_next,
    op: BinOpKind::Add,
    lhs: i_param,
    rhs: const_1,
}));
```

**Impact**: Additional carriers cannot be updated in loop body.

**Note**: This is the HARDEST part - we need actual AST body analysis to determine carrier updates!

---

### 8. Tail Call Arguments (Line 304-305)

**Location**: Tail recursion call to `loop_step()`
**Current Behavior**: Only passes `i_next`
**Missing**: Should pass all updated carrier values

```rust
// TODO(Phase 176): Multi-carrier support - tail call args should include all updated carriers
// Future: args = vec![i_next, sum_next, count_next, ...] from CarrierInfo
loop_step_func.body.push(JoinInst::Call {
    func: loop_step_id,
    args: vec![i_next],  // Only position carrier
    k_next: None,
    dst: None,
});
```

**Impact**: Additional carrier updates lost in iteration.

---

### 9. K_Exit Function Parameters (Line 319-320)

**Location**: `k_exit()` function creation (Exit PHI)
**Current Behavior**: Only takes `i_exit` as parameter
**Missing**: Should take all carrier exit values as parameters

```rust
// TODO(Phase 176): Multi-carrier support - k_exit params should include all carrier exits
// Future: params = vec![i_exit, sum_exit, count_exit, ...] from CarrierInfo
let mut k_exit_func = JoinFunction::new(
    k_exit_id,
    "k_exit".to_string(),
    vec![i_exit],  // Only position carrier
);
```

**Impact**: Additional carrier exit values cannot be received by Exit PHI.

---

### 10. ExitMeta Construction (Line 344-345)

**Location**: Final ExitMeta return value
**Current Behavior**: Only includes position carrier in exit bindings
**Missing**: Should include all carrier bindings

```rust
// TODO(Phase 176): Multi-carrier support - ExitMeta should include all carrier bindings
// Future: ExitMeta::multiple(vec![(loop_var_name, i_exit), ("sum", sum_exit), ...])
let exit_meta = ExitMeta::single(loop_var_name.to_string(), i_exit);
```

**Impact**: Additional carriers not visible to MIR merge layer - no carrier PHIs generated!

---

## Summary Statistics

- **Total Limitation Points**: 10
- **Easy Fixes** (iteration over CarrierInfo): 9 points
- **Hard Fix** (requires AST body analysis): 1 point (Loop Body Updates)

## Architecture Note

The CarrierInfo structure is already multi-carrier ready:

```rust
pub struct CarrierInfo {
    pub loop_var_name: String,    // Position carrier
    pub loop_var_id: ValueId,     // Host ValueId
    pub carriers: Vec<CarrierVar>, // Additional carriers (THIS IS IGNORED!)
    pub trim_helper: Option<TrimLoopHelper>,
}
```

The problem is that `lower_loop_with_break_minimal()` completely ignores `CarrierInfo.carriers` and only uses `loop_var_name` (passed as a separate string parameter).

## Next Steps (Phase 176-2/3)

1. **Phase 176-2**: Fix iteration-based points (points 1-6, 8-10)
   - Add carrier iteration logic
   - Extend function params, call args, jump args
   - Build multi-carrier ExitMeta

2. **Phase 176-3**: Fix loop body updates (point 7)
   - Requires AST body analysis
   - Need to track which carriers are updated by which statements
   - Most complex part of multi-carrier support

3. **Integration Test**: Pattern 3 (trim) with Pattern 2 shape
   - Test case: `loop(pos < len) { if ch == ' ' { break } pos = pos + 1 }`
   - Verify sum/count carriers survive through break exits
Status: Historical
