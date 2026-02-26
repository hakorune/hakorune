# Phase 114: if-only return+post parity

## Status: DONE

## Background

Phase 114 extends if-only JoinIR lowering to support cases where:
- The if-only branch contains an early return
- Post-if statements exist after the if block
- The function eventually returns a value from the post-if path

This pattern is common in early-exit optimization code where certain conditions trigger an immediate return, while the normal flow continues with additional processing.

## Test Fixture

**File**: `apps/tests/phase114_if_only_return_then_post_min.hako`

```hako
// Phase 114: if-only return+post parity
// Tests: early return in if-only with post-if statements

static box Main {
  f(flag) {
    local x = 1
    if flag == 1 { return 7 }
    x = x + 1
    return x
  }

  main() {
    print(f(1))
    print(f(0))
    return "OK"
  }
}
```

**Expected Output**:
```
7
2
```

**Logic**:
- `f(1)`: condition true → early return 7
- `f(0)`: condition false → x=1+1=2, return 2

## Implementation

This pattern was already supported by the existing JoinIR if-only lowering infrastructure:
- If-select pattern recognition
- Early return handling via exit line routing
- Post-if statement processing in the else path

No new implementation was required - Phase 114 validates that this pattern works correctly in both VM and LLVM backends.

## Testing

### Smoke Tests

1. **VM Backend**: `tools/smokes/v2/profiles/integration/apps/archive/phase114_if_only_return_then_post_vm.sh`
   ```bash
   bash tools/smokes/v2/profiles/integration/apps/archive/phase114_if_only_return_then_post_vm.sh
   ```

2. **LLVM EXE Backend**: `tools/smokes/v2/profiles/integration/apps/archive/phase114_if_only_return_then_post_llvm_exe.sh`
   ```bash
   bash tools/smokes/v2/profiles/integration/apps/archive/phase114_if_only_return_then_post_llvm_exe.sh
   ```

Both tests run with:
- `NYASH_DISABLE_PLUGINS=1` (core path only)
- `HAKO_JOINIR_STRICT=1` (strict validation)

### Regression Tests

Verified backward compatibility with:
- Phase 103: if-only early return (both branches return)
- Phase 113: if-only partial assign (one branch assigns)

All regression tests pass without modifications.

## Verification Commands

```bash
# Quick VM test
bash tools/smokes/v2/profiles/integration/apps/archive/phase114_if_only_return_then_post_vm.sh

# Full LLVM EXE test (requires plugins)
bash tools/smokes/v2/profiles/integration/apps/archive/phase114_if_only_return_then_post_llvm_exe.sh

# Regression verification
bash tools/smokes/v2/profiles/integration/apps/phase103_if_only_early_return_vm.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase113_if_only_partial_assign_vm.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase113_if_only_partial_assign_llvm_exe.sh
```

## Architecture Notes

### Pattern Coverage

Phase 114 completes the if-only pattern matrix:

| Pattern | Phase | Description |
|---------|-------|-------------|
| Both branches return early | Phase 103 | `if flag { return A } else { return B }` |
| One branch assigns, else implicit | Phase 113 | `if flag { x = value }` (else preserves x) |
| **One branch returns early, else continues** | **Phase 114** | `if flag { return A }; post-code; return B` |

### Box Theory Alignment

This implementation follows Box Theory modularization principles:
- **Single responsibility**: If-only lowering handles one pattern variation
- **Separation of concerns**: Exit line routing is independent from assignment merging
- **Reusability**: Post-if statement processing reuses existing boundary SSA infrastructure
- **Fail-Fast**: No fallback paths - pattern either matches or explicitly fails

### Exit Line Routing

The key to Phase 114 is proper exit line routing:
1. **Early return**: Creates an exit line directly to function return
2. **Post-if path**: Continues to post-if statements, then returns
3. **No join point**: The two paths never merge (one exits early)

This is handled by the existing ExitLineReconnector Box.

## Related Documentation

- [Phase 103: if-only early return](../phase-103/README.md)
- [Phase 113: if-only partial assign](../phase-113/README.md)
- [JoinIR Architecture Overview](../../joinir-architecture-overview.md)
- [Exit Line Architecture](../../design/exit-line-architecture.md)
