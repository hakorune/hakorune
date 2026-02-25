# Phase 190: NumberAccumulation Update Design (Doc-Only)

**Status**: Design Complete (Code TBD)
**Date**: 2025-12-09
**Goal**: Design JoinIR support for `result = result * 10 + digit` style updates

---

## Section 1: Target Loops and RHS Patterns

### 1.1 JsonParserBox._atoi (lines 436-467)

**Loop Pattern**:
```nyash
local v = 0
local digits = "0123456789"
loop(i < n) {
    local ch = s.substring(i, i+1)
    if ch < "0" || ch > "9" { break }
    local pos = digits.indexOf(ch)
    if pos < 0 { break }
    v = v * 10 + pos    // ← NumberAccumulation pattern
    i = i + 1
}
```

**AST Form of Update**:
```
Assign(
  lhs = "v",
  rhs = BinaryOp(
    op = Add,
    left = BinaryOp(
      op = Mul,
      left = Variable("v"),
      right = Literal(Integer(10))
    ),
    right = Variable("pos")
  )
)
```

**Characteristics**:
- LHS appears exactly once in RHS (in left-most multiplication)
- Base: 10 (constant)
- Addend: `pos` (loop-local variable)
- Type: Integer (MirType::Integer)

### 1.2 JsonParserBox._parse_number (lines 106-142)

**Loop Pattern**:
```nyash
local num_str = ""
local digits = "0123456789"
loop(p < s.length()) {
    local ch = s.substring(p, p+1)
    local digit_pos = digits.indexOf(ch)

    if digit_pos < 0 {
        break
    }

    num_str = num_str + ch    // ← StringAppendChar (already supported)
    p = p + 1
}
```

**Note**: This is string accumulation (`num_str + ch`), not number accumulation.
Already handled by existing `StringAppendChar` / `AccumulationLike`.

### 1.3 Other Instances in Codebase

From grep results:

1. **apps/tests/phase189_atoi_mini.hako:10**
   - `result = result * 10 + i`
   - Simplified test case

2. **apps/tests/phase183_p2_atoi.hako:25**
   - `result = result * 10 + digit`
   - Similar pattern

3. **apps/tests/phase183_p2_parse_number.hako:28**
   - `result = result * 10 + digit_pos`
   - Same pattern with different variable name

4. **lang/src/compiler/builder/ssa/exit_phi/break_finder.hako:277**
   - `result = result * 10 + (BreakFinderBox._char_to_digit(ch))`
   - Includes method call on RHS (Complex)

### 1.4 Canonical Form Requirements

**Safe NumberAccumulation Pattern**:
```
lhs = lhs * BASE + addend
```

**Where**:
- `lhs`: carrier variable (appears exactly 1 time in RHS)
- `BASE`: integer constant (typically 10)
- `addend`: one of:
  - Variable (loop-local or carrier)
  - Constant
  - **NOT** method call (→ Complex)

**Variants**:
- `lhs = lhs * BASE - addend` (subtraction also allowed)
- Base can be any integer constant (10, 2, 16, etc.)

---

## Section 2: Safe Pattern Candidates

### 2.1 Whitelist Criteria

A NumberAccumulation update is **safe** if:

1. **Type Check**: Carrier type is `MirType::Integer`
2. **Structure Check**: AST matches `lhs = Add(Mul(lhs, Const(base)), addend)`
3. **LHS Occurrence**: `lhs` appears exactly 1 time in RHS (in multiplication)
4. **Base Check**: Base is integer constant (not variable)
5. **Addend Check**: Addend is one of:
   - `Variable(name)` - loop-local or carrier
   - `Const(n)` - integer constant
6. **No Method Calls**: Addend does not contain method calls

### 2.2 Reject as Complex

Mark as `UpdateKind::Complex` if:

- LHS appears 2+ times in RHS
- Base is variable (not constant)
- Addend contains method call (e.g., `_char_to_digit(ch)`)
- Addend is complex expression (nested BinaryOp)
- Type is not Integer
- LoopBodyLocal appears in addend (Phase 5 未サポート)

### 2.3 Pattern Matrix

| Pattern | LHS Count | Base | Addend | Decision |
|---------|-----------|------|--------|----------|
| `v = v * 10 + pos` | 1 | Const(10) | Variable | NumberAccumulation |
| `v = v * 10 + 5` | 1 | Const(10) | Const(5) | NumberAccumulation |
| `v = v * base + x` | 1 | Variable | Variable | Complex |
| `v = v * 10 + f(x)` | 1 | Const(10) | MethodCall | Complex |
| `v = v * 10 + v` | 2 | Const(10) | Variable | Complex |

---

## Section 3: UpdateKind::NumberAccumulation Definition

### 3.1 Enum Extension

**Current** (Phase 170-C-2):
```rust
pub enum UpdateKind {
    CounterLike,
    AccumulationLike,
    Other,
}
```

**Proposed** (Phase 190):
```rust
pub enum UpdateKind {
    CounterLike,
    AccumulationLike,

    /// Phase 190: Number accumulation: result = result * base + addend
    ///
    /// Typical pattern: digit expansion (atoi, parse_number)
    /// Example: v = v * 10 + digit
    NumberAccumulation { base: i64 },

    /// Phase 178+: String append patterns (already implemented)
    StringAppendChar,
    StringAppendLiteral,

    /// Complex or unrecognized patterns
    Complex,

    /// Deprecated
    Other,
}
```

### 3.2 UpdateKind::name() Extension

```rust
impl UpdateKind {
    pub fn name(&self) -> &'static str {
        match self {
            UpdateKind::CounterLike => "CounterLike",
            UpdateKind::AccumulationLike => "AccumulationLike",
            UpdateKind::NumberAccumulation { base } => "NumberAccumulation",
            UpdateKind::StringAppendChar => "StringAppendChar",
            UpdateKind::StringAppendLiteral => "StringAppendLiteral",
            UpdateKind::Complex => "Complex",
            UpdateKind::Other => "Other",
        }
    }
}
```

### 3.3 Alternative: Subfield on AccumulationLike

Instead of new enum variant, could extend AccumulationLike:

```rust
pub enum UpdateKind {
    CounterLike,
    AccumulationLike {
        style: AccumulationStyle,
    },
    Complex,
    Other,
}

pub enum AccumulationStyle {
    Simple,           // result = result + x
    NumberDigit { base: i64 },  // result = result * base + x
    StringAppend,     // result = result + "literal"
}
```

**Decision**: Use dedicated `NumberAccumulation { base: i64 }` for clarity.
Simpler to pattern-match, clearer intent.

---

## Section 4: classify_number_update Pseudocode

### 4.1 Algorithm Overview

**Input**:
- `lhs: &str` - carrier variable name
- `rhs_ast: &ASTNode` - RHS expression AST

**Output**:
- `UpdateKind` - classified update pattern

**Steps**:
1. Check if RHS is `BinaryOp(Add, left, right)`
2. Check if `left` is `BinaryOp(Mul, mul_left, mul_right)`
3. Verify `mul_left` is `Variable(lhs)` (LHS appears once)
4. Verify `mul_right` is `Literal(Integer(base))`
5. Classify `right` (addend):
   - `Const(n)` → NumberAccumulation
   - `Variable(name)` → NumberAccumulation
   - `MethodCall/Other` → Complex

### 4.2 Pseudocode

```rust
fn classify_number_update(lhs: &str, rhs_ast: &ASTNode) -> UpdateKind {
    // Step 1: Check outer addition
    let BinaryOp { op: Add, left: mul_expr, right: addend } = rhs_ast else {
        return UpdateKind::Complex;
    };

    // Step 2: Check inner multiplication
    let BinaryOp { op: Mul, left: mul_left, right: mul_right } = mul_expr else {
        return UpdateKind::Complex;
    };

    // Step 3: Verify LHS appears in multiplication
    let Variable { name: lhs_name } = mul_left else {
        return UpdateKind::Complex;
    };
    if lhs_name != lhs {
        return UpdateKind::Complex;
    }

    // Step 4: Extract base constant
    let Literal { value: Integer(base) } = mul_right else {
        return UpdateKind::Complex;  // base is not constant
    };

    // Step 5: Classify addend
    match addend {
        // Variable or Const: NumberAccumulation
        Literal { value: Integer(_) } | Variable { .. } => {
            UpdateKind::NumberAccumulation { base: *base }
        }

        // Method call or complex expression: Complex
        MethodCall { .. } | BinaryOp { .. } | Call { .. } => {
            UpdateKind::Complex
        }

        _ => UpdateKind::Complex,
    }
}
```

### 4.3 LHS Occurrence Check

**Goal**: Ensure LHS appears exactly 1 time in RHS.

**Implementation**:
```rust
fn count_lhs_occurrences(lhs: &str, rhs: &ASTNode) -> usize {
    match rhs {
        Variable { name } if name == lhs => 1,
        BinaryOp { left, right, .. } => {
            count_lhs_occurrences(lhs, left) + count_lhs_occurrences(lhs, right)
        }
        MethodCall { receiver, args, .. } => {
            let mut count = count_lhs_occurrences(lhs, receiver);
            for arg in args {
                count += count_lhs_occurrences(lhs, arg);
            }
            count
        }
        _ => 0,
    }
}
```

**Usage**:
```rust
fn classify_number_update(lhs: &str, rhs_ast: &ASTNode) -> UpdateKind {
    // FIRST: Check LHS occurrence count
    let lhs_count = count_lhs_occurrences(lhs, rhs_ast);
    if lhs_count != 1 {
        return UpdateKind::Complex;
    }

    // THEN: Check structure
    // ... (rest of classify_number_update logic)
}
```

---

## Section 5: Pattern2/4 can_lower Specification

### 5.1 Current Behavior

**Pattern2 can_lower** (Phase 176+):
- Accepts: CounterLike, AccumulationLike, StringAppendChar, StringAppendLiteral
- Rejects: Complex → `[joinir/freeze]`

**Pattern4 can_lower** (Phase 33-19+):
- Similar criteria to Pattern2

### 5.2 Proposed Extension

**Pattern2 can_lower** (Phase 190+):
```rust
fn can_lower_carrier_updates(updates: &HashMap<String, UpdateExpr>) -> bool {
    for (_name, update_expr) in updates {
        let kind = classify_update_kind(update_expr);

        match kind {
            // Whitelist: Simple patterns
            UpdateKind::CounterLike
            | UpdateKind::AccumulationLike
            | UpdateKind::StringAppendChar
            | UpdateKind::StringAppendLiteral
            | UpdateKind::NumberAccumulation { .. }  // ← NEW!
                => { /* OK */ }

            // Fail-Fast: Complex patterns
            UpdateKind::Complex => {
                eprintln!("[joinir/freeze] Complex carrier update detected");
                return false;
            }

            UpdateKind::Other => {
                eprintln!("[joinir/freeze] Unknown update pattern");
                return false;
            }
        }
    }

    true
}
```

### 5.3 Type Constraint

**NumberAccumulation Type Check**:
```rust
fn verify_number_accumulation_type(carrier: &CarrierVar) -> bool {
    // Phase 190: NumberAccumulation requires Integer type
    // String types must use StringAppendChar/StringAppendLiteral
    match carrier.mir_type {
        MirType::Integer => true,
        _ => {
            eprintln!("[joinir/freeze] NumberAccumulation requires Integer type, got {:?}",
                      carrier.mir_type);
            false
        }
    }
}
```

### 5.4 LoopBodyLocal Handling

**Phase 5 未サポート**:
```rust
fn check_loopbodylocal_constraint(addend: &UpdateRhs, loop_scope: &LoopScopeShape) -> bool {
    match addend {
        UpdateRhs::Variable(name) => {
            // Check if variable is LoopBodyLocal
            if loop_scope.body_locals.contains(name) {
                eprintln!("[joinir/freeze] LoopBodyLocal in NumberAccumulation not supported yet");
                return false;
            }
            true
        }
        _ => true,
    }
}
```

### 5.5 Fallback Strategy

**Fail-Fast Principle**:
- NumberAccumulation detection failure → `Complex` → `[joinir/freeze]`
- **NO** silent fallback to LoopBuilder (already deleted)
- **NO** new fallback paths

**Error Message Template**:
```
[joinir/freeze] NumberAccumulation pattern not supported:
  carrier: v
  reason: addend contains method call
  suggestion: extract method call to loop-local variable
```

---

## Section 6: CarrierUpdateLowerer Pseudocode

### 6.1 Responsibility Assignment

**CarrierUpdateLowerer** (Phase 176+):
- Input: `CarrierVar`, `UpdateExpr`, `JoinIRBuilder`
- Output: Emits JoinIR instructions for carrier update

**Current Support**:
- CounterLike: `dst = lhs + 1`
- AccumulationLike: `dst = lhs + rhs`
- StringAppendChar: (BoxCall to StringBox.append)

**Phase 190 Extension**:
- NumberAccumulation: `tmp = lhs * base; dst = tmp + addend`

### 6.2 JoinIR Emission Design

**Option A: 2-Instruction Approach** (Recommended)
```rust
fn emit_number_accumulation(
    builder: &mut JoinIRBuilder,
    carrier: &CarrierVar,
    base: i64,
    addend: &UpdateRhs,
) -> ValueId {
    // Step 1: Multiply by base
    // tmp = lhs * base
    let base_value = builder.alloc_value();
    builder.emit(JoinInst::Const {
        dst: base_value,
        value: ConstValue::Integer(base),
    });

    let mul_result = builder.alloc_value();
    builder.emit(JoinInst::BinOp {
        dst: mul_result,
        op: BinOpKind::Mul,
        left: carrier.join_id.unwrap(),  // Current value from LoopHeader PHI
        right: base_value,
    });

    // Step 2: Add addend
    // dst = tmp + addend
    let addend_value = emit_addend_value(builder, addend);
    let result = builder.alloc_value();
    builder.emit(JoinInst::BinOp {
        dst: result,
        op: BinOpKind::Add,
        left: mul_result,
        right: addend_value,
    });

    result
}

fn emit_addend_value(builder: &mut JoinIRBuilder, addend: &UpdateRhs) -> ValueId {
    match addend {
        UpdateRhs::Const(n) => {
            let val = builder.alloc_value();
            builder.emit(JoinInst::Const {
                dst: val,
                value: ConstValue::Integer(*n),
            });
            val
        }
        UpdateRhs::Variable(name) => {
            // Look up variable in boundary.join_inputs or condition_bindings
            builder.lookup_variable(name)
        }
        _ => unreachable!("Complex addend should be rejected in can_lower"),
    }
}
```

**Option B: Single Complex Expression** (Not Recommended)
```rust
// Would require JoinInst::ComplexExpr or similar
// Violates "flat instruction" principle of JoinIR
// NOT RECOMMENDED
```

**Decision**: Use Option A (2-instruction approach).
- Pros: Clean separation, easy to optimize later, follows JoinIR flat instruction principle
- Cons: None

### 6.3 Type Constraint Enforcement

```rust
impl CarrierUpdateLowerer {
    pub fn emit_update(
        &self,
        builder: &mut JoinIRBuilder,
        carrier: &CarrierVar,
        update_expr: &UpdateExpr,
    ) -> Result<ValueId, String> {
        match classify_update_kind(update_expr) {
            UpdateKind::NumberAccumulation { base } => {
                // Type check
                if carrier.mir_type != MirType::Integer {
                    return Err(format!(
                        "NumberAccumulation requires Integer type, got {:?}",
                        carrier.mir_type
                    ));
                }

                // Emit instructions
                Ok(self.emit_number_accumulation(builder, carrier, base, &extract_addend(update_expr)))
            }
            // ... other cases
        }
    }
}
```

### 6.4 Name Dependency Prohibition

**Phase 170-C-1 Principle**: No name-based heuristics in lowering.

**Enforcement**:
```rust
// ✅ GOOD: Structural detection
fn is_number_accumulation(update_expr: &UpdateExpr) -> bool {
    matches!(
        update_expr,
        UpdateExpr::BinOp {
            op: BinOpKind::Add,
            lhs: _,
            rhs: UpdateRhs::Variable(_) | UpdateRhs::Const(_),
        }
    )
}

// ❌ BAD: Name-based detection
fn is_number_accumulation(carrier_name: &str) -> bool {
    carrier_name.contains("result") || carrier_name.contains("v")
}
```

**Rationale**:
- Variable names are user-controlled and unreliable
- Structural AST analysis is robust and refactoring-safe
- Follows "箱理論" separation of concerns

### 6.5 Integration with LoopHeaderPhiBuilder

**Phase 33-22 Context**:
- LoopHeader PHI creates SSOT for carrier current value
- `carrier.join_id` points to LoopHeader PHI dst

**Usage in NumberAccumulation**:
```rust
fn emit_number_accumulation(
    builder: &mut JoinIRBuilder,
    carrier: &CarrierVar,
    base: i64,
    addend: &UpdateRhs,
) -> ValueId {
    // Use carrier.join_id (LoopHeader PHI dst) as current value
    let current_value = carrier.join_id.expect("LoopHeader PHI should set join_id");

    // tmp = current_value * base
    let base_const = builder.emit_const(ConstValue::Integer(base));
    let mul_result = builder.emit_binop(BinOpKind::Mul, current_value, base_const);

    // dst = tmp + addend
    let addend_value = emit_addend_value(builder, addend);
    builder.emit_binop(BinOpKind::Add, mul_result, addend_value)
}
```

**Invariant**: Never directly access `carrier.host_id` in JoinIR emission.
Only use `carrier.join_id` (JoinIR-local ValueId).

---

## Section 7: Implementation Phases

### Phase 190-impl-A: Core Detection (2-3 commits)

**Goal**: Extend LoopUpdateAnalyzer to detect NumberAccumulation patterns.

**Tasks**:
1. Add `UpdateKind::NumberAccumulation { base: i64 }`
2. Implement `classify_number_update()` in LoopUpdateAnalyzer
3. Add `count_lhs_occurrences()` helper
4. Unit tests (5+ cases covering Section 2.3 matrix)

**Success Criteria**:
- `_atoi` loop detected as NumberAccumulation { base: 10 }
- Complex patterns (method calls) detected as Complex
- Type safety: Only Integer carriers allowed

### Phase 190-impl-B: CarrierUpdateLowerer Extension (2-3 commits)

**Goal**: Emit JoinIR for NumberAccumulation updates.

**Tasks**:
1. Extend `CarrierUpdateLowerer::emit_update()` with NumberAccumulation branch
2. Implement 2-instruction emission (mul + add)
3. Type constraint enforcement
4. Unit tests (3+ cases)

**Success Criteria**:
- Correct JoinIR emission: `tmp = v * 10; result = tmp + digit`
- Type error on non-Integer carriers
- Integration with LoopHeader PHI (carrier.join_id)

### Phase 190-impl-C: Pattern2/4 Integration (1-2 commits)

**Goal**: Enable NumberAccumulation in Pattern2/4 can_lower.

**Tasks**:
1. Update `can_lower_carrier_updates()` whitelist
2. Add NumberAccumulation test case to Pattern2
3. Verify Fail-Fast for Complex patterns

**Success Criteria**:
- `phase189_atoi_mini.hako` passes with JoinIR
- Complex patterns (with method calls) rejected at can_lower
- `[joinir/freeze]` message for unsupported cases

### Phase 190-impl-D: E2E Validation (1 commit)

**Goal**: Real-world JsonParserBox._atoi working via JoinIR.

**Tasks**:
1. Enable JoinIR for `_atoi` method
2. Verify MIR output correctness
3. Runtime test: parse "12345" → 12345

**Success Criteria**:
- JsonParserBox._atoi compiles via JoinIR Pattern2
- Correct MIR: multiply-add sequence
- Runtime correctness: atoi tests pass

---

## Section 8: Future Extensions

### 8.1 Other Bases

**Current**: `base` is extracted from AST (any integer constant).

**Example**:
- Binary: `v = v * 2 + bit`
- Hex: `v = v * 16 + hex_digit`

**No change needed**: Design already supports arbitrary bases.

### 8.2 Subtraction Variant

**Pattern**: `v = v * 10 - offset`

**Extension**:
```rust
fn classify_number_update(lhs: &str, rhs_ast: &ASTNode) -> UpdateKind {
    match rhs_ast {
        BinaryOp { op: Add | Sub, left: mul_expr, right: addend } => {
            // ... (same logic for both Add and Sub)
        }
        _ => UpdateKind::Complex,
    }
}
```

**Decision**: Support both Add and Sub in Phase 190-impl-A.

### 8.3 Complex Addends

**Examples**:
- `v = v * 10 + (ch - '0')`
- `v = v * 10 + digits.indexOf(ch)`

**Strategy**:
- Phase 190: Reject as Complex (Fail-Fast)
- Phase 191+: Extend to handle BinaryOp/MethodCall in addend
  - Emit extra JoinIR instructions for addend computation
  - Store addend in temporary ValueId

**Not in scope for Phase 190**.

### 8.4 Multi-Base Patterns

**Example** (unlikely):
```nyash
v1 = v1 * 10 + d1
v2 = v2 * 16 + d2
```

**Current Design**: Each carrier gets its own `NumberAccumulation { base }`.
Already supports different bases per carrier.

---

## Section 9: Testing Strategy

### 9.1 Unit Tests (LoopUpdateAnalyzer)

**Coverage Matrix**:

| Test Case | Pattern | Expected Kind |
|-----------|---------|---------------|
| `v = v * 10 + pos` | Basic NumberAccumulation | NumberAccumulation { base: 10 } |
| `v = v * 10 + 5` | Const addend | NumberAccumulation { base: 10 } |
| `v = v * 2 + bit` | Binary base | NumberAccumulation { base: 2 } |
| `v = v * 10 - offset` | Subtraction | NumberAccumulation { base: 10 } |
| `v = v * base + x` | Variable base | Complex |
| `v = v * 10 + f(x)` | Method call | Complex |
| `v = v * 10 + v` | LHS appears 2x | Complex |

**Implementation**:
```rust
#[test]
fn test_number_accumulation_basic() {
    let ast = parse("v = v * 10 + pos");
    let kind = LoopUpdateAnalyzer::classify_update("v", &ast);
    assert!(matches!(kind, UpdateKind::NumberAccumulation { base: 10 }));
}
```

### 9.2 Integration Tests (Pattern2)

**Test Files**:
1. `apps/tests/phase190_number_update_basic.hako`
   - Single carrier: `v = v * 10 + i`
   - Verify JoinIR emission

2. `apps/tests/phase190_number_update_multi.hako`
   - Two carriers: counter + accumulator
   - Verify multi-carrier lowering

3. `apps/tests/phase190_number_update_complex.hako`
   - Complex pattern (method call)
   - Verify Fail-Fast rejection

### 9.3 E2E Tests (JsonParserBox)

**Test Cases**:
1. `test_jsonparser_atoi_simple.hako`
   - Input: "123"
   - Expected: 123

2. `test_jsonparser_atoi_negative.hako`
   - Input: "-456"
   - Expected: -456

3. `test_jsonparser_parse_number_min_v2.hako`
   - Full `_parse_number` method
   - Verify string + number accumulation

**Success Criteria**:
- All tests pass with JoinIR enabled
- Same behavior as LoopBuilder (deleted, but historical behavior)

---

## Section 10: Migration Notes

### 10.1 From LoopBuilder (Deleted)

**Legacy**: LoopBuilder handled all loop patterns (deleted in Phase 170).

**Current**: JoinIR Pattern1-4 handle specific patterns.

**NumberAccumulation Migration**:
- Old: LoopBuilder would blindly emit MIR for any loop
- New: Pattern2/4 detect NumberAccumulation, emit specialized JoinIR

**No fallback needed**: LoopBuilder is deleted.

### 10.2 From Pattern Detection

**Phase 170-C-1**: Name-based heuristics (e.g., `is_typical_index_name`).

**Phase 190**: Structural AST analysis (no name dependency).

**Principle**: Lowering must be name-agnostic, only detection can use names.

### 10.3 Backward Compatibility

**Existing Patterns**:
- CounterLike: `i = i + 1` (unchanged)
- AccumulationLike: `sum = sum + x` (unchanged)
- StringAppendChar: `s = s + ch` (unchanged)

**New Pattern**:
- NumberAccumulation: `v = v * 10 + digit` (additive)

**No Breaking Changes**: All existing patterns continue to work.

---

## Section 11: Open Questions

### Q1: Should we support division?

**Pattern**: `v = v / 10`

**Current**: Only +, -, *, / in BinOpKind.

**Decision**: Out of scope for Phase 190 (no real-world use case found).

### Q2: Should addend be restricted to loop params only?

**Current Design**: Allow any Variable (loop param, outer local, carrier).

**Alternative**: Only allow LoopParam variables in addend.

**Decision**: Current design is more flexible, restrict if problems arise.

### Q3: How to handle floating-point bases?

**Example**: `v = v * 1.5 + x`

**Current**: `base: i64` only supports integers.

**Decision**: Out of scope (no real-world use case in Nyash codebase).

---

## Section 12: Success Metrics

### 12.1 Functional Metrics

- ✅ JsonParserBox._atoi compiles via JoinIR
- ✅ phase189_atoi_mini.hako passes
- ✅ Complex patterns rejected with `[joinir/freeze]`
- ✅ No silent fallbacks (Fail-Fast verified)

### 12.2 Code Quality Metrics

- ✅ No name-based heuristics in lowering
- ✅ Type safety enforced (Integer only)
- ✅ Unit test coverage > 80%
- ✅ Documentation updated (this doc + overview)

### 12.3 Performance Metrics

**Not a goal for Phase 190**: Focus on correctness, not optimization.

**Future**: Phase 191+ can optimize mul+add to single instruction if needed.

---

## Appendix A: AST Examples

### A.1 Basic NumberAccumulation

**Source**:
```nyash
v = v * 10 + pos
```

**AST**:
```
Assignment {
    target: Variable { name: "v" },
    value: BinaryOp {
        operator: Add,
        left: BinaryOp {
            operator: Mul,
            left: Variable { name: "v" },
            right: Literal { value: Integer(10) },
        },
        right: Variable { name: "pos" },
    },
}
```

### A.2 Complex Pattern (Rejected)

**Source**:
```nyash
v = v * 10 + digits.indexOf(ch)
```

**AST**:
```
Assignment {
    target: Variable { name: "v" },
    value: BinaryOp {
        operator: Add,
        left: BinaryOp {
            operator: Mul,
            left: Variable { name: "v" },
            right: Literal { value: Integer(10) },
        },
        right: MethodCall {
            receiver: Variable { name: "digits" },
            method: "indexOf",
            args: [Variable { name: "ch" }],
        },
    },
}
```

**Decision**: Rejected as Complex (MethodCall in addend).

---

## Appendix B: References

### B.1 Related Phases

- **Phase 170-C-1**: Carrier name heuristics
- **Phase 170-C-2**: LoopUpdateSummary skeleton
- **Phase 176**: Multi-carrier lowering
- **Phase 178**: String update detection

### B.2 Related Files

**Core**:
- `src/mir/join_ir/lowering/loop_update_summary.rs`
- `src/mir/join_ir/lowering/loop_update_analyzer.rs`
- `src/mir/join_ir/lowering/carrier_update_lowerer.rs`

**Patterns**:
- `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`
- `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs`

**Tests**:
- `tools/hako_shared/json_parser.hako` (line 436: _atoi)
- `apps/tests/phase189_atoi_mini.hako`

### B.3 Design Principles

1. **Fail-Fast**: Reject Complex patterns explicitly
2. **Whitelist Control**: Only safe patterns allowed
3. **Type Safety**: Integer-only for NumberAccumulation
4. **Name Agnostic**: No name-based lowering
5. **SSOT**: LoopHeader PHI as single source of truth

---

## Revision History

- **2025-12-09**: Initial design (Section 1-12)
- **2025-12-09**: Phase 190-impl 完了
  - Phase 190-impl-A: LoopUpdateAnalyzer に NumberAccumulation 検出実装
  - Phase 190-impl-B: CarrierUpdateLowerer で 2-instruction emission 実装
  - Phase 190-impl-C: Pattern2 can_lower ホワイトリスト更新
  - Phase 190-impl-D: E2E 検証成功 + PHI 配線修正
    - **バグ発見**: body-local と carrier の ValueId 衝突問題
    - **修正**: `body_local_start_offset = env.len() + carrier_info.carriers.len()` で安全な ValueId 空間分割
    - **E2E 結果**: `phase190_atoi_impl.hako` → 12 ✅、`phase190_parse_number_impl.hako` → 123 ✅
    - **制約**: body-local 変数 assignment は JoinIR 未対応（Phase 186 残タスク）
  - ExitLine contract Verifier 追加（`#[cfg(debug_assertions)]`）
Status: Historical
