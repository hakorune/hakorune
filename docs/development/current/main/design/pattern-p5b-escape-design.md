# Escape Route P5b: Escape Sequence Handling

## Overview

**Escape route P5b** extends JoinIR loop recognition to handle **variable-step carriers** in escape sequence parsing.

This route family is essential for:
- JSON string parsers
- CSV readers
- Template engine string processing
- Any escape-aware text processing loop

## Problem Statement

### Current Limitation

Standard route-family carriers always update by constant deltas:
```
Carrier i: i = i + 1 (always +1)
```

Escape sequences require conditional increments:
```
if escape_char { i = i + 2 }  // Skip both escape char and escaped char
else { i = i + 1 }            // Normal increment
```

**Why this matters**:
- Common in string parsing (JSON, CSV, config files)
- Appears in ~3 selfhost loops
- Currently forces Fail-Fast (route not supported)
- Could benefit from JoinIR exit-line optimization

### Real-World Example: JSON String Reader

```hako
loop(i < n) {
    local ch = s.substring(i, i+1)

    if ch == "\"" { break }        // End of string

    if ch == "\\" {
        i = i + 1                  // <-- CONDITIONAL: skip escape char
        ch = s.substring(i, i+1)   // Read escaped character
    }

    out = out + ch                 // Process character
    i = i + 1                      // <-- UNCONDITIONAL: advance
}
```

Loop progression:
- Normal case: `i` advances by 1
- Escape case: `i` advances by 2 (skip inside if + final increment)

## Route Definition

### Canonical Form

```
LoopSkeleton {
    steps: [
        HeaderCond(carrier < limit),
        Body(escape_check_block),
        Body(process_block),
        Update(carrier_increments)
    ]
}
```

### Header Contract

**Requirement**: Bounded loop on single integer carrier

```
loop(i < n)    ✅ Valid P5b route header
loop(i < 100)  ✅ Valid P5b route header
loop(i <= n)   ✅ Valid P5b route header (edge case)
loop(true)     ❌ Not P5b route (unbounded)
loop(i < n && j < m)  ❌ Not P5b (multi-carrier condition)
```

**Carrier**: Must be loop variable used in condition

### Escape Check Contract

**Requirement**: Conditional increment based on character test

#### Escape Detection Block

```
if ch == escape_char {
    carrier = carrier + escape_delta
    // Optional: read next character
    ch = s.substring(carrier, carrier+1)
}
```

**Escape character**: Typically `\\` (backslash), but can vary
- JSON: `\\`
- CSV: `"`  (context-dependent)
- Custom: Any single-character escape

**Escape delta**: How far to skip
- `+1`: Skip just the escape marker
- `+2`: Skip escape marker + escaped char (common case)
- `+N`: Other values possible

#### Detection Algorithm

1. **Find if statement in loop body**
2. **Check condition**: `ch == literal_char`
3. **Extract escape character**: The literal constant
4. **Find assignment in if block**: `carrier = carrier + <const>`
5. **Calculate escape_delta**: The constant value
6. **Validate**: Escape delta > 0

### Process Block Contract

**Requirement**: Character accumulation with optional processing

```
out = out + ch          ✅ Simple append
result = result + ch    ✅ Any accumulator
s = s + value           ❌ Not append pattern
```

**Accumulator carrier**: String-like box supporting append

### Update Block Contract

**Requirement**: Unconditional carrier increment after escape check

```
carrier = carrier + normal_delta
```

**Normal delta**: Almost always `+1`
- Defines "normal" loop progress
- Only incremented once per iteration (not in escape block)

#### Detection Algorithm

1. **Find assignment after escape if block**
2. **Route contract**: `carrier = carrier + <const>`
3. **Must be unconditional** (outside any if block)
4. **Extract normal_delta**: The constant

### Break Requirement

**Requirement**: Explicit break on string boundary

```
if ch == boundary_char { break }
```

**Boundary character**: Typically quote `"`
- JSON: `"`
- Custom strings: Any delimiter

**Position in loop**: Usually before escape check

### Exit Contract for P5b

```rust
ExitContract {
    has_break: true,        // Always for escape patterns
    has_continue: false,
    has_return: false,
    carriers: vec![
        CarrierInfo {
            name: "i",      // Loop variable
            deltas: [
                normal_delta,   // e.g., 1
                escape_delta    // e.g., 2
            ]
        },
        CarrierInfo {
            name: "out",    // Accumulator
            pattern: Append
        }
    ]
}
```

## Capability Analysis

### Required Capabilities (CapabilityTag)

For escape route P5b to be JoinIR-compatible, these must be present:

| Capability | Meaning | P5b Requirement | Status |
|------------|---------|-----------------|--------|
| `ConstStep` | Carrier updates are constants | ✅ Required | Both deltas constant |
| `SingleBreak` | Only one break point | ✅ Required | String boundary only |
| `PureHeader` | Condition has no side effects | ✅ Required | `i < n` is pure |
| `OuterLocalCond` | Condition doesn't reference locals | ⚠️ Soft req | Usually true |
| `ExitBindings` | Exit block is simple | ✅ Required | Break is unconditional |

### Missing Capabilities (Fail-Fast Reasons)

If any of these are detected, escape route P5b is rejected:

| Capability | Why It Blocks P5b | Example |
|------------|-------------------|---------|
| `MultipleBreak` | Multiple exit points | `if x { break } if y { break }` |
| `MultipleCarriers` | Condition uses multiple vars | `loop(i < n && j < m)` |
| `VariableStep` | Deltas aren't constants | `i = i + adjustment` |
| `NestedEscape` | Escape check inside other if | `if outer { if ch == \\ ... }` |

## Recognition Algorithm

### High-Level Steps

1. **Extract header carrier**: `i` from `loop(i < n)`
2. **Find escape check**: `if ch == "\\"`
3. **Find escape increment**: `i = i + 2` inside if
4. **Find process block**: `out = out + ch`
5. **Find normal increment**: `i = i + 1` after if
6. **Find break condition**: `if ch == "\"" { break }`
7. **Build LoopSkeleton**: `UpdateKind::ConditionalStep { cond, then_delta, else_delta }` を構築
8. **Build RoutingDecision**: `chosen = LoopBreak`（exit contract 優先）。P5b 固有の構造情報は `notes` に載せる

### Pseudo-Code

```rust
fn detect_escape_pattern(loop_expr: &Expr) -> Option<EscapePatternInfo> {
    // Step 1: Extract loop variable
    let (carrier_name, limit) = extract_header_carrier(loop_expr)?;

    // Step 2: Find escape check statement
    let escape_stmts = find_escape_check_block(loop_body)?;

    // Step 3: Extract escape character
    let escape_char = extract_escape_literal(escape_stmts)?;

    // Step 4: Extract escape delta
    let escape_delta = extract_escape_increment(escape_stmts, carrier_name)?;

    // Step 5: Find process statements
    let process_stmts = find_character_accumulation(loop_body)?;

    // Step 6: Extract normal increment
    let normal_delta = extract_normal_increment(loop_body, carrier_name)?;

    // Step 7: Find break condition
    let break_char = extract_break_literal(loop_body)?;

    // Build result
    Some(EscapePatternInfo {
        carrier_name,
        escape_char,
        normal_delta,
        escape_delta,
        break_char,
    })
}
```

### Implementation Location

**File**: `src/mir/loop_canonicalizer/canonicalizer.rs`

**Function**: `detect_escape_pattern()` (new)

**Integration point**: `canonicalize_loop_expr()` main dispatch

**Priority**: Call before `detect_skip_whitespace_shape()` (more specific)

## Skeleton Representation

### Standard Layout

```
LoopSkeleton {
    header: HeaderCond(Condition {
        operator: LessThan,
        left: Var("i"),
        right: Var("n")
    }),

    steps: [
        // Escape check block
        SkeletonStep::Body(vec![
            Expr::If {
                cond: Comparison("ch", Eq, Literal("\\")),
                then_body: [
                    Expr::Assign("i", Add, 1),  // escape_delta
                    Expr::Assign("ch", Substring("s", Var("i"), Add(Var("i"), 1))),
                ]
            }
        ]),

        // Character accumulation
        SkeletonStep::Body(vec![
            Expr::Assign("out", Append, Var("ch")),
        ]),

        // Normal increment
        SkeletonStep::Update(vec![
            Expr::Assign("i", Add, 1),  // normal_delta
        ]),
    ],

    carriers: vec![
        CarrierSlot {
            name: "i",
            update_kind: UpdateKind::ConditionalStep {
                cond: (ch == "\\"),
                then_delta: 2,
                else_delta: 1,
            },
            // ... other fields（role など）
        },
        CarrierSlot {
            name: "out",
            pattern: Append,
            // ... other fields
        }
    ],

    exit_contract: ExitContract {
        has_break: true,
        // ...
    }
}
```

## RoutingDecision Output

### For Valid P5b Route

```rust
RoutingDecision {
    chosen: LoopBreak,
    missing_caps: vec![],
    notes: vec![
        "escape_char: \\",
        "normal_delta: 1",
        "escape_delta: 2",
        "break_char: \"",
        "accumulator: out",
    ],
    confidence: High,
}
```

### For Invalid/Unsupported Cases

```rust
// Multiple escapes detected
RoutingDecision {
    chosen: Unknown,
    missing_caps: vec![CapabilityTag::MultipleBreak],
    notes: vec!["Multiple escape checks found"],
    confidence: Low,
}

// Variable step (not constant)
RoutingDecision {
    chosen: Unknown,
    missing_caps: vec![CapabilityTag::VariableStep],
    notes: vec!["Escape delta is not constant"],
    confidence: Low,
}
```

## Parity Verification

### Dev-Only Observation

In `src/mir/builder/control_flow/joinir/routing.rs`:

1. **Router makes decision** using existing route-family logic
2. **Canonicalizer analyzes** and detects escape route P5b
3. **Parity checker compares**:
   - Router decision (existing route family)
   - Canonicalizer decision (escape route P5b)
4. **If mismatch**:
   - Dev mode: Log with reason
   - Strict mode: Fail-Fast with error

### Expected Outcomes

**Case A: Router picks loop_simple_while, Canonicalizer picks P5b**
- Router: "Simple bounded loop"
- Canonicalizer: "Escape route detected"
- **Resolution**: Canonicalizer is more specific → router will eventually delegate

**Case B: Router fails, Canonicalizer succeeds**
- Router: "No route matched" (Fail-Fast)
- Canonicalizer: "escape route P5b matched"
- **Resolution**: P5b is new capability → expected until router updated

**Case C: Both agree P5b**
- Router: escape route P5b
- Canonicalizer: escape route P5b
- **Result**: ✅ Parity green

## Test Cases

Note:
- selfhost test filenames in this section use legacy test stems for traceability.
- Current semantics should be read as `escape route P5b`.
- Pin inventory: `docs/development/current/main/design/joinir-legacy-fixture-pin-inventory-ssot.md`

### Minimal Case

**Input**: String with one escape sequence
**Carrier**: Single position variable, single accumulator
**Deltas**: normal=1, escape=2
**Output**: Processed string (escape removed)

### Extended Cases (Phase 91 Step 2+; legacy selfhost test stems)

1. JSON string with multiple escapes
2. Custom escape character
3. Escape newline handling
4. Multiple escapes (should Fail-Fast)
5. Variable delta (should Fail-Fast)

## Lowering Strategy (Future Phase 92)

### Philosophy: Keep Return Simple

Escape route P5b lowering should:
1. **Reuse existing route-family lowering** for normal case
2. **Extend for conditional increment**:
   - PHI for carrier value after escape check
   - Separate paths for escape vs normal
3. **Close within escape-route P5b** (no cross-boundary complexity)

### Rough Outline

```
Entry: LoopPrefix
  ↓
Condition: i < n
  ↓
[BRANCH]
  ├→ EscapeBlock
  │   ├→ i = i + escape_delta
  │   └→ ch = substring(i)
  │
  └→ NormalBlock
      ├→ (ch already set)
      └→ noop

  (PHI: i from both branches)
  ↓
ProcessBlock: out = out + ch
  ↓
UpdateBlock: i = i + 1
  ↓
Condition check...
```

## Future Extensions

### Escape Route P5c: Multi-Character Escapes

```
if ch == "\\" {
    i = i + 2  // Skip \x
    if i < n {
        local second = s.substring(i, i+1)
        // Handle \n, \t, \x, etc.
    }
}
```

**Complexity**: Requires escape sequence table (not generic)

### Escape Route P5d: Nested Escape Contexts

```
// Regex with escaped /, inside JSON string with escaped "
loop(i < n) {
    if ch == "\"" { ... }     // String boundary
    if ch == "\\" {
        if in_regex {
            i = i + 2         // Regex escape
        } else {
            i = i + 1         // String escape
        }
    }
}
```

**Complexity**: State-dependent behavior (future work)

## References

- **JoinIR Architecture**: `joinir-architecture-overview.md`
- **Loop Canonicalizer**: `loop-canonicalizer.md`
- **CapabilityTag Enum**: `src/mir/loop_canonicalizer/capability_guard.rs`
- **Test Fixture pin inventory**: `docs/development/current/main/design/joinir-legacy-fixture-pin-inventory-ssot.md`
- **Phase 91 Plan**: `phases/phase-91/README.md`

---

## Summary

**Escape route P5b** enables JoinIR recognition of escape-sequence-aware string parsing loops by:

1. **Extending Canonicalizer** to detect conditional increments
2. **Adding exit-line optimization** for escape branching
3. **Preserving ExitContract** consistency with existing route families
4. **Enabling parity verification** in strict mode

**Status**: Design complete, implementation ready for Phase 91 Step 2
