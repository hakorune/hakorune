# Phase 183: LoopBodyLocal Role Separation Design

## Overview

Phase 182 discovered **Blocker 1**: LoopBodyLocal variables are currently always routed to Trim-specific carrier promotion logic, which is inappropriate for JsonParser integer loops where these variables are simple local computations.

This phase separates LoopBodyLocal variables into two categories based on their usage pattern:

1. **Condition LoopBodyLocal**: Used in loop conditions (header/break/continue) → Needs Trim promotion
2. **Body-only LoopBodyLocal**: Only used in loop body, never in conditions → No promotion needed

## Problem Statement

### Current Behavior (Phase 182 Blockers)

```nyash
// Example: _parse_number
loop(p < s.length()) {
    local digit_pos = "0123456789".indexOf(ch)  // LoopBodyLocal: digit_pos
    if (digit_pos < 0) {
        break  // digit_pos used in BREAK condition
    }
    num_str = num_str + ch
    p = p + 1
}
```

**Current routing**:
- `digit_pos` is detected as LoopBodyLocal (defined in body)
- Pattern2 tries to apply Trim carrier promotion
- **Error**: Not a Trim pattern (`indexOf` vs `substring`)

**Desired behavior**:
- `digit_pos` used in condition → Should attempt Trim promotion (and fail gracefully)
- But if `digit_pos` were only in body → Should be allowed as pure local variable

### Use Cases

#### Case A: Condition LoopBodyLocal (Trim Pattern)
```nyash
// _trim_leading_whitespace
loop(pos < s.length()) {
    local ch = s.substring(pos, pos + 1)  // LoopBodyLocal: ch
    if (ch == " " || ch == "\t") {        // ch in BREAK condition
        pos = pos + 1
    } else {
        break
    }
}
```
**Routing**: Needs Trim promotion (`ch` → `is_whitespace` carrier)

#### Case B: Body-only LoopBodyLocal (Pure Local)
```nyash
// Hypothetical simple loop
loop(i < n) {
    local temp = i * 2      // LoopBodyLocal: temp (not in any condition!)
    result = result + temp
    i = i + 1
}
```
**Routing**: No promotion needed (`temp` never used in conditions)

#### Case C: Condition LoopBodyLocal (Non-Trim)
```nyash
// _parse_number
loop(p < s.length()) {
    local digit_pos = "0123456789".indexOf(ch)  // LoopBodyLocal: digit_pos
    if (digit_pos < 0) {                         // digit_pos in BREAK condition
        break
    }
    p = p + 1
}
```
**Current**: Tries Trim promotion → Fails
**Desired**: Recognize non-Trim pattern → **Block with clear error message**

## Design Solution

### Architecture: Two-Stage Check

```
LoopConditionScopeBox
    ↓
has_loop_body_local() ?
    ↓ YES
Check: Where is LoopBodyLocal used?
    ├─ In CONDITION (header/break/continue) → Try Trim promotion
    │   ├─ Success → Pattern2/4 with Trim carrier
    │   └─ Fail → Reject loop (not supported yet)
    └─ Body-only (NOT in any condition) → Allow as pure local
```

### Implementation Strategy

#### Step 1: Extend LoopConditionScope Analysis

Add `is_in_condition()` check to differentiate:
- Condition LoopBodyLocal: Used in header/break/continue conditions
- Body-only LoopBodyLocal: Only in body assignments/expressions

```rust
impl LoopConditionScope {
    /// Check if a LoopBodyLocal is used in any condition
    pub fn is_body_local_in_condition(&self, var_name: &str) -> bool {
        // Implementation: Check if var_name appears in condition_nodes
    }
}
```

#### Step 2: Update TrimLoopLowerer

Modify `try_lower_trim_like_loop()` to:
1. Filter LoopBodyLocal to only process **condition LoopBodyLocal**
2. Skip body-only LoopBodyLocal (let Pattern1/2 handle naturally)

```rust
impl TrimLoopLowerer {
    pub fn try_lower_trim_like_loop(...) -> Result<Option<TrimLoweringResult>, String> {
        // Extract condition LoopBodyLocal only
        let cond_body_locals: Vec<_> = cond_scope.vars.iter()
            .filter(|v| v.scope == CondVarScope::LoopBodyLocal)
            .filter(|v| Self::is_used_in_condition(v.name, break_cond))
            .collect();

        if cond_body_locals.is_empty() {
            // No condition LoopBodyLocal → Not a Trim pattern
            return Ok(None);
        }

        // Try promotion for condition LoopBodyLocal
        // ...
    }
}
```

#### Step 3: Update Pattern2 can_lower

Ensure Pattern2 accepts loops with body-only LoopBodyLocal:

```rust
pub fn can_lower(builder: &MirBuilder, ctx: &LoopPatternContext) -> bool {
    // Existing checks...

    // NEW: Allow body-only LoopBodyLocal
    let cond_scope = &ctx.preprocessing.cond_scope;
    if cond_scope.has_loop_body_local() {
        // Check if all LoopBodyLocal are body-only (not in conditions)
        let all_body_only = cond_scope.vars.iter()
            .filter(|v| v.scope == CondVarScope::LoopBodyLocal)
            .all(|v| !is_in_any_condition(v.name, ctx));

        if !all_body_only {
            // Some LoopBodyLocal in conditions → Must be Trim pattern
            // Trim lowering will handle this
        }
    }

    true
}
```

## Implementation Plan

### Task 183-2: Core Implementation

1. **Add condition detection helper** (10 lines)
   - `TrimLoopLowerer::is_used_in_condition(var_name, cond_node)`
   - Simple AST traversal to check if variable appears

2. **Update Trim detection** (20 lines)
   - Filter LoopBodyLocal to condition-only
   - Skip body-only LoopBodyLocal

3. **Add unit test** (50 lines)
   - `test_body_only_loopbodylocal_allowed`
   - Loop with `local temp` never used in condition
   - Should NOT trigger Trim promotion

### Task 183-3: Integration Tests

Create 3 test files demonstrating the fix:

1. **phase183_p2_parse_number.hako** - _parse_number pattern
   - `digit_pos` in break condition → Should reject (not Trim)
   - Clear error message: "LoopBodyLocal in condition, but not Trim pattern"

2. **phase183_p2_atoi.hako** - _atoi pattern
   - Similar to parse_number
   - Multiple break conditions

3. **phase183_p1_match_literal.hako** - _match_literal pattern
   - No LoopBodyLocal → Should work (baseline)

## Validation Strategy

### Success Criteria

1. **Body-only LoopBodyLocal**: Loops with body-only locals compile successfully
2. **Condition LoopBodyLocal**:
   - Trim patterns → Promoted correctly
   - Non-Trim patterns → Rejected with clear error
3. **No regression**: Existing Trim tests still pass

### Test Commands

```bash
# Structure trace (verify no freeze)
NYASH_JOINIR_STRUCTURE_ONLY=1 ./target/release/hakorune apps/tests/phase183_p2_parse_number.hako

# Execution test (once promotion logic is ready)
NYASH_JOINIR_CORE=1 ./target/release/hakorune apps/tests/phase183_p1_match_literal.hako
```

## Future Work (Out of Scope)

### Phase 184+: Non-Trim LoopBodyLocal Patterns

To support `_parse_number` and `_atoi` fully, we need:

1. **Generic LoopBodyLocal promotion**
   - Pattern: `local x = expr; if (x op literal) break`
   - Promotion: Evaluate `expr` inline, no carrier needed
   - Alternative: Allow inline computation in JoinIR conditions

2. **String concatenation support** (Phase 178 blocker)
   - `num_str = num_str + ch` currently rejected
   - Need string carrier update support in Pattern2/4

**Decision for Phase 183**:
- Focus on architectural separation (condition vs body-only)
- Accept that `_parse_number`/`_atoi` will still fail (but with better error)
- Unblock body-only LoopBodyLocal use cases

## References

- Phase 182: JsonParser P1/P2 pattern validation (discovered blockers)
- Phase 181: JsonParser loop inventory
- Phase 171-C: LoopBodyCarrierPromoter original design
- Phase 170-D: LoopConditionScopeBox implementation
Status: Historical
