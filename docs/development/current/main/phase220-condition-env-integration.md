# Phase 220-B: ConditionEnv Integration for If-Sum

## Overview
Integrated ConditionEnv infrastructure into if-sum lowerer to support
variable conditions like `loop(i < len)`.

## Implementation Status

### Completed
- ✅ ConditionEnv construction in `lower_if_sum_pattern()`
- ✅ `extract_value_or_variable()` function (supports both literals and variables)
- ✅ Updated `extract_loop_condition()` and `extract_if_condition()` to use ConditionEnv
- ✅ JoinIR generation includes condition-only variables as parameters
- ✅ Condition bindings wired to boundary builder
- ✅ Updated call sites in `pattern3_with_if_phi.rs`

### In Progress
- 🔧 **Issue**: phase212_if_sum_min.hako returns RC=0 instead of RC=2
- 🔧 **Root Cause**: Condition variable (`len`) remapping issue during merge
  - HOST ValueId(6) → JoinIR ValueId(101) → After merge ValueId(10) ❌
  - Expected: HOST ValueId(6) → JoinIR ValueId(101) → After merge ValueId(6) ✅
- 🔧 **Next Step**: Investigate condition_bindings handling in merge pipeline

## Implementation

### Pattern 2 Reference
Followed proven Pattern 2 (break condition) integration pattern:
1. Build ConditionEnv early with extract_condition_variables
2. Pass to condition lowering functions
3. Lookup variables via cond_env.get()
4. Wire condition_bindings to JoinInlineBoundary

### Key Changes

**File**: `src/mir/join_ir/lowering/loop_with_if_phi_if_sum.rs`
- Added ConditionEnv construction (lines 70-127)
- Updated signature to accept `variable_map`, `loop_var_name`, `loop_var_id`
- Return type changed to include `Vec<ConditionBinding>`
- JoinIR generation updated to include condition variables as parameters

**File**: `src/mir/builder/control_flow/joinir/patterns/pattern3_with_if_phi.rs`
- Updated call site to pass variable_map and loop context
- Wired condition_bindings to boundary builder (line 189)

### ValueOrLiteral Enum
```rust
enum ValueOrLiteral {
    Literal(i64),
    Variable(String, ValueId),
}
```

### Function Signatures Updated
```rust
fn extract_value_or_variable(
    node: &ASTNode,
    cond_env: &ConditionEnv,
) -> Result<ValueOrLiteral, String>

fn extract_loop_condition(
    cond: &ASTNode,
    cond_env: &ConditionEnv,
) -> Result<(String, CompareOp, ValueOrLiteral), String>
```

## Test Results (Preliminary)

### phase212_if_sum_min.hako
- Expected: RC=2
- Actual: RC=0 ❌
- Issue: Loop not executing (no print output)
- Debug: Condition variable `len` extracted correctly but remapping issue

### Debug Trace
```
[joinir/pattern3/if-sum] Extracted 1 condition variables: ["len"]
[joinir/pattern3/if-sum] Condition variable 'len': host=ValueId(6), join=ValueId(101)
[if-sum/joinir] loop_step params: 4 total (3 carriers + 1 condition vars)
[DEBUG-177]   'len': JoinIR ValueId(101) → Some(ValueId(10))  ← Should be ValueId(6)
```

## Design Principles
1. **Reuse Existing Boxes** (ConditionEnv/ConditionBinding)
2. **Follow Pattern 2 Structure** (proven blueprint)
3. **Fail-Fast** (variable not in ConditionEnv → error)
4. **ParamRole::Condition Routing** (separate from carriers)

## Next Steps

1. **Investigate Merge Pipeline**:
   - Check how condition_bindings are processed in JoinIRConversionPipeline
   - Verify that condition variables are loaded at loop entry
   - Ensure remapping uses host_value from ConditionBinding

2. **Add Load Instructions**:
   - May need to generate Load instructions for condition variables at loop entry
   - Similar to how Pattern 2 handles break condition variables

3. **Test Additional Cases**:
   - phase218_json_if_sum_min.hako (expect RC=10)
   - Regression: loop_if_phi.hako (expect RC=2)
   - Regression: phase217_if_sum_multi_min.hako (expect RC=2)

## Files Modified

**Primary**:
- `src/mir/join_ir/lowering/loop_with_if_phi_if_sum.rs` (ConditionEnv integration)
- `src/mir/builder/control_flow/joinir/patterns/pattern3_with_if_phi.rs` (call site update)

**Documentation**:
- `docs/development/current/main/phase220-condition-env-integration.md` (this file)

## Important Notes

- **Fail-Fast Maintained**: Variables not in ConditionEnv cause explicit errors
- **Method Call Limitation**: `defs.len()` style not supported yet (Phase 221+)
- **Pattern 2 Parity**: Uses identical ConditionEnvBuilder API
- **ParamRole Separation**: Condition variables kept separate from carriers

## Known Issues

1. **Condition Variable Remapping** (Critical):
   - Condition variables get remapped to wrong ValueIds during merge
   - Need to verify condition_bindings handling in merge pipeline

2. **No Test Passing Yet**:
   - phase212: RC=0 (expected 2) ❌
   - Other tests not yet verified

## References

- [Phase 171-fix: ConditionEnv](../../../reference/joinir/condition-env.md)
- [Phase 200-A/B: ConditionBinding](../../../reference/joinir/condition-binding.md)
- [Phase 201: JoinValueSpace](../../../reference/joinir/join-value-space.md)
- [Pattern 2: Break Condition](./pattern2-break-condition.md)
Status: Active  
Scope: ConditionEnv 統合（JoinIR v2）
