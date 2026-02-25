# Phase 187: String UpdateLowering Design (Doc-Only)

**Date**: 2025-12-09
**Status**: Design Phase (No Code Changes)
**Prerequisite**: Phase 178 Fail-Fast must remain intact

---

## Executive Summary

Phase 187 defines **what kinds of string updates are safe to handle in JoinIR**, using an UpdateKind-based whitelist approach. This is a design-only phase—no code will be changed.

**Core Principle**: Maintain Phase 178's Fail-Fast behavior while establishing a clear path forward for string operations.

---

## 1. UpdateKind Candidates

We classify update patterns into categories based on their complexity and safety:

### 1.1 Safe Patterns (Whitelist Candidates)

#### CounterLike
**Pattern**: `pos = pos + 1`, `i = i - 1`
**String Relevance**: Position tracking in string scanning loops
**Safety**: ✅ Simple arithmetic, deterministic
**Decision**: **ALLOW** (already supported in Phase 178)

#### AccumulationLike (Numeric)
**Pattern**: `sum = sum + i`, `total = total * factor`
**String Relevance**: None (numeric only)
**Safety**: ✅ Arithmetic operations, well-understood
**Decision**: **ALLOW** (already supported in Phase 178)

#### StringAppendChar
**Pattern**: `result = result + ch` (where `ch` is a single character variable)
**Example**: JsonParser `_parse_number`: `num_str = num_str + digit_ch`
**Safety**: ⚠️ Requires:
  - RHS must be `UpdateRhs::Variable(name)`
  - Variable scope: LoopBodyLocal or OuterLocal
  - Single character (enforced at runtime by StringBox semantics)
**Decision**: **ALLOW** (with validation)

**Rationale**: This pattern is structurally identical to numeric accumulation:
```
sum = sum + i        // Numeric accumulation
result = result + ch // String accumulation (char-by-char)
```

#### StringAppendLiteral
**Pattern**: `s = s + "..."` (where `"..."` is a string literal)
**Example**: `debug_output = debug_output + "[INFO] "`
**Safety**: ⚠️ Requires:
  - RHS must be `UpdateRhs::StringLiteral(s)`
  - Literal must be compile-time constant
**Decision**: **ALLOW** (with validation)

**Rationale**: Simpler than StringAppendChar—no variable resolution needed.

### 1.2 Unsafe Patterns (Fail-Fast)

#### Complex (Method Calls)
**Pattern**: `result = result + s.substring(pos, end)`
**Example**: JsonParser `_unescape_string`
**Safety**: ❌ Requires:
  - Method call evaluation
  - Multiple arguments
  - Potentially non-deterministic results
**Decision**: **REJECT** with `[joinir/freeze]`

**Error Message**:
```
[pattern2/can_lower] Complex string update detected (method call in RHS).
JoinIR does not support this pattern yet. Use simpler string operations.
```

#### Complex (Nested BinOp)
**Pattern**: `x = x + (a + b)`, `result = result + s1 + s2`
**Safety**: ❌ Nested expression evaluation required
**Decision**: **REJECT** with `[joinir/freeze]`

---

## 2. Fail-Fast Policy (Phase 178 Preservation)

**Non-Negotiable**: Phase 178's Fail-Fast behavior must remain intact.

### 2.1 Current Fail-Fast Logic (Untouched)

**File**: `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`
**File**: `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs`

```rust
// Phase 178: Reject string/complex updates
fn can_lower(...) -> bool {
    for update in carrier_updates.values() {
        match update {
            UpdateExpr::BinOp { rhs, .. } => {
                if matches!(rhs, UpdateRhs::StringLiteral(_) | UpdateRhs::Other) {
                    // Phase 178: Fail-Fast for string updates
                    return false; // ← This stays unchanged in Phase 187
                }
            }
            _ => {}
        }
    }
    true
}
```

**Phase 187 Changes**: NONE (this code is not touched in Phase 187).

### 2.2 Future Whitelist Expansion (Phase 188+)

In **Phase 188** (implementation phase), we will:
1. Extend `can_lower()` to accept `StringAppendChar` and `StringAppendLiteral`
2. Add validation to ensure safety constraints (variable scope, literal type)
3. Extend `CarrierUpdateLowerer` to emit JoinIR for string append operations

**Phase 187 does NOT implement this**—we only design what "safe" means.

---

## 3. Lowerer Responsibility Separation

### 3.1 Detection Layer (Pattern2/4)

**Responsibility**: UpdateKind classification only
**Location**: `pattern2_with_break.rs`, `pattern4_with_continue.rs`

```rust
// Phase 187 Design: What Pattern2/4 WILL check (future)
fn can_lower_string_update(update: &UpdateExpr) -> bool {
    match update {
        UpdateExpr::BinOp { rhs, .. } => {
            match rhs {
                UpdateRhs::Variable(_) => true,       // StringAppendChar
                UpdateRhs::StringLiteral(_) => true,  // StringAppendLiteral
                UpdateRhs::Other => false,            // Complex (reject)
                UpdateRhs::Const(_) => true,          // Numeric (already allowed)
            }
        }
        _ => true,
    }
}
```

**Key Point**: Pattern2/4 only perform classification—they do NOT emit JoinIR for strings.

### 3.2 Emission Layer (CarrierUpdateLowerer + Expr Lowerer)

**Responsibility**: Actual JoinIR instruction emission
**Location**: `src/mir/join_ir/lowering/carrier_update_lowerer.rs`

**Current State (Phase 184)**:
- Handles numeric carriers only (`CounterLike`, `AccumulationLike`)
- Emits `Compute { op: Add/Sub/Mul, ... }` for numeric BinOp

**Future State (Phase 188+ Implementation)**:
- Extend to handle `StringAppendChar`:
  ```rust
  // Emit StringBox.concat() call or equivalent
  let concat_result = emit_string_concat(lhs_value, ch_value);
  ```
- Extend to handle `StringAppendLiteral`:
  ```rust
  // Emit string literal + concat
  let literal_value = emit_string_literal("...");
  let concat_result = emit_string_concat(lhs_value, literal_value);
  ```

**Phase 187 Design**: Document this separation, but do NOT implement.

---

## 4. Architecture Diagram

```
AST → LoopUpdateAnalyzer → UpdateKind classification
                              ↓
                    Pattern2/4.can_lower()
                    (Whitelist check only)
                              ↓
                    [ALLOW] → CarrierUpdateLowerer
                              (Emit JoinIR instructions)
                              ↓
                              JoinIR Module

                    [REJECT] → [joinir/freeze] error
```

**Separation of Concerns**:
1. **LoopUpdateAnalyzer**: Extracts `UpdateExpr` from AST (already exists)
2. **Pattern2/4**: Classifies into Allow/Reject (Phase 178 logic + Phase 188 extension)
3. **CarrierUpdateLowerer**: Emits JoinIR (Phase 184 for numeric, Phase 188+ for string)

---

## 5. Representative Cases (Not Implemented)

### 5.1 JsonParser Update Patterns

#### _parse_number: `num_str = num_str + ch`
**UpdateKind**: `StringAppendChar`
**Classification**:
- `num_str`: carrier name
- `ch`: LoopBodyLocal variable (single character from string scan)
- RHS: `UpdateRhs::Variable("ch")`
**Decision**: **ALLOW** (Phase 188+)

#### _atoi: `num = num * 10 + digit`
**UpdateKind**: `AccumulationLike` (numeric)
**Classification**:
- Nested BinOp: `(num * 10) + digit`
- Currently detected as `UpdateRhs::Other`
**Decision**: **COMPLEX** (requires BinOp tree analysis, Phase 189+)

#### _unescape_string: `result = result + s.substring(...)`
**UpdateKind**: `Complex` (method call)
**Classification**:
- RHS: `UpdateRhs::Other` (MethodCall)
**Decision**: **REJECT** with Fail-Fast

### 5.2 UpdateKind Mapping Table

| Loop Variable | Update Pattern | UpdateRhs | UpdateKind | Phase 187 Decision |
|---------------|----------------|-----------|------------|-------------------|
| `num_str` | `num_str + ch` | `Variable("ch")` | StringAppendChar | ALLOW (Phase 188+) |
| `result` | `result + "\n"` | `StringLiteral("\n")` | StringAppendLiteral | ALLOW (Phase 188+) |
| `num` | `num * 10 + digit` | `Other` (nested BinOp) | Complex | REJECT (Phase 189+) |
| `result` | `result + s.substring(...)` | `Other` (MethodCall) | Complex | REJECT (Fail-Fast) |
| `pos` | `pos + 1` | `Const(1)` | CounterLike | ALLOW (Phase 178 ✅) |
| `sum` | `sum + i` | `Variable("i")` | AccumulationLike | ALLOW (Phase 178 ✅) |

---

## 6. Next Steps (Phase 188+ Implementation)

### Phase 188: StringAppendChar/Literal Implementation

**Scope**: Extend Pattern2/4 and CarrierUpdateLowerer to support string append.

**Tasks**:
1. **Extend `can_lower()` whitelist** (Pattern2/4)
   - Accept `UpdateRhs::Variable(_)` for string carriers
   - Accept `UpdateRhs::StringLiteral(_)` for string carriers
   - Keep `UpdateRhs::Other` as Fail-Fast

2. **Extend CarrierUpdateLowerer** (emission layer)
   - Detect carrier type (String vs Integer)
   - Emit `StringBox.concat()` call for string append
   - Emit `Compute { Add }` for numeric (existing logic)

3. **Add validation**
   - Check variable scope (LoopBodyLocal or OuterLocal only)
   - Check literal type (string only)

4. **E2E Test**
   - `_parse_number` minimal version with `num_str = num_str + ch`

**Estimate**: 3-4 hours

### Phase 189+: Complex BinOp (Future)

**Scope**: Handle nested BinOp like `num * 10 + digit`.

**Tasks**:
1. Extend `analyze_rhs()` to recursively parse BinOp trees
2. Classify simple nested patterns (e.g., `(x * 10) + y`) as safe
3. Keep truly complex patterns (e.g., method calls in BinOp) as Fail-Fast

**Estimate**: 5-6 hours

---

## 7. Design Constraints

### 7.1 Box Theory Compliance

**Separation of Concerns**:
- UpdateKind classification → LoopUpdateAnalyzer (existing box)
- Can-lower decision → Pattern2/4 (control flow box)
- JoinIR emission → CarrierUpdateLowerer (lowering box)

**No Cross-Boundary Leakage**:
- Pattern2/4 do NOT emit JoinIR directly for string operations
- CarrierUpdateLowerer does NOT make can-lower decisions

### 7.2 Fail-Fast Preservation

**Phase 178 Logic Untouched**:
- All `UpdateRhs::StringLiteral` and `UpdateRhs::Other` continue to trigger Fail-Fast
- Phase 187 only documents what "safe" means—implementation is Phase 188+

**Error Messages**:
- Current: `"String/complex update detected, rejecting Pattern 2 (unsupported)"`
- Future (Phase 188+): More specific messages for different rejection reasons

### 7.3 Testability

**Unit Test Separation**:
- LoopUpdateAnalyzer tests: AST → UpdateExpr extraction
- Pattern2/4 tests: UpdateExpr → can_lower decision
- CarrierUpdateLowerer tests: UpdateExpr → JoinIR emission

**E2E Test**:
- JsonParser representative loops (Phase 188+)

---

## 8. Documentation Updates

### 8.1 joinir-architecture-overview.md

Add one sentence in Section 2.2 (条件式ライン):

```markdown
- **LoopUpdateAnalyzer / CarrierUpdateLowerer**
  - ファイル:
    - `src/mir/join_ir/lowering/loop_update_analyzer.rs`
    - `src/mir/join_ir/lowering/carrier_update_lowerer.rs`
  - 責務:
    - ループで更新される変数（carrier）を検出し、UpdateExpr を保持。
    - Pattern 4 では実際に更新されるキャリアだけを残す。
    - **Phase 187設計**: String 更新は UpdateKind ベースのホワイトリストで扱う方針（StringAppendChar/Literal は Phase 188+ で実装予定）。
```

### 8.2 CURRENT_TASK.md

Add Phase 187 entry:

```markdown
  - [x] **Phase 187: String UpdateLowering 設計** ✅ (2025-12-09)
        - UpdateKind ベースのホワイトリスト設計（doc-only）
        - StringAppendChar/StringAppendLiteral を安全パターンとして定義
        - Complex (method call / nested BinOp) は Fail-Fast 維持
        - Phase 178 の Fail-Fast は完全保持
        - Phase 188+ での実装方針を確立
```

---

## 9. Success Criteria (Phase 187)

- [x] Design document created (`phase187-string-update-design.md`)
- [x] UpdateKind whitelist defined (6 categories)
- [x] Fail-Fast preservation confirmed (Phase 178 untouched)
- [x] Lowerer responsibility separation documented
- [x] Representative cases analyzed (JsonParser loops)
- [x] Architecture diagram created
- [x] Next steps defined (Phase 188+ implementation)
- [x] `joinir-architecture-overview.md` updated (1-sentence addition)
- [x] `CURRENT_TASK.md` updated (Phase 187 entry added)

**All criteria met**: Phase 187 complete (design-only).

---

## Phase 188 Implementation Complete (2025-12-09)

### Implementation Summary

Phase 188 successfully implemented StringAppendChar and StringAppendLiteral support in JoinIR patterns.

**Changes Made**:

1. **Pattern2/4 `can_lower()` Whitelist** (Task 188-2)
   - Updated `pattern2_with_break.rs` and `pattern4_with_continue.rs`
   - Allow: `UpdateRhs::Const`, `UpdateRhs::Variable`, `UpdateRhs::StringLiteral`
   - Reject: `UpdateRhs::Other` (complex updates only)
   - Old behavior: Rejected all string updates
   - New behavior: Accept safe string patterns, reject only complex ones

2. **CarrierUpdateLowerer JoinIR Emission** (Task 188-3)
   - Updated `carrier_update_emitter.rs` (both UpdateEnv and ConditionEnv versions)
   - `UpdateRhs::StringLiteral(s)` → Emit `Const { value: ConstValue::String(s) }` + `BinOp`
   - `UpdateRhs::Variable(name)` → Resolve variable, emit `BinOp` (handles both numeric and string)
   - `UpdateRhs::Other` → Return error (should be caught by can_lower)

3. **E2E Test Files** (Task 188-4)
   - Created `apps/tests/phase188_string_append_char.hako` (Pattern 2 with break)
   - Created `apps/tests/phase188_string_append_literal.hako` (Pattern 4 with continue)
   - Both tests compile and run without errors
   - JoinIR generation succeeds for both patterns

**Verification**:
```bash
# Pattern 2: StringAppendChar
NYASH_JOINIR_CORE=1 ./target/release/hakorune apps/tests/phase188_string_append_char.hako
# Output: Pattern 2 triggered, JoinIR generated successfully

# Pattern 4: StringAppendLiteral
NYASH_JOINIR_CORE=1 ./target/release/hakorune apps/tests/phase188_string_append_literal.hako
# Output: Pattern 4 triggered, JoinIR generated successfully
```

**Key Achievement**: Phase 178's Fail-Fast is now selective - only rejects truly complex updates (method calls, nested BinOp), while allowing safe string concatenation patterns.

---

## 10. Conclusion

Phase 187 establishes a clear design for string update handling in JoinIR:

1. **Safe Patterns**: CounterLike, AccumulationLike, StringAppendChar, StringAppendLiteral
2. **Unsafe Patterns**: Complex (method calls, nested BinOp) → Fail-Fast
3. **Separation of Concerns**: Detection (Pattern2/4) vs Emission (CarrierUpdateLowerer)
4. **Phase 178 Preservation**: All Fail-Fast logic remains unchanged

**No code changes in Phase 187**—all design decisions documented for Phase 188+ implementation.

**Next Phase**: Phase 188 - Implement StringAppendChar/Literal lowering (3-4 hours estimate).
Status: Historical
