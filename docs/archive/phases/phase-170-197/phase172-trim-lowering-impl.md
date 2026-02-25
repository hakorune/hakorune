# Phase 172: Trim Pattern JoinIR Lowering Implementation

## Objective

Implement actual JoinIR → MIR lowering for Trim pattern loops, making `test_trim_main_pattern.hako` work end-to-end.

## Target Loop

**File**: `local_tests/test_trim_main_pattern.hako`

**Leading whitespace trim loop**:
```nyash
loop(start < end) {
  local ch = s.substring(start, start+1)
  if ch == " " || ch == "\t" || ch == "\n" || ch == "\r" {
    start = start + 1
  } else {
    break
  }
}
```

## Using Boxes from Phase 171

### Phase 171 Infrastructure
- ✅ **LoopConditionScopeBox**: Detects `ch` as LoopBodyLocal
- ✅ **LoopBodyCarrierPromoter**: Promotes `ch` to bool carrier `is_ch_match`
- ✅ **TrimPatternInfo**: Contains promotion metadata
- ✅ **TrimLoopHelper**: Attached to CarrierInfo for lowering guidance
- ✅ **Pattern2 validation**: Currently returns informative error at line 234-239

## Design Philosophy

### Responsibility Separation

**Host MIR Side** (before/after JoinIR):
- Execute `substring()` method calls
- Evaluate OR chain comparisons (`ch == " " || ch == "\t" || ...`)
- Generate bool result for carrier
- BoxCall operations

**JoinIR Side** (pure control flow):
- bool carrier-based loop control
- break condition: `!is_ch_match`
- No knowledge of substring/OR chain
- No new JoinIR instructions

### Key Insight

The Trim pattern transformation:
```
Original:
  local ch = s.substring(start, start+1)
  if (ch == " " || ch == "\t" || ...) { start++ } else { break }

Transformed:
  is_ch_match = (s.substring(start, start+1) == " " || ...)  # Host MIR
  if (!is_ch_match) { break }                                  # JoinIR
```

## Implementation Steps

### Step 1: Loop Pre-Initialization (Task 172-2)

**Location**: `pattern2_with_break.rs`, before JoinIR generation

**Goal**: Initialize bool carrier before entering loop

**Implementation**:
```rust
// After Line 264 (after carrier_info.merge_from)
if let Some(helper) = carrier_info.trim_helper() {
    if helper.is_safe_trim() {
        // Generate initial whitespace check
        // ch0 = s.substring(start, start+1)
        let ch0 = self.emit_method_call("substring", s_id, vec![start_id, start_plus_1_id])?;

        // is_ch_match0 = (ch0 == " " || ch0 == "\t" || ...)
        let is_ch_match0 = emit_whitespace_check(self, ch0, &helper.whitespace_chars)?;

        // Update carrier_info initial value
        // (Will be used in JoinIR input mapping)
    }
}
```

**Helper Function** (new):
```rust
fn emit_whitespace_check(
    builder: &mut MirBuilder,
    ch_value: ValueId,
    whitespace_chars: &[String],
) -> Result<ValueId, String> {
    // Build OR chain: ch == " " || ch == "\t" || ...
    let mut result = None;
    for ws_char in whitespace_chars {
        let ws_const = emit_string(builder, ws_char.clone());
        let eq_check = emit_eq_to(builder, ch_value, ws_const)?;

        result = Some(if let Some(prev) = result {
            // prev || eq_check
            emit_binary_op(builder, BinaryOp::Or, prev, eq_check)?
        } else {
            eq_check
        });
    }
    result.ok_or("Empty whitespace_chars".to_string())
}
```

### Step 2: Loop Body Update Logic (Task 172-3)

**Location**: Pattern2 lowerer's latch generation (after `start = start + 1`)

**Goal**: Update bool carrier for next iteration

**Implementation**:
```rust
// In latch block generation (conceptually after line 267+)
if let Some(helper) = carrier_info.trim_helper() {
    if helper.is_safe_trim() {
        // ch_next = s.substring(start_next, start_next+1)
        let ch_next = self.emit_method_call("substring", s_id, vec![start_next, start_next_plus_1])?;

        // is_ch_match_next = (ch_next == " " || ...)
        let is_ch_match_next = emit_whitespace_check(self, ch_next, &helper.whitespace_chars)?;

        // Store for JoinIR latch → header PHI
        // (Will be used in carrier update mapping)
    }
}
```

**Note**: The actual MIR emission happens in **host space**, not JoinIR space. The JoinIR only sees the bool carrier as a loop parameter.

### Step 3: JoinIR break Condition Replacement (Task 172-4)

**Location**: `lower_loop_with_break_minimal` call at line 267

**Goal**: Replace break condition with `!is_ch_match`

**Current Code**:
```rust
let (join_module, fragment_meta) = match lower_loop_with_break_minimal(
    scope,
    condition,        // loop condition: start < end
    &break_condition_node,  // if condition: ch == " " || ...
    &env,
    &loop_var_name
) { ... }
```

**Modified Approach**:
```rust
// Phase 172: Trim pattern special route
if let Some(helper) = carrier_info.trim_helper() {
    if helper.is_safe_trim() {
        // Create negated carrier check: !is_ch_match
        let carrier_var_node = ASTNode::Variable {
            name: helper.carrier_name.clone(), // "is_ch_match"
            span: Span::unknown(),
        };

        let negated_carrier_check = ASTNode::UnaryOp {
            operator: UnaryOperator::Not,
            operand: Box::new(carrier_var_node),
            span: Span::unknown(),
        };

        // Use negated carrier as break condition
        let (join_module, fragment_meta) = match lower_loop_with_break_minimal(
            scope,
            condition,  // start < end (unchanged)
            &negated_carrier_check,  // !is_ch_match (REPLACED)
            &env,
            &loop_var_name
        ) { ... }

        // Continue with normal merge flow...
    }
}
```

**Critical Design Decision**:
- The break condition AST is replaced, but the loop condition (`start < end`) remains unchanged
- JoinIR sees bool carrier as just another loop parameter
- No new JoinIR instructions needed!

### Step 4: Carrier Mapping Integration

**Location**: Boundary setup (line 283-291)

**Goal**: Ensure bool carrier is properly mapped between host and JoinIR

**Current boundary setup**:
```rust
let mut boundary = JoinInlineBoundary::new_inputs_only(
    vec![ValueId(0)],  // JoinIR: loop param
    vec![loop_var_id], // Host: "start"
);
boundary.condition_bindings = condition_bindings;
boundary.exit_bindings = exit_bindings.clone();
```

**Enhanced with Trim carrier**:
```rust
// Phase 172: Add bool carrier to condition_bindings
if let Some(helper) = carrier_info.trim_helper() {
    let carrier_host_id = self.variable_map.get(&helper.carrier_name)
        .copied()
        .ok_or_else(|| format!("Carrier '{}' not in variable_map", helper.carrier_name))?;

    let carrier_join_id = alloc_join_value(); // Allocate JoinIR-local ID

    env.insert(helper.carrier_name.clone(), carrier_join_id);
    condition_bindings.push(ConditionBinding {
        name: helper.carrier_name.clone(),
        host_value: carrier_host_id,
        join_value: carrier_join_id,
    });
}
```

## Implementation Constraints

### What We DON'T Change

1. **ExitLine Architecture**: No changes to ExitLineReconnector/ExitMetaCollector
2. **Header PHI Generation**: Existing LoopHeaderPhiBuilder handles bool carrier automatically
3. **JoinIR Instruction Set**: No new instructions added
4. **Pattern2 Basic Logic**: Core control flow unchanged

### What We DO Change

1. **Pre-loop initialization**: Add bool carrier setup
2. **Latch update logic**: Add bool carrier update
3. **Break condition AST**: Replace with `!is_ch_match`
4. **Condition bindings**: Add bool carrier mapping

## Testing Strategy

### E2E Test

**Command**:
```bash
./target/release/hakorune local_tests/test_trim_main_pattern.hako
```

**Expected Behavior**:
- **Before**: Error message "✅ Trim pattern validation successful! ... JoinIR lowering: TODO"
- **After**: `Result: [hello]` and `PASS: Trimmed correctly`

### Debug Traces

**With JoinIR debug**:
```bash
NYASH_JOINIR_DEBUG=1 ./target/release/hakorune local_tests/test_trim_main_pattern.hako 2>&1 | grep -E "\[pattern2\]|\[trim\]"
```

**Expected log patterns**:
```
[pattern2/promotion] LoopBodyLocal detected in condition scope
[pattern2/promoter] LoopBodyLocal 'ch' promoted to carrier 'is_ch_match'
[pattern2/trim] Safe Trim pattern detected, bypassing LoopBodyLocal restriction
[pattern2/trim] Carrier: 'is_ch_match', original var: 'ch', whitespace chars: [" ", "\t", "\n", "\r"]
[pattern2/trim] Emitting whitespace check initialization
[pattern2/trim] Replacing break condition with !is_ch_match
```

### MIR Validation

**Dump MIR**:
```bash
./target/release/hakorune --dump-mir local_tests/test_trim_main_pattern.hako 2>&1 | less
```

**Check for**:
- Pre-loop: `substring()` call + OR chain comparison
- Loop header: PHI for `is_ch_match`
- Loop body: Conditional branch on `!is_ch_match`
- Loop latch: `substring()` call + OR chain update
- Exit block: Proper carrier value propagation

## Success Criteria

1. ✅ `test_trim_main_pattern.hako` executes without errors
2. ✅ Output: `Result: [hello]` (correctly trimmed)
3. ✅ Test result: `PASS: Trimmed correctly`
4. ✅ `cargo build --release` succeeds with 0 errors
5. ✅ `cargo test` all tests pass
6. ✅ No new warnings introduced
7. ✅ MIR dump shows proper SSA form

## Implementation Status

- [ ] Task 172-1: Design document (THIS FILE)
- [ ] Task 172-2: Loop pre-initialization
- [ ] Task 172-3: Loop body update logic
- [ ] Task 172-4: JoinIR break condition replacement
- [ ] Task 172-5: E2E test validation
- [ ] Task 172-6: Documentation update

## Future Work (Out of Scope for Phase 172)

### Pattern 4 Support
- Apply same Trim lowering to Pattern 4 (loop with continue)
- Reuse `emit_whitespace_check()` helper

### JsonParser Integration
- Apply to all trim loops in JsonParser._trim
- Validate with JSON parsing smoke tests

### P6+ Patterns
- Complex control flow with Trim-like patterns
- Multi-carrier bool promotion

## Notes

### Why This Design Works

1. **Minimal Invasiveness**: Changes only affect Trim-specific paths
2. **Reusability**: `emit_whitespace_check()` can be shared with Pattern 4
3. **Maintainability**: Clear separation between host MIR and JoinIR concerns
4. **Testability**: Each step can be validated independently

### Phase 171 Validation Hook

Current code (line 234-239) already validates Trim pattern and returns early:
```rust
return Err(format!(
    "[cf_loop/pattern2] ✅ Trim pattern validation successful! \
     Carrier '{}' ready for Phase 172 implementation. \
     (Pattern detection: PASS, Safety check: PASS, JoinIR lowering: TODO)",
    helper.carrier_name
));
```

Phase 172 will **replace this early return** with actual lowering logic.

## References

- **Phase 171-C**: LoopBodyCarrierPromoter implementation
- **Phase 171-C-5**: TrimLoopHelper design
- **Pattern2 Lowerer**: `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`
- **Test File**: `local_tests/test_trim_main_pattern.hako`
Status: Historical
