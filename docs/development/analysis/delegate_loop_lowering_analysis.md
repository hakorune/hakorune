# Delegate Loop Lowering Analysis

## Executive Summary

**Root Cause**: The delegate path loop lowering issue is **NOT** a bug in the Rust lowering code (`src/runner/json_v0_bridge/lowering/loop_.rs`). The actual problem is in the **Stage-B self-hosting compiler** (`lang/src/compiler/entry/compiler_stageb.hako` and `lang/src/compiler/parser/parser_box.hako`) which produces malformed Program JSON v0.

**Status**: The Rust delegate lowering code is correct. The Stage-B parser is producing incorrect output.

## Problem Description

### Test Case
```hako
static box Main { method main(){
  local n=10; local i=0;
  loop(i<n){ i=i+1 }
  return i
} }
```

**Expected**: Returns 10
**Actual (delegate)**: Returns 0
**Actual (FORCE)**: Returns 10 ✅

## Investigation Findings

### 1. Malformed Program JSON v0

The Stage-B compiler produces this Program JSON:

```json
{
  "body": [
    {"type":"Local","name":"n","expr":{"type":"Int","value":10}},
    {"type":"Local","name":"i","expr":{"type":"Int","value":0}},
    {"type":"Loop","cond":{"type":"Compare","op":"<","lhs":{"type":"Var","name":"i"},"rhs":{"type":"Int","value":0}},"body":[]},
    ...
  ]
}
```

**Two Critical Bugs**:
1. **Empty loop body**: `"body":[]` instead of `[{"type":"Local","name":"i","expr":{"type":"Binary",...}}]`
2. **Wrong condition**: Compares `i < 0` instead of `i < n` (rhs is `{"type":"Int","value":0}` instead of `{"type":"Var","name":"n"}`)

### 2. Delegate MIR Structure (Incorrect)

The delegate path produces this MIR:

**Block 1 (Header)**:
```json
{
  "op": "phi", "dst": 3, "incoming": [[2,0],[2,2]]  // Wrong: both from reg 2
},
{
  "op": "phi", "dst": 5, "incoming": [[4,0],[4,2]]  // Wrong: both from reg 4
},
{
  "op": "compare", "operation": "<", "lhs": 5, "rhs": 6  // Wrong: compares i < 0
}
```

**Block 2 (Body)**:
```json
{
  "op": "jump", "target": 1  // Empty body - no i=i+1!
}
```

### 3. FORCE MIR Structure (Correct)

The FORCE path (using selfhost-first with JsonFrag) produces correct MIR:

**Block 1 (Header)**:
```json
{
  "op": "phi", "dst": 6, "incoming": [[2,0],[6,2]]  // Correct: n from preheader/itself
},
{
  "op": "phi", "dst": 3, "incoming": [[1,0],[5,2]]  // Correct: i from 0/updated
},
{
  "op": "compare", "operation": "<", "lhs": 3, "rhs": 6  // Correct: i < n
}
```

**Block 2 (Body)**:
```json
{
  "op": "const", "dst": 10, "value": {"type":"i64","value":1}
},
{
  "op": "binop", "operation": "+", "lhs": 3, "rhs": 10, "dst": 5  // Correct: i+1
},
{
  "op": "jump", "target": 1
}
```

## Rust Delegate Lowering Code Analysis

The Rust lowering code in `src/runner/json_v0_bridge/lowering/loop_.rs` is **CORRECT**:

1. **Line 109-111**: Correctly prepares loop PHIs with preheader seeds
2. **Line 115-116**: Correctly lowers condition and sets up branch
3. **Line 117-123**: Correctly clones vars and lowers body
4. **Line 133**: Correctly saves `body_vars` after body execution
5. **Line 145-152**: Correctly seals PHIs with latch values

The code correctly implements:
- PHI preparation with preheader copies
- Body variable tracking
- PHI sealing with latch values
- Exit PHI generation

**The problem is garbage-in, garbage-out**: When the Program JSON has an empty body and wrong condition, the lowering correctly processes that incorrect input.

## Verification

### Test Results

**FORCE Path** (✅ Works):
```bash
HAKO_SELFHOST_BUILDER_FIRST=1 \
HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG=1 \
bash tools/hakorune_emit_mir.sh /tmp/loop_min.hako /tmp/loop_min_force.json

# Result: MIR with correct structure, EXE returns 10
```

**Delegate Path** (❌ Broken):
```bash
NYASH_JSON_ONLY=1 \
bash tools/hakorune_emit_mir.sh /tmp/loop_min.hako /tmp/loop_min_delegate.json

# Result: MIR with empty body and wrong condition, EXE returns 0
```

## Root Cause Location

The bug is in the **Stage-B self-hosting compiler**:

**Entry Point**: `lang/src/compiler/entry/compiler_stageb.hako` line 341
```hako
local ast_json = p.parse_program2(body_src)
```

**Parser**: `lang/src/compiler/parser/parser_box.hako`
- Likely in loop parsing logic
- Incorrectly handles loop body extraction
- Incorrectly handles loop condition parsing

## Recommendations

### Short-Term (Immediate)

1. **Use FORCE path for production**:
   ```bash
   export HAKO_SELFHOST_BUILDER_FIRST=1
   export HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG=1
   ```

2. **Document delegate path limitation**:
   - Add warning in tools/hakorune_emit_mir.sh
   - Update phase documentation

### Medium-Term (Fix)

1. **Debug Stage-B parser**:
   - Add instrumentation to `parser_box.hako`
   - Trace loop parsing logic
   - Fix body extraction and condition parsing

2. **Add Stage-B tests**:
   - Create test suite for Program JSON v0 output
   - Include loop test cases
   - Verify against expected JSON structure

### Long-Term (Architecture)

1. **Phase out Stage-B for critical paths**:
   - Keep FORCE path as primary
   - Use delegate only for verified constructs
   - Consider Rust-based parser for reliability

2. **Improve JsonFrag robustness**:
   - The FORCE path already works correctly
   - Focus optimization efforts there

## Conclusion

**The delegate loop lowering code is correct**. The bug is upstream in the Stage-B self-hosting compiler which produces malformed Program JSON v0. The FORCE path works because it bypasses the buggy Stage-B parser and uses the JsonFrag-based MirBuilder implementation.

**Immediate Action**: Use FORCE path (`HAKO_SELFHOST_BUILDER_FIRST=1`) for all loop-related development and testing until the Stage-B parser is fixed.

## Files Analyzed

- ✅ `src/runner/json_v0_bridge/lowering/loop_.rs` - Correct implementation
- ✅ `src/mir/phi_core/loop_phi.rs` - Correct PHI management
- ❌ `lang/src/compiler/entry/compiler_stageb.hako` - Calls buggy parser
- ❌ `lang/src/compiler/parser/parser_box.hako` - Contains loop parsing bug
- ✅ `tools/hakorune_emit_mir.sh` - Script wrapper (works as designed)

---

**Date**: 2025-11-11
**Analyst**: Claude (Sonnet 4.5)
**Context**: Phase 21.5 Delegate Loop Lowering Investigation
