# Phase 137: loop(true) break-once with return add expression

**Date**: 2025-12-18
**Status**: DONE ✅
**Scope**: return add expression (x + 2) support in Normalized shadow (dev-only)

---

## Goal

Extend Phase 136 return literal to support minimal add expressions:
- Enable `return x + 2` (variable + integer literal)
- Enable `return 5 + 3` (integer literal + integer literal)
- Keep **PHI禁止**: merge via env + continuations only
- Keep **dev-only** and **既定挙動不変**: unmatched shapes fall back

## Supported Forms

### ✅ Supported (Phase 137 P0)

```nyash
// Form 1: return variable + integer literal
local x
x = 1
loop(true) {
    break
}
return x + 2  // Expected: 3 (1 + 2)
```

```nyash
// Form 2: return integer literal + integer literal
loop(true) {
    break
}
return 5 + 3  // Expected: 8
```

```nyash
// Form 3: loop + post assigns + return add
local x
x = 0
loop(true) {
    x = 1
    break
}
x = x + 10
return x + 2  // Expected: 13 (0 → 1 → 11 → 13)
```

### ❌ Not Supported (Out of Scope)

```nyash
// Variable + variable (Phase 138+)
return x + y

// Other operators (Phase 138+)
return x - 2
return x * 2

// Nested expressions
return (x + 2) + 3

// Function calls
return f()
```

## Implementation

### Core Changes

**File**: `src/mir/control_tree/normalized_shadow/loop_true_break_once.rs`

**Method**: `lower_return_value_to_vid()` (行 638-743, BinaryOp Add at 行 673)

**Added Pattern**: BinaryOp Add
```rust
ASTNode::BinaryOp { operator, left, right, .. } => {
    // Phase 137 contract: Add only
    if !matches!(operator, BinaryOperator::Add) {
        return Ok(None); // out of scope
    }

    // Lower LHS (Variable or Integer literal)
    let lhs_vid = match left.0.as_ref() {
        ASTNode::Variable { name, .. } => {
            match env.get(name).copied() {
                Some(vid) => vid,
                None => return Ok(None), // out of scope
            }
        }
        ASTNode::Literal { value: LiteralValue::Integer(i), .. } => {
            // Generate Const for LHS
            let vid = ValueId(*next_value_id);
            *next_value_id += 1;
            body.push(JoinInst::Compute(MirLikeInst::Const {
                dst: vid,
                value: ConstValue::Integer(*i),
            }));
            vid
        }
        _ => return Ok(None),
    };

    // Lower RHS (Integer literal only)
    let rhs_vid = match right.0.as_ref() {
        ASTNode::Literal { value: LiteralValue::Integer(i), .. } => {
            let vid = ValueId(*next_value_id);
            *next_value_id += 1;
            body.push(JoinInst::Compute(MirLikeInst::Const {
                dst: vid,
                value: ConstValue::Integer(*i),
            }));
            vid
        }
        _ => return Ok(None), // e.g., return x + y
    };

    // Generate BinOp Add
    let result_vid = ValueId(*next_value_id);
    *next_value_id += 1;
    body.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: result_vid,
        op: BinOpKind::Add,
        lhs: lhs_vid,
        rhs: rhs_vid,
    }));

    Ok(Some(result_vid))
}
```

### Return Value Lowering SSOT

**Location**: `src/mir/control_tree/normalized_shadow/loop_true_break_once.rs` (行 29-43)

**SSOT Documentation**:
```rust
//! ## Return Value Lowering SSOT (Phase 137+)
//!
//! - Function: `lower_return_value_to_vid()`
//! - Responsibility: Lower return values (variable, literal, expr) to ValueId
//! - Supported patterns:
//!   - Variable: env lookup
//!   - Integer literal: Const generation
//!   - Add expr (Phase 137): x + 2 → BinOp(Add, env[x], Const(2))
//! - Fallback: Out-of-scope patterns return `Ok(None)` for legacy routing
//!
//! ### Boxification Trigger
//!
//! If 2+ files need identical return lowering logic, promote to:
//! - `normalized_shadow/common/return_value_lowerer_box.rs`
//! - Single responsibility: return value → ValueId conversion
```

### Fixtures

1. **phase137_loop_true_break_once_return_add_min.hako**
   - Pattern: `x=1; loop(true){break}; return x+2`
   - Expected: exit code 3

2. **phase137_loop_true_break_once_return_add_const_min.hako**
   - Pattern: `loop(true){break}; return 5+3`
   - Expected: exit code 8

3. **phase137_loop_true_break_once_post_return_add_min.hako**
   - Pattern: `x=0; loop(true){x=1;break}; x=x+10; return x+2`
   - Expected: exit code 13

### Smoke Tests

**VM**:
- `tools/smokes/v2/profiles/integration/apps/archive/phase137_loop_true_break_once_return_add_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase137_loop_true_break_once_return_add_const_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase137_loop_true_break_once_post_return_add_vm.sh`

**LLVM EXE**:
- `tools/smokes/v2/profiles/integration/apps/archive/phase137_loop_true_break_once_return_add_llvm_exe.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase137_loop_true_break_once_return_add_const_llvm_exe.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase137_loop_true_break_once_post_return_add_llvm_exe.sh`

## Verification

```bash
# Build
cargo build --release -p nyash-rust --features llvm

# Phase 137 smokes (6 tests)
bash tools/smokes/v2/profiles/integration/apps/archive/phase137_loop_true_break_once_return_add_vm.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase137_loop_true_break_once_return_add_const_vm.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase137_loop_true_break_once_post_return_add_vm.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase137_loop_true_break_once_return_add_llvm_exe.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase137_loop_true_break_once_return_add_const_llvm_exe.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase137_loop_true_break_once_post_return_add_llvm_exe.sh

# Regressions
bash tools/smokes/v2/profiles/integration/apps/archive/phase97_next_non_ws_llvm_exe.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase97_json_loader_escape_llvm_exe.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase131_loop_true_break_once_vm.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase135_loop_true_break_once_post_empty_return_vm.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase136_loop_true_break_once_return_literal_vm.sh
```

## Acceptance Criteria

- ✅ Phase 137 VM: 3/3 PASS
- ✅ Phase 137 LLVM EXE: 3/3 PASS
- ✅ Phase 97 regression: 2/2 PASS
- ✅ Phase 131/135/136 regression: 3/3 PASS
- ✅ Dev toggle OFF → no impact (Ok(None) fallback)

## Key Design Points

### Ok(None) for Fallback
- Unsupported patterns (e.g., `return x + y`, `return x - 2`) return `Ok(None)`
- Fallback to existing JoinIR routing
- No hard errors for out-of-scope patterns

### BinOp Generation Pattern
- LHS: Variable (env lookup) or Integer literal (Const generation)
- RHS: Integer literal only (Const generation)
- Result: `BinOp(Add, lhs_vid, rhs_vid)`

### Phase 137 P0 Scope Decision
- **Approach A (adopted)**: Direct extension in `loop_true_break_once.rs`
- **Reasoning**: Small change scope, no boxification needed yet
- **Boxification Trigger**: When 2+ files need identical return lowering logic

### post_if_post_k.rs Not Modified
- Different responsibility (if-with-post normalization)
- Unification planned for Phase 138-139 when needed

## Current Status

Phase 137 - DONE ✅ (2025-12-18)

VM/LLVM EXE parity achieved (exit codes 3, 8, 13).

Return Value Lowering SSOT documented in `loop_true_break_once.rs`.
