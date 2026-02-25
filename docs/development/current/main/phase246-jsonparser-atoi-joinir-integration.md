# Phase 246-EX: JsonParser _atoi JoinIR Integration

## 0. Executive Summary

**Goal**: Integrate JsonParser's `_atoi` function into the JoinIR loop lowering system using the existing NumberAccumulation infrastructure.

**Status**: Step 0 - Infrastructure confirmed, design document created

**Key Finding**: ✅ `UpdateRhs::NumberAccumulation` already exists (Phase 190) and fully supports the `result = result * 10 + digit_pos` pattern!

---

## 1. _atoi Loop Structure Analysis

### Source Code (from `apps/json/jsonparser.hako`)

```nyash
box JsonParser {
    method _atoi(s, len) {
        local result = 0
        local digits = "0123456789"
        local i = 0

        loop(i < len) {
            local ch = s.substring(i, i + 1)
            local digit_pos = digits.indexOf(ch)
            if digit_pos < 0 { break }
            result = result * 10 + digit_pos
            i = i + 1
        }

        return result
    }
}
```

### Loop Components

#### Loop Header
- **Condition**: `i < len`
- **Type**: Simple comparison (supported by ExprLowerer)

#### Loop Variables
| Variable | Type | Role | Initial Value |
|----------|------|------|---------------|
| `i` | position | LoopState (counter) | 0 |
| `result` | accumulator | LoopState (number accumulation) | 0 |

#### Loop Body
1. **Local declarations**:
   - `ch = s.substring(i, i + 1)` - current character
   - `digit_pos = digits.indexOf(ch)` - digit value (-1 if not digit)

2. **Break condition**:
   - `if digit_pos < 0 { break }` - exit on non-digit character

3. **Updates**:
   - `result = result * 10 + digit_pos` - number accumulation (NumberAccumulation pattern)
   - `i = i + 1` - position increment (Const increment)

#### Captured Variables (Function Parameters)
- `s` - input string to parse
- `len` - length of string to process
- `digits` - pre-loop local (digit lookup string "0123456789")

---

## 2. Pattern Classification

### Pattern Type: **Pattern 2 (Break)**

**Rationale**:
- Single if-break structure
- No continue statements
- Early exit condition (non-digit character)
- Two carriers with different update patterns

### Carriers

| Carrier | Role | Update Pattern | UpdateExpr Variant |
|---------|------|----------------|-------------------|
| `i` | Loop counter | `i = i + 1` | `UpdateExpr::Const(1)` |
| `result` | Number accumulator | `result = result * 10 + digit_pos` | `UpdateExpr::BinOp { rhs: NumberAccumulation { base: 10, digit_var: "digit_pos" } }` |

---

## 3. UpdateExpr Infrastructure Confirmation (Phase 190)

### 3.1 Existing NumberAccumulation Support

**Location**: `/home/tomoaki/git/hakorune-selfhost/src/mir/join_ir/lowering/loop_update_analyzer.rs`

#### UpdateRhs Enum (lines 46-64)
```rust
pub enum UpdateRhs {
    Const(i64),
    Variable(String),
    StringLiteral(String),
    /// Phase 190: Number accumulation pattern: result = result * base + digit
    NumberAccumulation {
        base: i64,
        digit_var: String,
    },
    Other,
}
```

✅ **NumberAccumulation variant exists!**

### 3.2 Detection Logic (lines 157-192)

**Pattern Recognition**:
```rust
// Detects: (carrier * base) + digit
if matches!(operator, BinaryOperator::Add | BinaryOperator::Subtract) {
    if let ASTNode::BinaryOp {
        operator: BinaryOperator::Multiply,
        left: mul_left,
        right: mul_right,
        ..
    } = left.as_ref() {
        // Check if multiplication is: carrier * base
        if mul_lhs_name == carrier_name {
            if let ASTNode::Literal { value: LiteralValue::Integer(base), .. } = mul_right.as_ref() {
                if let Some(digit_var) = Self::extract_variable_name(right) {
                    // NumberAccumulation pattern detected!
                    return Some(UpdateExpr::BinOp {
                        lhs: carrier_name.to_string(),
                        op,
                        rhs: UpdateRhs::NumberAccumulation { base: *base, digit_var },
                    });
                }
            }
        }
    }
}
```

✅ **Exactly matches our pattern: `result = result * 10 + digit_pos`**

### 3.3 Emission Logic (carrier_update_emitter)

**Location**: `/home/tomoaki/git/hakorune-selfhost/src/mir/join_ir/lowering/carrier_update_emitter/mod.rs` (lines 139-170)

```rust
UpdateRhs::NumberAccumulation { base, digit_var } => {
    // Step 1: Emit const for base
    let base_id = alloc_value();
    instructions.push(JoinInst::Compute(MirLikeInst::Const {
        dst: base_id,
        value: ConstValue::Integer(*base),
    }));

    // Step 2: Emit multiplication: tmp = carrier * base
    let tmp_id = alloc_value();
    instructions.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: tmp_id,
        op: BinOpKind::Mul,
        lhs: carrier_param,
        rhs: base_id,
    }));

    // Step 3: Resolve digit variable
    let digit_id = env.resolve(digit_var).ok_or_else(...)?;

    // Step 4: Emit addition: result = tmp + digit
    let result = alloc_value();
    instructions.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: result,
        op: *op,  // Add or Subtract
        lhs: tmp_id,
        rhs: digit_id,
    }));
}
```

✅ **Complete emission infrastructure ready!**

### 3.4 Test Coverage

**Existing tests** (loop_update_analyzer.rs):
- ✅ `test_analyze_number_accumulation_base10()` - decimal pattern
- ✅ `test_analyze_number_accumulation_base2()` - binary pattern
- ✅ `test_analyze_number_accumulation_wrong_lhs()` - validation

**Existing tests** (carrier_update_emitter):
- ✅ `test_emit_number_accumulation_base10()` - JoinIR emission
- ✅ `test_emit_number_accumulation_digit_not_found()` - error handling

---

## 4. Infrastructure Confirmation Results

### ✅ Checklist

- [x] `UpdateRhs::NumberAccumulation` variant exists
- [x] `result = result * 10 + digit_pos` pattern fully supported
- [x] Detection logic handles nested BinaryOp (Mul inside Add)
- [x] Emission logic generates correct JoinIR sequence
- [x] Unit tests cover base10 and base2 patterns
- [x] Error handling for missing digit variable

### 🎯 No Extensions Needed!

The existing Phase 190 infrastructure is **complete and ready** for _atoi integration. No modifications to UpdateExpr, detection, or emission logic required.

---

## 5. Integration Requirements

### 5.1 Condition Expression Support

#### Header Condition: `i < len`
- **Type**: Simple comparison
- **Lowering**: ExprLowerer with ConditionEnv
- **Required vars in env**: `i`, `len`

#### Break Condition: `digit_pos < 0`
- **Type**: Simple comparison
- **Lowering**: ExprLowerer or digitpos_condition_normalizer
- **Required vars in env**: `digit_pos`

### 5.2 Variable Capture Requirements

#### Function Parameters (must be in ConditionEnv)
- `s` - input string
- `len` - string length

#### Loop-Local Variables (LoopBodyLocal detection required)
- `digits` - digit lookup string (pre-loop local)
- `ch` - current character (body-local)
- `digit_pos` - digit position (body-local, used in break condition)

### 5.3 UpdateEnv Resolution

The UpdateEnv must resolve:
1. **Carriers**: `i`, `result` (from carrier params)
2. **Condition vars**: `len` (from function params)
3. **Body-locals**: `digit_pos` (from loop body)

---

## 6. Test Plan

### 6.1 E2E Test Cases

**File**: `apps/tests/json_atoi_smoke.hako`

| Input | Expected Output | Test Case |
|-------|----------------|-----------|
| `"0"` | `0` | Single digit zero |
| `"42"` | `42` | Two digits |
| `"123"` | `123` | Multiple digits |
| `"007"` | `7` | Leading zeros |
| `"123abc"` | `123` | Break at non-digit |
| `"abc"` | `0` | Immediate break (no digits) |
| `""` | `0` | Empty string |

### 6.2 JoinIR Structure Tests

**Verify**:
1. **UpdateExpr detection**:
   - `i = i + 1` → `UpdateExpr::Const(1)`
   - `result = result * 10 + digit_pos` → `UpdateExpr::BinOp { rhs: NumberAccumulation { base: 10, digit_var: "digit_pos" } }`

2. **CarrierInfo**:
   - Both `i` and `result` marked as LoopState
   - Correct initial values (both 0)

3. **ExitMeta**:
   - Contains `("i", ...)` and `("result", ...)`
   - Exit ValueIds available for function return

4. **JoinIR instructions**:
   - Mul+Add sequence emitted for `result` update
   - Correct base const (10)
   - Correct digit_pos variable resolution

### 6.3 MIR Dump Verification

**Commands**:
```bash
# Basic MIR structure
./target/release/hakorune --dump-mir apps/tests/json_atoi_smoke.hako

# Detailed MIR with effects
./target/release/hakorune --dump-mir --mir-verbose --mir-verbose-effects apps/tests/json_atoi_smoke.hako

# JSON format for detailed analysis
./target/release/hakorune --emit-mir-json mir.json apps/tests/json_atoi_smoke.hako
jq '.functions[] | select(.name == "_atoi") | .blocks' mir.json
```

**Expected MIR patterns**:
- Const instruction for base 10
- Mul instruction: `%tmp = %result * %base`
- Add instruction: `%result_next = %tmp + %digit_pos`
- PHI nodes for both carriers at loop header

---

## 7. Implementation Strategy (Recommended Steps)

### Step 1: Minimal Smoke Test
- Create simple test case with hardcoded digit loop
- Verify NumberAccumulation detection works
- Confirm JoinIR emission is correct

### Step 2: LoopBodyLocal Detection
- Ensure `digit_pos` is detected as loop body local
- Verify it's available in UpdateEnv during emission
- Test break condition lowering with `digit_pos < 0`

### Step 3: Function Parameter Capture
- Verify `s` and `len` are captured correctly
- Test ConditionEnv resolution for header condition `i < len`
- Ensure ExprLowerer can access function params

### Step 4: Full _atoi Integration
- Test complete `_atoi` function from jsonparser.hako
- Verify all E2E test cases pass
- Check MIR dump for correct structure

### Step 5: Edge Cases
- Empty string handling
- Single character strings
- Non-digit immediate break
- Large numbers (overflow consideration)

---

## 8. Potential Challenges

### 8.1 LoopBodyLocal Detection

**Issue**: `digit_pos` must be recognized as a loop body local variable that is:
1. Declared inside loop body (`local digit_pos = ...`)
2. Used in break condition (`if digit_pos < 0`)
3. Available in UpdateEnv during result update emission

**Solution**: Existing LoopBodyLocalDetector should handle this (Phase 184).

### 8.2 Method Call in Loop Body

**Issue**: `s.substring(i, i + 1)` and `digits.indexOf(ch)` are method calls with multiple arguments.

**Solution**: LoopBodyLocal lowering should handle method calls as "complex expressions" (Phase 184 already supports this via `ExprLowerer`).

### 8.3 UpdateEnv Variable Resolution

**Issue**: UpdateEnv must resolve variables from three sources:
- Carriers: `i`, `result`
- Function params: `s`, `len`
- Body-locals: `ch`, `digit_pos`

**Solution**: Phase 184's `UpdateEnv::new(&cond_env, &body_env)` should handle this if:
- ConditionEnv contains function params + carriers
- LoopBodyLocalEnv contains body-local ValueIds

---

## 9. Success Criteria

### ✅ Phase 246-EX Complete When:

1. **Detection**: `result = result * 10 + digit_pos` correctly identified as NumberAccumulation
2. **Emission**: Mul+Add JoinIR sequence generated with correct ValueIds
3. **Execution**: All E2E test cases produce correct numeric outputs
4. **Validation**: MIR dump shows expected loop structure with PHI nodes
5. **Integration**: Works within full JsonParser context (not just isolated test)

---

## 10. References

### Phase 190: NumberAccumulation Infrastructure
- Detection: `/src/mir/join_ir/lowering/loop_update_analyzer.rs` (lines 157-192)
- Emission: `/src/mir/join_ir/lowering/carrier_update_emitter/mod.rs` (lines 139-170)
- Tests: Both files contain comprehensive unit tests

### Phase 184: UpdateEnv and LoopBodyLocal
- UpdateEnv: `/src/mir/join_ir/lowering/update_env.rs`
- LoopBodyLocal detection: (search for LoopBodyLocalDetector)

### Related Phases
- Phase 176-2: Carrier update emission basics
- Phase 178: String literal updates (similar multi-carrier pattern)
- Phase 197: Loop update analyzer extraction

---

## Appendix A: Quick Reference

### Pattern Match: _atoi Loop

```
Loop Type:        Pattern 2 (Break)
Header:           i < len
Break:            digit_pos < 0
Carriers:         i (counter), result (accumulator)
Body-locals:      ch, digit_pos
Captured:         s, len, digits (pre-loop)

UpdateExpr:
  i      -> Const(1)
  result -> BinOp { op: Add, rhs: NumberAccumulation { base: 10, digit_var: "digit_pos" } }
```

### JoinIR Emission Sequence (NumberAccumulation)

```
%base_10 = Const(10)
%tmp = BinOp(Mul, %result_param, %base_10)
%digit_id = <resolved from UpdateEnv>
%result_next = BinOp(Add, %tmp, %digit_id)
```

### Debug Commands

```bash
# Trace variable mapping
NYASH_TRACE_VARMAP=1 cargo test --release test_json_atoi -- --nocapture

# JoinIR debug
NYASH_JOINIR_DEBUG=1 ./target/release/hakorune apps/tests/json_atoi_smoke.hako 2>&1 | grep "\[trace:"

# UpdateExpr detection (if logging added)
NYASH_UPDATE_ANALYZER_DEBUG=1 ./target/release/hakorune apps/tests/json_atoi_smoke.hako
```

---

**Document Status**: ✅ Infrastructure confirmed, ready for implementation
**Next Step**: Phase 246-EX Step 1 - Minimal smoke test creation
**Last Updated**: 2025-12-11 (Phase 246-EX Step 0 completion)
