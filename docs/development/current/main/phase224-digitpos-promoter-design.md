# Phase 224: A-4 DigitPos Promoter Design

## Purpose

Implement carrier promotion support for the **A-4 Digit Position** pattern, enabling cascading LoopBodyLocal conditions like:

```nyash
loop(p < s.length()) {
    local ch = s.substring(p, p+1)           // First LoopBodyLocal
    local digit_pos = digits.indexOf(ch)     // Second LoopBodyLocal (depends on ch)

    if digit_pos < 0 {                       // Break condition uses digit_pos
        break
    }

    // Continue processing...
    p = p + 1
}
```

This pattern is blocked by Phase 223's Fail-Fast mechanism because `digit_pos` is a LoopBodyLocal variable appearing in the loop condition.

---

## Pattern Characteristics

### A-4 Pattern Structure

**Category**: A-4 Cascading LoopBodyLocal (from phase223-loopbodylocal-condition-inventory.md)

**Key Elements**:
1. **First LoopBodyLocal**: `ch = s.substring(...)` (substring extraction)
2. **Second LoopBodyLocal**: `digit_pos = digits.indexOf(ch)` (depends on first)
3. **Condition**: `if digit_pos < 0 { break }` (comparison, NOT equality)
4. **Dependency Chain**: `s` → `ch` → `digit_pos` → condition

**AST Structure Requirements**:
- First variable must be defined by `substring()` method call
- Second variable must be defined by `indexOf()` method call
- Second variable depends on first variable (cascading)
- Break condition uses comparison operator (`<`, `>`, `<=`, `>=`, `!=`)
- NOT equality operator (`==`) like A-3 Trim pattern

---

## Design Principles

### 1. Box-First Architecture

Following Nyash's "Everything is Box" philosophy:

- **DigitPosPromoter**: Single responsibility - detect and promote A-4 pattern
- **LoopBodyCondPromoter**: Orchestrator - delegates to specialized promoters
- **CarrierInfo**: Carrier metadata container

### 2. Separation of Concerns

**LoopBodyCarrierPromoter** (existing):
- Handles A-3 Trim pattern (equality-based)
- substring() + equality chain (`ch == " " || ch == "\t"`)
- Remains specialized for Trim/whitespace patterns

**DigitPosPromoter** (new):
- Handles A-4 Digit Position pattern (comparison-based)
- substring() → indexOf() → comparison
- Specialized for cascading indexOf patterns

**LoopBodyCondPromoter** (orchestrator):
- Tries Trim promotion first (A-3)
- Falls back to DigitPos promotion (A-4)
- Returns first successful promotion or CannotPromote

### 3. Fail-Fast Philosophy

- Explicit error messages for unsupported patterns
- Early return on detection failure
- Clear distinction between "safe" and "complex" patterns

---

## API Design

### DigitPosPromotionRequest

```rust
pub struct DigitPosPromotionRequest<'a> {
    /// Loop parameter name (e.g., "p")
    pub loop_param_name: &'a str,

    /// Condition scope analysis result
    pub cond_scope: &'a LoopConditionScope,

    /// Loop structure metadata (for future use)
    pub scope_shape: Option<&'a LoopScopeShape>,

    /// Break condition AST (Pattern2: Some, Pattern4: None)
    pub break_cond: Option<&'a ASTNode>,

    /// Continue condition AST (Pattern4: Some, Pattern2: None)
    pub continue_cond: Option<&'a ASTNode>,

    /// Loop body statements
    pub loop_body: &'a [ASTNode],
}
```

### DigitPosPromotionResult

```rust
pub enum DigitPosPromotionResult {
    /// Promotion successful
    Promoted {
        /// Carrier metadata (for Pattern2/Pattern4 integration)
        carrier_info: CarrierInfo,

        /// Variable name that was promoted (e.g., "digit_pos")
        promoted_var: String,

        /// Promoted carrier name (e.g., "is_digit")
        carrier_name: String,
    },

    /// Cannot promote (Fail-Fast)
    CannotPromote {
        /// Human-readable reason
        reason: String,

        /// List of problematic LoopBodyLocal variables
        vars: Vec<String>,
    },
}
```

### DigitPosPromoter

```rust
pub struct DigitPosPromoter;

impl DigitPosPromoter {
    /// Try to promote A-4 pattern (cascading indexOf)
    pub fn try_promote(req: DigitPosPromotionRequest) -> DigitPosPromotionResult;

    // Private helpers
    fn find_indexOf_definition<'a>(body: &'a [ASTNode], var_name: &str) -> Option<&'a ASTNode>;
    fn is_indexOf_method_call(node: &ASTNode) -> bool;
    fn extract_comparison_var(cond: &ASTNode) -> Option<String>;
    fn find_first_loopbodylocal_dependency<'a>(
        body: &'a [ASTNode],
        indexOf_call: &'a ASTNode
    ) -> Option<&'a str>;
}
```

---

## Detection Algorithm

### Step 1: Extract LoopBodyLocal Variables

From `LoopConditionScope`, extract all variables with `CondVarScope::LoopBodyLocal`.

**Expected for A-4 pattern**: `["ch", "digit_pos"]` (2 variables)

### Step 2: Find indexOf() Definition

For each LoopBodyLocal variable, find its definition in loop body:

```rust
// Search for: local digit_pos = digits.indexOf(ch)
let def_node = find_indexOf_definition(loop_body, "digit_pos");
if !is_indexOf_method_call(def_node) {
    return CannotPromote("Not an indexOf() pattern");
}
```

### Step 3: Extract Comparison Variable

Extract the variable used in break/continue condition:

```rust
// Pattern: if digit_pos < 0 { break }
let cond = break_cond.or(continue_cond);
let var_in_cond = extract_comparison_var(cond);
```

### Step 4: Verify Cascading Dependency

Check that indexOf() call depends on another LoopBodyLocal (e.g., `ch`):

```rust
// digits.indexOf(ch) - extract "ch"
let dependency = find_first_loopbodylocal_dependency(loop_body, indexOf_call);
if dependency.is_none() {
    return CannotPromote("indexOf() does not depend on LoopBodyLocal");
}
```

### Step 5: Build CarrierInfo

```rust
// Promote to bool carrier: is_digit = (digit_pos >= 0)
let carrier_name = format!("is_{}", var_in_cond);
let carrier_info = CarrierInfo::with_carriers(
    carrier_name.clone(),
    ValueId(0),  // Placeholder (will be remapped)
    vec![],
);

return Promoted {
    carrier_info,
    promoted_var: var_in_cond,
    carrier_name,
};
```

---

## CarrierInfo Construction

### Carrier Type Decision

**Option A: bool carrier** (Recommended)
```rust
carrier_name: "is_digit"
carrier_type: bool
initialization: is_digit = (digit_pos >= 0)
```

**Rationale**:
- Consistent with A-3 Trim pattern (also bool)
- Simplifies condition rewriting
- Pattern2/Pattern4 integration expects bool carriers

**Option B: int carrier**
```rust
carrier_name: "digit_pos_carrier"
carrier_type: int
initialization: digit_pos_carrier = digits.indexOf(ch)
```

**Rationale**:
- Preserves original int value
- Could be useful for downstream computations

**Decision**: Use **Option A (bool carrier)** for P0 implementation. Option B can be added later if needed.

### Carrier Initialization Logic

```rust
// At loop entry (before first iteration):
local ch = s.substring(p, p+1)
local digit_pos = digits.indexOf(ch)
local is_digit = (digit_pos >= 0)  // Bool carrier

// Loop condition uses carrier:
loop(p < n && is_digit) {
    // Body uses original digit_pos if needed
    num_str = num_str + ch
    p = p + 1

    // Update carrier at end of body:
    ch = s.substring(p, p+1)
    digit_pos = digits.indexOf(ch)
    is_digit = (digit_pos >= 0)
}
```

---

## LoopBodyCondPromoter Integration

### Current Implementation (A-3 Only)

```rust
pub fn try_promote_for_condition(req: ConditionPromotionRequest) -> ConditionPromotionResult {
    // Build request for LoopBodyCarrierPromoter (A-3 Trim)
    let promotion_request = PromotionRequest { ... };

    match LoopBodyCarrierPromoter::try_promote(&promotion_request) {
        PromotionResult::Promoted { trim_info } => {
            // Return Promoted with CarrierInfo
        }
        PromotionResult::CannotPromote { reason, vars } => {
            // Fail-Fast
        }
    }
}
```

### Extended Implementation (A-3 → A-4 Cascade)

```rust
pub fn try_promote_for_condition(req: ConditionPromotionRequest) -> ConditionPromotionResult {
    // Step 1: Try Trim promotion (A-3 pattern)
    let trim_request = PromotionRequest { ... };
    match LoopBodyCarrierPromoter::try_promote(&trim_request) {
        PromotionResult::Promoted { trim_info } => {
            eprintln!("[cond_promoter] A-3 Trim pattern promoted");
            return ConditionPromotionResult::Promoted {
                carrier_info: trim_info.to_carrier_info(),
                promoted_var: trim_info.var_name,
                carrier_name: trim_info.carrier_name,
            };
        }
        PromotionResult::CannotPromote { .. } => {
            eprintln!("[cond_promoter] A-3 Trim promotion failed, trying A-4 DigitPos");
        }
    }

    // Step 2: Try DigitPos promotion (A-4 pattern)
    let digitpos_request = DigitPosPromotionRequest {
        loop_param_name: req.loop_param_name,
        cond_scope: req.cond_scope,
        scope_shape: req.scope_shape,
        break_cond: req.break_cond,
        continue_cond: req.continue_cond,
        loop_body: req.loop_body,
    };

    match DigitPosPromoter::try_promote(digitpos_request) {
        DigitPosPromotionResult::Promoted { carrier_info, promoted_var, carrier_name } => {
            eprintln!("[cond_promoter] A-4 DigitPos pattern promoted");
            return ConditionPromotionResult::Promoted {
                carrier_info,
                promoted_var,
                carrier_name,
            };
        }
        DigitPosPromotionResult::CannotPromote { reason, vars } => {
            eprintln!("[cond_promoter] A-4 DigitPos promotion failed: {}", reason);
        }
    }

    // Step 3: Fail-Fast (no pattern matched)
    ConditionPromotionResult::CannotPromote {
        reason: "No promotable pattern detected (tried A-3 Trim, A-4 DigitPos)".to_string(),
        vars: extract_body_local_names(&req.cond_scope.vars),
    }
}
```

---

## Pattern Detection Edge Cases

### Edge Case 1: Multiple indexOf() Calls

```nyash
local digit_pos1 = digits.indexOf(ch1)
local digit_pos2 = digits.indexOf(ch2)
```

**Handling**: Promote the variable that appears in the condition. Use `extract_comparison_var()` to identify it.

### Edge Case 2: indexOf() on Non-LoopBodyLocal

```nyash
local pos = fixed_string.indexOf(ch)  // fixed_string is outer variable
```

**Handling**: This is NOT a cascading pattern. Return `CannotPromote("indexOf() does not depend on LoopBodyLocal")`.

### Edge Case 3: Comparison with Non-Zero

```nyash
if digit_pos < 5 { break }  // Not the standard "< 0" pattern
```

**Handling**: Still promote - comparison operator is what matters, not the literal value. The carrier becomes `is_digit = (digit_pos < 5)`.

### Edge Case 4: indexOf() with Multiple Arguments

```nyash
local pos = s.indexOf(ch, start_index)  // indexOf with start position
```

**Handling**: Still promote - as long as one argument is a LoopBodyLocal, it's a valid cascading pattern.

---

## Testing Strategy

### Unit Tests (in loop_body_digitpos_promoter.rs)

```rust
#[test]
fn test_digitpos_promoter_basic_pattern() {
    // ch = s.substring(...) → digit_pos = digits.indexOf(ch) → if digit_pos < 0
    // Expected: Promoted
}

#[test]
fn test_digitpos_promoter_non_indexOf_method() {
    // ch = s.substring(...) → pos = s.length() → if pos < 0
    // Expected: CannotPromote
}

#[test]
fn test_digitpos_promoter_no_loopbodylocal_dependency() {
    // digit_pos = fixed_string.indexOf("x")  // No LoopBodyLocal dependency
    // Expected: CannotPromote
}

#[test]
fn test_digitpos_promoter_comparison_operators() {
    // Test <, >, <=, >=, != operators
    // Expected: All should be Promoted
}

#[test]
fn test_digitpos_promoter_equality_operator() {
    // if digit_pos == -1 { break }  // Equality, not comparison
    // Expected: CannotPromote (this is A-3 Trim territory)
}
```

### Integration Test (Pattern2/Pattern4)

**Test File**: `apps/tests/phase2235_p2_digit_pos_min.hako`

**Before Phase 224**:
```
[joinir/freeze] LoopBodyLocal in condition: ["digit_pos"]
Cannot promote LoopBodyLocal variables ["digit_pos"]: No promotable Trim pattern detected
```

**After Phase 224**:
```
[cond_promoter] A-3 Trim promotion failed, trying A-4 DigitPos
[cond_promoter] A-4 DigitPos pattern promoted: digit_pos → is_digit
[pattern2/lowering] Using promoted carrier: is_digit
```

### E2E Test

```bash
# Compile and run
NYASH_JOINIR_DEBUG=1 ./target/release/hakorune apps/tests/phase2235_p2_digit_pos_min.hako

# Expected output:
# p = 3
# num_str = 123
# (No [joinir/freeze] error)
```

---

## Comparison: A-3 Trim vs A-4 DigitPos

| Feature | A-3 Trim (LoopBodyCarrierPromoter) | A-4 DigitPos (DigitPosPromoter) |
|---------|-----------------------------------|--------------------------------|
| **Method Call** | `substring()` | `substring()` → `indexOf()` |
| **Dependency** | Single LoopBodyLocal (`ch`) | Cascading (`ch` → `digit_pos`) |
| **Condition Type** | Equality (`==`, `!=`) | Comparison (`<`, `>`, `<=`, `>=`) |
| **Condition Structure** | OR chain: `ch == " "` \|\| `ch == "\t"` | Single comparison: `digit_pos < 0` |
| **Carrier Type** | Bool (`is_whitespace`) | Bool (`is_digit`) |
| **Pattern Count** | 1 variable | 2 variables (cascading) |

---

## Future Extensions (Post-P0)

### A-5: Multi-Variable Patterns (P2)

```nyash
local ch_s = s.substring(...)
local ch_lit = literal.substring(...)
if ch_s != ch_lit { break }
```

**Challenge**: Two independent LoopBodyLocal variables, not cascading.

### A-6: Multiple Break Conditions (P2)

```nyash
if ch < "0" || ch > "9" { break }  // Range check
if pos < 0 { break }                // indexOf check
```

**Challenge**: Two separate break conditions using different variables.

### Option B: Int Carrier Support

If downstream code needs the actual `digit_pos` value (not just bool), implement int carrier variant:

```rust
carrier_name: "digit_pos_carrier"
carrier_type: int
initialization: digit_pos_carrier = digits.indexOf(ch)
condition: loop(p < n && digit_pos_carrier >= 0)
```

---

## File Structure

```
src/mir/loop_pattern_detection/
├── loop_body_carrier_promoter.rs    (existing - A-3 Trim)
├── loop_body_digitpos_promoter.rs   (NEW - A-4 DigitPos)
├── loop_body_cond_promoter.rs       (modified - orchestrator)
└── mod.rs                           (add pub mod)
```

---

## Success Criteria

1. **Build Success**: `cargo build --release` with 0 errors, 0 warnings
2. **Unit Tests Pass**: All tests in `loop_body_digitpos_promoter.rs` pass
3. **Integration Test**: `apps/tests/phase2235_p2_digit_pos_min.hako` no longer Fail-Fasts
4. **E2E Test**: Program executes and outputs correct result (`p = 3`, `num_str = 123`)
5. **No Regression**: Existing A-3 Trim tests still pass (e.g., `skip_whitespace` tests)
6. **Documentation**: This design doc + updated architecture overview

---

## Implementation Order

1. **224-2**: This design document ✅
2. **224-3**: Implement `DigitPosPromoter` with unit tests
3. **224-4**: Integrate into `LoopBodyCondPromoter`
4. **224-5**: E2E test with `phase2235_p2_digit_pos_min.hako`
5. **224-6**: Update docs and CURRENT_TASK.md

---

## Revision History

- **2025-12-10**: Phase 224 design document created
Status: Active  
Scope: digitpos promoter 設計（ExprLowerer ライン）
