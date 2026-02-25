# Phase 136: loop(true) break-once with return literal

**Date**: 2025-12-18
**Status**: DONE ✅
**Scope**: return literal (Integer) support in Normalized shadow (dev-only)

---

## Goal

Extend Phase 131-135 `loop(true){...; break}` to support return integer literal:
- Enable `return 7` after loop exit
- Keep **PHI禁止**: merge via env + continuations only
- Keep **dev-only** and **既定挙動不変**: unmatched shapes fall back

## Supported Forms

### ✅ Supported (Phase 136)

```nyash
// Form 1: loop + return literal
local x
x = 0
loop(true) {
    x = 1
    break
}
return 7  // Expected: 7 (literal, not variable)
```

```nyash
// Form 2: loop + post assigns + return literal
local x
x = 0
loop(true) {
    x = 1
    break
}
x = x + 2
return 7  // Expected: 7 (literal)
```

### ❌ Not Supported (Out of Scope)

```nyash
// Return expression (Phase 137+)
return x + 2

// Return string literal
return "hello"

// Return float literal
return 3.14
```

## Implementation

### Core Changes

**File**: `src/mir/control_tree/normalized_shadow/loop_true_break_once.rs`

**Method**: `lower_return_value_to_vid()` (行 638-743, Integer literal at 行 661)

**Added Pattern**: Integer literal
```rust
ASTNode::Literal { value: LiteralValue::Integer(i), .. } => {
    let const_vid = ValueId(*next_value_id);
    *next_value_id += 1;

    body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: const_vid,
        value: ConstValue::Integer(*i),
    }));

    Ok(Some(const_vid))
}
```

### Fixtures

1. **phase136_loop_true_break_once_return_literal_min.hako**
   - Pattern: `loop(true){ x=1; break }; return 7`
   - Expected: exit code 7

2. **phase136_loop_true_break_once_post_return_literal_min.hako**
   - Pattern: `loop(true){ x=1; break }; x=x+2; return 7`
   - Expected: exit code 7

### Smoke Tests

**VM**:
- `tools/smokes/v2/profiles/integration/apps/phase136_loop_true_break_once_return_literal_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/phase136_loop_true_break_once_post_return_literal_vm.sh`

**LLVM EXE**:
- `tools/smokes/v2/profiles/integration/apps/phase136_loop_true_break_once_return_literal_llvm_exe.sh`
- `tools/smokes/v2/profiles/integration/apps/phase136_loop_true_break_once_post_return_literal_llvm_exe.sh`

## Verification

```bash
# Build
cargo build --release -p nyash-rust --features llvm

# Phase 136 smokes (4 tests)
bash tools/smokes/v2/profiles/integration/apps/phase136_loop_true_break_once_return_literal_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase136_loop_true_break_once_post_return_literal_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase136_loop_true_break_once_return_literal_llvm_exe.sh
bash tools/smokes/v2/profiles/integration/apps/phase136_loop_true_break_once_post_return_literal_llvm_exe.sh

# Regressions
bash tools/smokes/v2/profiles/integration/apps/phase131_loop_true_break_once_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase135_loop_true_break_once_post_empty_return_vm.sh
```

## Acceptance Criteria

- ✅ Phase 136 VM: 2/2 PASS
- ✅ Phase 136 LLVM EXE: 2/2 PASS
- ✅ Phase 131/135 regression: 2/2 PASS
- ✅ Dev toggle OFF → no impact (Ok(None) fallback)

## Key Design Points

### Ok(None) for Fallback
- Unsupported patterns (e.g., `return "hello"`) return `Ok(None)`
- Fallback to existing JoinIR routing (Pattern2, etc.)
- No hard errors for out-of-scope patterns

### Const Generation Pattern (Phase 123)
- `Const{dst, Integer(i)} → Ret{Some(dst)}`
- Reused existing pattern from Phase 123

### post_k/k_exit Both Supported
- Same helper used in both locations
- Unified return value lowering

## Current Status

Phase 136 - DONE ✅ (2025-12-18)

VM/LLVM EXE parity achieved (exit code 7).
